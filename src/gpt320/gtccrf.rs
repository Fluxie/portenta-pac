///Register `GTCCRF` reader
pub type R = crate::R<GtccrfSpec>;
///Register `GTCCRF` writer
pub type W = crate::W<GtccrfSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**General PWM Timer Compare Capture Register F

You can [`read`](crate::Reg::read) this register and get [`gtccrf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtccrf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GtccrfSpec;
impl crate::RegisterSpec for GtccrfSpec {
    type Ux = u32;
}
///`read()` method returns [`gtccrf::R`](R) reader structure
impl crate::Readable for GtccrfSpec {}
///`write(|w| ..)` method takes [`gtccrf::W`](W) writer structure
impl crate::Writable for GtccrfSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTCCRF to value 0xffff_ffff
impl crate::Resettable for GtccrfSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
