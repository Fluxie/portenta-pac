///Register `FCFTR` reader
pub type R = crate::R<FcftrSpec>;
///Register `FCFTR` writer
pub type W = crate::W<FcftrSpec>;
///Field `RFDO` reader - Receive FIFO Data PAUSE Output Threshold
pub type RfdoR = crate::FieldReader;
///Field `RFDO` writer - Receive FIFO Data PAUSE Output Threshold
pub type RfdoW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `RFFO` reader - Receive FIFO Frame PAUSE Output Threshold
pub type RffoR = crate::FieldReader;
///Field `RFFO` writer - Receive FIFO Frame PAUSE Output Threshold
pub type RffoW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - Receive FIFO Data PAUSE Output Threshold
    #[inline(always)]
    pub fn rfdo(&self) -> RfdoR {
        RfdoR::new((self.bits & 7) as u8)
    }
    ///Bits 16:18 - Receive FIFO Frame PAUSE Output Threshold
    #[inline(always)]
    pub fn rffo(&self) -> RffoR {
        RffoR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCFTR")
            .field("rfdo", &self.rfdo())
            .field("rffo", &self.rffo())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Receive FIFO Data PAUSE Output Threshold
    #[inline(always)]
    pub fn rfdo(&mut self) -> RfdoW<FcftrSpec> {
        RfdoW::new(self, 0)
    }
    ///Bits 16:18 - Receive FIFO Frame PAUSE Output Threshold
    #[inline(always)]
    pub fn rffo(&mut self) -> RffoW<FcftrSpec> {
        RffoW::new(self, 16)
    }
}
/**Flow Control Start FIFO Threshold Setting Register

You can [`read`](crate::Reg::read) this register and get [`fcftr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcftr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FcftrSpec;
impl crate::RegisterSpec for FcftrSpec {
    type Ux = u32;
}
///`read()` method returns [`fcftr::R`](R) reader structure
impl crate::Readable for FcftrSpec {}
///`write(|w| ..)` method takes [`fcftr::W`](W) writer structure
impl crate::Writable for FcftrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FCFTR to value 0x0007_0007
impl crate::Resettable for FcftrSpec {
    const RESET_VALUE: u32 = 0x0007_0007;
}
