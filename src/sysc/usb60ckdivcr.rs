///Register `USB60CKDIVCR` reader
pub type R = crate::R<Usb60ckdivcrSpec>;
///Register `USB60CKDIVCR` writer
pub type W = crate::W<Usb60ckdivcrSpec>;
/**USB clock (USB60CLK) Division Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Usb60ckdiv {
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
impl From<Usb60ckdiv> for u8 {
    #[inline(always)]
    fn from(variant: Usb60ckdiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Usb60ckdiv {
    type Ux = u8;
}
impl crate::IsEnum for Usb60ckdiv {}
///Field `USB60CKDIV` reader - USB clock (USB60CLK) Division Select
pub type Usb60ckdivR = crate::FieldReader<Usb60ckdiv>;
impl Usb60ckdivR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Usb60ckdiv {
        match self.bits {
            0 => Usb60ckdiv::_000,
            1 => Usb60ckdiv::_001,
            2 => Usb60ckdiv::_010,
            3 => Usb60ckdiv::_011,
            4 => Usb60ckdiv::_100,
            5 => Usb60ckdiv::_101,
            6 => Usb60ckdiv::_110,
            7 => Usb60ckdiv::Others,
            _ => unreachable!(),
        }
    }
    #[doc = "/1 (value after reset)"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Usb60ckdiv::_000
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Usb60ckdiv::_001
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Usb60ckdiv::_010
    }
    #[doc = "/6"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Usb60ckdiv::_011
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Usb60ckdiv::_100
    }
    #[doc = "/3"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Usb60ckdiv::_101
    }
    #[doc = "/5"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Usb60ckdiv::_110
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        *self == Usb60ckdiv::Others
    }
}
///Field `USB60CKDIV` writer - USB clock (USB60CLK) Division Select
pub type Usb60ckdivW<'a, REG> = crate::FieldWriter<'a, REG, 3, Usb60ckdiv, crate::Safe>;
impl<'a, REG> Usb60ckdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "/1 (value after reset)"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Usb60ckdiv::_000)
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Usb60ckdiv::_001)
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Usb60ckdiv::_010)
    }
    #[doc = "/6"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Usb60ckdiv::_011)
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Usb60ckdiv::_100)
    }
    #[doc = "/3"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Usb60ckdiv::_101)
    }
    #[doc = "/5"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Usb60ckdiv::_110)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Usb60ckdiv::Others)
    }
}
impl R {
    ///Bits 0:2 - USB clock (USB60CLK) Division Select
    #[inline(always)]
    pub fn usb60ckdiv(&self) -> Usb60ckdivR {
        Usb60ckdivR::new(self.bits & 7)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB60CKDIVCR").field("usb60ckdiv", &self.usb60ckdiv()).finish()
    }
}
impl W {
    ///Bits 0:2 - USB clock (USB60CLK) Division Select
    #[inline(always)]
    pub fn usb60ckdiv(&mut self) -> Usb60ckdivW<Usb60ckdivcrSpec> {
        Usb60ckdivW::new(self, 0)
    }
}
/**USB60 Clock Division Control Register

You can [`read`](crate::Reg::read) this register and get [`usb60ckdivcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb60ckdivcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Usb60ckdivcrSpec;
impl crate::RegisterSpec for Usb60ckdivcrSpec {
    type Ux = u8;
}
///`read()` method returns [`usb60ckdivcr::R`](R) reader structure
impl crate::Readable for Usb60ckdivcrSpec {}
///`write(|w| ..)` method takes [`usb60ckdivcr::W`](W) writer structure
impl crate::Writable for Usb60ckdivcrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets USB60CKDIVCR to value 0
impl crate::Resettable for Usb60ckdivcrSpec {}
