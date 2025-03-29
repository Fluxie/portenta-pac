///Register `CFDRFPCTR%s` reader
pub type R = crate::R<CfdrfpctrSpec>;
///Register `CFDRFPCTR%s` writer
pub type W = crate::W<CfdrfpctrSpec>;
///Field `RFPC` writer - RX FIFO Pointer Control
pub type RfpcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDRFPCTR").finish()
    }
}
impl W {
    ///Bits 0:7 - RX FIFO Pointer Control
    #[inline(always)]
    pub fn rfpc(&mut self) -> RfpcW<CfdrfpctrSpec> {
        RfpcW::new(self, 0)
    }
}
/**RX FIFO Pointer Control Registers %s

You can [`read`](crate::Reg::read) this register and get [`cfdrfpctr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdrfpctr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdrfpctrSpec;
impl crate::RegisterSpec for CfdrfpctrSpec {
    type Ux = u32;
}
///`read()` method returns [`cfdrfpctr::R`](R) reader structure
impl crate::Readable for CfdrfpctrSpec {}
///`write(|w| ..)` method takes [`cfdrfpctr::W`](W) writer structure
impl crate::Writable for CfdrfpctrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDRFPCTR%s to value 0
impl crate::Resettable for CfdrfpctrSpec {}
