///Register `CFDRMDF9_%s_0` reader
pub type R = crate::R<Cfdrmdf9_0Spec>;
///Field `RMDB_LL` reader - RX Message Buffer Data Byte 36
pub type RmdbLlR = crate::FieldReader;
///Field `RMDB_LH` reader - RX Message Buffer Data Byte 37
pub type RmdbLhR = crate::FieldReader;
///Field `RMDB_HL` reader - RX Message Buffer Data Byte 38
pub type RmdbHlR = crate::FieldReader;
///Field `RMDB_HH` reader - RX Message Buffer Data Byte 39
pub type RmdbHhR = crate::FieldReader;
impl R {
    ///Bits 0:7 - RX Message Buffer Data Byte 36
    #[inline(always)]
    pub fn rmdb_ll(&self) -> RmdbLlR {
        RmdbLlR::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - RX Message Buffer Data Byte 37
    #[inline(always)]
    pub fn rmdb_lh(&self) -> RmdbLhR {
        RmdbLhR::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - RX Message Buffer Data Byte 38
    #[inline(always)]
    pub fn rmdb_hl(&self) -> RmdbHlR {
        RmdbHlR::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - RX Message Buffer Data Byte 39
    #[inline(always)]
    pub fn rmdb_hh(&self) -> RmdbHhR {
        RmdbHhR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDRMDF9__0")
            .field("rmdb_ll", &self.rmdb_ll())
            .field("rmdb_lh", &self.rmdb_lh())
            .field("rmdb_hl", &self.rmdb_hl())
            .field("rmdb_hh", &self.rmdb_hh())
            .finish()
    }
}
/**RX Message Buffer Data Field 9 Register %s Channel 0

You can [`read`](crate::Reg::read) this register and get [`cfdrmdf9__0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Cfdrmdf9_0Spec;
impl crate::RegisterSpec for Cfdrmdf9_0Spec {
    type Ux = u32;
}
///`read()` method returns [`cfdrmdf9__0::R`](R) reader structure
impl crate::Readable for Cfdrmdf9_0Spec {}
///`reset()` method sets CFDRMDF9_%s_0 to value 0
impl crate::Resettable for Cfdrmdf9_0Spec {}
