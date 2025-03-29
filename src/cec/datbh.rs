///Register `DATBH` reader
pub type R = crate::R<DatbhSpec>;
///Register `DATBH` writer
pub type W = crate::W<DatbhSpec>;
///Field `DATBH` reader - CEC Reception Data Bit Maximum Bit Width Setting
pub type DatbhR = crate::FieldReader<u16>;
///Field `DATBH` writer - CEC Reception Data Bit Maximum Bit Width Setting
pub type DatbhW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - CEC Reception Data Bit Maximum Bit Width Setting
    #[inline(always)]
    pub fn datbh(&self) -> DatbhR {
        DatbhR::new(self.bits & 0x01ff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATBH").field("datbh", &self.datbh()).finish()
    }
}
impl W {
    ///Bits 0:8 - CEC Reception Data Bit Maximum Bit Width Setting
    #[inline(always)]
    pub fn datbh(&mut self) -> DatbhW<DatbhSpec> {
        DatbhW::new(self, 0)
    }
}
/**CEC Reception Data Bit Maximum Bit Width Setting Register

You can [`read`](crate::Reg::read) this register and get [`datbh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`datbh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DatbhSpec;
impl crate::RegisterSpec for DatbhSpec {
    type Ux = u16;
}
///`read()` method returns [`datbh::R`](R) reader structure
impl crate::Readable for DatbhSpec {}
///`write(|w| ..)` method takes [`datbh::W`](W) writer structure
impl crate::Writable for DatbhSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DATBH to value 0
impl crate::Resettable for DatbhSpec {}
