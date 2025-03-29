///Register `GTPBR` reader
pub type R = crate::R<GtpbrSpec>;
///Register `GTPBR` writer
pub type W = crate::W<GtpbrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**General PWM Timer Cycle Setting Buffer Register

You can [`read`](crate::Reg::read) this register and get [`gtpbr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtpbr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GtpbrSpec;
impl crate::RegisterSpec for GtpbrSpec {
    type Ux = u32;
}
///`read()` method returns [`gtpbr::R`](R) reader structure
impl crate::Readable for GtpbrSpec {}
///`write(|w| ..)` method takes [`gtpbr::W`](W) writer structure
impl crate::Writable for GtpbrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTPBR to value 0xffff_ffff
impl crate::Resettable for GtpbrSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
