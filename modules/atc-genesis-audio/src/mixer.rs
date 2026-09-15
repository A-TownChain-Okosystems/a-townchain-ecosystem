#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AudioVoiceId(pub u32);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AudioVoice { pub gain: f32, pub pitch: f32, pub looping: bool, pub playing: bool }

impl Default for AudioVoice { fn default() -> Self { Self { gain: 1.0, pitch: 1.0, looping: false, playing: false } } }

#[derive(Default)]
pub struct AudioMixer { voices: Vec<AudioVoice>, master_gain: f32 }
impl AudioMixer {
    pub fn new() -> Self { Self { voices: Vec::new(), master_gain: 1.0 } }
    pub fn play(&mut self, voice: AudioVoice) -> AudioVoiceId { let id = AudioVoiceId(self.voices.len() as u32); self.voices.push(AudioVoice { playing: true, ..voice }); id }
    pub fn stop(&mut self, id: AudioVoiceId) { if let Some(v) = self.voices.get_mut(id.0 as usize) { v.playing = false; } }
    pub fn set_master_gain(&mut self, gain: f32) { self.master_gain = gain.clamp(0.0, 4.0); }
    pub fn master_gain(&self) -> f32 { self.master_gain }
}
