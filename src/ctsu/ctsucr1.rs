///Register `CTSUCR1` reader
pub type R = crate::R<Ctsucr1Spec>;
///Register `CTSUCR1` writer
pub type W = crate::W<Ctsucr1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**CTSU Control Register 1

You can [`read`](crate::Reg::read) this register and get [`ctsucr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsucr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Ctsucr1Spec;
impl crate::RegisterSpec for Ctsucr1Spec {
    type Ux = u8;
}
///`read()` method returns [`ctsucr1::R`](R) reader structure
impl crate::Readable for Ctsucr1Spec {}
///`write(|w| ..)` method takes [`ctsucr1::W`](W) writer structure
impl crate::Writable for Ctsucr1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CTSUCR1 to value 0
impl crate::Resettable for Ctsucr1Spec {}
