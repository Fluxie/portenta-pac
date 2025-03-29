///Register `HOCOUTCR` reader
pub type R = crate::R<HocoutcrSpec>;
///Register `HOCOUTCR` writer
pub type W = crate::W<HocoutcrSpec>;
///Field `HOCOUTRM` reader - HOCO User Trimming
pub type HocoutrmR = crate::FieldReader;
///Field `HOCOUTRM` writer - HOCO User Trimming
pub type HocoutrmW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - HOCO User Trimming
    #[inline(always)]
    pub fn hocoutrm(&self) -> HocoutrmR {
        HocoutrmR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOCOUTCR").field("hocoutrm", &self.hocoutrm()).finish()
    }
}
impl W {
    ///Bits 0:7 - HOCO User Trimming
    #[inline(always)]
    pub fn hocoutrm(&mut self) -> HocoutrmW<HocoutcrSpec> {
        HocoutrmW::new(self, 0)
    }
}
/**HOCO User Trimming Control Register

You can [`read`](crate::Reg::read) this register and get [`hocoutcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hocoutcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct HocoutcrSpec;
impl crate::RegisterSpec for HocoutcrSpec {
    type Ux = u8;
}
///`read()` method returns [`hocoutcr::R`](R) reader structure
impl crate::Readable for HocoutcrSpec {}
///`write(|w| ..)` method takes [`hocoutcr::W`](W) writer structure
impl crate::Writable for HocoutcrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HOCOUTCR to value 0
impl crate::Resettable for HocoutcrSpec {}
