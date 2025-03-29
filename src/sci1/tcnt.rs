///Register `TCNT` reader
pub type R = crate::R<TcntSpec>;
///Register `TCNT` writer
pub type W = crate::W<TcntSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**Timer Count Register

You can [`read`](crate::Reg::read) this register and get [`tcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TcntSpec;
impl crate::RegisterSpec for TcntSpec {
    type Ux = u8;
}
///`read()` method returns [`tcnt::R`](R) reader structure
impl crate::Readable for TcntSpec {}
///`write(|w| ..)` method takes [`tcnt::W`](W) writer structure
impl crate::Writable for TcntSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TCNT to value 0xff
impl crate::Resettable for TcntSpec {
    const RESET_VALUE: u8 = 0xff;
}
