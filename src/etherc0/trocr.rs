///Register `TROCR` reader
pub type R = crate::R<TrocrSpec>;
///Register `TROCR` writer
pub type W = crate::W<TrocrSpec>;
///Field `TROCR` reader - Transmit Retry Over Counter
pub type TrocrR = crate::FieldReader<u32>;
///Field `TROCR` writer - Transmit Retry Over Counter
pub type TrocrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Transmit Retry Over Counter
    #[inline(always)]
    pub fn trocr(&self) -> TrocrR {
        TrocrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TROCR").field("trocr", &self.trocr()).finish()
    }
}
impl W {
    ///Bits 0:31 - Transmit Retry Over Counter
    #[inline(always)]
    pub fn trocr(&mut self) -> TrocrW<TrocrSpec> {
        TrocrW::new(self, 0)
    }
}
/**Transmit Retry Over Counter Register

You can [`read`](crate::Reg::read) this register and get [`trocr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trocr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TrocrSpec;
impl crate::RegisterSpec for TrocrSpec {
    type Ux = u32;
}
///`read()` method returns [`trocr::R`](R) reader structure
impl crate::Readable for TrocrSpec {}
///`write(|w| ..)` method takes [`trocr::W`](W) writer structure
impl crate::Writable for TrocrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TROCR to value 0
impl crate::Resettable for TrocrSpec {}
