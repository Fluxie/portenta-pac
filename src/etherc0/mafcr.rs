///Register `MAFCR` reader
pub type R = crate::R<MafcrSpec>;
///Register `MAFCR` writer
pub type W = crate::W<MafcrSpec>;
///Field `MAFCR` reader - Multicast Address Frame Receive Counter
pub type MafcrR = crate::FieldReader<u32>;
///Field `MAFCR` writer - Multicast Address Frame Receive Counter
pub type MafcrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Multicast Address Frame Receive Counter
    #[inline(always)]
    pub fn mafcr(&self) -> MafcrR {
        MafcrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MAFCR").field("mafcr", &self.mafcr()).finish()
    }
}
impl W {
    ///Bits 0:31 - Multicast Address Frame Receive Counter
    #[inline(always)]
    pub fn mafcr(&mut self) -> MafcrW<MafcrSpec> {
        MafcrW::new(self, 0)
    }
}
/**Multicast Address Frame Receive Counter Register

You can [`read`](crate::Reg::read) this register and get [`mafcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mafcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MafcrSpec;
impl crate::RegisterSpec for MafcrSpec {
    type Ux = u32;
}
///`read()` method returns [`mafcr::R`](R) reader structure
impl crate::Readable for MafcrSpec {}
///`write(|w| ..)` method takes [`mafcr::W`](W) writer structure
impl crate::Writable for MafcrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MAFCR to value 0
impl crate::Resettable for MafcrSpec {}
