///Register `RHRCP%s` reader
pub type R = crate::R<RhrcpSpec>;
///Field `HR1` reader - 1-Hour Capture
pub type Hr1R = crate::FieldReader;
///Field `HR10` reader - 10-Hour Capture
pub type Hr10R = crate::FieldReader;
/**PM

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pm {
    ///0: AM
    _0 = 0,
    ///1: PM
    _1 = 1,
}
impl From<Pm> for bool {
    #[inline(always)]
    fn from(variant: Pm) -> Self {
        variant as u8 != 0
    }
}
///Field `PM` reader - PM
pub type PmR = crate::BitReader<Pm>;
impl PmR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pm {
        match self.bits {
            false => Pm::_0,
            true => Pm::_1,
        }
    }
    ///AM
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pm::_0
    }
    ///PM
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pm::_1
    }
}
impl R {
    ///Bits 0:3 - 1-Hour Capture
    #[inline(always)]
    pub fn hr1(&self) -> Hr1R {
        Hr1R::new(self.bits & 0x0f)
    }
    ///Bits 4:5 - 10-Hour Capture
    #[inline(always)]
    pub fn hr10(&self) -> Hr10R {
        Hr10R::new((self.bits >> 4) & 3)
    }
    ///Bit 6 - PM
    #[inline(always)]
    pub fn pm(&self) -> PmR {
        PmR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RHRCP")
            .field("hr1", &self.hr1())
            .field("hr10", &self.hr10())
            .field("pm", &self.pm())
            .finish()
    }
}
/**Hour Capture Register %s

You can [`read`](crate::Reg::read) this register and get [`rhrcp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RhrcpSpec;
impl crate::RegisterSpec for RhrcpSpec {
    type Ux = u8;
}
///`read()` method returns [`rhrcp::R`](R) reader structure
impl crate::Readable for RhrcpSpec {}
///`reset()` method sets RHRCP%s to value 0
impl crate::Resettable for RhrcpSpec {}
