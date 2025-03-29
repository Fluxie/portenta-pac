///Register `ACAR1` reader
pub type R = crate::R<Acar1Spec>;
///Register `ACAR1` writer
pub type W = crate::W<Acar1Spec>;
///Field `CAD1` reader - Automatic calibration address
pub type Cad1R = crate::FieldReader<u32>;
///Field `CAD1` writer - Automatic calibration address
pub type Cad1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Automatic calibration address
    #[inline(always)]
    pub fn cad1(&self) -> Cad1R {
        Cad1R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ACAR1").field("cad1", &self.cad1()).finish()
    }
}
impl W {
    ///Bits 0:31 - Automatic calibration address
    #[inline(always)]
    pub fn cad1(&mut self) -> Cad1W<Acar1Spec> {
        Cad1W::new(self, 0)
    }
}
/**Auto-Calibration Address Register 1

You can [`read`](crate::Reg::read) this register and get [`acar1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acar1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Acar1Spec;
impl crate::RegisterSpec for Acar1Spec {
    type Ux = u32;
}
///`read()` method returns [`acar1::R`](R) reader structure
impl crate::Readable for Acar1Spec {}
///`write(|w| ..)` method takes [`acar1::W`](W) writer structure
impl crate::Writable for Acar1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ACAR1 to value 0
impl crate::Resettable for Acar1Spec {}
