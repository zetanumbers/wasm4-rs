#[derive(Clone, Copy)]
pub struct Sprite<Bytes: ?Sized = [u8]> {
    shape: [u32; 2],
    bpp: BitsPerPixel,
    pub indices: DrawIndices,
    bytes: Bytes,
}

impl<const N: usize> Sprite<[u8; N]> {
    pub const fn from_byte_array(
        bytes: [u8; N],
        shape: [u32; 2],
        bpp: BitsPerPixel,
        draw_colors: DrawIndices,
    ) -> Option<Self> {
        let resolution = shape[0].checked_mul(shape[1]);
        let capacity = N.checked_mul(1 << 3 - bpp as u32);

        match (resolution, capacity) {
            // SAFETY: calling unsafe function after the check
            (Some(resolution), Some(capacity)) if resolution as usize <= capacity => unsafe {
                Some(Self::from_bytes_unchecked(bytes, shape, bpp, draw_colors))
            },
            _ => None,
        }
    }
}

impl<Bytes> Sprite<Bytes> {
    pub const unsafe fn from_bytes_unchecked(
        bytes: Bytes,
        shape: [u32; 2],
        bpp: BitsPerPixel,
        draw_colors: DrawIndices,
    ) -> Self {
        Sprite {
            shape,
            bpp,
            indices: draw_colors,
            bytes,
        }
    }
}

impl<Bytes: AsRef<[u8]>> Sprite<Bytes> {
    pub fn from_bytes(
        bytes: Bytes,
        shape: [u32; 2],
        bpp: BitsPerPixel,
        draw_colors: DrawIndices,
    ) -> Option<Self> {
        let resolution = shape[0].checked_mul(shape[1]);
        let capacity = bytes.as_ref().len().checked_mul(1 << 3 - bpp as u32);

        match (resolution, capacity) {
            // SAFETY: calling unsafe function after the check
            (Some(resolution), Some(capacity)) if resolution as usize <= capacity => unsafe {
                Some(Self::from_bytes_unchecked(bytes, shape, bpp, draw_colors))
            },
            _ => None,
        }
    }
}

impl<Bytes: ?Sized> Sprite<Bytes> {
    /// Get a reference to the sprite's bytes.
    pub const fn bytes(&self) -> &Bytes {
        &self.bytes
    }

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
}

impl Sprite {
    /// Create a subview to the sprite. Returns `None` if subview is out of bounds of the sprite.
    pub const fn view(&self, start: [u32; 2], shape: [u32; 2]) -> Option<SpriteView<'_>> {
        let dst_bottom_right = match checked_add_pairs(start, shape) {
            Some(it) => it,
            None => return None,
        };
        if lt_pairs(start, self.shape) && le_pairs(dst_bottom_right, self.shape) {
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

#[derive(Clone, Copy)]
pub struct SpriteView<'a> {
    sprite: &'a Sprite,
    start: [u32; 2],
    shape: [u32; 2],
}

impl<'a> SpriteView<'a> {
    /// Get the view's underlying sprite.
    pub const fn sprite(&self) -> &'a Sprite {
        self.sprite
    }

    /// Get the sprite view's start.
    pub const fn start(&self) -> [u32; 2] {
        self.start
    }

    /// Get the sprite view's shape.
    pub const fn shape(&self) -> [u32; 2] {
        self.shape
    }
}

#[derive(Clone, Copy, Default)]
pub struct DrawIndices(u16);

impl DrawIndices {
    pub const TRANSPARENT: Self = DrawIndices(0);

    pub const fn from_array(array: [DrawIndex; 4]) -> Self {
        DrawIndices(
            array[0] as u16
                | (array[1] as u16) << 4
                | (array[2] as u16) << 8
                | (array[3] as u16) << 12,
        )
    }

    pub const fn into_array(self) -> [DrawIndex; 4] {
        unsafe {
            [
                DrawIndex::new_unchecked(self.0 & 0xf),
                DrawIndex::new_unchecked(self.0 >> 4 & 0xf),
                DrawIndex::new_unchecked(self.0 >> 8 & 0xf),
                DrawIndex::new_unchecked(self.0 >> 12 & 0xf),
            ]
        }
    }

    pub const unsafe fn from_u16_unchecked(inner: u16) -> Self {
        DrawIndices(inner)
    }

    pub const fn into_u16(self) -> u16 {
        self.0
    }
}

impl From<DrawIndices> for u16 {
    fn from(v: DrawIndices) -> Self {
        v.0
    }
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum DrawIndex {
    Transparent,
    First,
    Second,
    Third,
    Fourth,
}

impl DrawIndex {
    pub const fn new(index: u16) -> Option<Self> {
        if index <= 4 {
            Some(unsafe { Self::new_unchecked(index) })
        } else {
            None
        }
    }

    pub const unsafe fn new_unchecked(index: u16) -> Self {
        core::mem::transmute(index)
    }
}

impl Default for DrawIndex {
    fn default() -> Self {
        DrawIndex::Transparent
    }
}

pub type Palette = [Color; 4];

/// Has `0x__RRGGBB` layout
#[repr(transparent)]
#[derive(Clone, Copy, Default)]
pub struct Color(pub u32);

impl Color {
    pub const BLACK: Self = Color(0);

    pub const fn blue(self) -> u8 {
        (self.0 & 0xff) as u8
    }

    pub const fn green(self) -> u8 {
        (self.0 >> 8 & 0xff) as u8
    }

    pub const fn red(self) -> u8 {
        (self.0 >> 16 & 0xff) as u8
    }

    pub const fn with_blue(self, channel: u8) -> Self {
        Color((self.0 & !0x0000ff) | (channel as u32))
    }

    pub const fn with_green(self, channel: u8) -> Self {
        Color((self.0 & !0x00ff00) | (channel as u32) << 8)
    }

    pub const fn with_red(self, channel: u8) -> Self {
        Color((self.0 & !0xff0000) | (channel as u32) << 16)
    }
}

// helpers

const fn checked_add_pairs(lhs: [u32; 2], rhs: [u32; 2]) -> Option<[u32; 2]> {
    match [lhs[0].checked_add(rhs[0]), lhs[1].checked_add(rhs[1])] {
        [Some(first), Some(second)] => Some([first, second]),
        _ => None,
    }
}

const fn lt_pairs(lhs: [u32; 2], rhs: [u32; 2]) -> bool {
    lhs[0] < rhs[0] && lhs[1] < rhs[1]
}

const fn le_pairs(lhs: [u32; 2], rhs: [u32; 2]) -> bool {
    lhs[0] <= rhs[0] && lhs[1] <= rhs[1]
}
