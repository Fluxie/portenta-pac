///Register `RDAYCP%s` reader
pub type R = crate::R<RdaycpSpec>;
///Field `DATE1` reader - 1-Day Capture
pub type Date1R = crate::FieldReader;
///Field `DATE10` reader - 10-Day Capture
pub type Date10R = crate::FieldReader;
impl R {
    ///Bits 0:3 - 1-Day Capture
    #[inline(always)]
    pub fn date1(&self) -> Date1R {
        Date1R::new(self.bits & 0x0f)
    }
    ///Bits 4:5 - 10-Day Capture
    #[inline(always)]
    pub fn date10(&self) -> Date10R {
        Date10R::new((self.bits >> 4) & 3)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDAYCP")
            .field("date1", &self.date1())
            .field("date10", &self.date10())
            .finish()
    }
}
/**Date Capture Register %s

You can [`read`](crate::Reg::read) this register and get [`rdaycp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RdaycpSpec;
impl crate::RegisterSpec for RdaycpSpec {
    type Ux = u8;
}
///`read()` method returns [`rdaycp::R`](R) reader structure
impl crate::Readable for RdaycpSpec {}
///`reset()` method sets RDAYCP%s to value 0
impl crate::Resettable for RdaycpSpec {}
