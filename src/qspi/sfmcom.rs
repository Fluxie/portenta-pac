///Register `SFMCOM` reader
pub type R = crate::R<SfmcomSpec>;
///Register `SFMCOM` writer
pub type W = crate::W<SfmcomSpec>;
///Field `SFMD` reader - Port for direct communication with the SPI bus
pub type SfmdR = crate::FieldReader;
///Field `SFMD` writer - Port for direct communication with the SPI bus
pub type SfmdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Port for direct communication with the SPI bus
    #[inline(always)]
    pub fn sfmd(&self) -> SfmdR {
        SfmdR::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SFMCOM").field("sfmd", &self.sfmd()).finish()
    }
}
impl W {
    ///Bits 0:7 - Port for direct communication with the SPI bus
    #[inline(always)]
    pub fn sfmd(&mut self) -> SfmdW<SfmcomSpec> {
        SfmdW::new(self, 0)
    }
}
/**Communication Port Register

You can [`read`](crate::Reg::read) this register and get [`sfmcom::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfmcom::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SfmcomSpec;
impl crate::RegisterSpec for SfmcomSpec {
    type Ux = u32;
}
///`read()` method returns [`sfmcom::R`](R) reader structure
impl crate::Readable for SfmcomSpec {}
///`write(|w| ..)` method takes [`sfmcom::W`](W) writer structure
impl crate::Writable for SfmcomSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SFMCOM to value 0
impl crate::Resettable for SfmcomSpec {}
