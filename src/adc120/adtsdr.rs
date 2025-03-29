///Register `ADTSDR` reader
pub type R = crate::R<AdtsdrSpec>;
///Field `ADTSDR` reader - Converted Value 15 to 0
pub type AdtsdrR = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Converted Value 15 to 0
    #[inline(always)]
    pub fn adtsdr(&self) -> AdtsdrR {
        AdtsdrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADTSDR").field("adtsdr", &self.adtsdr()).finish()
    }
}
/**A/D Temperature Sensor Data Register

You can [`read`](crate::Reg::read) this register and get [`adtsdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AdtsdrSpec;
impl crate::RegisterSpec for AdtsdrSpec {
    type Ux = u16;
}
///`read()` method returns [`adtsdr::R`](R) reader structure
impl crate::Readable for AdtsdrSpec {}
///`reset()` method sets ADTSDR to value 0
impl crate::Resettable for AdtsdrSpec {}
