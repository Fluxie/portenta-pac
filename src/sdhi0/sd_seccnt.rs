///Register `SD_SECCNT` reader
pub type R = crate::R<SdSeccntSpec>;
///Register `SD_SECCNT` writer
pub type W = crate::W<SdSeccntSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**Block Count Register

You can [`read`](crate::Reg::read) this register and get [`sd_seccnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_seccnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SdSeccntSpec;
impl crate::RegisterSpec for SdSeccntSpec {
    type Ux = u32;
}
///`read()` method returns [`sd_seccnt::R`](R) reader structure
impl crate::Readable for SdSeccntSpec {}
///`write(|w| ..)` method takes [`sd_seccnt::W`](W) writer structure
impl crate::Writable for SdSeccntSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SD_SECCNT to value 0
impl crate::Resettable for SdSeccntSpec {}
