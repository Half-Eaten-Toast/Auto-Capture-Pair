use std::collections::HashMap;

use idevice::{
    amfi::AmfiClient,
    lockdown::LockdownClient,
    usbmuxd::{Connection, UsbmuxdAddr, UsbmuxdConnection},
    IdeviceError, IdeviceService,
};

/// Query usbmuxd for attached USB devices and try to read each device's
/// `DeviceName` from lockdown. Returns a map of `DeviceName -> UsbmuxdDevice`.
///
/// This is an async helper you can call from your tokio runtime:
/// let devices = idevice_helpers::get_devices().await?;
pub async fn get_devices() -> Result<HashMap<String, String>, IdeviceError> {
    // Connect to usbmuxd
    let mut uc = UsbmuxdConnection::default().await?;
    let devs = uc.get_devices().await?;

    let mut selections = HashMap::new();

    for dev in devs
        .into_iter()
        .filter(|x| x.connection_type == Connection::Usb)
    {
        // Create a provider for lockdown
        let provider = dev.to_provider(UsbmuxdAddr::default(), "idevice_pair");

        // Try to connect to lockdown; if it fails log a warning and skip the device.
        match LockdownClient::connect(&provider).await {
            Ok(mut lc) => {
                match lc.get_value(None, None).await {
                    Ok(values) => {
                        if let Some(name) = values
                            .as_dictionary()
                            .and_then(|d| d.get("DeviceName"))
                            .and_then(|v| v.as_string())
                        {
                            // store device name -> udid so the map is easily serializable to JSON
                            selections.insert(name.to_string(), dev.udid.clone());
                        } else {
                            // If DeviceName isn't present we skip; you might want to
                            // insert devices by UDID or another key instead.
                            log::warn!("Device {} had no DeviceName, skipping", dev.udid);
                        }
                    }
                    Err(e) => {
                        log::warn!("Failed to get lockdown values for {}: {e:?}", dev.udid);
                    }
                }
            }
            Err(e) => {
                log::warn!("Failed to connect to lockdown for {}: {e:?}", dev.udid);
            }
        }
    }

    Ok(selections)
}

// check dev mode
pub async fn is_device_in_dev_mode(udid: &str) -> Result<bool, IdeviceError> {
    log::info!("is_device_in_dev_mode: starting for udid={}", udid);

    // Connect to usbmuxd
    log::debug!("is_device_in_dev_mode: connecting to usbmuxd");
    let mut uc: UsbmuxdConnection = UsbmuxdConnection::default().await?;
    log::debug!("is_device_in_dev_mode: connected to usbmuxd");

    // Get device list and find by UDID
    let devices = uc.get_devices().await?;
    log::debug!("is_device_in_dev_mode: found {} devices", devices.len());
    let dev = match devices.into_iter().find(|d| d.udid == udid) {
        Some(d) => d,
        None => return Err(IdeviceError::DeviceNotFound),
    };
    log::info!("is_device_in_dev_mode: selected device {}", dev.udid);

    // Build a provider for lockdown and connect
    let provider = dev.to_provider(UsbmuxdAddr::default(), "idevice_pair");

    // Try to connect to lockdown; if it fails log a warning and skip the device.
    let mut amfi_client = AmfiClient::connect(&provider).await?;

    // Check DevelopmentMode status returns true if enabled, false if disabled
    let dev_mode_value = amfi_client.get_developer_mode_status().await?;

    Ok(dev_mode_value)
}

// Reveal dev mode
pub async fn reveal_dev_mode(udid: &str) -> Result<(), IdeviceError> {
    log::info!("is_device_in_dev_mode: starting for udid={}", udid);

    // Connect to usbmuxd
    log::debug!("is_device_in_dev_mode: connecting to usbmuxd");
    let mut uc: UsbmuxdConnection = UsbmuxdConnection::default().await?;
    log::debug!("is_device_in_dev_mode: connected to usbmuxd");

    // Get device list and find by UDID
    let devices = uc.get_devices().await?;
    log::debug!("is_device_in_dev_mode: found {} devices", devices.len());
    let dev = match devices.into_iter().find(|d| d.udid == udid) {
        Some(d) => d,
        None => return Err(IdeviceError::DeviceNotFound),
    };
    log::info!("is_device_in_dev_mode: selected device {}", dev.udid);

    // Build a provider for lockdown and connect
    let provider = dev.to_provider(UsbmuxdAddr::default(), "idevice_pair");

    // Try to connect to lockdown; if it fails log a warning and skip the device.
    let mut amfi_client = AmfiClient::connect(&provider).await?;

    // Check DevelopmentMode status returns true if enabled, false if disabled
    amfi_client.reveal_developer_mode_option_in_ui().await?;
    Ok(())
}

#[allow(dead_code)]
pub fn check_apple_drivers() -> Result<String, String> {
    // Only relevant on Windows
    if !cfg!(target_os = "windows") {
        return Ok("NotWindows".into());
    }

    use std::process::Command;

    let output = Command::new("pnputil")
        .arg("/enum-drivers")
        .output()
        .map_err(|e| format!("failed to run pnputil: {}", e))?;
    let stdout = String::from_utf8_lossy(&output.stdout).to_lowercase();

    if !stdout.contains("netaapl64")
        || stdout.contains("apple mobile")
        || stdout.contains("apple usb")
    {
        Ok("Installed".into())
    } else {
        Ok("Missing".into())
    }
}

pub fn install_apple_drivers() -> Result<String, String> {
    // Only run on Windows
    if !cfg!(target_os = "windows") {
        return Err("NotWindows".into());
    }

    use std::fs;
    use std::path::PathBuf;

    // Write embedded script into AppData/Auto Capture Pair
    let appdata = std::env::var("APPDATA").map_err(|_| "APPDATA not set".to_string())?;
    let dir = PathBuf::from(appdata).join("Auto Capture Pair");
    fs::create_dir_all(&dir).map_err(|e| format!("failed to create dir: {}", e))?;
    let script_path = dir.join("install-apple-drivers.ps1");

    // Embed the repository script at compile time and write it out
    let script_contents = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../install-apple-drivers.ps1"
    ));
    fs::write(&script_path, script_contents)
        .map_err(|e| format!("failed to write script: {}", e))?;

    // Start elevated PowerShell to run the script (prompts UAC).
    // Use ShellExecuteExW with the "runas" verb to request elevation and wait for the process to exit.
    #[cfg(target_os = "windows")]
    {
        use std::ffi::OsStr;
        use std::os::windows::ffi::OsStrExt;
        use std::ptr;
        use winapi::um::handleapi::CloseHandle;
        use winapi::um::processthreadsapi::GetExitCodeProcess;
        use winapi::um::shellapi::{ShellExecuteExW, SEE_MASK_NOCLOSEPROCESS, SHELLEXECUTEINFOW};
        use winapi::um::synchapi::WaitForSingleObject;
        use winapi::um::winbase::INFINITE;
        use winapi::um::winuser::SW_HIDE;

        fn to_wide(s: &OsStr) -> Vec<u16> {
            s.encode_wide().chain(Some(0)).collect()
        }

        let verb = to_wide(OsStr::new("runas"));
        let file = to_wide(OsStr::new("powershell.exe"));
        let params = to_wide(OsStr::new(&format!(
            "-NoProfile -ExecutionPolicy Bypass -WindowStyle Hidden -File \"{}\"",
            script_path.display()
        )));

        let mut sei: SHELLEXECUTEINFOW = unsafe { std::mem::zeroed() };
        sei.cbSize = std::mem::size_of::<SHELLEXECUTEINFOW>() as u32;
        sei.fMask = SEE_MASK_NOCLOSEPROCESS;
        sei.hwnd = ptr::null_mut();
        sei.lpVerb = verb.as_ptr();
        sei.lpFile = file.as_ptr();
        sei.lpParameters = params.as_ptr();
        sei.lpDirectory = ptr::null_mut();
        sei.nShow = SW_HIDE as i32;
        sei.hInstApp = ptr::null_mut() as _;

        let ok = unsafe { ShellExecuteExW(&mut sei as *mut SHELLEXECUTEINFOW) != 0 };
        if !ok {
            return Err("ShellExecuteExW failed".into());
        }

        let hproc = sei.hProcess;
        if hproc.is_null() {
            return Err("Failed to get process handle".into());
        }

        // Wait for process to finish
        let wait = unsafe { WaitForSingleObject(hproc, INFINITE) };
        if wait == winapi::um::winbase::WAIT_FAILED {
            unsafe { CloseHandle(hproc) };
            return Err("WaitForSingleObject failed".into());
        }

        let mut exit_code: u32 = 0;
        let got = unsafe { GetExitCodeProcess(hproc, &mut exit_code as *mut u32) };
        unsafe { CloseHandle(hproc) };
        if got == 0 {
            return Err("GetExitCodeProcess failed".into());
        }

        if exit_code == 0 {
            Ok(script_path.to_string_lossy().to_string())
        } else {
            Err(format!("installer exited with code {}", exit_code))
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        // Fallback (should not be hit, function guarded by target check)
        use std::process::Command;
        let cmd = format!(
            "Start-Process powershell -ArgumentList '-NoProfile -ExecutionPolicy Bypass -File \"{}\"'",
            script_path.display()
        );
        let status = Command::new("powershell")
            .arg("-NoProfile")
            .arg("-Command")
            .arg(cmd)
            .status()
            .map_err(|e| format!("failed to spawn powershell: {}", e))?;

        if status.success() {
            Ok(script_path.to_string_lossy().to_string())
        } else {
            Err(format!("powershell returned {:?}", status.code()))
        }
    }
}
