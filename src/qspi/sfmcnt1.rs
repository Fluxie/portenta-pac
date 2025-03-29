///Register `SFMCNT1` reader
pub type R = crate::R<Sfmcnt1Spec>;
///Register `SFMCNT1` writer
pub type W = crate::W<Sfmcnt1Spec>;
///Field `QSPI_EXT` reader - Bank switching address
pub type QspiExtR = crate::FieldReader;
///Field `QSPI_EXT` writer - Bank switching address
pub type QspiExtW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bits 26:31 - Bank switching address
    #[inline(always)]
    pub fn qspi_ext(&self) -> QspiExtR {
        QspiExtR::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SFMCNT1").field("qspi_ext", &self.qspi_ext()).finish()
    }
}
impl W {
    ///Bits 26:31 - Bank switching address
    #[inline(always)]
    pub fn qspi_ext(&mut self) -> QspiExtW<Sfmcnt1Spec> {
        QspiExtW::new(self, 26)
    }
}
/**External QSPI Address Register

You can [`read`](crate::Reg::read) this register and get [`sfmcnt1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfmcnt1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Sfmcnt1Spec;
impl crate::RegisterSpec for Sfmcnt1Spec {
    type Ux = u32;
}
///`read()` method returns [`sfmcnt1::R`](R) reader structure
impl crate::Readable for Sfmcnt1Spec {}
///`write(|w| ..)` method takes [`sfmcnt1::W`](W) writer structure
impl crate::Writable for Sfmcnt1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SFMCNT1 to value 0
impl crate::Resettable for Sfmcnt1Spec {}
