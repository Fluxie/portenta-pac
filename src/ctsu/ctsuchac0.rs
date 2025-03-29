///Register `CTSUCHAC0` reader
pub type R = crate::R<Ctsuchac0Spec>;
///Register `CTSUCHAC0` writer
pub type W = crate::W<Ctsuchac0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**CTSU Channel Enable Control Register 0

You can [`read`](crate::Reg::read) this register and get [`ctsuchac0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsuchac0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Ctsuchac0Spec;
impl crate::RegisterSpec for Ctsuchac0Spec {
    type Ux = u8;
}
///`read()` method returns [`ctsuchac0::R`](R) reader structure
impl crate::Readable for Ctsuchac0Spec {}
///`write(|w| ..)` method takes [`ctsuchac0::W`](W) writer structure
impl crate::Writable for Ctsuchac0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CTSUCHAC0 to value 0
impl crate::Resettable for Ctsuchac0Spec {}
