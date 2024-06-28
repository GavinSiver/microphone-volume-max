use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::thread;
use std::time::Duration;

use clap::Parser;
use windows::Win32::Media::Audio::{
    eCapture, eConsole, Endpoints::IAudioEndpointVolume, IMMDevice, IMMDeviceEnumerator,
    MMDeviceEnumerator,
};
use windows::Win32::System::Com::*;

#[derive(Parser)]
#[command(name = "Windows Microphone Volume Max")]
#[command(
    version,
    about = "Set the microphone volume to 100% on a recurring basis.",
    long_about = "Finds the default microphone, and sets its volume to 100%; repeats this action every T seconds. Default is 60 seconds."
)]
struct Config {
    /// Time in Seconds to wait before checking the volume again
    #[arg(short, long, default_value = "60", value_parser = clap::value_parser!(u64).range(1..))]
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
        CoInitializeEx(Some(std::ptr::null_mut()), COINIT_MULTITHREADED)?;

        // Get the device enumerator
        let device_enumerator: IMMDeviceEnumerator =
            CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)?;

        // Get the default audio endpoint (microphone)
        let device: IMMDevice = device_enumerator
            .GetDefaultAudioEndpoint(eCapture, eConsole)
            .unwrap();

        // Get the volume level
        let endpoint_volume_level: IAudioEndpointVolume = device.Activate(CLSCTX_ALL, None)?;

        while running.load(Ordering::SeqCst) {
            // Get the current volume
            let current_volume: f32 = endpoint_volume_level.GetMasterVolumeLevelScalar()?;

            if current_volume < 1.0 {
                endpoint_volume_level.SetMasterVolumeLevelScalar(1.0, std::ptr::null())?;
                println!("Microphone volume set to 100%");
            }

            thread::sleep(Duration::from_millis(config.time * 1000));
        }

        println!("Exiting...");
        CoUninitialize();
    }

    Ok(())
}
