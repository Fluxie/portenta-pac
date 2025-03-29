///Register `PSR` reader
pub type R = crate::R<PsrSpec>;
///Field `LMON` reader - ET0_LINKSTA Pin Status Flag
pub type LmonR = crate::BitReader;
impl R {
    ///Bit 0 - ET0_LINKSTA Pin Status Flag
    #[inline(always)]
    pub fn lmon(&self) -> LmonR {
        LmonR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSR").field("lmon", &self.lmon()).finish()
    }
}
/**PHY Status Register

You can [`read`](crate::Reg::read) this register and get [`psr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PsrSpec;
impl crate::RegisterSpec for PsrSpec {
    type Ux = u32;
}
///`read()` method returns [`psr::R`](R) reader structure
impl crate::Readable for PsrSpec {}
///`reset()` method sets PSR to value 0
impl crate::Resettable for PsrSpec {}
