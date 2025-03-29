///Register `BTZF%sERRADD` reader
pub type R = crate::R<BtzferraddSpec>;
///Field `BTZFERAD` reader - Bus TrustZone Filter Error Address
pub type BtzferadR = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Bus TrustZone Filter Error Address
    #[inline(always)]
    pub fn btzferad(&self) -> BtzferadR {
        BtzferadR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BTZFERRADD").field("btzferad", &self.btzferad()).finish()
    }
}
/**BUS TZF Error Address Register

You can [`read`](crate::Reg::read) this register and get [`btzferradd::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BtzferraddSpec;
impl crate::RegisterSpec for BtzferraddSpec {
    type Ux = u32;
}
///`read()` method returns [`btzferradd::R`](R) reader structure
impl crate::Readable for BtzferraddSpec {}
///`reset()` method sets BTZF%sERRADD to value 0
impl crate::Resettable for BtzferraddSpec {}
