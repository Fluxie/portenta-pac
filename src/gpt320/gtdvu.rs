///Register `GTDVU` reader
pub type R = crate::R<GtdvuSpec>;
///Register `GTDVU` writer
pub type W = crate::W<GtdvuSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**General PWM Timer Dead Time Value Register U

You can [`read`](crate::Reg::read) this register and get [`gtdvu::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtdvu::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GtdvuSpec;
impl crate::RegisterSpec for GtdvuSpec {
    type Ux = u32;
}
///`read()` method returns [`gtdvu::R`](R) reader structure
impl crate::Readable for GtdvuSpec {}
///`write(|w| ..)` method takes [`gtdvu::W`](W) writer structure
impl crate::Writable for GtdvuSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTDVU to value 0xffff_ffff
impl crate::Resettable for GtdvuSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
