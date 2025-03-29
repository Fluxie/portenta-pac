///Register `SELSR0` reader
pub type R = crate::R<Selsr0Spec>;
///Register `SELSR0` writer
pub type W = crate::W<Selsr0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**SYS Event Link Setting Register

You can [`read`](crate::Reg::read) this register and get [`selsr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`selsr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Selsr0Spec;
impl crate::RegisterSpec for Selsr0Spec {
    type Ux = u16;
}
///`read()` method returns [`selsr0::R`](R) reader structure
impl crate::Readable for Selsr0Spec {}
///`write(|w| ..)` method takes [`selsr0::W`](W) writer structure
impl crate::Writable for Selsr0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SELSR0 to value 0
impl crate::Resettable for Selsr0Spec {}
