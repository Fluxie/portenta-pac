///Register `GTSSR` reader
pub type R = crate::R<GtssrSpec>;
///Register `GTSSR` writer
pub type W = crate::W<GtssrSpec>;
/**GTETRGA Pin Rising Input Source Counter Start Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ssgtrgar {
    ///0: Counter start disabled on the rising edge of GTETRGA input
    _0 = 0,
    ///1: Counter start enabled on the rising edge of GTETRGA input
    _1 = 1,
}
impl From<Ssgtrgar> for bool {
    #[inline(always)]
    fn from(variant: Ssgtrgar) -> Self {
        variant as u8 != 0
    }
}
///Field `SSGTRGAR` reader - GTETRGA Pin Rising Input Source Counter Start Enable
pub type SsgtrgarR = crate::BitReader<Ssgtrgar>;
impl SsgtrgarR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ssgtrgar {
        match self.bits {
            false => Ssgtrgar::_0,
            true => Ssgtrgar::_1,
        }
    }
    ///Counter start disabled on the rising edge of GTETRGA input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ssgtrgar::_0
    }
    ///Counter start enabled on the rising edge of GTETRGA input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ssgtrgar::_1
    }
}
///Field `SSGTRGAR` writer - GTETRGA Pin Rising Input Source Counter Start Enable
pub type SsgtrgarW<'a, REG> = crate::BitWriter<'a, REG, Ssgtrgar>;
impl<'a, REG> SsgtrgarW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter start disabled on the rising edge of GTETRGA input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ssgtrgar::_0)
    }
    ///Counter start enabled on the rising edge of GTETRGA input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ssgtrgar::_1)
    }
}
/**GTETRGA Pin Falling Input Source Counter Start Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ssgtrgaf {
    ///0: Counter start disabled on the falling edge of GTETRGA input
    _0 = 0,
    ///1: Counter start enabled on the falling edge of GTETRGA input
    _1 = 1,
}
impl From<Ssgtrgaf> for bool {
    #[inline(always)]
    fn from(variant: Ssgtrgaf) -> Self {
        variant as u8 != 0
    }
}
///Field `SSGTRGAF` reader - GTETRGA Pin Falling Input Source Counter Start Enable
pub type SsgtrgafR = crate::BitReader<Ssgtrgaf>;
impl SsgtrgafR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ssgtrgaf {
        match self.bits {
            false => Ssgtrgaf::_0,
            true => Ssgtrgaf::_1,
        }
    }
    ///Counter start disabled on the falling edge of GTETRGA input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ssgtrgaf::_0
    }
    ///Counter start enabled on the falling edge of GTETRGA input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ssgtrgaf::_1
    }
}
///Field `SSGTRGAF` writer - GTETRGA Pin Falling Input Source Counter Start Enable
pub type SsgtrgafW<'a, REG> = crate::BitWriter<'a, REG, Ssgtrgaf>;
impl<'a, REG> SsgtrgafW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter start disabled on the falling edge of GTETRGA input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ssgtrgaf::_0)
    }
    ///Counter start enabled on the falling edge of GTETRGA input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ssgtrgaf::_1)
    }
}
/**GTETRGB Pin Rising Input Source Counter Start Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ssgtrgbr {
    ///0: Counter start disabled on the rising edge of GTETRGB input
    _0 = 0,
    ///1: Counter start enabled on the rising edge of GTETRGB input
    _1 = 1,
}
impl From<Ssgtrgbr> for bool {
    #[inline(always)]
    fn from(variant: Ssgtrgbr) -> Self {
        variant as u8 != 0
    }
}
///Field `SSGTRGBR` reader - GTETRGB Pin Rising Input Source Counter Start Enable
pub type SsgtrgbrR = crate::BitReader<Ssgtrgbr>;
impl SsgtrgbrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ssgtrgbr {
        match self.bits {
            false => Ssgtrgbr::_0,
            true => Ssgtrgbr::_1,
        }
    }
    ///Counter start disabled on the rising edge of GTETRGB input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ssgtrgbr::_0
    }
    ///Counter start enabled on the rising edge of GTETRGB input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ssgtrgbr::_1
    }
}
///Field `SSGTRGBR` writer - GTETRGB Pin Rising Input Source Counter Start Enable
pub type SsgtrgbrW<'a, REG> = crate::BitWriter<'a, REG, Ssgtrgbr>;
impl<'a, REG> SsgtrgbrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter start disabled on the rising edge of GTETRGB input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ssgtrgbr::_0)
    }
    ///Counter start enabled on the rising edge of GTETRGB input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ssgtrgbr::_1)
    }
}
/**GTETRGB Pin Falling Input Source Counter Start Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ssgtrgbf {
    ///0: Counter start disabled on the falling edge of GTETRGB input
    _0 = 0,
    ///1: Counter start enabled on the falling edge of GTETRGB input
    _1 = 1,
}
impl From<Ssgtrgbf> for bool {
    #[inline(always)]
    fn from(variant: Ssgtrgbf) -> Self {
        variant as u8 != 0
    }
}
///Field `SSGTRGBF` reader - GTETRGB Pin Falling Input Source Counter Start Enable
pub type SsgtrgbfR = crate::BitReader<Ssgtrgbf>;
impl SsgtrgbfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ssgtrgbf {
        match self.bits {
            false => Ssgtrgbf::_0,
            true => Ssgtrgbf::_1,
        }
    }
    ///Counter start disabled on the falling edge of GTETRGB input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ssgtrgbf::_0
    }
    ///Counter start enabled on the falling edge of GTETRGB input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ssgtrgbf::_1
    }
}
///Field `SSGTRGBF` writer - GTETRGB Pin Falling Input Source Counter Start Enable
pub type SsgtrgbfW<'a, REG> = crate::BitWriter<'a, REG, Ssgtrgbf>;
impl<'a, REG> SsgtrgbfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter start disabled on the falling edge of GTETRGB input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ssgtrgbf::_0)
    }
    ///Counter start enabled on the falling edge of GTETRGB input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ssgtrgbf::_1)
    }
}
/**GTETRGC Pin Rising Input Source Counter Start Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ssgtrgcr {
    ///0: Counter start disabled on the rising edge of GTETRGC input
    _0 = 0,
    ///1: Counter start enabled on the rising edge of GTETRGC input
    _1 = 1,
}
impl From<Ssgtrgcr> for bool {
    #[inline(always)]
    fn from(variant: Ssgtrgcr) -> Self {
        variant as u8 != 0
    }
}
///Field `SSGTRGCR` reader - GTETRGC Pin Rising Input Source Counter Start Enable
pub type SsgtrgcrR = crate::BitReader<Ssgtrgcr>;
impl SsgtrgcrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ssgtrgcr {
        match self.bits {
            false => Ssgtrgcr::_0,
            true => Ssgtrgcr::_1,
        }
    }
    ///Counter start disabled on the rising edge of GTETRGC input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ssgtrgcr::_0
    }
    ///Counter start enabled on the rising edge of GTETRGC input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ssgtrgcr::_1
    }
}
///Field `SSGTRGCR` writer - GTETRGC Pin Rising Input Source Counter Start Enable
pub type SsgtrgcrW<'a, REG> = crate::BitWriter<'a, REG, Ssgtrgcr>;
impl<'a, REG> SsgtrgcrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter start disabled on the rising edge of GTETRGC input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ssgtrgcr::_0)
    }
    ///Counter start enabled on the rising edge of GTETRGC input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ssgtrgcr::_1)
    }
}
/**GTETRGC Pin Falling Input Source Counter Start Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ssgtrgcf {
    ///0: Counter start disabled on the falling edge of GTETRGC input
    _0 = 0,
    ///1: Counter start enabled on the falling edge of GTETRGC input
    _1 = 1,
}
impl From<Ssgtrgcf> for bool {
    #[inline(always)]
    fn from(variant: Ssgtrgcf) -> Self {
        variant as u8 != 0
    }
}
///Field `SSGTRGCF` reader - GTETRGC Pin Falling Input Source Counter Start Enable
pub type SsgtrgcfR = crate::BitReader<Ssgtrgcf>;
impl SsgtrgcfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ssgtrgcf {
        match self.bits {
            false => Ssgtrgcf::_0,
            true => Ssgtrgcf::_1,
        }
    }
    ///Counter start disabled on the falling edge of GTETRGC input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ssgtrgcf::_0
    }
    ///Counter start enabled on the falling edge of GTETRGC input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ssgtrgcf::_1
    }
}
///Field `SSGTRGCF` writer - GTETRGC Pin Falling Input Source Counter Start Enable
pub type SsgtrgcfW<'a, REG> = crate::BitWriter<'a, REG, Ssgtrgcf>;
impl<'a, REG> SsgtrgcfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter start disabled on the falling edge of GTETRGC input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ssgtrgcf::_0)
    }
    ///Counter start enabled on the falling edge of GTETRGC input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ssgtrgcf::_1)
    }
}
/**GTETRGD Pin Rising Input Source Counter Start Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ssgtrgdr {
    ///0: Counter start disabled on the rising edge of GTETRGD input
    _0 = 0,
    ///1: Counter start enabled on the rising edge of GTETRGD input
    _1 = 1,
}
impl From<Ssgtrgdr> for bool {
    #[inline(always)]
    fn from(variant: Ssgtrgdr) -> Self {
        variant as u8 != 0
    }
}
///Field `SSGTRGDR` reader - GTETRGD Pin Rising Input Source Counter Start Enable
pub type SsgtrgdrR = crate::BitReader<Ssgtrgdr>;
impl SsgtrgdrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ssgtrgdr {
        match self.bits {
            false => Ssgtrgdr::_0,
            true => Ssgtrgdr::_1,
        }
    }
    ///Counter start disabled on the rising edge of GTETRGD input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ssgtrgdr::_0
    }
    ///Counter start enabled on the rising edge of GTETRGD input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ssgtrgdr::_1
    }
}
///Field `SSGTRGDR` writer - GTETRGD Pin Rising Input Source Counter Start Enable
pub type SsgtrgdrW<'a, REG> = crate::BitWriter<'a, REG, Ssgtrgdr>;
impl<'a, REG> SsgtrgdrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter start disabled on the rising edge of GTETRGD input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ssgtrgdr::_0)
    }
    ///Counter start enabled on the rising edge of GTETRGD input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ssgtrgdr::_1)
    }
}
/**GTETRGD Pin Falling Input Source Counter Start Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ssgtrgdf {
    ///0: Counter start disabled on the falling edge of GTETRGD input
    _0 = 0,
    ///1: Counter start enabled on the falling edge of GTETRGD input
    _1 = 1,
}
impl From<Ssgtrgdf> for bool {
    #[inline(always)]
    fn from(variant: Ssgtrgdf) -> Self {
        variant as u8 != 0
    }
}
///Field `SSGTRGDF` reader - GTETRGD Pin Falling Input Source Counter Start Enable
pub type SsgtrgdfR = crate::BitReader<Ssgtrgdf>;
impl SsgtrgdfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ssgtrgdf {
        match self.bits {
            false => Ssgtrgdf::_0,
            true => Ssgtrgdf::_1,
        }
    }
    ///Counter start disabled on the falling edge of GTETRGD input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ssgtrgdf::_0
    }
    ///Counter start enabled on the falling edge of GTETRGD input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ssgtrgdf::_1
    }
}
///Field `SSGTRGDF` writer - GTETRGD Pin Falling Input Source Counter Start Enable
pub type SsgtrgdfW<'a, REG> = crate::BitWriter<'a, REG, Ssgtrgdf>;
impl<'a, REG> SsgtrgdfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter start disabled on the falling edge of GTETRGD input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ssgtrgdf::_0)
    }
    ///Counter start enabled on the falling edge of GTETRGD input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ssgtrgdf::_1)
    }
}
/**GTIOCnA Pin Rising Input during GTIOCnB Value Low Source Counter Start Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sscarbl {
    ///0: Counter start disabled on the rising edge of GTIOCnA input when GTIOCnB input is 0
    _0 = 0,
    ///1: Counter start enabled on the rising edge of GTIOCnA input when GTIOCnB input is 0
    _1 = 1,
}
impl From<Sscarbl> for bool {
    #[inline(always)]
    fn from(variant: Sscarbl) -> Self {
        variant as u8 != 0
    }
}
///Field `SSCARBL` reader - GTIOCnA Pin Rising Input during GTIOCnB Value Low Source Counter Start Enable
pub type SscarblR = crate::BitReader<Sscarbl>;
impl SscarblR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sscarbl {
        match self.bits {
            false => Sscarbl::_0,
            true => Sscarbl::_1,
        }
    }
    ///Counter start disabled on the rising edge of GTIOCnA input when GTIOCnB input is 0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sscarbl::_0
    }
    ///Counter start enabled on the rising edge of GTIOCnA input when GTIOCnB input is 0
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sscarbl::_1
    }
}
///Field `SSCARBL` writer - GTIOCnA Pin Rising Input during GTIOCnB Value Low Source Counter Start Enable
pub type SscarblW<'a, REG> = crate::BitWriter<'a, REG, Sscarbl>;
impl<'a, REG> SscarblW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter start disabled on the rising edge of GTIOCnA input when GTIOCnB input is 0
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sscarbl::_0)
    }
    ///Counter start enabled on the rising edge of GTIOCnA input when GTIOCnB input is 0
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sscarbl::_1)
    }
}
/**GTIOCnA Pin Rising Input during GTIOCnB Value High Source Counter Start Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sscarbh {
    ///0: Counter start disabled on the rising edge of GTIOCnA input when GTIOCnB input is 1
    _0 = 0,
    ///1: Counter start enabled on the rising edge of GTIOCnA input when GTIOCnB input is 1
    _1 = 1,
}
impl From<Sscarbh> for bool {
    #[inline(always)]
    fn from(variant: Sscarbh) -> Self {
        variant as u8 != 0
    }
}
///Field `SSCARBH` reader - GTIOCnA Pin Rising Input during GTIOCnB Value High Source Counter Start Enable
pub type SscarbhR = crate::BitReader<Sscarbh>;
impl SscarbhR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sscarbh {
        match self.bits {
            false => Sscarbh::_0,
            true => Sscarbh::_1,
        }
    }
    ///Counter start disabled on the rising edge of GTIOCnA input when GTIOCnB input is 1
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sscarbh::_0
    }
    ///Counter start enabled on the rising edge of GTIOCnA input when GTIOCnB input is 1
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sscarbh::_1
    }
}
///Field `SSCARBH` writer - GTIOCnA Pin Rising Input during GTIOCnB Value High Source Counter Start Enable
pub type SscarbhW<'a, REG> = crate::BitWriter<'a, REG, Sscarbh>;
impl<'a, REG> SscarbhW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter start disabled on the rising edge of GTIOCnA input when GTIOCnB input is 1
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sscarbh::_0)
    }
    ///Counter start enabled on the rising edge of GTIOCnA input when GTIOCnB input is 1
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sscarbh::_1)
    }
}
/**GTIOCnA Pin Falling Input during GTIOCnB Value Low Source Counter Start Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sscafbl {
    ///0: Counter start disabled on the falling edge of GTIOCnA input when GTIOCnB input is 0
    _0 = 0,
    ///1: Counter start enabled on the falling edge of GTIOCnA input when GTIOCnB input is 0
    _1 = 1,
}
impl From<Sscafbl> for bool {
    #[inline(always)]
    fn from(variant: Sscafbl) -> Self {
        variant as u8 != 0
    }
}
///Field `SSCAFBL` reader - GTIOCnA Pin Falling Input during GTIOCnB Value Low Source Counter Start Enable
pub type SscafblR = crate::BitReader<Sscafbl>;
impl SscafblR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sscafbl {
        match self.bits {
            false => Sscafbl::_0,
            true => Sscafbl::_1,
        }
    }
    ///Counter start disabled on the falling edge of GTIOCnA input when GTIOCnB input is 0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sscafbl::_0
    }
    ///Counter start enabled on the falling edge of GTIOCnA input when GTIOCnB input is 0
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sscafbl::_1
    }
}
///Field `SSCAFBL` writer - GTIOCnA Pin Falling Input during GTIOCnB Value Low Source Counter Start Enable
pub type SscafblW<'a, REG> = crate::BitWriter<'a, REG, Sscafbl>;
impl<'a, REG> SscafblW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter start disabled on the falling edge of GTIOCnA input when GTIOCnB input is 0
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sscafbl::_0)
    }
    ///Counter start enabled on the falling edge of GTIOCnA input when GTIOCnB input is 0
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sscafbl::_1)
    }
}
/**GTIOCnA Pin Falling Input during GTIOCnB Value High Source Counter Start Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sscafbh {
    ///0: Counter start disabled on the falling edge of GTIOCnA input when GTIOCnB input is 1
    _0 = 0,
    ///1: Counter start enabled on the falling edge of GTIOCnA input when GTIOCnB input is 1
    _1 = 1,
}
impl From<Sscafbh> for bool {
    #[inline(always)]
    fn from(variant: Sscafbh) -> Self {
        variant as u8 != 0
    }
}
///Field `SSCAFBH` reader - GTIOCnA Pin Falling Input during GTIOCnB Value High Source Counter Start Enable
pub type SscafbhR = crate::BitReader<Sscafbh>;
impl SscafbhR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sscafbh {
        match self.bits {
            false => Sscafbh::_0,
            true => Sscafbh::_1,
        }
    }
    ///Counter start disabled on the falling edge of GTIOCnA input when GTIOCnB input is 1
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sscafbh::_0
    }
    ///Counter start enabled on the falling edge of GTIOCnA input when GTIOCnB input is 1
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sscafbh::_1
    }
}
///Field `SSCAFBH` writer - GTIOCnA Pin Falling Input during GTIOCnB Value High Source Counter Start Enable
pub type SscafbhW<'a, REG> = crate::BitWriter<'a, REG, Sscafbh>;
impl<'a, REG> SscafbhW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter start disabled on the falling edge of GTIOCnA input when GTIOCnB input is 1
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sscafbh::_0)
    }
    ///Counter start enabled on the falling edge of GTIOCnA input when GTIOCnB input is 1
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sscafbh::_1)
    }
}
/**GTIOCnB Pin Rising Input during GTIOCnA Value Low Source Counter Start Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sscbral {
    ///0: Counter start disabled on the rising edge of GTIOCnB input when GTIOCnA input is 0
    _0 = 0,
    ///1: Counter start enabled on the rising edge of GTIOCnB input when GTIOCnA input is 0
    _1 = 1,
}
impl From<Sscbral> for bool {
    #[inline(always)]
    fn from(variant: Sscbral) -> Self {
        variant as u8 != 0
    }
}
///Field `SSCBRAL` reader - GTIOCnB Pin Rising Input during GTIOCnA Value Low Source Counter Start Enable
pub type SscbralR = crate::BitReader<Sscbral>;
impl SscbralR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sscbral {
        match self.bits {
            false => Sscbral::_0,
            true => Sscbral::_1,
        }
    }
    ///Counter start disabled on the rising edge of GTIOCnB input when GTIOCnA input is 0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sscbral::_0
    }
    ///Counter start enabled on the rising edge of GTIOCnB input when GTIOCnA input is 0
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sscbral::_1
    }
}
///Field `SSCBRAL` writer - GTIOCnB Pin Rising Input during GTIOCnA Value Low Source Counter Start Enable
pub type SscbralW<'a, REG> = crate::BitWriter<'a, REG, Sscbral>;
impl<'a, REG> SscbralW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter start disabled on the rising edge of GTIOCnB input when GTIOCnA input is 0
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sscbral::_0)
    }
    ///Counter start enabled on the rising edge of GTIOCnB input when GTIOCnA input is 0
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sscbral::_1)
    }
}
/**GTIOCnB Pin Rising Input during GTIOCnA Value High Source Counter Start Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sscbrah {
    ///0: Counter start disabled on the rising edge of GTIOCnB input when GTIOCnA input is 1
    _0 = 0,
    ///1: Counter start enabled on the rising edge of GTIOCnB input when GTIOCnA input is 1
    _1 = 1,
}
impl From<Sscbrah> for bool {
    #[inline(always)]
    fn from(variant: Sscbrah) -> Self {
        variant as u8 != 0
    }
}
///Field `SSCBRAH` reader - GTIOCnB Pin Rising Input during GTIOCnA Value High Source Counter Start Enable
pub type SscbrahR = crate::BitReader<Sscbrah>;
impl SscbrahR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sscbrah {
        match self.bits {
            false => Sscbrah::_0,
            true => Sscbrah::_1,
        }
    }
    ///Counter start disabled on the rising edge of GTIOCnB input when GTIOCnA input is 1
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sscbrah::_0
    }
    ///Counter start enabled on the rising edge of GTIOCnB input when GTIOCnA input is 1
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sscbrah::_1
    }
}
///Field `SSCBRAH` writer - GTIOCnB Pin Rising Input during GTIOCnA Value High Source Counter Start Enable
pub type SscbrahW<'a, REG> = crate::BitWriter<'a, REG, Sscbrah>;
impl<'a, REG> SscbrahW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter start disabled on the rising edge of GTIOCnB input when GTIOCnA input is 1
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sscbrah::_0)
    }
    ///Counter start enabled on the rising edge of GTIOCnB input when GTIOCnA input is 1
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sscbrah::_1)
    }
}
/**GTIOCnB Pin Falling Input during GTIOCnA Value Low Source Counter Start Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sscbfal {
    ///0: Counter start disabled on the falling edge of GTIOCnB input when GTIOCnA input is 0
    _0 = 0,
    ///1: Counter start enabled on the falling edge of GTIOCnB input when GTIOCnA input is 0
    _1 = 1,
}
impl From<Sscbfal> for bool {
    #[inline(always)]
    fn from(variant: Sscbfal) -> Self {
        variant as u8 != 0
    }
}
///Field `SSCBFAL` reader - GTIOCnB Pin Falling Input during GTIOCnA Value Low Source Counter Start Enable
pub type SscbfalR = crate::BitReader<Sscbfal>;
impl SscbfalR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sscbfal {
        match self.bits {
            false => Sscbfal::_0,
            true => Sscbfal::_1,
        }
    }
    ///Counter start disabled on the falling edge of GTIOCnB input when GTIOCnA input is 0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sscbfal::_0
    }
    ///Counter start enabled on the falling edge of GTIOCnB input when GTIOCnA input is 0
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sscbfal::_1
    }
}
///Field `SSCBFAL` writer - GTIOCnB Pin Falling Input during GTIOCnA Value Low Source Counter Start Enable
pub type SscbfalW<'a, REG> = crate::BitWriter<'a, REG, Sscbfal>;
impl<'a, REG> SscbfalW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter start disabled on the falling edge of GTIOCnB input when GTIOCnA input is 0
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sscbfal::_0)
    }
    ///Counter start enabled on the falling edge of GTIOCnB input when GTIOCnA input is 0
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sscbfal::_1)
    }
}
/**GTIOCnB Pin Falling Input during GTIOCnA Value High Source Counter Start Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sscbfah {
    ///0: Counter start disabled on the falling edge of GTIOCnB input when GTIOCnA input is 1
    _0 = 0,
    ///1: Counter start enabled on the falling edge of GTIOCnB input when GTIOCnA input is 1
    _1 = 1,
}
impl From<Sscbfah> for bool {
    #[inline(always)]
    fn from(variant: Sscbfah) -> Self {
        variant as u8 != 0
    }
}
///Field `SSCBFAH` reader - GTIOCnB Pin Falling Input during GTIOCnA Value High Source Counter Start Enable
pub type SscbfahR = crate::BitReader<Sscbfah>;
impl SscbfahR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sscbfah {
        match self.bits {
            false => Sscbfah::_0,
            true => Sscbfah::_1,
        }
    }
    ///Counter start disabled on the falling edge of GTIOCnB input when GTIOCnA input is 1
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sscbfah::_0
    }
    ///Counter start enabled on the falling edge of GTIOCnB input when GTIOCnA input is 1
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sscbfah::_1
    }
}
///Field `SSCBFAH` writer - GTIOCnB Pin Falling Input during GTIOCnA Value High Source Counter Start Enable
pub type SscbfahW<'a, REG> = crate::BitWriter<'a, REG, Sscbfah>;
impl<'a, REG> SscbfahW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter start disabled on the falling edge of GTIOCnB input when GTIOCnA input is 1
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sscbfah::_0)
    }
    ///Counter start enabled on the falling edge of GTIOCnB input when GTIOCnA input is 1
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sscbfah::_1)
    }
}
/**ELC_GPTA Event Source Counter Start Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sselca {
    ///0: Counter start disabled at the ELC_GPTA input
    _0 = 0,
    ///1: Counter start enabled at the ELC_GPTA input
    _1 = 1,
}
impl From<Sselca> for bool {
    #[inline(always)]
    fn from(variant: Sselca) -> Self {
        variant as u8 != 0
    }
}
///Field `SSELCA` reader - ELC_GPTA Event Source Counter Start Enable
pub type SselcaR = crate::BitReader<Sselca>;
impl SselcaR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sselca {
        match self.bits {
            false => Sselca::_0,
            true => Sselca::_1,
        }
    }
    ///Counter start disabled at the ELC_GPTA input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sselca::_0
    }
    ///Counter start enabled at the ELC_GPTA input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sselca::_1
    }
}
///Field `SSELCA` writer - ELC_GPTA Event Source Counter Start Enable
pub type SselcaW<'a, REG> = crate::BitWriter<'a, REG, Sselca>;
impl<'a, REG> SselcaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter start disabled at the ELC_GPTA input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sselca::_0)
    }
    ///Counter start enabled at the ELC_GPTA input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sselca::_1)
    }
}
/**ELC_GPTB Event Source Counter Start Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sselcb {
    ///0: Counter start disabled at the ELC_GPTB input
    _0 = 0,
    ///1: Counter start enabled at the ELC_GPTB input
    _1 = 1,
}
impl From<Sselcb> for bool {
    #[inline(always)]
    fn from(variant: Sselcb) -> Self {
        variant as u8 != 0
    }
}
///Field `SSELCB` reader - ELC_GPTB Event Source Counter Start Enable
pub type SselcbR = crate::BitReader<Sselcb>;
impl SselcbR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sselcb {
        match self.bits {
            false => Sselcb::_0,
            true => Sselcb::_1,
        }
    }
    ///Counter start disabled at the ELC_GPTB input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sselcb::_0
    }
    ///Counter start enabled at the ELC_GPTB input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sselcb::_1
    }
}
///Field `SSELCB` writer - ELC_GPTB Event Source Counter Start Enable
pub type SselcbW<'a, REG> = crate::BitWriter<'a, REG, Sselcb>;
impl<'a, REG> SselcbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter start disabled at the ELC_GPTB input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sselcb::_0)
    }
    ///Counter start enabled at the ELC_GPTB input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sselcb::_1)
    }
}
/**ELC_GPTC Event Source Counter Start Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sselcc {
    ///0: Counter start disabled at the ELC_GPTC input
    _0 = 0,
    ///1: Counter start enabled at the ELC_GPTC input
    _1 = 1,
}
impl From<Sselcc> for bool {
    #[inline(always)]
    fn from(variant: Sselcc) -> Self {
        variant as u8 != 0
    }
}
///Field `SSELCC` reader - ELC_GPTC Event Source Counter Start Enable
pub type SselccR = crate::BitReader<Sselcc>;
impl SselccR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sselcc {
        match self.bits {
            false => Sselcc::_0,
            true => Sselcc::_1,
        }
    }
    ///Counter start disabled at the ELC_GPTC input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sselcc::_0
    }
    ///Counter start enabled at the ELC_GPTC input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sselcc::_1
    }
}
///Field `SSELCC` writer - ELC_GPTC Event Source Counter Start Enable
pub type SselccW<'a, REG> = crate::BitWriter<'a, REG, Sselcc>;
impl<'a, REG> SselccW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter start disabled at the ELC_GPTC input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sselcc::_0)
    }
    ///Counter start enabled at the ELC_GPTC input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sselcc::_1)
    }
}
/**ELC_GPTD Event Source Counter Start Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sselcd {
    ///0: Counter start disabled at the ELC_GPTD input
    _0 = 0,
    ///1: Counter start enabled at the ELC_GPTD input
    _1 = 1,
}
impl From<Sselcd> for bool {
    #[inline(always)]
    fn from(variant: Sselcd) -> Self {
        variant as u8 != 0
    }
}
///Field `SSELCD` reader - ELC_GPTD Event Source Counter Start Enable
pub type SselcdR = crate::BitReader<Sselcd>;
impl SselcdR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sselcd {
        match self.bits {
            false => Sselcd::_0,
            true => Sselcd::_1,
        }
    }
    ///Counter start disabled at the ELC_GPTD input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sselcd::_0
    }
    ///Counter start enabled at the ELC_GPTD input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sselcd::_1
    }
}
///Field `SSELCD` writer - ELC_GPTD Event Source Counter Start Enable
pub type SselcdW<'a, REG> = crate::BitWriter<'a, REG, Sselcd>;
impl<'a, REG> SselcdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter start disabled at the ELC_GPTD input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sselcd::_0)
    }
    ///Counter start enabled at the ELC_GPTD input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sselcd::_1)
    }
}
/**ELC_GPTE Event Source Counter Start Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sselce {
    ///0: Counter start disabled at the ELC_GPTE input
    _0 = 0,
    ///1: Counter start enabled at the ELC_GPTE input
    _1 = 1,
}
impl From<Sselce> for bool {
    #[inline(always)]
    fn from(variant: Sselce) -> Self {
        variant as u8 != 0
    }
}
///Field `SSELCE` reader - ELC_GPTE Event Source Counter Start Enable
pub type SselceR = crate::BitReader<Sselce>;
impl SselceR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sselce {
        match self.bits {
            false => Sselce::_0,
            true => Sselce::_1,
        }
    }
    ///Counter start disabled at the ELC_GPTE input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sselce::_0
    }
    ///Counter start enabled at the ELC_GPTE input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sselce::_1
    }
}
///Field `SSELCE` writer - ELC_GPTE Event Source Counter Start Enable
pub type SselceW<'a, REG> = crate::BitWriter<'a, REG, Sselce>;
impl<'a, REG> SselceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter start disabled at the ELC_GPTE input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sselce::_0)
    }
    ///Counter start enabled at the ELC_GPTE input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sselce::_1)
    }
}
/**ELC_GPTF Event Source Counter Start Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sselcf {
    ///0: Counter start disabled at the ELC_GPTF input
    _0 = 0,
    ///1: Counter start enabled at the ELC_GPTF input
    _1 = 1,
}
impl From<Sselcf> for bool {
    #[inline(always)]
    fn from(variant: Sselcf) -> Self {
        variant as u8 != 0
    }
}
///Field `SSELCF` reader - ELC_GPTF Event Source Counter Start Enable
pub type SselcfR = crate::BitReader<Sselcf>;
impl SselcfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sselcf {
        match self.bits {
            false => Sselcf::_0,
            true => Sselcf::_1,
        }
    }
    ///Counter start disabled at the ELC_GPTF input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sselcf::_0
    }
    ///Counter start enabled at the ELC_GPTF input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sselcf::_1
    }
}
///Field `SSELCF` writer - ELC_GPTF Event Source Counter Start Enable
pub type SselcfW<'a, REG> = crate::BitWriter<'a, REG, Sselcf>;
impl<'a, REG> SselcfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter start disabled at the ELC_GPTF input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sselcf::_0)
    }
    ///Counter start enabled at the ELC_GPTF input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sselcf::_1)
    }
}
/**ELC_GPTG Event Source Counter Start Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sselcg {
    ///0: Counter start disabled at the ELC_GPTG input
    _0 = 0,
    ///1: Counter start enabled at the ELC_GPTG input
    _1 = 1,
}
impl From<Sselcg> for bool {
    #[inline(always)]
    fn from(variant: Sselcg) -> Self {
        variant as u8 != 0
    }
}
///Field `SSELCG` reader - ELC_GPTG Event Source Counter Start Enable
pub type SselcgR = crate::BitReader<Sselcg>;
impl SselcgR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sselcg {
        match self.bits {
            false => Sselcg::_0,
            true => Sselcg::_1,
        }
    }
    ///Counter start disabled at the ELC_GPTG input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sselcg::_0
    }
    ///Counter start enabled at the ELC_GPTG input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sselcg::_1
    }
}
///Field `SSELCG` writer - ELC_GPTG Event Source Counter Start Enable
pub type SselcgW<'a, REG> = crate::BitWriter<'a, REG, Sselcg>;
impl<'a, REG> SselcgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter start disabled at the ELC_GPTG input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sselcg::_0)
    }
    ///Counter start enabled at the ELC_GPTG input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sselcg::_1)
    }
}
/**ELC_GPTH Event Source Counter Start Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sselch {
    ///0: Counter start disabled at the ELC_GPTH input
    _0 = 0,
    ///1: Counter start enabled at the ELC_GPTH input
    _1 = 1,
}
impl From<Sselch> for bool {
    #[inline(always)]
    fn from(variant: Sselch) -> Self {
        variant as u8 != 0
    }
}
///Field `SSELCH` reader - ELC_GPTH Event Source Counter Start Enable
pub type SselchR = crate::BitReader<Sselch>;
impl SselchR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sselch {
        match self.bits {
            false => Sselch::_0,
            true => Sselch::_1,
        }
    }
    ///Counter start disabled at the ELC_GPTH input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sselch::_0
    }
    ///Counter start enabled at the ELC_GPTH input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sselch::_1
    }
}
///Field `SSELCH` writer - ELC_GPTH Event Source Counter Start Enable
pub type SselchW<'a, REG> = crate::BitWriter<'a, REG, Sselch>;
impl<'a, REG> SselchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter start disabled at the ELC_GPTH input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sselch::_0)
    }
    ///Counter start enabled at the ELC_GPTH input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sselch::_1)
    }
}
/**Software Source Counter Start Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cstrt {
    ///0: Counter start disabled by the GTSTR register
    _0 = 0,
    ///1: Counter start enabled by the GTSTR register
    _1 = 1,
}
impl From<Cstrt> for bool {
    #[inline(always)]
    fn from(variant: Cstrt) -> Self {
        variant as u8 != 0
    }
}
///Field `CSTRT` reader - Software Source Counter Start Enable
pub type CstrtR = crate::BitReader<Cstrt>;
impl CstrtR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cstrt {
        match self.bits {
            false => Cstrt::_0,
            true => Cstrt::_1,
        }
    }
    ///Counter start disabled by the GTSTR register
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cstrt::_0
    }
    ///Counter start enabled by the GTSTR register
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cstrt::_1
    }
}
///Field `CSTRT` writer - Software Source Counter Start Enable
pub type CstrtW<'a, REG> = crate::BitWriter<'a, REG, Cstrt>;
impl<'a, REG> CstrtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counter start disabled by the GTSTR register
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cstrt::_0)
    }
    ///Counter start enabled by the GTSTR register
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cstrt::_1)
    }
}
impl R {
    ///Bit 0 - GTETRGA Pin Rising Input Source Counter Start Enable
    #[inline(always)]
    pub fn ssgtrgar(&self) -> SsgtrgarR {
        SsgtrgarR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - GTETRGA Pin Falling Input Source Counter Start Enable
    #[inline(always)]
    pub fn ssgtrgaf(&self) -> SsgtrgafR {
        SsgtrgafR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - GTETRGB Pin Rising Input Source Counter Start Enable
    #[inline(always)]
    pub fn ssgtrgbr(&self) -> SsgtrgbrR {
        SsgtrgbrR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - GTETRGB Pin Falling Input Source Counter Start Enable
    #[inline(always)]
    pub fn ssgtrgbf(&self) -> SsgtrgbfR {
        SsgtrgbfR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - GTETRGC Pin Rising Input Source Counter Start Enable
    #[inline(always)]
    pub fn ssgtrgcr(&self) -> SsgtrgcrR {
        SsgtrgcrR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - GTETRGC Pin Falling Input Source Counter Start Enable
    #[inline(always)]
    pub fn ssgtrgcf(&self) -> SsgtrgcfR {
        SsgtrgcfR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - GTETRGD Pin Rising Input Source Counter Start Enable
    #[inline(always)]
    pub fn ssgtrgdr(&self) -> SsgtrgdrR {
        SsgtrgdrR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - GTETRGD Pin Falling Input Source Counter Start Enable
    #[inline(always)]
    pub fn ssgtrgdf(&self) -> SsgtrgdfR {
        SsgtrgdfR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - GTIOCnA Pin Rising Input during GTIOCnB Value Low Source Counter Start Enable
    #[inline(always)]
    pub fn sscarbl(&self) -> SscarblR {
        SscarblR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - GTIOCnA Pin Rising Input during GTIOCnB Value High Source Counter Start Enable
    #[inline(always)]
    pub fn sscarbh(&self) -> SscarbhR {
        SscarbhR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - GTIOCnA Pin Falling Input during GTIOCnB Value Low Source Counter Start Enable
    #[inline(always)]
    pub fn sscafbl(&self) -> SscafblR {
        SscafblR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - GTIOCnA Pin Falling Input during GTIOCnB Value High Source Counter Start Enable
    #[inline(always)]
    pub fn sscafbh(&self) -> SscafbhR {
        SscafbhR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - GTIOCnB Pin Rising Input during GTIOCnA Value Low Source Counter Start Enable
    #[inline(always)]
    pub fn sscbral(&self) -> SscbralR {
        SscbralR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - GTIOCnB Pin Rising Input during GTIOCnA Value High Source Counter Start Enable
    #[inline(always)]
    pub fn sscbrah(&self) -> SscbrahR {
        SscbrahR::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - GTIOCnB Pin Falling Input during GTIOCnA Value Low Source Counter Start Enable
    #[inline(always)]
    pub fn sscbfal(&self) -> SscbfalR {
        SscbfalR::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - GTIOCnB Pin Falling Input during GTIOCnA Value High Source Counter Start Enable
    #[inline(always)]
    pub fn sscbfah(&self) -> SscbfahR {
        SscbfahR::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - ELC_GPTA Event Source Counter Start Enable
    #[inline(always)]
    pub fn sselca(&self) -> SselcaR {
        SselcaR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - ELC_GPTB Event Source Counter Start Enable
    #[inline(always)]
    pub fn sselcb(&self) -> SselcbR {
        SselcbR::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - ELC_GPTC Event Source Counter Start Enable
    #[inline(always)]
    pub fn sselcc(&self) -> SselccR {
        SselccR::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - ELC_GPTD Event Source Counter Start Enable
    #[inline(always)]
    pub fn sselcd(&self) -> SselcdR {
        SselcdR::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - ELC_GPTE Event Source Counter Start Enable
    #[inline(always)]
    pub fn sselce(&self) -> SselceR {
        SselceR::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - ELC_GPTF Event Source Counter Start Enable
    #[inline(always)]
    pub fn sselcf(&self) -> SselcfR {
        SselcfR::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - ELC_GPTG Event Source Counter Start Enable
    #[inline(always)]
    pub fn sselcg(&self) -> SselcgR {
        SselcgR::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - ELC_GPTH Event Source Counter Start Enable
    #[inline(always)]
    pub fn sselch(&self) -> SselchR {
        SselchR::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 31 - Software Source Counter Start Enable
    #[inline(always)]
    pub fn cstrt(&self) -> CstrtR {
        CstrtR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GTSSR")
            .field("ssgtrgar", &self.ssgtrgar())
            .field("ssgtrgaf", &self.ssgtrgaf())
            .field("ssgtrgbr", &self.ssgtrgbr())
            .field("ssgtrgbf", &self.ssgtrgbf())
            .field("ssgtrgcr", &self.ssgtrgcr())
            .field("ssgtrgcf", &self.ssgtrgcf())
            .field("ssgtrgdr", &self.ssgtrgdr())
            .field("ssgtrgdf", &self.ssgtrgdf())
            .field("sscarbl", &self.sscarbl())
            .field("sscarbh", &self.sscarbh())
            .field("sscafbl", &self.sscafbl())
            .field("sscafbh", &self.sscafbh())
            .field("sscbral", &self.sscbral())
            .field("sscbrah", &self.sscbrah())
            .field("sscbfal", &self.sscbfal())
            .field("sscbfah", &self.sscbfah())
            .field("sselca", &self.sselca())
            .field("sselcb", &self.sselcb())
            .field("sselcc", &self.sselcc())
            .field("sselcd", &self.sselcd())
            .field("sselce", &self.sselce())
            .field("sselcf", &self.sselcf())
            .field("sselcg", &self.sselcg())
            .field("sselch", &self.sselch())
            .field("cstrt", &self.cstrt())
            .finish()
    }
}
impl W {
    ///Bit 0 - GTETRGA Pin Rising Input Source Counter Start Enable
    #[inline(always)]
    pub fn ssgtrgar(&mut self) -> SsgtrgarW<GtssrSpec> {
        SsgtrgarW::new(self, 0)
    }
    ///Bit 1 - GTETRGA Pin Falling Input Source Counter Start Enable
    #[inline(always)]
    pub fn ssgtrgaf(&mut self) -> SsgtrgafW<GtssrSpec> {
        SsgtrgafW::new(self, 1)
    }
    ///Bit 2 - GTETRGB Pin Rising Input Source Counter Start Enable
    #[inline(always)]
    pub fn ssgtrgbr(&mut self) -> SsgtrgbrW<GtssrSpec> {
        SsgtrgbrW::new(self, 2)
    }
    ///Bit 3 - GTETRGB Pin Falling Input Source Counter Start Enable
    #[inline(always)]
    pub fn ssgtrgbf(&mut self) -> SsgtrgbfW<GtssrSpec> {
        SsgtrgbfW::new(self, 3)
    }
    ///Bit 4 - GTETRGC Pin Rising Input Source Counter Start Enable
    #[inline(always)]
    pub fn ssgtrgcr(&mut self) -> SsgtrgcrW<GtssrSpec> {
        SsgtrgcrW::new(self, 4)
    }
    ///Bit 5 - GTETRGC Pin Falling Input Source Counter Start Enable
    #[inline(always)]
    pub fn ssgtrgcf(&mut self) -> SsgtrgcfW<GtssrSpec> {
        SsgtrgcfW::new(self, 5)
    }
    ///Bit 6 - GTETRGD Pin Rising Input Source Counter Start Enable
    #[inline(always)]
    pub fn ssgtrgdr(&mut self) -> SsgtrgdrW<GtssrSpec> {
        SsgtrgdrW::new(self, 6)
    }
    ///Bit 7 - GTETRGD Pin Falling Input Source Counter Start Enable
    #[inline(always)]
    pub fn ssgtrgdf(&mut self) -> SsgtrgdfW<GtssrSpec> {
        SsgtrgdfW::new(self, 7)
    }
    ///Bit 8 - GTIOCnA Pin Rising Input during GTIOCnB Value Low Source Counter Start Enable
    #[inline(always)]
    pub fn sscarbl(&mut self) -> SscarblW<GtssrSpec> {
        SscarblW::new(self, 8)
    }
    ///Bit 9 - GTIOCnA Pin Rising Input during GTIOCnB Value High Source Counter Start Enable
    #[inline(always)]
    pub fn sscarbh(&mut self) -> SscarbhW<GtssrSpec> {
        SscarbhW::new(self, 9)
    }
    ///Bit 10 - GTIOCnA Pin Falling Input during GTIOCnB Value Low Source Counter Start Enable
    #[inline(always)]
    pub fn sscafbl(&mut self) -> SscafblW<GtssrSpec> {
        SscafblW::new(self, 10)
    }
    ///Bit 11 - GTIOCnA Pin Falling Input during GTIOCnB Value High Source Counter Start Enable
    #[inline(always)]
    pub fn sscafbh(&mut self) -> SscafbhW<GtssrSpec> {
        SscafbhW::new(self, 11)
    }
    ///Bit 12 - GTIOCnB Pin Rising Input during GTIOCnA Value Low Source Counter Start Enable
    #[inline(always)]
    pub fn sscbral(&mut self) -> SscbralW<GtssrSpec> {
        SscbralW::new(self, 12)
    }
    ///Bit 13 - GTIOCnB Pin Rising Input during GTIOCnA Value High Source Counter Start Enable
    #[inline(always)]
    pub fn sscbrah(&mut self) -> SscbrahW<GtssrSpec> {
        SscbrahW::new(self, 13)
    }
    ///Bit 14 - GTIOCnB Pin Falling Input during GTIOCnA Value Low Source Counter Start Enable
    #[inline(always)]
    pub fn sscbfal(&mut self) -> SscbfalW<GtssrSpec> {
        SscbfalW::new(self, 14)
    }
    ///Bit 15 - GTIOCnB Pin Falling Input during GTIOCnA Value High Source Counter Start Enable
    #[inline(always)]
    pub fn sscbfah(&mut self) -> SscbfahW<GtssrSpec> {
        SscbfahW::new(self, 15)
    }
    ///Bit 16 - ELC_GPTA Event Source Counter Start Enable
    #[inline(always)]
    pub fn sselca(&mut self) -> SselcaW<GtssrSpec> {
        SselcaW::new(self, 16)
    }
    ///Bit 17 - ELC_GPTB Event Source Counter Start Enable
    #[inline(always)]
    pub fn sselcb(&mut self) -> SselcbW<GtssrSpec> {
        SselcbW::new(self, 17)
    }
    ///Bit 18 - ELC_GPTC Event Source Counter Start Enable
    #[inline(always)]
    pub fn sselcc(&mut self) -> SselccW<GtssrSpec> {
        SselccW::new(self, 18)
    }
    ///Bit 19 - ELC_GPTD Event Source Counter Start Enable
    #[inline(always)]
    pub fn sselcd(&mut self) -> SselcdW<GtssrSpec> {
        SselcdW::new(self, 19)
    }
    ///Bit 20 - ELC_GPTE Event Source Counter Start Enable
    #[inline(always)]
    pub fn sselce(&mut self) -> SselceW<GtssrSpec> {
        SselceW::new(self, 20)
    }
    ///Bit 21 - ELC_GPTF Event Source Counter Start Enable
    #[inline(always)]
    pub fn sselcf(&mut self) -> SselcfW<GtssrSpec> {
        SselcfW::new(self, 21)
    }
    ///Bit 22 - ELC_GPTG Event Source Counter Start Enable
    #[inline(always)]
    pub fn sselcg(&mut self) -> SselcgW<GtssrSpec> {
        SselcgW::new(self, 22)
    }
    ///Bit 23 - ELC_GPTH Event Source Counter Start Enable
    #[inline(always)]
    pub fn sselch(&mut self) -> SselchW<GtssrSpec> {
        SselchW::new(self, 23)
    }
    ///Bit 31 - Software Source Counter Start Enable
    #[inline(always)]
    pub fn cstrt(&mut self) -> CstrtW<GtssrSpec> {
        CstrtW::new(self, 31)
    }
}
/**General PWM Timer Start Source Select Register

You can [`read`](crate::Reg::read) this register and get [`gtssr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtssr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GtssrSpec;
impl crate::RegisterSpec for GtssrSpec {
    type Ux = u32;
}
///`read()` method returns [`gtssr::R`](R) reader structure
impl crate::Readable for GtssrSpec {}
///`write(|w| ..)` method takes [`gtssr::W`](W) writer structure
impl crate::Writable for GtssrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTSSR to value 0
impl crate::Resettable for GtssrSpec {}
