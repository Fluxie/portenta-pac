///Register `GTUPSR` reader
pub type R = crate::R<GtupsrSpec>;
///Register `GTUPSR` writer
pub type W = crate::W<GtupsrSpec>;
/**GTETRGA Pin Rising Input Source Counter Count Up Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usgtrgar {
    ///0: Counter count up disabled on the rising edge of GTETRGA input
    _0 = 0,
    ///1: Counter count up enabled on the rising edge of GTETRGA input
    _1 = 1,
}
impl From<Usgtrgar> for bool {
    #[inline(always)]
    fn from(variant: Usgtrgar) -> Self {
        variant as u8 != 0
    }
}
///Field `USGTRGAR` reader - GTETRGA Pin Rising Input Source Counter Count Up Enable
pub type UsgtrgarR = crate::BitReader<Usgtrgar>;
impl UsgtrgarR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Usgtrgar {
        match self.bits {
            false => Usgtrgar::_0,
            true => Usgtrgar::_1,
        }
    }
    ///Counter count up disabled on the rising edge of GTETRGA input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Usgtrgar::_0
    }
    ///Counter count up enabled on the rising edge of GTETRGA input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Usgtrgar::_1
    }
}
///Field `USGTRGAR` writer - GTETRGA Pin Rising Input Source Counter Count Up Enable
pub type UsgtrgarW<'a, REG> = crate::BitWriter<'a, REG, Usgtrgar>;
impl<'a, REG> UsgtrgarW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter count up disabled on the rising edge of GTETRGA input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Usgtrgar::_0)
    }
    ///Counter count up enabled on the rising edge of GTETRGA input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Usgtrgar::_1)
    }
}
/**GTETRGA Pin Falling Input Source Counter Count Up Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usgtrgaf {
    ///0: Counter count up disabled on the falling edge of GTETRGA input
    _0 = 0,
    ///1: Counter count up enabled on the falling edge of GTETRGA input
    _1 = 1,
}
impl From<Usgtrgaf> for bool {
    #[inline(always)]
    fn from(variant: Usgtrgaf) -> Self {
        variant as u8 != 0
    }
}
///Field `USGTRGAF` reader - GTETRGA Pin Falling Input Source Counter Count Up Enable
pub type UsgtrgafR = crate::BitReader<Usgtrgaf>;
impl UsgtrgafR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Usgtrgaf {
        match self.bits {
            false => Usgtrgaf::_0,
            true => Usgtrgaf::_1,
        }
    }
    ///Counter count up disabled on the falling edge of GTETRGA input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Usgtrgaf::_0
    }
    ///Counter count up enabled on the falling edge of GTETRGA input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Usgtrgaf::_1
    }
}
///Field `USGTRGAF` writer - GTETRGA Pin Falling Input Source Counter Count Up Enable
pub type UsgtrgafW<'a, REG> = crate::BitWriter<'a, REG, Usgtrgaf>;
impl<'a, REG> UsgtrgafW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter count up disabled on the falling edge of GTETRGA input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Usgtrgaf::_0)
    }
    ///Counter count up enabled on the falling edge of GTETRGA input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Usgtrgaf::_1)
    }
}
/**GTETRGB Pin Rising Input Source Counter Count Up Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usgtrgbr {
    ///0: Counter count up disabled on the rising edge of GTETRGB input
    _0 = 0,
    ///1: Counter count up enabled on the rising edge of GTETRGB input
    _1 = 1,
}
impl From<Usgtrgbr> for bool {
    #[inline(always)]
    fn from(variant: Usgtrgbr) -> Self {
        variant as u8 != 0
    }
}
///Field `USGTRGBR` reader - GTETRGB Pin Rising Input Source Counter Count Up Enable
pub type UsgtrgbrR = crate::BitReader<Usgtrgbr>;
impl UsgtrgbrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Usgtrgbr {
        match self.bits {
            false => Usgtrgbr::_0,
            true => Usgtrgbr::_1,
        }
    }
    ///Counter count up disabled on the rising edge of GTETRGB input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Usgtrgbr::_0
    }
    ///Counter count up enabled on the rising edge of GTETRGB input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Usgtrgbr::_1
    }
}
///Field `USGTRGBR` writer - GTETRGB Pin Rising Input Source Counter Count Up Enable
pub type UsgtrgbrW<'a, REG> = crate::BitWriter<'a, REG, Usgtrgbr>;
impl<'a, REG> UsgtrgbrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter count up disabled on the rising edge of GTETRGB input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Usgtrgbr::_0)
    }
    ///Counter count up enabled on the rising edge of GTETRGB input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Usgtrgbr::_1)
    }
}
/**GTETRGB Pin Falling Input Source Counter Count Up Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usgtrgbf {
    ///0: Counter count up disabled on the falling edge of GTETRGB input
    _0 = 0,
    ///1: Counter count up enabled on the falling edge of GTETRGB input
    _1 = 1,
}
impl From<Usgtrgbf> for bool {
    #[inline(always)]
    fn from(variant: Usgtrgbf) -> Self {
        variant as u8 != 0
    }
}
///Field `USGTRGBF` reader - GTETRGB Pin Falling Input Source Counter Count Up Enable
pub type UsgtrgbfR = crate::BitReader<Usgtrgbf>;
impl UsgtrgbfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Usgtrgbf {
        match self.bits {
            false => Usgtrgbf::_0,
            true => Usgtrgbf::_1,
        }
    }
    ///Counter count up disabled on the falling edge of GTETRGB input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Usgtrgbf::_0
    }
    ///Counter count up enabled on the falling edge of GTETRGB input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Usgtrgbf::_1
    }
}
///Field `USGTRGBF` writer - GTETRGB Pin Falling Input Source Counter Count Up Enable
pub type UsgtrgbfW<'a, REG> = crate::BitWriter<'a, REG, Usgtrgbf>;
impl<'a, REG> UsgtrgbfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter count up disabled on the falling edge of GTETRGB input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Usgtrgbf::_0)
    }
    ///Counter count up enabled on the falling edge of GTETRGB input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Usgtrgbf::_1)
    }
}
/**GTETRGC Pin Rising Input Source Counter Count Up Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usgtrgcr {
    ///0: Counter count up disabled on the rising edge of GTETRGC input
    _0 = 0,
    ///1: Counter count up enabled on the rising edge of GTETRGC input
    _1 = 1,
}
impl From<Usgtrgcr> for bool {
    #[inline(always)]
    fn from(variant: Usgtrgcr) -> Self {
        variant as u8 != 0
    }
}
///Field `USGTRGCR` reader - GTETRGC Pin Rising Input Source Counter Count Up Enable
pub type UsgtrgcrR = crate::BitReader<Usgtrgcr>;
impl UsgtrgcrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Usgtrgcr {
        match self.bits {
            false => Usgtrgcr::_0,
            true => Usgtrgcr::_1,
        }
    }
    ///Counter count up disabled on the rising edge of GTETRGC input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Usgtrgcr::_0
    }
    ///Counter count up enabled on the rising edge of GTETRGC input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Usgtrgcr::_1
    }
}
///Field `USGTRGCR` writer - GTETRGC Pin Rising Input Source Counter Count Up Enable
pub type UsgtrgcrW<'a, REG> = crate::BitWriter<'a, REG, Usgtrgcr>;
impl<'a, REG> UsgtrgcrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter count up disabled on the rising edge of GTETRGC input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Usgtrgcr::_0)
    }
    ///Counter count up enabled on the rising edge of GTETRGC input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Usgtrgcr::_1)
    }
}
/**GTETRGC Pin Falling Input Source Counter Count Up Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usgtrgcf {
    ///0: Counter count up disabled on the falling edge of GTETRGC input
    _0 = 0,
    ///1: Counter count up enabled on the falling edge of GTETRGC input
    _1 = 1,
}
impl From<Usgtrgcf> for bool {
    #[inline(always)]
    fn from(variant: Usgtrgcf) -> Self {
        variant as u8 != 0
    }
}
///Field `USGTRGCF` reader - GTETRGC Pin Falling Input Source Counter Count Up Enable
pub type UsgtrgcfR = crate::BitReader<Usgtrgcf>;
impl UsgtrgcfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Usgtrgcf {
        match self.bits {
            false => Usgtrgcf::_0,
            true => Usgtrgcf::_1,
        }
    }
    ///Counter count up disabled on the falling edge of GTETRGC input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Usgtrgcf::_0
    }
    ///Counter count up enabled on the falling edge of GTETRGC input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Usgtrgcf::_1
    }
}
///Field `USGTRGCF` writer - GTETRGC Pin Falling Input Source Counter Count Up Enable
pub type UsgtrgcfW<'a, REG> = crate::BitWriter<'a, REG, Usgtrgcf>;
impl<'a, REG> UsgtrgcfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter count up disabled on the falling edge of GTETRGC input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Usgtrgcf::_0)
    }
    ///Counter count up enabled on the falling edge of GTETRGC input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Usgtrgcf::_1)
    }
}
/**GTETRGD Pin Rising Input Source Counter Count Up Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usgtrgdr {
    ///0: Counter count up disabled on the rising edge of GTETRGD input
    _0 = 0,
    ///1: Counter count up enabled on the rising edge of GTETRGD input
    _1 = 1,
}
impl From<Usgtrgdr> for bool {
    #[inline(always)]
    fn from(variant: Usgtrgdr) -> Self {
        variant as u8 != 0
    }
}
///Field `USGTRGDR` reader - GTETRGD Pin Rising Input Source Counter Count Up Enable
pub type UsgtrgdrR = crate::BitReader<Usgtrgdr>;
impl UsgtrgdrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Usgtrgdr {
        match self.bits {
            false => Usgtrgdr::_0,
            true => Usgtrgdr::_1,
        }
    }
    ///Counter count up disabled on the rising edge of GTETRGD input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Usgtrgdr::_0
    }
    ///Counter count up enabled on the rising edge of GTETRGD input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Usgtrgdr::_1
    }
}
///Field `USGTRGDR` writer - GTETRGD Pin Rising Input Source Counter Count Up Enable
pub type UsgtrgdrW<'a, REG> = crate::BitWriter<'a, REG, Usgtrgdr>;
impl<'a, REG> UsgtrgdrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter count up disabled on the rising edge of GTETRGD input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Usgtrgdr::_0)
    }
    ///Counter count up enabled on the rising edge of GTETRGD input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Usgtrgdr::_1)
    }
}
/**GTETRGD Pin Falling Input Source Counter Count Up Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usgtrgdf {
    ///0: Counter count up disabled on the falling edge of GTETRGD input
    _0 = 0,
    ///1: Counter count up enabled on the falling edge of GTETRGD input
    _1 = 1,
}
impl From<Usgtrgdf> for bool {
    #[inline(always)]
    fn from(variant: Usgtrgdf) -> Self {
        variant as u8 != 0
    }
}
///Field `USGTRGDF` reader - GTETRGD Pin Falling Input Source Counter Count Up Enable
pub type UsgtrgdfR = crate::BitReader<Usgtrgdf>;
impl UsgtrgdfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Usgtrgdf {
        match self.bits {
            false => Usgtrgdf::_0,
            true => Usgtrgdf::_1,
        }
    }
    ///Counter count up disabled on the falling edge of GTETRGD input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Usgtrgdf::_0
    }
    ///Counter count up enabled on the falling edge of GTETRGD input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Usgtrgdf::_1
    }
}
///Field `USGTRGDF` writer - GTETRGD Pin Falling Input Source Counter Count Up Enable
pub type UsgtrgdfW<'a, REG> = crate::BitWriter<'a, REG, Usgtrgdf>;
impl<'a, REG> UsgtrgdfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter count up disabled on the falling edge of GTETRGD input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Usgtrgdf::_0)
    }
    ///Counter count up enabled on the falling edge of GTETRGD input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Usgtrgdf::_1)
    }
}
/**GTIOCnA Pin Rising Input during GTIOCnB Value Low Source Counter Count Up Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uscarbl {
    ///0: Counter count up disabled on the rising edge of GTIOCnA input when GTIOCnB input is 0
    _0 = 0,
    ///1: Counter count up enabled on the rising edge of GTIOCnA input when GTIOCnB input is 0
    _1 = 1,
}
impl From<Uscarbl> for bool {
    #[inline(always)]
    fn from(variant: Uscarbl) -> Self {
        variant as u8 != 0
    }
}
///Field `USCARBL` reader - GTIOCnA Pin Rising Input during GTIOCnB Value Low Source Counter Count Up Enable
pub type UscarblR = crate::BitReader<Uscarbl>;
impl UscarblR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Uscarbl {
        match self.bits {
            false => Uscarbl::_0,
            true => Uscarbl::_1,
        }
    }
    ///Counter count up disabled on the rising edge of GTIOCnA input when GTIOCnB input is 0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Uscarbl::_0
    }
    ///Counter count up enabled on the rising edge of GTIOCnA input when GTIOCnB input is 0
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Uscarbl::_1
    }
}
///Field `USCARBL` writer - GTIOCnA Pin Rising Input during GTIOCnB Value Low Source Counter Count Up Enable
pub type UscarblW<'a, REG> = crate::BitWriter<'a, REG, Uscarbl>;
impl<'a, REG> UscarblW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter count up disabled on the rising edge of GTIOCnA input when GTIOCnB input is 0
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Uscarbl::_0)
    }
    ///Counter count up enabled on the rising edge of GTIOCnA input when GTIOCnB input is 0
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Uscarbl::_1)
    }
}
/**GTIOCnA Pin Rising Input during GTIOCnB Value High Source Counter Count Up Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uscarbh {
    ///0: Counter count up disabled on the rising edge of GTIOCnA input when GTIOCnB input is 1
    _0 = 0,
    ///1: Counter count up enabled on the rising edge of GTIOCnA input when GTIOCnB input is 1
    _1 = 1,
}
impl From<Uscarbh> for bool {
    #[inline(always)]
    fn from(variant: Uscarbh) -> Self {
        variant as u8 != 0
    }
}
///Field `USCARBH` reader - GTIOCnA Pin Rising Input during GTIOCnB Value High Source Counter Count Up Enable
pub type UscarbhR = crate::BitReader<Uscarbh>;
impl UscarbhR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Uscarbh {
        match self.bits {
            false => Uscarbh::_0,
            true => Uscarbh::_1,
        }
    }
    ///Counter count up disabled on the rising edge of GTIOCnA input when GTIOCnB input is 1
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Uscarbh::_0
    }
    ///Counter count up enabled on the rising edge of GTIOCnA input when GTIOCnB input is 1
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Uscarbh::_1
    }
}
///Field `USCARBH` writer - GTIOCnA Pin Rising Input during GTIOCnB Value High Source Counter Count Up Enable
pub type UscarbhW<'a, REG> = crate::BitWriter<'a, REG, Uscarbh>;
impl<'a, REG> UscarbhW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter count up disabled on the rising edge of GTIOCnA input when GTIOCnB input is 1
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Uscarbh::_0)
    }
    ///Counter count up enabled on the rising edge of GTIOCnA input when GTIOCnB input is 1
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Uscarbh::_1)
    }
}
/**GTIOCnA Pin Falling Input during GTIOCnB Value Low Source Counter Count Up Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uscafbl {
    ///0: Counter count up disabled on the falling edge of GTIOCnA input when GTIOCnB input is 0
    _0 = 0,
    ///1: Counter count up enabled on the falling edge of GTIOCnA input when GTIOCnB input is 0
    _1 = 1,
}
impl From<Uscafbl> for bool {
    #[inline(always)]
    fn from(variant: Uscafbl) -> Self {
        variant as u8 != 0
    }
}
///Field `USCAFBL` reader - GTIOCnA Pin Falling Input during GTIOCnB Value Low Source Counter Count Up Enable
pub type UscafblR = crate::BitReader<Uscafbl>;
impl UscafblR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Uscafbl {
        match self.bits {
            false => Uscafbl::_0,
            true => Uscafbl::_1,
        }
    }
    ///Counter count up disabled on the falling edge of GTIOCnA input when GTIOCnB input is 0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Uscafbl::_0
    }
    ///Counter count up enabled on the falling edge of GTIOCnA input when GTIOCnB input is 0
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Uscafbl::_1
    }
}
///Field `USCAFBL` writer - GTIOCnA Pin Falling Input during GTIOCnB Value Low Source Counter Count Up Enable
pub type UscafblW<'a, REG> = crate::BitWriter<'a, REG, Uscafbl>;
impl<'a, REG> UscafblW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter count up disabled on the falling edge of GTIOCnA input when GTIOCnB input is 0
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Uscafbl::_0)
    }
    ///Counter count up enabled on the falling edge of GTIOCnA input when GTIOCnB input is 0
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Uscafbl::_1)
    }
}
/**GTIOCnA Pin Falling Input during GTIOCnB Value High Source Counter Count Up Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uscafbh {
    ///0: Counter count up disabled on the falling edge of GTIOCnA input when GTIOCnB input is 1
    _0 = 0,
    ///1: Counter count up enabled on the falling edge of GTIOCnA input when GTIOCnB input is 1
    _1 = 1,
}
impl From<Uscafbh> for bool {
    #[inline(always)]
    fn from(variant: Uscafbh) -> Self {
        variant as u8 != 0
    }
}
///Field `USCAFBH` reader - GTIOCnA Pin Falling Input during GTIOCnB Value High Source Counter Count Up Enable
pub type UscafbhR = crate::BitReader<Uscafbh>;
impl UscafbhR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Uscafbh {
        match self.bits {
            false => Uscafbh::_0,
            true => Uscafbh::_1,
        }
    }
    ///Counter count up disabled on the falling edge of GTIOCnA input when GTIOCnB input is 1
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Uscafbh::_0
    }
    ///Counter count up enabled on the falling edge of GTIOCnA input when GTIOCnB input is 1
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Uscafbh::_1
    }
}
///Field `USCAFBH` writer - GTIOCnA Pin Falling Input during GTIOCnB Value High Source Counter Count Up Enable
pub type UscafbhW<'a, REG> = crate::BitWriter<'a, REG, Uscafbh>;
impl<'a, REG> UscafbhW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter count up disabled on the falling edge of GTIOCnA input when GTIOCnB input is 1
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Uscafbh::_0)
    }
    ///Counter count up enabled on the falling edge of GTIOCnA input when GTIOCnB input is 1
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Uscafbh::_1)
    }
}
/**GTIOCnB Pin Rising Input during GTIOCnA Value Low Source Counter Count Up Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uscbral {
    ///0: Counter count up disabled on the rising edge of GTIOCnB input when GTIOCnA input is 0
    _0 = 0,
    ///1: Counter count up enabled on the rising edge of GTIOCnB input when GTIOCnA input is 0
    _1 = 1,
}
impl From<Uscbral> for bool {
    #[inline(always)]
    fn from(variant: Uscbral) -> Self {
        variant as u8 != 0
    }
}
///Field `USCBRAL` reader - GTIOCnB Pin Rising Input during GTIOCnA Value Low Source Counter Count Up Enable
pub type UscbralR = crate::BitReader<Uscbral>;
impl UscbralR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Uscbral {
        match self.bits {
            false => Uscbral::_0,
            true => Uscbral::_1,
        }
    }
    ///Counter count up disabled on the rising edge of GTIOCnB input when GTIOCnA input is 0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Uscbral::_0
    }
    ///Counter count up enabled on the rising edge of GTIOCnB input when GTIOCnA input is 0
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Uscbral::_1
    }
}
///Field `USCBRAL` writer - GTIOCnB Pin Rising Input during GTIOCnA Value Low Source Counter Count Up Enable
pub type UscbralW<'a, REG> = crate::BitWriter<'a, REG, Uscbral>;
impl<'a, REG> UscbralW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter count up disabled on the rising edge of GTIOCnB input when GTIOCnA input is 0
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Uscbral::_0)
    }
    ///Counter count up enabled on the rising edge of GTIOCnB input when GTIOCnA input is 0
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Uscbral::_1)
    }
}
/**GTIOCnB Pin Rising Input during GTIOCnA Value High Source Counter Count Up Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uscbrah {
    ///0: Counter count up disabled on the rising edge of GTIOCnB input when GTIOCnA input is 1
    _0 = 0,
    ///1: Counter count up enabled on the rising edge of GTIOCnB input when GTIOCnA input is 1
    _1 = 1,
}
impl From<Uscbrah> for bool {
    #[inline(always)]
    fn from(variant: Uscbrah) -> Self {
        variant as u8 != 0
    }
}
///Field `USCBRAH` reader - GTIOCnB Pin Rising Input during GTIOCnA Value High Source Counter Count Up Enable
pub type UscbrahR = crate::BitReader<Uscbrah>;
impl UscbrahR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Uscbrah {
        match self.bits {
            false => Uscbrah::_0,
            true => Uscbrah::_1,
        }
    }
    ///Counter count up disabled on the rising edge of GTIOCnB input when GTIOCnA input is 1
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Uscbrah::_0
    }
    ///Counter count up enabled on the rising edge of GTIOCnB input when GTIOCnA input is 1
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Uscbrah::_1
    }
}
///Field `USCBRAH` writer - GTIOCnB Pin Rising Input during GTIOCnA Value High Source Counter Count Up Enable
pub type UscbrahW<'a, REG> = crate::BitWriter<'a, REG, Uscbrah>;
impl<'a, REG> UscbrahW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter count up disabled on the rising edge of GTIOCnB input when GTIOCnA input is 1
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Uscbrah::_0)
    }
    ///Counter count up enabled on the rising edge of GTIOCnB input when GTIOCnA input is 1
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Uscbrah::_1)
    }
}
/**GTIOCnB Pin Falling Input during GTIOCnA Value Low Source Counter Count Up Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uscbfal {
    ///0: Counter count up disabled on the falling edge of GTIOCnB input when GTIOCnA input is 0
    _0 = 0,
    ///1: Counter count up enabled on the falling edge of GTIOCnB input when GTIOCnA input is 0
    _1 = 1,
}
impl From<Uscbfal> for bool {
    #[inline(always)]
    fn from(variant: Uscbfal) -> Self {
        variant as u8 != 0
    }
}
///Field `USCBFAL` reader - GTIOCnB Pin Falling Input during GTIOCnA Value Low Source Counter Count Up Enable
pub type UscbfalR = crate::BitReader<Uscbfal>;
impl UscbfalR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Uscbfal {
        match self.bits {
            false => Uscbfal::_0,
            true => Uscbfal::_1,
        }
    }
    ///Counter count up disabled on the falling edge of GTIOCnB input when GTIOCnA input is 0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Uscbfal::_0
    }
    ///Counter count up enabled on the falling edge of GTIOCnB input when GTIOCnA input is 0
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Uscbfal::_1
    }
}
///Field `USCBFAL` writer - GTIOCnB Pin Falling Input during GTIOCnA Value Low Source Counter Count Up Enable
pub type UscbfalW<'a, REG> = crate::BitWriter<'a, REG, Uscbfal>;
impl<'a, REG> UscbfalW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter count up disabled on the falling edge of GTIOCnB input when GTIOCnA input is 0
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Uscbfal::_0)
    }
    ///Counter count up enabled on the falling edge of GTIOCnB input when GTIOCnA input is 0
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Uscbfal::_1)
    }
}
/**GTIOCnB Pin Falling Input during GTIOCnA Value High Source Counter Count Up Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uscbfah {
    ///0: Counter count up disabled on the falling edge of GTIOCnB input when GTIOCnA input is 1
    _0 = 0,
    ///1: Counter count up enabled on the falling edge of GTIOCnB input when GTIOCnA input is 1
    _1 = 1,
}
impl From<Uscbfah> for bool {
    #[inline(always)]
    fn from(variant: Uscbfah) -> Self {
        variant as u8 != 0
    }
}
///Field `USCBFAH` reader - GTIOCnB Pin Falling Input during GTIOCnA Value High Source Counter Count Up Enable
pub type UscbfahR = crate::BitReader<Uscbfah>;
impl UscbfahR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Uscbfah {
        match self.bits {
            false => Uscbfah::_0,
            true => Uscbfah::_1,
        }
    }
    ///Counter count up disabled on the falling edge of GTIOCnB input when GTIOCnA input is 1
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Uscbfah::_0
    }
    ///Counter count up enabled on the falling edge of GTIOCnB input when GTIOCnA input is 1
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Uscbfah::_1
    }
}
///Field `USCBFAH` writer - GTIOCnB Pin Falling Input during GTIOCnA Value High Source Counter Count Up Enable
pub type UscbfahW<'a, REG> = crate::BitWriter<'a, REG, Uscbfah>;
impl<'a, REG> UscbfahW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter count up disabled on the falling edge of GTIOCnB input when GTIOCnA input is 1
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Uscbfah::_0)
    }
    ///Counter count up enabled on the falling edge of GTIOCnB input when GTIOCnA input is 1
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Uscbfah::_1)
    }
}
/**ELC_GPTA Event Source Counter Count Up Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uselca {
    ///0: Counter count up disabled at the ELC_GPTA input
    _0 = 0,
    ///1: Counter count up enabled at the ELC_GPTA input
    _1 = 1,
}
impl From<Uselca> for bool {
    #[inline(always)]
    fn from(variant: Uselca) -> Self {
        variant as u8 != 0
    }
}
///Field `USELCA` reader - ELC_GPTA Event Source Counter Count Up Enable
pub type UselcaR = crate::BitReader<Uselca>;
impl UselcaR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Uselca {
        match self.bits {
            false => Uselca::_0,
            true => Uselca::_1,
        }
    }
    ///Counter count up disabled at the ELC_GPTA input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Uselca::_0
    }
    ///Counter count up enabled at the ELC_GPTA input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Uselca::_1
    }
}
///Field `USELCA` writer - ELC_GPTA Event Source Counter Count Up Enable
pub type UselcaW<'a, REG> = crate::BitWriter<'a, REG, Uselca>;
impl<'a, REG> UselcaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter count up disabled at the ELC_GPTA input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Uselca::_0)
    }
    ///Counter count up enabled at the ELC_GPTA input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Uselca::_1)
    }
}
/**ELC_GPTB Event Source Counter Count Up Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uselcb {
    ///0: Counter count up disabled at the ELC_GPTB input
    _0 = 0,
    ///1: Counter count up enabled at the ELC_GPTB input
    _1 = 1,
}
impl From<Uselcb> for bool {
    #[inline(always)]
    fn from(variant: Uselcb) -> Self {
        variant as u8 != 0
    }
}
///Field `USELCB` reader - ELC_GPTB Event Source Counter Count Up Enable
pub type UselcbR = crate::BitReader<Uselcb>;
impl UselcbR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Uselcb {
        match self.bits {
            false => Uselcb::_0,
            true => Uselcb::_1,
        }
    }
    ///Counter count up disabled at the ELC_GPTB input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Uselcb::_0
    }
    ///Counter count up enabled at the ELC_GPTB input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Uselcb::_1
    }
}
///Field `USELCB` writer - ELC_GPTB Event Source Counter Count Up Enable
pub type UselcbW<'a, REG> = crate::BitWriter<'a, REG, Uselcb>;
impl<'a, REG> UselcbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter count up disabled at the ELC_GPTB input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Uselcb::_0)
    }
    ///Counter count up enabled at the ELC_GPTB input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Uselcb::_1)
    }
}
/**ELC_GPTC Event Source Counter Count Up Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uselcc {
    ///0: Counter count up disabled at the ELC_GPTC input
    _0 = 0,
    ///1: Counter count up enabled at the ELC_GPTC input
    _1 = 1,
}
impl From<Uselcc> for bool {
    #[inline(always)]
    fn from(variant: Uselcc) -> Self {
        variant as u8 != 0
    }
}
///Field `USELCC` reader - ELC_GPTC Event Source Counter Count Up Enable
pub type UselccR = crate::BitReader<Uselcc>;
impl UselccR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Uselcc {
        match self.bits {
            false => Uselcc::_0,
            true => Uselcc::_1,
        }
    }
    ///Counter count up disabled at the ELC_GPTC input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Uselcc::_0
    }
    ///Counter count up enabled at the ELC_GPTC input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Uselcc::_1
    }
}
///Field `USELCC` writer - ELC_GPTC Event Source Counter Count Up Enable
pub type UselccW<'a, REG> = crate::BitWriter<'a, REG, Uselcc>;
impl<'a, REG> UselccW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter count up disabled at the ELC_GPTC input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Uselcc::_0)
    }
    ///Counter count up enabled at the ELC_GPTC input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Uselcc::_1)
    }
}
/**ELC_GPTD Event Source Counter Count Up Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uselcd {
    ///0: Counter count up disabled at the ELC_GPTD input
    _0 = 0,
    ///1: Counter count up enabled at the ELC_GPTD input
    _1 = 1,
}
impl From<Uselcd> for bool {
    #[inline(always)]
    fn from(variant: Uselcd) -> Self {
        variant as u8 != 0
    }
}
///Field `USELCD` reader - ELC_GPTD Event Source Counter Count Up Enable
pub type UselcdR = crate::BitReader<Uselcd>;
impl UselcdR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Uselcd {
        match self.bits {
            false => Uselcd::_0,
            true => Uselcd::_1,
        }
    }
    ///Counter count up disabled at the ELC_GPTD input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Uselcd::_0
    }
    ///Counter count up enabled at the ELC_GPTD input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Uselcd::_1
    }
}
///Field `USELCD` writer - ELC_GPTD Event Source Counter Count Up Enable
pub type UselcdW<'a, REG> = crate::BitWriter<'a, REG, Uselcd>;
impl<'a, REG> UselcdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter count up disabled at the ELC_GPTD input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Uselcd::_0)
    }
    ///Counter count up enabled at the ELC_GPTD input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Uselcd::_1)
    }
}
/**ELC_GPTE Event Source Counter Count Up Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uselce {
    ///0: Counter count up disabled at the ELC_GPTE input
    _0 = 0,
    ///1: Counter count up enabled at the ELC_GPTE input
    _1 = 1,
}
impl From<Uselce> for bool {
    #[inline(always)]
    fn from(variant: Uselce) -> Self {
        variant as u8 != 0
    }
}
///Field `USELCE` reader - ELC_GPTE Event Source Counter Count Up Enable
pub type UselceR = crate::BitReader<Uselce>;
impl UselceR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Uselce {
        match self.bits {
            false => Uselce::_0,
            true => Uselce::_1,
        }
    }
    ///Counter count up disabled at the ELC_GPTE input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Uselce::_0
    }
    ///Counter count up enabled at the ELC_GPTE input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Uselce::_1
    }
}
///Field `USELCE` writer - ELC_GPTE Event Source Counter Count Up Enable
pub type UselceW<'a, REG> = crate::BitWriter<'a, REG, Uselce>;
impl<'a, REG> UselceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter count up disabled at the ELC_GPTE input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Uselce::_0)
    }
    ///Counter count up enabled at the ELC_GPTE input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Uselce::_1)
    }
}
/**ELC_GPTF Event Source Counter Count Up Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uselcf {
    ///0: Counter count up disabled at the ELC_GPTF input
    _0 = 0,
    ///1: Counter count up enabled at the ELC_GPTF input
    _1 = 1,
}
impl From<Uselcf> for bool {
    #[inline(always)]
    fn from(variant: Uselcf) -> Self {
        variant as u8 != 0
    }
}
///Field `USELCF` reader - ELC_GPTF Event Source Counter Count Up Enable
pub type UselcfR = crate::BitReader<Uselcf>;
impl UselcfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Uselcf {
        match self.bits {
            false => Uselcf::_0,
            true => Uselcf::_1,
        }
    }
    ///Counter count up disabled at the ELC_GPTF input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Uselcf::_0
    }
    ///Counter count up enabled at the ELC_GPTF input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Uselcf::_1
    }
}
///Field `USELCF` writer - ELC_GPTF Event Source Counter Count Up Enable
pub type UselcfW<'a, REG> = crate::BitWriter<'a, REG, Uselcf>;
impl<'a, REG> UselcfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter count up disabled at the ELC_GPTF input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Uselcf::_0)
    }
    ///Counter count up enabled at the ELC_GPTF input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Uselcf::_1)
    }
}
/**ELC_GPTG Event Source Counter Count Up Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uselcg {
    ///0: Counter count up disabled at the ELC_GPTG input
    _0 = 0,
    ///1: Counter count up enabled at the ELC_GPTG input
    _1 = 1,
}
impl From<Uselcg> for bool {
    #[inline(always)]
    fn from(variant: Uselcg) -> Self {
        variant as u8 != 0
    }
}
///Field `USELCG` reader - ELC_GPTG Event Source Counter Count Up Enable
pub type UselcgR = crate::BitReader<Uselcg>;
impl UselcgR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Uselcg {
        match self.bits {
            false => Uselcg::_0,
            true => Uselcg::_1,
        }
    }
    ///Counter count up disabled at the ELC_GPTG input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Uselcg::_0
    }
    ///Counter count up enabled at the ELC_GPTG input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Uselcg::_1
    }
}
///Field `USELCG` writer - ELC_GPTG Event Source Counter Count Up Enable
pub type UselcgW<'a, REG> = crate::BitWriter<'a, REG, Uselcg>;
impl<'a, REG> UselcgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter count up disabled at the ELC_GPTG input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Uselcg::_0)
    }
    ///Counter count up enabled at the ELC_GPTG input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Uselcg::_1)
    }
}
/**ELC_GPTH Event Source Counter Count Up Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uselch {
    ///0: Counter count up disabled at the ELC_GPTH input
    _0 = 0,
    ///1: Counter count up enabled at the ELC_GPTH input
    _1 = 1,
}
impl From<Uselch> for bool {
    #[inline(always)]
    fn from(variant: Uselch) -> Self {
        variant as u8 != 0
    }
}
///Field `USELCH` reader - ELC_GPTH Event Source Counter Count Up Enable
pub type UselchR = crate::BitReader<Uselch>;
impl UselchR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Uselch {
        match self.bits {
            false => Uselch::_0,
            true => Uselch::_1,
        }
    }
    ///Counter count up disabled at the ELC_GPTH input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Uselch::_0
    }
    ///Counter count up enabled at the ELC_GPTH input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Uselch::_1
    }
}
///Field `USELCH` writer - ELC_GPTH Event Source Counter Count Up Enable
pub type UselchW<'a, REG> = crate::BitWriter<'a, REG, Uselch>;
impl<'a, REG> UselchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter count up disabled at the ELC_GPTH input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Uselch::_0)
    }
    ///Counter count up enabled at the ELC_GPTH input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Uselch::_1)
    }
}
impl R {
    ///Bit 0 - GTETRGA Pin Rising Input Source Counter Count Up Enable
    #[inline(always)]
    pub fn usgtrgar(&self) -> UsgtrgarR {
        UsgtrgarR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - GTETRGA Pin Falling Input Source Counter Count Up Enable
    #[inline(always)]
    pub fn usgtrgaf(&self) -> UsgtrgafR {
        UsgtrgafR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - GTETRGB Pin Rising Input Source Counter Count Up Enable
    #[inline(always)]
    pub fn usgtrgbr(&self) -> UsgtrgbrR {
        UsgtrgbrR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - GTETRGB Pin Falling Input Source Counter Count Up Enable
    #[inline(always)]
    pub fn usgtrgbf(&self) -> UsgtrgbfR {
        UsgtrgbfR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - GTETRGC Pin Rising Input Source Counter Count Up Enable
    #[inline(always)]
    pub fn usgtrgcr(&self) -> UsgtrgcrR {
        UsgtrgcrR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - GTETRGC Pin Falling Input Source Counter Count Up Enable
    #[inline(always)]
    pub fn usgtrgcf(&self) -> UsgtrgcfR {
        UsgtrgcfR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - GTETRGD Pin Rising Input Source Counter Count Up Enable
    #[inline(always)]
    pub fn usgtrgdr(&self) -> UsgtrgdrR {
        UsgtrgdrR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - GTETRGD Pin Falling Input Source Counter Count Up Enable
    #[inline(always)]
    pub fn usgtrgdf(&self) -> UsgtrgdfR {
        UsgtrgdfR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - GTIOCnA Pin Rising Input during GTIOCnB Value Low Source Counter Count Up Enable
    #[inline(always)]
    pub fn uscarbl(&self) -> UscarblR {
        UscarblR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - GTIOCnA Pin Rising Input during GTIOCnB Value High Source Counter Count Up Enable
    #[inline(always)]
    pub fn uscarbh(&self) -> UscarbhR {
        UscarbhR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - GTIOCnA Pin Falling Input during GTIOCnB Value Low Source Counter Count Up Enable
    #[inline(always)]
    pub fn uscafbl(&self) -> UscafblR {
        UscafblR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - GTIOCnA Pin Falling Input during GTIOCnB Value High Source Counter Count Up Enable
    #[inline(always)]
    pub fn uscafbh(&self) -> UscafbhR {
        UscafbhR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - GTIOCnB Pin Rising Input during GTIOCnA Value Low Source Counter Count Up Enable
    #[inline(always)]
    pub fn uscbral(&self) -> UscbralR {
        UscbralR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - GTIOCnB Pin Rising Input during GTIOCnA Value High Source Counter Count Up Enable
    #[inline(always)]
    pub fn uscbrah(&self) -> UscbrahR {
        UscbrahR::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - GTIOCnB Pin Falling Input during GTIOCnA Value Low Source Counter Count Up Enable
    #[inline(always)]
    pub fn uscbfal(&self) -> UscbfalR {
        UscbfalR::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - GTIOCnB Pin Falling Input during GTIOCnA Value High Source Counter Count Up Enable
    #[inline(always)]
    pub fn uscbfah(&self) -> UscbfahR {
        UscbfahR::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - ELC_GPTA Event Source Counter Count Up Enable
    #[inline(always)]
    pub fn uselca(&self) -> UselcaR {
        UselcaR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - ELC_GPTB Event Source Counter Count Up Enable
    #[inline(always)]
    pub fn uselcb(&self) -> UselcbR {
        UselcbR::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - ELC_GPTC Event Source Counter Count Up Enable
    #[inline(always)]
    pub fn uselcc(&self) -> UselccR {
        UselccR::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - ELC_GPTD Event Source Counter Count Up Enable
    #[inline(always)]
    pub fn uselcd(&self) -> UselcdR {
        UselcdR::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - ELC_GPTE Event Source Counter Count Up Enable
    #[inline(always)]
    pub fn uselce(&self) -> UselceR {
        UselceR::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - ELC_GPTF Event Source Counter Count Up Enable
    #[inline(always)]
    pub fn uselcf(&self) -> UselcfR {
        UselcfR::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - ELC_GPTG Event Source Counter Count Up Enable
    #[inline(always)]
    pub fn uselcg(&self) -> UselcgR {
        UselcgR::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - ELC_GPTH Event Source Counter Count Up Enable
    #[inline(always)]
    pub fn uselch(&self) -> UselchR {
        UselchR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GTUPSR")
            .field("usgtrgar", &self.usgtrgar())
            .field("usgtrgaf", &self.usgtrgaf())
            .field("usgtrgbr", &self.usgtrgbr())
            .field("usgtrgbf", &self.usgtrgbf())
            .field("usgtrgcr", &self.usgtrgcr())
            .field("usgtrgcf", &self.usgtrgcf())
            .field("usgtrgdr", &self.usgtrgdr())
            .field("usgtrgdf", &self.usgtrgdf())
            .field("uscarbl", &self.uscarbl())
            .field("uscarbh", &self.uscarbh())
            .field("uscafbl", &self.uscafbl())
            .field("uscafbh", &self.uscafbh())
            .field("uscbral", &self.uscbral())
            .field("uscbrah", &self.uscbrah())
            .field("uscbfal", &self.uscbfal())
            .field("uscbfah", &self.uscbfah())
            .field("uselca", &self.uselca())
            .field("uselcb", &self.uselcb())
            .field("uselcc", &self.uselcc())
            .field("uselcd", &self.uselcd())
            .field("uselce", &self.uselce())
            .field("uselcf", &self.uselcf())
            .field("uselcg", &self.uselcg())
            .field("uselch", &self.uselch())
            .finish()
    }
}
impl W {
    ///Bit 0 - GTETRGA Pin Rising Input Source Counter Count Up Enable
    #[inline(always)]
    pub fn usgtrgar(&mut self) -> UsgtrgarW<GtupsrSpec> {
        UsgtrgarW::new(self, 0)
    }
    ///Bit 1 - GTETRGA Pin Falling Input Source Counter Count Up Enable
    #[inline(always)]
    pub fn usgtrgaf(&mut self) -> UsgtrgafW<GtupsrSpec> {
        UsgtrgafW::new(self, 1)
    }
    ///Bit 2 - GTETRGB Pin Rising Input Source Counter Count Up Enable
    #[inline(always)]
    pub fn usgtrgbr(&mut self) -> UsgtrgbrW<GtupsrSpec> {
        UsgtrgbrW::new(self, 2)
    }
    ///Bit 3 - GTETRGB Pin Falling Input Source Counter Count Up Enable
    #[inline(always)]
    pub fn usgtrgbf(&mut self) -> UsgtrgbfW<GtupsrSpec> {
        UsgtrgbfW::new(self, 3)
    }
    ///Bit 4 - GTETRGC Pin Rising Input Source Counter Count Up Enable
    #[inline(always)]
    pub fn usgtrgcr(&mut self) -> UsgtrgcrW<GtupsrSpec> {
        UsgtrgcrW::new(self, 4)
    }
    ///Bit 5 - GTETRGC Pin Falling Input Source Counter Count Up Enable
    #[inline(always)]
    pub fn usgtrgcf(&mut self) -> UsgtrgcfW<GtupsrSpec> {
        UsgtrgcfW::new(self, 5)
    }
    ///Bit 6 - GTETRGD Pin Rising Input Source Counter Count Up Enable
    #[inline(always)]
    pub fn usgtrgdr(&mut self) -> UsgtrgdrW<GtupsrSpec> {
        UsgtrgdrW::new(self, 6)
    }
    ///Bit 7 - GTETRGD Pin Falling Input Source Counter Count Up Enable
    #[inline(always)]
    pub fn usgtrgdf(&mut self) -> UsgtrgdfW<GtupsrSpec> {
        UsgtrgdfW::new(self, 7)
    }
    ///Bit 8 - GTIOCnA Pin Rising Input during GTIOCnB Value Low Source Counter Count Up Enable
    #[inline(always)]
    pub fn uscarbl(&mut self) -> UscarblW<GtupsrSpec> {
        UscarblW::new(self, 8)
    }
    ///Bit 9 - GTIOCnA Pin Rising Input during GTIOCnB Value High Source Counter Count Up Enable
    #[inline(always)]
    pub fn uscarbh(&mut self) -> UscarbhW<GtupsrSpec> {
        UscarbhW::new(self, 9)
    }
    ///Bit 10 - GTIOCnA Pin Falling Input during GTIOCnB Value Low Source Counter Count Up Enable
    #[inline(always)]
    pub fn uscafbl(&mut self) -> UscafblW<GtupsrSpec> {
        UscafblW::new(self, 10)
    }
    ///Bit 11 - GTIOCnA Pin Falling Input during GTIOCnB Value High Source Counter Count Up Enable
    #[inline(always)]
    pub fn uscafbh(&mut self) -> UscafbhW<GtupsrSpec> {
        UscafbhW::new(self, 11)
    }
    ///Bit 12 - GTIOCnB Pin Rising Input during GTIOCnA Value Low Source Counter Count Up Enable
    #[inline(always)]
    pub fn uscbral(&mut self) -> UscbralW<GtupsrSpec> {
        UscbralW::new(self, 12)
    }
    ///Bit 13 - GTIOCnB Pin Rising Input during GTIOCnA Value High Source Counter Count Up Enable
    #[inline(always)]
    pub fn uscbrah(&mut self) -> UscbrahW<GtupsrSpec> {
        UscbrahW::new(self, 13)
    }
    ///Bit 14 - GTIOCnB Pin Falling Input during GTIOCnA Value Low Source Counter Count Up Enable
    #[inline(always)]
    pub fn uscbfal(&mut self) -> UscbfalW<GtupsrSpec> {
        UscbfalW::new(self, 14)
    }
    ///Bit 15 - GTIOCnB Pin Falling Input during GTIOCnA Value High Source Counter Count Up Enable
    #[inline(always)]
    pub fn uscbfah(&mut self) -> UscbfahW<GtupsrSpec> {
        UscbfahW::new(self, 15)
    }
    ///Bit 16 - ELC_GPTA Event Source Counter Count Up Enable
    #[inline(always)]
    pub fn uselca(&mut self) -> UselcaW<GtupsrSpec> {
        UselcaW::new(self, 16)
    }
    ///Bit 17 - ELC_GPTB Event Source Counter Count Up Enable
    #[inline(always)]
    pub fn uselcb(&mut self) -> UselcbW<GtupsrSpec> {
        UselcbW::new(self, 17)
    }
    ///Bit 18 - ELC_GPTC Event Source Counter Count Up Enable
    #[inline(always)]
    pub fn uselcc(&mut self) -> UselccW<GtupsrSpec> {
        UselccW::new(self, 18)
    }
    ///Bit 19 - ELC_GPTD Event Source Counter Count Up Enable
    #[inline(always)]
    pub fn uselcd(&mut self) -> UselcdW<GtupsrSpec> {
        UselcdW::new(self, 19)
    }
    ///Bit 20 - ELC_GPTE Event Source Counter Count Up Enable
    #[inline(always)]
    pub fn uselce(&mut self) -> UselceW<GtupsrSpec> {
        UselceW::new(self, 20)
    }
    ///Bit 21 - ELC_GPTF Event Source Counter Count Up Enable
    #[inline(always)]
    pub fn uselcf(&mut self) -> UselcfW<GtupsrSpec> {
        UselcfW::new(self, 21)
    }
    ///Bit 22 - ELC_GPTG Event Source Counter Count Up Enable
    #[inline(always)]
    pub fn uselcg(&mut self) -> UselcgW<GtupsrSpec> {
        UselcgW::new(self, 22)
    }
    ///Bit 23 - ELC_GPTH Event Source Counter Count Up Enable
    #[inline(always)]
    pub fn uselch(&mut self) -> UselchW<GtupsrSpec> {
        UselchW::new(self, 23)
    }
}
/**General PWM Timer Up Count Source Select Register

You can [`read`](crate::Reg::read) this register and get [`gtupsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtupsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GtupsrSpec;
impl crate::RegisterSpec for GtupsrSpec {
    type Ux = u32;
}
///`read()` method returns [`gtupsr::R`](R) reader structure
impl crate::Readable for GtupsrSpec {}
///`write(|w| ..)` method takes [`gtupsr::W`](W) writer structure
impl crate::Writable for GtupsrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTUPSR to value 0
impl crate::Resettable for GtupsrSpec {}
