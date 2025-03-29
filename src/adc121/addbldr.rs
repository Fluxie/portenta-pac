///Register `ADDBLDR` reader
pub type R = crate::R<AddbldrSpec>;
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
        f.debug_struct("ADDBLDR").field("addbldr", &self.addbldr()).finish()
    }
}
/**A/D Data Duplexing Register

You can [`read`](crate::Reg::read) this register and get [`addbldr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AddbldrSpec;
impl crate::RegisterSpec for AddbldrSpec {
    type Ux = u16;
}
///`read()` method returns [`addbldr::R`](R) reader structure
impl crate::Readable for AddbldrSpec {}
///`reset()` method sets ADDBLDR to value 0
impl crate::Resettable for AddbldrSpec {}
