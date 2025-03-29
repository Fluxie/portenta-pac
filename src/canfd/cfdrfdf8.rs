///Register `CFDRFDF8%s` reader
pub type R = crate::R<Cfdrfdf8Spec>;
///Field `RFDB_LL` reader - RX FIFO Buffer Data Byte 32
pub type RfdbLlR = crate::FieldReader;
///Field `RFDB_LH` reader - RX FIFO Buffer Data Byte 33
pub type RfdbLhR = crate::FieldReader;
///Field `RFDB_HL` reader - RX FIFO Buffer Data Byte 34
pub type RfdbHlR = crate::FieldReader;
///Field `RFDB_HH` reader - RX FIFO Buffer Data Byte 35
pub type RfdbHhR = crate::FieldReader;
impl R {
    ///Bits 0:7 - RX FIFO Buffer Data Byte 32
    #[inline(always)]
    pub fn rfdb_ll(&self) -> RfdbLlR {
        RfdbLlR::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - RX FIFO Buffer Data Byte 33
    #[inline(always)]
    pub fn rfdb_lh(&self) -> RfdbLhR {
        RfdbLhR::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - RX FIFO Buffer Data Byte 34
    #[inline(always)]
    pub fn rfdb_hl(&self) -> RfdbHlR {
        RfdbHlR::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - RX FIFO Buffer Data Byte 35
    #[inline(always)]
    pub fn rfdb_hh(&self) -> RfdbHhR {
        RfdbHhR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDRFDF8")
            .field("rfdb_ll", &self.rfdb_ll())
            .field("rfdb_lh", &self.rfdb_lh())
            .field("rfdb_hl", &self.rfdb_hl())
            .field("rfdb_hh", &self.rfdb_hh())
            .finish()
    }
}
/**RX FIFO Access Data Field 8 Register %s

You can [`read`](crate::Reg::read) this register and get [`cfdrfdf8::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Cfdrfdf8Spec;
impl crate::RegisterSpec for Cfdrfdf8Spec {
    type Ux = u32;
}
///`read()` method returns [`cfdrfdf8::R`](R) reader structure
impl crate::Readable for Cfdrfdf8Spec {}
///`reset()` method sets CFDRFDF8%s to value 0
impl crate::Resettable for Cfdrfdf8Spec {}
