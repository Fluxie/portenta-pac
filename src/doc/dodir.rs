///Register `DODIR` reader
pub type R = crate::R<DodirSpec>;
///Register `DODIR` writer
pub type W = crate::W<DodirSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**DOC Data Input Register

You can [`read`](crate::Reg::read) this register and get [`dodir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dodir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DodirSpec;
impl crate::RegisterSpec for DodirSpec {
    type Ux = u16;
}
///`read()` method returns [`dodir::R`](R) reader structure
impl crate::Readable for DodirSpec {}
///`write(|w| ..)` method takes [`dodir::W`](W) writer structure
impl crate::Writable for DodirSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DODIR to value 0
impl crate::Resettable for DodirSpec {}
