///Register `SSAMONA` reader
pub type R = crate::R<SsamonaSpec>;
///Field `SS2` reader - SRAM Secure area 2
pub type Ss2R = crate::FieldReader;
impl R {
    ///Bits 13:20 - SRAM Secure area 2
    #[inline(always)]
    pub fn ss2(&self) -> Ss2R {
        Ss2R::new(((self.bits >> 13) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SSAMONA").field("ss2", &self.ss2()).finish()
    }
}
/**SRAM Security Attribution Monitor Register A

You can [`read`](crate::Reg::read) this register and get [`ssamona::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SsamonaSpec;
impl crate::RegisterSpec for SsamonaSpec {
    type Ux = u32;
}
///`read()` method returns [`ssamona::R`](R) reader structure
impl crate::Readable for SsamonaSpec {}
///`reset()` method sets SSAMONA to value 0
impl crate::Resettable for SsamonaSpec {}
