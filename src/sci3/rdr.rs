///Register `RDR` reader
pub type R = crate::R<RdrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
/**Receive Data Register

You can [`read`](crate::Reg::read) this register and get [`rdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RdrSpec;
impl crate::RegisterSpec for RdrSpec {
    type Ux = u8;
}
///`read()` method returns [`rdr::R`](R) reader structure
impl crate::Readable for RdrSpec {}
///`reset()` method sets RDR to value 0
impl crate::Resettable for RdrSpec {}
