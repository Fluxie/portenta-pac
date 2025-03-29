///Register `SPSSR` reader
pub type R = crate::R<SpssrSpec>;
/**SPI Command Pointer

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Spcp {
    ///0: SPCMD0
    _000 = 0,
    ///1: SPCMD1
    _001 = 1,
    ///2: SPCMD2
    _010 = 2,
    ///3: SPCMD3
    _011 = 3,
    ///4: SPCMD4
    _100 = 4,
    ///5: SPCMD5
    _101 = 5,
    ///6: SPCMD6
    _110 = 6,
    ///7: SPCMD7
    _111 = 7,
}
impl From<Spcp> for u8 {
    #[inline(always)]
    fn from(variant: Spcp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Spcp {
    type Ux = u8;
}
impl crate::IsEnum for Spcp {}
///Field `SPCP` reader - SPI Command Pointer
pub type SpcpR = crate::FieldReader<Spcp>;
impl SpcpR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Spcp {
        match self.bits {
            0 => Spcp::_000,
            1 => Spcp::_001,
            2 => Spcp::_010,
            3 => Spcp::_011,
            4 => Spcp::_100,
            5 => Spcp::_101,
            6 => Spcp::_110,
            7 => Spcp::_111,
            _ => unreachable!(),
        }
    }
    ///SPCMD0
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Spcp::_000
    }
    ///SPCMD1
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Spcp::_001
    }
    ///SPCMD2
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Spcp::_010
    }
    ///SPCMD3
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Spcp::_011
    }
    ///SPCMD4
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Spcp::_100
    }
    ///SPCMD5
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Spcp::_101
    }
    ///SPCMD6
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Spcp::_110
    }
    ///SPCMD7
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == Spcp::_111
    }
}
/**SPI Error Command

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Specm {
    ///0: SPCMD0
    _000 = 0,
    ///1: SPCMD1
    _001 = 1,
    ///2: SPCMD2
    _010 = 2,
    ///3: SPCMD3
    _011 = 3,
    ///4: SPCMD4
    _100 = 4,
    ///5: SPCMD5
    _101 = 5,
    ///6: SPCMD6
    _110 = 6,
    ///7: SPCMD7
    _111 = 7,
}
impl From<Specm> for u8 {
    #[inline(always)]
    fn from(variant: Specm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Specm {
    type Ux = u8;
}
impl crate::IsEnum for Specm {}
///Field `SPECM` reader - SPI Error Command
pub type SpecmR = crate::FieldReader<Specm>;
impl SpecmR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Specm {
        match self.bits {
            0 => Specm::_000,
            1 => Specm::_001,
            2 => Specm::_010,
            3 => Specm::_011,
            4 => Specm::_100,
            5 => Specm::_101,
            6 => Specm::_110,
            7 => Specm::_111,
            _ => unreachable!(),
        }
    }
    ///SPCMD0
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Specm::_000
    }
    ///SPCMD1
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Specm::_001
    }
    ///SPCMD2
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Specm::_010
    }
    ///SPCMD3
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Specm::_011
    }
    ///SPCMD4
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Specm::_100
    }
    ///SPCMD5
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Specm::_101
    }
    ///SPCMD6
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Specm::_110
    }
    ///SPCMD7
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == Specm::_111
    }
}
impl R {
    ///Bits 0:2 - SPI Command Pointer
    #[inline(always)]
    pub fn spcp(&self) -> SpcpR {
        SpcpR::new(self.bits & 7)
    }
    ///Bits 4:6 - SPI Error Command
    #[inline(always)]
    pub fn specm(&self) -> SpecmR {
        SpecmR::new((self.bits >> 4) & 7)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPSSR")
            .field("spcp", &self.spcp())
            .field("specm", &self.specm())
            .finish()
    }
}
/**SPI Sequence Status Register

You can [`read`](crate::Reg::read) this register and get [`spssr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SpssrSpec;
impl crate::RegisterSpec for SpssrSpec {
    type Ux = u8;
}
///`read()` method returns [`spssr::R`](R) reader structure
impl crate::Readable for SpssrSpec {}
///`reset()` method sets SPSSR to value 0
impl crate::Resettable for SpssrSpec {}
