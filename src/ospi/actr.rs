///Register `ACTR` reader
pub type R = crate::R<ActrSpec>;
///Register `ACTR` writer
pub type W = crate::W<ActrSpec>;
///Field `CTP` reader - Automatic calibration cycle time setting
pub type CtpR = crate::FieldReader<u32>;
///Field `CTP` writer - Automatic calibration cycle time setting
pub type CtpW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Automatic calibration cycle time setting
    #[inline(always)]
    pub fn ctp(&self) -> CtpR {
        CtpR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ACTR").field("ctp", &self.ctp()).finish()
    }
}
impl W {
    ///Bits 0:31 - Automatic calibration cycle time setting
    #[inline(always)]
    pub fn ctp(&mut self) -> CtpW<ActrSpec> {
        CtpW::new(self, 0)
    }
}
/**Auto-Calibration Timer Register

You can [`read`](crate::Reg::read) this register and get [`actr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`actr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ActrSpec;
impl crate::RegisterSpec for ActrSpec {
    type Ux = u32;
}
///`read()` method returns [`actr::R`](R) reader structure
impl crate::Readable for ActrSpec {}
///`write(|w| ..)` method takes [`actr::W`](W) writer structure
impl crate::Writable for ActrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ACTR to value 0x1000_0000
impl crate::Resettable for ActrSpec {
    const RESET_VALUE: u32 = 0x1000_0000;
}
