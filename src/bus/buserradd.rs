///Register `BUS%sERRADD` reader
pub type R = crate::R<BuserraddSpec>;
///Field `BERAD` reader - Bus Error Address
pub type BeradR = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Bus Error Address
    #[inline(always)]
    pub fn berad(&self) -> BeradR {
        BeradR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BUSERRADD").field("berad", &self.berad()).finish()
    }
}
/**BUS Error Address Register

You can [`read`](crate::Reg::read) this register and get [`buserradd::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BuserraddSpec;
impl crate::RegisterSpec for BuserraddSpec {
    type Ux = u32;
}
///`read()` method returns [`buserradd::R`](R) reader structure
impl crate::Readable for BuserraddSpec {}
///`reset()` method sets BUS%sERRADD to value 0
impl crate::Resettable for BuserraddSpec {}
