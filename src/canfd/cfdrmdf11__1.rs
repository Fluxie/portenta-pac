///Register `CFDRMDF11_%s_1` reader
pub type R = crate::R<Cfdrmdf11_1Spec>;
///Field `RMDB_LL` reader - RX Message Buffer Data Byte 44
pub type RmdbLlR = crate::FieldReader;
///Field `RMDB_LH` reader - RX Message Buffer Data Byte 45
pub type RmdbLhR = crate::FieldReader;
///Field `RMDB_HL` reader - RX Message Buffer Data Byte 46
pub type RmdbHlR = crate::FieldReader;
///Field `RMDB_HH` reader - RX Message Buffer Data Byte 47
pub type RmdbHhR = crate::FieldReader;
impl R {
    ///Bits 0:7 - RX Message Buffer Data Byte 44
    #[inline(always)]
    pub fn rmdb_ll(&self) -> RmdbLlR {
        RmdbLlR::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - RX Message Buffer Data Byte 45
    #[inline(always)]
    pub fn rmdb_lh(&self) -> RmdbLhR {
        RmdbLhR::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - RX Message Buffer Data Byte 46
    #[inline(always)]
    pub fn rmdb_hl(&self) -> RmdbHlR {
        RmdbHlR::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - RX Message Buffer Data Byte 47
    #[inline(always)]
    pub fn rmdb_hh(&self) -> RmdbHhR {
        RmdbHhR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDRMDF11__1")
            .field("rmdb_ll", &self.rmdb_ll())
            .field("rmdb_lh", &self.rmdb_lh())
            .field("rmdb_hl", &self.rmdb_hl())
            .field("rmdb_hh", &self.rmdb_hh())
            .finish()
    }
}
/**RX Message Buffer Data Field 11 Register %s Channel 1

You can [`read`](crate::Reg::read) this register and get [`cfdrmdf11__1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Cfdrmdf11_1Spec;
impl crate::RegisterSpec for Cfdrmdf11_1Spec {
    type Ux = u32;
}
///`read()` method returns [`cfdrmdf11__1::R`](R) reader structure
impl crate::Readable for Cfdrmdf11_1Spec {}
///`reset()` method sets CFDRMDF11_%s_1 to value 0
impl crate::Resettable for Cfdrmdf11_1Spec {}
