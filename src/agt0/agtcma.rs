///Register `AGTCMA` reader
pub type R = crate::R<AgtcmaSpec>;
///Register `AGTCMA` writer
pub type W = crate::W<AgtcmaSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**AGT Compare Match A Register

You can [`read`](crate::Reg::read) this register and get [`agtcma::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agtcma::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AgtcmaSpec;
impl crate::RegisterSpec for AgtcmaSpec {
    type Ux = u16;
}
///`read()` method returns [`agtcma::R`](R) reader structure
impl crate::Readable for AgtcmaSpec {}
///`write(|w| ..)` method takes [`agtcma::W`](W) writer structure
impl crate::Writable for AgtcmaSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AGTCMA to value 0xffff
impl crate::Resettable for AgtcmaSpec {
    const RESET_VALUE: u16 = 0xffff;
}
