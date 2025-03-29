///Register `GTICASR` reader
pub type R = crate::R<GticasrSpec>;
///Register `GTICASR` writer
pub type W = crate::W<GticasrSpec>;
/**GTETRGA Pin Rising Input Source GTCCRA Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Asgtrgar {
    ///0: GTCCRA input capture disabled on the rising edge of GTETRGA input
    _0 = 0,
    ///1: GTCCRA input capture enabled on the rising edge of GTETRGA input
    _1 = 1,
}
impl From<Asgtrgar> for bool {
    #[inline(always)]
    fn from(variant: Asgtrgar) -> Self {
        variant as u8 != 0
    }
}
///Field `ASGTRGAR` reader - GTETRGA Pin Rising Input Source GTCCRA Input Capture Enable
pub type AsgtrgarR = crate::BitReader<Asgtrgar>;
impl AsgtrgarR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Asgtrgar {
        match self.bits {
            false => Asgtrgar::_0,
            true => Asgtrgar::_1,
        }
    }
    ///GTCCRA input capture disabled on the rising edge of GTETRGA input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Asgtrgar::_0
    }
    ///GTCCRA input capture enabled on the rising edge of GTETRGA input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Asgtrgar::_1
    }
}
///Field `ASGTRGAR` writer - GTETRGA Pin Rising Input Source GTCCRA Input Capture Enable
pub type AsgtrgarW<'a, REG> = crate::BitWriter<'a, REG, Asgtrgar>;
impl<'a, REG> AsgtrgarW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///GTCCRA input capture disabled on the rising edge of GTETRGA input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Asgtrgar::_0)
    }
    ///GTCCRA input capture enabled on the rising edge of GTETRGA input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Asgtrgar::_1)
    }
}
/**GTETRGA Pin Falling Input Source GTCCRA Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Asgtrgaf {
    ///0: GTCCRA input capture disabled on the falling edge of GTETRGA input
    _0 = 0,
    ///1: GTCCRA input capture enabled on the falling edge of GTETRGA input
    _1 = 1,
}
impl From<Asgtrgaf> for bool {
    #[inline(always)]
    fn from(variant: Asgtrgaf) -> Self {
        variant as u8 != 0
    }
}
///Field `ASGTRGAF` reader - GTETRGA Pin Falling Input Source GTCCRA Input Capture Enable
pub type AsgtrgafR = crate::BitReader<Asgtrgaf>;
impl AsgtrgafR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Asgtrgaf {
        match self.bits {
            false => Asgtrgaf::_0,
            true => Asgtrgaf::_1,
        }
    }
    ///GTCCRA input capture disabled on the falling edge of GTETRGA input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Asgtrgaf::_0
    }
    ///GTCCRA input capture enabled on the falling edge of GTETRGA input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Asgtrgaf::_1
    }
}
///Field `ASGTRGAF` writer - GTETRGA Pin Falling Input Source GTCCRA Input Capture Enable
pub type AsgtrgafW<'a, REG> = crate::BitWriter<'a, REG, Asgtrgaf>;
impl<'a, REG> AsgtrgafW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///GTCCRA input capture disabled on the falling edge of GTETRGA input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Asgtrgaf::_0)
    }
    ///GTCCRA input capture enabled on the falling edge of GTETRGA input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Asgtrgaf::_1)
    }
}
/**GTETRGB Pin Rising Input Source GTCCRA Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Asgtrgbr {
    ///0: GTCCRA input capture disabled on the rising edge of GTETRGB input
    _0 = 0,
    ///1: GTCCRA input capture enabled on the rising edge of GTETRGB input
    _1 = 1,
}
impl From<Asgtrgbr> for bool {
    #[inline(always)]
    fn from(variant: Asgtrgbr) -> Self {
        variant as u8 != 0
    }
}
///Field `ASGTRGBR` reader - GTETRGB Pin Rising Input Source GTCCRA Input Capture Enable
pub type AsgtrgbrR = crate::BitReader<Asgtrgbr>;
impl AsgtrgbrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Asgtrgbr {
        match self.bits {
            false => Asgtrgbr::_0,
            true => Asgtrgbr::_1,
        }
    }
    ///GTCCRA input capture disabled on the rising edge of GTETRGB input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Asgtrgbr::_0
    }
    ///GTCCRA input capture enabled on the rising edge of GTETRGB input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Asgtrgbr::_1
    }
}
///Field `ASGTRGBR` writer - GTETRGB Pin Rising Input Source GTCCRA Input Capture Enable
pub type AsgtrgbrW<'a, REG> = crate::BitWriter<'a, REG, Asgtrgbr>;
impl<'a, REG> AsgtrgbrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///GTCCRA input capture disabled on the rising edge of GTETRGB input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Asgtrgbr::_0)
    }
    ///GTCCRA input capture enabled on the rising edge of GTETRGB input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Asgtrgbr::_1)
    }
}
/**GTETRGB Pin Falling Input Source GTCCRA Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Asgtrgbf {
    ///0: GTCCRA input capture disabled on the falling edge of GTETRGB input
    _0 = 0,
    ///1: GTCCRA input capture enabled on the falling edge of GTETRGB input
    _1 = 1,
}
impl From<Asgtrgbf> for bool {
    #[inline(always)]
    fn from(variant: Asgtrgbf) -> Self {
        variant as u8 != 0
    }
}
///Field `ASGTRGBF` reader - GTETRGB Pin Falling Input Source GTCCRA Input Capture Enable
pub type AsgtrgbfR = crate::BitReader<Asgtrgbf>;
impl AsgtrgbfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Asgtrgbf {
        match self.bits {
            false => Asgtrgbf::_0,
            true => Asgtrgbf::_1,
        }
    }
    ///GTCCRA input capture disabled on the falling edge of GTETRGB input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Asgtrgbf::_0
    }
    ///GTCCRA input capture enabled on the falling edge of GTETRGB input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Asgtrgbf::_1
    }
}
///Field `ASGTRGBF` writer - GTETRGB Pin Falling Input Source GTCCRA Input Capture Enable
pub type AsgtrgbfW<'a, REG> = crate::BitWriter<'a, REG, Asgtrgbf>;
impl<'a, REG> AsgtrgbfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///GTCCRA input capture disabled on the falling edge of GTETRGB input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Asgtrgbf::_0)
    }
    ///GTCCRA input capture enabled on the falling edge of GTETRGB input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Asgtrgbf::_1)
    }
}
/**GTETRGC Pin Rising Input Source GTCCRA Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Asgtrgcr {
    ///0: GTCCRA input capture disabled on the rising edge of GTETRGC input
    _0 = 0,
    ///1: GTCCRA input capture enabled on the rising edge of GTETRGC input
    _1 = 1,
}
impl From<Asgtrgcr> for bool {
    #[inline(always)]
    fn from(variant: Asgtrgcr) -> Self {
        variant as u8 != 0
    }
}
///Field `ASGTRGCR` reader - GTETRGC Pin Rising Input Source GTCCRA Input Capture Enable
pub type AsgtrgcrR = crate::BitReader<Asgtrgcr>;
impl AsgtrgcrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Asgtrgcr {
        match self.bits {
            false => Asgtrgcr::_0,
            true => Asgtrgcr::_1,
        }
    }
    ///GTCCRA input capture disabled on the rising edge of GTETRGC input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Asgtrgcr::_0
    }
    ///GTCCRA input capture enabled on the rising edge of GTETRGC input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Asgtrgcr::_1
    }
}
///Field `ASGTRGCR` writer - GTETRGC Pin Rising Input Source GTCCRA Input Capture Enable
pub type AsgtrgcrW<'a, REG> = crate::BitWriter<'a, REG, Asgtrgcr>;
impl<'a, REG> AsgtrgcrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///GTCCRA input capture disabled on the rising edge of GTETRGC input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Asgtrgcr::_0)
    }
    ///GTCCRA input capture enabled on the rising edge of GTETRGC input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Asgtrgcr::_1)
    }
}
/**GTETRGC Pin Falling Input Source GTCCRA Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Asgtrgcf {
    ///0: GTCCRA input capture disabled on the falling edge of GTETRGC input
    _0 = 0,
    ///1: GTCCRA input capture enabled on the falling edge of GTETRGC input
    _1 = 1,
}
impl From<Asgtrgcf> for bool {
    #[inline(always)]
    fn from(variant: Asgtrgcf) -> Self {
        variant as u8 != 0
    }
}
///Field `ASGTRGCF` reader - GTETRGC Pin Falling Input Source GTCCRA Input Capture Enable
pub type AsgtrgcfR = crate::BitReader<Asgtrgcf>;
impl AsgtrgcfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Asgtrgcf {
        match self.bits {
            false => Asgtrgcf::_0,
            true => Asgtrgcf::_1,
        }
    }
    ///GTCCRA input capture disabled on the falling edge of GTETRGC input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Asgtrgcf::_0
    }
    ///GTCCRA input capture enabled on the falling edge of GTETRGC input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Asgtrgcf::_1
    }
}
///Field `ASGTRGCF` writer - GTETRGC Pin Falling Input Source GTCCRA Input Capture Enable
pub type AsgtrgcfW<'a, REG> = crate::BitWriter<'a, REG, Asgtrgcf>;
impl<'a, REG> AsgtrgcfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///GTCCRA input capture disabled on the falling edge of GTETRGC input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Asgtrgcf::_0)
    }
    ///GTCCRA input capture enabled on the falling edge of GTETRGC input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Asgtrgcf::_1)
    }
}
/**GTETRGD Pin Rising Input Source GTCCRA Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Asgtrgdr {
    ///0: GTCCRA input capture disabled on the rising edge of GTETRGD input
    _0 = 0,
    ///1: GTCCRA input capture enabled on the rising edge of GTETRGD input
    _1 = 1,
}
impl From<Asgtrgdr> for bool {
    #[inline(always)]
    fn from(variant: Asgtrgdr) -> Self {
        variant as u8 != 0
    }
}
///Field `ASGTRGDR` reader - GTETRGD Pin Rising Input Source GTCCRA Input Capture Enable
pub type AsgtrgdrR = crate::BitReader<Asgtrgdr>;
impl AsgtrgdrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Asgtrgdr {
        match self.bits {
            false => Asgtrgdr::_0,
            true => Asgtrgdr::_1,
        }
    }
    ///GTCCRA input capture disabled on the rising edge of GTETRGD input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Asgtrgdr::_0
    }
    ///GTCCRA input capture enabled on the rising edge of GTETRGD input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Asgtrgdr::_1
    }
}
///Field `ASGTRGDR` writer - GTETRGD Pin Rising Input Source GTCCRA Input Capture Enable
pub type AsgtrgdrW<'a, REG> = crate::BitWriter<'a, REG, Asgtrgdr>;
impl<'a, REG> AsgtrgdrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///GTCCRA input capture disabled on the rising edge of GTETRGD input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Asgtrgdr::_0)
    }
    ///GTCCRA input capture enabled on the rising edge of GTETRGD input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Asgtrgdr::_1)
    }
}
/**GTETRGD Pin Falling Input Source GTCCRA Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Asgtrgdf {
    ///0: GTCCRA input capture disabled on the falling edge of GTETRGD input
    _0 = 0,
    ///1: GTCCRA input capture enabled on the falling edge of GTETRGD input
    _1 = 1,
}
impl From<Asgtrgdf> for bool {
    #[inline(always)]
    fn from(variant: Asgtrgdf) -> Self {
        variant as u8 != 0
    }
}
///Field `ASGTRGDF` reader - GTETRGD Pin Falling Input Source GTCCRA Input Capture Enable
pub type AsgtrgdfR = crate::BitReader<Asgtrgdf>;
impl AsgtrgdfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Asgtrgdf {
        match self.bits {
            false => Asgtrgdf::_0,
            true => Asgtrgdf::_1,
        }
    }
    ///GTCCRA input capture disabled on the falling edge of GTETRGD input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Asgtrgdf::_0
    }
    ///GTCCRA input capture enabled on the falling edge of GTETRGD input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Asgtrgdf::_1
    }
}
///Field `ASGTRGDF` writer - GTETRGD Pin Falling Input Source GTCCRA Input Capture Enable
pub type AsgtrgdfW<'a, REG> = crate::BitWriter<'a, REG, Asgtrgdf>;
impl<'a, REG> AsgtrgdfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///GTCCRA input capture disabled on the falling edge of GTETRGD input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Asgtrgdf::_0)
    }
    ///GTCCRA input capture enabled on the falling edge of GTETRGD input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Asgtrgdf::_1)
    }
}
/**GTIOCnA Pin Rising Input during GTIOCnB Value Low Source GTCCRA Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ascarbl {
    ///0: GTCCRA input capture disabled on the rising edge of GTIOCnA input when GTIOCnB input is 0
    _0 = 0,
    ///1: GTCCRA input capture enabled on the rising edge of GTIOCnA input when GTIOCnB input is 0
    _1 = 1,
}
impl From<Ascarbl> for bool {
    #[inline(always)]
    fn from(variant: Ascarbl) -> Self {
        variant as u8 != 0
    }
}
///Field `ASCARBL` reader - GTIOCnA Pin Rising Input during GTIOCnB Value Low Source GTCCRA Input Capture Enable
pub type AscarblR = crate::BitReader<Ascarbl>;
impl AscarblR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ascarbl {
        match self.bits {
            false => Ascarbl::_0,
            true => Ascarbl::_1,
        }
    }
    ///GTCCRA input capture disabled on the rising edge of GTIOCnA input when GTIOCnB input is 0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ascarbl::_0
    }
    ///GTCCRA input capture enabled on the rising edge of GTIOCnA input when GTIOCnB input is 0
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ascarbl::_1
    }
}
///Field `ASCARBL` writer - GTIOCnA Pin Rising Input during GTIOCnB Value Low Source GTCCRA Input Capture Enable
pub type AscarblW<'a, REG> = crate::BitWriter<'a, REG, Ascarbl>;
impl<'a, REG> AscarblW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///GTCCRA input capture disabled on the rising edge of GTIOCnA input when GTIOCnB input is 0
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ascarbl::_0)
    }
    ///GTCCRA input capture enabled on the rising edge of GTIOCnA input when GTIOCnB input is 0
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ascarbl::_1)
    }
}
/**GTIOCnA Pin Rising Input during GTIOCnB Value High Source GTCCRA Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ascarbh {
    ///0: GTCCRA input capture disabled on the rising edge of GTIOCnA input when GTIOCnB input is 1
    _0 = 0,
    ///1: GTCCRA input capture enabled on the rising edge of GTIOCnA input when GTIOCnB input is 1
    _1 = 1,
}
impl From<Ascarbh> for bool {
    #[inline(always)]
    fn from(variant: Ascarbh) -> Self {
        variant as u8 != 0
    }
}
///Field `ASCARBH` reader - GTIOCnA Pin Rising Input during GTIOCnB Value High Source GTCCRA Input Capture Enable
pub type AscarbhR = crate::BitReader<Ascarbh>;
impl AscarbhR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ascarbh {
        match self.bits {
            false => Ascarbh::_0,
            true => Ascarbh::_1,
        }
    }
    ///GTCCRA input capture disabled on the rising edge of GTIOCnA input when GTIOCnB input is 1
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ascarbh::_0
    }
    ///GTCCRA input capture enabled on the rising edge of GTIOCnA input when GTIOCnB input is 1
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ascarbh::_1
    }
}
///Field `ASCARBH` writer - GTIOCnA Pin Rising Input during GTIOCnB Value High Source GTCCRA Input Capture Enable
pub type AscarbhW<'a, REG> = crate::BitWriter<'a, REG, Ascarbh>;
impl<'a, REG> AscarbhW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///GTCCRA input capture disabled on the rising edge of GTIOCnA input when GTIOCnB input is 1
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ascarbh::_0)
    }
    ///GTCCRA input capture enabled on the rising edge of GTIOCnA input when GTIOCnB input is 1
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ascarbh::_1)
    }
}
/**GTIOCnA Pin Falling Input during GTIOCnB Value Low Source GTCCRA Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ascafbl {
    ///0: GTCCRA input capture disabled on the falling edge of GTIOCnA input when GTIOCnB input is 0
    _0 = 0,
    ///1: GTCCRA input capture enabled on the falling edge of GTIOCnA input when GTIOCnB input is 0
    _1 = 1,
}
impl From<Ascafbl> for bool {
    #[inline(always)]
    fn from(variant: Ascafbl) -> Self {
        variant as u8 != 0
    }
}
///Field `ASCAFBL` reader - GTIOCnA Pin Falling Input during GTIOCnB Value Low Source GTCCRA Input Capture Enable
pub type AscafblR = crate::BitReader<Ascafbl>;
impl AscafblR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ascafbl {
        match self.bits {
            false => Ascafbl::_0,
            true => Ascafbl::_1,
        }
    }
    ///GTCCRA input capture disabled on the falling edge of GTIOCnA input when GTIOCnB input is 0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ascafbl::_0
    }
    ///GTCCRA input capture enabled on the falling edge of GTIOCnA input when GTIOCnB input is 0
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ascafbl::_1
    }
}
///Field `ASCAFBL` writer - GTIOCnA Pin Falling Input during GTIOCnB Value Low Source GTCCRA Input Capture Enable
pub type AscafblW<'a, REG> = crate::BitWriter<'a, REG, Ascafbl>;
impl<'a, REG> AscafblW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///GTCCRA input capture disabled on the falling edge of GTIOCnA input when GTIOCnB input is 0
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ascafbl::_0)
    }
    ///GTCCRA input capture enabled on the falling edge of GTIOCnA input when GTIOCnB input is 0
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ascafbl::_1)
    }
}
/**GTIOCnA Pin Falling Input during GTIOCnB Value High Source GTCCRA Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ascafbh {
    ///0: GTCCRA input capture disabled on the falling edge of GTIOCnA input when GTIOCnB input is 1
    _0 = 0,
    ///1: GTCCRA input capture enabled on the falling edge of GTIOCnA input when GTIOCnB input is 1
    _1 = 1,
}
impl From<Ascafbh> for bool {
    #[inline(always)]
    fn from(variant: Ascafbh) -> Self {
        variant as u8 != 0
    }
}
///Field `ASCAFBH` reader - GTIOCnA Pin Falling Input during GTIOCnB Value High Source GTCCRA Input Capture Enable
pub type AscafbhR = crate::BitReader<Ascafbh>;
impl AscafbhR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ascafbh {
        match self.bits {
            false => Ascafbh::_0,
            true => Ascafbh::_1,
        }
    }
    ///GTCCRA input capture disabled on the falling edge of GTIOCnA input when GTIOCnB input is 1
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ascafbh::_0
    }
    ///GTCCRA input capture enabled on the falling edge of GTIOCnA input when GTIOCnB input is 1
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ascafbh::_1
    }
}
///Field `ASCAFBH` writer - GTIOCnA Pin Falling Input during GTIOCnB Value High Source GTCCRA Input Capture Enable
pub type AscafbhW<'a, REG> = crate::BitWriter<'a, REG, Ascafbh>;
impl<'a, REG> AscafbhW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///GTCCRA input capture disabled on the falling edge of GTIOCnA input when GTIOCnB input is 1
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ascafbh::_0)
    }
    ///GTCCRA input capture enabled on the falling edge of GTIOCnA input when GTIOCnB input is 1
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ascafbh::_1)
    }
}
/**GTIOCnB Pin Rising Input during GTIOCnA Value Low Source GTCCRA Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ascbral {
    ///0: GTCCRA input capture disabled on the rising edge of GTIOCnB input when GTIOCnA input is 0
    _0 = 0,
    ///1: GTCCRA input capture enabled on the rising edge of GTIOCnB input when GTIOCnA input is 0
    _1 = 1,
}
impl From<Ascbral> for bool {
    #[inline(always)]
    fn from(variant: Ascbral) -> Self {
        variant as u8 != 0
    }
}
///Field `ASCBRAL` reader - GTIOCnB Pin Rising Input during GTIOCnA Value Low Source GTCCRA Input Capture Enable
pub type AscbralR = crate::BitReader<Ascbral>;
impl AscbralR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ascbral {
        match self.bits {
            false => Ascbral::_0,
            true => Ascbral::_1,
        }
    }
    ///GTCCRA input capture disabled on the rising edge of GTIOCnB input when GTIOCnA input is 0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ascbral::_0
    }
    ///GTCCRA input capture enabled on the rising edge of GTIOCnB input when GTIOCnA input is 0
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ascbral::_1
    }
}
///Field `ASCBRAL` writer - GTIOCnB Pin Rising Input during GTIOCnA Value Low Source GTCCRA Input Capture Enable
pub type AscbralW<'a, REG> = crate::BitWriter<'a, REG, Ascbral>;
impl<'a, REG> AscbralW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///GTCCRA input capture disabled on the rising edge of GTIOCnB input when GTIOCnA input is 0
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ascbral::_0)
    }
    ///GTCCRA input capture enabled on the rising edge of GTIOCnB input when GTIOCnA input is 0
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ascbral::_1)
    }
}
/**GTIOCnB Pin Rising Input during GTIOCnA Value High Source GTCCRA Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ascbrah {
    ///0: GTCCRA input capture disabled on the rising edge of GTIOCnB input when GTIOCnA input is 1
    _0 = 0,
    ///1: GTCCRA input capture enabled on the rising edge of GTIOCnB input when GTIOCnA input is 1
    _1 = 1,
}
impl From<Ascbrah> for bool {
    #[inline(always)]
    fn from(variant: Ascbrah) -> Self {
        variant as u8 != 0
    }
}
///Field `ASCBRAH` reader - GTIOCnB Pin Rising Input during GTIOCnA Value High Source GTCCRA Input Capture Enable
pub type AscbrahR = crate::BitReader<Ascbrah>;
impl AscbrahR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ascbrah {
        match self.bits {
            false => Ascbrah::_0,
            true => Ascbrah::_1,
        }
    }
    ///GTCCRA input capture disabled on the rising edge of GTIOCnB input when GTIOCnA input is 1
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ascbrah::_0
    }
    ///GTCCRA input capture enabled on the rising edge of GTIOCnB input when GTIOCnA input is 1
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ascbrah::_1
    }
}
///Field `ASCBRAH` writer - GTIOCnB Pin Rising Input during GTIOCnA Value High Source GTCCRA Input Capture Enable
pub type AscbrahW<'a, REG> = crate::BitWriter<'a, REG, Ascbrah>;
impl<'a, REG> AscbrahW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///GTCCRA input capture disabled on the rising edge of GTIOCnB input when GTIOCnA input is 1
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ascbrah::_0)
    }
    ///GTCCRA input capture enabled on the rising edge of GTIOCnB input when GTIOCnA input is 1
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ascbrah::_1)
    }
}
/**GTIOCnB Pin Falling Input during GTIOCnA Value Low Source GTCCRA Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ascbfal {
    ///0: GTCCRA input capture disabled on the falling edge of GTIOCnB input when GTIOCnA input is 0
    _0 = 0,
    ///1: GTCCRA input capture enabled on the falling edge of GTIOCnB input when GTIOCnA input is 0
    _1 = 1,
}
impl From<Ascbfal> for bool {
    #[inline(always)]
    fn from(variant: Ascbfal) -> Self {
        variant as u8 != 0
    }
}
///Field `ASCBFAL` reader - GTIOCnB Pin Falling Input during GTIOCnA Value Low Source GTCCRA Input Capture Enable
pub type AscbfalR = crate::BitReader<Ascbfal>;
impl AscbfalR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ascbfal {
        match self.bits {
            false => Ascbfal::_0,
            true => Ascbfal::_1,
        }
    }
    ///GTCCRA input capture disabled on the falling edge of GTIOCnB input when GTIOCnA input is 0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ascbfal::_0
    }
    ///GTCCRA input capture enabled on the falling edge of GTIOCnB input when GTIOCnA input is 0
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ascbfal::_1
    }
}
///Field `ASCBFAL` writer - GTIOCnB Pin Falling Input during GTIOCnA Value Low Source GTCCRA Input Capture Enable
pub type AscbfalW<'a, REG> = crate::BitWriter<'a, REG, Ascbfal>;
impl<'a, REG> AscbfalW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///GTCCRA input capture disabled on the falling edge of GTIOCnB input when GTIOCnA input is 0
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ascbfal::_0)
    }
    ///GTCCRA input capture enabled on the falling edge of GTIOCnB input when GTIOCnA input is 0
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ascbfal::_1)
    }
}
/**GTIOCnB Pin Falling Input during GTIOCnA Value High Source GTCCRA Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ascbfah {
    ///0: GTCCRA input capture disabled on the falling edge of GTIOCnB input when GTIOCnA input is 1
    _0 = 0,
    ///1: GTCCRA input capture enabled on the falling edge of GTIOCnB input when GTIOCnA input is 1
    _1 = 1,
}
impl From<Ascbfah> for bool {
    #[inline(always)]
    fn from(variant: Ascbfah) -> Self {
        variant as u8 != 0
    }
}
///Field `ASCBFAH` reader - GTIOCnB Pin Falling Input during GTIOCnA Value High Source GTCCRA Input Capture Enable
pub type AscbfahR = crate::BitReader<Ascbfah>;
impl AscbfahR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ascbfah {
        match self.bits {
            false => Ascbfah::_0,
            true => Ascbfah::_1,
        }
    }
    ///GTCCRA input capture disabled on the falling edge of GTIOCnB input when GTIOCnA input is 1
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ascbfah::_0
    }
    ///GTCCRA input capture enabled on the falling edge of GTIOCnB input when GTIOCnA input is 1
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ascbfah::_1
    }
}
///Field `ASCBFAH` writer - GTIOCnB Pin Falling Input during GTIOCnA Value High Source GTCCRA Input Capture Enable
pub type AscbfahW<'a, REG> = crate::BitWriter<'a, REG, Ascbfah>;
impl<'a, REG> AscbfahW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///GTCCRA input capture disabled on the falling edge of GTIOCnB input when GTIOCnA input is 1
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ascbfah::_0)
    }
    ///GTCCRA input capture enabled on the falling edge of GTIOCnB input when GTIOCnA input is 1
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ascbfah::_1)
    }
}
/**ELC_GPTA Event Source GTCCRA Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aselca {
    ///0: GTCCRA input capture disabled at the ELC_GPTA input
    _0 = 0,
    ///1: GTCCRA input capture enabled at the ELC_GPTA input
    _1 = 1,
}
impl From<Aselca> for bool {
    #[inline(always)]
    fn from(variant: Aselca) -> Self {
        variant as u8 != 0
    }
}
///Field `ASELCA` reader - ELC_GPTA Event Source GTCCRA Input Capture Enable
pub type AselcaR = crate::BitReader<Aselca>;
impl AselcaR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Aselca {
        match self.bits {
            false => Aselca::_0,
            true => Aselca::_1,
        }
    }
    ///GTCCRA input capture disabled at the ELC_GPTA input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Aselca::_0
    }
    ///GTCCRA input capture enabled at the ELC_GPTA input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Aselca::_1
    }
}
///Field `ASELCA` writer - ELC_GPTA Event Source GTCCRA Input Capture Enable
pub type AselcaW<'a, REG> = crate::BitWriter<'a, REG, Aselca>;
impl<'a, REG> AselcaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///GTCCRA input capture disabled at the ELC_GPTA input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Aselca::_0)
    }
    ///GTCCRA input capture enabled at the ELC_GPTA input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Aselca::_1)
    }
}
/**ELC_GPTB Event Source GTCCRA Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aselcb {
    ///0: GTCCRA input capture disabled at the ELC_GPTB input
    _0 = 0,
    ///1: GTCCRA input capture enabled at the ELC_GPTB input
    _1 = 1,
}
impl From<Aselcb> for bool {
    #[inline(always)]
    fn from(variant: Aselcb) -> Self {
        variant as u8 != 0
    }
}
///Field `ASELCB` reader - ELC_GPTB Event Source GTCCRA Input Capture Enable
pub type AselcbR = crate::BitReader<Aselcb>;
impl AselcbR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Aselcb {
        match self.bits {
            false => Aselcb::_0,
            true => Aselcb::_1,
        }
    }
    ///GTCCRA input capture disabled at the ELC_GPTB input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Aselcb::_0
    }
    ///GTCCRA input capture enabled at the ELC_GPTB input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Aselcb::_1
    }
}
///Field `ASELCB` writer - ELC_GPTB Event Source GTCCRA Input Capture Enable
pub type AselcbW<'a, REG> = crate::BitWriter<'a, REG, Aselcb>;
impl<'a, REG> AselcbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///GTCCRA input capture disabled at the ELC_GPTB input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Aselcb::_0)
    }
    ///GTCCRA input capture enabled at the ELC_GPTB input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Aselcb::_1)
    }
}
/**ELC_GPTC Event Source GTCCRA Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aselcc {
    ///0: GTCCRA input capture disabled at the ELC_GPTC input
    _0 = 0,
    ///1: GTCCRA input capture enabled at the ELC_GPTC input
    _1 = 1,
}
impl From<Aselcc> for bool {
    #[inline(always)]
    fn from(variant: Aselcc) -> Self {
        variant as u8 != 0
    }
}
///Field `ASELCC` reader - ELC_GPTC Event Source GTCCRA Input Capture Enable
pub type AselccR = crate::BitReader<Aselcc>;
impl AselccR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Aselcc {
        match self.bits {
            false => Aselcc::_0,
            true => Aselcc::_1,
        }
    }
    ///GTCCRA input capture disabled at the ELC_GPTC input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Aselcc::_0
    }
    ///GTCCRA input capture enabled at the ELC_GPTC input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Aselcc::_1
    }
}
///Field `ASELCC` writer - ELC_GPTC Event Source GTCCRA Input Capture Enable
pub type AselccW<'a, REG> = crate::BitWriter<'a, REG, Aselcc>;
impl<'a, REG> AselccW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///GTCCRA input capture disabled at the ELC_GPTC input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Aselcc::_0)
    }
    ///GTCCRA input capture enabled at the ELC_GPTC input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Aselcc::_1)
    }
}
/**ELC_GPTD Event Source GTCCRA Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aselcd {
    ///0: GTCCRA input capture disabled at the ELC_GPTD input
    _0 = 0,
    ///1: GTCCRA input capture enabled at the ELC_GPTD input
    _1 = 1,
}
impl From<Aselcd> for bool {
    #[inline(always)]
    fn from(variant: Aselcd) -> Self {
        variant as u8 != 0
    }
}
///Field `ASELCD` reader - ELC_GPTD Event Source GTCCRA Input Capture Enable
pub type AselcdR = crate::BitReader<Aselcd>;
impl AselcdR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Aselcd {
        match self.bits {
            false => Aselcd::_0,
            true => Aselcd::_1,
        }
    }
    ///GTCCRA input capture disabled at the ELC_GPTD input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Aselcd::_0
    }
    ///GTCCRA input capture enabled at the ELC_GPTD input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Aselcd::_1
    }
}
///Field `ASELCD` writer - ELC_GPTD Event Source GTCCRA Input Capture Enable
pub type AselcdW<'a, REG> = crate::BitWriter<'a, REG, Aselcd>;
impl<'a, REG> AselcdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///GTCCRA input capture disabled at the ELC_GPTD input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Aselcd::_0)
    }
    ///GTCCRA input capture enabled at the ELC_GPTD input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Aselcd::_1)
    }
}
/**ELC_GPTE Event Source GTCCRA Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aselce {
    ///0: GTCCRA input capture disabled at the ELC_GPTE input
    _0 = 0,
    ///1: GTCCRA input capture enabled at the ELC_GPTE input
    _1 = 1,
}
impl From<Aselce> for bool {
    #[inline(always)]
    fn from(variant: Aselce) -> Self {
        variant as u8 != 0
    }
}
///Field `ASELCE` reader - ELC_GPTE Event Source GTCCRA Input Capture Enable
pub type AselceR = crate::BitReader<Aselce>;
impl AselceR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Aselce {
        match self.bits {
            false => Aselce::_0,
            true => Aselce::_1,
        }
    }
    ///GTCCRA input capture disabled at the ELC_GPTE input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Aselce::_0
    }
    ///GTCCRA input capture enabled at the ELC_GPTE input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Aselce::_1
    }
}
///Field `ASELCE` writer - ELC_GPTE Event Source GTCCRA Input Capture Enable
pub type AselceW<'a, REG> = crate::BitWriter<'a, REG, Aselce>;
impl<'a, REG> AselceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///GTCCRA input capture disabled at the ELC_GPTE input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Aselce::_0)
    }
    ///GTCCRA input capture enabled at the ELC_GPTE input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Aselce::_1)
    }
}
/**ELC_GPTF Event Source GTCCRA Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aselcf {
    ///0: GTCCRA input capture disabled at the ELC_GPTF input
    _0 = 0,
    ///1: GTCCRA input capture enabled at the ELC_GPTF input
    _1 = 1,
}
impl From<Aselcf> for bool {
    #[inline(always)]
    fn from(variant: Aselcf) -> Self {
        variant as u8 != 0
    }
}
///Field `ASELCF` reader - ELC_GPTF Event Source GTCCRA Input Capture Enable
pub type AselcfR = crate::BitReader<Aselcf>;
impl AselcfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Aselcf {
        match self.bits {
            false => Aselcf::_0,
            true => Aselcf::_1,
        }
    }
    ///GTCCRA input capture disabled at the ELC_GPTF input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Aselcf::_0
    }
    ///GTCCRA input capture enabled at the ELC_GPTF input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Aselcf::_1
    }
}
///Field `ASELCF` writer - ELC_GPTF Event Source GTCCRA Input Capture Enable
pub type AselcfW<'a, REG> = crate::BitWriter<'a, REG, Aselcf>;
impl<'a, REG> AselcfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///GTCCRA input capture disabled at the ELC_GPTF input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Aselcf::_0)
    }
    ///GTCCRA input capture enabled at the ELC_GPTF input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Aselcf::_1)
    }
}
/**ELC_GPTG Event Source GTCCRA Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aselcg {
    ///0: GTCCRA input capture disabled at the ELC_GPTG input
    _0 = 0,
    ///1: GTCCRA input capture enabled at the ELC_GPTG input
    _1 = 1,
}
impl From<Aselcg> for bool {
    #[inline(always)]
    fn from(variant: Aselcg) -> Self {
        variant as u8 != 0
    }
}
///Field `ASELCG` reader - ELC_GPTG Event Source GTCCRA Input Capture Enable
pub type AselcgR = crate::BitReader<Aselcg>;
impl AselcgR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Aselcg {
        match self.bits {
            false => Aselcg::_0,
            true => Aselcg::_1,
        }
    }
    ///GTCCRA input capture disabled at the ELC_GPTG input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Aselcg::_0
    }
    ///GTCCRA input capture enabled at the ELC_GPTG input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Aselcg::_1
    }
}
///Field `ASELCG` writer - ELC_GPTG Event Source GTCCRA Input Capture Enable
pub type AselcgW<'a, REG> = crate::BitWriter<'a, REG, Aselcg>;
impl<'a, REG> AselcgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///GTCCRA input capture disabled at the ELC_GPTG input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Aselcg::_0)
    }
    ///GTCCRA input capture enabled at the ELC_GPTG input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Aselcg::_1)
    }
}
/**ELC_GPTH Event Source GTCCRA Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aselch {
    ///0: GTCCRA input capture disabled at the ELC_GPTH input
    _0 = 0,
    ///1: GTCCRA input capture enabled at the ELC_GPTH input
    _1 = 1,
}
impl From<Aselch> for bool {
    #[inline(always)]
    fn from(variant: Aselch) -> Self {
        variant as u8 != 0
    }
}
///Field `ASELCH` reader - ELC_GPTH Event Source GTCCRA Input Capture Enable
pub type AselchR = crate::BitReader<Aselch>;
impl AselchR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Aselch {
        match self.bits {
            false => Aselch::_0,
            true => Aselch::_1,
        }
    }
    ///GTCCRA input capture disabled at the ELC_GPTH input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Aselch::_0
    }
    ///GTCCRA input capture enabled at the ELC_GPTH input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Aselch::_1
    }
}
///Field `ASELCH` writer - ELC_GPTH Event Source GTCCRA Input Capture Enable
pub type AselchW<'a, REG> = crate::BitWriter<'a, REG, Aselch>;
impl<'a, REG> AselchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///GTCCRA input capture disabled at the ELC_GPTH input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Aselch::_0)
    }
    ///GTCCRA input capture enabled at the ELC_GPTH input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Aselch::_1)
    }
}
impl R {
    ///Bit 0 - GTETRGA Pin Rising Input Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn asgtrgar(&self) -> AsgtrgarR {
        AsgtrgarR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - GTETRGA Pin Falling Input Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn asgtrgaf(&self) -> AsgtrgafR {
        AsgtrgafR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - GTETRGB Pin Rising Input Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn asgtrgbr(&self) -> AsgtrgbrR {
        AsgtrgbrR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - GTETRGB Pin Falling Input Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn asgtrgbf(&self) -> AsgtrgbfR {
        AsgtrgbfR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - GTETRGC Pin Rising Input Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn asgtrgcr(&self) -> AsgtrgcrR {
        AsgtrgcrR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - GTETRGC Pin Falling Input Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn asgtrgcf(&self) -> AsgtrgcfR {
        AsgtrgcfR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - GTETRGD Pin Rising Input Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn asgtrgdr(&self) -> AsgtrgdrR {
        AsgtrgdrR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - GTETRGD Pin Falling Input Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn asgtrgdf(&self) -> AsgtrgdfR {
        AsgtrgdfR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - GTIOCnA Pin Rising Input during GTIOCnB Value Low Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn ascarbl(&self) -> AscarblR {
        AscarblR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - GTIOCnA Pin Rising Input during GTIOCnB Value High Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn ascarbh(&self) -> AscarbhR {
        AscarbhR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - GTIOCnA Pin Falling Input during GTIOCnB Value Low Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn ascafbl(&self) -> AscafblR {
        AscafblR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - GTIOCnA Pin Falling Input during GTIOCnB Value High Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn ascafbh(&self) -> AscafbhR {
        AscafbhR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - GTIOCnB Pin Rising Input during GTIOCnA Value Low Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn ascbral(&self) -> AscbralR {
        AscbralR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - GTIOCnB Pin Rising Input during GTIOCnA Value High Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn ascbrah(&self) -> AscbrahR {
        AscbrahR::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - GTIOCnB Pin Falling Input during GTIOCnA Value Low Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn ascbfal(&self) -> AscbfalR {
        AscbfalR::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - GTIOCnB Pin Falling Input during GTIOCnA Value High Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn ascbfah(&self) -> AscbfahR {
        AscbfahR::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - ELC_GPTA Event Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn aselca(&self) -> AselcaR {
        AselcaR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - ELC_GPTB Event Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn aselcb(&self) -> AselcbR {
        AselcbR::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - ELC_GPTC Event Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn aselcc(&self) -> AselccR {
        AselccR::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - ELC_GPTD Event Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn aselcd(&self) -> AselcdR {
        AselcdR::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - ELC_GPTE Event Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn aselce(&self) -> AselceR {
        AselceR::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - ELC_GPTF Event Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn aselcf(&self) -> AselcfR {
        AselcfR::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - ELC_GPTG Event Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn aselcg(&self) -> AselcgR {
        AselcgR::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - ELC_GPTH Event Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn aselch(&self) -> AselchR {
        AselchR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GTICASR")
            .field("asgtrgar", &self.asgtrgar())
            .field("asgtrgaf", &self.asgtrgaf())
            .field("asgtrgbr", &self.asgtrgbr())
            .field("asgtrgbf", &self.asgtrgbf())
            .field("asgtrgcr", &self.asgtrgcr())
            .field("asgtrgcf", &self.asgtrgcf())
            .field("asgtrgdr", &self.asgtrgdr())
            .field("asgtrgdf", &self.asgtrgdf())
            .field("ascarbl", &self.ascarbl())
            .field("ascarbh", &self.ascarbh())
            .field("ascafbl", &self.ascafbl())
            .field("ascafbh", &self.ascafbh())
            .field("ascbral", &self.ascbral())
            .field("ascbrah", &self.ascbrah())
            .field("ascbfal", &self.ascbfal())
            .field("ascbfah", &self.ascbfah())
            .field("aselca", &self.aselca())
            .field("aselcb", &self.aselcb())
            .field("aselcc", &self.aselcc())
            .field("aselcd", &self.aselcd())
            .field("aselce", &self.aselce())
            .field("aselcf", &self.aselcf())
            .field("aselcg", &self.aselcg())
            .field("aselch", &self.aselch())
            .finish()
    }
}
impl W {
    ///Bit 0 - GTETRGA Pin Rising Input Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn asgtrgar(&mut self) -> AsgtrgarW<GticasrSpec> {
        AsgtrgarW::new(self, 0)
    }
    ///Bit 1 - GTETRGA Pin Falling Input Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn asgtrgaf(&mut self) -> AsgtrgafW<GticasrSpec> {
        AsgtrgafW::new(self, 1)
    }
    ///Bit 2 - GTETRGB Pin Rising Input Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn asgtrgbr(&mut self) -> AsgtrgbrW<GticasrSpec> {
        AsgtrgbrW::new(self, 2)
    }
    ///Bit 3 - GTETRGB Pin Falling Input Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn asgtrgbf(&mut self) -> AsgtrgbfW<GticasrSpec> {
        AsgtrgbfW::new(self, 3)
    }
    ///Bit 4 - GTETRGC Pin Rising Input Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn asgtrgcr(&mut self) -> AsgtrgcrW<GticasrSpec> {
        AsgtrgcrW::new(self, 4)
    }
    ///Bit 5 - GTETRGC Pin Falling Input Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn asgtrgcf(&mut self) -> AsgtrgcfW<GticasrSpec> {
        AsgtrgcfW::new(self, 5)
    }
    ///Bit 6 - GTETRGD Pin Rising Input Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn asgtrgdr(&mut self) -> AsgtrgdrW<GticasrSpec> {
        AsgtrgdrW::new(self, 6)
    }
    ///Bit 7 - GTETRGD Pin Falling Input Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn asgtrgdf(&mut self) -> AsgtrgdfW<GticasrSpec> {
        AsgtrgdfW::new(self, 7)
    }
    ///Bit 8 - GTIOCnA Pin Rising Input during GTIOCnB Value Low Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn ascarbl(&mut self) -> AscarblW<GticasrSpec> {
        AscarblW::new(self, 8)
    }
    ///Bit 9 - GTIOCnA Pin Rising Input during GTIOCnB Value High Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn ascarbh(&mut self) -> AscarbhW<GticasrSpec> {
        AscarbhW::new(self, 9)
    }
    ///Bit 10 - GTIOCnA Pin Falling Input during GTIOCnB Value Low Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn ascafbl(&mut self) -> AscafblW<GticasrSpec> {
        AscafblW::new(self, 10)
    }
    ///Bit 11 - GTIOCnA Pin Falling Input during GTIOCnB Value High Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn ascafbh(&mut self) -> AscafbhW<GticasrSpec> {
        AscafbhW::new(self, 11)
    }
    ///Bit 12 - GTIOCnB Pin Rising Input during GTIOCnA Value Low Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn ascbral(&mut self) -> AscbralW<GticasrSpec> {
        AscbralW::new(self, 12)
    }
    ///Bit 13 - GTIOCnB Pin Rising Input during GTIOCnA Value High Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn ascbrah(&mut self) -> AscbrahW<GticasrSpec> {
        AscbrahW::new(self, 13)
    }
    ///Bit 14 - GTIOCnB Pin Falling Input during GTIOCnA Value Low Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn ascbfal(&mut self) -> AscbfalW<GticasrSpec> {
        AscbfalW::new(self, 14)
    }
    ///Bit 15 - GTIOCnB Pin Falling Input during GTIOCnA Value High Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn ascbfah(&mut self) -> AscbfahW<GticasrSpec> {
        AscbfahW::new(self, 15)
    }
    ///Bit 16 - ELC_GPTA Event Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn aselca(&mut self) -> AselcaW<GticasrSpec> {
        AselcaW::new(self, 16)
    }
    ///Bit 17 - ELC_GPTB Event Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn aselcb(&mut self) -> AselcbW<GticasrSpec> {
        AselcbW::new(self, 17)
    }
    ///Bit 18 - ELC_GPTC Event Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn aselcc(&mut self) -> AselccW<GticasrSpec> {
        AselccW::new(self, 18)
    }
    ///Bit 19 - ELC_GPTD Event Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn aselcd(&mut self) -> AselcdW<GticasrSpec> {
        AselcdW::new(self, 19)
    }
    ///Bit 20 - ELC_GPTE Event Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn aselce(&mut self) -> AselceW<GticasrSpec> {
        AselceW::new(self, 20)
    }
    ///Bit 21 - ELC_GPTF Event Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn aselcf(&mut self) -> AselcfW<GticasrSpec> {
        AselcfW::new(self, 21)
    }
    ///Bit 22 - ELC_GPTG Event Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn aselcg(&mut self) -> AselcgW<GticasrSpec> {
        AselcgW::new(self, 22)
    }
    ///Bit 23 - ELC_GPTH Event Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn aselch(&mut self) -> AselchW<GticasrSpec> {
        AselchW::new(self, 23)
    }
}
/**General PWM Timer Input Capture Source Select Register A

You can [`read`](crate::Reg::read) this register and get [`gticasr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gticasr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GticasrSpec;
impl crate::RegisterSpec for GticasrSpec {
    type Ux = u32;
}
///`read()` method returns [`gticasr::R`](R) reader structure
impl crate::Readable for GticasrSpec {}
///`write(|w| ..)` method takes [`gticasr::W`](W) writer structure
impl crate::Writable for GticasrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTICASR to value 0
impl crate::Resettable for GticasrSpec {}
