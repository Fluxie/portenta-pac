///Register `TDLAR` reader
pub type R = crate::R<TdlarSpec>;
///Register `TDLAR` writer
pub type W = crate::W<TdlarSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**Transmit Descriptor List Start Address Register

You can [`read`](crate::Reg::read) this register and get [`tdlar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdlar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TdlarSpec;
impl crate::RegisterSpec for TdlarSpec {
    type Ux = u32;
}
///`read()` method returns [`tdlar::R`](R) reader structure
impl crate::Readable for TdlarSpec {}
///`write(|w| ..)` method takes [`tdlar::W`](W) writer structure
impl crate::Writable for TdlarSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TDLAR to value 0
impl crate::Resettable for TdlarSpec {}
