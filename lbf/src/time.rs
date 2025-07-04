#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use js_sys::{Date, Reflect};
#[cfg(target_arch = "wasm32")]
use web_sys::Performance;
#[cfg(target_arch = "wasm32")]
use web_sys::js_sys;

use std::sync::LazyLock;

#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;

#[derive(Debug, Clone, Copy)]
pub enum TimeStamp {
    #[cfg(not(target_arch = "wasm32"))]
    Instant(Instant),

    #[cfg(target_arch = "wasm32")]
    Millis(f64),
}

impl TimeStamp {

    #[cfg(not(target_arch = "wasm32"))]
    pub fn as_instant(&self) -> Instant {
        match self {
            TimeStamp::Instant(i) => *i,
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn as_millis(&self) -> f64 {
        match self {
            TimeStamp::Millis(ms) => *ms,
        }
    }

    /// Get a new timestamp for "now"
    pub fn now() -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        {
            TimeStamp::Instant(Instant::now())
        }

        #[cfg(target_arch = "wasm32")]
        {
            TimeStamp::Millis(now_millis())
        }
    }

    /// Get elapsed milliseconds since this timestamp
    pub fn elapsed_ms(&self) -> f64 {
        match *self {
            #[cfg(not(target_arch = "wasm32"))]
            TimeStamp::Instant(t) => t.elapsed().as_secs_f64() * 1000.0,

            #[cfg(target_arch = "wasm32")]
            TimeStamp::Millis(start) => (now_millis() - start).max(0.0),
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    /// Returns the elapsed [`Duration`] since the timestamp.
    pub fn elapsed(&self) -> std::time::Duration {
        match self {
            TimeStamp::Instant(t) => t.elapsed(),
        }
    }

    /// Compute milliseconds elapsed between two timestamps
    pub fn since_ms(start: &Self, end: &Self) -> f64 {
        #[cfg(not(target_arch = "wasm32"))]
        {
            match (*start, *end) {
                (TimeStamp::Instant(a), TimeStamp::Instant(b)) => {
                    b.duration_since(a).as_secs_f64() * 1000.0
                }
            }
        }

        #[cfg(target_arch = "wasm32")]
        {
            match (*start, *end) {
                (TimeStamp::Millis(a), TimeStamp::Millis(b)) => (b - a).max(0.0),
            }
        }
    }
}


pub static EPOCH: LazyLock<TimeStamp> = LazyLock::new(TimeStamp::now);

#[cfg(target_arch = "wasm32")]
pub fn now_millis() -> f64 {
    let global = js_sys::global();

    let performance = Reflect::get(&global, &JsValue::from_str("performance"))
        .ok()
        .and_then(|p| p.dyn_into::<Performance>().ok());

    if let Some(perf) = performance {
        perf.now()
    } else {
        Date::now()
    }
}

#[cfg(target_arch = "wasm32")]
pub fn since_millis(start: f64) -> f64 {
    (now_millis() - start).max(0.0)
}
