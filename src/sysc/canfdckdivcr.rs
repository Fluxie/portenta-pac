///Register `CANFDCKDIVCR` reader
pub type R = crate::R<CanfdckdivcrSpec>;
///Register `CANFDCKDIVCR` writer
pub type W = crate::W<CanfdckdivcrSpec>;
/**CANFD clock (CANFDCLK) Division Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Canfdckdiv {
    ///0: /1 (value after reset) /2 /4 /6 /3 /5
    _000 = 0,
}
impl From<Canfdckdiv> for u8 {
    #[inline(always)]
    fn from(variant: Canfdckdiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Canfdckdiv {
    type Ux = u8;
}
impl crate::IsEnum for Canfdckdiv {}
///Field `CANFDCKDIV` reader - CANFD clock (CANFDCLK) Division Select
pub type CanfdckdivR = crate::FieldReader<Canfdckdiv>;
impl CanfdckdivR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<Canfdckdiv> {
        match self.bits {
            0 => Some(Canfdckdiv::_000),
            _ => None,
        }
    }
    #[doc = "/1 (value after reset) /2 /4 /6 /3 /5"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Canfdckdiv::_000
    }
}
///Field `CANFDCKDIV` writer - CANFD clock (CANFDCLK) Division Select
pub type CanfdckdivW<'a, REG> = crate::FieldWriter<'a, REG, 3, Canfdckdiv>;
impl<'a, REG> CanfdckdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "/1 (value after reset) /2 /4 /6 /3 /5"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Canfdckdiv::_000)
    }
}
impl R {
    ///Bits 0:2 - CANFD clock (CANFDCLK) Division Select
    #[inline(always)]
    pub fn canfdckdiv(&self) -> CanfdckdivR {
        CanfdckdivR::new(self.bits & 7)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CANFDCKDIVCR").field("canfdckdiv", &self.canfdckdiv()).finish()
    }
}
impl W {
    ///Bits 0:2 - CANFD clock (CANFDCLK) Division Select
    #[inline(always)]
    pub fn canfdckdiv(&mut self) -> CanfdckdivW<CanfdckdivcrSpec> {
        CanfdckdivW::new(self, 0)
    }
}
/**CANFD Clock Division Control Register

You can [`read`](crate::Reg::read) this register and get [`canfdckdivcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`canfdckdivcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CanfdckdivcrSpec;
impl crate::RegisterSpec for CanfdckdivcrSpec {
    type Ux = u8;
}
///`read()` method returns [`canfdckdivcr::R`](R) reader structure
impl crate::Readable for CanfdckdivcrSpec {}
///`write(|w| ..)` method takes [`canfdckdivcr::W`](W) writer structure
impl crate::Writable for CanfdckdivcrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CANFDCKDIVCR to value 0
impl crate::Resettable for CanfdckdivcrSpec {}
