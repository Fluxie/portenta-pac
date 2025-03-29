///Register `CFSAMONA` reader
pub type R = crate::R<CfsamonaSpec>;
///Field `CFS2` reader - Code Flash Secure area 2
pub type Cfs2R = crate::FieldReader<u16>;
impl R {
    ///Bits 15:23 - Code Flash Secure area 2
    #[inline(always)]
    pub fn cfs2(&self) -> Cfs2R {
        Cfs2R::new(((self.bits >> 15) & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFSAMONA").field("cfs2", &self.cfs2()).finish()
    }
}
/**Code Flash Security Attribution Monitor Register A

You can [`read`](crate::Reg::read) this register and get [`cfsamona::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfsamonaSpec;
impl crate::RegisterSpec for CfsamonaSpec {
    type Ux = u32;
}
///`read()` method returns [`cfsamona::R`](R) reader structure
impl crate::Readable for CfsamonaSpec {}
///`reset()` method sets CFSAMONA to value 0
impl crate::Resettable for CfsamonaSpec {}
