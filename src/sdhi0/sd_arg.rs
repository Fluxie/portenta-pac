///Register `SD_ARG` reader
pub type R = crate::R<SdArgSpec>;
///Register `SD_ARG` writer
pub type W = crate::W<SdArgSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**SD Command Argument Register

You can [`read`](crate::Reg::read) this register and get [`sd_arg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_arg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SdArgSpec;
impl crate::RegisterSpec for SdArgSpec {
    type Ux = u32;
}
///`read()` method returns [`sd_arg::R`](R) reader structure
impl crate::Readable for SdArgSpec {}
///`write(|w| ..)` method takes [`sd_arg::W`](W) writer structure
impl crate::Writable for SdArgSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SD_ARG to value 0
impl crate::Resettable for SdArgSpec {}
