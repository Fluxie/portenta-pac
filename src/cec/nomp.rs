///Register `NOMP` reader
pub type R = crate::R<NompSpec>;
///Register `NOMP` writer
pub type W = crate::W<NompSpec>;
///Field `NOMP` reader - CEC Data Bit Reference Width Setting
pub type NompR = crate::FieldReader<u16>;
///Field `NOMP` writer - CEC Data Bit Reference Width Setting
pub type NompW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - CEC Data Bit Reference Width Setting
    #[inline(always)]
    pub fn nomp(&self) -> NompR {
        NompR::new(self.bits & 0x01ff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NOMP").field("nomp", &self.nomp()).finish()
    }
}
impl W {
    ///Bits 0:8 - CEC Data Bit Reference Width Setting
    #[inline(always)]
    pub fn nomp(&mut self) -> NompW<NompSpec> {
        NompW::new(self, 0)
    }
}
/**CEC Data Bit Reference Width Setting Register

You can [`read`](crate::Reg::read) this register and get [`nomp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nomp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct NompSpec;
impl crate::RegisterSpec for NompSpec {
    type Ux = u16;
}
///`read()` method returns [`nomp::R`](R) reader structure
impl crate::Readable for NompSpec {}
///`write(|w| ..)` method takes [`nomp::W`](W) writer structure
impl crate::Writable for NompSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets NOMP to value 0
impl crate::Resettable for NompSpec {}
