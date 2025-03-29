///Register `CTXD` reader
pub type R = crate::R<CtxdSpec>;
///Register `CTXD` writer
pub type W = crate::W<CtxdSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**CEC Transmission Buffer Register

You can [`read`](crate::Reg::read) this register and get [`ctxd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctxd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CtxdSpec;
impl crate::RegisterSpec for CtxdSpec {
    type Ux = u8;
}
///`read()` method returns [`ctxd::R`](R) reader structure
impl crate::Readable for CtxdSpec {}
///`write(|w| ..)` method takes [`ctxd::W`](W) writer structure
impl crate::Writable for CtxdSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CTXD to value 0
impl crate::Resettable for CtxdSpec {}
