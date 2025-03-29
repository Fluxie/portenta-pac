///Register `CFDRMID%s_0` reader
pub type R = crate::R<Cfdrmid0Spec>;
///Field `RMID` reader - RX Message Buffer ID Field
pub type RmidR = crate::FieldReader<u32>;
/**RX Message Buffer RTR Bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rmrtr {
    ///0: Data frame
    _0 = 0,
    ///1: Remote frame
    _1 = 1,
}
impl From<Rmrtr> for bool {
    #[inline(always)]
    fn from(variant: Rmrtr) -> Self {
        variant as u8 != 0
    }
}
///Field `RMRTR` reader - RX Message Buffer RTR Bit
pub type RmrtrR = crate::BitReader<Rmrtr>;
impl RmrtrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rmrtr {
        match self.bits {
            false => Rmrtr::_0,
            true => Rmrtr::_1,
        }
    }
    ///Data frame
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rmrtr::_0
    }
    ///Remote frame
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rmrtr::_1
    }
}
/**RX Message Buffer IDE Bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rmide {
    ///0: STD-ID is stored
    _0 = 0,
    ///1: EXT-ID is stored
    _1 = 1,
}
impl From<Rmide> for bool {
    #[inline(always)]
    fn from(variant: Rmide) -> Self {
        variant as u8 != 0
    }
}
///Field `RMIDE` reader - RX Message Buffer IDE Bit
pub type RmideR = crate::BitReader<Rmide>;
impl RmideR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rmide {
        match self.bits {
            false => Rmide::_0,
            true => Rmide::_1,
        }
    }
    ///STD-ID is stored
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rmide::_0
    }
    ///EXT-ID is stored
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rmide::_1
    }
}
impl R {
    ///Bits 0:28 - RX Message Buffer ID Field
    #[inline(always)]
    pub fn rmid(&self) -> RmidR {
        RmidR::new(self.bits & 0x1fff_ffff)
    }
    ///Bit 30 - RX Message Buffer RTR Bit
    #[inline(always)]
    pub fn rmrtr(&self) -> RmrtrR {
        RmrtrR::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - RX Message Buffer IDE Bit
    #[inline(always)]
    pub fn rmide(&self) -> RmideR {
        RmideR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDRMID_0")
            .field("rmid", &self.rmid())
            .field("rmrtr", &self.rmrtr())
            .field("rmide", &self.rmide())
            .finish()
    }
}
/**RX Message Buffer ID Register %s Channel 0

You can [`read`](crate::Reg::read) this register and get [`cfdrmid_0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Cfdrmid0Spec;
impl crate::RegisterSpec for Cfdrmid0Spec {
    type Ux = u32;
}
///`read()` method returns [`cfdrmid_0::R`](R) reader structure
impl crate::Readable for Cfdrmid0Spec {}
///`reset()` method sets CFDRMID%s_0 to value 0
impl crate::Resettable for Cfdrmid0Spec {}
