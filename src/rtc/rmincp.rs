///Register `RMINCP%s` reader
pub type R = crate::R<RmincpSpec>;
///Field `MIN1` reader - 1-Minute Capture
pub type Min1R = crate::FieldReader;
///Field `MIN10` reader - 10-Minute Capture
pub type Min10R = crate::FieldReader;
impl R {
    ///Bits 0:3 - 1-Minute Capture
    #[inline(always)]
    pub fn min1(&self) -> Min1R {
        Min1R::new(self.bits & 0x0f)
    }
    ///Bits 4:6 - 10-Minute Capture
    #[inline(always)]
    pub fn min10(&self) -> Min10R {
        Min10R::new((self.bits >> 4) & 7)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RMINCP")
            .field("min1", &self.min1())
            .field("min10", &self.min10())
            .finish()
    }
}
/**Minute Capture Register %s

You can [`read`](crate::Reg::read) this register and get [`rmincp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RmincpSpec;
impl crate::RegisterSpec for RmincpSpec {
    type Ux = u8;
}
///`read()` method returns [`rmincp::R`](R) reader structure
impl crate::Readable for RmincpSpec {}
///`reset()` method sets RMINCP%s to value 0
impl crate::Resettable for RmincpSpec {}
