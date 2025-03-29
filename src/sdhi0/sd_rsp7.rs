///Register `SD_RSP7` reader
pub type R = crate::R<SdRsp7Spec>;
///Field `SD_RSP7` reader - These bits store the response from the SD card/MMC.
pub type SdRsp7R = crate::FieldReader;
impl R {
    ///Bits 0:7 - These bits store the response from the SD card/MMC.
    #[inline(always)]
    pub fn sd_rsp7(&self) -> SdRsp7R {
        SdRsp7R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SD_RSP7").field("sd_rsp7", &self.sd_rsp7()).finish()
    }
}
/**SD Card Response Register 7

You can [`read`](crate::Reg::read) this register and get [`sd_rsp7::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SdRsp7Spec;
impl crate::RegisterSpec for SdRsp7Spec {
    type Ux = u32;
}
///`read()` method returns [`sd_rsp7::R`](R) reader structure
impl crate::Readable for SdRsp7Spec {}
///`reset()` method sets SD_RSP7 to value 0
impl crate::Resettable for SdRsp7Spec {}
