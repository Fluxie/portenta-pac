///Register `STATBH` reader
pub type R = crate::R<StatbhSpec>;
///Register `STATBH` writer
pub type W = crate::W<StatbhSpec>;
///Field `STATBH` reader - CEC Reception Start Bit Maximum Bit Width Setting
pub type StatbhR = crate::FieldReader<u16>;
///Field `STATBH` writer - CEC Reception Start Bit Maximum Bit Width Setting
pub type StatbhW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - CEC Reception Start Bit Maximum Bit Width Setting
    #[inline(always)]
    pub fn statbh(&self) -> StatbhR {
        StatbhR::new(self.bits & 0x01ff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATBH").field("statbh", &self.statbh()).finish()
    }
}
impl W {
    ///Bits 0:8 - CEC Reception Start Bit Maximum Bit Width Setting
    #[inline(always)]
    pub fn statbh(&mut self) -> StatbhW<StatbhSpec> {
        StatbhW::new(self, 0)
    }
}
/**CEC Reception Start Bit Maximum Bit Width Setting Register

You can [`read`](crate::Reg::read) this register and get [`statbh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`statbh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct StatbhSpec;
impl crate::RegisterSpec for StatbhSpec {
    type Ux = u16;
}
///`read()` method returns [`statbh::R`](R) reader structure
impl crate::Readable for StatbhSpec {}
///`write(|w| ..)` method takes [`statbh::W`](W) writer structure
impl crate::Writable for StatbhSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets STATBH to value 0
impl crate::Resettable for StatbhSpec {}
