// TODO: #![deny(missing_docs)]

pub mod draw;
pub mod prelude;
pub mod sound;
pub mod utils;

pub use wasm4_sys as sys;

pub fn trace(msg: &str) {
    unsafe { sys::traceUtf8(msg.as_ptr(), msg.len()) }
}
