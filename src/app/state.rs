use std::time::Duration;

pub enum AppState {
    Init,
    Initialized {
        duration: Duration,
        counter_tick: u64,
        console: Vec<String>,
    },
}

impl Default for AppState {
    fn default() -> Self {
        Self::Init
    }
}

impl AppState {
    pub fn initialized() -> Self {
        let duration = Duration::from_secs(1);
        let counter_tick = 0;
        Self::Initialized {
            duration,
            counter_tick,
            console: vec![],
        }
    }

    pub fn is_initialized(&self) -> bool {
        matches!(self, &Self::Initialized { .. })
    }

    pub fn incr_tick(&mut self) {
        if let Self::Initialized { counter_tick, .. } = self {
            *counter_tick += 1;
        }
    }

    pub fn count_tick(&self) -> Option<u64> {
        if let Self::Initialized { counter_tick, .. } = self {
            Some(*counter_tick)
        } else {
            None
        }
    }

    pub fn duration(&self) -> Option<&Duration> {
        if let Self::Initialized { duration, .. } = self {
            Some(duration)
        } else {
            None
        }
    }

    pub fn console(&self) -> Option<&Vec<String>> {
        if let Self::Initialized { console, .. } = self {
            Some(console)
        } else {
            None
        }
    }

    pub fn send_to_console(&mut self, content: String) {
        if let Self::Initialized { console, .. } = self {
            Some(console.push(content));
        }
    }

    pub fn increment_delay(&mut self) {
        if let Self::Initialized { duration, .. } = self {
            // Set the duration, note that the duration is in 1s..10s
            let secs = (duration.as_secs() + 1).clamp(1, 10);
            *duration = Duration::from_secs(secs);
        }
    }

    pub fn decrement_delay(&mut self) {
        if let Self::Initialized { duration, .. } = self {
            // Set the duration, note that the duration is in 1s..10s
            let secs = (duration.as_secs() - 1).clamp(1, 10);
            *duration = Duration::from_secs(secs);
        }
    }
}
