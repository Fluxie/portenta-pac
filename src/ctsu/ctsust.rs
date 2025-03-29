///Register `CTSUST` reader
pub type R = crate::R<CtsustSpec>;
///Register `CTSUST` writer
pub type W = crate::W<CtsustSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**CTSU Status Register

You can [`read`](crate::Reg::read) this register and get [`ctsust::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsust::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CtsustSpec;
impl crate::RegisterSpec for CtsustSpec {
    type Ux = u8;
}
///`read()` method returns [`ctsust::R`](R) reader structure
impl crate::Readable for CtsustSpec {}
///`write(|w| ..)` method takes [`ctsust::W`](W) writer structure
impl crate::Writable for CtsustSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CTSUST to value 0
impl crate::Resettable for CtsustSpec {}
