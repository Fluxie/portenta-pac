///Register `SD_SIZE` reader
pub type R = crate::R<SdSizeSpec>;
///Register `SD_SIZE` writer
pub type W = crate::W<SdSizeSpec>;
///Field `LEN` reader - Transfer Data Size Setting
pub type LenR = crate::FieldReader<u16>;
///Field `LEN` writer - Transfer Data Size Setting
pub type LenW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bits 0:9 - Transfer Data Size Setting
    #[inline(always)]
    pub fn len(&self) -> LenR {
        LenR::new((self.bits & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SD_SIZE").field("len", &self.len()).finish()
    }
}
impl W {
    ///Bits 0:9 - Transfer Data Size Setting
    #[inline(always)]
    pub fn len(&mut self) -> LenW<SdSizeSpec> {
        LenW::new(self, 0)
    }
}
/**Transfer Data Length Register

You can [`read`](crate::Reg::read) this register and get [`sd_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SdSizeSpec;
impl crate::RegisterSpec for SdSizeSpec {
    type Ux = u32;
}
///`read()` method returns [`sd_size::R`](R) reader structure
impl crate::Readable for SdSizeSpec {}
///`write(|w| ..)` method takes [`sd_size::W`](W) writer structure
impl crate::Writable for SdSizeSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SD_SIZE to value 0x0200
impl crate::Resettable for SdSizeSpec {
    const RESET_VALUE: u32 = 0x0200;
}
