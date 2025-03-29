///Register `CTSUTRMR` reader
pub type R = crate::R<CtsutrmrSpec>;
///Register `CTSUTRMR` writer
pub type W = crate::W<CtsutrmrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**CTSU Reference Current Calibration Register

You can [`read`](crate::Reg::read) this register and get [`ctsutrmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsutrmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CtsutrmrSpec;
impl crate::RegisterSpec for CtsutrmrSpec {
    type Ux = u8;
}
///`read()` method returns [`ctsutrmr::R`](R) reader structure
impl crate::Readable for CtsutrmrSpec {}
///`write(|w| ..)` method takes [`ctsutrmr::W`](W) writer structure
impl crate::Writable for CtsutrmrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CTSUTRMR to value 0
impl crate::Resettable for CtsutrmrSpec {}
