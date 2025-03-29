///Register `GTPSR` reader
pub type R = crate::R<GtpsrSpec>;
///Register `GTPSR` writer
pub type W = crate::W<GtpsrSpec>;
/**GTETRGA Pin Rising Input Source Counter Stop Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psgtrgar {
    ///0: Counter stop disabled on the rising edge of GTETRGA input
    _0 = 0,
    ///1: Counter stop enabled on the rising edge of GTETRGA input
    _1 = 1,
}
impl From<Psgtrgar> for bool {
    #[inline(always)]
    fn from(variant: Psgtrgar) -> Self {
        variant as u8 != 0
    }
}
///Field `PSGTRGAR` reader - GTETRGA Pin Rising Input Source Counter Stop Enable
pub type PsgtrgarR = crate::BitReader<Psgtrgar>;
impl PsgtrgarR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Psgtrgar {
        match self.bits {
            false => Psgtrgar::_0,
            true => Psgtrgar::_1,
        }
    }
    ///Counter stop disabled on the rising edge of GTETRGA input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psgtrgar::_0
    }
    ///Counter stop enabled on the rising edge of GTETRGA input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psgtrgar::_1
    }
}
///Field `PSGTRGAR` writer - GTETRGA Pin Rising Input Source Counter Stop Enable
pub type PsgtrgarW<'a, REG> = crate::BitWriter<'a, REG, Psgtrgar>;
impl<'a, REG> PsgtrgarW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter stop disabled on the rising edge of GTETRGA input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psgtrgar::_0)
    }
    ///Counter stop enabled on the rising edge of GTETRGA input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psgtrgar::_1)
    }
}
/**GTETRGA Pin Falling Input Source Counter Stop Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psgtrgaf {
    ///0: Counter stop disabled on the falling edge of GTETRGA input
    _0 = 0,
    ///1: Counter stop enabled on the falling edge of GTETRGA input
    _1 = 1,
}
impl From<Psgtrgaf> for bool {
    #[inline(always)]
    fn from(variant: Psgtrgaf) -> Self {
        variant as u8 != 0
    }
}
///Field `PSGTRGAF` reader - GTETRGA Pin Falling Input Source Counter Stop Enable
pub type PsgtrgafR = crate::BitReader<Psgtrgaf>;
impl PsgtrgafR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Psgtrgaf {
        match self.bits {
            false => Psgtrgaf::_0,
            true => Psgtrgaf::_1,
        }
    }
    ///Counter stop disabled on the falling edge of GTETRGA input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psgtrgaf::_0
    }
    ///Counter stop enabled on the falling edge of GTETRGA input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psgtrgaf::_1
    }
}
///Field `PSGTRGAF` writer - GTETRGA Pin Falling Input Source Counter Stop Enable
pub type PsgtrgafW<'a, REG> = crate::BitWriter<'a, REG, Psgtrgaf>;
impl<'a, REG> PsgtrgafW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter stop disabled on the falling edge of GTETRGA input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psgtrgaf::_0)
    }
    ///Counter stop enabled on the falling edge of GTETRGA input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psgtrgaf::_1)
    }
}
/**GTETRGB Pin Rising Input Source Counter Stop Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psgtrgbr {
    ///0: Counter stop disabled on the rising edge of GTETRGB input
    _0 = 0,
    ///1: Counter stop enabled on the rising edge of GTETRGB input
    _1 = 1,
}
impl From<Psgtrgbr> for bool {
    #[inline(always)]
    fn from(variant: Psgtrgbr) -> Self {
        variant as u8 != 0
    }
}
///Field `PSGTRGBR` reader - GTETRGB Pin Rising Input Source Counter Stop Enable
pub type PsgtrgbrR = crate::BitReader<Psgtrgbr>;
impl PsgtrgbrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Psgtrgbr {
        match self.bits {
            false => Psgtrgbr::_0,
            true => Psgtrgbr::_1,
        }
    }
    ///Counter stop disabled on the rising edge of GTETRGB input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psgtrgbr::_0
    }
    ///Counter stop enabled on the rising edge of GTETRGB input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psgtrgbr::_1
    }
}
///Field `PSGTRGBR` writer - GTETRGB Pin Rising Input Source Counter Stop Enable
pub type PsgtrgbrW<'a, REG> = crate::BitWriter<'a, REG, Psgtrgbr>;
impl<'a, REG> PsgtrgbrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter stop disabled on the rising edge of GTETRGB input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psgtrgbr::_0)
    }
    ///Counter stop enabled on the rising edge of GTETRGB input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psgtrgbr::_1)
    }
}
/**GTETRGB Pin Falling Input Source Counter Stop Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psgtrgbf {
    ///0: Counter stop disabled on the falling edge of GTETRGB input
    _0 = 0,
    ///1: Counter stop enabled on the falling edge of GTETRGB input
    _1 = 1,
}
impl From<Psgtrgbf> for bool {
    #[inline(always)]
    fn from(variant: Psgtrgbf) -> Self {
        variant as u8 != 0
    }
}
///Field `PSGTRGBF` reader - GTETRGB Pin Falling Input Source Counter Stop Enable
pub type PsgtrgbfR = crate::BitReader<Psgtrgbf>;
impl PsgtrgbfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Psgtrgbf {
        match self.bits {
            false => Psgtrgbf::_0,
            true => Psgtrgbf::_1,
        }
    }
    ///Counter stop disabled on the falling edge of GTETRGB input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psgtrgbf::_0
    }
    ///Counter stop enabled on the falling edge of GTETRGB input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psgtrgbf::_1
    }
}
///Field `PSGTRGBF` writer - GTETRGB Pin Falling Input Source Counter Stop Enable
pub type PsgtrgbfW<'a, REG> = crate::BitWriter<'a, REG, Psgtrgbf>;
impl<'a, REG> PsgtrgbfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter stop disabled on the falling edge of GTETRGB input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psgtrgbf::_0)
    }
    ///Counter stop enabled on the falling edge of GTETRGB input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psgtrgbf::_1)
    }
}
/**GTETRGC Pin Rising Input Source Counter Stop Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psgtrgcr {
    ///0: Counter stop disabled on the rising edge of GTETRGC input
    _0 = 0,
    ///1: Counter stop enabled on the rising edge of GTETRGC input
    _1 = 1,
}
impl From<Psgtrgcr> for bool {
    #[inline(always)]
    fn from(variant: Psgtrgcr) -> Self {
        variant as u8 != 0
    }
}
///Field `PSGTRGCR` reader - GTETRGC Pin Rising Input Source Counter Stop Enable
pub type PsgtrgcrR = crate::BitReader<Psgtrgcr>;
impl PsgtrgcrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Psgtrgcr {
        match self.bits {
            false => Psgtrgcr::_0,
            true => Psgtrgcr::_1,
        }
    }
    ///Counter stop disabled on the rising edge of GTETRGC input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psgtrgcr::_0
    }
    ///Counter stop enabled on the rising edge of GTETRGC input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psgtrgcr::_1
    }
}
///Field `PSGTRGCR` writer - GTETRGC Pin Rising Input Source Counter Stop Enable
pub type PsgtrgcrW<'a, REG> = crate::BitWriter<'a, REG, Psgtrgcr>;
impl<'a, REG> PsgtrgcrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter stop disabled on the rising edge of GTETRGC input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psgtrgcr::_0)
    }
    ///Counter stop enabled on the rising edge of GTETRGC input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psgtrgcr::_1)
    }
}
/**GTETRGC Pin Falling Input Source Counter Stop Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psgtrgcf {
    ///0: Counter stop disabled on the falling edge of GTETRGC input
    _0 = 0,
    ///1: Counter stop enabled on the falling edge of GTETRGC input
    _1 = 1,
}
impl From<Psgtrgcf> for bool {
    #[inline(always)]
    fn from(variant: Psgtrgcf) -> Self {
        variant as u8 != 0
    }
}
///Field `PSGTRGCF` reader - GTETRGC Pin Falling Input Source Counter Stop Enable
pub type PsgtrgcfR = crate::BitReader<Psgtrgcf>;
impl PsgtrgcfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Psgtrgcf {
        match self.bits {
            false => Psgtrgcf::_0,
            true => Psgtrgcf::_1,
        }
    }
    ///Counter stop disabled on the falling edge of GTETRGC input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psgtrgcf::_0
    }
    ///Counter stop enabled on the falling edge of GTETRGC input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psgtrgcf::_1
    }
}
///Field `PSGTRGCF` writer - GTETRGC Pin Falling Input Source Counter Stop Enable
pub type PsgtrgcfW<'a, REG> = crate::BitWriter<'a, REG, Psgtrgcf>;
impl<'a, REG> PsgtrgcfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter stop disabled on the falling edge of GTETRGC input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psgtrgcf::_0)
    }
    ///Counter stop enabled on the falling edge of GTETRGC input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psgtrgcf::_1)
    }
}
/**GTETRGD Pin Rising Input Source Counter Stop Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psgtrgdr {
    ///0: Counter stop disabled on the rising edge of GTETRGD input
    _0 = 0,
    ///1: Counter stop enabled on the rising edge of GTETRGD input
    _1 = 1,
}
impl From<Psgtrgdr> for bool {
    #[inline(always)]
    fn from(variant: Psgtrgdr) -> Self {
        variant as u8 != 0
    }
}
///Field `PSGTRGDR` reader - GTETRGD Pin Rising Input Source Counter Stop Enable
pub type PsgtrgdrR = crate::BitReader<Psgtrgdr>;
impl PsgtrgdrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Psgtrgdr {
        match self.bits {
            false => Psgtrgdr::_0,
            true => Psgtrgdr::_1,
        }
    }
    ///Counter stop disabled on the rising edge of GTETRGD input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psgtrgdr::_0
    }
    ///Counter stop enabled on the rising edge of GTETRGD input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psgtrgdr::_1
    }
}
///Field `PSGTRGDR` writer - GTETRGD Pin Rising Input Source Counter Stop Enable
pub type PsgtrgdrW<'a, REG> = crate::BitWriter<'a, REG, Psgtrgdr>;
impl<'a, REG> PsgtrgdrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter stop disabled on the rising edge of GTETRGD input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psgtrgdr::_0)
    }
    ///Counter stop enabled on the rising edge of GTETRGD input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psgtrgdr::_1)
    }
}
/**GTETRGD Pin Falling Input Source Counter Stop Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psgtrgdf {
    ///0: Counter stop disabled on the falling edge of GTETRGD input
    _0 = 0,
    ///1: Counter stop enabled on the falling edge of GTETRGD input
    _1 = 1,
}
impl From<Psgtrgdf> for bool {
    #[inline(always)]
    fn from(variant: Psgtrgdf) -> Self {
        variant as u8 != 0
    }
}
///Field `PSGTRGDF` reader - GTETRGD Pin Falling Input Source Counter Stop Enable
pub type PsgtrgdfR = crate::BitReader<Psgtrgdf>;
impl PsgtrgdfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Psgtrgdf {
        match self.bits {
            false => Psgtrgdf::_0,
            true => Psgtrgdf::_1,
        }
    }
    ///Counter stop disabled on the falling edge of GTETRGD input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psgtrgdf::_0
    }
    ///Counter stop enabled on the falling edge of GTETRGD input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psgtrgdf::_1
    }
}
///Field `PSGTRGDF` writer - GTETRGD Pin Falling Input Source Counter Stop Enable
pub type PsgtrgdfW<'a, REG> = crate::BitWriter<'a, REG, Psgtrgdf>;
impl<'a, REG> PsgtrgdfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter stop disabled on the falling edge of GTETRGD input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psgtrgdf::_0)
    }
    ///Counter stop enabled on the falling edge of GTETRGD input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psgtrgdf::_1)
    }
}
/**GTIOCnA Pin Rising Input during GTIOCnB Value Low Source Counter Stop Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pscarbl {
    ///0: Counter stop disabled on the rising edge of GTIOCnA input when GTIOCnB input is 0
    _0 = 0,
    ///1: Counter stop enabled on the rising edge of GTIOCnA input when GTIOCnB input is 0
    _1 = 1,
}
impl From<Pscarbl> for bool {
    #[inline(always)]
    fn from(variant: Pscarbl) -> Self {
        variant as u8 != 0
    }
}
///Field `PSCARBL` reader - GTIOCnA Pin Rising Input during GTIOCnB Value Low Source Counter Stop Enable
pub type PscarblR = crate::BitReader<Pscarbl>;
impl PscarblR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pscarbl {
        match self.bits {
            false => Pscarbl::_0,
            true => Pscarbl::_1,
        }
    }
    ///Counter stop disabled on the rising edge of GTIOCnA input when GTIOCnB input is 0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pscarbl::_0
    }
    ///Counter stop enabled on the rising edge of GTIOCnA input when GTIOCnB input is 0
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pscarbl::_1
    }
}
///Field `PSCARBL` writer - GTIOCnA Pin Rising Input during GTIOCnB Value Low Source Counter Stop Enable
pub type PscarblW<'a, REG> = crate::BitWriter<'a, REG, Pscarbl>;
impl<'a, REG> PscarblW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter stop disabled on the rising edge of GTIOCnA input when GTIOCnB input is 0
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pscarbl::_0)
    }
    ///Counter stop enabled on the rising edge of GTIOCnA input when GTIOCnB input is 0
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pscarbl::_1)
    }
}
/**GTIOCnA Pin Rising Input during GTIOCnB Value High Source Counter Stop Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pscarbh {
    ///0: Counter stop disabled on the rising edge of GTIOCnA input when GTIOCnB input is 1
    _0 = 0,
    ///1: Counter stop enabled on the rising edge of GTIOCnA input when GTIOCnB input is 1
    _1 = 1,
}
impl From<Pscarbh> for bool {
    #[inline(always)]
    fn from(variant: Pscarbh) -> Self {
        variant as u8 != 0
    }
}
///Field `PSCARBH` reader - GTIOCnA Pin Rising Input during GTIOCnB Value High Source Counter Stop Enable
pub type PscarbhR = crate::BitReader<Pscarbh>;
impl PscarbhR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pscarbh {
        match self.bits {
            false => Pscarbh::_0,
            true => Pscarbh::_1,
        }
    }
    ///Counter stop disabled on the rising edge of GTIOCnA input when GTIOCnB input is 1
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pscarbh::_0
    }
    ///Counter stop enabled on the rising edge of GTIOCnA input when GTIOCnB input is 1
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pscarbh::_1
    }
}
///Field `PSCARBH` writer - GTIOCnA Pin Rising Input during GTIOCnB Value High Source Counter Stop Enable
pub type PscarbhW<'a, REG> = crate::BitWriter<'a, REG, Pscarbh>;
impl<'a, REG> PscarbhW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter stop disabled on the rising edge of GTIOCnA input when GTIOCnB input is 1
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pscarbh::_0)
    }
    ///Counter stop enabled on the rising edge of GTIOCnA input when GTIOCnB input is 1
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pscarbh::_1)
    }
}
/**GTIOCnA Pin Falling Input during GTIOCnB Value Low Source Counter Stop Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pscafbl {
    ///0: Counter stop disabled on the falling edge of GTIOCnA input when GTIOCnB input is 0
    _0 = 0,
    ///1: Counter stop enabled on the falling edge of GTIOCnA input when GTIOCnB input is 0
    _1 = 1,
}
impl From<Pscafbl> for bool {
    #[inline(always)]
    fn from(variant: Pscafbl) -> Self {
        variant as u8 != 0
    }
}
///Field `PSCAFBL` reader - GTIOCnA Pin Falling Input during GTIOCnB Value Low Source Counter Stop Enable
pub type PscafblR = crate::BitReader<Pscafbl>;
impl PscafblR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pscafbl {
        match self.bits {
            false => Pscafbl::_0,
            true => Pscafbl::_1,
        }
    }
    ///Counter stop disabled on the falling edge of GTIOCnA input when GTIOCnB input is 0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pscafbl::_0
    }
    ///Counter stop enabled on the falling edge of GTIOCnA input when GTIOCnB input is 0
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pscafbl::_1
    }
}
///Field `PSCAFBL` writer - GTIOCnA Pin Falling Input during GTIOCnB Value Low Source Counter Stop Enable
pub type PscafblW<'a, REG> = crate::BitWriter<'a, REG, Pscafbl>;
impl<'a, REG> PscafblW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter stop disabled on the falling edge of GTIOCnA input when GTIOCnB input is 0
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pscafbl::_0)
    }
    ///Counter stop enabled on the falling edge of GTIOCnA input when GTIOCnB input is 0
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pscafbl::_1)
    }
}
/**GTIOCnA Pin Falling Input during GTIOCnB Value High Source Counter Stop Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pscafbh {
    ///0: Counter stop disabled on the falling edge of GTIOCnA input when GTIOCnB input is 1
    _0 = 0,
    ///1: Counter stop enabled on the falling edge of GTIOCnA input when GTIOCnB input is 1
    _1 = 1,
}
impl From<Pscafbh> for bool {
    #[inline(always)]
    fn from(variant: Pscafbh) -> Self {
        variant as u8 != 0
    }
}
///Field `PSCAFBH` reader - GTIOCnA Pin Falling Input during GTIOCnB Value High Source Counter Stop Enable
pub type PscafbhR = crate::BitReader<Pscafbh>;
impl PscafbhR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pscafbh {
        match self.bits {
            false => Pscafbh::_0,
            true => Pscafbh::_1,
        }
    }
    ///Counter stop disabled on the falling edge of GTIOCnA input when GTIOCnB input is 1
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pscafbh::_0
    }
    ///Counter stop enabled on the falling edge of GTIOCnA input when GTIOCnB input is 1
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pscafbh::_1
    }
}
///Field `PSCAFBH` writer - GTIOCnA Pin Falling Input during GTIOCnB Value High Source Counter Stop Enable
pub type PscafbhW<'a, REG> = crate::BitWriter<'a, REG, Pscafbh>;
impl<'a, REG> PscafbhW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter stop disabled on the falling edge of GTIOCnA input when GTIOCnB input is 1
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pscafbh::_0)
    }
    ///Counter stop enabled on the falling edge of GTIOCnA input when GTIOCnB input is 1
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pscafbh::_1)
    }
}
/**GTIOCnB Pin Rising Input during GTIOCnA Value Low Source Counter Stop Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pscbral {
    ///0: Counter stop disabled on the rising edge of GTIOCnB input when GTIOCnA input is 0
    _0 = 0,
    ///1: Counter stop enabled on the rising edge of GTIOCnB input when GTIOCnA input is 0
    _1 = 1,
}
impl From<Pscbral> for bool {
    #[inline(always)]
    fn from(variant: Pscbral) -> Self {
        variant as u8 != 0
    }
}
///Field `PSCBRAL` reader - GTIOCnB Pin Rising Input during GTIOCnA Value Low Source Counter Stop Enable
pub type PscbralR = crate::BitReader<Pscbral>;
impl PscbralR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pscbral {
        match self.bits {
            false => Pscbral::_0,
            true => Pscbral::_1,
        }
    }
    ///Counter stop disabled on the rising edge of GTIOCnB input when GTIOCnA input is 0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pscbral::_0
    }
    ///Counter stop enabled on the rising edge of GTIOCnB input when GTIOCnA input is 0
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pscbral::_1
    }
}
///Field `PSCBRAL` writer - GTIOCnB Pin Rising Input during GTIOCnA Value Low Source Counter Stop Enable
pub type PscbralW<'a, REG> = crate::BitWriter<'a, REG, Pscbral>;
impl<'a, REG> PscbralW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter stop disabled on the rising edge of GTIOCnB input when GTIOCnA input is 0
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pscbral::_0)
    }
    ///Counter stop enabled on the rising edge of GTIOCnB input when GTIOCnA input is 0
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pscbral::_1)
    }
}
/**GTIOCnB Pin Rising Input during GTIOCnA Value High Source Counter Stop Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pscbrah {
    ///0: Counter stop disabled on the rising edge of GTIOCnB input when GTIOCnA input is 1
    _0 = 0,
    ///1: Counter stop enabled on the rising edge of GTIOCnB input when GTIOCnA input is 1
    _1 = 1,
}
impl From<Pscbrah> for bool {
    #[inline(always)]
    fn from(variant: Pscbrah) -> Self {
        variant as u8 != 0
    }
}
///Field `PSCBRAH` reader - GTIOCnB Pin Rising Input during GTIOCnA Value High Source Counter Stop Enable
pub type PscbrahR = crate::BitReader<Pscbrah>;
impl PscbrahR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pscbrah {
        match self.bits {
            false => Pscbrah::_0,
            true => Pscbrah::_1,
        }
    }
    ///Counter stop disabled on the rising edge of GTIOCnB input when GTIOCnA input is 1
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pscbrah::_0
    }
    ///Counter stop enabled on the rising edge of GTIOCnB input when GTIOCnA input is 1
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pscbrah::_1
    }
}
///Field `PSCBRAH` writer - GTIOCnB Pin Rising Input during GTIOCnA Value High Source Counter Stop Enable
pub type PscbrahW<'a, REG> = crate::BitWriter<'a, REG, Pscbrah>;
impl<'a, REG> PscbrahW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter stop disabled on the rising edge of GTIOCnB input when GTIOCnA input is 1
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pscbrah::_0)
    }
    ///Counter stop enabled on the rising edge of GTIOCnB input when GTIOCnA input is 1
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pscbrah::_1)
    }
}
/**GTIOCnB Pin Falling Input during GTIOCnA Value Low Source Counter Stop Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pscbfal {
    ///0: Counter stop disabled on the falling edge of GTIOCnB input when GTIOCnA input is 0
    _0 = 0,
    ///1: Counter stop enabled on the falling edge of GTIOCnB input when GTIOCnA input is 0
    _1 = 1,
}
impl From<Pscbfal> for bool {
    #[inline(always)]
    fn from(variant: Pscbfal) -> Self {
        variant as u8 != 0
    }
}
///Field `PSCBFAL` reader - GTIOCnB Pin Falling Input during GTIOCnA Value Low Source Counter Stop Enable
pub type PscbfalR = crate::BitReader<Pscbfal>;
impl PscbfalR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pscbfal {
        match self.bits {
            false => Pscbfal::_0,
            true => Pscbfal::_1,
        }
    }
    ///Counter stop disabled on the falling edge of GTIOCnB input when GTIOCnA input is 0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pscbfal::_0
    }
    ///Counter stop enabled on the falling edge of GTIOCnB input when GTIOCnA input is 0
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pscbfal::_1
    }
}
///Field `PSCBFAL` writer - GTIOCnB Pin Falling Input during GTIOCnA Value Low Source Counter Stop Enable
pub type PscbfalW<'a, REG> = crate::BitWriter<'a, REG, Pscbfal>;
impl<'a, REG> PscbfalW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter stop disabled on the falling edge of GTIOCnB input when GTIOCnA input is 0
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pscbfal::_0)
    }
    ///Counter stop enabled on the falling edge of GTIOCnB input when GTIOCnA input is 0
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pscbfal::_1)
    }
}
/**GTIOCnB Pin Falling Input during GTIOCnA Value High Source Counter Stop Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pscbfah {
    ///0: Counter stop disabled on the falling edge of GTIOCnB input when GTIOCnA input is 1
    _0 = 0,
    ///1: Counter stop enabled on the falling edge of GTIOCnB input when GTIOCnA input is 1
    _1 = 1,
}
impl From<Pscbfah> for bool {
    #[inline(always)]
    fn from(variant: Pscbfah) -> Self {
        variant as u8 != 0
    }
}
///Field `PSCBFAH` reader - GTIOCnB Pin Falling Input during GTIOCnA Value High Source Counter Stop Enable
pub type PscbfahR = crate::BitReader<Pscbfah>;
impl PscbfahR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pscbfah {
        match self.bits {
            false => Pscbfah::_0,
            true => Pscbfah::_1,
        }
    }
    ///Counter stop disabled on the falling edge of GTIOCnB input when GTIOCnA input is 1
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pscbfah::_0
    }
    ///Counter stop enabled on the falling edge of GTIOCnB input when GTIOCnA input is 1
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pscbfah::_1
    }
}
///Field `PSCBFAH` writer - GTIOCnB Pin Falling Input during GTIOCnA Value High Source Counter Stop Enable
pub type PscbfahW<'a, REG> = crate::BitWriter<'a, REG, Pscbfah>;
impl<'a, REG> PscbfahW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter stop disabled on the falling edge of GTIOCnB input when GTIOCnA input is 1
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pscbfah::_0)
    }
    ///Counter stop enabled on the falling edge of GTIOCnB input when GTIOCnA input is 1
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pscbfah::_1)
    }
}
/**ELC_GPTA Event Source Counter Stop Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pselca {
    ///0: Counter stop disabled at the ELC_GPTA input
    _0 = 0,
    ///1: Counter stop enabled at the ELC_GPTA input
    _1 = 1,
}
impl From<Pselca> for bool {
    #[inline(always)]
    fn from(variant: Pselca) -> Self {
        variant as u8 != 0
    }
}
///Field `PSELCA` reader - ELC_GPTA Event Source Counter Stop Enable
pub type PselcaR = crate::BitReader<Pselca>;
impl PselcaR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pselca {
        match self.bits {
            false => Pselca::_0,
            true => Pselca::_1,
        }
    }
    ///Counter stop disabled at the ELC_GPTA input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pselca::_0
    }
    ///Counter stop enabled at the ELC_GPTA input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pselca::_1
    }
}
///Field `PSELCA` writer - ELC_GPTA Event Source Counter Stop Enable
pub type PselcaW<'a, REG> = crate::BitWriter<'a, REG, Pselca>;
impl<'a, REG> PselcaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter stop disabled at the ELC_GPTA input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pselca::_0)
    }
    ///Counter stop enabled at the ELC_GPTA input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pselca::_1)
    }
}
/**ELC_GPTB Event Source Counter Stop Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pselcb {
    ///0: Counter stop disabled at the ELC_GPTB input
    _0 = 0,
    ///1: Counter stop enabled at the ELC_GPTB input
    _1 = 1,
}
impl From<Pselcb> for bool {
    #[inline(always)]
    fn from(variant: Pselcb) -> Self {
        variant as u8 != 0
    }
}
///Field `PSELCB` reader - ELC_GPTB Event Source Counter Stop Enable
pub type PselcbR = crate::BitReader<Pselcb>;
impl PselcbR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pselcb {
        match self.bits {
            false => Pselcb::_0,
            true => Pselcb::_1,
        }
    }
    ///Counter stop disabled at the ELC_GPTB input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pselcb::_0
    }
    ///Counter stop enabled at the ELC_GPTB input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pselcb::_1
    }
}
///Field `PSELCB` writer - ELC_GPTB Event Source Counter Stop Enable
pub type PselcbW<'a, REG> = crate::BitWriter<'a, REG, Pselcb>;
impl<'a, REG> PselcbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter stop disabled at the ELC_GPTB input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pselcb::_0)
    }
    ///Counter stop enabled at the ELC_GPTB input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pselcb::_1)
    }
}
/**ELC_GPTC Event Source Counter Stop Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pselcc {
    ///0: Counter stop disabled at the ELC_GPTC input
    _0 = 0,
    ///1: Counter stop enabled at the ELC_GPTC input
    _1 = 1,
}
impl From<Pselcc> for bool {
    #[inline(always)]
    fn from(variant: Pselcc) -> Self {
        variant as u8 != 0
    }
}
///Field `PSELCC` reader - ELC_GPTC Event Source Counter Stop Enable
pub type PselccR = crate::BitReader<Pselcc>;
impl PselccR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pselcc {
        match self.bits {
            false => Pselcc::_0,
            true => Pselcc::_1,
        }
    }
    ///Counter stop disabled at the ELC_GPTC input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pselcc::_0
    }
    ///Counter stop enabled at the ELC_GPTC input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pselcc::_1
    }
}
///Field `PSELCC` writer - ELC_GPTC Event Source Counter Stop Enable
pub type PselccW<'a, REG> = crate::BitWriter<'a, REG, Pselcc>;
impl<'a, REG> PselccW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter stop disabled at the ELC_GPTC input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pselcc::_0)
    }
    ///Counter stop enabled at the ELC_GPTC input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pselcc::_1)
    }
}
/**ELC_GPTD Event Source Counter Stop Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pselcd {
    ///0: Counter stop disabled at the ELC_GPTD input
    _0 = 0,
    ///1: Counter stop enabled at the ELC_GPTD input
    _1 = 1,
}
impl From<Pselcd> for bool {
    #[inline(always)]
    fn from(variant: Pselcd) -> Self {
        variant as u8 != 0
    }
}
///Field `PSELCD` reader - ELC_GPTD Event Source Counter Stop Enable
pub type PselcdR = crate::BitReader<Pselcd>;
impl PselcdR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pselcd {
        match self.bits {
            false => Pselcd::_0,
            true => Pselcd::_1,
        }
    }
    ///Counter stop disabled at the ELC_GPTD input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pselcd::_0
    }
    ///Counter stop enabled at the ELC_GPTD input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pselcd::_1
    }
}
///Field `PSELCD` writer - ELC_GPTD Event Source Counter Stop Enable
pub type PselcdW<'a, REG> = crate::BitWriter<'a, REG, Pselcd>;
impl<'a, REG> PselcdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter stop disabled at the ELC_GPTD input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pselcd::_0)
    }
    ///Counter stop enabled at the ELC_GPTD input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pselcd::_1)
    }
}
/**ELC_GPTE Event Source Counter Stop Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pselce {
    ///0: Counter stop disabled at the ELC_GPTE input
    _0 = 0,
    ///1: Counter stop enabled at the ELC_GPTE input
    _1 = 1,
}
impl From<Pselce> for bool {
    #[inline(always)]
    fn from(variant: Pselce) -> Self {
        variant as u8 != 0
    }
}
///Field `PSELCE` reader - ELC_GPTE Event Source Counter Stop Enable
pub type PselceR = crate::BitReader<Pselce>;
impl PselceR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pselce {
        match self.bits {
            false => Pselce::_0,
            true => Pselce::_1,
        }
    }
    ///Counter stop disabled at the ELC_GPTE input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pselce::_0
    }
    ///Counter stop enabled at the ELC_GPTE input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pselce::_1
    }
}
///Field `PSELCE` writer - ELC_GPTE Event Source Counter Stop Enable
pub type PselceW<'a, REG> = crate::BitWriter<'a, REG, Pselce>;
impl<'a, REG> PselceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter stop disabled at the ELC_GPTE input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pselce::_0)
    }
    ///Counter stop enabled at the ELC_GPTE input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pselce::_1)
    }
}
/**ELC_GPTF Event Source Counter Stop Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pselcf {
    ///0: Counter stop disabled at the ELC_GPTF input
    _0 = 0,
    ///1: Counter stop enabled at the ELC_GPTF input
    _1 = 1,
}
impl From<Pselcf> for bool {
    #[inline(always)]
    fn from(variant: Pselcf) -> Self {
        variant as u8 != 0
    }
}
///Field `PSELCF` reader - ELC_GPTF Event Source Counter Stop Enable
pub type PselcfR = crate::BitReader<Pselcf>;
impl PselcfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pselcf {
        match self.bits {
            false => Pselcf::_0,
            true => Pselcf::_1,
        }
    }
    ///Counter stop disabled at the ELC_GPTF input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pselcf::_0
    }
    ///Counter stop enabled at the ELC_GPTF input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pselcf::_1
    }
}
///Field `PSELCF` writer - ELC_GPTF Event Source Counter Stop Enable
pub type PselcfW<'a, REG> = crate::BitWriter<'a, REG, Pselcf>;
impl<'a, REG> PselcfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter stop disabled at the ELC_GPTF input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pselcf::_0)
    }
    ///Counter stop enabled at the ELC_GPTF input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pselcf::_1)
    }
}
/**ELC_GPTG Event Source Counter Stop Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pselcg {
    ///0: Counter stop disabled at the ELC_GPTG input
    _0 = 0,
    ///1: Counter stop enabled at the ELC_GPTG input
    _1 = 1,
}
impl From<Pselcg> for bool {
    #[inline(always)]
    fn from(variant: Pselcg) -> Self {
        variant as u8 != 0
    }
}
///Field `PSELCG` reader - ELC_GPTG Event Source Counter Stop Enable
pub type PselcgR = crate::BitReader<Pselcg>;
impl PselcgR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pselcg {
        match self.bits {
            false => Pselcg::_0,
            true => Pselcg::_1,
        }
    }
    ///Counter stop disabled at the ELC_GPTG input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pselcg::_0
    }
    ///Counter stop enabled at the ELC_GPTG input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pselcg::_1
    }
}
///Field `PSELCG` writer - ELC_GPTG Event Source Counter Stop Enable
pub type PselcgW<'a, REG> = crate::BitWriter<'a, REG, Pselcg>;
impl<'a, REG> PselcgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter stop disabled at the ELC_GPTG input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pselcg::_0)
    }
    ///Counter stop enabled at the ELC_GPTG input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pselcg::_1)
    }
}
/**ELC_GPTH Event Source Counter Stop Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pselch {
    ///0: Counter stop disabled at the ELC_GPTH input
    _0 = 0,
    ///1: Counter stop enabled at the ELC_GPTH input
    _1 = 1,
}
impl From<Pselch> for bool {
    #[inline(always)]
    fn from(variant: Pselch) -> Self {
        variant as u8 != 0
    }
}
///Field `PSELCH` reader - ELC_GPTH Event Source Counter Stop Enable
pub type PselchR = crate::BitReader<Pselch>;
impl PselchR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pselch {
        match self.bits {
            false => Pselch::_0,
            true => Pselch::_1,
        }
    }
    ///Counter stop disabled at the ELC_GPTH input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pselch::_0
    }
    ///Counter stop enabled at the ELC_GPTH input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pselch::_1
    }
}
///Field `PSELCH` writer - ELC_GPTH Event Source Counter Stop Enable
pub type PselchW<'a, REG> = crate::BitWriter<'a, REG, Pselch>;
impl<'a, REG> PselchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter stop disabled at the ELC_GPTH input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pselch::_0)
    }
    ///Counter stop enabled at the ELC_GPTH input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pselch::_1)
    }
}
/**Software Source Counter Stop Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cstop {
    ///0: Counter stop disabled by the GTSTP register
    _0 = 0,
    ///1: Counter stop enabled by the GTSTP register
    _1 = 1,
}
impl From<Cstop> for bool {
    #[inline(always)]
    fn from(variant: Cstop) -> Self {
        variant as u8 != 0
    }
}
///Field `CSTOP` reader - Software Source Counter Stop Enable
pub type CstopR = crate::BitReader<Cstop>;
impl CstopR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cstop {
        match self.bits {
            false => Cstop::_0,
            true => Cstop::_1,
        }
    }
    ///Counter stop disabled by the GTSTP register
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cstop::_0
    }
    ///Counter stop enabled by the GTSTP register
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cstop::_1
    }
}
///Field `CSTOP` writer - Software Source Counter Stop Enable
pub type CstopW<'a, REG> = crate::BitWriter<'a, REG, Cstop>;
impl<'a, REG> CstopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter stop disabled by the GTSTP register
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cstop::_0)
    }
    ///Counter stop enabled by the GTSTP register
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cstop::_1)
    }
}
impl R {
    ///Bit 0 - GTETRGA Pin Rising Input Source Counter Stop Enable
    #[inline(always)]
    pub fn psgtrgar(&self) -> PsgtrgarR {
        PsgtrgarR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - GTETRGA Pin Falling Input Source Counter Stop Enable
    #[inline(always)]
    pub fn psgtrgaf(&self) -> PsgtrgafR {
        PsgtrgafR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - GTETRGB Pin Rising Input Source Counter Stop Enable
    #[inline(always)]
    pub fn psgtrgbr(&self) -> PsgtrgbrR {
        PsgtrgbrR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - GTETRGB Pin Falling Input Source Counter Stop Enable
    #[inline(always)]
    pub fn psgtrgbf(&self) -> PsgtrgbfR {
        PsgtrgbfR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - GTETRGC Pin Rising Input Source Counter Stop Enable
    #[inline(always)]
    pub fn psgtrgcr(&self) -> PsgtrgcrR {
        PsgtrgcrR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - GTETRGC Pin Falling Input Source Counter Stop Enable
    #[inline(always)]
    pub fn psgtrgcf(&self) -> PsgtrgcfR {
        PsgtrgcfR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - GTETRGD Pin Rising Input Source Counter Stop Enable
    #[inline(always)]
    pub fn psgtrgdr(&self) -> PsgtrgdrR {
        PsgtrgdrR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - GTETRGD Pin Falling Input Source Counter Stop Enable
    #[inline(always)]
    pub fn psgtrgdf(&self) -> PsgtrgdfR {
        PsgtrgdfR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - GTIOCnA Pin Rising Input during GTIOCnB Value Low Source Counter Stop Enable
    #[inline(always)]
    pub fn pscarbl(&self) -> PscarblR {
        PscarblR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - GTIOCnA Pin Rising Input during GTIOCnB Value High Source Counter Stop Enable
    #[inline(always)]
    pub fn pscarbh(&self) -> PscarbhR {
        PscarbhR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - GTIOCnA Pin Falling Input during GTIOCnB Value Low Source Counter Stop Enable
    #[inline(always)]
    pub fn pscafbl(&self) -> PscafblR {
        PscafblR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - GTIOCnA Pin Falling Input during GTIOCnB Value High Source Counter Stop Enable
    #[inline(always)]
    pub fn pscafbh(&self) -> PscafbhR {
        PscafbhR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - GTIOCnB Pin Rising Input during GTIOCnA Value Low Source Counter Stop Enable
    #[inline(always)]
    pub fn pscbral(&self) -> PscbralR {
        PscbralR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - GTIOCnB Pin Rising Input during GTIOCnA Value High Source Counter Stop Enable
    #[inline(always)]
    pub fn pscbrah(&self) -> PscbrahR {
        PscbrahR::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - GTIOCnB Pin Falling Input during GTIOCnA Value Low Source Counter Stop Enable
    #[inline(always)]
    pub fn pscbfal(&self) -> PscbfalR {
        PscbfalR::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - GTIOCnB Pin Falling Input during GTIOCnA Value High Source Counter Stop Enable
    #[inline(always)]
    pub fn pscbfah(&self) -> PscbfahR {
        PscbfahR::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - ELC_GPTA Event Source Counter Stop Enable
    #[inline(always)]
    pub fn pselca(&self) -> PselcaR {
        PselcaR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - ELC_GPTB Event Source Counter Stop Enable
    #[inline(always)]
    pub fn pselcb(&self) -> PselcbR {
        PselcbR::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - ELC_GPTC Event Source Counter Stop Enable
    #[inline(always)]
    pub fn pselcc(&self) -> PselccR {
        PselccR::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - ELC_GPTD Event Source Counter Stop Enable
    #[inline(always)]
    pub fn pselcd(&self) -> PselcdR {
        PselcdR::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - ELC_GPTE Event Source Counter Stop Enable
    #[inline(always)]
    pub fn pselce(&self) -> PselceR {
        PselceR::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - ELC_GPTF Event Source Counter Stop Enable
    #[inline(always)]
    pub fn pselcf(&self) -> PselcfR {
        PselcfR::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - ELC_GPTG Event Source Counter Stop Enable
    #[inline(always)]
    pub fn pselcg(&self) -> PselcgR {
        PselcgR::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - ELC_GPTH Event Source Counter Stop Enable
    #[inline(always)]
    pub fn pselch(&self) -> PselchR {
        PselchR::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 31 - Software Source Counter Stop Enable
    #[inline(always)]
    pub fn cstop(&self) -> CstopR {
        CstopR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GTPSR")
            .field("psgtrgar", &self.psgtrgar())
            .field("psgtrgaf", &self.psgtrgaf())
            .field("psgtrgbr", &self.psgtrgbr())
            .field("psgtrgbf", &self.psgtrgbf())
            .field("psgtrgcr", &self.psgtrgcr())
            .field("psgtrgcf", &self.psgtrgcf())
            .field("psgtrgdr", &self.psgtrgdr())
            .field("psgtrgdf", &self.psgtrgdf())
            .field("pscarbl", &self.pscarbl())
            .field("pscarbh", &self.pscarbh())
            .field("pscafbl", &self.pscafbl())
            .field("pscafbh", &self.pscafbh())
            .field("pscbral", &self.pscbral())
            .field("pscbrah", &self.pscbrah())
            .field("pscbfal", &self.pscbfal())
            .field("pscbfah", &self.pscbfah())
            .field("pselca", &self.pselca())
            .field("pselcb", &self.pselcb())
            .field("pselcc", &self.pselcc())
            .field("pselcd", &self.pselcd())
            .field("pselce", &self.pselce())
            .field("pselcf", &self.pselcf())
            .field("pselcg", &self.pselcg())
            .field("pselch", &self.pselch())
            .field("cstop", &self.cstop())
            .finish()
    }
}
impl W {
    ///Bit 0 - GTETRGA Pin Rising Input Source Counter Stop Enable
    #[inline(always)]
    pub fn psgtrgar(&mut self) -> PsgtrgarW<GtpsrSpec> {
        PsgtrgarW::new(self, 0)
    }
    ///Bit 1 - GTETRGA Pin Falling Input Source Counter Stop Enable
    #[inline(always)]
    pub fn psgtrgaf(&mut self) -> PsgtrgafW<GtpsrSpec> {
        PsgtrgafW::new(self, 1)
    }
    ///Bit 2 - GTETRGB Pin Rising Input Source Counter Stop Enable
    #[inline(always)]
    pub fn psgtrgbr(&mut self) -> PsgtrgbrW<GtpsrSpec> {
        PsgtrgbrW::new(self, 2)
    }
    ///Bit 3 - GTETRGB Pin Falling Input Source Counter Stop Enable
    #[inline(always)]
    pub fn psgtrgbf(&mut self) -> PsgtrgbfW<GtpsrSpec> {
        PsgtrgbfW::new(self, 3)
    }
    ///Bit 4 - GTETRGC Pin Rising Input Source Counter Stop Enable
    #[inline(always)]
    pub fn psgtrgcr(&mut self) -> PsgtrgcrW<GtpsrSpec> {
        PsgtrgcrW::new(self, 4)
    }
    ///Bit 5 - GTETRGC Pin Falling Input Source Counter Stop Enable
    #[inline(always)]
    pub fn psgtrgcf(&mut self) -> PsgtrgcfW<GtpsrSpec> {
        PsgtrgcfW::new(self, 5)
    }
    ///Bit 6 - GTETRGD Pin Rising Input Source Counter Stop Enable
    #[inline(always)]
    pub fn psgtrgdr(&mut self) -> PsgtrgdrW<GtpsrSpec> {
        PsgtrgdrW::new(self, 6)
    }
    ///Bit 7 - GTETRGD Pin Falling Input Source Counter Stop Enable
    #[inline(always)]
    pub fn psgtrgdf(&mut self) -> PsgtrgdfW<GtpsrSpec> {
        PsgtrgdfW::new(self, 7)
    }
    ///Bit 8 - GTIOCnA Pin Rising Input during GTIOCnB Value Low Source Counter Stop Enable
    #[inline(always)]
    pub fn pscarbl(&mut self) -> PscarblW<GtpsrSpec> {
        PscarblW::new(self, 8)
    }
    ///Bit 9 - GTIOCnA Pin Rising Input during GTIOCnB Value High Source Counter Stop Enable
    #[inline(always)]
    pub fn pscarbh(&mut self) -> PscarbhW<GtpsrSpec> {
        PscarbhW::new(self, 9)
    }
    ///Bit 10 - GTIOCnA Pin Falling Input during GTIOCnB Value Low Source Counter Stop Enable
    #[inline(always)]
    pub fn pscafbl(&mut self) -> PscafblW<GtpsrSpec> {
        PscafblW::new(self, 10)
    }
    ///Bit 11 - GTIOCnA Pin Falling Input during GTIOCnB Value High Source Counter Stop Enable
    #[inline(always)]
    pub fn pscafbh(&mut self) -> PscafbhW<GtpsrSpec> {
        PscafbhW::new(self, 11)
    }
    ///Bit 12 - GTIOCnB Pin Rising Input during GTIOCnA Value Low Source Counter Stop Enable
    #[inline(always)]
    pub fn pscbral(&mut self) -> PscbralW<GtpsrSpec> {
        PscbralW::new(self, 12)
    }
    ///Bit 13 - GTIOCnB Pin Rising Input during GTIOCnA Value High Source Counter Stop Enable
    #[inline(always)]
    pub fn pscbrah(&mut self) -> PscbrahW<GtpsrSpec> {
        PscbrahW::new(self, 13)
    }
    ///Bit 14 - GTIOCnB Pin Falling Input during GTIOCnA Value Low Source Counter Stop Enable
    #[inline(always)]
    pub fn pscbfal(&mut self) -> PscbfalW<GtpsrSpec> {
        PscbfalW::new(self, 14)
    }
    ///Bit 15 - GTIOCnB Pin Falling Input during GTIOCnA Value High Source Counter Stop Enable
    #[inline(always)]
    pub fn pscbfah(&mut self) -> PscbfahW<GtpsrSpec> {
        PscbfahW::new(self, 15)
    }
    ///Bit 16 - ELC_GPTA Event Source Counter Stop Enable
    #[inline(always)]
    pub fn pselca(&mut self) -> PselcaW<GtpsrSpec> {
        PselcaW::new(self, 16)
    }
    ///Bit 17 - ELC_GPTB Event Source Counter Stop Enable
    #[inline(always)]
    pub fn pselcb(&mut self) -> PselcbW<GtpsrSpec> {
        PselcbW::new(self, 17)
    }
    ///Bit 18 - ELC_GPTC Event Source Counter Stop Enable
    #[inline(always)]
    pub fn pselcc(&mut self) -> PselccW<GtpsrSpec> {
        PselccW::new(self, 18)
    }
    ///Bit 19 - ELC_GPTD Event Source Counter Stop Enable
    #[inline(always)]
    pub fn pselcd(&mut self) -> PselcdW<GtpsrSpec> {
        PselcdW::new(self, 19)
    }
    ///Bit 20 - ELC_GPTE Event Source Counter Stop Enable
    #[inline(always)]
    pub fn pselce(&mut self) -> PselceW<GtpsrSpec> {
        PselceW::new(self, 20)
    }
    ///Bit 21 - ELC_GPTF Event Source Counter Stop Enable
    #[inline(always)]
    pub fn pselcf(&mut self) -> PselcfW<GtpsrSpec> {
        PselcfW::new(self, 21)
    }
    ///Bit 22 - ELC_GPTG Event Source Counter Stop Enable
    #[inline(always)]
    pub fn pselcg(&mut self) -> PselcgW<GtpsrSpec> {
        PselcgW::new(self, 22)
    }
    ///Bit 23 - ELC_GPTH Event Source Counter Stop Enable
    #[inline(always)]
    pub fn pselch(&mut self) -> PselchW<GtpsrSpec> {
        PselchW::new(self, 23)
    }
    ///Bit 31 - Software Source Counter Stop Enable
    #[inline(always)]
    pub fn cstop(&mut self) -> CstopW<GtpsrSpec> {
        CstopW::new(self, 31)
    }
}
/**General PWM Timer Stop Source Select Register

You can [`read`](crate::Reg::read) this register and get [`gtpsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtpsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GtpsrSpec;
impl crate::RegisterSpec for GtpsrSpec {
    type Ux = u32;
}
///`read()` method returns [`gtpsr::R`](R) reader structure
impl crate::Readable for GtpsrSpec {}
///`write(|w| ..)` method takes [`gtpsr::W`](W) writer structure
impl crate::Writable for GtpsrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTPSR to value 0
impl crate::Resettable for GtpsrSpec {}
