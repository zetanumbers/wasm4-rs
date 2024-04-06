//! # Examples
//!
//! ```no_run
#![doc = include_str!("../examples/ticking.rs")]
//! ```
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]
// TODO: #![deny(missing_docs)]

#[doc(hidden)]
#[path = "private.rs"]
pub mod __private;
pub mod draw;
pub mod rt;
pub mod sound;
mod utils;

pub use self::utils::OutOfDomainError;
pub use wasm4_sys as sys;

pub fn trace(msg: &str) {
    unsafe { sys::traceUtf8(msg.as_ptr(), msg.len()) }
}
