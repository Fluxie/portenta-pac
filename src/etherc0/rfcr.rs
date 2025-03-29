///Register `RFCR` reader
pub type R = crate::R<RfcrSpec>;
///Register `RFCR` writer
pub type W = crate::W<RfcrSpec>;
///Field `RFCR` reader - Received Alignment Error Frame Counter
pub type RfcrR = crate::FieldReader<u32>;
///Field `RFCR` writer - Received Alignment Error Frame Counter
pub type RfcrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Received Alignment Error Frame Counter
    #[inline(always)]
    pub fn rfcr(&self) -> RfcrR {
        RfcrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RFCR").field("rfcr", &self.rfcr()).finish()
    }
}
impl W {
    ///Bits 0:31 - Received Alignment Error Frame Counter
    #[inline(always)]
    pub fn rfcr(&mut self) -> RfcrW<RfcrSpec> {
        RfcrW::new(self, 0)
    }
}
/**Received Alignment Error Frame Counter Register

You can [`read`](crate::Reg::read) this register and get [`rfcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RfcrSpec;
impl crate::RegisterSpec for RfcrSpec {
    type Ux = u32;
}
///`read()` method returns [`rfcr::R`](R) reader structure
impl crate::Readable for RfcrSpec {}
///`write(|w| ..)` method takes [`rfcr::W`](W) writer structure
impl crate::Writable for RfcrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RFCR to value 0
impl crate::Resettable for RfcrSpec {}
