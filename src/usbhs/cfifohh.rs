///Register `CFIFOHH` reader
pub type R = crate::R<CfifohhSpec>;
///Register `CFIFOHH` writer
pub type W = crate::W<CfifohhSpec>;
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
        f.debug_struct("CFIFOHH").field("fifoport", &self.fifoport()).finish()
    }
}
impl W {
    ///Bits 0:7 - Read receive data from the FIFO buffer or write transmit data to the FIFO buffer by accessing these bits.
    #[inline(always)]
    pub fn fifoport(&mut self) -> FifoportW<CfifohhSpec> {
        FifoportW::new(self, 0)
    }
}
/**FIFO Port Register

You can [`read`](crate::Reg::read) this register and get [`cfifohh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfifohh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfifohhSpec;
impl crate::RegisterSpec for CfifohhSpec {
    type Ux = u8;
}
///`read()` method returns [`cfifohh::R`](R) reader structure
impl crate::Readable for CfifohhSpec {}
///`write(|w| ..)` method takes [`cfifohh::W`](W) writer structure
impl crate::Writable for CfifohhSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFIFOHH to value 0
impl crate::Resettable for CfifohhSpec {}
