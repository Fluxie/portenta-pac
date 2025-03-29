///Register `TSCDR` reader
pub type R = crate::R<TscdrSpec>;
///Field `TSCDR` reader - Temperature Sensor Calibration Data
pub type TscdrR = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Temperature Sensor Calibration Data
    #[inline(always)]
    pub fn tscdr(&self) -> TscdrR {
        TscdrR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TSCDR").field("tscdr", &self.tscdr()).finish()
    }
}
/**Temperature Sensor Calibration Data Register

You can [`read`](crate::Reg::read) this register and get [`tscdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TscdrSpec;
impl crate::RegisterSpec for TscdrSpec {
    type Ux = u32;
}
///`read()` method returns [`tscdr::R`](R) reader structure
impl crate::Readable for TscdrSpec {}
///`reset()` method sets TSCDR to value 0
impl crate::Resettable for TscdrSpec {}
