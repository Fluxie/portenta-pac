///Register `STATLH` reader
pub type R = crate::R<StatlhSpec>;
///Register `STATLH` writer
pub type W = crate::W<StatlhSpec>;
///Field `STATLH` reader - CEC Reception Start Bit Maximum Bit Width Setting
pub type StatlhR = crate::FieldReader<u16>;
///Field `STATLH` writer - CEC Reception Start Bit Maximum Bit Width Setting
pub type StatlhW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - CEC Reception Start Bit Maximum Bit Width Setting
    #[inline(always)]
    pub fn statlh(&self) -> StatlhR {
        StatlhR::new(self.bits & 0x01ff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATLH").field("statlh", &self.statlh()).finish()
    }
}
impl W {
    ///Bits 0:8 - CEC Reception Start Bit Maximum Bit Width Setting
    #[inline(always)]
    pub fn statlh(&mut self) -> StatlhW<StatlhSpec> {
        StatlhW::new(self, 0)
    }
}
/**CEC Reception Start Bit Maximum Low Width Setting Register

You can [`read`](crate::Reg::read) this register and get [`statlh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`statlh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct StatlhSpec;
impl crate::RegisterSpec for StatlhSpec {
    type Ux = u16;
}
///`read()` method returns [`statlh::R`](R) reader structure
impl crate::Readable for StatlhSpec {}
///`write(|w| ..)` method takes [`statlh::W`](W) writer structure
impl crate::Writable for StatlhSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets STATLH to value 0
impl crate::Resettable for StatlhSpec {}
