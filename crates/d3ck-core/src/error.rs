use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum VolumeError {
    OutOfBounds(f32)
}

impl Display for VolumeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VolumeError::OutOfBounds(v) => {
                write!(f, "Volume is out of bounds [0..=1] - value is {}", v)
            }
        }
    }
}

impl Error for VolumeError {
    
}