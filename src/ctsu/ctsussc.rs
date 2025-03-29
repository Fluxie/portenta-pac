///Register `CTSUSSC` reader
pub type R = crate::R<CtsusscSpec>;
///Register `CTSUSSC` writer
pub type W = crate::W<CtsusscSpec>;
///Field `CTSUSSDIV` reader - CTSU Spectrum Diffusion Frequency Division Setting
pub type CtsussdivR = crate::FieldReader;
///Field `CTSUSSDIV` writer - CTSU Spectrum Diffusion Frequency Division Setting
pub type CtsussdivW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 8:11 - CTSU Spectrum Diffusion Frequency Division Setting
    #[inline(always)]
    pub fn ctsussdiv(&self) -> CtsussdivR {
        CtsussdivR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTSUSSC").field("ctsussdiv", &self.ctsussdiv()).finish()
    }
}
impl W {
    ///Bits 8:11 - CTSU Spectrum Diffusion Frequency Division Setting
    #[inline(always)]
    pub fn ctsussdiv(&mut self) -> CtsussdivW<CtsusscSpec> {
        CtsussdivW::new(self, 8)
    }
}
/**CTSU High-Pass Noise Reduction Spectrum Diffusion Control Register

You can [`read`](crate::Reg::read) this register and get [`ctsussc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsussc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CtsusscSpec;
impl crate::RegisterSpec for CtsusscSpec {
    type Ux = u16;
}
///`read()` method returns [`ctsussc::R`](R) reader structure
impl crate::Readable for CtsusscSpec {}
///`write(|w| ..)` method takes [`ctsussc::W`](W) writer structure
impl crate::Writable for CtsusscSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CTSUSSC to value 0
impl crate::Resettable for CtsusscSpec {}
