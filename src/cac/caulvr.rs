///Register `CAULVR` reader
pub type R = crate::R<CaulvrSpec>;
///Register `CAULVR` writer
pub type W = crate::W<CaulvrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**CAC Upper-Limit Value Setting Register

You can [`read`](crate::Reg::read) this register and get [`caulvr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`caulvr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CaulvrSpec;
impl crate::RegisterSpec for CaulvrSpec {
    type Ux = u16;
}
///`read()` method returns [`caulvr::R`](R) reader structure
impl crate::Readable for CaulvrSpec {}
///`write(|w| ..)` method takes [`caulvr::W`](W) writer structure
impl crate::Writable for CaulvrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CAULVR to value 0
impl crate::Resettable for CaulvrSpec {}
