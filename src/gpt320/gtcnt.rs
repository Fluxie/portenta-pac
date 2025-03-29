///Register `GTCNT` reader
pub type R = crate::R<GtcntSpec>;
///Register `GTCNT` writer
pub type W = crate::W<GtcntSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**General PWM Timer Counter

You can [`read`](crate::Reg::read) this register and get [`gtcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GtcntSpec;
impl crate::RegisterSpec for GtcntSpec {
    type Ux = u32;
}
///`read()` method returns [`gtcnt::R`](R) reader structure
impl crate::Readable for GtcntSpec {}
///`write(|w| ..)` method takes [`gtcnt::W`](W) writer structure
impl crate::Writable for GtcntSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTCNT to value 0
impl crate::Resettable for GtcntSpec {}
