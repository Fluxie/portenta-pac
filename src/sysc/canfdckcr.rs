///Register `CANFDCKCR` reader
pub type R = crate::R<CanfdckcrSpec>;
///Register `CANFDCKCR` writer
pub type W = crate::W<CanfdckcrSpec>;
/**CANFD clock (CANFDCLK) Source Select

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Canfdcksel {
    ///5: PLL
    _101 = 5,
    ///6: PLL2
    _110 = 6,
    ///0: Setting prohibited
    Others = 0,
}
impl From<Canfdcksel> for u8 {
    #[inline(always)]
    fn from(variant: Canfdcksel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Canfdcksel {
    type Ux = u8;
}
impl crate::IsEnum for Canfdcksel {}
///Field `CANFDCKSEL` reader - CANFD clock (CANFDCLK) Source Select
pub type CanfdckselR = crate::FieldReader<Canfdcksel>;
impl CanfdckselR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Canfdcksel {
        match self.bits {
            5 => Canfdcksel::_101,
            6 => Canfdcksel::_110,
            _ => Canfdcksel::Others,
        }
    }
    ///PLL
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Canfdcksel::_101
    }
    ///PLL2
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Canfdcksel::_110
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Canfdcksel::Others)
    }
}
///Field `CANFDCKSEL` writer - CANFD clock (CANFDCLK) Source Select
pub type CanfdckselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Canfdcksel, crate::Safe>;
impl<'a, REG> CanfdckselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PLL
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Canfdcksel::_101)
    }
    ///PLL2
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Canfdcksel::_110)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Canfdcksel::Others)
    }
}
/**CANFD clock (CANFDCLK) Switching Request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Canfdcksreq {
    ///0: No request
    _0 = 0,
    ///1: Request switching
    _1 = 1,
}
impl From<Canfdcksreq> for bool {
    #[inline(always)]
    fn from(variant: Canfdcksreq) -> Self {
        variant as u8 != 0
    }
}
///Field `CANFDCKSREQ` reader - CANFD clock (CANFDCLK) Switching Request
pub type CanfdcksreqR = crate::BitReader<Canfdcksreq>;
impl CanfdcksreqR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Canfdcksreq {
        match self.bits {
            false => Canfdcksreq::_0,
            true => Canfdcksreq::_1,
        }
    }
    ///No request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Canfdcksreq::_0
    }
    ///Request switching
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Canfdcksreq::_1
    }
}
///Field `CANFDCKSREQ` writer - CANFD clock (CANFDCLK) Switching Request
pub type CanfdcksreqW<'a, REG> = crate::BitWriter<'a, REG, Canfdcksreq>;
impl<'a, REG> CanfdcksreqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Canfdcksreq::_0)
    }
    ///Request switching
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Canfdcksreq::_1)
    }
}
/**CANFD clock (CANFDCLK) Switching Ready state flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Canfdcksrdy {
    ///0: Impossible to Switch
    _0 = 0,
    ///1: Possible to Switch
    _1 = 1,
}
impl From<Canfdcksrdy> for bool {
    #[inline(always)]
    fn from(variant: Canfdcksrdy) -> Self {
        variant as u8 != 0
    }
}
///Field `CANFDCKSRDY` reader - CANFD clock (CANFDCLK) Switching Ready state flag
pub type CanfdcksrdyR = crate::BitReader<Canfdcksrdy>;
impl CanfdcksrdyR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Canfdcksrdy {
        match self.bits {
            false => Canfdcksrdy::_0,
            true => Canfdcksrdy::_1,
        }
    }
    ///Impossible to Switch
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Canfdcksrdy::_0
    }
    ///Possible to Switch
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Canfdcksrdy::_1
    }
}
impl R {
    ///Bits 0:2 - CANFD clock (CANFDCLK) Source Select
    #[inline(always)]
    pub fn canfdcksel(&self) -> CanfdckselR {
        CanfdckselR::new(self.bits & 7)
    }
    ///Bit 6 - CANFD clock (CANFDCLK) Switching Request
    #[inline(always)]
    pub fn canfdcksreq(&self) -> CanfdcksreqR {
        CanfdcksreqR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - CANFD clock (CANFDCLK) Switching Ready state flag
    #[inline(always)]
    pub fn canfdcksrdy(&self) -> CanfdcksrdyR {
        CanfdcksrdyR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CANFDCKCR")
            .field("canfdcksel", &self.canfdcksel())
            .field("canfdcksreq", &self.canfdcksreq())
            .field("canfdcksrdy", &self.canfdcksrdy())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - CANFD clock (CANFDCLK) Source Select
    #[inline(always)]
    pub fn canfdcksel(&mut self) -> CanfdckselW<CanfdckcrSpec> {
        CanfdckselW::new(self, 0)
    }
    ///Bit 6 - CANFD clock (CANFDCLK) Switching Request
    #[inline(always)]
    pub fn canfdcksreq(&mut self) -> CanfdcksreqW<CanfdckcrSpec> {
        CanfdcksreqW::new(self, 6)
    }
}
/**CANFD Clock Control Register

You can [`read`](crate::Reg::read) this register and get [`canfdckcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`canfdckcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CanfdckcrSpec;
impl crate::RegisterSpec for CanfdckcrSpec {
    type Ux = u8;
}
///`read()` method returns [`canfdckcr::R`](R) reader structure
impl crate::Readable for CanfdckcrSpec {}
///`write(|w| ..)` method takes [`canfdckcr::W`](W) writer structure
impl crate::Writable for CanfdckcrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CANFDCKCR to value 0x01
impl crate::Resettable for CanfdckcrSpec {
    const RESET_VALUE: u8 = 0x01;
}
