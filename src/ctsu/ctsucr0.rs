///Register `CTSUCR0` reader
pub type R = crate::R<Ctsucr0Spec>;
///Register `CTSUCR0` writer
pub type W = crate::W<Ctsucr0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**CTSU Control Register 0

You can [`read`](crate::Reg::read) this register and get [`ctsucr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsucr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Ctsucr0Spec;
impl crate::RegisterSpec for Ctsucr0Spec {
    type Ux = u8;
}
///`read()` method returns [`ctsucr0::R`](R) reader structure
impl crate::Readable for Ctsucr0Spec {}
///`write(|w| ..)` method takes [`ctsucr0::W`](W) writer structure
impl crate::Writable for Ctsucr0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CTSUCR0 to value 0
impl crate::Resettable for Ctsucr0Spec {}
