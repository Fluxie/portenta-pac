///Register `STATB` reader
pub type R = crate::R<StatbSpec>;
///Register `STATB` writer
pub type W = crate::W<StatbSpec>;
///Field `STATB` reader - CEC Transmission Start Bit Width Setting
pub type StatbR = crate::FieldReader<u16>;
///Field `STATB` writer - CEC Transmission Start Bit Width Setting
pub type StatbW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - CEC Transmission Start Bit Width Setting
    #[inline(always)]
    pub fn statb(&self) -> StatbR {
        StatbR::new(self.bits & 0x01ff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATB").field("statb", &self.statb()).finish()
    }
}
impl W {
    ///Bits 0:8 - CEC Transmission Start Bit Width Setting
    #[inline(always)]
    pub fn statb(&mut self) -> StatbW<StatbSpec> {
        StatbW::new(self, 0)
    }
}
/**CEC Transmission Start Bit Width Setting Register

You can [`read`](crate::Reg::read) this register and get [`statb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`statb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct StatbSpec;
impl crate::RegisterSpec for StatbSpec {
    type Ux = u16;
}
///`read()` method returns [`statb::R`](R) reader structure
impl crate::Readable for StatbSpec {}
///`write(|w| ..)` method takes [`statb::W`](W) writer structure
impl crate::Writable for StatbSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets STATB to value 0
impl crate::Resettable for StatbSpec {}
