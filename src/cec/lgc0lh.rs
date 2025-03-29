///Register `LGC0LH` reader
pub type R = crate::R<Lgc0lhSpec>;
///Register `LGC0LH` writer
pub type W = crate::W<Lgc0lhSpec>;
///Field `LGC0LH` reader - CEC Reception Logical 0 Minimum Low Width Setting
pub type Lgc0lhR = crate::FieldReader<u16>;
///Field `LGC0LH` writer - CEC Reception Logical 0 Minimum Low Width Setting
pub type Lgc0lhW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - CEC Reception Logical 0 Minimum Low Width Setting
    #[inline(always)]
    pub fn lgc0lh(&self) -> Lgc0lhR {
        Lgc0lhR::new(self.bits & 0x01ff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LGC0LH").field("lgc0lh", &self.lgc0lh()).finish()
    }
}
impl W {
    ///Bits 0:8 - CEC Reception Logical 0 Minimum Low Width Setting
    #[inline(always)]
    pub fn lgc0lh(&mut self) -> Lgc0lhW<Lgc0lhSpec> {
        Lgc0lhW::new(self, 0)
    }
}
/**CEC Reception Logical 0 Maximum Low Width Setting Register

You can [`read`](crate::Reg::read) this register and get [`lgc0lh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lgc0lh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Lgc0lhSpec;
impl crate::RegisterSpec for Lgc0lhSpec {
    type Ux = u16;
}
///`read()` method returns [`lgc0lh::R`](R) reader structure
impl crate::Readable for Lgc0lhSpec {}
///`write(|w| ..)` method takes [`lgc0lh::W`](W) writer structure
impl crate::Writable for Lgc0lhSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LGC0LH to value 0
impl crate::Resettable for Lgc0lhSpec {}
