///Register `CFDRFDF1%s` reader
pub type R = crate::R<Cfdrfdf1Spec>;
///Field `RFDB_LL` reader - RX FIFO Buffer Data Byte 4
pub type RfdbLlR = crate::FieldReader;
///Field `RFDB_LH` reader - RX FIFO Buffer Data Byte 5
pub type RfdbLhR = crate::FieldReader;
///Field `RFDB_HL` reader - RX FIFO Buffer Data Byte 6
pub type RfdbHlR = crate::FieldReader;
///Field `RFDB_HH` reader - RX FIFO Buffer Data Byte 7
pub type RfdbHhR = crate::FieldReader;
impl R {
    ///Bits 0:7 - RX FIFO Buffer Data Byte 4
    #[inline(always)]
    pub fn rfdb_ll(&self) -> RfdbLlR {
        RfdbLlR::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - RX FIFO Buffer Data Byte 5
    #[inline(always)]
    pub fn rfdb_lh(&self) -> RfdbLhR {
        RfdbLhR::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - RX FIFO Buffer Data Byte 6
    #[inline(always)]
    pub fn rfdb_hl(&self) -> RfdbHlR {
        RfdbHlR::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - RX FIFO Buffer Data Byte 7
    #[inline(always)]
    pub fn rfdb_hh(&self) -> RfdbHhR {
        RfdbHhR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDRFDF1")
            .field("rfdb_ll", &self.rfdb_ll())
            .field("rfdb_lh", &self.rfdb_lh())
            .field("rfdb_hl", &self.rfdb_hl())
            .field("rfdb_hh", &self.rfdb_hh())
            .finish()
    }
}
/**RX FIFO Access Data Field 1 Register %s

You can [`read`](crate::Reg::read) this register and get [`cfdrfdf1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Cfdrfdf1Spec;
impl crate::RegisterSpec for Cfdrfdf1Spec {
    type Ux = u32;
}
///`read()` method returns [`cfdrfdf1::R`](R) reader structure
impl crate::Readable for Cfdrfdf1Spec {}
///`reset()` method sets CFDRFDF1%s to value 0
impl crate::Resettable for Cfdrfdf1Spec {}
