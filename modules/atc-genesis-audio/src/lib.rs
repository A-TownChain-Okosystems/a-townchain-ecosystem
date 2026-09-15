use atc_genesis_platform::AudioRuntime;

#[derive(Default)]
pub struct NullAudioRuntime {
    master_gain: f32,
}

impl NullAudioRuntime {
    pub fn master_gain(&self) -> f32 { self.master_gain }
}

impl AudioRuntime for NullAudioRuntime {
    fn update(&mut self, _dt_seconds: f32) {}

    fn set_master_gain(&mut self, gain: f32) {
        self.master_gain = gain.clamp(0.0, 4.0);
    }
}
