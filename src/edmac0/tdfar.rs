///Register `TDFAR` reader
pub type R = crate::R<TdfarSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
/**Transmit Descriptor Fetch Address Register

You can [`read`](crate::Reg::read) this register and get [`tdfar::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TdfarSpec;
impl crate::RegisterSpec for TdfarSpec {
    type Ux = u32;
}
///`read()` method returns [`tdfar::R`](R) reader structure
impl crate::Readable for TdfarSpec {}
///`reset()` method sets TDFAR to value 0
impl crate::Resettable for TdfarSpec {}
