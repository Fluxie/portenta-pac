///Register `GTPR` reader
pub type R = crate::R<GtprSpec>;
///Register `GTPR` writer
pub type W = crate::W<GtprSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**General PWM Timer Cycle Setting Register

You can [`read`](crate::Reg::read) this register and get [`gtpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GtprSpec;
impl crate::RegisterSpec for GtprSpec {
    type Ux = u32;
}
///`read()` method returns [`gtpr::R`](R) reader structure
impl crate::Readable for GtprSpec {}
///`write(|w| ..)` method takes [`gtpr::W`](W) writer structure
impl crate::Writable for GtprSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTPR to value 0xffff_ffff
impl crate::Resettable for GtprSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
