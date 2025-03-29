///Register `ADSTRGR` reader
pub type R = crate::R<AdstrgrSpec>;
///Register `ADSTRGR` writer
pub type W = crate::W<AdstrgrSpec>;
///Field `TRSB` reader - A/D Conversion Start Trigger Select for Group B
pub type TrsbR = crate::FieldReader;
///Field `TRSB` writer - A/D Conversion Start Trigger Select for Group B
pub type TrsbW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `TRSA` reader - A/D Conversion Start Trigger Select
pub type TrsaR = crate::FieldReader;
///Field `TRSA` writer - A/D Conversion Start Trigger Select
pub type TrsaW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bits 0:5 - A/D Conversion Start Trigger Select for Group B
    #[inline(always)]
    pub fn trsb(&self) -> TrsbR {
        TrsbR::new((self.bits & 0x3f) as u8)
    }
    ///Bits 8:13 - A/D Conversion Start Trigger Select
    #[inline(always)]
    pub fn trsa(&self) -> TrsaR {
        TrsaR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADSTRGR")
            .field("trsb", &self.trsb())
            .field("trsa", &self.trsa())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - A/D Conversion Start Trigger Select for Group B
    #[inline(always)]
    pub fn trsb(&mut self) -> TrsbW<AdstrgrSpec> {
        TrsbW::new(self, 0)
    }
    ///Bits 8:13 - A/D Conversion Start Trigger Select
    #[inline(always)]
    pub fn trsa(&mut self) -> TrsaW<AdstrgrSpec> {
        TrsaW::new(self, 8)
    }
}
/**A/D Conversion Start Trigger Select Register

You can [`read`](crate::Reg::read) this register and get [`adstrgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adstrgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AdstrgrSpec;
impl crate::RegisterSpec for AdstrgrSpec {
    type Ux = u16;
}
///`read()` method returns [`adstrgr::R`](R) reader structure
impl crate::Readable for AdstrgrSpec {}
///`write(|w| ..)` method takes [`adstrgr::W`](W) writer structure
impl crate::Writable for AdstrgrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADSTRGR to value 0
impl crate::Resettable for AdstrgrSpec {}
