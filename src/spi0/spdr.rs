///Register `SPDR` reader
pub type R = crate::R<SpdrSpec>;
///Register `SPDR` writer
pub type W = crate::W<SpdrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**SPI Data Register

You can [`read`](crate::Reg::read) this register and get [`spdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SpdrSpec;
impl crate::RegisterSpec for SpdrSpec {
    type Ux = u32;
}
///`read()` method returns [`spdr::R`](R) reader structure
impl crate::Readable for SpdrSpec {}
///`write(|w| ..)` method takes [`spdr::W`](W) writer structure
impl crate::Writable for SpdrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SPDR to value 0
impl crate::Resettable for SpdrSpec {}
