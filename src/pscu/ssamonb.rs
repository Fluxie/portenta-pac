///Register `SSAMONB` reader
pub type R = crate::R<SsamonbSpec>;
///Field `SS1` reader - SRAM secure area 1
pub type Ss1R = crate::FieldReader<u16>;
impl R {
    ///Bits 10:20 - SRAM secure area 1
    #[inline(always)]
    pub fn ss1(&self) -> Ss1R {
        Ss1R::new(((self.bits >> 10) & 0x07ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SSAMONB").field("ss1", &self.ss1()).finish()
    }
}
/**SRAM Security Attribution Monitor Register B

You can [`read`](crate::Reg::read) this register and get [`ssamonb::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SsamonbSpec;
impl crate::RegisterSpec for SsamonbSpec {
    type Ux = u32;
}
///`read()` method returns [`ssamonb::R`](R) reader structure
impl crate::Readable for SsamonbSpec {}
///`reset()` method sets SSAMONB to value 0
impl crate::Resettable for SsamonbSpec {}
