use atc_genesis_ecs::World;
use atc_genesis_platform::{EntityId, Transform};
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TimeStep {
    pub fixed_seconds: f32,
    pub frame: u64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SpawnedEntity(pub EntityId);
pub trait GamePlugin {
    fn name(&self) -> &'static str;
    fn startup(&mut self, _ctx: &mut GameContext) {}
    fn update(&mut self, _ctx: &mut GameContext, _time: TimeStep) {}
    fn shutdown(&mut self, _ctx: &mut GameContext) {}
}
#[derive(Default)]
pub struct GameContext {
    pub world: World,
    frame: u64,
}
impl GameContext {
    pub fn new() -> Self {
        Self {
            world: World::new(),
            frame: 0,
        }
    }
    pub fn spawn(&mut self, transform: Transform) -> SpawnedEntity {
        SpawnedEntity(self.world.spawn(transform))
    }
    pub fn transform(&self, id: EntityId) -> Option<Transform> {
        self.world.world_transform(id)
    }
    pub fn set_transform(&mut self, id: EntityId, t: Transform) -> bool {
        self.world.set_transform(id, t)
    }
    pub fn despawn(&mut self, id: EntityId) -> bool {
        self.world.despawn(id)
    }
    pub fn entity_count(&self) -> usize {
        self.world.len()
    }
    pub fn frame(&self) -> u64 {
        self.frame
    }
    pub fn advance_frame(&mut self) {
        self.frame = self.frame.saturating_add(1)
    }
}
pub struct PluginRuntime {
    plugins: Vec<Box<dyn GamePlugin>>,
    started: bool,
}
impl Default for PluginRuntime {
    fn default() -> Self {
        Self {
            plugins: Vec::new(),
            started: false,
        }
    }
}
impl PluginRuntime {
    pub fn add<P: GamePlugin + 'static>(&mut self, plugin: P) {
        self.plugins.push(Box::new(plugin))
    }
    pub fn startup(&mut self, ctx: &mut GameContext) {
        if self.started {
            return;
        }
        for p in &mut self.plugins {
            p.startup(ctx)
        }
        self.started = true
    }
    pub fn update(&mut self, ctx: &mut GameContext, dt: f32) {
        if !self.started {
            self.startup(ctx)
        }
        let time = TimeStep {
            fixed_seconds: dt.max(0.0),
            frame: ctx.frame(),
        };
        for p in &mut self.plugins {
            p.update(ctx, time)
        }
        ctx.advance_frame()
    }
    pub fn shutdown(&mut self, ctx: &mut GameContext) {
        if !self.started {
            return;
        }
        for p in self.plugins.iter_mut().rev() {
            p.shutdown(ctx)
        }
        self.started = false
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn entities_get_stable_ids() {
        let mut c = GameContext::new();
        assert_eq!(c.spawn(Transform::default()).0, EntityId(1));
        assert_eq!(c.spawn(Transform::default()).0, EntityId(2));
    }
    struct P {
        updates: u32,
    }
    impl GamePlugin for P {
        fn name(&self) -> &'static str {
            "p"
        }
        fn update(&mut self, _: &mut GameContext, _: TimeStep) {
            self.updates += 1
        }
    }
    #[test]
    fn plugin_runtime_updates() {
        let mut c = GameContext::new();
        let mut r = PluginRuntime::default();
        r.add(P { updates: 0 });
        r.update(&mut c, 1.0 / 60.0);
        assert_eq!(c.frame(), 1);
    }
}


/// Allow-listed control protocol for external AI agents such as Aurora.
pub mod control {
    use std::fmt;

    #[derive(Clone, Debug, PartialEq)]
    pub enum ControlCommand {
        Status,
        Spawn { x: f32, y: f32 },
        Destroy { entity_id: u64 },
        SetPosition { entity_id: u64, x: f32, y: f32 },
        Tick { frames: u32, dt: f32 },
        Snapshot,
        Reset,
        Quit,
    }

    impl ControlCommand {
        pub fn encode(&self) -> String {
            match self {
                Self::Status => "STATUS".into(),
                Self::Spawn { x, y } => format!("SPAWN {x} {y}"),
                Self::Destroy { entity_id } => format!("DESTROY {entity_id}"),
                Self::SetPosition { entity_id, x, y } => format!("SET_POSITION {entity_id} {x} {y}"),
                Self::Tick { frames, dt } => format!("TICK {frames} {dt}"),
                Self::Snapshot => "SNAPSHOT".into(),
                Self::Reset => "RESET".into(),
                Self::Quit => "QUIT".into(),
            }
        }
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct ControlResponse {
        pub line: String,
    }

    impl ControlResponse {
        pub fn parse(line: impl Into<String>) -> Result<Self, ControlError> {
            let line = line.into();
            if line.starts_with("OK ") {
                Ok(Self { line })
            } else if line.starts_with("ERR ") {
                Err(ControlError::Engine(line))
            } else {
                Err(ControlError::Protocol(line))
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum ControlError {
        Engine(String),
        Protocol(String),
    }

    impl fmt::Display for ControlError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::Engine(v) => write!(f, "Genesis Engine rejected command: {v}"),
                Self::Protocol(v) => write!(f, "Genesis control protocol error: {v}"),
            }
        }
    }

    impl std::error::Error for ControlError {}
}
