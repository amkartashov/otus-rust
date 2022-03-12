use std::sync::{Arc, Mutex, MutexGuard};
use time::{Duration, Instant};

/// thread-safe struct
#[derive(Clone)]
pub(crate) struct Smartsocket {
    state: Arc<Mutex<State>>,
}

struct State {
    is_on: bool,
    power: u32,
    updated_at: Instant,
    /// load_func should convert time to consumed power
    load_func: Box<dyn Fn(Duration) -> u32 + Send>,
}

impl Smartsocket {
    pub(crate) fn new<F: 'static + Send + Fn(time::Duration) -> u32>(load_func: F) -> Self {
        Self {
            state: Arc::new(Mutex::new(State {
                is_on: false,
                power: 0,
                updated_at: Instant::now(),
                load_func: Box::new(load_func),
            })),
        }
    }

    /// update power usage and return state mutex guard
    fn update(&self) -> MutexGuard<State> {
        let mut state = self.state.lock().unwrap();
        if state.is_on {
            let period = state.updated_at.elapsed();
            state.power += (state.load_func)(period);
            state.updated_at += period;
        };
        state
    }

    pub(crate) fn state(&self) -> (bool, u32) {
        let state = self.update();
        (state.is_on, state.power)
    }

    pub(crate) fn switch(&self, on: bool) {
        self.update().is_on = on;
    }
}
