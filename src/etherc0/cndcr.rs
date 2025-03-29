///Register `CNDCR` reader
pub type R = crate::R<CndcrSpec>;
///Register `CNDCR` writer
pub type W = crate::W<CndcrSpec>;
///Field `CNDCR` reader - Carrier Not Detect Counter
pub type CndcrR = crate::FieldReader<u32>;
///Field `CNDCR` writer - Carrier Not Detect Counter
pub type CndcrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Carrier Not Detect Counter
    #[inline(always)]
    pub fn cndcr(&self) -> CndcrR {
        CndcrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CNDCR").field("cndcr", &self.cndcr()).finish()
    }
}
impl W {
    ///Bits 0:31 - Carrier Not Detect Counter
    #[inline(always)]
    pub fn cndcr(&mut self) -> CndcrW<CndcrSpec> {
        CndcrW::new(self, 0)
    }
}
/**Carrier Not Detect Counter Register

You can [`read`](crate::Reg::read) this register and get [`cndcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cndcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CndcrSpec;
impl crate::RegisterSpec for CndcrSpec {
    type Ux = u32;
}
///`read()` method returns [`cndcr::R`](R) reader structure
impl crate::Readable for CndcrSpec {}
///`write(|w| ..)` method takes [`cndcr::W`](W) writer structure
impl crate::Writable for CndcrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CNDCR to value 0
impl crate::Resettable for CndcrSpec {}
