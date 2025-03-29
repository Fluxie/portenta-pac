///Register `CRCDOR` reader
pub type R = crate::R<CrcdorSpec>;
///Register `CRCDOR` writer
pub type W = crate::W<CrcdorSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**CRC Data Output Register

You can [`read`](crate::Reg::read) this register and get [`crcdor::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcdor::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CrcdorSpec;
impl crate::RegisterSpec for CrcdorSpec {
    type Ux = u32;
}
///`read()` method returns [`crcdor::R`](R) reader structure
impl crate::Readable for CrcdorSpec {}
///`write(|w| ..)` method takes [`crcdor::W`](W) writer structure
impl crate::Writable for CrcdorSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CRCDOR to value 0
impl crate::Resettable for CrcdorSpec {}
