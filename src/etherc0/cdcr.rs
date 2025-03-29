///Register `CDCR` reader
pub type R = crate::R<CdcrSpec>;
///Register `CDCR` writer
pub type W = crate::W<CdcrSpec>;
///Field `CDCR` reader - Late Collision Detect Counter
pub type CdcrR = crate::FieldReader<u32>;
///Field `CDCR` writer - Late Collision Detect Counter
pub type CdcrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Late Collision Detect Counter
    #[inline(always)]
    pub fn cdcr(&self) -> CdcrR {
        CdcrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CDCR").field("cdcr", &self.cdcr()).finish()
    }
}
impl W {
    ///Bits 0:31 - Late Collision Detect Counter
    #[inline(always)]
    pub fn cdcr(&mut self) -> CdcrW<CdcrSpec> {
        CdcrW::new(self, 0)
    }
}
/**Late Collision Detect Counter Register

You can [`read`](crate::Reg::read) this register and get [`cdcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cdcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CdcrSpec;
impl crate::RegisterSpec for CdcrSpec {
    type Ux = u32;
}
///`read()` method returns [`cdcr::R`](R) reader structure
impl crate::Readable for CdcrSpec {}
///`write(|w| ..)` method takes [`cdcr::W`](W) writer structure
impl crate::Writable for CdcrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CDCR to value 0
impl crate::Resettable for CdcrSpec {}
