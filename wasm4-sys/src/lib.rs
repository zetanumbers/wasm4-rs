//! # Safety
//!
//! Many seemingly safe functions are marked unsafe.
//! The reason for this is that the crate [`wasm4`](https://crates.io/crates/wasm4)
//! uses them to achieve safe, idiomatic, and zero-overhead api.
//! The major capability of that crate is being able to restrict ownership of
//! various resources like framebuffer, sound, etc.
//! To achieve that it implements WASM-4 api via methods on a struct like
//! [`wasm4::sound::Resouce`](https://docs.rs/wasm4/latest/wasm4/sound/struct.Resource.html),
//! ownership of which is restricted until you share it.
//! As you may guess, calling [`crate::tone`] may interfere with assumptions about
//! which sounds are playing in a code using `wasm4::sound::Resource`.
//! Use of raw bindings usually gives little to no benefit, but is possible if you
//! respect these assumptions described above.
//!
//! Some of these functions are unsafe for other reasons too, like raw memory access.
//!
//! However all of that assumes you or any dependency do not use any other bindings
//! except for [`wasm4`](https://crates.io/crates/wasm4) and this [`crate`]
//! (or uses them while respecting assumptions mentioned above, but this is discouraged).
#![no_std]

use core::ffi::c_void;

pub const SCREEN_SIZE: u32 = 160;

pub const PALETTE: *mut [u32; 4] = 0x04 as *mut [u32; 4];
pub const DRAW_COLORS: *mut u16 = 0x14 as *mut u16;
pub const GAMEPAD1: *const u8 = 0x16 as *const u8;
pub const GAMEPAD2: *const u8 = 0x17 as *const u8;
pub const GAMEPAD3: *const u8 = 0x18 as *const u8;
pub const GAMEPAD4: *const u8 = 0x19 as *const u8;
pub const MOUSE_X: *const i16 = 0x1a as *const i16;
pub const MOUSE_Y: *const i16 = 0x1c as *const i16;
pub const MOUSE_BUTTONS: *const u8 = 0x1e as *const u8;
pub const SYSTEM_FLAGS: *mut u8 = 0x1f as *mut u8;
pub const FRAMEBUFFER: *mut [u8; 6400] = 0xa0 as *mut [u8; 6400];

pub const BUTTON_1: u8 = 1;
pub const BUTTON_2: u8 = 2;
pub const BUTTON_LEFT: u8 = 16;
pub const BUTTON_RIGHT: u8 = 32;
pub const BUTTON_UP: u8 = 64;
pub const BUTTON_DOWN: u8 = 128;

pub const MOUSE_LEFT: u8 = 1;
pub const MOUSE_RIGHT: u8 = 2;
pub const MOUSE_MIDDLE: u8 = 4;

pub const SYSTEM_PRESERVE_FRAMEBUFFER: u8 = 1;
pub const SYSTEM_HIDE_GAMEPAD_OVERLAY: u8 = 2;

pub const BLIT_2BPP: u32 = 1;
pub const BLIT_1BPP: u32 = 0;
pub const BLIT_FLIP_X: u32 = 2;
pub const BLIT_FLIP_Y: u32 = 4;
pub const BLIT_ROTATE: u32 = 8;

pub const TONE_PULSE1: u32 = 0;
pub const TONE_PULSE2: u32 = 1;
pub const TONE_TRIANGLE: u32 = 2;
pub const TONE_NOISE: u32 = 3;
pub const TONE_MODE1: u32 = 0;
pub const TONE_MODE2: u32 = 4;
pub const TONE_MODE3: u32 = 8;
pub const TONE_MODE4: u32 = 12;

extern "C" {
    pub fn rect(x: i32, y: i32, width: u32, height: u32);
    pub fn oval(x: i32, y: i32, width: u32, height: u32);
    pub fn line(x1: i32, y1: i32, x2: i32, y2: i32);

    pub fn hline(x: i32, y: i32, len: u32);
    pub fn vline(x: i32, y: i32, len: u32);

    pub fn text(text: *const u8, x: i32, y: i32);
    pub fn textUtf8(text: *const u8, byte_length: usize, x: i32, y: i32);
    pub fn textUtf16(text: *const u16, byte_length: usize, x: i32, y: i32);

    pub fn blit(sprite: *const u8, x: i32, y: i32, width: u32, height: u32, flags: u32);
    pub fn blitSub(
        sprite: *const u8,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        src_x: u32,
        src_y: u32,
        stride: u32,
        flags: u32,
    );

    pub fn tone(frequency: u32, duration: u32, volume: u32, flags: u32);

    pub fn diskr(dest: *mut u8, size: usize) -> u32;
    pub fn diskw(src: *const u8, size: usize) -> u32;

    pub fn trace(trace: *const u8);
    pub fn traceUtf8(trace: *const u8, byte_length: usize);
    pub fn traceUtf16(trace: *const u16, byte_length: usize);
    pub fn tracef(fmt: *const u8, stack_ptr: *const c_void);

}
