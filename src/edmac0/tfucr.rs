///Register `TFUCR` reader
pub type R = crate::R<TfucrSpec>;
///Register `TFUCR` writer
pub type W = crate::W<TfucrSpec>;
///Field `UNDER` reader - Transmit FIFO Underflow Count
pub type UnderR = crate::FieldReader<u16>;
///Field `UNDER` writer - Transmit FIFO Underflow Count
pub type UnderW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Transmit FIFO Underflow Count
    #[inline(always)]
    pub fn under(&self) -> UnderR {
        UnderR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TFUCR").field("under", &self.under()).finish()
    }
}
impl W {
    ///Bits 0:15 - Transmit FIFO Underflow Count
    #[inline(always)]
    pub fn under(&mut self) -> UnderW<TfucrSpec> {
        UnderW::new(self, 0)
    }
}
/**Transmit FIFO Underflow Counter

You can [`read`](crate::Reg::read) this register and get [`tfucr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tfucr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TfucrSpec;
impl crate::RegisterSpec for TfucrSpec {
    type Ux = u32;
}
///`read()` method returns [`tfucr::R`](R) reader structure
impl crate::Readable for TfucrSpec {}
///`write(|w| ..)` method takes [`tfucr::W`](W) writer structure
impl crate::Writable for TfucrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TFUCR to value 0
impl crate::Resettable for TfucrSpec {}
