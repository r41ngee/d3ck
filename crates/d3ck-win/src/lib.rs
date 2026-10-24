use windows::core::Result as WinResult;
use windows::Win32::System::Com::{CLSCTX_ALL, CoCreateInstance};
use windows::Win32::Media::Audio::{IMMDevice, IMMDeviceEnumerator, MMDeviceEnumerator, eConsole, eRender};

pub struct WindowsAudio {
    sessions: Vec<d3ck_core::AudioSession>,
    devices: Vec<d3ck_core::AudioDevice>,
    processes: Vec<d3ck_core::Process>,
}

fn create_device_enumerator() -> WinResult<IMMDeviceEnumerator> {
    unsafe {
        CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)
    }
}

fn get_audio_mgr() -> WinResult<IMMDevice> {
    let enumerator = create_device_enumerator()?;
    unsafe {
        enumerator.GetDefaultAudioEndpoint(eRender, eConsole)
    }
}