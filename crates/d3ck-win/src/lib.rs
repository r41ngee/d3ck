pub struct WindowsAudio {
    sessions: Vec<d3ck_core::AudioSession>,
    devices: Vec<d3ck_core::AudioDevice>,
    processes: Vec<d3ck_core::Process>,
}