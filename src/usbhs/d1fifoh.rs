///Register `D1FIFOH` reader
pub type R = crate::R<D1fifohSpec>;
///Register `D1FIFOH` writer
pub type W = crate::W<D1fifohSpec>;
///Field `FIFOPORT` reader - Read receive data from the FIFO buffer or write transmit data to the FIFO buffer by accessing these bits.
pub type FifoportR = crate::FieldReader<u16>;
///Field `FIFOPORT` writer - Read receive data from the FIFO buffer or write transmit data to the FIFO buffer by accessing these bits.
pub type FifoportW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Read receive data from the FIFO buffer or write transmit data to the FIFO buffer by accessing these bits.
    #[inline(always)]
    pub fn fifoport(&self) -> FifoportR {
        FifoportR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("D1FIFOH").field("fifoport", &self.fifoport()).finish()
    }
}
impl W {
    ///Bits 0:15 - Read receive data from the FIFO buffer or write transmit data to the FIFO buffer by accessing these bits.
    #[inline(always)]
    pub fn fifoport(&mut self) -> FifoportW<D1fifohSpec> {
        FifoportW::new(self, 0)
    }
}
/**FIFO Port Register

You can [`read`](crate::Reg::read) this register and get [`d1fifoh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d1fifoh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct D1fifohSpec;
impl crate::RegisterSpec for D1fifohSpec {
    type Ux = u16;
}
///`read()` method returns [`d1fifoh::R`](R) reader structure
impl crate::Readable for D1fifohSpec {}
///`write(|w| ..)` method takes [`d1fifoh::W`](W) writer structure
impl crate::Writable for D1fifohSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets D1FIFOH to value 0
impl crate::Resettable for D1fifohSpec {}
