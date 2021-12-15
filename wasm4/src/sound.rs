use crate::Field;
use core::{fmt, mem};

#[derive(PartialEq, Eq)]
pub struct Resource {
    #[doc(hidden)]
    __field: (),
}

static mut SINGLETON: Option<Resource> = Some(Resource::new_());

impl Resource {
    const fn new_() -> Self {
        Resource { __field: () }
    }

    pub fn take() -> Option<Self> {
        // SAFETY: wasm4 is single-threaded
        unsafe { SINGLETON.take() }
    }

    /// Plays a sound tone. Volume is between 0 and 100.
    pub fn tone(&self, frequency: Frequency, duration: Duration, volume: u32, flags: Flags) {
        // SAFETY: calling extern function; no syncronization is required
        unsafe { wasm4_sys::tone(frequency.inner(), duration.inner(), volume, flags.inner()) }
    }

    pub fn leak(self) -> &'static Self {
        static mut SOUND_LEAK: mem::MaybeUninit<Resource> = mem::MaybeUninit::uninit();

        // SAFETY: wasm4 is single-threaded
        unsafe { SOUND_LEAK.write(self) }
    }
}

impl fmt::Debug for Resource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Audio")
    }
}

#[derive(Clone, Copy)]
pub struct Flags(u32);

impl Flags {
    pub fn new() -> Self {
        Flags(0)
    }

    pub fn inner(self) -> u32 {
        self.0
    }
}

#[derive(Clone, Copy)]
pub struct Channel(u32);

impl Channel {
    /// Returns channel from 0 to 3
    pub fn new(value: u32) -> Option<Self> {
        (value < 4).then(|| Self(value))
    }

    pub fn inner(self) -> u32 {
        self.0
    }
}

impl Field<Channel> for Flags {
    fn with(self, value: Channel) -> Self {
        Flags(self.0 & !0b0011 | value.0)
    }

    fn get(&self) -> Channel {
        Channel(self.0 & 0b0011)
    }
}

impl Channel {
    pub const PULSE1: Self = Channel(0);
    pub const PULSE2: Self = Channel(1);
    pub const TRIANGLE: Self = Channel(2);
    pub const NOISE: Self = Channel(3);
}

/// For pulse channels, the pulse wave duty cycle.
#[derive(Clone, Copy)]
pub struct Mode(u32);

impl Mode {
    /// Returns mode from 0 to 3
    pub fn new(mode: u32) -> Option<Self> {
        (mode < 4).then(|| Mode(mode))
    }

    pub fn inner(self) -> u32 {
        self.0
    }
}

impl Field<Mode> for Flags {
    fn with(self, mode: Mode) -> Self {
        Flags(self.0 & !0b1100 | mode.0 << 2)
    }

    fn get(&self) -> Mode {
        Mode((self.0 & 0b1100) >> 2)
    }
}

impl Mode {
    /// 1/8
    pub const N_1_D_8: Self = Mode(0);
    /// 1/4
    pub const N_1_D_4: Self = Mode(1);
    /// 1/2
    pub const N_1_D_2: Self = Mode(2);
    /// 3/4
    pub const N_3_D_4: Self = Mode(3);
}

#[derive(Clone, Copy)]
pub struct Frequency(u32);

impl Frequency {
    /// Zero frequency
    pub fn new() -> Self {
        Frequency(0)
    }

    pub fn inner(self) -> u32 {
        self.0
    }
}

/// Wave frequency in hertz
#[derive(Clone, Copy)]
pub struct StartFrequency(pub u16);

impl StartFrequency {
    /// Returns mode from 0 to 3
    pub fn new(value: u16) -> Self {
        StartFrequency(value)
    }

    pub fn inner(self) -> u16 {
        self.0
    }
}

impl Field<StartFrequency> for Frequency {
    fn with(self, value: StartFrequency) -> Self {
        Frequency(self.0 & !0x0000ffff | u32::from(value.0) << 16)
    }

    fn get(&self) -> StartFrequency {
        StartFrequency((self.0 & 0x0000ffff) as u16)
    }
}

/// Wave frequency in hertz
#[derive(Clone, Copy)]
pub struct EndFrequency(pub u16);

impl EndFrequency {
    pub fn new(value: u16) -> Self {
        EndFrequency(value)
    }

    pub fn inner(self) -> u16 {
        self.0
    }
}

impl Field<EndFrequency> for Frequency {
    fn with(self, value: EndFrequency) -> Self {
        Frequency(self.0 & !0xffff0000 | u32::from(value.0) << 16)
    }

    fn get(&self) -> EndFrequency {
        EndFrequency((self.0 >> 16) as u16)
    }
}

/// Duration of the tone
#[derive(Clone, Copy)]
pub struct Duration(u32);

impl Duration {
    pub fn new() -> Self {
        Duration(0)
    }

    pub fn inner(self) -> u32 {
        self.0
    }
}

/// Sustain time of the tone in frames (1/60th of a second), up to 255 frames
#[derive(Clone, Copy)]
pub struct SustainTime(pub u8);

impl SustainTime {
    pub fn new(value: u8) -> Self {
        SustainTime(value)
    }

    pub fn inner(self) -> u8 {
        self.0
    }
}

impl Field<SustainTime> for Frequency {
    fn with(self, value: SustainTime) -> Self {
        Frequency(self.0 & !0x000000ff | u32::from(value.0))
    }

    fn get(&self) -> SustainTime {
        SustainTime((self.0 & 0x000000ff) as u8)
    }
}

/// Release time of the tone in frames (1/60th of a second), up to 255 frames
#[derive(Clone, Copy)]
pub struct ReleaseTime(pub u8);

impl ReleaseTime {
    pub fn new(value: u8) -> Self {
        ReleaseTime(value)
    }

    pub fn inner(self) -> u8 {
        self.0
    }
}

impl Field<ReleaseTime> for Frequency {
    fn with(self, value: ReleaseTime) -> Self {
        Frequency(self.0 & !0x0000ff00 | u32::from(value.0) << 8)
    }

    fn get(&self) -> ReleaseTime {
        ReleaseTime(((self.0 & 0x0000ff00) >> 8) as u8)
    }
}

/// Decay time of the tone in frames (1/60th of a second), up to 255 frames
#[derive(Clone, Copy)]
pub struct DecayTime(pub u8);

impl DecayTime {
    pub fn new(value: u8) -> Self {
        DecayTime(value)
    }

    pub fn inner(self) -> u8 {
        self.0
    }
}

impl Field<DecayTime> for Frequency {
    fn with(self, value: DecayTime) -> Self {
        Frequency(self.0 & !0x00ff0000 | u32::from(value.0) << 16)
    }

    fn get(&self) -> DecayTime {
        DecayTime(((self.0 & 0x00ff0000) >> 16) as u8)
    }
}

/// Attack time of the tone in frames (1/60th of a second), up to 255 frames
#[derive(Clone, Copy)]
pub struct AttackTime(pub u8);

impl AttackTime {
    pub fn new(value: u8) -> Self {
        AttackTime(value)
    }

    pub fn inner(self) -> u8 {
        self.0
    }
}

impl Field<AttackTime> for Frequency {
    fn with(self, value: AttackTime) -> Self {
        Frequency(self.0 & !0xff000000 | u32::from(value.0) << 24)
    }

    fn get(&self) -> AttackTime {
        AttackTime(((self.0 & 0xff000000) >> 24) as u8)
    }
}
