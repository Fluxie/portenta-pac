///Register `CTSUSO1` reader
pub type R = crate::R<Ctsuso1Spec>;
///Register `CTSUSO1` writer
pub type W = crate::W<Ctsuso1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**CTSU Sensor Offset Register 1

You can [`read`](crate::Reg::read) this register and get [`ctsuso1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsuso1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Ctsuso1Spec;
impl crate::RegisterSpec for Ctsuso1Spec {
    type Ux = u16;
}
///`read()` method returns [`ctsuso1::R`](R) reader structure
impl crate::Readable for Ctsuso1Spec {}
///`write(|w| ..)` method takes [`ctsuso1::W`](W) writer structure
impl crate::Writable for Ctsuso1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CTSUSO1 to value 0
impl crate::Resettable for Ctsuso1Spec {}
