///Register `DMDAR` reader
pub type R = crate::R<DmdarSpec>;
///Register `DMDAR` writer
pub type W = crate::W<DmdarSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**DMA Destination Address Register

You can [`read`](crate::Reg::read) this register and get [`dmdar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmdar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DmdarSpec;
impl crate::RegisterSpec for DmdarSpec {
    type Ux = u32;
}
///`read()` method returns [`dmdar::R`](R) reader structure
impl crate::Readable for DmdarSpec {}
///`write(|w| ..)` method takes [`dmdar::W`](W) writer structure
impl crate::Writable for DmdarSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMDAR to value 0
impl crate::Resettable for DmdarSpec {}
