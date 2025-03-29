///Register `CACNTBR` reader
pub type R = crate::R<CacntbrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
/**CAC Counter Buffer Register

You can [`read`](crate::Reg::read) this register and get [`cacntbr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CacntbrSpec;
impl crate::RegisterSpec for CacntbrSpec {
    type Ux = u16;
}
///`read()` method returns [`cacntbr::R`](R) reader structure
impl crate::Readable for CacntbrSpec {}
///`reset()` method sets CACNTBR to value 0
impl crate::Resettable for CacntbrSpec {}
