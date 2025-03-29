///Register `RDMLR` reader
pub type R = crate::R<RdmlrSpec>;
///Register `RDMLR` writer
pub type W = crate::W<RdmlrSpec>;
///Field `RMD` reader - Random Number Generation Counter
pub type RmdR = crate::FieldReader<u32>;
///Field `RMD` writer - Random Number Generation Counter
pub type RmdW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    ///Bits 0:19 - Random Number Generation Counter
    #[inline(always)]
    pub fn rmd(&self) -> RmdR {
        RmdR::new(self.bits & 0x000f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDMLR").field("rmd", &self.rmd()).finish()
    }
}
impl W {
    ///Bits 0:19 - Random Number Generation Counter
    #[inline(always)]
    pub fn rmd(&mut self) -> RmdW<RdmlrSpec> {
        RmdW::new(self, 0)
    }
}
/**Random Number Generation Counter Upper Limit Setting Register

You can [`read`](crate::Reg::read) this register and get [`rdmlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rdmlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RdmlrSpec;
impl crate::RegisterSpec for RdmlrSpec {
    type Ux = u32;
}
///`read()` method returns [`rdmlr::R`](R) reader structure
impl crate::Readable for RdmlrSpec {}
///`write(|w| ..)` method takes [`rdmlr::W`](W) writer structure
impl crate::Writable for RdmlrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RDMLR to value 0
impl crate::Resettable for RdmlrSpec {}
