///Register `FRECR` reader
pub type R = crate::R<FrecrSpec>;
///Register `FRECR` writer
pub type W = crate::W<FrecrSpec>;
///Field `FRECR` reader - Frame Receive Error Counter
pub type FrecrR = crate::FieldReader<u32>;
///Field `FRECR` writer - Frame Receive Error Counter
pub type FrecrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Frame Receive Error Counter
    #[inline(always)]
    pub fn frecr(&self) -> FrecrR {
        FrecrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FRECR").field("frecr", &self.frecr()).finish()
    }
}
impl W {
    ///Bits 0:31 - Frame Receive Error Counter
    #[inline(always)]
    pub fn frecr(&mut self) -> FrecrW<FrecrSpec> {
        FrecrW::new(self, 0)
    }
}
/**Frame Receive Error Counter Register

You can [`read`](crate::Reg::read) this register and get [`frecr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frecr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FrecrSpec;
impl crate::RegisterSpec for FrecrSpec {
    type Ux = u32;
}
///`read()` method returns [`frecr::R`](R) reader structure
impl crate::Readable for FrecrSpec {}
///`write(|w| ..)` method takes [`frecr::W`](W) writer structure
impl crate::Writable for FrecrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FRECR to value 0
impl crate::Resettable for FrecrSpec {}
