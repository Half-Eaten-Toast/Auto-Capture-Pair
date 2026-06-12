//! Helper to generate a pairing file for a connected device given its UDID.

use uuid::Uuid;

use idevice::{
    afc::{opcode::AfcFopenMode, AfcClient},
    core_device_proxy::CoreDeviceProxy,
    house_arrest::{self, HouseArrestClient},
    lockdown::LockdownClient,
    pairing_file::PairingFile,
    remote_pairing::{RemotePairingClient, RpPairingFile},
    rsd::RsdHandshake,
    usbmuxd::{UsbmuxdAddr, UsbmuxdConnection},
    IdeviceError, IdeviceService, RemoteXpcClient,
};

use plist::{Dictionary as PlistDict, Value as PlistValue};

fn pairing_hostname() -> String {
    let suffix: String = uuid::Uuid::new_v4()
        .simple()
        .to_string()
        .chars()
        .take(6)
        .collect();
    format!("Auto Capture Pairing-{suffix}")
}

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
pub async fn generate_pairing_file_for_udid(udid: &str) -> Result<RpPairingFile, IdeviceError> {
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

    let pairing_file = uc.get_pair_record(udid).await?;

    // Build a provider for lockdown and connect
    let provider = dev.to_provider(UsbmuxdAddr::default(), "idevice_pair");
    let mut lc = LockdownClient::connect(&provider).await?;
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

    let hostname = pairing_hostname();

    let proxy =
        CoreDeviceProxy::connect(&(dev.to_provider(UsbmuxdAddr::default(), "idevice_pair")))
            .await?;
    let rsd_port = proxy.tunnel_info().server_rsd_port;

    let adapter = proxy.create_software_tunnel()?;
    let mut adapter = adapter.to_async_handle();

    let rsd_stream = adapter.connect(rsd_port).await?;
    let handshake = RsdHandshake::new(rsd_stream).await?;

    let tunnel_service = handshake
        .services
        .get("com.apple.internal.dt.coredevice.untrusted.tunnelservice")
        .ok_or_else(|| IdeviceError::InternalError("Untrusted tunnel service not found".into()))?;

    let tunnel_service_stream = adapter.connect(tunnel_service.port).await?;
    let mut remote_xpc = RemoteXpcClient::new(tunnel_service_stream).await?;
    remote_xpc.do_handshake().await?;
    let _ = remote_xpc.recv_root().await;

    let mut rp_pairing_file = RpPairingFile::generate(&hostname);
    let mut pairing_client = RemotePairingClient::new(remote_xpc, &hostname, &mut rp_pairing_file);
    pairing_client
        .connect(async |_| "000000".to_string(), ())
        .await?;

    // use it to try and force keychain commitment
    // iOS has trouble commiting I guess
    let tunnel_service_stream = adapter.connect(tunnel_service.port).await?;
    let mut remote_xpc = RemoteXpcClient::new(tunnel_service_stream).await?;
    remote_xpc.do_handshake().await?;
    let _ = remote_xpc.recv_root().await;

    let mut pairing_client = RemotePairingClient::new(remote_xpc, &hostname, &mut rp_pairing_file);
    pairing_client
        .connect(async |_| "000000".to_string(), ())
        .await?;

    log::info!(
        "generate_pairing_file_for_udid: pairing succeeded for {}",
        udid
    );
    /*log::debug!(
        "generate_pairing_file_for_udid: pairing_file= {:?}",
        pairing_file
    );*/

    Ok(rp_pairing_file)
}

pub async fn upload_pairing_file_to_device(
    udid: &str,
    pairing_file: &RpPairingFile,
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
    let ha_client = house_arrest::HouseArrestClient::connect(&provider)
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
    let pairing_file_plist: Vec<u8> = pairing_file.clone().to_bytes();
    log::debug!(
        "upload_pairing_file_to_device: serialized pairing file ({} bytes)",
        pairing_file_plist.len()
    );
    // write pairing file to device
    log::debug!("upload_pairing_file_to_device: opening file on device");
    let mut file = afc
        .open("/Documents/rpPairingFile.plist", AfcFopenMode::WrOnly)
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
