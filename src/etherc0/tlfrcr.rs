///Register `TLFRCR` reader
pub type R = crate::R<TlfrcrSpec>;
///Register `TLFRCR` writer
pub type W = crate::W<TlfrcrSpec>;
///Field `TLFRCR` reader - Too-Long Frame Receive Counter
pub type TlfrcrR = crate::FieldReader<u32>;
///Field `TLFRCR` writer - Too-Long Frame Receive Counter
pub type TlfrcrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Too-Long Frame Receive Counter
    #[inline(always)]
    pub fn tlfrcr(&self) -> TlfrcrR {
        TlfrcrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TLFRCR").field("tlfrcr", &self.tlfrcr()).finish()
    }
}
impl W {
    ///Bits 0:31 - Too-Long Frame Receive Counter
    #[inline(always)]
    pub fn tlfrcr(&mut self) -> TlfrcrW<TlfrcrSpec> {
        TlfrcrW::new(self, 0)
    }
}
/**Too-Long Frame Receive Counter Register

You can [`read`](crate::Reg::read) this register and get [`tlfrcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tlfrcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TlfrcrSpec;
impl crate::RegisterSpec for TlfrcrSpec {
    type Ux = u32;
}
///`read()` method returns [`tlfrcr::R`](R) reader structure
impl crate::Readable for TlfrcrSpec {}
///`write(|w| ..)` method takes [`tlfrcr::W`](W) writer structure
impl crate::Writable for TlfrcrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TLFRCR to value 0
impl crate::Resettable for TlfrcrSpec {}
