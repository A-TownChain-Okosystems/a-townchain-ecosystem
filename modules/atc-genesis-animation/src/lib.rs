#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AnimationClipId(pub u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AnimationStateId(pub u32);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AnimationPlayer {
    pub clip: Option<AnimationClipId>,
    pub time_seconds: f32,
    pub speed: f32,
    pub looping: bool,
}

impl Default for AnimationPlayer {
    fn default() -> Self {
        Self { clip: None, time_seconds: 0.0, speed: 1.0, looping: true }
    }
}

impl AnimationPlayer {
    pub fn update(&mut self, dt_seconds: f32, duration_seconds: f32) {
        self.time_seconds += dt_seconds.max(0.0) * self.speed;
        if duration_seconds > 0.0 && self.looping {
            self.time_seconds %= duration_seconds;
        }
    }
}
