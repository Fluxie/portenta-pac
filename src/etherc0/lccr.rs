///Register `LCCR` reader
pub type R = crate::R<LccrSpec>;
///Register `LCCR` writer
pub type W = crate::W<LccrSpec>;
///Field `LCCR` reader - Lost Carrier Counter
pub type LccrR = crate::FieldReader<u32>;
///Field `LCCR` writer - Lost Carrier Counter
pub type LccrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Lost Carrier Counter
    #[inline(always)]
    pub fn lccr(&self) -> LccrR {
        LccrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LCCR").field("lccr", &self.lccr()).finish()
    }
}
impl W {
    ///Bits 0:31 - Lost Carrier Counter
    #[inline(always)]
    pub fn lccr(&mut self) -> LccrW<LccrSpec> {
        LccrW::new(self, 0)
    }
}
/**Lost Carrier Counter Register

You can [`read`](crate::Reg::read) this register and get [`lccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LccrSpec;
impl crate::RegisterSpec for LccrSpec {
    type Ux = u32;
}
///`read()` method returns [`lccr::R`](R) reader structure
impl crate::Readable for LccrSpec {}
///`write(|w| ..)` method takes [`lccr::W`](W) writer structure
impl crate::Writable for LccrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LCCR to value 0
impl crate::Resettable for LccrSpec {}
