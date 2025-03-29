///Register `CFDRMDF8_%s_1` reader
pub type R = crate::R<Cfdrmdf8_1Spec>;
///Field `RMDB_LL` reader - RX Message Buffer Data Byte (p * 4)
pub type RmdbLlR = crate::FieldReader;
///Field `RMDB_LH` reader - RX Message Buffer Data Byte ((p * 4) + 1)
pub type RmdbLhR = crate::FieldReader;
///Field `RMDB_HL` reader - RX Message Buffer Data Byte ((p * 4) + 2)
pub type RmdbHlR = crate::FieldReader;
///Field `RMDB_HH` reader - RX Message Buffer Data Byte ((p * 4) + 3)
pub type RmdbHhR = crate::FieldReader;
impl R {
    ///Bits 0:7 - RX Message Buffer Data Byte (p * 4)
    #[inline(always)]
    pub fn rmdb_ll(&self) -> RmdbLlR {
        RmdbLlR::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - RX Message Buffer Data Byte ((p * 4) + 1)
    #[inline(always)]
    pub fn rmdb_lh(&self) -> RmdbLhR {
        RmdbLhR::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - RX Message Buffer Data Byte ((p * 4) + 2)
    #[inline(always)]
    pub fn rmdb_hl(&self) -> RmdbHlR {
        RmdbHlR::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - RX Message Buffer Data Byte ((p * 4) + 3)
    #[inline(always)]
    pub fn rmdb_hh(&self) -> RmdbHhR {
        RmdbHhR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDRMDF8__1")
            .field("rmdb_ll", &self.rmdb_ll())
            .field("rmdb_lh", &self.rmdb_lh())
            .field("rmdb_hl", &self.rmdb_hl())
            .field("rmdb_hh", &self.rmdb_hh())
            .finish()
    }
}
/**RX Message Buffer Data Field 8 Register %s Channel 1

You can [`read`](crate::Reg::read) this register and get [`cfdrmdf8__1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Cfdrmdf8_1Spec;
impl crate::RegisterSpec for Cfdrmdf8_1Spec {
    type Ux = u32;
}
///`read()` method returns [`cfdrmdf8__1::R`](R) reader structure
impl crate::Readable for Cfdrmdf8_1Spec {}
///`reset()` method sets CFDRMDF8_%s_1 to value 0
impl crate::Resettable for Cfdrmdf8_1Spec {}
