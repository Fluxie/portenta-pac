///Register `BUS%sERRSTAT` reader
pub type R = crate::R<BuserrstatSpec>;
/**Slave bus Error Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slerrstat {
    ///0: No error occurred
    _0 = 0,
    ///1: Error occurred
    _1 = 1,
}
impl From<Slerrstat> for bool {
    #[inline(always)]
    fn from(variant: Slerrstat) -> Self {
        variant as u8 != 0
    }
}
///Field `SLERRSTAT` reader - Slave bus Error Status
pub type SlerrstatR = crate::BitReader<Slerrstat>;
impl SlerrstatR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Slerrstat {
        match self.bits {
            false => Slerrstat::_0,
            true => Slerrstat::_1,
        }
    }
    ///No error occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Slerrstat::_0
    }
    ///Error occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Slerrstat::_1
    }
}
/**Slave TrustZone filter Error Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sterrstat {
    ///0: No error occurred
    _0 = 0,
    ///1: Error occurred
    _1 = 1,
}
impl From<Sterrstat> for bool {
    #[inline(always)]
    fn from(variant: Sterrstat) -> Self {
        variant as u8 != 0
    }
}
///Field `STERRSTAT` reader - Slave TrustZone filter Error Status
pub type SterrstatR = crate::BitReader<Sterrstat>;
impl SterrstatR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sterrstat {
        match self.bits {
            false => Sterrstat::_0,
            true => Sterrstat::_1,
        }
    }
    ///No error occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sterrstat::_0
    }
    ///Error occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sterrstat::_1
    }
}
/**Master MPU Error Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mmerrstat {
    ///0: No error occurred
    _0 = 0,
    ///1: Error occurred
    _1 = 1,
}
impl From<Mmerrstat> for bool {
    #[inline(always)]
    fn from(variant: Mmerrstat) -> Self {
        variant as u8 != 0
    }
}
///Field `MMERRSTAT` reader - Master MPU Error Status
pub type MmerrstatR = crate::BitReader<Mmerrstat>;
impl MmerrstatR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mmerrstat {
        match self.bits {
            false => Mmerrstat::_0,
            true => Mmerrstat::_1,
        }
    }
    ///No error occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mmerrstat::_0
    }
    ///Error occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mmerrstat::_1
    }
}
/**Illegal address access Error Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ilerrstat {
    ///0: No error occurred
    _0 = 0,
    ///1: Error occurred
    _1 = 1,
}
impl From<Ilerrstat> for bool {
    #[inline(always)]
    fn from(variant: Ilerrstat) -> Self {
        variant as u8 != 0
    }
}
///Field `ILERRSTAT` reader - Illegal address access Error Status
pub type IlerrstatR = crate::BitReader<Ilerrstat>;
impl IlerrstatR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ilerrstat {
        match self.bits {
            false => Ilerrstat::_0,
            true => Ilerrstat::_1,
        }
    }
    ///No error occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ilerrstat::_0
    }
    ///Error occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ilerrstat::_1
    }
}
impl R {
    ///Bit 0 - Slave bus Error Status
    #[inline(always)]
    pub fn slerrstat(&self) -> SlerrstatR {
        SlerrstatR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Slave TrustZone filter Error Status
    #[inline(always)]
    pub fn sterrstat(&self) -> SterrstatR {
        SterrstatR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - Master MPU Error Status
    #[inline(always)]
    pub fn mmerrstat(&self) -> MmerrstatR {
        MmerrstatR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Illegal address access Error Status
    #[inline(always)]
    pub fn ilerrstat(&self) -> IlerrstatR {
        IlerrstatR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BUSERRSTAT")
            .field("slerrstat", &self.slerrstat())
            .field("sterrstat", &self.sterrstat())
            .field("mmerrstat", &self.mmerrstat())
            .field("ilerrstat", &self.ilerrstat())
            .finish()
    }
}
/**BUS Error Status Register %s

You can [`read`](crate::Reg::read) this register and get [`buserrstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BuserrstatSpec;
impl crate::RegisterSpec for BuserrstatSpec {
    type Ux = u8;
}
///`read()` method returns [`buserrstat::R`](R) reader structure
impl crate::Readable for BuserrstatSpec {}
///`reset()` method sets BUS%sERRSTAT to value 0
impl crate::Resettable for BuserrstatSpec {}
