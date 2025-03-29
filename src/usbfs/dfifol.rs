///Register `D%sFIFOL` reader
pub type R = crate::R<DfifolSpec>;
///Register `D%sFIFOL` writer
pub type W = crate::W<DfifolSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**D%sFIFO Port Register

You can [`read`](crate::Reg::read) this register and get [`dfifol::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfifol::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DfifolSpec;
impl crate::RegisterSpec for DfifolSpec {
    type Ux = u8;
}
///`read()` method returns [`dfifol::R`](R) reader structure
impl crate::Readable for DfifolSpec {}
///`write(|w| ..)` method takes [`dfifol::W`](W) writer structure
impl crate::Writable for DfifolSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets D%sFIFOL to value 0
impl crate::Resettable for DfifolSpec {}
