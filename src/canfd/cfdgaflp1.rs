///Register `CFDGAFLP1%s` reader
pub type R = crate::R<Cfdgaflp1Spec>;
///Register `CFDGAFLP1%s` writer
pub type W = crate::W<Cfdgaflp1Spec>;
///Field `GAFLFDP` reader - Global Acceptance Filter List FIFO Direction Pointer
pub type GaflfdpR = crate::FieldReader<u16>;
///Field `GAFLFDP` writer - Global Acceptance Filter List FIFO Direction Pointer
pub type GaflfdpW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    ///Bits 0:13 - Global Acceptance Filter List FIFO Direction Pointer
    #[inline(always)]
    pub fn gaflfdp(&self) -> GaflfdpR {
        GaflfdpR::new((self.bits & 0x3fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDGAFLP1").field("gaflfdp", &self.gaflfdp()).finish()
    }
}
impl W {
    ///Bits 0:13 - Global Acceptance Filter List FIFO Direction Pointer
    #[inline(always)]
    pub fn gaflfdp(&mut self) -> GaflfdpW<Cfdgaflp1Spec> {
        GaflfdpW::new(self, 0)
    }
}
/**Global Acceptance Filter List Pointer 1 Registers

You can [`read`](crate::Reg::read) this register and get [`cfdgaflp1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdgaflp1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Cfdgaflp1Spec;
impl crate::RegisterSpec for Cfdgaflp1Spec {
    type Ux = u32;
}
///`read()` method returns [`cfdgaflp1::R`](R) reader structure
impl crate::Readable for Cfdgaflp1Spec {}
///`write(|w| ..)` method takes [`cfdgaflp1::W`](W) writer structure
impl crate::Writable for Cfdgaflp1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDGAFLP1%s to value 0
impl crate::Resettable for Cfdgaflp1Spec {}
