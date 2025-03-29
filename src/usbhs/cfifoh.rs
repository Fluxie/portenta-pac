///Register `CFIFOH` reader
pub type R = crate::R<CfifohSpec>;
///Register `CFIFOH` writer
pub type W = crate::W<CfifohSpec>;
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
        f.debug_struct("CFIFOH").field("fifoport", &self.fifoport()).finish()
    }
}
impl W {
    ///Bits 0:15 - Read receive data from the FIFO buffer or write transmit data to the FIFO buffer by accessing these bits.
    #[inline(always)]
    pub fn fifoport(&mut self) -> FifoportW<CfifohSpec> {
        FifoportW::new(self, 0)
    }
}
/**FIFO Port Register

You can [`read`](crate::Reg::read) this register and get [`cfifoh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfifoh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfifohSpec;
impl crate::RegisterSpec for CfifohSpec {
    type Ux = u16;
}
///`read()` method returns [`cfifoh::R`](R) reader structure
impl crate::Readable for CfifohSpec {}
///`write(|w| ..)` method takes [`cfifoh::W`](W) writer structure
impl crate::Writable for CfifohSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFIFOH to value 0
impl crate::Resettable for CfifohSpec {}
