///Register `LGC0L` reader
pub type R = crate::R<Lgc0lSpec>;
///Register `LGC0L` writer
pub type W = crate::W<Lgc0lSpec>;
///Field `LGC0L` reader - CEC Transmission Logical 0 Low Width Setting
pub type Lgc0lR = crate::FieldReader<u16>;
///Field `LGC0L` writer - CEC Transmission Logical 0 Low Width Setting
pub type Lgc0lW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - CEC Transmission Logical 0 Low Width Setting
    #[inline(always)]
    pub fn lgc0l(&self) -> Lgc0lR {
        Lgc0lR::new(self.bits & 0x01ff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LGC0L").field("lgc0l", &self.lgc0l()).finish()
    }
}
impl W {
    ///Bits 0:8 - CEC Transmission Logical 0 Low Width Setting
    #[inline(always)]
    pub fn lgc0l(&mut self) -> Lgc0lW<Lgc0lSpec> {
        Lgc0lW::new(self, 0)
    }
}
/**CEC Transmission Logical 0 Low Width Setting Register

You can [`read`](crate::Reg::read) this register and get [`lgc0l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lgc0l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Lgc0lSpec;
impl crate::RegisterSpec for Lgc0lSpec {
    type Ux = u16;
}
///`read()` method returns [`lgc0l::R`](R) reader structure
impl crate::Readable for Lgc0lSpec {}
///`write(|w| ..)` method takes [`lgc0l::W`](W) writer structure
impl crate::Writable for Lgc0lSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LGC0L to value 0
impl crate::Resettable for Lgc0lSpec {}
