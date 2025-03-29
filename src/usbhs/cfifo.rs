///Register `CFIFO` reader
pub type R = crate::R<CfifoSpec>;
///Register `CFIFO` writer
pub type W = crate::W<CfifoSpec>;
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
        f.debug_struct("CFIFO").field("fifoport", &self.fifoport()).finish()
    }
}
impl W {
    ///Bits 0:31 - Read receive data from the FIFO buffer or write transmit data to the FIFO buffer by accessing these bits.
    #[inline(always)]
    pub fn fifoport(&mut self) -> FifoportW<CfifoSpec> {
        FifoportW::new(self, 0)
    }
}
/**FIFO Port Register

You can [`read`](crate::Reg::read) this register and get [`cfifo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfifo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfifoSpec;
impl crate::RegisterSpec for CfifoSpec {
    type Ux = u32;
}
///`read()` method returns [`cfifo::R`](R) reader structure
impl crate::Readable for CfifoSpec {}
///`write(|w| ..)` method takes [`cfifo::W`](W) writer structure
impl crate::Writable for CfifoSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFIFO to value 0
impl crate::Resettable for CfifoSpec {}
