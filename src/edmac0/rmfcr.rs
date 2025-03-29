///Register `RMFCR` reader
pub type R = crate::R<RmfcrSpec>;
///Register `RMFCR` writer
pub type W = crate::W<RmfcrSpec>;
///Field `MFC` reader - Missed-Frame Counter
pub type MfcR = crate::FieldReader<u16>;
///Field `MFC` writer - Missed-Frame Counter
pub type MfcW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Missed-Frame Counter
    #[inline(always)]
    pub fn mfc(&self) -> MfcR {
        MfcR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RMFCR").field("mfc", &self.mfc()).finish()
    }
}
impl W {
    ///Bits 0:15 - Missed-Frame Counter
    #[inline(always)]
    pub fn mfc(&mut self) -> MfcW<RmfcrSpec> {
        MfcW::new(self, 0)
    }
}
/**Missed-Frame Counter Register

You can [`read`](crate::Reg::read) this register and get [`rmfcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rmfcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RmfcrSpec;
impl crate::RegisterSpec for RmfcrSpec {
    type Ux = u32;
}
///`read()` method returns [`rmfcr::R`](R) reader structure
impl crate::Readable for RmfcrSpec {}
///`write(|w| ..)` method takes [`rmfcr::W`](W) writer structure
impl crate::Writable for RmfcrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RMFCR to value 0
impl crate::Resettable for RmfcrSpec {}
