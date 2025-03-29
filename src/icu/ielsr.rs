///Register `IELSR%s` reader
pub type R = crate::R<IelsrSpec>;
///Register `IELSR%s` writer
pub type W = crate::W<IelsrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**ICU Event Link Setting Register %s

You can [`read`](crate::Reg::read) this register and get [`ielsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ielsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IelsrSpec;
impl crate::RegisterSpec for IelsrSpec {
    type Ux = u32;
}
///`read()` method returns [`ielsr::R`](R) reader structure
impl crate::Readable for IelsrSpec {}
///`write(|w| ..)` method takes [`ielsr::W`](W) writer structure
impl crate::Writable for IelsrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IELSR%s to value 0
impl crate::Resettable for IelsrSpec {}
