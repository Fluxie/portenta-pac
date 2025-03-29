///Register `CFDTXQPCTR1%s` reader
pub type R = crate::R<Cfdtxqpctr1Spec>;
///Register `CFDTXQPCTR1%s` writer
pub type W = crate::W<Cfdtxqpctr1Spec>;
///Field `TXQPC` writer - TX Queue Pointer Control
pub type TxqpcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDTXQPCTR1").finish()
    }
}
impl W {
    ///Bits 0:7 - TX Queue Pointer Control
    #[inline(always)]
    pub fn txqpc(&mut self) -> TxqpcW<Cfdtxqpctr1Spec> {
        TxqpcW::new(self, 0)
    }
}
/**TX Queue Pointer Control Registers 1%s

You can [`read`](crate::Reg::read) this register and get [`cfdtxqpctr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtxqpctr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Cfdtxqpctr1Spec;
impl crate::RegisterSpec for Cfdtxqpctr1Spec {
    type Ux = u32;
}
///`read()` method returns [`cfdtxqpctr1::R`](R) reader structure
impl crate::Readable for Cfdtxqpctr1Spec {}
///`write(|w| ..)` method takes [`cfdtxqpctr1::W`](W) writer structure
impl crate::Writable for Cfdtxqpctr1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDTXQPCTR1%s to value 0
impl crate::Resettable for Cfdtxqpctr1Spec {}
