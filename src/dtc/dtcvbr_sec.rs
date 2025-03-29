///Register `DTCVBR_SEC` reader
pub type R = crate::R<DtcvbrSecSpec>;
///Register `DTCVBR_SEC` writer
pub type W = crate::W<DtcvbrSecSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**DTC Vector Base Register for secure Region

You can [`read`](crate::Reg::read) this register and get [`dtcvbr_sec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtcvbr_sec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DtcvbrSecSpec;
impl crate::RegisterSpec for DtcvbrSecSpec {
    type Ux = u32;
}
///`read()` method returns [`dtcvbr_sec::R`](R) reader structure
impl crate::Readable for DtcvbrSecSpec {}
///`write(|w| ..)` method takes [`dtcvbr_sec::W`](W) writer structure
impl crate::Writable for DtcvbrSecSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DTCVBR_SEC to value 0
impl crate::Resettable for DtcvbrSecSpec {}
