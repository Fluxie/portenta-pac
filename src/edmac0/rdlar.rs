///Register `RDLAR` reader
pub type R = crate::R<RdlarSpec>;
///Register `RDLAR` writer
pub type W = crate::W<RdlarSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**Receive Descriptor List Start Address Register

You can [`read`](crate::Reg::read) this register and get [`rdlar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rdlar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RdlarSpec;
impl crate::RegisterSpec for RdlarSpec {
    type Ux = u32;
}
///`read()` method returns [`rdlar::R`](R) reader structure
impl crate::Readable for RdlarSpec {}
///`write(|w| ..)` method takes [`rdlar::W`](W) writer structure
impl crate::Writable for RdlarSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RDLAR to value 0
impl crate::Resettable for RdlarSpec {}
