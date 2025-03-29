///Register `TPAUSER` reader
pub type R = crate::R<TpauserSpec>;
///Register `TPAUSER` writer
pub type W = crate::W<TpauserSpec>;
///Field `TPAUSE` reader -
pub type TpauseR = crate::FieldReader<u16>;
///Field `TPAUSE` writer -
pub type TpauseW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15
    #[inline(always)]
    pub fn tpause(&self) -> TpauseR {
        TpauseR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TPAUSER").field("tpause", &self.tpause()).finish()
    }
}
impl W {
    ///Bits 0:15
    #[inline(always)]
    pub fn tpause(&mut self) -> TpauseW<TpauserSpec> {
        TpauseW::new(self, 0)
    }
}
/**PAUSE Frame Retransmit Count Setting Register

You can [`read`](crate::Reg::read) this register and get [`tpauser::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tpauser::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TpauserSpec;
impl crate::RegisterSpec for TpauserSpec {
    type Ux = u32;
}
///`read()` method returns [`tpauser::R`](R) reader structure
impl crate::Readable for TpauserSpec {}
///`write(|w| ..)` method takes [`tpauser::W`](W) writer structure
impl crate::Writable for TpauserSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TPAUSER to value 0
impl crate::Resettable for TpauserSpec {}
