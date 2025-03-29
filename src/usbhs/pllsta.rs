///Register `PLLSTA` reader
pub type R = crate::R<PllstaSpec>;
/**PLL Lock Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Plllock {
    ///0: PLL not locked
    _0 = 0,
    ///1: PLL locked
    _1 = 1,
}
impl From<Plllock> for bool {
    #[inline(always)]
    fn from(variant: Plllock) -> Self {
        variant as u8 != 0
    }
}
///Field `PLLLOCK` reader - PLL Lock Flag
pub type PlllockR = crate::BitReader<Plllock>;
impl PlllockR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Plllock {
        match self.bits {
            false => Plllock::_0,
            true => Plllock::_1,
        }
    }
    ///PLL not locked
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Plllock::_0
    }
    ///PLL locked
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Plllock::_1
    }
}
impl R {
    ///Bit 0 - PLL Lock Flag
    #[inline(always)]
    pub fn plllock(&self) -> PlllockR {
        PlllockR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLLSTA").field("plllock", &self.plllock()).finish()
    }
}
/**PLL Status Register

You can [`read`](crate::Reg::read) this register and get [`pllsta::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PllstaSpec;
impl crate::RegisterSpec for PllstaSpec {
    type Ux = u16;
}
///`read()` method returns [`pllsta::R`](R) reader structure
impl crate::Readable for PllstaSpec {}
///`reset()` method sets PLLSTA to value 0
impl crate::Resettable for PllstaSpec {}
