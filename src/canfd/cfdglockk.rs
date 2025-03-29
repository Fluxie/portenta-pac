///Register `CFDGLOCKK` writer
pub type W = crate::W<CfdglockkSpec>;
///Field `LOCK` writer - Lock Key
pub type LockW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl core::fmt::Debug for crate::generic::Reg<CfdglockkSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:15 - Lock Key
    #[inline(always)]
    pub fn lock(&mut self) -> LockW<CfdglockkSpec> {
        LockW::new(self, 0)
    }
}
/**Global Lock Key Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdglockk::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdglockkSpec;
impl crate::RegisterSpec for CfdglockkSpec {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`cfdglockk::W`](W) writer structure
impl crate::Writable for CfdglockkSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDGLOCKK to value 0
impl crate::Resettable for CfdglockkSpec {}
