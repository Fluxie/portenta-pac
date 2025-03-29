///Register `EDTRR` reader
pub type R = crate::R<EdtrrSpec>;
///Register `EDTRR` writer
pub type W = crate::W<EdtrrSpec>;
///Field `TR` reader - Transmit Request
pub type TrR = crate::BitReader;
///Field `TR` writer - Transmit Request
pub type TrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Transmit Request
    #[inline(always)]
    pub fn tr(&self) -> TrR {
        TrR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EDTRR").field("tr", &self.tr()).finish()
    }
}
impl W {
    ///Bit 0 - Transmit Request
    #[inline(always)]
    pub fn tr(&mut self) -> TrW<EdtrrSpec> {
        TrW::new(self, 0)
    }
}
/**EDMAC Transmit Request Register

You can [`read`](crate::Reg::read) this register and get [`edtrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`edtrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct EdtrrSpec;
impl crate::RegisterSpec for EdtrrSpec {
    type Ux = u32;
}
///`read()` method returns [`edtrr::R`](R) reader structure
impl crate::Readable for EdtrrSpec {}
///`write(|w| ..)` method takes [`edtrr::W`](W) writer structure
impl crate::Writable for EdtrrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EDTRR to value 0
impl crate::Resettable for EdtrrSpec {}
