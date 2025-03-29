///Register `SD_RSP32` reader
pub type R = crate::R<SdRsp32Spec>;
///Register `SD_RSP32` writer
pub type W = crate::W<SdRsp32Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**SD Card Response Register 32

You can [`read`](crate::Reg::read) this register and get [`sd_rsp32::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_rsp32::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SdRsp32Spec;
impl crate::RegisterSpec for SdRsp32Spec {
    type Ux = u32;
}
///`read()` method returns [`sd_rsp32::R`](R) reader structure
impl crate::Readable for SdRsp32Spec {}
///`write(|w| ..)` method takes [`sd_rsp32::W`](W) writer structure
impl crate::Writable for SdRsp32Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SD_RSP32 to value 0
impl crate::Resettable for SdRsp32Spec {}
