use core::mem;

pub mod draw;
pub mod sound;
pub mod prelude {
    pub use crate::{draw, sound, Field};
}

pub use wasm4_sys as sys;

pub fn trace(msg: &str) {
    unsafe { sys::traceUtf8(msg.as_ptr(), msg.len()) }
}

pub trait Field<T>: Sized {
    fn with(self, value: T) -> Self;

    fn get(&self) -> T;

    fn set<'a>(&'a mut self, value: T) {
        // SAFETY: MaybeUninit<Self> has the same layout as Self
        let this: &'a mut mem::MaybeUninit<Self> = unsafe { mem::transmute(self) };

        let old_self = mem::replace(this, mem::MaybeUninit::uninit());

        // SAFETY: self was previously initialized
        let new_self = unsafe { old_self.assume_init() }.with(value);

        // self is initialized
        let _this: &mut Self = this.write(new_self);
    }
}
