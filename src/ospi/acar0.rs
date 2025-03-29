///Register `ACAR0` reader
pub type R = crate::R<Acar0Spec>;
///Register `ACAR0` writer
pub type W = crate::W<Acar0Spec>;
///Field `CAD0` reader - Automatic calibration address
pub type Cad0R = crate::FieldReader<u32>;
///Field `CAD0` writer - Automatic calibration address
pub type Cad0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Automatic calibration address
    #[inline(always)]
    pub fn cad0(&self) -> Cad0R {
        Cad0R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ACAR0").field("cad0", &self.cad0()).finish()
    }
}
impl W {
    ///Bits 0:31 - Automatic calibration address
    #[inline(always)]
    pub fn cad0(&mut self) -> Cad0W<Acar0Spec> {
        Cad0W::new(self, 0)
    }
}
/**Auto-Calibration Address Register 0

You can [`read`](crate::Reg::read) this register and get [`acar0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acar0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Acar0Spec;
impl crate::RegisterSpec for Acar0Spec {
    type Ux = u32;
}
///`read()` method returns [`acar0::R`](R) reader structure
impl crate::Readable for Acar0Spec {}
///`write(|w| ..)` method takes [`acar0::W`](W) writer structure
impl crate::Writable for Acar0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ACAR0 to value 0
impl crate::Resettable for Acar0Spec {}
