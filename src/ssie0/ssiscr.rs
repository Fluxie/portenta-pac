///Register `SSISCR` reader
pub type R = crate::R<SsiscrSpec>;
///Register `SSISCR` writer
pub type W = crate::W<SsiscrSpec>;
///Field `RDFS` reader - RDF Setting Condition Select
pub type RdfsR = crate::FieldReader;
///Field `RDFS` writer - RDF Setting Condition Select
pub type RdfsW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `TDES` reader - TDE Setting Condition Select
pub type TdesR = crate::FieldReader;
///Field `TDES` writer - TDE Setting Condition Select
pub type TdesW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4 - RDF Setting Condition Select
    #[inline(always)]
    pub fn rdfs(&self) -> RdfsR {
        RdfsR::new((self.bits & 0x1f) as u8)
    }
    ///Bits 8:12 - TDE Setting Condition Select
    #[inline(always)]
    pub fn tdes(&self) -> TdesR {
        TdesR::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SSISCR")
            .field("rdfs", &self.rdfs())
            .field("tdes", &self.tdes())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - RDF Setting Condition Select
    #[inline(always)]
    pub fn rdfs(&mut self) -> RdfsW<SsiscrSpec> {
        RdfsW::new(self, 0)
    }
    ///Bits 8:12 - TDE Setting Condition Select
    #[inline(always)]
    pub fn tdes(&mut self) -> TdesW<SsiscrSpec> {
        TdesW::new(self, 8)
    }
}
/**Status Control Register

You can [`read`](crate::Reg::read) this register and get [`ssiscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssiscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SsiscrSpec;
impl crate::RegisterSpec for SsiscrSpec {
    type Ux = u32;
}
///`read()` method returns [`ssiscr::R`](R) reader structure
impl crate::Readable for SsiscrSpec {}
///`write(|w| ..)` method takes [`ssiscr::W`](W) writer structure
impl crate::Writable for SsiscrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SSISCR to value 0
impl crate::Resettable for SsiscrSpec {}
