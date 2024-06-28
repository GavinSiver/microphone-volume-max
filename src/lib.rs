use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::thread;
use std::time::Duration;

use clap::Parser;
use windows::{
    Win32::Media::Audio::{DEVICE_STATE_ACTIVE, eCapture, IMMDevice, IMMDeviceEnumerator},
    Win32::System::Com::STGM_READ,
};
use windows::Win32::Devices::Properties::DEVPKEY_Device_FriendlyName;
use windows::Win32::Media::Audio::{eConsole, MMDeviceEnumerator};
use windows::Win32::Media::Audio::Endpoints::IAudioEndpointVolume;
use windows::Win32::System::Com::*;
use windows::Win32::System::Com::StructuredStorage::PropVariantToStringAlloc;
use windows::Win32::UI::Shell::PropertiesSystem::IPropertyStore;

#[derive(Parser)]
#[command(name = "Windows Microphone Volume Max")]
#[command(
    version,
    about = "Set the microphone volume to 100% on a recurring basis.",
    long_about = "Finds the default microphone, and sets its volume to 100%; repeats this action every T seconds. Default is 60 seconds."
)]
struct Config {
    /// Time in Seconds to wait before checking the volume again
    #[arg(short, long, default_value = "60", value_parser = clap::value_parser ! (u64).range(1..))]
    time: u64,
}

pub fn start_windows_microphone_volume_max() -> Result<(), windows::core::Error> {
    let config = Config::parse();

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || r.store(false, Ordering::SeqCst))
        .expect("Error setting Ctrl-C handler");

    unsafe {
        // Initialize COM
        if CoInitializeEx(Some(std::ptr::null_mut()), COINIT_MULTITHREADED).is_err() {
            eprintln!("Failed to initialize COM library");
            return Err(windows::core::Error::from_win32());
        }

        // Get the device enumerator
        let device_enumerator: IMMDeviceEnumerator =
            CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL).map_err(|e| {
                eprintln!("Failed to create device enumerator: {:?}", e);
                e
            })?;

        // Get the default audio endpoint (microphone)
        let default_microphone: IMMDevice = device_enumerator
            .GetDefaultAudioEndpoint(eCapture, eConsole)
            .unwrap();

        // Get the volume level
        let default_microphone_volume_level: IAudioEndpointVolume = default_microphone.Activate(CLSCTX_ALL, None)?;

        // Find the Krisp microphone
        let krisp_microphone = find_krisp_microphone(&device_enumerator);
        let krisp_microphone_volume_level = krisp_microphone.map(|microphone| microphone.Activate::<IAudioEndpointVolume>(CLSCTX_ALL, None).unwrap());

        while running.load(Ordering::SeqCst) {
            // Set default microphone volume to 100%
            let current_volume: f32 = default_microphone_volume_level.GetMasterVolumeLevelScalar()?;
            if current_volume < 1.0 {
                default_microphone_volume_level.SetMasterVolumeLevelScalar(1.0, std::ptr::null())?;
                println!("Default Microphone volume set to 100%");
            }

            // Set Krisp microphone volume to 100% if it exists
            if let Some(krisp_microphone_volume_level) = &krisp_microphone_volume_level {
                let current_volume: f32 = krisp_microphone_volume_level.GetMasterVolumeLevelScalar()?;
                if current_volume < 1.0 {
                    krisp_microphone_volume_level.SetMasterVolumeLevelScalar(1.0, std::ptr::null())?;
                    println!("Krisp Microphone volume set to 100%");
                }
            }

            thread::sleep(Duration::from_millis(config.time * 1000));
        }

        println!("Exiting...");
        CoUninitialize();
    }

    Ok(())
}

unsafe fn find_krisp_microphone(device_enumerator: &IMMDeviceEnumerator) -> Option<IMMDevice> {
    let collection = device_enumerator.EnumAudioEndpoints(eCapture, DEVICE_STATE_ACTIVE).ok()?;
    let count = collection.GetCount().ok()?;

    for i in 0..count {
        let device = collection.Item(i).ok()?;
        let property_store: IPropertyStore = device.OpenPropertyStore(STGM_READ).ok()?;

        if let Ok(prop_variant) = property_store.GetValue(&DEVPKEY_Device_FriendlyName as *const _ as *const _) {
            if let Ok(friendly_name_ptr) = PropVariantToStringAlloc(&prop_variant) {
                let friendly_name = friendly_name_ptr.to_string().unwrap_or_default();
                if friendly_name.to_lowercase().contains("krisp") {
                    return Some(device);
                }
            }
        }
    }
    None
}