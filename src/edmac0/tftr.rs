///Register `TFTR` reader
pub type R = crate::R<TftrSpec>;
///Register `TFTR` writer
pub type W = crate::W<TftrSpec>;
///Field `TFT` reader - Transmit FIFO Threshold
pub type TftR = crate::FieldReader<u16>;
///Field `TFT` writer - Transmit FIFO Threshold
pub type TftW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    ///Bits 0:10 - Transmit FIFO Threshold
    #[inline(always)]
    pub fn tft(&self) -> TftR {
        TftR::new((self.bits & 0x07ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TFTR").field("tft", &self.tft()).finish()
    }
}
impl W {
    ///Bits 0:10 - Transmit FIFO Threshold
    #[inline(always)]
    pub fn tft(&mut self) -> TftW<TftrSpec> {
        TftW::new(self, 0)
    }
}
/**Transmit FIFO Threshold Register

You can [`read`](crate::Reg::read) this register and get [`tftr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tftr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TftrSpec;
impl crate::RegisterSpec for TftrSpec {
    type Ux = u32;
}
///`read()` method returns [`tftr::R`](R) reader structure
impl crate::Readable for TftrSpec {}
///`write(|w| ..)` method takes [`tftr::W`](W) writer structure
impl crate::Writable for TftrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TFTR to value 0
impl crate::Resettable for TftrSpec {}
