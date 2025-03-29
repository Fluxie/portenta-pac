///Register `SD_RSP3` reader
pub type R = crate::R<SdRsp3Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
/**SD Card Response Register 3

You can [`read`](crate::Reg::read) this register and get [`sd_rsp3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SdRsp3Spec;
impl crate::RegisterSpec for SdRsp3Spec {
    type Ux = u32;
}
///`read()` method returns [`sd_rsp3::R`](R) reader structure
impl crate::Readable for SdRsp3Spec {}
///`reset()` method sets SD_RSP3 to value 0
impl crate::Resettable for SdRsp3Spec {}
