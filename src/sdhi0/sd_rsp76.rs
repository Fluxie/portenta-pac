///Register `SD_RSP76` reader
pub type R = crate::R<SdRsp76Spec>;
///Field `SD_RSP76` reader - These bits store the response from the SD card/MMC.
pub type SdRsp76R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:23 - These bits store the response from the SD card/MMC.
    #[inline(always)]
    pub fn sd_rsp76(&self) -> SdRsp76R {
        SdRsp76R::new(self.bits & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SD_RSP76").field("sd_rsp76", &self.sd_rsp76()).finish()
    }
}
/**SD Card Response Register 76

You can [`read`](crate::Reg::read) this register and get [`sd_rsp76::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SdRsp76Spec;
impl crate::RegisterSpec for SdRsp76Spec {
    type Ux = u32;
}
///`read()` method returns [`sd_rsp76::R`](R) reader structure
impl crate::Readable for SdRsp76Spec {}
///`reset()` method sets SD_RSP76 to value 0
impl crate::Resettable for SdRsp76Spec {}
