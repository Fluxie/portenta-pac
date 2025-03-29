///Register `DATBL` reader
pub type R = crate::R<DatblSpec>;
///Register `DATBL` writer
pub type W = crate::W<DatblSpec>;
///Field `DATBL` reader - CEC Reception Data Bit Minimum Bit Width Setting
pub type DatblR = crate::FieldReader<u16>;
///Field `DATBL` writer - CEC Reception Data Bit Minimum Bit Width Setting
pub type DatblW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - CEC Reception Data Bit Minimum Bit Width Setting
    #[inline(always)]
    pub fn datbl(&self) -> DatblR {
        DatblR::new(self.bits & 0x01ff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATBL").field("datbl", &self.datbl()).finish()
    }
}
impl W {
    ///Bits 0:8 - CEC Reception Data Bit Minimum Bit Width Setting
    #[inline(always)]
    pub fn datbl(&mut self) -> DatblW<DatblSpec> {
        DatblW::new(self, 0)
    }
}
/**CEC Reception Data Bit Minimum Bit Width Setting Register

You can [`read`](crate::Reg::read) this register and get [`datbl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`datbl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DatblSpec;
impl crate::RegisterSpec for DatblSpec {
    type Ux = u16;
}
///`read()` method returns [`datbl::R`](R) reader structure
impl crate::Readable for DatblSpec {}
///`write(|w| ..)` method takes [`datbl::W`](W) writer structure
impl crate::Writable for DatblSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DATBL to value 0
impl crate::Resettable for DatblSpec {}
