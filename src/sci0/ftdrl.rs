///Register `FTDRL` writer
pub type W = crate::W<FtdrlSpec>;
///Field `TDAT` writer - Serial transmit data
pub type TdatW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl core::fmt::Debug for crate::generic::Reg<FtdrlSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:7 - Serial transmit data
    #[inline(always)]
    pub fn tdat(&mut self) -> TdatW<FtdrlSpec> {
        TdatW::new(self, 0)
    }
}
/**Transmit FIFO Data Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftdrl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FtdrlSpec;
impl crate::RegisterSpec for FtdrlSpec {
    type Ux = u8;
}
///`write(|w| ..)` method takes [`ftdrl::W`](W) writer structure
impl crate::Writable for FtdrlSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FTDRL to value 0xff
impl crate::Resettable for FtdrlSpec {
    const RESET_VALUE: u8 = 0xff;
}
