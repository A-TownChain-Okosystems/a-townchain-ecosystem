//! Deterministic priority scheduler for userspace processes.
//!
//! Highest priority is selected first. Processes with equal priority are
//! scheduled round-robin by process id order so a runnable process cannot be
//! selected forever merely because it sorts first.

use crate::{ProcessId, ProcessInfo, ProcessState};

#[derive(Debug, Default)]
pub struct Scheduler {
    processes: Vec<ProcessInfo>,
    current: Option<ProcessId>,
}

impl Scheduler {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, process: ProcessInfo) {
        if self.processes.iter().any(|p| p.id == process.id) {
            return;
        }
        self.processes.push(process);
        self.sort_processes();
    }

    pub fn set_state(&mut self, id: ProcessId, state: ProcessState) -> bool {
        let Some(process) = self.processes.iter_mut().find(|p| p.id == id) else {
            return false;
        };
        process.state = state;
        if state == ProcessState::Exited && self.current == Some(id) {
            self.current = None;
        }
        true
    }

    pub fn next(&mut self) -> Option<ProcessId> {
        let max_priority = self
            .processes
            .iter()
            .filter(|p| matches!(p.state, ProcessState::Ready | ProcessState::Running))
            .map(|p| p.priority)
            .max()?;

        let mut candidates: Vec<ProcessId> = self
            .processes
            .iter()
            .filter(|p| {
                p.priority == max_priority
                    && matches!(p.state, ProcessState::Ready | ProcessState::Running)
            })
            .map(|p| p.id)
            .collect();
        candidates.sort_unstable();

        let next = match self.current {
            Some(current) => candidates
                .iter()
                .copied()
                .find(|id| *id > current)
                .or_else(|| candidates.first().copied()),
            None => candidates.first().copied(),
        };

        self.current = next;
        if let Some(id) = next {
            for process in &mut self.processes {
                if process.id == id {
                    process.state = ProcessState::Running;
                } else if process.state == ProcessState::Running {
                    process.state = ProcessState::Ready;
                }
            }
        }
        next
    }

    pub fn current(&self) -> Option<ProcessId> {
        self.current
    }

    pub fn processes(&self) -> &[ProcessInfo] {
        &self.processes
    }

    fn sort_processes(&mut self) {
        self.processes
            .sort_by(|a, b| b.priority.cmp(&a.priority).then_with(|| a.id.cmp(&b.id)));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn process(id: u64, priority: u8) -> ProcessInfo {
        ProcessInfo {
            id: ProcessId(id),
            state: ProcessState::Ready,
            priority,
        }
    }

    #[test]
    fn selects_highest_priority_deterministically() {
        let mut s = Scheduler::new();
        s.register(process(2, 10));
        s.register(process(1, 10));
        assert_eq!(s.next(), Some(ProcessId(1)));
    }

    #[test]
    fn rotates_equal_priority_processes() {
        let mut s = Scheduler::new();
        s.register(process(1, 10));
        s.register(process(2, 10));
        s.register(process(3, 10));

        assert_eq!(s.next(), Some(ProcessId(1)));
        assert_eq!(s.next(), Some(ProcessId(2)));
        assert_eq!(s.next(), Some(ProcessId(3)));
        assert_eq!(s.next(), Some(ProcessId(1)));
    }

    #[test]
    fn higher_priority_preempts_lower_priority() {
        let mut s = Scheduler::new();
        s.register(process(1, 1));
        s.register(process(2, 2));
        assert_eq!(s.next(), Some(ProcessId(2)));
    }

    #[test]
    fn exited_current_process_is_cleared() {
        let mut s = Scheduler::new();
        s.register(process(1, 10));
        assert_eq!(s.next(), Some(ProcessId(1)));
        assert!(s.set_state(ProcessId(1), ProcessState::Exited));
        assert_eq!(s.current(), None);
    }
}
