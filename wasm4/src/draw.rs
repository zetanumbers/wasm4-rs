use core::{cell::Cell, marker::PhantomData, mem};

pub struct Framebuffer(PhantomData<*mut ()>);

impl Framebuffer {
    pub(crate) unsafe fn new_() -> Self {
        Framebuffer(PhantomData)
    }

    pub const WIDTH: usize = 160;
    pub const HEIGHT: usize = 160;
    pub const BYTE_LENGTH: usize = Self::WIDTH * Self::HEIGHT * 8 / 2;

    pub fn get(&self) -> &Cell<[u8; Self::BYTE_LENGTH]> {
        // SAFETY: WASM-4 is single-threaded
        unsafe { mem::transmute(wasm4_sys::FRAMEBUFFER) }
    }

    pub fn line(&self, x1: i32, y1: i32, x2: i32, y2: i32) {
        unsafe { wasm4_sys::line(x1, y1, x2, y2) }
    }

    pub fn hline(&self, x: i32, y: i32, len: u32) {
        unsafe { wasm4_sys::hline(x, y, len) }
    }

    pub fn vline(&self, x: i32, y: i32, len: u32) {
        unsafe { wasm4_sys::vline(x, y, len) }
    }

    pub fn oval(&self, x: i32, y: i32, width: u32, height: u32) {
        unsafe { wasm4_sys::oval(x, y, width, height) }
    }

    pub fn rect(&self, x: i32, y: i32, width: u32, height: u32) {
        unsafe { wasm4_sys::rect(x, y, width, height) }
    }

    pub fn text(&self, text: &str, x: i32, y: i32) {
        unsafe { wasm4_sys::textUtf8(text.as_ptr(), text.len(), x, y) }
    }
}
