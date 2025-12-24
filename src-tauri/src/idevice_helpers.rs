use std::{any::Any, collections::HashMap};

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
