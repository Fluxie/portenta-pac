///Register `DMSRR` reader
pub type R = crate::R<DmsrrSpec>;
///Register `DMSRR` writer
pub type W = crate::W<DmsrrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**DMA Source Reload Address Register

You can [`read`](crate::Reg::read) this register and get [`dmsrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmsrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DmsrrSpec;
impl crate::RegisterSpec for DmsrrSpec {
    type Ux = u32;
}
///`read()` method returns [`dmsrr::R`](R) reader structure
impl crate::Readable for DmsrrSpec {}
///`write(|w| ..)` method takes [`dmsrr::W`](W) writer structure
impl crate::Writable for DmsrrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMSRR to value 0
impl crate::Resettable for DmsrrSpec {}
