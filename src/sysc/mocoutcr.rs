///Register `MOCOUTCR` reader
pub type R = crate::R<MocoutcrSpec>;
///Register `MOCOUTCR` writer
pub type W = crate::W<MocoutcrSpec>;
///Field `MOCOUTRM` reader - MOCO User Trimming
pub type MocoutrmR = crate::FieldReader;
///Field `MOCOUTRM` writer - MOCO User Trimming
pub type MocoutrmW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - MOCO User Trimming
    #[inline(always)]
    pub fn mocoutrm(&self) -> MocoutrmR {
        MocoutrmR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MOCOUTCR").field("mocoutrm", &self.mocoutrm()).finish()
    }
}
impl W {
    ///Bits 0:7 - MOCO User Trimming
    #[inline(always)]
    pub fn mocoutrm(&mut self) -> MocoutrmW<MocoutcrSpec> {
        MocoutrmW::new(self, 0)
    }
}
/**MOCO User Trimming Control Register

You can [`read`](crate::Reg::read) this register and get [`mocoutcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mocoutcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MocoutcrSpec;
impl crate::RegisterSpec for MocoutcrSpec {
    type Ux = u8;
}
///`read()` method returns [`mocoutcr::R`](R) reader structure
impl crate::Readable for MocoutcrSpec {}
///`write(|w| ..)` method takes [`mocoutcr::W`](W) writer structure
impl crate::Writable for MocoutcrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MOCOUTCR to value 0
impl crate::Resettable for MocoutcrSpec {}
