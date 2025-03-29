///Register `STATLL` reader
pub type R = crate::R<StatllSpec>;
///Register `STATLL` writer
pub type W = crate::W<StatllSpec>;
///Field `STATLL` reader - CEC Reception Start Bit Minimum Low Width Setting
pub type StatllR = crate::FieldReader<u16>;
///Field `STATLL` writer - CEC Reception Start Bit Minimum Low Width Setting
pub type StatllW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - CEC Reception Start Bit Minimum Low Width Setting
    #[inline(always)]
    pub fn statll(&self) -> StatllR {
        StatllR::new(self.bits & 0x01ff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATLL").field("statll", &self.statll()).finish()
    }
}
impl W {
    ///Bits 0:8 - CEC Reception Start Bit Minimum Low Width Setting
    #[inline(always)]
    pub fn statll(&mut self) -> StatllW<StatllSpec> {
        StatllW::new(self, 0)
    }
}
/**CEC Reception Start Bit Minimum Low Width Setting Register

You can [`read`](crate::Reg::read) this register and get [`statll::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`statll::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct StatllSpec;
impl crate::RegisterSpec for StatllSpec {
    type Ux = u16;
}
///`read()` method returns [`statll::R`](R) reader structure
impl crate::Readable for StatllSpec {}
///`write(|w| ..)` method takes [`statll::W`](W) writer structure
impl crate::Writable for StatllSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets STATLL to value 0
impl crate::Resettable for StatllSpec {}
