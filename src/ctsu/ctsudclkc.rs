///Register `CTSUDCLKC` reader
pub type R = crate::R<CtsudclkcSpec>;
///Register `CTSUDCLKC` writer
pub type W = crate::W<CtsudclkcSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**CTSU High-Pass Noise Reduction Control Register

You can [`read`](crate::Reg::read) this register and get [`ctsudclkc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsudclkc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CtsudclkcSpec;
impl crate::RegisterSpec for CtsudclkcSpec {
    type Ux = u8;
}
///`read()` method returns [`ctsudclkc::R`](R) reader structure
impl crate::Readable for CtsudclkcSpec {}
///`write(|w| ..)` method takes [`ctsudclkc::W`](W) writer structure
impl crate::Writable for CtsudclkcSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CTSUDCLKC to value 0
impl crate::Resettable for CtsudclkcSpec {}
