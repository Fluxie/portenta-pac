///Register `TPRE` reader
pub type R = crate::R<TpreSpec>;
///Register `TPRE` writer
pub type W = crate::W<TpreSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**Timer Prescaler Register

You can [`read`](crate::Reg::read) this register and get [`tpre::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tpre::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TpreSpec;
impl crate::RegisterSpec for TpreSpec {
    type Ux = u8;
}
///`read()` method returns [`tpre::R`](R) reader structure
impl crate::Readable for TpreSpec {}
///`write(|w| ..)` method takes [`tpre::W`](W) writer structure
impl crate::Writable for TpreSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TPRE to value 0xff
impl crate::Resettable for TpreSpec {
    const RESET_VALUE: u8 = 0xff;
}
