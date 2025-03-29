///Register `ICDRR` reader
pub type R = crate::R<IcdrrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
/**I2C Bus Receive Data Register

You can [`read`](crate::Reg::read) this register and get [`icdrr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IcdrrSpec;
impl crate::RegisterSpec for IcdrrSpec {
    type Ux = u8;
}
///`read()` method returns [`icdrr::R`](R) reader structure
impl crate::Readable for IcdrrSpec {}
///`reset()` method sets ICDRR to value 0
impl crate::Resettable for IcdrrSpec {}
