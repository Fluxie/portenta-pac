///Register `SSIFTDR` writer
pub type W = crate::W<SsiftdrSpec>;
///Field `SSIFTDR` writer - Transmit FIFO Data
pub type SsiftdrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<SsiftdrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - Transmit FIFO Data
    #[inline(always)]
    pub fn ssiftdr(&mut self) -> SsiftdrW<SsiftdrSpec> {
        SsiftdrW::new(self, 0)
    }
}
/**Transmit FIFO Data Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssiftdr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SsiftdrSpec;
impl crate::RegisterSpec for SsiftdrSpec {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`ssiftdr::W`](W) writer structure
impl crate::Writable for SsiftdrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SSIFTDR to value 0
impl crate::Resettable for SsiftdrSpec {}
