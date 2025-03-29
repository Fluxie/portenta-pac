///Register `D0FIFOHH` reader
pub type R = crate::R<D0fifohhSpec>;
///Register `D0FIFOHH` writer
pub type W = crate::W<D0fifohhSpec>;
///Field `FIFOPORT` reader - Read receive data from the FIFO buffer or write transmit data to the FIFO buffer by accessing these bits.
pub type FifoportR = crate::FieldReader;
///Field `FIFOPORT` writer - Read receive data from the FIFO buffer or write transmit data to the FIFO buffer by accessing these bits.
pub type FifoportW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Read receive data from the FIFO buffer or write transmit data to the FIFO buffer by accessing these bits.
    #[inline(always)]
    pub fn fifoport(&self) -> FifoportR {
        FifoportR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("D0FIFOHH").field("fifoport", &self.fifoport()).finish()
    }
}
impl W {
    ///Bits 0:7 - Read receive data from the FIFO buffer or write transmit data to the FIFO buffer by accessing these bits.
    #[inline(always)]
    pub fn fifoport(&mut self) -> FifoportW<D0fifohhSpec> {
        FifoportW::new(self, 0)
    }
}
/**FIFO Port Register

You can [`read`](crate::Reg::read) this register and get [`d0fifohh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d0fifohh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct D0fifohhSpec;
impl crate::RegisterSpec for D0fifohhSpec {
    type Ux = u8;
}
///`read()` method returns [`d0fifohh::R`](R) reader structure
impl crate::Readable for D0fifohhSpec {}
///`write(|w| ..)` method takes [`d0fifohh::W`](W) writer structure
impl crate::Writable for D0fifohhSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets D0FIFOHH to value 0
impl crate::Resettable for D0fifohhSpec {}
