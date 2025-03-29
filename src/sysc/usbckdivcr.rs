///Register `USBCKDIVCR` reader
pub type R = crate::R<UsbckdivcrSpec>;
///Register `USBCKDIVCR` writer
pub type W = crate::W<UsbckdivcrSpec>;
/**USB Clock (USBCLK) Division Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Usbckdiv {
    ///2: ∕ 4
    _010 = 2,
    ///5: ∕ 3
    _101 = 5,
    ///6: ∕ 5
    _110 = 6,
    ///0: Setting prohibited.
    Others = 0,
}
impl From<Usbckdiv> for u8 {
    #[inline(always)]
    fn from(variant: Usbckdiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Usbckdiv {
    type Ux = u8;
}
impl crate::IsEnum for Usbckdiv {}
///Field `USBCKDIV` reader - USB Clock (USBCLK) Division Select
pub type UsbckdivR = crate::FieldReader<Usbckdiv>;
impl UsbckdivR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Usbckdiv {
        match self.bits {
            2 => Usbckdiv::_010,
            5 => Usbckdiv::_101,
            6 => Usbckdiv::_110,
            _ => Usbckdiv::Others,
        }
    }
    ///∕ 4
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Usbckdiv::_010
    }
    ///∕ 3
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Usbckdiv::_101
    }
    ///∕ 5
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Usbckdiv::_110
    }
    ///Setting prohibited.
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Usbckdiv::Others)
    }
}
///Field `USBCKDIV` writer - USB Clock (USBCLK) Division Select
pub type UsbckdivW<'a, REG> = crate::FieldWriter<'a, REG, 3, Usbckdiv, crate::Safe>;
impl<'a, REG> UsbckdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///∕ 4
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Usbckdiv::_010)
    }
    ///∕ 3
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Usbckdiv::_101)
    }
    ///∕ 5
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Usbckdiv::_110)
    }
    ///Setting prohibited.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Usbckdiv::Others)
    }
}
impl R {
    ///Bits 0:2 - USB Clock (USBCLK) Division Select
    #[inline(always)]
    pub fn usbckdiv(&self) -> UsbckdivR {
        UsbckdivR::new(self.bits & 7)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USBCKDIVCR").field("usbckdiv", &self.usbckdiv()).finish()
    }
}
impl W {
    ///Bits 0:2 - USB Clock (USBCLK) Division Select
    #[inline(always)]
    pub fn usbckdiv(&mut self) -> UsbckdivW<UsbckdivcrSpec> {
        UsbckdivW::new(self, 0)
    }
}
/**USB Clock Division Control Register

You can [`read`](crate::Reg::read) this register and get [`usbckdivcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbckdivcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct UsbckdivcrSpec;
impl crate::RegisterSpec for UsbckdivcrSpec {
    type Ux = u8;
}
///`read()` method returns [`usbckdivcr::R`](R) reader structure
impl crate::Readable for UsbckdivcrSpec {}
///`write(|w| ..)` method takes [`usbckdivcr::W`](W) writer structure
impl crate::Writable for UsbckdivcrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets USBCKDIVCR to value 0
impl crate::Resettable for UsbckdivcrSpec {}
