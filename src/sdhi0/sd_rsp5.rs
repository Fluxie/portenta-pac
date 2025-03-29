///Register `SD_RSP5` reader
pub type R = crate::R<SdRsp5Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
/**SD Card Response Register 5

You can [`read`](crate::Reg::read) this register and get [`sd_rsp5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SdRsp5Spec;
impl crate::RegisterSpec for SdRsp5Spec {
    type Ux = u32;
}
///`read()` method returns [`sd_rsp5::R`](R) reader structure
impl crate::Readable for SdRsp5Spec {}
///`reset()` method sets SD_RSP5 to value 0
impl crate::Resettable for SdRsp5Spec {}
