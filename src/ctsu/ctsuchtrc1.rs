///Register `CTSUCHTRC1` reader
pub type R = crate::R<Ctsuchtrc1Spec>;
///Register `CTSUCHTRC1` writer
pub type W = crate::W<Ctsuchtrc1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**CTSU Channel Transmit/Receive Control Register 1

You can [`read`](crate::Reg::read) this register and get [`ctsuchtrc1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsuchtrc1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Ctsuchtrc1Spec;
impl crate::RegisterSpec for Ctsuchtrc1Spec {
    type Ux = u8;
}
///`read()` method returns [`ctsuchtrc1::R`](R) reader structure
impl crate::Readable for Ctsuchtrc1Spec {}
///`write(|w| ..)` method takes [`ctsuchtrc1::W`](W) writer structure
impl crate::Writable for Ctsuchtrc1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CTSUCHTRC1 to value 0
impl crate::Resettable for Ctsuchtrc1Spec {}
