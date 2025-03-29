///Register `DFSAMON` reader
pub type R = crate::R<DfsamonSpec>;
///Field `DFS` reader - Data flash Secure area
pub type DfsR = crate::FieldReader;
impl R {
    ///Bits 10:15 - Data flash Secure area
    #[inline(always)]
    pub fn dfs(&self) -> DfsR {
        DfsR::new(((self.bits >> 10) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DFSAMON").field("dfs", &self.dfs()).finish()
    }
}
/**Data Flash Security Attribution Monitor Register

You can [`read`](crate::Reg::read) this register and get [`dfsamon::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DfsamonSpec;
impl crate::RegisterSpec for DfsamonSpec {
    type Ux = u32;
}
///`read()` method returns [`dfsamon::R`](R) reader structure
impl crate::Readable for DfsamonSpec {}
///`reset()` method sets DFSAMON to value 0
impl crate::Resettable for DfsamonSpec {}
