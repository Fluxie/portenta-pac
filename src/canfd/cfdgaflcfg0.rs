///Register `CFDGAFLCFG0` reader
pub type R = crate::R<Cfdgaflcfg0Spec>;
///Register `CFDGAFLCFG0` writer
pub type W = crate::W<Cfdgaflcfg0Spec>;
///Field `RNC1` reader - Rule Number for Channel 1
pub type Rnc1R = crate::FieldReader<u16>;
///Field `RNC1` writer - Rule Number for Channel 1
pub type Rnc1W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `RNC0` reader - Rule Number for Channel 0
pub type Rnc0R = crate::FieldReader<u16>;
///Field `RNC0` writer - Rule Number for Channel 0
pub type Rnc0W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - Rule Number for Channel 1
    #[inline(always)]
    pub fn rnc1(&self) -> Rnc1R {
        Rnc1R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 16:24 - Rule Number for Channel 0
    #[inline(always)]
    pub fn rnc0(&self) -> Rnc0R {
        Rnc0R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDGAFLCFG0")
            .field("rnc1", &self.rnc1())
            .field("rnc0", &self.rnc0())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - Rule Number for Channel 1
    #[inline(always)]
    pub fn rnc1(&mut self) -> Rnc1W<Cfdgaflcfg0Spec> {
        Rnc1W::new(self, 0)
    }
    ///Bits 16:24 - Rule Number for Channel 0
    #[inline(always)]
    pub fn rnc0(&mut self) -> Rnc0W<Cfdgaflcfg0Spec> {
        Rnc0W::new(self, 16)
    }
}
/**Global Acceptance Filter List Configuration Register 0

You can [`read`](crate::Reg::read) this register and get [`cfdgaflcfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdgaflcfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Cfdgaflcfg0Spec;
impl crate::RegisterSpec for Cfdgaflcfg0Spec {
    type Ux = u32;
}
///`read()` method returns [`cfdgaflcfg0::R`](R) reader structure
impl crate::Readable for Cfdgaflcfg0Spec {}
///`write(|w| ..)` method takes [`cfdgaflcfg0::W`](W) writer structure
impl crate::Writable for Cfdgaflcfg0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDGAFLCFG0 to value 0
impl crate::Resettable for Cfdgaflcfg0Spec {}
