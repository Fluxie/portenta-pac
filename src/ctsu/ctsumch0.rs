///Register `CTSUMCH0` reader
pub type R = crate::R<Ctsumch0Spec>;
///Register `CTSUMCH0` writer
pub type W = crate::W<Ctsumch0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**CTSU Measurement Channel Register 0

You can [`read`](crate::Reg::read) this register and get [`ctsumch0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsumch0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Ctsumch0Spec;
impl crate::RegisterSpec for Ctsumch0Spec {
    type Ux = u8;
}
///`read()` method returns [`ctsumch0::R`](R) reader structure
impl crate::Readable for Ctsumch0Spec {}
///`write(|w| ..)` method takes [`ctsumch0::W`](W) writer structure
impl crate::Writable for Ctsumch0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CTSUMCH0 to value 0x1f
impl crate::Resettable for Ctsumch0Spec {
    const RESET_VALUE: u8 = 0x1f;
}
