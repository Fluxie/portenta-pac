///Register `LGC1L` reader
pub type R = crate::R<Lgc1lSpec>;
///Register `LGC1L` writer
pub type W = crate::W<Lgc1lSpec>;
///Field `LGC1L` reader - CEC Transmission Logical 1 Low Width Setting
pub type Lgc1lR = crate::FieldReader<u16>;
///Field `LGC1L` writer - CEC Transmission Logical 1 Low Width Setting
pub type Lgc1lW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - CEC Transmission Logical 1 Low Width Setting
    #[inline(always)]
    pub fn lgc1l(&self) -> Lgc1lR {
        Lgc1lR::new(self.bits & 0x01ff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LGC1L").field("lgc1l", &self.lgc1l()).finish()
    }
}
impl W {
    ///Bits 0:8 - CEC Transmission Logical 1 Low Width Setting
    #[inline(always)]
    pub fn lgc1l(&mut self) -> Lgc1lW<Lgc1lSpec> {
        Lgc1lW::new(self, 0)
    }
}
/**CEC Transmission Logical 1 Low Width Setting Register

You can [`read`](crate::Reg::read) this register and get [`lgc1l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lgc1l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Lgc1lSpec;
impl crate::RegisterSpec for Lgc1lSpec {
    type Ux = u16;
}
///`read()` method returns [`lgc1l::R`](R) reader structure
impl crate::Readable for Lgc1lSpec {}
///`write(|w| ..)` method takes [`lgc1l::W`](W) writer structure
impl crate::Writable for Lgc1lSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LGC1L to value 0
impl crate::Resettable for Lgc1lSpec {}
