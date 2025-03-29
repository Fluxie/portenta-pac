///Register `D%sFIFO` reader
pub type R = crate::R<DfifoSpec>;
///Register `D%sFIFO` writer
pub type W = crate::W<DfifoSpec>;
///Field `FIFOPORT` reader - FIFO Port
pub type FifoportR = crate::FieldReader<u16>;
///Field `FIFOPORT` writer - FIFO Port
pub type FifoportW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - FIFO Port
    #[inline(always)]
    pub fn fifoport(&self) -> FifoportR {
        FifoportR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DFIFO").field("fifoport", &self.fifoport()).finish()
    }
}
impl W {
    ///Bits 0:15 - FIFO Port
    #[inline(always)]
    pub fn fifoport(&mut self) -> FifoportW<DfifoSpec> {
        FifoportW::new(self, 0)
    }
}
/**D%sFIFO Port Register

You can [`read`](crate::Reg::read) this register and get [`dfifo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfifo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DfifoSpec;
impl crate::RegisterSpec for DfifoSpec {
    type Ux = u16;
}
///`read()` method returns [`dfifo::R`](R) reader structure
impl crate::Readable for DfifoSpec {}
///`write(|w| ..)` method takes [`dfifo::W`](W) writer structure
impl crate::Writable for DfifoSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets D%sFIFO to value 0
impl crate::Resettable for DfifoSpec {}
