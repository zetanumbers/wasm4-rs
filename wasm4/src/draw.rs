use core::{cell::Cell, marker::PhantomData, mem};

pub struct Framebuffer(PhantomData<*mut ()>);

impl Framebuffer {
    pub(crate) unsafe fn new_() -> Self {
        Framebuffer(PhantomData)
    }

    pub const WIDTH: usize = 160;
    pub const HEIGHT: usize = 160;
    pub const BYTE_LENGTH: usize = Self::WIDTH * Self::HEIGHT / 4;

    pub fn as_cell(&self) -> &Cell<[u8; Self::BYTE_LENGTH]> {
        // SAFETY: WASM-4 is single-threaded
        unsafe { mem::transmute(wasm4_sys::FRAMEBUFFER) }
    }

    pub fn as_cells(&self) -> &[Cell<u8>; Self::BYTE_LENGTH] {
        // SAFETY: WASM-4 is single-threaded
        unsafe { mem::transmute(wasm4_sys::FRAMEBUFFER) }
    }

    pub fn line(&self, start: [i32; 2], end: [i32; 2]) {
        unsafe { wasm4_sys::line(start[0], start[1], end[0], end[0]) }
    }

    pub fn hline(&self, start: [i32; 2], len: u32) {
        unsafe { wasm4_sys::hline(start[0], start[1], len) }
    }

    pub fn vline(&self, start: [i32; 2], len: u32) {
        unsafe { wasm4_sys::vline(start[0], start[1], len) }
    }

    pub fn oval(&self, start: [i32; 2], shape: [u32; 2]) {
        unsafe { wasm4_sys::oval(start[0], start[1], shape[0], shape[1]) }
    }

    pub fn rect(&self, start: [i32; 2], shape: [u32; 2]) {
        unsafe { wasm4_sys::rect(start[0], start[1], shape[0], shape[1]) }
    }

    pub fn text(&self, text: &str, start: [i32; 2]) {
        unsafe { wasm4_sys::textUtf8(text.as_ptr(), text.len(), start[0], start[1]) }
    }
}
