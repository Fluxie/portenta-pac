///Register `AGTCMB` reader
pub type R = crate::R<AgtcmbSpec>;
///Register `AGTCMB` writer
pub type W = crate::W<AgtcmbSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**AGT Compare Match B Register

You can [`read`](crate::Reg::read) this register and get [`agtcmb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agtcmb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AgtcmbSpec;
impl crate::RegisterSpec for AgtcmbSpec {
    type Ux = u16;
}
///`read()` method returns [`agtcmb::R`](R) reader structure
impl crate::Readable for AgtcmbSpec {}
///`write(|w| ..)` method takes [`agtcmb::W`](W) writer structure
impl crate::Writable for AgtcmbSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AGTCMB to value 0xffff
impl crate::Resettable for AgtcmbSpec {
    const RESET_VALUE: u16 = 0xffff;
}
