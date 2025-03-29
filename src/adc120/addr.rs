///Register `ADDR%s` reader
pub type R = crate::R<AddrSpec>;
///Field `ADDR` reader - Converted Value 15 to 0
pub type AddrR = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Converted Value 15 to 0
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADDR").field("addr", &self.addr()).finish()
    }
}
/**A/D Data Registers %s

You can [`read`](crate::Reg::read) this register and get [`addr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AddrSpec;
impl crate::RegisterSpec for AddrSpec {
    type Ux = u16;
}
///`read()` method returns [`addr::R`](R) reader structure
impl crate::Readable for AddrSpec {}
///`reset()` method sets ADDR%s to value 0
impl crate::Resettable for AddrSpec {}
