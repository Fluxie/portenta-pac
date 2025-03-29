///Register `CTSUMCH1` reader
pub type R = crate::R<Ctsumch1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
/**CTSU Measurement Channel Register 1

You can [`read`](crate::Reg::read) this register and get [`ctsumch1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Ctsumch1Spec;
impl crate::RegisterSpec for Ctsumch1Spec {
    type Ux = u8;
}
///`read()` method returns [`ctsumch1::R`](R) reader structure
impl crate::Readable for Ctsumch1Spec {}
///`reset()` method sets CTSUMCH1 to value 0x1f
impl crate::Resettable for Ctsumch1Spec {
    const RESET_VALUE: u8 = 0x1f;
}
