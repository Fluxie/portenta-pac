///Register `CTSUCHAC1` reader
pub type R = crate::R<Ctsuchac1Spec>;
///Register `CTSUCHAC1` writer
pub type W = crate::W<Ctsuchac1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**CTSU Channel Enable Control Register 1

You can [`read`](crate::Reg::read) this register and get [`ctsuchac1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsuchac1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Ctsuchac1Spec;
impl crate::RegisterSpec for Ctsuchac1Spec {
    type Ux = u8;
}
///`read()` method returns [`ctsuchac1::R`](R) reader structure
impl crate::Readable for Ctsuchac1Spec {}
///`write(|w| ..)` method takes [`ctsuchac1::W`](W) writer structure
impl crate::Writable for Ctsuchac1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CTSUCHAC1 to value 0
impl crate::Resettable for Ctsuchac1Spec {}
