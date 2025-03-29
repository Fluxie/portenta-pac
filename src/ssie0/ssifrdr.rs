///Register `SSIFRDR` reader
pub type R = crate::R<SsifrdrSpec>;
///Field `SSIFRDR` reader - Receive FIFO Data
pub type SsifrdrR = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Receive FIFO Data
    #[inline(always)]
    pub fn ssifrdr(&self) -> SsifrdrR {
        SsifrdrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SSIFRDR").field("ssifrdr", &self.ssifrdr()).finish()
    }
}
/**Receive FIFO Data Register

You can [`read`](crate::Reg::read) this register and get [`ssifrdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SsifrdrSpec;
impl crate::RegisterSpec for SsifrdrSpec {
    type Ux = u32;
}
///`read()` method returns [`ssifrdr::R`](R) reader structure
impl crate::Readable for SsifrdrSpec {}
///`reset()` method sets SSIFRDR to value 0
impl crate::Resettable for SsifrdrSpec {}
