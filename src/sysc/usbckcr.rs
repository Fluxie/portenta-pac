///Register `USBCKCR` reader
pub type R = crate::R<UsbckcrSpec>;
///Register `USBCKCR` writer
pub type W = crate::W<UsbckcrSpec>;
/**USB Clock (USBCLK) Source Select

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Usbcksel {
    ///5: PLL
    _101 = 5,
    ///6: PLL2
    _110 = 6,
    ///0: Setting prohibited.
    Others = 0,
}
impl From<Usbcksel> for u8 {
    #[inline(always)]
    fn from(variant: Usbcksel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Usbcksel {
    type Ux = u8;
}
impl crate::IsEnum for Usbcksel {}
///Field `USBCKSEL` reader - USB Clock (USBCLK) Source Select
pub type UsbckselR = crate::FieldReader<Usbcksel>;
impl UsbckselR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Usbcksel {
        match self.bits {
            5 => Usbcksel::_101,
            6 => Usbcksel::_110,
            _ => Usbcksel::Others,
        }
    }
    ///PLL
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Usbcksel::_101
    }
    ///PLL2
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Usbcksel::_110
    }
    ///Setting prohibited.
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Usbcksel::Others)
    }
}
///Field `USBCKSEL` writer - USB Clock (USBCLK) Source Select
pub type UsbckselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Usbcksel, crate::Safe>;
impl<'a, REG> UsbckselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PLL
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Usbcksel::_101)
    }
    ///PLL2
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Usbcksel::_110)
    }
    ///Setting prohibited.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Usbcksel::Others)
    }
}
/**USB Clock (USBCLK) Switching Request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbcksreq {
    ///0: No request
    _0 = 0,
    ///1: Request switching.
    _1 = 1,
}
impl From<Usbcksreq> for bool {
    #[inline(always)]
    fn from(variant: Usbcksreq) -> Self {
        variant as u8 != 0
    }
}
///Field `USBCKSREQ` reader - USB Clock (USBCLK) Switching Request
pub type UsbcksreqR = crate::BitReader<Usbcksreq>;
impl UsbcksreqR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Usbcksreq {
        match self.bits {
            false => Usbcksreq::_0,
            true => Usbcksreq::_1,
        }
    }
    ///No request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Usbcksreq::_0
    }
    ///Request switching.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Usbcksreq::_1
    }
}
///Field `USBCKSREQ` writer - USB Clock (USBCLK) Switching Request
pub type UsbcksreqW<'a, REG> = crate::BitWriter<'a, REG, Usbcksreq>;
impl<'a, REG> UsbcksreqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Usbcksreq::_0)
    }
    ///Request switching.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Usbcksreq::_1)
    }
}
/**USB Clock (USBCLK) Switching Ready state flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbcksrdy {
    ///0: Impossible to Switch
    _0 = 0,
    ///1: Possible to Switch
    _1 = 1,
}
impl From<Usbcksrdy> for bool {
    #[inline(always)]
    fn from(variant: Usbcksrdy) -> Self {
        variant as u8 != 0
    }
}
///Field `USBCKSRDY` reader - USB Clock (USBCLK) Switching Ready state flag
pub type UsbcksrdyR = crate::BitReader<Usbcksrdy>;
impl UsbcksrdyR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Usbcksrdy {
        match self.bits {
            false => Usbcksrdy::_0,
            true => Usbcksrdy::_1,
        }
    }
    ///Impossible to Switch
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Usbcksrdy::_0
    }
    ///Possible to Switch
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Usbcksrdy::_1
    }
}
impl R {
    ///Bits 0:2 - USB Clock (USBCLK) Source Select
    #[inline(always)]
    pub fn usbcksel(&self) -> UsbckselR {
        UsbckselR::new(self.bits & 7)
    }
    ///Bit 6 - USB Clock (USBCLK) Switching Request
    #[inline(always)]
    pub fn usbcksreq(&self) -> UsbcksreqR {
        UsbcksreqR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - USB Clock (USBCLK) Switching Ready state flag
    #[inline(always)]
    pub fn usbcksrdy(&self) -> UsbcksrdyR {
        UsbcksrdyR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USBCKCR")
            .field("usbcksel", &self.usbcksel())
            .field("usbcksreq", &self.usbcksreq())
            .field("usbcksrdy", &self.usbcksrdy())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - USB Clock (USBCLK) Source Select
    #[inline(always)]
    pub fn usbcksel(&mut self) -> UsbckselW<UsbckcrSpec> {
        UsbckselW::new(self, 0)
    }
    ///Bit 6 - USB Clock (USBCLK) Switching Request
    #[inline(always)]
    pub fn usbcksreq(&mut self) -> UsbcksreqW<UsbckcrSpec> {
        UsbcksreqW::new(self, 6)
    }
}
/**USB Clock Control Register

You can [`read`](crate::Reg::read) this register and get [`usbckcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbckcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct UsbckcrSpec;
impl crate::RegisterSpec for UsbckcrSpec {
    type Ux = u8;
}
///`read()` method returns [`usbckcr::R`](R) reader structure
impl crate::Readable for UsbckcrSpec {}
///`write(|w| ..)` method takes [`usbckcr::W`](W) writer structure
impl crate::Writable for UsbckcrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets USBCKCR to value 0x01
impl crate::Resettable for UsbckcrSpec {
    const RESET_VALUE: u8 = 0x01;
}
