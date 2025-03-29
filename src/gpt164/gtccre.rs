///Register `GTCCRE` reader
pub type R = crate::R<GtccreSpec>;
///Register `GTCCRE` writer
pub type W = crate::W<GtccreSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**General PWM Timer Compare Capture Register E

You can [`read`](crate::Reg::read) this register and get [`gtccre::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtccre::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GtccreSpec;
impl crate::RegisterSpec for GtccreSpec {
    type Ux = u32;
}
///`read()` method returns [`gtccre::R`](R) reader structure
impl crate::Readable for GtccreSpec {}
///`write(|w| ..)` method takes [`gtccre::W`](W) writer structure
impl crate::Writable for GtccreSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTCCRE to value 0xffff
impl crate::Resettable for GtccreSpec {
    const RESET_VALUE: u32 = 0xffff;
}
