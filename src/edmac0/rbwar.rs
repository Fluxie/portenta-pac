///Register `RBWAR` reader
pub type R = crate::R<RbwarSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
/**Receive Buffer Write Address Register

You can [`read`](crate::Reg::read) this register and get [`rbwar::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RbwarSpec;
impl crate::RegisterSpec for RbwarSpec {
    type Ux = u32;
}
///`read()` method returns [`rbwar::R`](R) reader structure
impl crate::Readable for RbwarSpec {}
///`reset()` method sets RBWAR to value 0
impl crate::Resettable for RbwarSpec {}
