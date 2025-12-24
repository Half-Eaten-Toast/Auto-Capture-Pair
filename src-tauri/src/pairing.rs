//! Helper to generate a pairing file for a connected device given its UDID.

use uuid::Uuid;

use idevice::{
    afc::{opcode::AfcFopenMode, AfcClient},
    house_arrest::{self, HouseArrestClient},
    lockdown::LockdownClient,
    pairing_file::PairingFile,
    usbmuxd::{UsbmuxdAddr, UsbmuxdConnection},
    IdeviceError, IdeviceService,
};

use plist::{Dictionary as PlistDict, Value as PlistValue};

/// Generate a new pairing file for the device with the given `udid`.
///
/// This will:
/// - connect to usbmuxd,
/// - locate the device by UDID,
/// - connect to lockdown on that device,
/// - fetch the host BUID and tweak it (to avoid invalidating an already-connected pairing),
/// - call lockdown pair to create a new PairingFile,
/// - set the PairingFile.udid to the provided UDID and return it.
///
/// Returns Err(IdeviceError::DeviceNotFound) if there is no connected device
/// with the provided UDID.
pub async fn generate_pairing_file_for_udid(udid: &str) -> Result<PairingFile, IdeviceError> {
    log::info!("generate_pairing_file_for_udid: starting for udid={}", udid);

    // Connect to usbmuxd
    log::debug!("generate_pairing_file_for_udid: connecting to usbmuxd");
    let mut uc: UsbmuxdConnection = UsbmuxdConnection::default().await?;
    log::debug!("generate_pairing_file_for_udid: connected to usbmuxd");

    // Get device list and find by UDID
    let devices = uc.get_devices().await?;
    log::debug!(
        "generate_pairing_file_for_udid: found {} devices",
        devices.len()
    );
    let dev = match devices.into_iter().find(|d| d.udid == udid) {
        Some(d) => d,
        None => return Err(IdeviceError::DeviceNotFound),
    };
    log::info!(
        "generate_pairing_file_for_udid: selected device {}",
        dev.udid
    );

    // Build a provider for lockdown and connect
    let provider = dev.to_provider(UsbmuxdAddr::default(), "idevice_pair");
    log::debug!("generate_pairing_file_for_udid: connecting to lockdown");
    let mut lc = LockdownClient::connect(&provider).await?;
    log::debug!("generate_pairing_file_for_udid: connected to lockdown");

    // Get the host BUID and tweak it like the main app to avoid invalidating the active one.
    let buid = uc.get_buid().await?;
    let mut buid_chars: Vec<char> = buid.chars().collect();
    if !buid_chars.is_empty() {
        buid_chars[0] = if buid_chars[0] == 'F' { 'A' } else { 'F' };
    }
    let buid: String = buid_chars.into_iter().collect();

    // Generate a new uppercase UUID for the host id
    let id = Uuid::new_v4().to_string().to_uppercase();
    log::debug!(
        "generate_pairing_file_for_udid: using host id {} and buid {}",
        id,
        buid
    );

    // Pair and return the pairing file
    let mut pairing_file = match uc.get_pair_record(udid).await {
        Ok(p) => p,
        Err(e) => {
            // pair using lc.pair
            let pf = lc.pair(id, buid).await?;
            log::debug!(
                "generate_pairing_file_for_udid: created new pair record for {}",
                udid
            );
            pf
        }
    };
    pairing_file.udid = Some(dev.udid.clone());
    log::info!(
        "generate_pairing_file_for_udid: pairing succeeded for {}",
        udid
    );
    /*log::debug!(
        "generate_pairing_file_for_udid: pairing_file= {:?}",
        pairing_file
    );*/

    lc.start_session(&pairing_file).await?;

    lc.set_value(
        "EnableWifiDebugging",
        true.into(),
        Some("com.apple.mobile.wireless_lockdown"),
    )
    .await?;
    log::debug!(
        "generate_pairing_file_for_udid: enabled wifi debugging for {}",
        udid
    );

    Ok(pairing_file)
}

pub async fn upload_pairing_file_to_device(
    udid: &str,
    pairing_file: &PairingFile,
) -> Result<(), IdeviceError> {
    log::info!("upload_pairing_file_to_device: starting for udid={}", udid);

    // Connect to usbmuxd
    log::debug!("upload_pairing_file_to_device: connecting to usbmuxd");
    let mut uc: UsbmuxdConnection = UsbmuxdConnection::default().await?;
    log::debug!("upload_pairing_file_to_device: connected to usbmuxd");

    // Get device list and find by UDID
    let devices = uc.get_devices().await?;
    log::debug!(
        "upload_pairing_file_to_device: found {} devices",
        devices.len()
    );
    let dev = match devices.into_iter().find(|d| d.udid == udid) {
        Some(d) => d,
        None => return Err(IdeviceError::DeviceNotFound),
    };
    log::info!(
        "upload_pairing_file_to_device: selected device {}",
        dev.udid
    );

    // Build a provider for lockdown and connect
    let provider = dev.to_provider(UsbmuxdAddr::default(), "idevice_pair");

    // connect to afc
    log::debug!("upload_pairing_file_to_device: connecting to HouseArrestClient");
    let mut ha_client = house_arrest::HouseArrestClient::connect(&provider)
        .await
        .map_err(|e| {
            log::error!("Failed to connect to HouseArrestClient: {:?}", e);
            e
        })?;
    log::debug!("upload_pairing_file_to_device: connected to HouseArrestClient");
    let mut afc = ha_client
        .vend_documents("com.halfeatentoast.devcapture")
        .await
        .map_err(|e| {
            log::error!("Failed to vend documents: {:?}", e);
            e
        })?;
    log::debug!("upload_pairing_file_to_device: obtained afc client");

    // serialize pairing file to plist data
    let pairing_file_plist: Vec<u8> = pairing_file.clone().serialize().map_err(|e| {
        log::error!("Failed to serialize pairing file: {:?}", e);
        e
    })?;
    log::debug!(
        "upload_pairing_file_to_device: serialized pairing file ({} bytes)",
        pairing_file_plist.len()
    );
    // write pairing file to device
    log::debug!("upload_pairing_file_to_device: opening file on device");
    let mut file = afc
        .open("/Documents/pairing_record.plist", AfcFopenMode::WrOnly)
        .await
        .map_err(|e| {
            log::error!("Failed to open file on device: {:?}", e);
            e
        })?;
    file.write_entire(&pairing_file_plist).await.map_err(|e| {
        log::error!("Failed to write pairing file to device: {:?}", e);
        e
    })?;
    log::info!(
        "upload_pairing_file_to_device: successfully wrote pairing file to device {}",
        udid
    );
    Ok(())
}
