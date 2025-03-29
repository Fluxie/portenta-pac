///Register `LOCOUTCR` reader
pub type R = crate::R<LocoutcrSpec>;
///Register `LOCOUTCR` writer
pub type W = crate::W<LocoutcrSpec>;
///Field `LOCOUTRM` reader - LOCO User Trimming
pub type LocoutrmR = crate::FieldReader;
///Field `LOCOUTRM` writer - LOCO User Trimming
pub type LocoutrmW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - LOCO User Trimming
    #[inline(always)]
    pub fn locoutrm(&self) -> LocoutrmR {
        LocoutrmR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LOCOUTCR").field("locoutrm", &self.locoutrm()).finish()
    }
}
impl W {
    ///Bits 0:7 - LOCO User Trimming
    #[inline(always)]
    pub fn locoutrm(&mut self) -> LocoutrmW<LocoutcrSpec> {
        LocoutrmW::new(self, 0)
    }
}
/**LOCO User Trimming Control Register

You can [`read`](crate::Reg::read) this register and get [`locoutcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`locoutcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LocoutcrSpec;
impl crate::RegisterSpec for LocoutcrSpec {
    type Ux = u8;
}
///`read()` method returns [`locoutcr::R`](R) reader structure
impl crate::Readable for LocoutcrSpec {}
///`write(|w| ..)` method takes [`locoutcr::W`](W) writer structure
impl crate::Writable for LocoutcrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LOCOUTCR to value 0
impl crate::Resettable for LocoutcrSpec {}
