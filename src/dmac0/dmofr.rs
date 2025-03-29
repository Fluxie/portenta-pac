///Register `DMOFR` reader
pub type R = crate::R<DmofrSpec>;
///Register `DMOFR` writer
pub type W = crate::W<DmofrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**DMA Offset Register

You can [`read`](crate::Reg::read) this register and get [`dmofr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmofr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DmofrSpec;
impl crate::RegisterSpec for DmofrSpec {
    type Ux = u32;
}
///`read()` method returns [`dmofr::R`](R) reader structure
impl crate::Readable for DmofrSpec {}
///`write(|w| ..)` method takes [`dmofr::W`](W) writer structure
impl crate::Writable for DmofrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMOFR to value 0
impl crate::Resettable for DmofrSpec {}
