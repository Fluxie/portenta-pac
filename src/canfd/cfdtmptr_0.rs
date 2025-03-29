///Register `CFDTMPTR%s_0` reader
pub type R = crate::R<Cfdtmptr0Spec>;
///Register `CFDTMPTR%s_0` writer
pub type W = crate::W<Cfdtmptr0Spec>;
///Field `TMDLC` reader - TX Message Buffer DLC Field
pub type TmdlcR = crate::FieldReader;
///Field `TMDLC` writer - TX Message Buffer DLC Field
pub type TmdlcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 28:31 - TX Message Buffer DLC Field
    #[inline(always)]
    pub fn tmdlc(&self) -> TmdlcR {
        TmdlcR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDTMPTR_0").field("tmdlc", &self.tmdlc()).finish()
    }
}
impl W {
    ///Bits 28:31 - TX Message Buffer DLC Field
    #[inline(always)]
    pub fn tmdlc(&mut self) -> TmdlcW<Cfdtmptr0Spec> {
        TmdlcW::new(self, 28)
    }
}
/**TX Message Buffer Pointer Register %s Channel 0

You can [`read`](crate::Reg::read) this register and get [`cfdtmptr_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtmptr_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Cfdtmptr0Spec;
impl crate::RegisterSpec for Cfdtmptr0Spec {
    type Ux = u32;
}
///`read()` method returns [`cfdtmptr_0::R`](R) reader structure
impl crate::Readable for Cfdtmptr0Spec {}
///`write(|w| ..)` method takes [`cfdtmptr_0::W`](W) writer structure
impl crate::Writable for Cfdtmptr0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDTMPTR%s_0 to value 0
impl crate::Resettable for Cfdtmptr0Spec {}
