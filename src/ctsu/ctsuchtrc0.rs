///Register `CTSUCHTRC0` reader
pub type R = crate::R<Ctsuchtrc0Spec>;
///Register `CTSUCHTRC0` writer
pub type W = crate::W<Ctsuchtrc0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**CTSU Channel Transmit/Receive Control Register 0

You can [`read`](crate::Reg::read) this register and get [`ctsuchtrc0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsuchtrc0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Ctsuchtrc0Spec;
impl crate::RegisterSpec for Ctsuchtrc0Spec {
    type Ux = u8;
}
///`read()` method returns [`ctsuchtrc0::R`](R) reader structure
impl crate::Readable for Ctsuchtrc0Spec {}
///`write(|w| ..)` method takes [`ctsuchtrc0::W`](W) writer structure
impl crate::Writable for Ctsuchtrc0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CTSUCHTRC0 to value 0
impl crate::Resettable for Ctsuchtrc0Spec {}
