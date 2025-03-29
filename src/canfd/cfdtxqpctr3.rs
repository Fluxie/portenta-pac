///Register `CFDTXQPCTR3%s` reader
pub type R = crate::R<Cfdtxqpctr3Spec>;
///Register `CFDTXQPCTR3%s` writer
pub type W = crate::W<Cfdtxqpctr3Spec>;
///Field `TXQPC` writer - TX Queue Pointer Control
pub type TxqpcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDTXQPCTR3").finish()
    }
}
impl W {
    ///Bits 0:7 - TX Queue Pointer Control
    #[inline(always)]
    pub fn txqpc(&mut self) -> TxqpcW<Cfdtxqpctr3Spec> {
        TxqpcW::new(self, 0)
    }
}
/**TX Queue Pointer Control Registers 3%s

You can [`read`](crate::Reg::read) this register and get [`cfdtxqpctr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtxqpctr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Cfdtxqpctr3Spec;
impl crate::RegisterSpec for Cfdtxqpctr3Spec {
    type Ux = u32;
}
///`read()` method returns [`cfdtxqpctr3::R`](R) reader structure
impl crate::Readable for Cfdtxqpctr3Spec {}
///`write(|w| ..)` method takes [`cfdtxqpctr3::W`](W) writer structure
impl crate::Writable for Cfdtxqpctr3Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDTXQPCTR3%s to value 0
impl crate::Resettable for Cfdtxqpctr3Spec {}
