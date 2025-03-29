///Register `RSECCP%s` reader
pub type R = crate::R<RseccpSpec>;
///Field `SEC1` reader - 1-Second Capture
pub type Sec1R = crate::FieldReader;
///Field `SEC10` reader - 10-Second Capture
pub type Sec10R = crate::FieldReader;
impl R {
    ///Bits 0:3 - 1-Second Capture
    #[inline(always)]
    pub fn sec1(&self) -> Sec1R {
        Sec1R::new(self.bits & 0x0f)
    }
    ///Bits 4:6 - 10-Second Capture
    #[inline(always)]
    pub fn sec10(&self) -> Sec10R {
        Sec10R::new((self.bits >> 4) & 7)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSECCP")
            .field("sec1", &self.sec1())
            .field("sec10", &self.sec10())
            .finish()
    }
}
/**Second Capture Register %s

You can [`read`](crate::Reg::read) this register and get [`rseccp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RseccpSpec;
impl crate::RegisterSpec for RseccpSpec {
    type Ux = u8;
}
///`read()` method returns [`rseccp::R`](R) reader structure
impl crate::Readable for RseccpSpec {}
///`reset()` method sets RSECCP%s to value 0
impl crate::Resettable for RseccpSpec {}
