#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BehaviorState { Idle, Patrol, Chase, Attack, Flee }

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AiAgent {
    pub state: BehaviorState,
    pub detection_radius: f32,
    pub attack_radius: f32,
    pub health_fraction: f32,
}

impl Default for AiAgent {
    fn default() -> Self { Self { state: BehaviorState::Idle, detection_radius: 20.0, attack_radius: 2.0, health_fraction: 1.0 } }
}

impl AiAgent {
    pub fn decide(&mut self, distance_to_target: Option<f32>) -> BehaviorState {
        self.state = if self.health_fraction < 0.2 { BehaviorState::Flee }
        else if let Some(d) = distance_to_target {
            if d <= self.attack_radius { BehaviorState::Attack }
            else if d <= self.detection_radius { BehaviorState::Chase }
            else { BehaviorState::Patrol }
        } else { BehaviorState::Patrol };
        self.state
    }
}

#[cfg(test)]
mod tests { use super::*; #[test] fn deterministic_decision() { let mut a=AiAgent::default(); assert_eq!(a.decide(Some(1.0)), BehaviorState::Attack); assert_eq!(a.decide(Some(10.0)), BehaviorState::Chase); } }
