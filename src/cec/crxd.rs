///Register `CRXD` reader
pub type R = crate::R<CrxdSpec>;
///Register `CRXD` writer
pub type W = crate::W<CrxdSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**CEC Reception Buffer Register

You can [`read`](crate::Reg::read) this register and get [`crxd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crxd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CrxdSpec;
impl crate::RegisterSpec for CrxdSpec {
    type Ux = u8;
}
///`read()` method returns [`crxd::R`](R) reader structure
impl crate::Readable for CrxdSpec {}
///`write(|w| ..)` method takes [`crxd::W`](W) writer structure
impl crate::Writable for CrxdSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CRXD to value 0
impl crate::Resettable for CrxdSpec {}
