///Register `NOMT` reader
pub type R = crate::R<NomtSpec>;
///Register `NOMT` writer
pub type W = crate::W<NomtSpec>;
///Field `NOMT` reader - CEC Reception Data Sampling Time Setting,
pub type NomtR = crate::FieldReader<u16>;
///Field `NOMT` writer - CEC Reception Data Sampling Time Setting,
pub type NomtW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - CEC Reception Data Sampling Time Setting,
    #[inline(always)]
    pub fn nomt(&self) -> NomtR {
        NomtR::new(self.bits & 0x01ff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NOMT").field("nomt", &self.nomt()).finish()
    }
}
impl W {
    ///Bits 0:8 - CEC Reception Data Sampling Time Setting,
    #[inline(always)]
    pub fn nomt(&mut self) -> NomtW<NomtSpec> {
        NomtW::new(self, 0)
    }
}
/**CEC Reception Data Sampling Time Setting Register

You can [`read`](crate::Reg::read) this register and get [`nomt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nomt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct NomtSpec;
impl crate::RegisterSpec for NomtSpec {
    type Ux = u16;
}
///`read()` method returns [`nomt::R`](R) reader structure
impl crate::Readable for NomtSpec {}
///`write(|w| ..)` method takes [`nomt::W`](W) writer structure
impl crate::Writable for NomtSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets NOMT to value 0
impl crate::Resettable for NomtSpec {}
