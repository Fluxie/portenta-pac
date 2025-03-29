///Register `CECCKDIVCR` reader
pub type R = crate::R<CecckdivcrSpec>;
///Register `CECCKDIVCR` writer
pub type W = crate::W<CecckdivcrSpec>;
/**CEC clock (CECCLK) Division Select

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cecckdiv {
    ///0: /1 (value after reset)
    _000 = 0,
    ///1: /2
    _001 = 1,
    ///2: /4
    _010 = 2,
    ///3: /6
    _011 = 3,
    ///4: /8
    _100 = 4,
    ///5: /3
    _101 = 5,
    ///6: /5
    _110 = 6,
    ///7: Setting prohibited
    Others = 7,
}
impl From<Cecckdiv> for u8 {
    #[inline(always)]
    fn from(variant: Cecckdiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cecckdiv {
    type Ux = u8;
}
impl crate::IsEnum for Cecckdiv {}
///Field `CECCKDIV` reader - CEC clock (CECCLK) Division Select
pub type CecckdivR = crate::FieldReader<Cecckdiv>;
impl CecckdivR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cecckdiv {
        match self.bits {
            0 => Cecckdiv::_000,
            1 => Cecckdiv::_001,
            2 => Cecckdiv::_010,
            3 => Cecckdiv::_011,
            4 => Cecckdiv::_100,
            5 => Cecckdiv::_101,
            6 => Cecckdiv::_110,
            7 => Cecckdiv::Others,
            _ => unreachable!(),
        }
    }
    #[doc = "/1 (value after reset)"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Cecckdiv::_000
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Cecckdiv::_001
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Cecckdiv::_010
    }
    #[doc = "/6"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Cecckdiv::_011
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Cecckdiv::_100
    }
    #[doc = "/3"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Cecckdiv::_101
    }
    #[doc = "/5"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Cecckdiv::_110
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        *self == Cecckdiv::Others
    }
}
///Field `CECCKDIV` writer - CEC clock (CECCLK) Division Select
pub type CecckdivW<'a, REG> = crate::FieldWriter<'a, REG, 3, Cecckdiv, crate::Safe>;
impl<'a, REG> CecckdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "/1 (value after reset)"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Cecckdiv::_000)
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Cecckdiv::_001)
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Cecckdiv::_010)
    }
    #[doc = "/6"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Cecckdiv::_011)
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Cecckdiv::_100)
    }
    #[doc = "/3"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Cecckdiv::_101)
    }
    #[doc = "/5"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Cecckdiv::_110)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Cecckdiv::Others)
    }
}
impl R {
    ///Bits 0:2 - CEC clock (CECCLK) Division Select
    #[inline(always)]
    pub fn cecckdiv(&self) -> CecckdivR {
        CecckdivR::new(self.bits & 7)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CECCKDIVCR").field("cecckdiv", &self.cecckdiv()).finish()
    }
}
impl W {
    ///Bits 0:2 - CEC clock (CECCLK) Division Select
    #[inline(always)]
    pub fn cecckdiv(&mut self) -> CecckdivW<CecckdivcrSpec> {
        CecckdivW::new(self, 0)
    }
}
/**CEC Clock Division Control Register

You can [`read`](crate::Reg::read) this register and get [`cecckdivcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cecckdivcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CecckdivcrSpec;
impl crate::RegisterSpec for CecckdivcrSpec {
    type Ux = u8;
}
///`read()` method returns [`cecckdivcr::R`](R) reader structure
impl crate::Readable for CecckdivcrSpec {}
///`write(|w| ..)` method takes [`cecckdivcr::W`](W) writer structure
impl crate::Writable for CecckdivcrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CECCKDIVCR to value 0x01
impl crate::Resettable for CecckdivcrSpec {
    const RESET_VALUE: u8 = 0x01;
}
