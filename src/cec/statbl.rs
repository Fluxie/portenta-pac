///Register `STATBL` reader
pub type R = crate::R<StatblSpec>;
///Register `STATBL` writer
pub type W = crate::W<StatblSpec>;
///Field `STATBL` reader - CEC Reception Start Bit Minimum Bit Width Setting
pub type StatblR = crate::FieldReader<u16>;
///Field `STATBL` writer - CEC Reception Start Bit Minimum Bit Width Setting
pub type StatblW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - CEC Reception Start Bit Minimum Bit Width Setting
    #[inline(always)]
    pub fn statbl(&self) -> StatblR {
        StatblR::new(self.bits & 0x01ff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATBL").field("statbl", &self.statbl()).finish()
    }
}
impl W {
    ///Bits 0:8 - CEC Reception Start Bit Minimum Bit Width Setting
    #[inline(always)]
    pub fn statbl(&mut self) -> StatblW<StatblSpec> {
        StatblW::new(self, 0)
    }
}
/**CEC Reception Start Bit Minimum Bit Width Setting Register

You can [`read`](crate::Reg::read) this register and get [`statbl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`statbl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct StatblSpec;
impl crate::RegisterSpec for StatblSpec {
    type Ux = u16;
}
///`read()` method returns [`statbl::R`](R) reader structure
impl crate::Readable for StatblSpec {}
///`write(|w| ..)` method takes [`statbl::W`](W) writer structure
impl crate::Writable for StatblSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets STATBL to value 0
impl crate::Resettable for StatblSpec {}
