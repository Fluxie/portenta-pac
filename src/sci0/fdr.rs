///Register `FDR` reader
pub type R = crate::R<FdrSpec>;
///Field `R` reader - Receive FIFO Data Count
pub type RR = crate::FieldReader;
///Field `T` reader - Transmit FIFO Data Count
pub type TR = crate::FieldReader;
impl R {
    ///Bits 0:4 - Receive FIFO Data Count
    #[inline(always)]
    pub fn r(&self) -> RR {
        RR::new((self.bits & 0x1f) as u8)
    }
    ///Bits 8:12 - Transmit FIFO Data Count
    #[inline(always)]
    pub fn t(&self) -> TR {
        TR::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDR").field("r", &self.r()).field("t", &self.t()).finish()
    }
}
/**FIFO Data Count Register

You can [`read`](crate::Reg::read) this register and get [`fdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FdrSpec;
impl crate::RegisterSpec for FdrSpec {
    type Ux = u16;
}
///`read()` method returns [`fdr::R`](R) reader structure
impl crate::Readable for FdrSpec {}
///`reset()` method sets FDR to value 0
impl crate::Resettable for FdrSpec {}
