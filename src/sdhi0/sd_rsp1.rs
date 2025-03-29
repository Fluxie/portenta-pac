///Register `SD_RSP1` reader
pub type R = crate::R<SdRsp1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
/**SD Card Response Register 1

You can [`read`](crate::Reg::read) this register and get [`sd_rsp1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SdRsp1Spec;
impl crate::RegisterSpec for SdRsp1Spec {
    type Ux = u32;
}
///`read()` method returns [`sd_rsp1::R`](R) reader structure
impl crate::Readable for SdRsp1Spec {}
///`reset()` method sets SD_RSP1 to value 0
impl crate::Resettable for SdRsp1Spec {}
