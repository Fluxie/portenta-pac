///Register `FRDRL` reader
pub type R = crate::R<FrdrlSpec>;
///Field `RDAT` reader - Serial receive data
pub type RdatR = crate::FieldReader;
impl R {
    ///Bits 0:7 - Serial receive data
    #[inline(always)]
    pub fn rdat(&self) -> RdatR {
        RdatR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FRDRL").field("rdat", &self.rdat()).finish()
    }
}
/**Receive FIFO Data Register

You can [`read`](crate::Reg::read) this register and get [`frdrl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FrdrlSpec;
impl crate::RegisterSpec for FrdrlSpec {
    type Ux = u8;
}
///`read()` method returns [`frdrl::R`](R) reader structure
impl crate::Readable for FrdrlSpec {}
///`reset()` method sets FRDRL to value 0
impl crate::Resettable for FrdrlSpec {}
