///Register `CEFCR` reader
pub type R = crate::R<CefcrSpec>;
///Register `CEFCR` writer
pub type W = crate::W<CefcrSpec>;
///Field `CEFCR` reader - CRC Error Frame Receive Counter
pub type CefcrR = crate::FieldReader<u32>;
///Field `CEFCR` writer - CRC Error Frame Receive Counter
pub type CefcrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CRC Error Frame Receive Counter
    #[inline(always)]
    pub fn cefcr(&self) -> CefcrR {
        CefcrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CEFCR").field("cefcr", &self.cefcr()).finish()
    }
}
impl W {
    ///Bits 0:31 - CRC Error Frame Receive Counter
    #[inline(always)]
    pub fn cefcr(&mut self) -> CefcrW<CefcrSpec> {
        CefcrW::new(self, 0)
    }
}
/**CRC Error Frame Receive Counter Register

You can [`read`](crate::Reg::read) this register and get [`cefcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cefcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CefcrSpec;
impl crate::RegisterSpec for CefcrSpec {
    type Ux = u32;
}
///`read()` method returns [`cefcr::R`](R) reader structure
impl crate::Readable for CefcrSpec {}
///`write(|w| ..)` method takes [`cefcr::W`](W) writer structure
impl crate::Writable for CefcrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CEFCR to value 0
impl crate::Resettable for CefcrSpec {}
