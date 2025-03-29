///Register `SPDR_HA` reader
pub type R = crate::R<SpdrHaSpec>;
///Register `SPDR_HA` writer
pub type W = crate::W<SpdrHaSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**SPI Data Register

You can [`read`](crate::Reg::read) this register and get [`spdr_ha::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spdr_ha::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SpdrHaSpec;
impl crate::RegisterSpec for SpdrHaSpec {
    type Ux = u16;
}
///`read()` method returns [`spdr_ha::R`](R) reader structure
impl crate::Readable for SpdrHaSpec {}
///`write(|w| ..)` method takes [`spdr_ha::W`](W) writer structure
impl crate::Writable for SpdrHaSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SPDR_HA to value 0
impl crate::Resettable for SpdrHaSpec {}
