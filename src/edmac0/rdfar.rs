///Register `RDFAR` reader
pub type R = crate::R<RdfarSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
/**Receive Descriptor Fetch Address Register

You can [`read`](crate::Reg::read) this register and get [`rdfar::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RdfarSpec;
impl crate::RegisterSpec for RdfarSpec {
    type Ux = u32;
}
///`read()` method returns [`rdfar::R`](R) reader structure
impl crate::Readable for RdfarSpec {}
///`reset()` method sets RDFAR to value 0
impl crate::Resettable for RdfarSpec {}
