use crate::utils::{checked_add_pairs, le_pairs, lt_pairs};
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

    pub fn blit(&self, sprite: &impl Blit, start: [i32; 2], transform: BlitTransform) {
        sprite.blit(start, transform, self)
    }
}

#[derive(Clone, Copy)]
pub struct Sprite<Bytes: ?Sized = [u8]> {
    shape: [u32; 2],
    bpp: BitsPerPixel,
    bytes: Bytes,
}

impl<const N: usize> Sprite<[u8; N]> {
    pub const fn from_bytes(bytes: [u8; N], shape: [u32; 2], bpp: BitsPerPixel) -> Option<Self> {
        if match shape[0].checked_mul(shape[1]) {
            Some(it) => it as usize,
            None => return None,
        } < match N.checked_shl(3 - bpp as u32) {
            Some(it) => it,
            None => return None,
        } {
            Some(Sprite { shape, bpp, bytes })
        } else {
            None
        }
    }

    pub const unsafe fn from_bytes_unchecked(
        bytes: [u8; N],
        shape: [u32; 2],
        bpp: BitsPerPixel,
    ) -> Self {
        Sprite { shape, bpp, bytes }
    }
}

impl Sprite {
    /// Get the sprite's shape.
    pub const fn shape(&self) -> [u32; 2] {
        self.shape
    }

    /// Get the sprite's width.
    pub const fn width(&self) -> u32 {
        self.shape[0]
    }

    /// Get the sprite's height.
    pub const fn height(&self) -> u32 {
        self.shape[1]
    }

    /// Get the sprite's bpp (bits per pixel).
    pub const fn bpp(&self) -> BitsPerPixel {
        self.bpp
    }

    /// Get a reference to the sprite's bytes.
    pub const fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Create a subview to the sprite. Returns `None` if subview is out of bounds of the sprite.
    pub const fn view(&self, start: [u32; 2], shape: [u32; 2]) -> Option<SpriteView<'_>> {
        if lt_pairs(start, self.shape)
            && le_pairs(
                match checked_add_pairs(start, shape) {
                    Some(it) => it,
                    None => return None,
                },
                self.shape,
            )
        {
            Some(SpriteView {
                sprite: self,
                start,
                shape,
            })
        } else {
            None
        }
    }

    /// Create a subview to the sprite. Does not perform in bounds checks.
    ///
    /// # Safety
    ///
    /// Resulting subview should be inside of the bounds of the `Sprite`
    pub const unsafe fn view_unchecked<'a>(
        &self,
        start: [u32; 2],
        shape: [u32; 2],
    ) -> SpriteView<'_> {
        SpriteView {
            sprite: self,
            start,
            shape,
        }
    }
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BitsPerPixel {
    One,
    Two,
}

bitflags::bitflags! {
    #[derive(Default)]
    pub struct BlitTransform: u32 {
        const FLIP_X = 0b0010;
        const FLIP_Y = 0b0100;
        const ROTATE = 0b1000;
    }
}

#[derive(Clone, Copy)]
pub struct SpriteView<'a> {
    sprite: &'a Sprite,
    start: [u32; 2],
    shape: [u32; 2],
}

impl<'a> SpriteView<'a> {
    /// Get the view's underlying sprite.
    pub fn sprite(&self) -> &'a Sprite {
        self.sprite
    }

    /// Get the sprite view's start.
    pub fn start(&self) -> [u32; 2] {
        self.start
    }

    /// Get the sprite view's shape.
    pub fn shape(&self) -> [u32; 2] {
        self.shape
    }
}

pub trait Blit {
    fn blit(&self, start: [i32; 2], transform: BlitTransform, _framebuffer: &Framebuffer);
}

impl Blit for Sprite {
    fn blit(&self, start: [i32; 2], transform: BlitTransform, _framebuffer: &Framebuffer) {
        let flags = self.bpp as u32 | transform.bits();
        unsafe {
            wasm4_sys::blit(
                self.bytes.as_ptr(),
                start[0],
                start[1],
                self.shape[0],
                self.shape[1],
                flags,
            )
        }
    }
}

impl Blit for SpriteView<'_> {
    fn blit(&self, start: [i32; 2], transform: BlitTransform, _framebuffer: &Framebuffer) {
        let flags = self.sprite.bpp as u32 | transform.bits();
        unsafe {
            wasm4_sys::blitSub(
                self.sprite.bytes.as_ptr(),
                start[0],
                start[1],
                self.shape[0],
                self.shape[1],
                self.start[0],
                self.start[1],
                self.sprite.shape[0],
                flags,
            )
        }
    }
}
