///Register `CTSUSST` reader
pub type R = crate::R<CtsusstSpec>;
///Register `CTSUSST` writer
pub type W = crate::W<CtsusstSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**CTSU Sensor Stabilization Wait Control Register

You can [`read`](crate::Reg::read) this register and get [`ctsusst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsusst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CtsusstSpec;
impl crate::RegisterSpec for CtsusstSpec {
    type Ux = u8;
}
///`read()` method returns [`ctsusst::R`](R) reader structure
impl crate::Readable for CtsusstSpec {}
///`write(|w| ..)` method takes [`ctsusst::W`](W) writer structure
impl crate::Writable for CtsusstSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CTSUSST to value 0
impl crate::Resettable for CtsusstSpec {}
