///Register `DODSR` reader
pub type R = crate::R<DodsrSpec>;
///Register `DODSR` writer
pub type W = crate::W<DodsrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**DOC Data Setting Register

You can [`read`](crate::Reg::read) this register and get [`dodsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dodsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DodsrSpec;
impl crate::RegisterSpec for DodsrSpec {
    type Ux = u16;
}
///`read()` method returns [`dodsr::R`](R) reader structure
impl crate::Readable for DodsrSpec {}
///`write(|w| ..)` method takes [`dodsr::W`](W) writer structure
impl crate::Writable for DodsrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DODSR to value 0
impl crate::Resettable for DodsrSpec {}
