///Register `DATB` reader
pub type R = crate::R<DatbSpec>;
///Register `DATB` writer
pub type W = crate::W<DatbSpec>;
///Field `DATB` reader - CEC Transmission Data Bit Width Setting
pub type DatbR = crate::FieldReader<u16>;
///Field `DATB` writer - CEC Transmission Data Bit Width Setting
pub type DatbW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - CEC Transmission Data Bit Width Setting
    #[inline(always)]
    pub fn datb(&self) -> DatbR {
        DatbR::new(self.bits & 0x01ff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATB").field("datb", &self.datb()).finish()
    }
}
impl W {
    ///Bits 0:8 - CEC Transmission Data Bit Width Setting
    #[inline(always)]
    pub fn datb(&mut self) -> DatbW<DatbSpec> {
        DatbW::new(self, 0)
    }
}
/**CEC Transmission Data Bit Width Setting Register

You can [`read`](crate::Reg::read) this register and get [`datb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`datb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DatbSpec;
impl crate::RegisterSpec for DatbSpec {
    type Ux = u16;
}
///`read()` method returns [`datb::R`](R) reader structure
impl crate::Readable for DatbSpec {}
///`write(|w| ..)` method takes [`datb::W`](W) writer structure
impl crate::Writable for DatbSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DATB to value 0
impl crate::Resettable for DatbSpec {}
