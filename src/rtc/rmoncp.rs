///Register `RMONCP%s` reader
pub type R = crate::R<RmoncpSpec>;
///Field `MON1` reader - 1-Month Capture
pub type Mon1R = crate::FieldReader;
///Field `MON10` reader - 10-Month Capture
pub type Mon10R = crate::BitReader;
impl R {
    ///Bits 0:3 - 1-Month Capture
    #[inline(always)]
    pub fn mon1(&self) -> Mon1R {
        Mon1R::new(self.bits & 0x0f)
    }
    ///Bit 4 - 10-Month Capture
    #[inline(always)]
    pub fn mon10(&self) -> Mon10R {
        Mon10R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RMONCP")
            .field("mon1", &self.mon1())
            .field("mon10", &self.mon10())
            .finish()
    }
}
/**Month Capture Register %s

You can [`read`](crate::Reg::read) this register and get [`rmoncp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RmoncpSpec;
impl crate::RegisterSpec for RmoncpSpec {
    type Ux = u8;
}
///`read()` method returns [`rmoncp::R`](R) reader structure
impl crate::Readable for RmoncpSpec {}
///`reset()` method sets RMONCP%s to value 0
impl crate::Resettable for RmoncpSpec {}
