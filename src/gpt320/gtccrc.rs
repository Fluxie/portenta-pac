///Register `GTCCRC` reader
pub type R = crate::R<GtccrcSpec>;
///Register `GTCCRC` writer
pub type W = crate::W<GtccrcSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**General PWM Timer Compare Capture Register C

You can [`read`](crate::Reg::read) this register and get [`gtccrc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtccrc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GtccrcSpec;
impl crate::RegisterSpec for GtccrcSpec {
    type Ux = u32;
}
///`read()` method returns [`gtccrc::R`](R) reader structure
impl crate::Readable for GtccrcSpec {}
///`write(|w| ..)` method takes [`gtccrc::W`](W) writer structure
impl crate::Writable for GtccrcSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTCCRC to value 0xffff_ffff
impl crate::Resettable for GtccrcSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
