///Register `GTDNSR` reader
pub type R = crate::R<GtdnsrSpec>;
///Register `GTDNSR` writer
pub type W = crate::W<GtdnsrSpec>;
/**GTETRGA Pin Rising Input Source Counter Count Down Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dsgtrgar {
    ///0: Counter count down disabled on the rising edge of GTETRGA input
    _0 = 0,
    ///1: Counter count down enabled on the rising edge of GTETRGA input
    _1 = 1,
}
impl From<Dsgtrgar> for bool {
    #[inline(always)]
    fn from(variant: Dsgtrgar) -> Self {
        variant as u8 != 0
    }
}
///Field `DSGTRGAR` reader - GTETRGA Pin Rising Input Source Counter Count Down Enable
pub type DsgtrgarR = crate::BitReader<Dsgtrgar>;
impl DsgtrgarR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dsgtrgar {
        match self.bits {
            false => Dsgtrgar::_0,
            true => Dsgtrgar::_1,
        }
    }
    ///Counter count down disabled on the rising edge of GTETRGA input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dsgtrgar::_0
    }
    ///Counter count down enabled on the rising edge of GTETRGA input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dsgtrgar::_1
    }
}
///Field `DSGTRGAR` writer - GTETRGA Pin Rising Input Source Counter Count Down Enable
pub type DsgtrgarW<'a, REG> = crate::BitWriter<'a, REG, Dsgtrgar>;
impl<'a, REG> DsgtrgarW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter count down disabled on the rising edge of GTETRGA input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dsgtrgar::_0)
    }
    ///Counter count down enabled on the rising edge of GTETRGA input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dsgtrgar::_1)
    }
}
/**GTETRGA Pin Falling Input Source Counter Count Down Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dsgtrgaf {
    ///0: Counter count down disabled on the falling edge of GTETRGA input
    _0 = 0,
    ///1: Counter count down enabled on the falling edge of GTETRGA input
    _1 = 1,
}
impl From<Dsgtrgaf> for bool {
    #[inline(always)]
    fn from(variant: Dsgtrgaf) -> Self {
        variant as u8 != 0
    }
}
///Field `DSGTRGAF` reader - GTETRGA Pin Falling Input Source Counter Count Down Enable
pub type DsgtrgafR = crate::BitReader<Dsgtrgaf>;
impl DsgtrgafR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dsgtrgaf {
        match self.bits {
            false => Dsgtrgaf::_0,
            true => Dsgtrgaf::_1,
        }
    }
    ///Counter count down disabled on the falling edge of GTETRGA input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dsgtrgaf::_0
    }
    ///Counter count down enabled on the falling edge of GTETRGA input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dsgtrgaf::_1
    }
}
///Field `DSGTRGAF` writer - GTETRGA Pin Falling Input Source Counter Count Down Enable
pub type DsgtrgafW<'a, REG> = crate::BitWriter<'a, REG, Dsgtrgaf>;
impl<'a, REG> DsgtrgafW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter count down disabled on the falling edge of GTETRGA input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dsgtrgaf::_0)
    }
    ///Counter count down enabled on the falling edge of GTETRGA input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dsgtrgaf::_1)
    }
}
/**GTETRGB Pin Rising Input Source Counter Count Down Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dsgtrgbr {
    ///0: Counter count down disabled on the rising edge of GTETRGB input
    _0 = 0,
    ///1: Counter count down enabled on the rising edge of GTETRGB input
    _1 = 1,
}
impl From<Dsgtrgbr> for bool {
    #[inline(always)]
    fn from(variant: Dsgtrgbr) -> Self {
        variant as u8 != 0
    }
}
///Field `DSGTRGBR` reader - GTETRGB Pin Rising Input Source Counter Count Down Enable
pub type DsgtrgbrR = crate::BitReader<Dsgtrgbr>;
impl DsgtrgbrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dsgtrgbr {
        match self.bits {
            false => Dsgtrgbr::_0,
            true => Dsgtrgbr::_1,
        }
    }
    ///Counter count down disabled on the rising edge of GTETRGB input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dsgtrgbr::_0
    }
    ///Counter count down enabled on the rising edge of GTETRGB input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dsgtrgbr::_1
    }
}
///Field `DSGTRGBR` writer - GTETRGB Pin Rising Input Source Counter Count Down Enable
pub type DsgtrgbrW<'a, REG> = crate::BitWriter<'a, REG, Dsgtrgbr>;
impl<'a, REG> DsgtrgbrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter count down disabled on the rising edge of GTETRGB input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dsgtrgbr::_0)
    }
    ///Counter count down enabled on the rising edge of GTETRGB input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dsgtrgbr::_1)
    }
}
/**GTETRGB Pin Falling Input Source Counter Count Down Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dsgtrgbf {
    ///0: Counter count down disabled on the falling edge of GTETRGB input
    _0 = 0,
    ///1: Counter count down enabled on the falling edge of GTETRGB input
    _1 = 1,
}
impl From<Dsgtrgbf> for bool {
    #[inline(always)]
    fn from(variant: Dsgtrgbf) -> Self {
        variant as u8 != 0
    }
}
///Field `DSGTRGBF` reader - GTETRGB Pin Falling Input Source Counter Count Down Enable
pub type DsgtrgbfR = crate::BitReader<Dsgtrgbf>;
impl DsgtrgbfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dsgtrgbf {
        match self.bits {
            false => Dsgtrgbf::_0,
            true => Dsgtrgbf::_1,
        }
    }
    ///Counter count down disabled on the falling edge of GTETRGB input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dsgtrgbf::_0
    }
    ///Counter count down enabled on the falling edge of GTETRGB input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dsgtrgbf::_1
    }
}
///Field `DSGTRGBF` writer - GTETRGB Pin Falling Input Source Counter Count Down Enable
pub type DsgtrgbfW<'a, REG> = crate::BitWriter<'a, REG, Dsgtrgbf>;
impl<'a, REG> DsgtrgbfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter count down disabled on the falling edge of GTETRGB input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dsgtrgbf::_0)
    }
    ///Counter count down enabled on the falling edge of GTETRGB input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dsgtrgbf::_1)
    }
}
/**GTETRGC Pin Rising Input Source Counter Count Down Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dsgtrgcr {
    ///0: Counter count down disabled on the rising edge of GTETRGC input
    _0 = 0,
    ///1: Counter count down enabled on the rising edge of GTETRGC input
    _1 = 1,
}
impl From<Dsgtrgcr> for bool {
    #[inline(always)]
    fn from(variant: Dsgtrgcr) -> Self {
        variant as u8 != 0
    }
}
///Field `DSGTRGCR` reader - GTETRGC Pin Rising Input Source Counter Count Down Enable
pub type DsgtrgcrR = crate::BitReader<Dsgtrgcr>;
impl DsgtrgcrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dsgtrgcr {
        match self.bits {
            false => Dsgtrgcr::_0,
            true => Dsgtrgcr::_1,
        }
    }
    ///Counter count down disabled on the rising edge of GTETRGC input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dsgtrgcr::_0
    }
    ///Counter count down enabled on the rising edge of GTETRGC input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dsgtrgcr::_1
    }
}
///Field `DSGTRGCR` writer - GTETRGC Pin Rising Input Source Counter Count Down Enable
pub type DsgtrgcrW<'a, REG> = crate::BitWriter<'a, REG, Dsgtrgcr>;
impl<'a, REG> DsgtrgcrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter count down disabled on the rising edge of GTETRGC input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dsgtrgcr::_0)
    }
    ///Counter count down enabled on the rising edge of GTETRGC input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dsgtrgcr::_1)
    }
}
/**GTETRGC Pin Falling Input Source Counter Count Down Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dsgtrgcf {
    ///0: Counter count down disabled on the falling edge of GTETRGC input
    _0 = 0,
    ///1: Counter count down enabled on the falling edge of GTETRGC input
    _1 = 1,
}
impl From<Dsgtrgcf> for bool {
    #[inline(always)]
    fn from(variant: Dsgtrgcf) -> Self {
        variant as u8 != 0
    }
}
///Field `DSGTRGCF` reader - GTETRGC Pin Falling Input Source Counter Count Down Enable
pub type DsgtrgcfR = crate::BitReader<Dsgtrgcf>;
impl DsgtrgcfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dsgtrgcf {
        match self.bits {
            false => Dsgtrgcf::_0,
            true => Dsgtrgcf::_1,
        }
    }
    ///Counter count down disabled on the falling edge of GTETRGC input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dsgtrgcf::_0
    }
    ///Counter count down enabled on the falling edge of GTETRGC input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dsgtrgcf::_1
    }
}
///Field `DSGTRGCF` writer - GTETRGC Pin Falling Input Source Counter Count Down Enable
pub type DsgtrgcfW<'a, REG> = crate::BitWriter<'a, REG, Dsgtrgcf>;
impl<'a, REG> DsgtrgcfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter count down disabled on the falling edge of GTETRGC input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dsgtrgcf::_0)
    }
    ///Counter count down enabled on the falling edge of GTETRGC input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dsgtrgcf::_1)
    }
}
/**GTETRGD Pin Rising Input Source Counter Count Down Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dsgtrgdr {
    ///0: Counter count down disabled on the rising edge of GTETRGD input
    _0 = 0,
    ///1: Counter count down enabled on the rising edge of GTETRGD input
    _1 = 1,
}
impl From<Dsgtrgdr> for bool {
    #[inline(always)]
    fn from(variant: Dsgtrgdr) -> Self {
        variant as u8 != 0
    }
}
///Field `DSGTRGDR` reader - GTETRGD Pin Rising Input Source Counter Count Down Enable
pub type DsgtrgdrR = crate::BitReader<Dsgtrgdr>;
impl DsgtrgdrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dsgtrgdr {
        match self.bits {
            false => Dsgtrgdr::_0,
            true => Dsgtrgdr::_1,
        }
    }
    ///Counter count down disabled on the rising edge of GTETRGD input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dsgtrgdr::_0
    }
    ///Counter count down enabled on the rising edge of GTETRGD input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dsgtrgdr::_1
    }
}
///Field `DSGTRGDR` writer - GTETRGD Pin Rising Input Source Counter Count Down Enable
pub type DsgtrgdrW<'a, REG> = crate::BitWriter<'a, REG, Dsgtrgdr>;
impl<'a, REG> DsgtrgdrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter count down disabled on the rising edge of GTETRGD input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dsgtrgdr::_0)
    }
    ///Counter count down enabled on the rising edge of GTETRGD input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dsgtrgdr::_1)
    }
}
/**GTETRGD Pin Falling Input Source Counter Count Down Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dsgtrgdf {
    ///0: Counter count down disabled on the falling edge of GTETRGD input
    _0 = 0,
    ///1: Counter count down enabled on the falling edge of GTETRGD input
    _1 = 1,
}
impl From<Dsgtrgdf> for bool {
    #[inline(always)]
    fn from(variant: Dsgtrgdf) -> Self {
        variant as u8 != 0
    }
}
///Field `DSGTRGDF` reader - GTETRGD Pin Falling Input Source Counter Count Down Enable
pub type DsgtrgdfR = crate::BitReader<Dsgtrgdf>;
impl DsgtrgdfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dsgtrgdf {
        match self.bits {
            false => Dsgtrgdf::_0,
            true => Dsgtrgdf::_1,
        }
    }
    ///Counter count down disabled on the falling edge of GTETRGD input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dsgtrgdf::_0
    }
    ///Counter count down enabled on the falling edge of GTETRGD input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dsgtrgdf::_1
    }
}
///Field `DSGTRGDF` writer - GTETRGD Pin Falling Input Source Counter Count Down Enable
pub type DsgtrgdfW<'a, REG> = crate::BitWriter<'a, REG, Dsgtrgdf>;
impl<'a, REG> DsgtrgdfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter count down disabled on the falling edge of GTETRGD input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dsgtrgdf::_0)
    }
    ///Counter count down enabled on the falling edge of GTETRGD input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dsgtrgdf::_1)
    }
}
/**GTIOCnA Pin Rising Input during GTIOCnB Value Low Source Counter Count Down Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dscarbl {
    ///0: Counter count down disabled on the rising edge of GTIOCnA input when GTIOCnB input is 0
    _0 = 0,
    ///1: Counter count down enabled on the rising edge of GTIOCnA input when GTIOCnB input is 0
    _1 = 1,
}
impl From<Dscarbl> for bool {
    #[inline(always)]
    fn from(variant: Dscarbl) -> Self {
        variant as u8 != 0
    }
}
///Field `DSCARBL` reader - GTIOCnA Pin Rising Input during GTIOCnB Value Low Source Counter Count Down Enable
pub type DscarblR = crate::BitReader<Dscarbl>;
impl DscarblR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dscarbl {
        match self.bits {
            false => Dscarbl::_0,
            true => Dscarbl::_1,
        }
    }
    ///Counter count down disabled on the rising edge of GTIOCnA input when GTIOCnB input is 0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dscarbl::_0
    }
    ///Counter count down enabled on the rising edge of GTIOCnA input when GTIOCnB input is 0
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dscarbl::_1
    }
}
///Field `DSCARBL` writer - GTIOCnA Pin Rising Input during GTIOCnB Value Low Source Counter Count Down Enable
pub type DscarblW<'a, REG> = crate::BitWriter<'a, REG, Dscarbl>;
impl<'a, REG> DscarblW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter count down disabled on the rising edge of GTIOCnA input when GTIOCnB input is 0
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dscarbl::_0)
    }
    ///Counter count down enabled on the rising edge of GTIOCnA input when GTIOCnB input is 0
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dscarbl::_1)
    }
}
/**GTIOCnA Pin Rising Input during GTIOCnB Value High Source Counter Count Down Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dscarbh {
    ///0: Counter count down disabled on the rising edge of GTIOCnA input when GTIOCnB input is 1
    _0 = 0,
    ///1: Counter count down enabled on the rising edge of GTIOCnA input when GTIOCnB input is 1
    _1 = 1,
}
impl From<Dscarbh> for bool {
    #[inline(always)]
    fn from(variant: Dscarbh) -> Self {
        variant as u8 != 0
    }
}
///Field `DSCARBH` reader - GTIOCnA Pin Rising Input during GTIOCnB Value High Source Counter Count Down Enable
pub type DscarbhR = crate::BitReader<Dscarbh>;
impl DscarbhR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dscarbh {
        match self.bits {
            false => Dscarbh::_0,
            true => Dscarbh::_1,
        }
    }
    ///Counter count down disabled on the rising edge of GTIOCnA input when GTIOCnB input is 1
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dscarbh::_0
    }
    ///Counter count down enabled on the rising edge of GTIOCnA input when GTIOCnB input is 1
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dscarbh::_1
    }
}
///Field `DSCARBH` writer - GTIOCnA Pin Rising Input during GTIOCnB Value High Source Counter Count Down Enable
pub type DscarbhW<'a, REG> = crate::BitWriter<'a, REG, Dscarbh>;
impl<'a, REG> DscarbhW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter count down disabled on the rising edge of GTIOCnA input when GTIOCnB input is 1
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dscarbh::_0)
    }
    ///Counter count down enabled on the rising edge of GTIOCnA input when GTIOCnB input is 1
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dscarbh::_1)
    }
}
/**GTIOCnA Pin Falling Input during GTIOCnB Value Low Source Counter Count Down Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dscafbl {
    ///0: Counter count down disabled on the falling edge of GTIOCnA input when GTIOCnB input is 0
    _0 = 0,
    ///1: Counter count down enabled on the falling edge of GTIOCnA input when GTIOCnB input is 0
    _1 = 1,
}
impl From<Dscafbl> for bool {
    #[inline(always)]
    fn from(variant: Dscafbl) -> Self {
        variant as u8 != 0
    }
}
///Field `DSCAFBL` reader - GTIOCnA Pin Falling Input during GTIOCnB Value Low Source Counter Count Down Enable
pub type DscafblR = crate::BitReader<Dscafbl>;
impl DscafblR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dscafbl {
        match self.bits {
            false => Dscafbl::_0,
            true => Dscafbl::_1,
        }
    }
    ///Counter count down disabled on the falling edge of GTIOCnA input when GTIOCnB input is 0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dscafbl::_0
    }
    ///Counter count down enabled on the falling edge of GTIOCnA input when GTIOCnB input is 0
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dscafbl::_1
    }
}
///Field `DSCAFBL` writer - GTIOCnA Pin Falling Input during GTIOCnB Value Low Source Counter Count Down Enable
pub type DscafblW<'a, REG> = crate::BitWriter<'a, REG, Dscafbl>;
impl<'a, REG> DscafblW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter count down disabled on the falling edge of GTIOCnA input when GTIOCnB input is 0
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dscafbl::_0)
    }
    ///Counter count down enabled on the falling edge of GTIOCnA input when GTIOCnB input is 0
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dscafbl::_1)
    }
}
/**GTIOCnA Pin Falling Input during GTIOCnB Value High Source Counter Count Down Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dscafbh {
    ///0: Counter count down disabled on the falling edge of GTIOCnA input when GTIOCnB input is 1
    _0 = 0,
    ///1: Counter count down enabled on the falling edge of GTIOCnA input when GTIOCnB input is 1
    _1 = 1,
}
impl From<Dscafbh> for bool {
    #[inline(always)]
    fn from(variant: Dscafbh) -> Self {
        variant as u8 != 0
    }
}
///Field `DSCAFBH` reader - GTIOCnA Pin Falling Input during GTIOCnB Value High Source Counter Count Down Enable
pub type DscafbhR = crate::BitReader<Dscafbh>;
impl DscafbhR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dscafbh {
        match self.bits {
            false => Dscafbh::_0,
            true => Dscafbh::_1,
        }
    }
    ///Counter count down disabled on the falling edge of GTIOCnA input when GTIOCnB input is 1
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dscafbh::_0
    }
    ///Counter count down enabled on the falling edge of GTIOCnA input when GTIOCnB input is 1
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dscafbh::_1
    }
}
///Field `DSCAFBH` writer - GTIOCnA Pin Falling Input during GTIOCnB Value High Source Counter Count Down Enable
pub type DscafbhW<'a, REG> = crate::BitWriter<'a, REG, Dscafbh>;
impl<'a, REG> DscafbhW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter count down disabled on the falling edge of GTIOCnA input when GTIOCnB input is 1
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dscafbh::_0)
    }
    ///Counter count down enabled on the falling edge of GTIOCnA input when GTIOCnB input is 1
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dscafbh::_1)
    }
}
/**GTIOCnB Pin Rising Input during GTIOCnA Value Low Source Counter Count Down Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dscbral {
    ///0: Counter count down disabled on the rising edge of GTIOCnB input when GTIOCnA input is 0
    _0 = 0,
    ///1: Counter count down enabled on the rising edge of GTIOCnB input when GTIOCnA input is 0
    _1 = 1,
}
impl From<Dscbral> for bool {
    #[inline(always)]
    fn from(variant: Dscbral) -> Self {
        variant as u8 != 0
    }
}
///Field `DSCBRAL` reader - GTIOCnB Pin Rising Input during GTIOCnA Value Low Source Counter Count Down Enable
pub type DscbralR = crate::BitReader<Dscbral>;
impl DscbralR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dscbral {
        match self.bits {
            false => Dscbral::_0,
            true => Dscbral::_1,
        }
    }
    ///Counter count down disabled on the rising edge of GTIOCnB input when GTIOCnA input is 0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dscbral::_0
    }
    ///Counter count down enabled on the rising edge of GTIOCnB input when GTIOCnA input is 0
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dscbral::_1
    }
}
///Field `DSCBRAL` writer - GTIOCnB Pin Rising Input during GTIOCnA Value Low Source Counter Count Down Enable
pub type DscbralW<'a, REG> = crate::BitWriter<'a, REG, Dscbral>;
impl<'a, REG> DscbralW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter count down disabled on the rising edge of GTIOCnB input when GTIOCnA input is 0
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dscbral::_0)
    }
    ///Counter count down enabled on the rising edge of GTIOCnB input when GTIOCnA input is 0
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dscbral::_1)
    }
}
/**GTIOCnB Pin Rising Input during GTIOCnA Value High Source Counter Count Down Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dscbrah {
    ///0: Counter count down disabled on the rising edge of GTIOCnB input when GTIOCnA input is 1
    _0 = 0,
    ///1: Counter count down enabled on the rising edge of GTIOCnB input when GTIOCnA input is 1
    _1 = 1,
}
impl From<Dscbrah> for bool {
    #[inline(always)]
    fn from(variant: Dscbrah) -> Self {
        variant as u8 != 0
    }
}
///Field `DSCBRAH` reader - GTIOCnB Pin Rising Input during GTIOCnA Value High Source Counter Count Down Enable
pub type DscbrahR = crate::BitReader<Dscbrah>;
impl DscbrahR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dscbrah {
        match self.bits {
            false => Dscbrah::_0,
            true => Dscbrah::_1,
        }
    }
    ///Counter count down disabled on the rising edge of GTIOCnB input when GTIOCnA input is 1
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dscbrah::_0
    }
    ///Counter count down enabled on the rising edge of GTIOCnB input when GTIOCnA input is 1
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dscbrah::_1
    }
}
///Field `DSCBRAH` writer - GTIOCnB Pin Rising Input during GTIOCnA Value High Source Counter Count Down Enable
pub type DscbrahW<'a, REG> = crate::BitWriter<'a, REG, Dscbrah>;
impl<'a, REG> DscbrahW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter count down disabled on the rising edge of GTIOCnB input when GTIOCnA input is 1
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dscbrah::_0)
    }
    ///Counter count down enabled on the rising edge of GTIOCnB input when GTIOCnA input is 1
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dscbrah::_1)
    }
}
/**GTIOCnB Pin Falling Input during GTIOCnA Value Low Source Counter Count Down Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dscbfal {
    ///0: Counter count down disabled on the falling edge of GTIOCnB input when GTIOCnA input is 0
    _0 = 0,
    ///1: Counter count down enabled on the falling edge of GTIOCnB input when GTIOCnA input is 0
    _1 = 1,
}
impl From<Dscbfal> for bool {
    #[inline(always)]
    fn from(variant: Dscbfal) -> Self {
        variant as u8 != 0
    }
}
///Field `DSCBFAL` reader - GTIOCnB Pin Falling Input during GTIOCnA Value Low Source Counter Count Down Enable
pub type DscbfalR = crate::BitReader<Dscbfal>;
impl DscbfalR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dscbfal {
        match self.bits {
            false => Dscbfal::_0,
            true => Dscbfal::_1,
        }
    }
    ///Counter count down disabled on the falling edge of GTIOCnB input when GTIOCnA input is 0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dscbfal::_0
    }
    ///Counter count down enabled on the falling edge of GTIOCnB input when GTIOCnA input is 0
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dscbfal::_1
    }
}
///Field `DSCBFAL` writer - GTIOCnB Pin Falling Input during GTIOCnA Value Low Source Counter Count Down Enable
pub type DscbfalW<'a, REG> = crate::BitWriter<'a, REG, Dscbfal>;
impl<'a, REG> DscbfalW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter count down disabled on the falling edge of GTIOCnB input when GTIOCnA input is 0
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dscbfal::_0)
    }
    ///Counter count down enabled on the falling edge of GTIOCnB input when GTIOCnA input is 0
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dscbfal::_1)
    }
}
/**GTIOCnB Pin Falling Input during GTIOCnA Value High Source Counter Count Down Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dscbfah {
    ///0: Counter count down disabled on the falling edge of GTIOCnB input when GTIOCnA input is 1
    _0 = 0,
    ///1: Counter count down enabled on the falling edge of GTIOCnB input when GTIOCnA input is 1
    _1 = 1,
}
impl From<Dscbfah> for bool {
    #[inline(always)]
    fn from(variant: Dscbfah) -> Self {
        variant as u8 != 0
    }
}
///Field `DSCBFAH` reader - GTIOCnB Pin Falling Input during GTIOCnA Value High Source Counter Count Down Enable
pub type DscbfahR = crate::BitReader<Dscbfah>;
impl DscbfahR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dscbfah {
        match self.bits {
            false => Dscbfah::_0,
            true => Dscbfah::_1,
        }
    }
    ///Counter count down disabled on the falling edge of GTIOCnB input when GTIOCnA input is 1
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dscbfah::_0
    }
    ///Counter count down enabled on the falling edge of GTIOCnB input when GTIOCnA input is 1
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dscbfah::_1
    }
}
///Field `DSCBFAH` writer - GTIOCnB Pin Falling Input during GTIOCnA Value High Source Counter Count Down Enable
pub type DscbfahW<'a, REG> = crate::BitWriter<'a, REG, Dscbfah>;
impl<'a, REG> DscbfahW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter count down disabled on the falling edge of GTIOCnB input when GTIOCnA input is 1
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dscbfah::_0)
    }
    ///Counter count down enabled on the falling edge of GTIOCnB input when GTIOCnA input is 1
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dscbfah::_1)
    }
}
/**ELC_GPTA Event Source Counter Count Down Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dselca {
    ///0: Counter count down disabled at the ELC_GPTA input
    _0 = 0,
    ///1: Counter count down enabled at the ELC_GPTA input
    _1 = 1,
}
impl From<Dselca> for bool {
    #[inline(always)]
    fn from(variant: Dselca) -> Self {
        variant as u8 != 0
    }
}
///Field `DSELCA` reader - ELC_GPTA Event Source Counter Count Down Enable
pub type DselcaR = crate::BitReader<Dselca>;
impl DselcaR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dselca {
        match self.bits {
            false => Dselca::_0,
            true => Dselca::_1,
        }
    }
    ///Counter count down disabled at the ELC_GPTA input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dselca::_0
    }
    ///Counter count down enabled at the ELC_GPTA input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dselca::_1
    }
}
///Field `DSELCA` writer - ELC_GPTA Event Source Counter Count Down Enable
pub type DselcaW<'a, REG> = crate::BitWriter<'a, REG, Dselca>;
impl<'a, REG> DselcaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter count down disabled at the ELC_GPTA input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dselca::_0)
    }
    ///Counter count down enabled at the ELC_GPTA input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dselca::_1)
    }
}
/**ELC_GPTB Event Source Counter Count Down Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dselcb {
    ///0: Counter count down disabled at the ELC_GPTB input
    _0 = 0,
    ///1: Counter count down enabled at the ELC_GPTB input
    _1 = 1,
}
impl From<Dselcb> for bool {
    #[inline(always)]
    fn from(variant: Dselcb) -> Self {
        variant as u8 != 0
    }
}
///Field `DSELCB` reader - ELC_GPTB Event Source Counter Count Down Enable
pub type DselcbR = crate::BitReader<Dselcb>;
impl DselcbR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dselcb {
        match self.bits {
            false => Dselcb::_0,
            true => Dselcb::_1,
        }
    }
    ///Counter count down disabled at the ELC_GPTB input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dselcb::_0
    }
    ///Counter count down enabled at the ELC_GPTB input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dselcb::_1
    }
}
///Field `DSELCB` writer - ELC_GPTB Event Source Counter Count Down Enable
pub type DselcbW<'a, REG> = crate::BitWriter<'a, REG, Dselcb>;
impl<'a, REG> DselcbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter count down disabled at the ELC_GPTB input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dselcb::_0)
    }
    ///Counter count down enabled at the ELC_GPTB input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dselcb::_1)
    }
}
/**ELC_GPTC Event Source Counter Count Down Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dselcc {
    ///0: Counter count down disabled at the ELC_GPTC input
    _0 = 0,
    ///1: Counter count down enabled at the ELC_GPTC input
    _1 = 1,
}
impl From<Dselcc> for bool {
    #[inline(always)]
    fn from(variant: Dselcc) -> Self {
        variant as u8 != 0
    }
}
///Field `DSELCC` reader - ELC_GPTC Event Source Counter Count Down Enable
pub type DselccR = crate::BitReader<Dselcc>;
impl DselccR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dselcc {
        match self.bits {
            false => Dselcc::_0,
            true => Dselcc::_1,
        }
    }
    ///Counter count down disabled at the ELC_GPTC input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dselcc::_0
    }
    ///Counter count down enabled at the ELC_GPTC input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dselcc::_1
    }
}
///Field `DSELCC` writer - ELC_GPTC Event Source Counter Count Down Enable
pub type DselccW<'a, REG> = crate::BitWriter<'a, REG, Dselcc>;
impl<'a, REG> DselccW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter count down disabled at the ELC_GPTC input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dselcc::_0)
    }
    ///Counter count down enabled at the ELC_GPTC input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dselcc::_1)
    }
}
/**ELC_GPTD Event Source Counter Count Down Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dselcd {
    ///0: Counter count down disabled at the ELC_GPTD input
    _0 = 0,
    ///1: Counter count down enabled at the ELC_GPTD input
    _1 = 1,
}
impl From<Dselcd> for bool {
    #[inline(always)]
    fn from(variant: Dselcd) -> Self {
        variant as u8 != 0
    }
}
///Field `DSELCD` reader - ELC_GPTD Event Source Counter Count Down Enable
pub type DselcdR = crate::BitReader<Dselcd>;
impl DselcdR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dselcd {
        match self.bits {
            false => Dselcd::_0,
            true => Dselcd::_1,
        }
    }
    ///Counter count down disabled at the ELC_GPTD input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dselcd::_0
    }
    ///Counter count down enabled at the ELC_GPTD input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dselcd::_1
    }
}
///Field `DSELCD` writer - ELC_GPTD Event Source Counter Count Down Enable
pub type DselcdW<'a, REG> = crate::BitWriter<'a, REG, Dselcd>;
impl<'a, REG> DselcdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter count down disabled at the ELC_GPTD input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dselcd::_0)
    }
    ///Counter count down enabled at the ELC_GPTD input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dselcd::_1)
    }
}
/**ELC_GPTE Event Source Counter Count Down Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dselce {
    ///0: Counter count down disabled at the ELC_GPTE input
    _0 = 0,
    ///1: Counter count down enabled at the ELC_GPTE input
    _1 = 1,
}
impl From<Dselce> for bool {
    #[inline(always)]
    fn from(variant: Dselce) -> Self {
        variant as u8 != 0
    }
}
///Field `DSELCE` reader - ELC_GPTE Event Source Counter Count Down Enable
pub type DselceR = crate::BitReader<Dselce>;
impl DselceR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dselce {
        match self.bits {
            false => Dselce::_0,
            true => Dselce::_1,
        }
    }
    ///Counter count down disabled at the ELC_GPTE input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dselce::_0
    }
    ///Counter count down enabled at the ELC_GPTE input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dselce::_1
    }
}
///Field `DSELCE` writer - ELC_GPTE Event Source Counter Count Down Enable
pub type DselceW<'a, REG> = crate::BitWriter<'a, REG, Dselce>;
impl<'a, REG> DselceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter count down disabled at the ELC_GPTE input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dselce::_0)
    }
    ///Counter count down enabled at the ELC_GPTE input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dselce::_1)
    }
}
/**ELC_GPTF Event Source Counter Count Down Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dselcf {
    ///0: Counter count down disabled at the ELC_GPTF input
    _0 = 0,
    ///1: Counter count down enabled at the ELC_GPTF input
    _1 = 1,
}
impl From<Dselcf> for bool {
    #[inline(always)]
    fn from(variant: Dselcf) -> Self {
        variant as u8 != 0
    }
}
///Field `DSELCF` reader - ELC_GPTF Event Source Counter Count Down Enable
pub type DselcfR = crate::BitReader<Dselcf>;
impl DselcfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dselcf {
        match self.bits {
            false => Dselcf::_0,
            true => Dselcf::_1,
        }
    }
    ///Counter count down disabled at the ELC_GPTF input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dselcf::_0
    }
    ///Counter count down enabled at the ELC_GPTF input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dselcf::_1
    }
}
///Field `DSELCF` writer - ELC_GPTF Event Source Counter Count Down Enable
pub type DselcfW<'a, REG> = crate::BitWriter<'a, REG, Dselcf>;
impl<'a, REG> DselcfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter count down disabled at the ELC_GPTF input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dselcf::_0)
    }
    ///Counter count down enabled at the ELC_GPTF input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dselcf::_1)
    }
}
/**ELC_GPTG Event Source Counter Count Down Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dselcg {
    ///0: Counter count down disabled at the ELC_GPTG input
    _0 = 0,
    ///1: Counter count down enabled at the ELC_GPTG input
    _1 = 1,
}
impl From<Dselcg> for bool {
    #[inline(always)]
    fn from(variant: Dselcg) -> Self {
        variant as u8 != 0
    }
}
///Field `DSELCG` reader - ELC_GPTG Event Source Counter Count Down Enable
pub type DselcgR = crate::BitReader<Dselcg>;
impl DselcgR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dselcg {
        match self.bits {
            false => Dselcg::_0,
            true => Dselcg::_1,
        }
    }
    ///Counter count down disabled at the ELC_GPTG input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dselcg::_0
    }
    ///Counter count down enabled at the ELC_GPTG input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dselcg::_1
    }
}
///Field `DSELCG` writer - ELC_GPTG Event Source Counter Count Down Enable
pub type DselcgW<'a, REG> = crate::BitWriter<'a, REG, Dselcg>;
impl<'a, REG> DselcgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter count down disabled at the ELC_GPTG input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dselcg::_0)
    }
    ///Counter count down enabled at the ELC_GPTG input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dselcg::_1)
    }
}
/**ELC_GPTF Event Source Counter Count Down Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dselch {
    ///0: Counter count down disabled at the ELC_GPTF input
    _0 = 0,
    ///1: Counter count down enabled at the ELC_GPTF input
    _1 = 1,
}
impl From<Dselch> for bool {
    #[inline(always)]
    fn from(variant: Dselch) -> Self {
        variant as u8 != 0
    }
}
///Field `DSELCH` reader - ELC_GPTF Event Source Counter Count Down Enable
pub type DselchR = crate::BitReader<Dselch>;
impl DselchR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dselch {
        match self.bits {
            false => Dselch::_0,
            true => Dselch::_1,
        }
    }
    ///Counter count down disabled at the ELC_GPTF input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dselch::_0
    }
    ///Counter count down enabled at the ELC_GPTF input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dselch::_1
    }
}
///Field `DSELCH` writer - ELC_GPTF Event Source Counter Count Down Enable
pub type DselchW<'a, REG> = crate::BitWriter<'a, REG, Dselch>;
impl<'a, REG> DselchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter count down disabled at the ELC_GPTF input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dselch::_0)
    }
    ///Counter count down enabled at the ELC_GPTF input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dselch::_1)
    }
}
impl R {
    ///Bit 0 - GTETRGA Pin Rising Input Source Counter Count Down Enable
    #[inline(always)]
    pub fn dsgtrgar(&self) -> DsgtrgarR {
        DsgtrgarR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - GTETRGA Pin Falling Input Source Counter Count Down Enable
    #[inline(always)]
    pub fn dsgtrgaf(&self) -> DsgtrgafR {
        DsgtrgafR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - GTETRGB Pin Rising Input Source Counter Count Down Enable
    #[inline(always)]
    pub fn dsgtrgbr(&self) -> DsgtrgbrR {
        DsgtrgbrR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - GTETRGB Pin Falling Input Source Counter Count Down Enable
    #[inline(always)]
    pub fn dsgtrgbf(&self) -> DsgtrgbfR {
        DsgtrgbfR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - GTETRGC Pin Rising Input Source Counter Count Down Enable
    #[inline(always)]
    pub fn dsgtrgcr(&self) -> DsgtrgcrR {
        DsgtrgcrR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - GTETRGC Pin Falling Input Source Counter Count Down Enable
    #[inline(always)]
    pub fn dsgtrgcf(&self) -> DsgtrgcfR {
        DsgtrgcfR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - GTETRGD Pin Rising Input Source Counter Count Down Enable
    #[inline(always)]
    pub fn dsgtrgdr(&self) -> DsgtrgdrR {
        DsgtrgdrR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - GTETRGD Pin Falling Input Source Counter Count Down Enable
    #[inline(always)]
    pub fn dsgtrgdf(&self) -> DsgtrgdfR {
        DsgtrgdfR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - GTIOCnA Pin Rising Input during GTIOCnB Value Low Source Counter Count Down Enable
    #[inline(always)]
    pub fn dscarbl(&self) -> DscarblR {
        DscarblR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - GTIOCnA Pin Rising Input during GTIOCnB Value High Source Counter Count Down Enable
    #[inline(always)]
    pub fn dscarbh(&self) -> DscarbhR {
        DscarbhR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - GTIOCnA Pin Falling Input during GTIOCnB Value Low Source Counter Count Down Enable
    #[inline(always)]
    pub fn dscafbl(&self) -> DscafblR {
        DscafblR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - GTIOCnA Pin Falling Input during GTIOCnB Value High Source Counter Count Down Enable
    #[inline(always)]
    pub fn dscafbh(&self) -> DscafbhR {
        DscafbhR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - GTIOCnB Pin Rising Input during GTIOCnA Value Low Source Counter Count Down Enable
    #[inline(always)]
    pub fn dscbral(&self) -> DscbralR {
        DscbralR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - GTIOCnB Pin Rising Input during GTIOCnA Value High Source Counter Count Down Enable
    #[inline(always)]
    pub fn dscbrah(&self) -> DscbrahR {
        DscbrahR::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - GTIOCnB Pin Falling Input during GTIOCnA Value Low Source Counter Count Down Enable
    #[inline(always)]
    pub fn dscbfal(&self) -> DscbfalR {
        DscbfalR::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - GTIOCnB Pin Falling Input during GTIOCnA Value High Source Counter Count Down Enable
    #[inline(always)]
    pub fn dscbfah(&self) -> DscbfahR {
        DscbfahR::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - ELC_GPTA Event Source Counter Count Down Enable
    #[inline(always)]
    pub fn dselca(&self) -> DselcaR {
        DselcaR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - ELC_GPTB Event Source Counter Count Down Enable
    #[inline(always)]
    pub fn dselcb(&self) -> DselcbR {
        DselcbR::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - ELC_GPTC Event Source Counter Count Down Enable
    #[inline(always)]
    pub fn dselcc(&self) -> DselccR {
        DselccR::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - ELC_GPTD Event Source Counter Count Down Enable
    #[inline(always)]
    pub fn dselcd(&self) -> DselcdR {
        DselcdR::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - ELC_GPTE Event Source Counter Count Down Enable
    #[inline(always)]
    pub fn dselce(&self) -> DselceR {
        DselceR::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - ELC_GPTF Event Source Counter Count Down Enable
    #[inline(always)]
    pub fn dselcf(&self) -> DselcfR {
        DselcfR::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - ELC_GPTG Event Source Counter Count Down Enable
    #[inline(always)]
    pub fn dselcg(&self) -> DselcgR {
        DselcgR::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - ELC_GPTF Event Source Counter Count Down Enable
    #[inline(always)]
    pub fn dselch(&self) -> DselchR {
        DselchR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GTDNSR")
            .field("dsgtrgar", &self.dsgtrgar())
            .field("dsgtrgaf", &self.dsgtrgaf())
            .field("dsgtrgbr", &self.dsgtrgbr())
            .field("dsgtrgbf", &self.dsgtrgbf())
            .field("dsgtrgcr", &self.dsgtrgcr())
            .field("dsgtrgcf", &self.dsgtrgcf())
            .field("dsgtrgdr", &self.dsgtrgdr())
            .field("dsgtrgdf", &self.dsgtrgdf())
            .field("dscarbl", &self.dscarbl())
            .field("dscarbh", &self.dscarbh())
            .field("dscafbl", &self.dscafbl())
            .field("dscafbh", &self.dscafbh())
            .field("dscbral", &self.dscbral())
            .field("dscbrah", &self.dscbrah())
            .field("dscbfal", &self.dscbfal())
            .field("dscbfah", &self.dscbfah())
            .field("dselca", &self.dselca())
            .field("dselcb", &self.dselcb())
            .field("dselcc", &self.dselcc())
            .field("dselcd", &self.dselcd())
            .field("dselce", &self.dselce())
            .field("dselcf", &self.dselcf())
            .field("dselcg", &self.dselcg())
            .field("dselch", &self.dselch())
            .finish()
    }
}
impl W {
    ///Bit 0 - GTETRGA Pin Rising Input Source Counter Count Down Enable
    #[inline(always)]
    pub fn dsgtrgar(&mut self) -> DsgtrgarW<GtdnsrSpec> {
        DsgtrgarW::new(self, 0)
    }
    ///Bit 1 - GTETRGA Pin Falling Input Source Counter Count Down Enable
    #[inline(always)]
    pub fn dsgtrgaf(&mut self) -> DsgtrgafW<GtdnsrSpec> {
        DsgtrgafW::new(self, 1)
    }
    ///Bit 2 - GTETRGB Pin Rising Input Source Counter Count Down Enable
    #[inline(always)]
    pub fn dsgtrgbr(&mut self) -> DsgtrgbrW<GtdnsrSpec> {
        DsgtrgbrW::new(self, 2)
    }
    ///Bit 3 - GTETRGB Pin Falling Input Source Counter Count Down Enable
    #[inline(always)]
    pub fn dsgtrgbf(&mut self) -> DsgtrgbfW<GtdnsrSpec> {
        DsgtrgbfW::new(self, 3)
    }
    ///Bit 4 - GTETRGC Pin Rising Input Source Counter Count Down Enable
    #[inline(always)]
    pub fn dsgtrgcr(&mut self) -> DsgtrgcrW<GtdnsrSpec> {
        DsgtrgcrW::new(self, 4)
    }
    ///Bit 5 - GTETRGC Pin Falling Input Source Counter Count Down Enable
    #[inline(always)]
    pub fn dsgtrgcf(&mut self) -> DsgtrgcfW<GtdnsrSpec> {
        DsgtrgcfW::new(self, 5)
    }
    ///Bit 6 - GTETRGD Pin Rising Input Source Counter Count Down Enable
    #[inline(always)]
    pub fn dsgtrgdr(&mut self) -> DsgtrgdrW<GtdnsrSpec> {
        DsgtrgdrW::new(self, 6)
    }
    ///Bit 7 - GTETRGD Pin Falling Input Source Counter Count Down Enable
    #[inline(always)]
    pub fn dsgtrgdf(&mut self) -> DsgtrgdfW<GtdnsrSpec> {
        DsgtrgdfW::new(self, 7)
    }
    ///Bit 8 - GTIOCnA Pin Rising Input during GTIOCnB Value Low Source Counter Count Down Enable
    #[inline(always)]
    pub fn dscarbl(&mut self) -> DscarblW<GtdnsrSpec> {
        DscarblW::new(self, 8)
    }
    ///Bit 9 - GTIOCnA Pin Rising Input during GTIOCnB Value High Source Counter Count Down Enable
    #[inline(always)]
    pub fn dscarbh(&mut self) -> DscarbhW<GtdnsrSpec> {
        DscarbhW::new(self, 9)
    }
    ///Bit 10 - GTIOCnA Pin Falling Input during GTIOCnB Value Low Source Counter Count Down Enable
    #[inline(always)]
    pub fn dscafbl(&mut self) -> DscafblW<GtdnsrSpec> {
        DscafblW::new(self, 10)
    }
    ///Bit 11 - GTIOCnA Pin Falling Input during GTIOCnB Value High Source Counter Count Down Enable
    #[inline(always)]
    pub fn dscafbh(&mut self) -> DscafbhW<GtdnsrSpec> {
        DscafbhW::new(self, 11)
    }
    ///Bit 12 - GTIOCnB Pin Rising Input during GTIOCnA Value Low Source Counter Count Down Enable
    #[inline(always)]
    pub fn dscbral(&mut self) -> DscbralW<GtdnsrSpec> {
        DscbralW::new(self, 12)
    }
    ///Bit 13 - GTIOCnB Pin Rising Input during GTIOCnA Value High Source Counter Count Down Enable
    #[inline(always)]
    pub fn dscbrah(&mut self) -> DscbrahW<GtdnsrSpec> {
        DscbrahW::new(self, 13)
    }
    ///Bit 14 - GTIOCnB Pin Falling Input during GTIOCnA Value Low Source Counter Count Down Enable
    #[inline(always)]
    pub fn dscbfal(&mut self) -> DscbfalW<GtdnsrSpec> {
        DscbfalW::new(self, 14)
    }
    ///Bit 15 - GTIOCnB Pin Falling Input during GTIOCnA Value High Source Counter Count Down Enable
    #[inline(always)]
    pub fn dscbfah(&mut self) -> DscbfahW<GtdnsrSpec> {
        DscbfahW::new(self, 15)
    }
    ///Bit 16 - ELC_GPTA Event Source Counter Count Down Enable
    #[inline(always)]
    pub fn dselca(&mut self) -> DselcaW<GtdnsrSpec> {
        DselcaW::new(self, 16)
    }
    ///Bit 17 - ELC_GPTB Event Source Counter Count Down Enable
    #[inline(always)]
    pub fn dselcb(&mut self) -> DselcbW<GtdnsrSpec> {
        DselcbW::new(self, 17)
    }
    ///Bit 18 - ELC_GPTC Event Source Counter Count Down Enable
    #[inline(always)]
    pub fn dselcc(&mut self) -> DselccW<GtdnsrSpec> {
        DselccW::new(self, 18)
    }
    ///Bit 19 - ELC_GPTD Event Source Counter Count Down Enable
    #[inline(always)]
    pub fn dselcd(&mut self) -> DselcdW<GtdnsrSpec> {
        DselcdW::new(self, 19)
    }
    ///Bit 20 - ELC_GPTE Event Source Counter Count Down Enable
    #[inline(always)]
    pub fn dselce(&mut self) -> DselceW<GtdnsrSpec> {
        DselceW::new(self, 20)
    }
    ///Bit 21 - ELC_GPTF Event Source Counter Count Down Enable
    #[inline(always)]
    pub fn dselcf(&mut self) -> DselcfW<GtdnsrSpec> {
        DselcfW::new(self, 21)
    }
    ///Bit 22 - ELC_GPTG Event Source Counter Count Down Enable
    #[inline(always)]
    pub fn dselcg(&mut self) -> DselcgW<GtdnsrSpec> {
        DselcgW::new(self, 22)
    }
    ///Bit 23 - ELC_GPTF Event Source Counter Count Down Enable
    #[inline(always)]
    pub fn dselch(&mut self) -> DselchW<GtdnsrSpec> {
        DselchW::new(self, 23)
    }
}
/**General PWM Timer Down Count Source Select Register

You can [`read`](crate::Reg::read) this register and get [`gtdnsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtdnsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GtdnsrSpec;
impl crate::RegisterSpec for GtdnsrSpec {
    type Ux = u32;
}
///`read()` method returns [`gtdnsr::R`](R) reader structure
impl crate::Readable for GtdnsrSpec {}
///`write(|w| ..)` method takes [`gtdnsr::W`](W) writer structure
impl crate::Writable for GtdnsrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTDNSR to value 0
impl crate::Resettable for GtdnsrSpec {}
