///Register `DADR%s` reader
pub type R = crate::R<DadrSpec>;
///Register `DADR%s` writer
pub type W = crate::W<DadrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**D/A Data Register %s

You can [`read`](crate::Reg::read) this register and get [`dadr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dadr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DadrSpec;
impl crate::RegisterSpec for DadrSpec {
    type Ux = u16;
}
///`read()` method returns [`dadr::R`](R) reader structure
impl crate::Readable for DadrSpec {}
///`write(|w| ..)` method takes [`dadr::W`](W) writer structure
impl crate::Writable for DadrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DADR%s to value 0
impl crate::Resettable for DadrSpec {}
