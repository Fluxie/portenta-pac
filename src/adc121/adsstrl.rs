///Register `ADSSTRL` reader
pub type R = crate::R<AdsstrlSpec>;
///Register `ADSSTRL` writer
pub type W = crate::W<AdsstrlSpec>;
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
        f.debug_struct("ADSSTRL").field("sst", &self.sst()).finish()
    }
}
impl W {
    ///Bits 0:7 - Sampling Time Setting
    #[inline(always)]
    pub fn sst(&mut self) -> SstW<AdsstrlSpec> {
        SstW::new(self, 0)
    }
}
/**A/D Sampling State Register

You can [`read`](crate::Reg::read) this register and get [`adsstrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adsstrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AdsstrlSpec;
impl crate::RegisterSpec for AdsstrlSpec {
    type Ux = u8;
}
///`read()` method returns [`adsstrl::R`](R) reader structure
impl crate::Readable for AdsstrlSpec {}
///`write(|w| ..)` method takes [`adsstrl::W`](W) writer structure
impl crate::Writable for AdsstrlSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADSSTRL to value 0x0b
impl crate::Resettable for AdsstrlSpec {
    const RESET_VALUE: u8 = 0x0b;
}
