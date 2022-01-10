// TODO: #![deny(missing_docs)]
//! # Examples
//!
//! ```no_run
//! #![no_main]
//!
//! use wasm4 as w4;
//!
//! struct MyRuntime {
//!     count: i32,
//! }
//!
//! // prints "tick" every second
//! impl w4::rt::Runtime for MyRuntime {
//!     fn start(_: w4::rt::Resources) -> Self {
//!         MyRuntime { count: 0 }
//!     }
//!
//!     fn update(&mut self) {
//!         if self.count % 60 == 0 {
//!             w4::trace("tick");
//!             self.count = 0;
//!         }
//!         self.count += 1;
//!     }
//! }
//!
//! w4::main! { MyRuntime }
//! ```

pub mod draw;
pub mod rt;
pub mod sound;
mod utils;

pub use self::utils::OutOfDomainError;
pub use wasm4_sys as sys;

pub fn trace(msg: &str) {
    unsafe { sys::traceUtf8(msg.as_ptr(), msg.len()) }
}
