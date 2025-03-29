///Register `STATL` reader
pub type R = crate::R<StatlSpec>;
///Register `STATL` writer
pub type W = crate::W<StatlSpec>;
///Field `STATL` reader - CEC Transmission Start Bit Low Width Setting
pub type StatlR = crate::FieldReader<u16>;
///Field `STATL` writer - CEC Transmission Start Bit Low Width Setting
pub type StatlW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - CEC Transmission Start Bit Low Width Setting
    #[inline(always)]
    pub fn statl(&self) -> StatlR {
        StatlR::new(self.bits & 0x01ff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATL").field("statl", &self.statl()).finish()
    }
}
impl W {
    ///Bits 0:8 - CEC Transmission Start Bit Low Width Setting
    #[inline(always)]
    pub fn statl(&mut self) -> StatlW<StatlSpec> {
        StatlW::new(self, 0)
    }
}
/**CEC Transmission Start Bit Low Width Setting Register

You can [`read`](crate::Reg::read) this register and get [`statl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`statl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct StatlSpec;
impl crate::RegisterSpec for StatlSpec {
    type Ux = u16;
}
///`read()` method returns [`statl::R`](R) reader structure
impl crate::Readable for StatlSpec {}
///`write(|w| ..)` method takes [`statl::W`](W) writer structure
impl crate::Writable for StatlSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets STATL to value 0
impl crate::Resettable for StatlSpec {}
