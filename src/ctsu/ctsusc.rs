///Register `CTSUSC` reader
pub type R = crate::R<CtsuscSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
/**CTSU Sensor Counter

You can [`read`](crate::Reg::read) this register and get [`ctsusc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CtsuscSpec;
impl crate::RegisterSpec for CtsuscSpec {
    type Ux = u16;
}
///`read()` method returns [`ctsusc::R`](R) reader structure
impl crate::Readable for CtsuscSpec {}
///`reset()` method sets CTSUSC to value 0
impl crate::Resettable for CtsuscSpec {}
