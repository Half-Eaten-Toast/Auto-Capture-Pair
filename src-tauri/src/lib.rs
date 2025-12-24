use idevice::pairing_file;

mod idevice_helpers;
mod pairing;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
/*#[tauri::command]
fn greet(name: &str) -> String {
    idevice_helpers::get_devices();
}*/

#[tauri::command]
async fn get_devices() -> Result<std::collections::HashMap<String, String>, String> {
    // Retrieve a serializable map from the helper (DeviceName -> UDID)
    let map = idevice_helpers::get_devices()
        .await
        .map_err(|e| format!("idevice error: {:?}", e))?;

    // Return the map directly; Tauri will serialize it to JSON for the frontend
    Ok(map)
}

#[tauri::command]
async fn generate_pairing_file(
    udid: String,
    destination: Option<String>,
) -> Result<String, String> {
    // Call the async pairing helper and return a serialized result
    let pairing_file = pairing::generate_pairing_file_for_udid(&udid)
        .await
        .map_err(|e| format!("idevice error: {:?}", e))?;

    let pairing_file_plist: Vec<u8> = pairing_file
        .serialize()
        .map_err(|e| format!("failed to serialize pairing file: {}", e))?;

    if let Some(dest) = destination {
        std::fs::write(&dest, &pairing_file_plist)
            .map_err(|e| format!("failed to write pairing file to {}: {}", dest, e))?;
        Ok(dest)
    } else {
        Ok(String::from_utf8(pairing_file_plist).map_err(|e| format!("invalid utf8: {}", e))?)
    }
}

#[tauri::command]
fn get_app_data_folder() -> Result<String, String> {
    // Return an application-specific data folder path for the current platform.
    // We avoid adding a new dependency by consulting common environment variables.
    let app_name = "Auto Capture Pair";

    #[cfg(target_os = "windows")]
    {
        if let Some(appdata) = std::env::var_os("APPDATA") {
            let mut path = std::path::PathBuf::from(appdata);
            path.push(app_name);
            return Ok(path.to_string_lossy().to_string());
        }
        return Err("APPDATA not set".into());
    }

    #[cfg(not(target_os = "windows"))]
    {
        if let Some(xdg) = std::env::var_os("XDG_DATA_HOME") {
            let mut path = std::path::PathBuf::from(xdg);
            path.push(app_name);
            return Ok(path.to_string_lossy().to_string());
        }
        if let Some(home) = std::env::var_os("HOME") {
            let mut path = std::path::PathBuf::from(home);
            path.push(".local");
            path.push("share");
            path.push(app_name);
            return Ok(path.to_string_lossy().to_string());
        }
        return Err("Could not determine data directory".into());
    }
}

//setup_device(gens the pairing file and uploads it to the device)
#[tauri::command]
async fn setup_device(udid: String) -> Result<(), String> {
    log::info!("Setting up device with UDID: {}", &udid);
    let pairing_file = pairing::generate_pairing_file_for_udid(&udid)
        .await
        .map_err(|e| format!("idevice error: {:?}", e))?;
    log::info!("Generated pairing file for device {}", &udid);

    pairing::upload_pairing_file_to_device(&udid, &pairing_file)
        .await
        .map_err(|e| format!("idevice error: {:?}", e))?;
    log::info!("Uploaded pairing file to device {}", &udid);
    Ok(())
}

// get_device_in_dev_mode
#[tauri::command]
async fn get_device_in_dev_mode(udid: String) -> Result<bool, String> {
    log::info!("Checking if device with UDID: {} is in dev mode", &udid);
    let in_dev_mode = idevice_helpers::is_device_in_dev_mode(&udid)
        .await
        .map_err(|e| format!("idevice error: {:?}", e))?;
    log::info!(
        "Device with UDID: {} is in dev mode: {}",
        &udid,
        in_dev_mode
    );
    Ok(in_dev_mode)
}

// reveal dev mode option in ui
#[tauri::command]
async fn reveal_dev_mode(udid: String) -> Result<(), String> {
    log::info!(
        "Revealing developer mode option in UI for device with UDID: {}",
        &udid
    );
    idevice_helpers::reveal_dev_mode(&udid)
        .await
        .map_err(|e| format!("idevice error: {:?}", e))?;
    log::info!(
        "Revealed developer mode option in UI for device with UDID: {}",
        &udid
    );
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize logging (no-op if already initialized).
    let _ = env_logger::try_init();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_devices,
            generate_pairing_file,
            get_app_data_folder,
            setup_device,
            get_device_in_dev_mode,
            reveal_dev_mode
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
