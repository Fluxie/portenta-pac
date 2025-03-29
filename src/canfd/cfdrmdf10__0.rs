///Register `CFDRMDF10_%s_0` reader
pub type R = crate::R<Cfdrmdf10_0Spec>;
///Field `RMDB_LL` reader - RX Message Buffer Data Byte 40
pub type RmdbLlR = crate::FieldReader;
///Field `RMDB_LH` reader - RX Message Buffer Data Byte 41
pub type RmdbLhR = crate::FieldReader;
///Field `RMDB_HL` reader - RX Message Buffer Data Byte 42
pub type RmdbHlR = crate::FieldReader;
///Field `RMDB_HH` reader - RX Message Buffer Data Byte 43
pub type RmdbHhR = crate::FieldReader;
impl R {
    ///Bits 0:7 - RX Message Buffer Data Byte 40
    #[inline(always)]
    pub fn rmdb_ll(&self) -> RmdbLlR {
        RmdbLlR::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - RX Message Buffer Data Byte 41
    #[inline(always)]
    pub fn rmdb_lh(&self) -> RmdbLhR {
        RmdbLhR::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - RX Message Buffer Data Byte 42
    #[inline(always)]
    pub fn rmdb_hl(&self) -> RmdbHlR {
        RmdbHlR::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - RX Message Buffer Data Byte 43
    #[inline(always)]
    pub fn rmdb_hh(&self) -> RmdbHhR {
        RmdbHhR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDRMDF10__0")
            .field("rmdb_ll", &self.rmdb_ll())
            .field("rmdb_lh", &self.rmdb_lh())
            .field("rmdb_hl", &self.rmdb_hl())
            .field("rmdb_hh", &self.rmdb_hh())
            .finish()
    }
}
/**RX Message Buffer Data Field 10 Register %s Channel 0

You can [`read`](crate::Reg::read) this register and get [`cfdrmdf10__0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Cfdrmdf10_0Spec;
impl crate::RegisterSpec for Cfdrmdf10_0Spec {
    type Ux = u32;
}
///`read()` method returns [`cfdrmdf10__0::R`](R) reader structure
impl crate::Readable for Cfdrmdf10_0Spec {}
///`reset()` method sets CFDRMDF10_%s_0 to value 0
impl crate::Resettable for Cfdrmdf10_0Spec {}
