///Register `D1FIFO` reader
pub type R = crate::R<D1fifoSpec>;
///Register `D1FIFO` writer
pub type W = crate::W<D1fifoSpec>;
///Field `FIFOPORT` reader - Read receive data from the FIFO buffer or write transmit data to the FIFO buffer by accessing these bits.
pub type FifoportR = crate::FieldReader<u32>;
///Field `FIFOPORT` writer - Read receive data from the FIFO buffer or write transmit data to the FIFO buffer by accessing these bits.
pub type FifoportW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Read receive data from the FIFO buffer or write transmit data to the FIFO buffer by accessing these bits.
    #[inline(always)]
    pub fn fifoport(&self) -> FifoportR {
        FifoportR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("D1FIFO").field("fifoport", &self.fifoport()).finish()
    }
}
impl W {
    ///Bits 0:31 - Read receive data from the FIFO buffer or write transmit data to the FIFO buffer by accessing these bits.
    #[inline(always)]
    pub fn fifoport(&mut self) -> FifoportW<D1fifoSpec> {
        FifoportW::new(self, 0)
    }
}
/**FIFO Port Register

You can [`read`](crate::Reg::read) this register and get [`d1fifo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d1fifo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct D1fifoSpec;
impl crate::RegisterSpec for D1fifoSpec {
    type Ux = u32;
}
///`read()` method returns [`d1fifo::R`](R) reader structure
impl crate::Readable for D1fifoSpec {}
///`write(|w| ..)` method takes [`d1fifo::W`](W) writer structure
impl crate::Writable for D1fifoSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets D1FIFO to value 0
impl crate::Resettable for D1fifoSpec {}
