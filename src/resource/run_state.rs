#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunState {
    Paused,
    Running,
}

impl RunState {
    pub fn is_running(&self) -> bool {
        matches!(self, RunState::Running)
    }

    pub fn is_paused(&self) -> bool {
        matches!(self, RunState::Paused)
    }

    pub fn pause(&mut self) {
        *self = RunState::Paused;
    }

    pub fn run(&mut self) {
        *self = RunState::Running;
    }
}
