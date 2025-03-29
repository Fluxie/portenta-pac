///Register `CTSURC` reader
pub type R = crate::R<CtsurcSpec>;
///Field `CTSURC` reader - CTSU Reference Counter
pub type CtsurcR = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - CTSU Reference Counter
    #[inline(always)]
    pub fn ctsurc(&self) -> CtsurcR {
        CtsurcR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTSURC").field("ctsurc", &self.ctsurc()).finish()
    }
}
/**CTSU Reference Counter

You can [`read`](crate::Reg::read) this register and get [`ctsurc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CtsurcSpec;
impl crate::RegisterSpec for CtsurcSpec {
    type Ux = u16;
}
///`read()` method returns [`ctsurc::R`](R) reader structure
impl crate::Readable for CtsurcSpec {}
///`reset()` method sets CTSURC to value 0
impl crate::Resettable for CtsurcSpec {}
