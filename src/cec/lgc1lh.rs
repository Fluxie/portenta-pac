///Register `LGC1LH` reader
pub type R = crate::R<Lgc1lhSpec>;
///Register `LGC1LH` writer
pub type W = crate::W<Lgc1lhSpec>;
///Field `LGC1LH` reader - CEC Reception Logical 1 Maximum Low Width Setting
pub type Lgc1lhR = crate::FieldReader<u16>;
///Field `LGC1LH` writer - CEC Reception Logical 1 Maximum Low Width Setting
pub type Lgc1lhW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - CEC Reception Logical 1 Maximum Low Width Setting
    #[inline(always)]
    pub fn lgc1lh(&self) -> Lgc1lhR {
        Lgc1lhR::new(self.bits & 0x01ff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LGC1LH").field("lgc1lh", &self.lgc1lh()).finish()
    }
}
impl W {
    ///Bits 0:8 - CEC Reception Logical 1 Maximum Low Width Setting
    #[inline(always)]
    pub fn lgc1lh(&mut self) -> Lgc1lhW<Lgc1lhSpec> {
        Lgc1lhW::new(self, 0)
    }
}
/**CEC Reception Logical 1 Maximum Low Width Setting Register

You can [`read`](crate::Reg::read) this register and get [`lgc1lh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lgc1lh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Lgc1lhSpec;
impl crate::RegisterSpec for Lgc1lhSpec {
    type Ux = u16;
}
///`read()` method returns [`lgc1lh::R`](R) reader structure
impl crate::Readable for Lgc1lhSpec {}
///`write(|w| ..)` method takes [`lgc1lh::W`](W) writer structure
impl crate::Writable for Lgc1lhSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LGC1LH to value 0
impl crate::Resettable for Lgc1lhSpec {}
