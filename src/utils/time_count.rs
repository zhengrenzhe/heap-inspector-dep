use std::time::{Duration, SystemTime};

pub struct TimeCount {
    start: SystemTime,
}

impl TimeCount {
    pub fn start() -> Self {
        TimeCount {
            start: SystemTime::now(),
        }
    }

    pub fn end(&self) -> Duration {
        SystemTime::now().duration_since(self.start).unwrap()
    }
}
