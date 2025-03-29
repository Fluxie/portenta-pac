///Register `CTSUCHAC2` reader
pub type R = crate::R<Ctsuchac2Spec>;
///Register `CTSUCHAC2` writer
pub type W = crate::W<Ctsuchac2Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**CTSU Channel Enable Control Register 2

You can [`read`](crate::Reg::read) this register and get [`ctsuchac2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsuchac2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Ctsuchac2Spec;
impl crate::RegisterSpec for Ctsuchac2Spec {
    type Ux = u8;
}
///`read()` method returns [`ctsuchac2::R`](R) reader structure
impl crate::Readable for Ctsuchac2Spec {}
///`write(|w| ..)` method takes [`ctsuchac2::W`](W) writer structure
impl crate::Writable for Ctsuchac2Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CTSUCHAC2 to value 0
impl crate::Resettable for Ctsuchac2Spec {}
