///Register `CFDCFPCTR%s` reader
pub type R = crate::R<CfdcfpctrSpec>;
///Register `CFDCFPCTR%s` writer
pub type W = crate::W<CfdcfpctrSpec>;
///Field `CFPC` writer - Common FIFO Pointer Control
pub type CfpcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDCFPCTR").finish()
    }
}
impl W {
    ///Bits 0:7 - Common FIFO Pointer Control
    #[inline(always)]
    pub fn cfpc(&mut self) -> CfpcW<CfdcfpctrSpec> {
        CfpcW::new(self, 0)
    }
}
/**Common FIFO Pointer Control Registers %s

You can [`read`](crate::Reg::read) this register and get [`cfdcfpctr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcfpctr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdcfpctrSpec;
impl crate::RegisterSpec for CfdcfpctrSpec {
    type Ux = u32;
}
///`read()` method returns [`cfdcfpctr::R`](R) reader structure
impl crate::Readable for CfdcfpctrSpec {}
///`write(|w| ..)` method takes [`cfdcfpctr::W`](W) writer structure
impl crate::Writable for CfdcfpctrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDCFPCTR%s to value 0
impl crate::Resettable for CfdcfpctrSpec {}
