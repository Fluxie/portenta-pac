///Register `RFCF` reader
pub type R = crate::R<RfcfSpec>;
///Field `RPAUSE` reader - Received PAUSE Frame Count
pub type RpauseR = crate::FieldReader;
impl R {
    ///Bits 0:7 - Received PAUSE Frame Count
    #[inline(always)]
    pub fn rpause(&self) -> RpauseR {
        RpauseR::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RFCF").field("rpause", &self.rpause()).finish()
    }
}
/**Received PAUSE Frame Counter

You can [`read`](crate::Reg::read) this register and get [`rfcf::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RfcfSpec;
impl crate::RegisterSpec for RfcfSpec {
    type Ux = u32;
}
///`read()` method returns [`rfcf::R`](R) reader structure
impl crate::Readable for RfcfSpec {}
///`reset()` method sets RFCF to value 0
impl crate::Resettable for RfcfSpec {}
