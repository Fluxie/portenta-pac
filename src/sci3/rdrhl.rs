///Register `RDRHL` reader
pub type R = crate::R<RdrhlSpec>;
///Field `RDAT` reader - Serial Receive Data
pub type RdatR = crate::FieldReader<u16>;
impl R {
    ///Bits 0:8 - Serial Receive Data
    #[inline(always)]
    pub fn rdat(&self) -> RdatR {
        RdatR::new(self.bits & 0x01ff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDRHL").field("rdat", &self.rdat()).finish()
    }
}
/**Receive Data Register for Non-Manchester mode (MMR.MANEN = 0)

You can [`read`](crate::Reg::read) this register and get [`rdrhl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RdrhlSpec;
impl crate::RegisterSpec for RdrhlSpec {
    type Ux = u16;
}
///`read()` method returns [`rdrhl::R`](R) reader structure
impl crate::Readable for RdrhlSpec {}
///`reset()` method sets RDRHL to value 0
impl crate::Resettable for RdrhlSpec {}
