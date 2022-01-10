// TODO: #![deny(missing_docs)]
//! # Examples
//!
//! ```no_run
//! #![no_main]
//!
//! struct MyRuntime {
//!     count: i32,
//! }
//!
//! // prints "tick..." every second
//! impl wasm4::Runtime for MyRuntime {
//!     fn start(_: wasm4::Resources) -> Self {
//!         MyRuntime { count: 0 }
//!     }
//!
//!     fn update(&mut self) {
//!         if self.count % 60 == 0 {
//!             wasm4::trace("tick");
//!             self.count = 0;
//!         }
//!         self.count += 1;
//!     }
//! }
//!
//! wasm4::main! { MyRuntime }
//! ```

pub mod draw;
pub mod runtime;
pub mod sound;
mod utils;

pub use self::{
    draw::{Framebuffer, Sprite},
    runtime::*,
    sound::Audio,
    utils::OutOfDomainError,
};
pub use wasm4_sys as sys;

pub fn trace(msg: &str) {
    unsafe { sys::traceUtf8(msg.as_ptr(), msg.len()) }
}
