///Register `CWDR` writer
pub type W = crate::W<CwdrSpec>;
///Field `WD0` writer - Write data 0
pub type Wd0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `WD1` writer - Write data 1
pub type Wd1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `WD2` writer - Write data 2
pub type Wd2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `WD3` writer - Write data 3
pub type Wd3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl core::fmt::Debug for crate::generic::Reg<CwdrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:7 - Write data 0
    #[inline(always)]
    pub fn wd0(&mut self) -> Wd0W<CwdrSpec> {
        Wd0W::new(self, 0)
    }
    ///Bits 8:15 - Write data 1
    #[inline(always)]
    pub fn wd1(&mut self) -> Wd1W<CwdrSpec> {
        Wd1W::new(self, 8)
    }
    ///Bits 16:23 - Write data 2
    #[inline(always)]
    pub fn wd2(&mut self) -> Wd2W<CwdrSpec> {
        Wd2W::new(self, 16)
    }
    ///Bits 24:31 - Write data 3
    #[inline(always)]
    pub fn wd3(&mut self) -> Wd3W<CwdrSpec> {
        Wd3W::new(self, 24)
    }
}
/**Configure Write Data Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cwdr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CwdrSpec;
impl crate::RegisterSpec for CwdrSpec {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`cwdr::W`](W) writer structure
impl crate::Writable for CwdrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CWDR to value 0
impl crate::Resettable for CwdrSpec {}
