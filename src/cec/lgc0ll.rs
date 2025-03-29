///Register `LGC0LL` reader
pub type R = crate::R<Lgc0llSpec>;
///Register `LGC0LL` writer
pub type W = crate::W<Lgc0llSpec>;
///Field `LGC0LL` reader - CEC Reception Logical 0 Minimum Low Width Setting
pub type Lgc0llR = crate::FieldReader<u16>;
///Field `LGC0LL` writer - CEC Reception Logical 0 Minimum Low Width Setting
pub type Lgc0llW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - CEC Reception Logical 0 Minimum Low Width Setting
    #[inline(always)]
    pub fn lgc0ll(&self) -> Lgc0llR {
        Lgc0llR::new(self.bits & 0x01ff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LGC0LL").field("lgc0ll", &self.lgc0ll()).finish()
    }
}
impl W {
    ///Bits 0:8 - CEC Reception Logical 0 Minimum Low Width Setting
    #[inline(always)]
    pub fn lgc0ll(&mut self) -> Lgc0llW<Lgc0llSpec> {
        Lgc0llW::new(self, 0)
    }
}
/**CEC Reception Logical 0 Minimum Low Width Setting Register

You can [`read`](crate::Reg::read) this register and get [`lgc0ll::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lgc0ll::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Lgc0llSpec;
impl crate::RegisterSpec for Lgc0llSpec {
    type Ux = u16;
}
///`read()` method returns [`lgc0ll::R`](R) reader structure
impl crate::Readable for Lgc0llSpec {}
///`write(|w| ..)` method takes [`lgc0ll::W`](W) writer structure
impl crate::Writable for Lgc0llSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LGC0LL to value 0
impl crate::Resettable for Lgc0llSpec {}
