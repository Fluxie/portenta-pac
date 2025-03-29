///Register `ADWINLLB` reader
pub type R = crate::R<AdwinllbSpec>;
///Register `ADWINLLB` writer
pub type W = crate::W<AdwinllbSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**A/D Compare Function Window B Lower-Side/Upper-Side Level Setting Register

You can [`read`](crate::Reg::read) this register and get [`adwinllb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adwinllb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AdwinllbSpec;
impl crate::RegisterSpec for AdwinllbSpec {
    type Ux = u16;
}
///`read()` method returns [`adwinllb::R`](R) reader structure
impl crate::Readable for AdwinllbSpec {}
///`write(|w| ..)` method takes [`adwinllb::W`](W) writer structure
impl crate::Writable for AdwinllbSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADWINLLB to value 0
impl crate::Resettable for AdwinllbSpec {}
