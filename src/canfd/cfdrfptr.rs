///Register `CFDRFPTR%s` reader
pub type R = crate::R<CfdrfptrSpec>;
///Field `RFTS` reader - RX FIFO Timestamp Value
pub type RftsR = crate::FieldReader<u16>;
///Field `RFDLC` reader - RX FIFO Buffer DLC Field
pub type RfdlcR = crate::FieldReader;
impl R {
    ///Bits 0:15 - RX FIFO Timestamp Value
    #[inline(always)]
    pub fn rfts(&self) -> RftsR {
        RftsR::new((self.bits & 0xffff) as u16)
    }
    ///Bits 28:31 - RX FIFO Buffer DLC Field
    #[inline(always)]
    pub fn rfdlc(&self) -> RfdlcR {
        RfdlcR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDRFPTR")
            .field("rfts", &self.rfts())
            .field("rfdlc", &self.rfdlc())
            .finish()
    }
}
/**RX FIFO Access Pointer Register %s

You can [`read`](crate::Reg::read) this register and get [`cfdrfptr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdrfptrSpec;
impl crate::RegisterSpec for CfdrfptrSpec {
    type Ux = u32;
}
///`read()` method returns [`cfdrfptr::R`](R) reader structure
impl crate::Readable for CfdrfptrSpec {}
///`reset()` method sets CFDRFPTR%s to value 0
impl crate::Resettable for CfdrfptrSpec {}
