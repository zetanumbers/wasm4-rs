// TODO: #![deny(missing_docs)]
//! # Examples
//!
//! ```no_run
#![doc = include_str!("../../examples/count/src/main.rs")]
//! ```

pub mod draw;
pub mod prelude;
pub mod runtime;
pub mod sound;
pub mod utils;

pub use runtime::*;
pub use wasm4_sys as sys;

pub fn trace(msg: &str) {
    unsafe { sys::traceUtf8(msg.as_ptr(), msg.len()) }
}
