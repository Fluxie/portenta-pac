///Register `SPBR` reader
pub type R = crate::R<SpbrSpec>;
///Register `SPBR` writer
pub type W = crate::W<SpbrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**SPI Bit Rate Register

You can [`read`](crate::Reg::read) this register and get [`spbr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spbr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SpbrSpec;
impl crate::RegisterSpec for SpbrSpec {
    type Ux = u8;
}
///`read()` method returns [`spbr::R`](R) reader structure
impl crate::Readable for SpbrSpec {}
///`write(|w| ..)` method takes [`spbr::W`](W) writer structure
impl crate::Writable for SpbrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SPBR to value 0xff
impl crate::Resettable for SpbrSpec {
    const RESET_VALUE: u8 = 0xff;
}
