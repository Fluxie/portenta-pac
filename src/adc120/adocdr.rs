///Register `ADOCDR` reader
pub type R = crate::R<AdocdrSpec>;
///Field `ADOCDR` reader - Converted Value 15 to 0
pub type AdocdrR = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Converted Value 15 to 0
    #[inline(always)]
    pub fn adocdr(&self) -> AdocdrR {
        AdocdrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADOCDR").field("adocdr", &self.adocdr()).finish()
    }
}
/**A/D Internal Reference Voltage Data Register

You can [`read`](crate::Reg::read) this register and get [`adocdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AdocdrSpec;
impl crate::RegisterSpec for AdocdrSpec {
    type Ux = u16;
}
///`read()` method returns [`adocdr::R`](R) reader structure
impl crate::Readable for AdocdrSpec {}
///`reset()` method sets ADOCDR to value 0
impl crate::Resettable for AdocdrSpec {}
