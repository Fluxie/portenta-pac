///Register `FEADDR` reader
pub type R = crate::R<FeaddrSpec>;
///Register `FEADDR` writer
pub type W = crate::W<FeaddrSpec>;
///Field `FEADDR` reader - End Address for FACI Command Processing
pub type FeaddrR = crate::FieldReader<u32>;
///Field `FEADDR` writer - End Address for FACI Command Processing
pub type FeaddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - End Address for FACI Command Processing
    #[inline(always)]
    pub fn feaddr(&self) -> FeaddrR {
        FeaddrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FEADDR").field("feaddr", &self.feaddr()).finish()
    }
}
impl W {
    ///Bits 0:31 - End Address for FACI Command Processing
    #[inline(always)]
    pub fn feaddr(&mut self) -> FeaddrW<FeaddrSpec> {
        FeaddrW::new(self, 0)
    }
}
/**FACI Command End Address Register

You can [`read`](crate::Reg::read) this register and get [`feaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`feaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FeaddrSpec;
impl crate::RegisterSpec for FeaddrSpec {
    type Ux = u32;
}
///`read()` method returns [`feaddr::R`](R) reader structure
impl crate::Readable for FeaddrSpec {}
///`write(|w| ..)` method takes [`feaddr::W`](W) writer structure
impl crate::Writable for FeaddrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FEADDR to value 0
impl crate::Resettable for FeaddrSpec {}
