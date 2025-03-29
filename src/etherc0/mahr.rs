///Register `MAHR` reader
pub type R = crate::R<MahrSpec>;
///Register `MAHR` writer
pub type W = crate::W<MahrSpec>;
///Field `MAHR` reader - MAC Address Upper Bit
pub type MahrR = crate::FieldReader<u32>;
///Field `MAHR` writer - MAC Address Upper Bit
pub type MahrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - MAC Address Upper Bit
    #[inline(always)]
    pub fn mahr(&self) -> MahrR {
        MahrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MAHR").field("mahr", &self.mahr()).finish()
    }
}
impl W {
    ///Bits 0:31 - MAC Address Upper Bit
    #[inline(always)]
    pub fn mahr(&mut self) -> MahrW<MahrSpec> {
        MahrW::new(self, 0)
    }
}
/**MAC Address Upper Bit Register

You can [`read`](crate::Reg::read) this register and get [`mahr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mahr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MahrSpec;
impl crate::RegisterSpec for MahrSpec {
    type Ux = u32;
}
///`read()` method returns [`mahr::R`](R) reader structure
impl crate::Readable for MahrSpec {}
///`write(|w| ..)` method takes [`mahr::W`](W) writer structure
impl crate::Writable for MahrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MAHR to value 0
impl crate::Resettable for MahrSpec {}
