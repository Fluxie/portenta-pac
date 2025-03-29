///Register `DMDRR` reader
pub type R = crate::R<DmdrrSpec>;
///Register `DMDRR` writer
pub type W = crate::W<DmdrrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**DMA Destination Reload Address Register

You can [`read`](crate::Reg::read) this register and get [`dmdrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmdrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DmdrrSpec;
impl crate::RegisterSpec for DmdrrSpec {
    type Ux = u32;
}
///`read()` method returns [`dmdrr::R`](R) reader structure
impl crate::Readable for DmdrrSpec {}
///`write(|w| ..)` method takes [`dmdrr::W`](W) writer structure
impl crate::Writable for DmdrrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMDRR to value 0
impl crate::Resettable for DmdrrSpec {}
