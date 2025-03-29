///Register `ADCMPDR%s` reader
pub type R = crate::R<AdcmpdrSpec>;
///Register `ADCMPDR%s` writer
pub type W = crate::W<AdcmpdrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**A/D Compare Function Window A Lower-Side/Upper-Side Level Setting Register

You can [`read`](crate::Reg::read) this register and get [`adcmpdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AdcmpdrSpec;
impl crate::RegisterSpec for AdcmpdrSpec {
    type Ux = u16;
}
///`read()` method returns [`adcmpdr::R`](R) reader structure
impl crate::Readable for AdcmpdrSpec {}
///`write(|w| ..)` method takes [`adcmpdr::W`](W) writer structure
impl crate::Writable for AdcmpdrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADCMPDR%s to value 0
impl crate::Resettable for AdcmpdrSpec {}
