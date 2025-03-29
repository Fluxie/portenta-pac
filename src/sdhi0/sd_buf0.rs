///Register `SD_BUF0` reader
pub type R = crate::R<SdBuf0Spec>;
///Register `SD_BUF0` writer
pub type W = crate::W<SdBuf0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**SD Buffer Register

You can [`read`](crate::Reg::read) this register and get [`sd_buf0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_buf0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SdBuf0Spec;
impl crate::RegisterSpec for SdBuf0Spec {
    type Ux = u32;
}
///`read()` method returns [`sd_buf0::R`](R) reader structure
impl crate::Readable for SdBuf0Spec {}
///`write(|w| ..)` method takes [`sd_buf0::W`](W) writer structure
impl crate::Writable for SdBuf0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SD_BUF0 to value 0
impl crate::Resettable for SdBuf0Spec {}
