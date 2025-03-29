///Register `SFMSST` reader
pub type R = crate::R<SfmsstSpec>;
/**Number of bytes of prefetched data

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pfcnt {
    ///0: 0 byte
    _0x00 = 0,
    ///1: 1 byte
    _0x01 = 1,
    ///2: 2 bytes
    _0x02 = 2,
    ///3: 3 bytes
    _0x03 = 3,
    ///4: 4 bytes
    _0x04 = 4,
    ///5: 5 bytes
    _0x05 = 5,
    ///6: 6 bytes
    _0x06 = 6,
    ///7: 7 bytes
    _0x07 = 7,
    ///8: 8 bytes
    _0x08 = 8,
    ///9: 9 bytes
    _0x09 = 9,
    ///10: 10 bytes
    _0x0a = 10,
    ///11: 11 bytes
    _0x0b = 11,
    ///12: 12 bytes
    _0x0c = 12,
    ///13: 13 bytes
    _0x0d = 13,
    ///14: 14 bytes
    _0x0e = 14,
    ///15: 15 bytes
    _0x0f = 15,
    ///16: 16 bytes
    _0x10 = 16,
    ///17: 17 bytes
    _0x11 = 17,
    ///18: 18 bytes
    _0x12 = 18,
    ///19: Reserved
    Others = 19,
}
impl From<Pfcnt> for u8 {
    #[inline(always)]
    fn from(variant: Pfcnt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pfcnt {
    type Ux = u8;
}
impl crate::IsEnum for Pfcnt {}
///Field `PFCNT` reader - Number of bytes of prefetched data
pub type PfcntR = crate::FieldReader<Pfcnt>;
impl PfcntR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pfcnt {
        match self.bits {
            0 => Pfcnt::_0x00,
            1 => Pfcnt::_0x01,
            2 => Pfcnt::_0x02,
            3 => Pfcnt::_0x03,
            4 => Pfcnt::_0x04,
            5 => Pfcnt::_0x05,
            6 => Pfcnt::_0x06,
            7 => Pfcnt::_0x07,
            8 => Pfcnt::_0x08,
            9 => Pfcnt::_0x09,
            10 => Pfcnt::_0x0a,
            11 => Pfcnt::_0x0b,
            12 => Pfcnt::_0x0c,
            13 => Pfcnt::_0x0d,
            14 => Pfcnt::_0x0e,
            15 => Pfcnt::_0x0f,
            16 => Pfcnt::_0x10,
            17 => Pfcnt::_0x11,
            18 => Pfcnt::_0x12,
            _ => Pfcnt::Others,
        }
    }
    ///0 byte
    #[inline(always)]
    pub fn is_0x00(&self) -> bool {
        *self == Pfcnt::_0x00
    }
    ///1 byte
    #[inline(always)]
    pub fn is_0x01(&self) -> bool {
        *self == Pfcnt::_0x01
    }
    ///2 bytes
    #[inline(always)]
    pub fn is_0x02(&self) -> bool {
        *self == Pfcnt::_0x02
    }
    ///3 bytes
    #[inline(always)]
    pub fn is_0x03(&self) -> bool {
        *self == Pfcnt::_0x03
    }
    ///4 bytes
    #[inline(always)]
    pub fn is_0x04(&self) -> bool {
        *self == Pfcnt::_0x04
    }
    ///5 bytes
    #[inline(always)]
    pub fn is_0x05(&self) -> bool {
        *self == Pfcnt::_0x05
    }
    ///6 bytes
    #[inline(always)]
    pub fn is_0x06(&self) -> bool {
        *self == Pfcnt::_0x06
    }
    ///7 bytes
    #[inline(always)]
    pub fn is_0x07(&self) -> bool {
        *self == Pfcnt::_0x07
    }
    ///8 bytes
    #[inline(always)]
    pub fn is_0x08(&self) -> bool {
        *self == Pfcnt::_0x08
    }
    ///9 bytes
    #[inline(always)]
    pub fn is_0x09(&self) -> bool {
        *self == Pfcnt::_0x09
    }
    ///10 bytes
    #[inline(always)]
    pub fn is_0x0a(&self) -> bool {
        *self == Pfcnt::_0x0a
    }
    ///11 bytes
    #[inline(always)]
    pub fn is_0x0b(&self) -> bool {
        *self == Pfcnt::_0x0b
    }
    ///12 bytes
    #[inline(always)]
    pub fn is_0x0c(&self) -> bool {
        *self == Pfcnt::_0x0c
    }
    ///13 bytes
    #[inline(always)]
    pub fn is_0x0d(&self) -> bool {
        *self == Pfcnt::_0x0d
    }
    ///14 bytes
    #[inline(always)]
    pub fn is_0x0e(&self) -> bool {
        *self == Pfcnt::_0x0e
    }
    ///15 bytes
    #[inline(always)]
    pub fn is_0x0f(&self) -> bool {
        *self == Pfcnt::_0x0f
    }
    ///16 bytes
    #[inline(always)]
    pub fn is_0x10(&self) -> bool {
        *self == Pfcnt::_0x10
    }
    ///17 bytes
    #[inline(always)]
    pub fn is_0x11(&self) -> bool {
        *self == Pfcnt::_0x11
    }
    ///18 bytes
    #[inline(always)]
    pub fn is_0x12(&self) -> bool {
        *self == Pfcnt::_0x12
    }
    ///Reserved
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Pfcnt::Others)
    }
}
/**Prefetch buffer state

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pfful {
    ///0: Prefetch buffer has free space
    _0 = 0,
    ///1: Prefetch buffer is full
    _1 = 1,
}
impl From<Pfful> for bool {
    #[inline(always)]
    fn from(variant: Pfful) -> Self {
        variant as u8 != 0
    }
}
///Field `PFFUL` reader - Prefetch buffer state
pub type PffulR = crate::BitReader<Pfful>;
impl PffulR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pfful {
        match self.bits {
            false => Pfful::_0,
            true => Pfful::_1,
        }
    }
    ///Prefetch buffer has free space
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pfful::_0
    }
    ///Prefetch buffer is full
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pfful::_1
    }
}
/**Prefetch function operating state

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pfoff {
    ///0: Prefetch function operating
    _0 = 0,
    ///1: Prefetch function not enabled or not operating
    _1 = 1,
}
impl From<Pfoff> for bool {
    #[inline(always)]
    fn from(variant: Pfoff) -> Self {
        variant as u8 != 0
    }
}
///Field `PFOFF` reader - Prefetch function operating state
pub type PfoffR = crate::BitReader<Pfoff>;
impl PfoffR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pfoff {
        match self.bits {
            false => Pfoff::_0,
            true => Pfoff::_1,
        }
    }
    ///Prefetch function operating
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pfoff::_0
    }
    ///Prefetch function not enabled or not operating
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pfoff::_1
    }
}
impl R {
    ///Bits 0:4 - Number of bytes of prefetched data
    #[inline(always)]
    pub fn pfcnt(&self) -> PfcntR {
        PfcntR::new((self.bits & 0x1f) as u8)
    }
    ///Bit 6 - Prefetch buffer state
    #[inline(always)]
    pub fn pfful(&self) -> PffulR {
        PffulR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Prefetch function operating state
    #[inline(always)]
    pub fn pfoff(&self) -> PfoffR {
        PfoffR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SFMSST")
            .field("pfcnt", &self.pfcnt())
            .field("pfful", &self.pfful())
            .field("pfoff", &self.pfoff())
            .finish()
    }
}
/**Status Register

You can [`read`](crate::Reg::read) this register and get [`sfmsst::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SfmsstSpec;
impl crate::RegisterSpec for SfmsstSpec {
    type Ux = u32;
}
///`read()` method returns [`sfmsst::R`](R) reader structure
impl crate::Readable for SfmsstSpec {}
///`reset()` method sets SFMSST to value 0x80
impl crate::Resettable for SfmsstSpec {
    const RESET_VALUE: u32 = 0x80;
}
