///Register `FLWT` reader
pub type R = crate::R<FlwtSpec>;
///Register `FLWT` writer
pub type W = crate::W<FlwtSpec>;
///Field `FLWT` reader - Flash Wait Cycle
pub type FlwtR = crate::FieldReader;
///Field `FLWT` writer - Flash Wait Cycle
pub type FlwtW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - Flash Wait Cycle
    #[inline(always)]
    pub fn flwt(&self) -> FlwtR {
        FlwtR::new(self.bits & 7)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLWT").field("flwt", &self.flwt()).finish()
    }
}
impl W {
    ///Bits 0:2 - Flash Wait Cycle
    #[inline(always)]
    pub fn flwt(&mut self) -> FlwtW<FlwtSpec> {
        FlwtW::new(self, 0)
    }
}
/**Flash Wait Cycle Register

You can [`read`](crate::Reg::read) this register and get [`flwt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flwt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FlwtSpec;
impl crate::RegisterSpec for FlwtSpec {
    type Ux = u8;
}
///`read()` method returns [`flwt::R`](R) reader structure
impl crate::Readable for FlwtSpec {}
///`write(|w| ..)` method takes [`flwt::W`](W) writer structure
impl crate::Writable for FlwtSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FLWT to value 0
impl crate::Resettable for FlwtSpec {}
