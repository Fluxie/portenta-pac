///Register `SD_ARG1` reader
pub type R = crate::R<SdArg1Spec>;
///Register `SD_ARG1` writer
pub type W = crate::W<SdArg1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**SD Command Argument Register 1

You can [`read`](crate::Reg::read) this register and get [`sd_arg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_arg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SdArg1Spec;
impl crate::RegisterSpec for SdArg1Spec {
    type Ux = u32;
}
///`read()` method returns [`sd_arg1::R`](R) reader structure
impl crate::Readable for SdArg1Spec {}
///`write(|w| ..)` method takes [`sd_arg1::W`](W) writer structure
impl crate::Writable for SdArg1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SD_ARG1 to value 0
impl crate::Resettable for SdArg1Spec {}
