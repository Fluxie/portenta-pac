///Register `AGT` reader
pub type R = crate::R<AgtSpec>;
///Register `AGT` writer
pub type W = crate::W<AgtSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**AGT Counter Register

You can [`read`](crate::Reg::read) this register and get [`agt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AgtSpec;
impl crate::RegisterSpec for AgtSpec {
    type Ux = u16;
}
///`read()` method returns [`agt::R`](R) reader structure
impl crate::Readable for AgtSpec {}
///`write(|w| ..)` method takes [`agt::W`](W) writer structure
impl crate::Writable for AgtSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AGT to value 0xffff
impl crate::Resettable for AgtSpec {
    const RESET_VALUE: u16 = 0xffff;
}
