///Register `DMSAR` reader
pub type R = crate::R<DmsarSpec>;
///Register `DMSAR` writer
pub type W = crate::W<DmsarSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**DMA Source Address Register

You can [`read`](crate::Reg::read) this register and get [`dmsar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmsar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DmsarSpec;
impl crate::RegisterSpec for DmsarSpec {
    type Ux = u32;
}
///`read()` method returns [`dmsar::R`](R) reader structure
impl crate::Readable for DmsarSpec {}
///`write(|w| ..)` method takes [`dmsar::W`](W) writer structure
impl crate::Writable for DmsarSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMSAR to value 0
impl crate::Resettable for DmsarSpec {}
