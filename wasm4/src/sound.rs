use crate::utils::OutOfDomainError;
use core::mem;

#[derive(PartialEq, Eq)]
pub struct Resource(());

impl Resource {
    pub fn take() -> Option<Self> {
        static mut SINGLETON: Option<Resource> = Some(Resource(()));

        // SAFETY: wasm4 is single-threaded
        unsafe { SINGLETON.take() }
    }

    /// Plays a sound tone. Volume is between 0 and 100.
    pub fn tone(&self, frequency: LinearFrequency, duration: Duration, volume: u32, flags: Flags) {
        // SAFETY: calling extern function, makes ownership assumptions useful
        unsafe { wasm4_sys::tone(frequency.inner(), duration.inner(), volume, flags.inner()) }
    }

    pub fn share(self) -> &'static Self {
        &Resource(())
    }
}

#[derive(Clone, Copy)]
pub struct Flags(u32);

impl Flags {
    pub const fn new(channel: Channel, mode: Mode) -> Self {
        Flags((mode as u32) << 2 | (channel as u32))
    }

    pub const fn inner(self) -> u32 {
        self.0
    }

    pub const fn channel(self) -> Channel {
        // SAFETY: `Channel` is `repr(u32)` and defined for every 2 bit value
        unsafe { mem::transmute(self.0 & 0b11) }
    }

    pub const fn mode(self) -> Mode {
        // SAFETY: `Mode` is `repr(u32)` and defined for every 2 bit value, and `Flags` holds only 4 bit values
        unsafe { mem::transmute(self.0 >> 2) }
    }

    pub const fn with_channel(self, value: Channel) -> Self {
        Flags(self.0 & !0b0011 | (value as u32))
    }

    pub const fn with_mode(self, mode: Mode) -> Self {
        Flags(self.0 & !0b1100 | (mode as u32) << 2)
    }
}

/// Channel from 0 to 3
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Channel {
    Pulse1,
    Pulse2,
    Triangle,
    Noise,
}

/// For pulse channels, the pulse wave duty cycle.
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Mode {
    /// 1/8
    N1D8,
    /// 1/4
    N1D4,
    /// 1/2
    N1D2,
    /// 3/4
    N3D4,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct LinearFrequency(pub u32);

impl LinearFrequency {
    /// Frequency does not change over time
    pub fn constant(freq: Frequency) -> Self {
        LinearFrequency(freq.0)
    }

    /// Frequency linearly changes from the `start` frequency to the `end`
    pub fn linear(start: Frequency, end: Frequency) -> Self {
        LinearFrequency(start.0 | end.0 << 16)
    }

    pub fn inner(self) -> u32 {
        self.0
    }

    pub fn start(self) -> Frequency {
        Frequency(self.0 & 0x0000ffff)
    }

    pub fn end(self) -> Frequency {
        Frequency(self.0 >> 16)
    }

    pub fn with_start(self, freq: Frequency) -> Self {
        LinearFrequency(self.0 & !0x0000ffff | freq.0)
    }

    pub fn with_end(self, freq: Frequency) -> Self {
        LinearFrequency(self.0 & !0xffff0000 | freq.0 << 16)
    }
}

/// Wave frequency in hertz
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Frequency(u32);

impl From<u16> for Frequency {
    fn from(value: u16) -> Self {
        Frequency(u32::from(value))
    }
}

impl TryFrom<u32> for Frequency {
    type Error = OutOfDomainError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value < 0x10000 {
            Ok(Frequency(value))
        } else {
            Err(OutOfDomainError(()))
        }
    }
}

impl From<Frequency> for u32 {
    fn from(value: Frequency) -> Self {
        value.0
    }
}

impl From<Frequency> for u16 {
    fn from(value: Frequency) -> Self {
        value.0 as u16
    }
}

/// Duration of the tone
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct Duration(pub u32);

impl Duration {
    pub fn inner(self) -> u32 {
        self.0
    }

    pub fn sustain(self) -> Frames {
        Frames(self.0 & 0xff)
    }

    pub fn release(self) -> Frames {
        Frames(self.0 >> 8 & 0xff)
    }

    pub fn decay(self) -> Frames {
        Frames(self.0 >> 16 & 0xff)
    }

    pub fn attack(&self) -> Frames {
        Frames(self.0 >> 24)
    }

    pub fn with_sustain(self, value: Frames) -> Self {
        Duration(self.0 & !0x000000ff | value.0)
    }

    pub fn with_release(self, value: Frames) -> Self {
        Duration(self.0 & !0x0000ff00 | value.0 << 8)
    }

    pub fn with_decay(self, value: Frames) -> Self {
        Duration(self.0 & !0x00ff0000 | value.0 << 16)
    }

    pub fn with_attack(self, value: Frames) -> Self {
        Duration(self.0 & !0xff000000 | value.0 << 24)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Frames(u32);

impl Frames {
    pub fn inner(self) -> u32 {
        self.0
    }
}

impl From<u8> for Frames {
    fn from(value: u8) -> Self {
        Frames(u32::from(value))
    }
}

impl TryFrom<u32> for Frames {
    type Error = OutOfDomainError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value < 0x100 {
            Ok(Frames(value))
        } else {
            Err(OutOfDomainError(()))
        }
    }
}

impl From<Frames> for u32 {
    fn from(value: Frames) -> Self {
        value.0
    }
}

impl From<Frames> for u8 {
    fn from(value: Frames) -> Self {
        value.0 as u8
    }
}
