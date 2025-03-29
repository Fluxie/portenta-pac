///Register `TSFRCR` reader
pub type R = crate::R<TsfrcrSpec>;
///Register `TSFRCR` writer
pub type W = crate::W<TsfrcrSpec>;
///Field `TSFRCR` reader - Too-Short Frame Receive Counter
pub type TsfrcrR = crate::FieldReader<u32>;
///Field `TSFRCR` writer - Too-Short Frame Receive Counter
pub type TsfrcrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Too-Short Frame Receive Counter
    #[inline(always)]
    pub fn tsfrcr(&self) -> TsfrcrR {
        TsfrcrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TSFRCR").field("tsfrcr", &self.tsfrcr()).finish()
    }
}
impl W {
    ///Bits 0:31 - Too-Short Frame Receive Counter
    #[inline(always)]
    pub fn tsfrcr(&mut self) -> TsfrcrW<TsfrcrSpec> {
        TsfrcrW::new(self, 0)
    }
}
/**Too-Short Frame Receive Counter Register

You can [`read`](crate::Reg::read) this register and get [`tsfrcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsfrcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TsfrcrSpec;
impl crate::RegisterSpec for TsfrcrSpec {
    type Ux = u32;
}
///`read()` method returns [`tsfrcr::R`](R) reader structure
impl crate::Readable for TsfrcrSpec {}
///`write(|w| ..)` method takes [`tsfrcr::W`](W) writer structure
impl crate::Writable for TsfrcrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TSFRCR to value 0
impl crate::Resettable for TsfrcrSpec {}
