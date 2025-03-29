///Register `ADDBLDRB` reader
pub type R = crate::R<AddbldrbSpec>;
///Field `ADDBLDR` reader - Converted Value 15 to 0
pub type AddbldrR = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Converted Value 15 to 0
    #[inline(always)]
    pub fn addbldr(&self) -> AddbldrR {
        AddbldrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADDBLDRB").field("addbldr", &self.addbldr()).finish()
    }
}
/**A/D Data Duplexing Register B

You can [`read`](crate::Reg::read) this register and get [`addbldrb::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AddbldrbSpec;
impl crate::RegisterSpec for AddbldrbSpec {
    type Ux = u16;
}
///`read()` method returns [`addbldrb::R`](R) reader structure
impl crate::Readable for AddbldrbSpec {}
///`reset()` method sets ADDBLDRB to value 0
impl crate::Resettable for AddbldrbSpec {}
