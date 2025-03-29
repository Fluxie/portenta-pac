///Register `GTCCRA` reader
pub type R = crate::R<GtccraSpec>;
///Register `GTCCRA` writer
pub type W = crate::W<GtccraSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**General PWM Timer Compare Capture Register A

You can [`read`](crate::Reg::read) this register and get [`gtccra::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtccra::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GtccraSpec;
impl crate::RegisterSpec for GtccraSpec {
    type Ux = u32;
}
///`read()` method returns [`gtccra::R`](R) reader structure
impl crate::Readable for GtccraSpec {}
///`write(|w| ..)` method takes [`gtccra::W`](W) writer structure
impl crate::Writable for GtccraSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTCCRA to value 0xffff_ffff
impl crate::Resettable for GtccraSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
