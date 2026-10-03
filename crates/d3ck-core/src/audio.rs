use crate::{ProcId, error::VolumeError};

pub type AudioSessionId = u32;

pub struct AudioSession {
    id: AudioSessionId,
    process_id: Option<ProcId>,
    volume: Volume,
    muted: bool,
}

impl AudioSession {
    pub fn new(id: AudioSessionId, process_id: Option<ProcId>, volume: Volume, muted: bool) -> Self {
        Self { id, process_id, volume, muted }
    }
}

impl AudioSession {
    pub fn id(&self) -> AudioSessionId {
        self.id
    }

    pub fn process_id(&self) -> Option<ProcId> {
        self.process_id
    }

    pub fn volume(&self) -> &Volume {
        &self.volume
    }

    pub fn volume_mut(&mut self) -> &mut Volume {
        &mut self.volume
    }

    pub fn muted(&self) -> bool {
        self.muted
    }

    pub fn muted_mut(&mut self) -> &mut bool {
        &mut self.muted
    }
}

pub struct Volume(f32);
impl Volume {
    fn check_acceptable_value(value: &f32) -> bool {
        (0f32..=1f32).contains(value)
    }

    pub fn new(value: f32) -> Result<Self, VolumeError> {
        if Self::check_acceptable_value(&value) {
            return Ok(Self(value))
        } else {
            return Err(VolumeError::OutOfBounds(value))
        }
    }

    pub fn value(&self) -> f32 {
        let value = self.0;
        if Self::check_acceptable_value(&value) {
            value
        } else { panic!("Volume(f32) is out of bounds") }
    }

    pub fn set_value(&mut self, value: f32) -> Result<(), VolumeError> {
        if Self::check_acceptable_value(&value) {
            self.0 = value;
            Ok(())
        } else {
            Err(VolumeError::OutOfBounds(value))
        }
    }
}

pub trait AudioSessionControl {
    fn session(&self, id: AudioSessionId) -> Option<&AudioSession>;
    fn session_mut(&mut self, id: AudioSessionId) -> Option<&mut AudioSession>;

    fn volume(&self, id: AudioSessionId) -> Option<&Volume> {
        Some(self.session(id)?.volume())
    }
    fn set_volume(&mut self, id: AudioSessionId, volume: Volume) -> bool {
        if let Some(s) = self.session_mut(id) {
            *s.volume_mut() = volume;
            return true;
        }
        return false;
    }

    fn is_muted(&self, id: AudioSessionId) -> Option<bool> {
        Some(self.session(id)?.muted())
    }
    fn set_muted(&mut self, id: AudioSessionId, state: bool) -> bool {
        if let Some(s) = self.session_mut(id) {
            *s.muted_mut() = state;
            return true;
        }
        return false;
    }

    type SystemError: std::error::Error;

    fn pull(&mut self) -> Result<(), Self::SystemError>;
    fn push(&self) -> Result<(), Self::SystemError>;
}


pub type AudioDeviceId = u32;

pub struct AudioDevice {
    id: AudioDeviceId,
    name: String,
}

impl AudioDevice {
    pub fn new(id: AudioDeviceId, name: String) -> Self {
        Self { id, name }
    }
}

impl AudioDevice {
    pub fn id(&self) -> AudioDeviceId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

pub trait AudioProvider {
    fn devices(&self) -> Vec<AudioDevice>;
    fn sessions(&self) -> Vec<AudioSession>;
}