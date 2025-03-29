///Register `EC710EAD0` reader
pub type R = crate::R<Ec710ead0Spec>;
///Field `ECEAD` reader - ECC Error Address
pub type EceadR = crate::FieldReader<u16>;
impl R {
    ///Bits 0:10 - ECC Error Address
    #[inline(always)]
    pub fn ecead(&self) -> EceadR {
        EceadR::new((self.bits & 0x07ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EC710EAD0").field("ecead", &self.ecead()).finish()
    }
}
/**ECC Error Address Register

You can [`read`](crate::Reg::read) this register and get [`ec710ead0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Ec710ead0Spec;
impl crate::RegisterSpec for Ec710ead0Spec {
    type Ux = u32;
}
///`read()` method returns [`ec710ead0::R`](R) reader structure
impl crate::Readable for Ec710ead0Spec {}
///`reset()` method sets EC710EAD0 to value 0
impl crate::Resettable for Ec710ead0Spec {}
