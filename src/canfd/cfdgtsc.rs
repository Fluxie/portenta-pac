///Register `CFDGTSC` reader
pub type R = crate::R<CfdgtscSpec>;
///Field `TS` reader - Timestamp value
pub type TsR = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Timestamp value
    #[inline(always)]
    pub fn ts(&self) -> TsR {
        TsR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDGTSC").field("ts", &self.ts()).finish()
    }
}
/**Global Timestamp Counter Register

You can [`read`](crate::Reg::read) this register and get [`cfdgtsc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdgtscSpec;
impl crate::RegisterSpec for CfdgtscSpec {
    type Ux = u32;
}
///`read()` method returns [`cfdgtsc::R`](R) reader structure
impl crate::Readable for CfdgtscSpec {}
///`reset()` method sets CFDGTSC to value 0
impl crate::Resettable for CfdgtscSpec {}
