///Register `TDR` reader
pub type R = crate::R<TdrSpec>;
///Register `TDR` writer
pub type W = crate::W<TdrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**Transmit Data Register

You can [`read`](crate::Reg::read) this register and get [`tdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TdrSpec;
impl crate::RegisterSpec for TdrSpec {
    type Ux = u8;
}
///`read()` method returns [`tdr::R`](R) reader structure
impl crate::Readable for TdrSpec {}
///`write(|w| ..)` method takes [`tdr::W`](W) writer structure
impl crate::Writable for TdrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TDR to value 0xff
impl crate::Resettable for TdrSpec {
    const RESET_VALUE: u8 = 0xff;
}
