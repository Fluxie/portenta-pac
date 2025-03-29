///Register `SPDR_BY` reader
pub type R = crate::R<SpdrBySpec>;
///Register `SPDR_BY` writer
pub type W = crate::W<SpdrBySpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**SPI Data Register

You can [`read`](crate::Reg::read) this register and get [`spdr_by::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spdr_by::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SpdrBySpec;
impl crate::RegisterSpec for SpdrBySpec {
    type Ux = u8;
}
///`read()` method returns [`spdr_by::R`](R) reader structure
impl crate::Readable for SpdrBySpec {}
///`write(|w| ..)` method takes [`spdr_by::W`](W) writer structure
impl crate::Writable for SpdrBySpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SPDR_BY to value 0
impl crate::Resettable for SpdrBySpec {}
