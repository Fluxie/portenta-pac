///Register `CFDRMPTR%s_1` reader
pub type R = crate::R<Cfdrmptr1Spec>;
///Field `RMTS` reader - RX Message Buffer Timestamp Field
pub type RmtsR = crate::FieldReader<u16>;
///Field `RMDLC` reader - RX Message Buffer DLC Field
pub type RmdlcR = crate::FieldReader;
impl R {
    ///Bits 0:15 - RX Message Buffer Timestamp Field
    #[inline(always)]
    pub fn rmts(&self) -> RmtsR {
        RmtsR::new((self.bits & 0xffff) as u16)
    }
    ///Bits 28:31 - RX Message Buffer DLC Field
    #[inline(always)]
    pub fn rmdlc(&self) -> RmdlcR {
        RmdlcR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDRMPTR_1")
            .field("rmts", &self.rmts())
            .field("rmdlc", &self.rmdlc())
            .finish()
    }
}
/**RX Message Buffer Pointer Register %s Channel 1

You can [`read`](crate::Reg::read) this register and get [`cfdrmptr_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Cfdrmptr1Spec;
impl crate::RegisterSpec for Cfdrmptr1Spec {
    type Ux = u32;
}
///`read()` method returns [`cfdrmptr_1::R`](R) reader structure
impl crate::Readable for Cfdrmptr1Spec {}
///`reset()` method sets CFDRMPTR%s_1 to value 0
impl crate::Resettable for Cfdrmptr1Spec {}
