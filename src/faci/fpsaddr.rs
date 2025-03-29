///Register `FPSADDR` reader
pub type R = crate::R<FpsaddrSpec>;
///Register `FPSADDR` writer
pub type W = crate::W<FpsaddrSpec>;
///Field `PSADR` reader - Programmed Area Start Address
pub type PsadrR = crate::FieldReader<u32>;
impl R {
    ///Bits 0:16 - Programmed Area Start Address
    #[inline(always)]
    pub fn psadr(&self) -> PsadrR {
        PsadrR::new(self.bits & 0x0001_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FPSADDR").field("psadr", &self.psadr()).finish()
    }
}
impl W {}
/**Data Flash Programming Start Address Register

You can [`read`](crate::Reg::read) this register and get [`fpsaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpsaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FpsaddrSpec;
impl crate::RegisterSpec for FpsaddrSpec {
    type Ux = u32;
}
///`read()` method returns [`fpsaddr::R`](R) reader structure
impl crate::Readable for FpsaddrSpec {}
///`write(|w| ..)` method takes [`fpsaddr::W`](W) writer structure
impl crate::Writable for FpsaddrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FPSADDR to value 0
impl crate::Resettable for FpsaddrSpec {}
