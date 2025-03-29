///Register `GTCCRB` reader
pub type R = crate::R<GtccrbSpec>;
///Register `GTCCRB` writer
pub type W = crate::W<GtccrbSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**General PWM Timer Compare Capture Register B

You can [`read`](crate::Reg::read) this register and get [`gtccrb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtccrb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GtccrbSpec;
impl crate::RegisterSpec for GtccrbSpec {
    type Ux = u32;
}
///`read()` method returns [`gtccrb::R`](R) reader structure
impl crate::Readable for GtccrbSpec {}
///`write(|w| ..)` method takes [`gtccrb::W`](W) writer structure
impl crate::Writable for GtccrbSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTCCRB to value 0xffff_ffff
impl crate::Resettable for GtccrbSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
