///Register `SFMSIC` reader
pub type R = crate::R<SfmsicSpec>;
///Register `SFMSIC` writer
pub type W = crate::W<SfmsicSpec>;
///Field `SFMCIC` reader - Serial flash instruction code to substitute
pub type SfmcicR = crate::FieldReader;
///Field `SFMCIC` writer - Serial flash instruction code to substitute
pub type SfmcicW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Serial flash instruction code to substitute
    #[inline(always)]
    pub fn sfmcic(&self) -> SfmcicR {
        SfmcicR::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SFMSIC").field("sfmcic", &self.sfmcic()).finish()
    }
}
impl W {
    ///Bits 0:7 - Serial flash instruction code to substitute
    #[inline(always)]
    pub fn sfmcic(&mut self) -> SfmcicW<SfmsicSpec> {
        SfmcicW::new(self, 0)
    }
}
/**Instruction Code Register

You can [`read`](crate::Reg::read) this register and get [`sfmsic::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfmsic::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SfmsicSpec;
impl crate::RegisterSpec for SfmsicSpec {
    type Ux = u32;
}
///`read()` method returns [`sfmsic::R`](R) reader structure
impl crate::Readable for SfmsicSpec {}
///`write(|w| ..)` method takes [`sfmsic::W`](W) writer structure
impl crate::Writable for SfmsicSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SFMSIC to value 0
impl crate::Resettable for SfmsicSpec {}
