///Register `FLLCR2` reader
pub type R = crate::R<Fllcr2Spec>;
///Register `FLLCR2` writer
pub type W = crate::W<Fllcr2Spec>;
///Field `FLLCNTL` reader - FLL Multiplication Control
pub type FllcntlR = crate::FieldReader<u16>;
///Field `FLLCNTL` writer - FLL Multiplication Control
pub type FllcntlW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    ///Bits 0:10 - FLL Multiplication Control
    #[inline(always)]
    pub fn fllcntl(&self) -> FllcntlR {
        FllcntlR::new(self.bits & 0x07ff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLLCR2").field("fllcntl", &self.fllcntl()).finish()
    }
}
impl W {
    ///Bits 0:10 - FLL Multiplication Control
    #[inline(always)]
    pub fn fllcntl(&mut self) -> FllcntlW<Fllcr2Spec> {
        FllcntlW::new(self, 0)
    }
}
/**FLL Control Register2

You can [`read`](crate::Reg::read) this register and get [`fllcr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fllcr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Fllcr2Spec;
impl crate::RegisterSpec for Fllcr2Spec {
    type Ux = u16;
}
///`read()` method returns [`fllcr2::R`](R) reader structure
impl crate::Readable for Fllcr2Spec {}
///`write(|w| ..)` method takes [`fllcr2::W`](W) writer structure
impl crate::Writable for Fllcr2Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FLLCR2 to value 0
impl crate::Resettable for Fllcr2Spec {}
