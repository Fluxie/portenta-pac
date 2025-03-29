///Register `FCMDR` reader
pub type R = crate::R<FcmdrSpec>;
///Field `PCMDR` reader - Pre-command Flag
pub type PcmdrR = crate::FieldReader;
///Field `CMDR` reader - Command Flag
pub type CmdrR = crate::FieldReader;
impl R {
    ///Bits 0:7 - Pre-command Flag
    #[inline(always)]
    pub fn pcmdr(&self) -> PcmdrR {
        PcmdrR::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Command Flag
    #[inline(always)]
    pub fn cmdr(&self) -> CmdrR {
        CmdrR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCMDR")
            .field("pcmdr", &self.pcmdr())
            .field("cmdr", &self.cmdr())
            .finish()
    }
}
/**FACI Command Register

You can [`read`](crate::Reg::read) this register and get [`fcmdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FcmdrSpec;
impl crate::RegisterSpec for FcmdrSpec {
    type Ux = u16;
}
///`read()` method returns [`fcmdr::R`](R) reader structure
impl crate::Readable for FcmdrSpec {}
///`reset()` method sets FCMDR to value 0
impl crate::Resettable for FcmdrSpec {}
