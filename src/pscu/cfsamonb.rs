///Register `CFSAMONB` reader
pub type R = crate::R<CfsamonbSpec>;
///Field `CFS1` reader - Code Flash Secure area 1
pub type Cfs1R = crate::FieldReader<u16>;
impl R {
    ///Bits 10:23 - Code Flash Secure area 1
    #[inline(always)]
    pub fn cfs1(&self) -> Cfs1R {
        Cfs1R::new(((self.bits >> 10) & 0x3fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFSAMONB").field("cfs1", &self.cfs1()).finish()
    }
}
/**Code Flash Security Attribution Monitor Register B

You can [`read`](crate::Reg::read) this register and get [`cfsamonb::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfsamonbSpec;
impl crate::RegisterSpec for CfsamonbSpec {
    type Ux = u32;
}
///`read()` method returns [`cfsamonb::R`](R) reader structure
impl crate::Readable for CfsamonbSpec {}
///`reset()` method sets CFSAMONB to value 0
impl crate::Resettable for CfsamonbSpec {}
