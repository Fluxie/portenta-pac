///Register `GTCCRD` reader
pub type R = crate::R<GtccrdSpec>;
///Register `GTCCRD` writer
pub type W = crate::W<GtccrdSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**General PWM Timer Compare Capture Register D

You can [`read`](crate::Reg::read) this register and get [`gtccrd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtccrd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GtccrdSpec;
impl crate::RegisterSpec for GtccrdSpec {
    type Ux = u32;
}
///`read()` method returns [`gtccrd::R`](R) reader structure
impl crate::Readable for GtccrdSpec {}
///`write(|w| ..)` method takes [`gtccrd::W`](W) writer structure
impl crate::Writable for GtccrdSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTCCRD to value 0xffff_ffff
impl crate::Resettable for GtccrdSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
