///Register `ADSSTRO` reader
pub type R = crate::R<AdsstroSpec>;
///Register `ADSSTRO` writer
pub type W = crate::W<AdsstroSpec>;
///Field `SST` reader - Sampling Time Setting
pub type SstR = crate::FieldReader;
///Field `SST` writer - Sampling Time Setting
pub type SstW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Sampling Time Setting
    #[inline(always)]
    pub fn sst(&self) -> SstR {
        SstR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADSSTRO").field("sst", &self.sst()).finish()
    }
}
impl W {
    ///Bits 0:7 - Sampling Time Setting
    #[inline(always)]
    pub fn sst(&mut self) -> SstW<AdsstroSpec> {
        SstW::new(self, 0)
    }
}
/**A/D Sampling State Register

You can [`read`](crate::Reg::read) this register and get [`adsstro::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adsstro::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AdsstroSpec;
impl crate::RegisterSpec for AdsstroSpec {
    type Ux = u8;
}
///`read()` method returns [`adsstro::R`](R) reader structure
impl crate::Readable for AdsstroSpec {}
///`write(|w| ..)` method takes [`adsstro::W`](W) writer structure
impl crate::Writable for AdsstroSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADSSTRO to value 0x0b
impl crate::Resettable for AdsstroSpec {
    const RESET_VALUE: u8 = 0x0b;
}
