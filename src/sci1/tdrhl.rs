///Register `TDRHL` reader
pub type R = crate::R<TdrhlSpec>;
///Register `TDRHL` writer
pub type W = crate::W<TdrhlSpec>;
///Field `TDAT` reader - Serial Transmit Data
pub type TdatR = crate::FieldReader<u16>;
///Field `TDAT` writer - Serial Transmit Data
pub type TdatW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - Serial Transmit Data
    #[inline(always)]
    pub fn tdat(&self) -> TdatR {
        TdatR::new(self.bits & 0x01ff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TDRHL").field("tdat", &self.tdat()).finish()
    }
}
impl W {
    ///Bits 0:8 - Serial Transmit Data
    #[inline(always)]
    pub fn tdat(&mut self) -> TdatW<TdrhlSpec> {
        TdatW::new(self, 0)
    }
}
/**Transmit Data Register for Non-Manchester mode (MMR.MANEN = 0)

You can [`read`](crate::Reg::read) this register and get [`tdrhl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdrhl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TdrhlSpec;
impl crate::RegisterSpec for TdrhlSpec {
    type Ux = u16;
}
///`read()` method returns [`tdrhl::R`](R) reader structure
impl crate::Readable for TdrhlSpec {}
///`write(|w| ..)` method takes [`tdrhl::W`](W) writer structure
impl crate::Writable for TdrhlSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TDRHL to value 0xffff
impl crate::Resettable for TdrhlSpec {
    const RESET_VALUE: u16 = 0xffff;
}
