///Register `DMACDTCERRCLR` reader
pub type R = crate::R<DmacdtcerrclrSpec>;
///Register `DMACDTCERRCLR` writer
pub type W = crate::W<DmacdtcerrclrSpec>;
///Field `MTERRCLR` reader - Master TrustZone filter Error Clear
pub type MterrclrR = crate::BitReader;
///Field `MTERRCLR` writer - Master TrustZone filter Error Clear
pub type MterrclrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Master TrustZone filter Error Clear
    #[inline(always)]
    pub fn mterrclr(&self) -> MterrclrR {
        MterrclrR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACDTCERRCLR").field("mterrclr", &self.mterrclr()).finish()
    }
}
impl W {
    ///Bit 0 - Master TrustZone filter Error Clear
    #[inline(always)]
    pub fn mterrclr(&mut self) -> MterrclrW<DmacdtcerrclrSpec> {
        MterrclrW::new(self, 0)
    }
}
/**DMAC/DTC Error Clear Register

You can [`read`](crate::Reg::read) this register and get [`dmacdtcerrclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacdtcerrclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DmacdtcerrclrSpec;
impl crate::RegisterSpec for DmacdtcerrclrSpec {
    type Ux = u8;
}
///`read()` method returns [`dmacdtcerrclr::R`](R) reader structure
impl crate::Readable for DmacdtcerrclrSpec {}
///`write(|w| ..)` method takes [`dmacdtcerrclr::W`](W) writer structure
impl crate::Writable for DmacdtcerrclrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMACDTCERRCLR to value 0
impl crate::Resettable for DmacdtcerrclrSpec {}
