///Register `CTSUSO0` reader
pub type R = crate::R<Ctsuso0Spec>;
///Register `CTSUSO0` writer
pub type W = crate::W<Ctsuso0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**CTSU Sensor Offset Register 0

You can [`read`](crate::Reg::read) this register and get [`ctsuso0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsuso0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Ctsuso0Spec;
impl crate::RegisterSpec for Ctsuso0Spec {
    type Ux = u16;
}
///`read()` method returns [`ctsuso0::R`](R) reader structure
impl crate::Readable for Ctsuso0Spec {}
///`write(|w| ..)` method takes [`ctsuso0::W`](W) writer structure
impl crate::Writable for Ctsuso0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CTSUSO0 to value 0
impl crate::Resettable for Ctsuso0Spec {}
