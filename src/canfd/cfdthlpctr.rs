///Register `CFDTHLPCTR%s` reader
pub type R = crate::R<CfdthlpctrSpec>;
///Register `CFDTHLPCTR%s` writer
pub type W = crate::W<CfdthlpctrSpec>;
///Field `THLPC` writer - TX History List Pointer Control
pub type ThlpcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDTHLPCTR").finish()
    }
}
impl W {
    ///Bits 0:7 - TX History List Pointer Control
    #[inline(always)]
    pub fn thlpc(&mut self) -> ThlpcW<CfdthlpctrSpec> {
        ThlpcW::new(self, 0)
    }
}
/**TX History List Pointer Control Registers %s

You can [`read`](crate::Reg::read) this register and get [`cfdthlpctr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdthlpctr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdthlpctrSpec;
impl crate::RegisterSpec for CfdthlpctrSpec {
    type Ux = u32;
}
///`read()` method returns [`cfdthlpctr::R`](R) reader structure
impl crate::Readable for CfdthlpctrSpec {}
///`write(|w| ..)` method takes [`cfdthlpctr::W`](W) writer structure
impl crate::Writable for CfdthlpctrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDTHLPCTR%s to value 0
impl crate::Resettable for CfdthlpctrSpec {}
