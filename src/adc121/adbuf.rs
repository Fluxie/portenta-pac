///Register `ADBUF%s` reader
pub type R = crate::R<AdbufSpec>;
///Field `ADBUF` reader - Converted Value 15 to 0
pub type AdbufR = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Converted Value 15 to 0
    #[inline(always)]
    pub fn adbuf(&self) -> AdbufR {
        AdbufR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADBUF").field("adbuf", &self.adbuf()).finish()
    }
}
/**A/D Data Buffer Registers %s

You can [`read`](crate::Reg::read) this register and get [`adbuf::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AdbufSpec;
impl crate::RegisterSpec for AdbufSpec {
    type Ux = u16;
}
///`read()` method returns [`adbuf::R`](R) reader structure
impl crate::Readable for AdbufSpec {}
///`reset()` method sets ADBUF%s to value 0
impl crate::Resettable for AdbufSpec {}
