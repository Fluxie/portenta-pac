///Register `CWNDR` writer
pub type W = crate::W<CwndrSpec>;
///Field `WND` writer - The write value should be 0.
pub type WndW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<CwndrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - The write value should be 0.
    #[inline(always)]
    pub fn wnd(&mut self) -> WndW<CwndrSpec> {
        WndW::new(self, 0)
    }
}
/**Configure Write without Data Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cwndr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CwndrSpec;
impl crate::RegisterSpec for CwndrSpec {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`cwndr::W`](W) writer structure
impl crate::Writable for CwndrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CWNDR to value 0
impl crate::Resettable for CwndrSpec {}
