pub mod audio;
#[doc(inline)]
pub use audio::*;

pub type ProcId = u32;

pub struct Process {
    id: ProcId,
    name: String,
}

impl Process {
    pub fn id(&self) -> ProcId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

pub trait ProcessProvider {
    fn processes(&self) -> Vec<Process>;
    fn process(&self, id: ProcId) -> Option<Process>;
}
