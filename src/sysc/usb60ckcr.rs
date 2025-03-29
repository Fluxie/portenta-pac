///Register `USB60CKCR` reader
pub type R = crate::R<Usb60ckcrSpec>;
///Register `USB60CKCR` writer
pub type W = crate::W<Usb60ckcrSpec>;
/**USB clock (USB60CLK) Source Select

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Usb60cksel {
    ///5: PLL
    _101 = 5,
    ///6: PLL2
    _110 = 6,
    ///0: Setting prohibited
    Others = 0,
}
impl From<Usb60cksel> for u8 {
    #[inline(always)]
    fn from(variant: Usb60cksel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Usb60cksel {
    type Ux = u8;
}
impl crate::IsEnum for Usb60cksel {}
///Field `USB60CKSEL` reader - USB clock (USB60CLK) Source Select
pub type Usb60ckselR = crate::FieldReader<Usb60cksel>;
impl Usb60ckselR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Usb60cksel {
        match self.bits {
            5 => Usb60cksel::_101,
            6 => Usb60cksel::_110,
            _ => Usb60cksel::Others,
        }
    }
    ///PLL
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Usb60cksel::_101
    }
    ///PLL2
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Usb60cksel::_110
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Usb60cksel::Others)
    }
}
///Field `USB60CKSEL` writer - USB clock (USB60CLK) Source Select
pub type Usb60ckselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Usb60cksel, crate::Safe>;
impl<'a, REG> Usb60ckselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PLL
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Usb60cksel::_101)
    }
    ///PLL2
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Usb60cksel::_110)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Usb60cksel::Others)
    }
}
/**USB clock (USB60CLK) Switching Request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usb60cksreq {
    ///0: No request
    _0 = 0,
    ///1: Request switching
    _1 = 1,
}
impl From<Usb60cksreq> for bool {
    #[inline(always)]
    fn from(variant: Usb60cksreq) -> Self {
        variant as u8 != 0
    }
}
///Field `USB60CKSREQ` reader - USB clock (USB60CLK) Switching Request
pub type Usb60cksreqR = crate::BitReader<Usb60cksreq>;
impl Usb60cksreqR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Usb60cksreq {
        match self.bits {
            false => Usb60cksreq::_0,
            true => Usb60cksreq::_1,
        }
    }
    ///No request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Usb60cksreq::_0
    }
    ///Request switching
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Usb60cksreq::_1
    }
}
///Field `USB60CKSREQ` writer - USB clock (USB60CLK) Switching Request
pub type Usb60cksreqW<'a, REG> = crate::BitWriter<'a, REG, Usb60cksreq>;
impl<'a, REG> Usb60cksreqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Usb60cksreq::_0)
    }
    ///Request switching
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Usb60cksreq::_1)
    }
}
/**USB clock (USB60CLK) Switching Ready state flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usb60cksrdy {
    ///0: Impossible to Switch
    _0 = 0,
    ///1: Possible to Switch
    _1 = 1,
}
impl From<Usb60cksrdy> for bool {
    #[inline(always)]
    fn from(variant: Usb60cksrdy) -> Self {
        variant as u8 != 0
    }
}
///Field `USB60CKSRDY` reader - USB clock (USB60CLK) Switching Ready state flag
pub type Usb60cksrdyR = crate::BitReader<Usb60cksrdy>;
impl Usb60cksrdyR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Usb60cksrdy {
        match self.bits {
            false => Usb60cksrdy::_0,
            true => Usb60cksrdy::_1,
        }
    }
    ///Impossible to Switch
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Usb60cksrdy::_0
    }
    ///Possible to Switch
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Usb60cksrdy::_1
    }
}
impl R {
    ///Bits 0:2 - USB clock (USB60CLK) Source Select
    #[inline(always)]
    pub fn usb60cksel(&self) -> Usb60ckselR {
        Usb60ckselR::new(self.bits & 7)
    }
    ///Bit 6 - USB clock (USB60CLK) Switching Request
    #[inline(always)]
    pub fn usb60cksreq(&self) -> Usb60cksreqR {
        Usb60cksreqR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - USB clock (USB60CLK) Switching Ready state flag
    #[inline(always)]
    pub fn usb60cksrdy(&self) -> Usb60cksrdyR {
        Usb60cksrdyR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB60CKCR")
            .field("usb60cksel", &self.usb60cksel())
            .field("usb60cksreq", &self.usb60cksreq())
            .field("usb60cksrdy", &self.usb60cksrdy())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - USB clock (USB60CLK) Source Select
    #[inline(always)]
    pub fn usb60cksel(&mut self) -> Usb60ckselW<Usb60ckcrSpec> {
        Usb60ckselW::new(self, 0)
    }
    ///Bit 6 - USB clock (USB60CLK) Switching Request
    #[inline(always)]
    pub fn usb60cksreq(&mut self) -> Usb60cksreqW<Usb60ckcrSpec> {
        Usb60cksreqW::new(self, 6)
    }
}
/**USB60 Clock Control Register

You can [`read`](crate::Reg::read) this register and get [`usb60ckcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb60ckcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Usb60ckcrSpec;
impl crate::RegisterSpec for Usb60ckcrSpec {
    type Ux = u8;
}
///`read()` method returns [`usb60ckcr::R`](R) reader structure
impl crate::Readable for Usb60ckcrSpec {}
///`write(|w| ..)` method takes [`usb60ckcr::W`](W) writer structure
impl crate::Writable for Usb60ckcrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets USB60CKCR to value 0x01
impl crate::Resettable for Usb60ckcrSpec {
    const RESET_VALUE: u8 = 0x01;
}
