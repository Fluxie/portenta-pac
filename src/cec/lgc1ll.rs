///Register `LGC1LL` reader
pub type R = crate::R<Lgc1llSpec>;
///Register `LGC1LL` writer
pub type W = crate::W<Lgc1llSpec>;
///Field `LGC1LL` reader - CEC Reception Logical 1 Minimum Low Width Setting
pub type Lgc1llR = crate::FieldReader<u16>;
///Field `LGC1LL` writer - CEC Reception Logical 1 Minimum Low Width Setting
pub type Lgc1llW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - CEC Reception Logical 1 Minimum Low Width Setting
    #[inline(always)]
    pub fn lgc1ll(&self) -> Lgc1llR {
        Lgc1llR::new(self.bits & 0x01ff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LGC1LL").field("lgc1ll", &self.lgc1ll()).finish()
    }
}
impl W {
    ///Bits 0:8 - CEC Reception Logical 1 Minimum Low Width Setting
    #[inline(always)]
    pub fn lgc1ll(&mut self) -> Lgc1llW<Lgc1llSpec> {
        Lgc1llW::new(self, 0)
    }
}
/**CEC Reception Logical 1 Minimum Low Width Setting Register

You can [`read`](crate::Reg::read) this register and get [`lgc1ll::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lgc1ll::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Lgc1llSpec;
impl crate::RegisterSpec for Lgc1llSpec {
    type Ux = u16;
}
///`read()` method returns [`lgc1ll::R`](R) reader structure
impl crate::Readable for Lgc1llSpec {}
///`write(|w| ..)` method takes [`lgc1ll::W`](W) writer structure
impl crate::Writable for Lgc1llSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LGC1LL to value 0
impl crate::Resettable for Lgc1llSpec {}
