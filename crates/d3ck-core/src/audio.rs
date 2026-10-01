use getset::{CopyGetters, Getters};

use crate::{ProcId, error::VolumeError};

pub type AudioSessionId = u32;

#[allow(dead_code)]
#[derive(Getters, CopyGetters)]
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
    fn volume(&self, id: AudioSessionId) -> Option<Volume>;
    fn set_volume(&self, id: AudioSessionId, volume: Volume);

    fn is_muted(&self, id: AudioSessionId) -> Option<bool>;
    fn set_muted(&self, id: AudioSessionId, state: bool);
}


pub type AudioDeviceId = u32;

#[derive(Getters, CopyGetters)]
#[allow(dead_code)]
pub struct AudioDevice {
    id: AudioDeviceId,
    name: String,
}

impl AudioDevice {
    pub fn new(id: AudioDeviceId, name: String) -> Self {
        Self { id, name }
    }
}

pub trait AudioProvider {
    fn devices(&self) -> Vec<AudioDevice>;
    fn sessions(&self) -> Vec<AudioSession>;
}