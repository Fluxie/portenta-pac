///Register `CRCDOR_HA` reader
pub type R = crate::R<CrcdorHaSpec>;
///Register `CRCDOR_HA` writer
pub type W = crate::W<CrcdorHaSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**CRC Data Output Register

You can [`read`](crate::Reg::read) this register and get [`crcdor_ha::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcdor_ha::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CrcdorHaSpec;
impl crate::RegisterSpec for CrcdorHaSpec {
    type Ux = u16;
}
///`read()` method returns [`crcdor_ha::R`](R) reader structure
impl crate::Readable for CrcdorHaSpec {}
///`write(|w| ..)` method takes [`crcdor_ha::W`](W) writer structure
impl crate::Writable for CrcdorHaSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CRCDOR_HA to value 0
impl crate::Resettable for CrcdorHaSpec {}
