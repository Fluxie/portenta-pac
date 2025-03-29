///Register `PODR` reader
pub type R = crate::R<PodrSpec>;
///Register `PODR` writer
pub type W = crate::W<PodrSpec>;
/**Pmn Output Data

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Podr00 {
    ///0: Low output
    _0 = 0,
    ///1: High output
    _1 = 1,
}
impl From<Podr00> for bool {
    #[inline(always)]
    fn from(variant: Podr00) -> Self {
        variant as u8 != 0
    }
}
///Field `PODR00` reader - Pmn Output Data
pub type Podr00R = crate::BitReader<Podr00>;
impl Podr00R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Podr00 {
        match self.bits {
            false => Podr00::_0,
            true => Podr00::_1,
        }
    }
    ///Low output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Podr00::_0
    }
    ///High output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Podr00::_1
    }
}
///Field `PODR00` writer - Pmn Output Data
pub type Podr00W<'a, REG> = crate::BitWriter<'a, REG, Podr00>;
impl<'a, REG> Podr00W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Low output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Podr00::_0)
    }
    ///High output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Podr00::_1)
    }
}
/**Pmn Output Data

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Podr01 {
    ///0: Low output
    _0 = 0,
    ///1: High output
    _1 = 1,
}
impl From<Podr01> for bool {
    #[inline(always)]
    fn from(variant: Podr01) -> Self {
        variant as u8 != 0
    }
}
///Field `PODR01` reader - Pmn Output Data
pub type Podr01R = crate::BitReader<Podr01>;
impl Podr01R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Podr01 {
        match self.bits {
            false => Podr01::_0,
            true => Podr01::_1,
        }
    }
    ///Low output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Podr01::_0
    }
    ///High output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Podr01::_1
    }
}
///Field `PODR01` writer - Pmn Output Data
pub type Podr01W<'a, REG> = crate::BitWriter<'a, REG, Podr01>;
impl<'a, REG> Podr01W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Low output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Podr01::_0)
    }
    ///High output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Podr01::_1)
    }
}
/**Pmn Output Data

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Podr02 {
    ///0: Low output
    _0 = 0,
    ///1: High output
    _1 = 1,
}
impl From<Podr02> for bool {
    #[inline(always)]
    fn from(variant: Podr02) -> Self {
        variant as u8 != 0
    }
}
///Field `PODR02` reader - Pmn Output Data
pub type Podr02R = crate::BitReader<Podr02>;
impl Podr02R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Podr02 {
        match self.bits {
            false => Podr02::_0,
            true => Podr02::_1,
        }
    }
    ///Low output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Podr02::_0
    }
    ///High output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Podr02::_1
    }
}
///Field `PODR02` writer - Pmn Output Data
pub type Podr02W<'a, REG> = crate::BitWriter<'a, REG, Podr02>;
impl<'a, REG> Podr02W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Low output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Podr02::_0)
    }
    ///High output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Podr02::_1)
    }
}
/**Pmn Output Data

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Podr03 {
    ///0: Low output
    _0 = 0,
    ///1: High output
    _1 = 1,
}
impl From<Podr03> for bool {
    #[inline(always)]
    fn from(variant: Podr03) -> Self {
        variant as u8 != 0
    }
}
///Field `PODR03` reader - Pmn Output Data
pub type Podr03R = crate::BitReader<Podr03>;
impl Podr03R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Podr03 {
        match self.bits {
            false => Podr03::_0,
            true => Podr03::_1,
        }
    }
    ///Low output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Podr03::_0
    }
    ///High output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Podr03::_1
    }
}
///Field `PODR03` writer - Pmn Output Data
pub type Podr03W<'a, REG> = crate::BitWriter<'a, REG, Podr03>;
impl<'a, REG> Podr03W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Low output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Podr03::_0)
    }
    ///High output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Podr03::_1)
    }
}
/**Pmn Output Data

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Podr04 {
    ///0: Low output
    _0 = 0,
    ///1: High output
    _1 = 1,
}
impl From<Podr04> for bool {
    #[inline(always)]
    fn from(variant: Podr04) -> Self {
        variant as u8 != 0
    }
}
///Field `PODR04` reader - Pmn Output Data
pub type Podr04R = crate::BitReader<Podr04>;
impl Podr04R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Podr04 {
        match self.bits {
            false => Podr04::_0,
            true => Podr04::_1,
        }
    }
    ///Low output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Podr04::_0
    }
    ///High output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Podr04::_1
    }
}
///Field `PODR04` writer - Pmn Output Data
pub type Podr04W<'a, REG> = crate::BitWriter<'a, REG, Podr04>;
impl<'a, REG> Podr04W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Low output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Podr04::_0)
    }
    ///High output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Podr04::_1)
    }
}
/**Pmn Output Data

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Podr05 {
    ///0: Low output
    _0 = 0,
    ///1: High output
    _1 = 1,
}
impl From<Podr05> for bool {
    #[inline(always)]
    fn from(variant: Podr05) -> Self {
        variant as u8 != 0
    }
}
///Field `PODR05` reader - Pmn Output Data
pub type Podr05R = crate::BitReader<Podr05>;
impl Podr05R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Podr05 {
        match self.bits {
            false => Podr05::_0,
            true => Podr05::_1,
        }
    }
    ///Low output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Podr05::_0
    }
    ///High output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Podr05::_1
    }
}
///Field `PODR05` writer - Pmn Output Data
pub type Podr05W<'a, REG> = crate::BitWriter<'a, REG, Podr05>;
impl<'a, REG> Podr05W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Low output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Podr05::_0)
    }
    ///High output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Podr05::_1)
    }
}
/**Pmn Output Data

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Podr06 {
    ///0: Low output
    _0 = 0,
    ///1: High output
    _1 = 1,
}
impl From<Podr06> for bool {
    #[inline(always)]
    fn from(variant: Podr06) -> Self {
        variant as u8 != 0
    }
}
///Field `PODR06` reader - Pmn Output Data
pub type Podr06R = crate::BitReader<Podr06>;
impl Podr06R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Podr06 {
        match self.bits {
            false => Podr06::_0,
            true => Podr06::_1,
        }
    }
    ///Low output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Podr06::_0
    }
    ///High output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Podr06::_1
    }
}
///Field `PODR06` writer - Pmn Output Data
pub type Podr06W<'a, REG> = crate::BitWriter<'a, REG, Podr06>;
impl<'a, REG> Podr06W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Low output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Podr06::_0)
    }
    ///High output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Podr06::_1)
    }
}
/**Pmn Output Data

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Podr07 {
    ///0: Low output
    _0 = 0,
    ///1: High output
    _1 = 1,
}
impl From<Podr07> for bool {
    #[inline(always)]
    fn from(variant: Podr07) -> Self {
        variant as u8 != 0
    }
}
///Field `PODR07` reader - Pmn Output Data
pub type Podr07R = crate::BitReader<Podr07>;
impl Podr07R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Podr07 {
        match self.bits {
            false => Podr07::_0,
            true => Podr07::_1,
        }
    }
    ///Low output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Podr07::_0
    }
    ///High output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Podr07::_1
    }
}
///Field `PODR07` writer - Pmn Output Data
pub type Podr07W<'a, REG> = crate::BitWriter<'a, REG, Podr07>;
impl<'a, REG> Podr07W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Low output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Podr07::_0)
    }
    ///High output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Podr07::_1)
    }
}
/**Pmn Output Data

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Podr08 {
    ///0: Low output
    _0 = 0,
    ///1: High output
    _1 = 1,
}
impl From<Podr08> for bool {
    #[inline(always)]
    fn from(variant: Podr08) -> Self {
        variant as u8 != 0
    }
}
///Field `PODR08` reader - Pmn Output Data
pub type Podr08R = crate::BitReader<Podr08>;
impl Podr08R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Podr08 {
        match self.bits {
            false => Podr08::_0,
            true => Podr08::_1,
        }
    }
    ///Low output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Podr08::_0
    }
    ///High output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Podr08::_1
    }
}
///Field `PODR08` writer - Pmn Output Data
pub type Podr08W<'a, REG> = crate::BitWriter<'a, REG, Podr08>;
impl<'a, REG> Podr08W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Low output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Podr08::_0)
    }
    ///High output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Podr08::_1)
    }
}
/**Pmn Output Data

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Podr09 {
    ///0: Low output
    _0 = 0,
    ///1: High output
    _1 = 1,
}
impl From<Podr09> for bool {
    #[inline(always)]
    fn from(variant: Podr09) -> Self {
        variant as u8 != 0
    }
}
///Field `PODR09` reader - Pmn Output Data
pub type Podr09R = crate::BitReader<Podr09>;
impl Podr09R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Podr09 {
        match self.bits {
            false => Podr09::_0,
            true => Podr09::_1,
        }
    }
    ///Low output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Podr09::_0
    }
    ///High output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Podr09::_1
    }
}
///Field `PODR09` writer - Pmn Output Data
pub type Podr09W<'a, REG> = crate::BitWriter<'a, REG, Podr09>;
impl<'a, REG> Podr09W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Low output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Podr09::_0)
    }
    ///High output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Podr09::_1)
    }
}
/**Pmn Output Data

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Podr10 {
    ///0: Low output
    _0 = 0,
    ///1: High output
    _1 = 1,
}
impl From<Podr10> for bool {
    #[inline(always)]
    fn from(variant: Podr10) -> Self {
        variant as u8 != 0
    }
}
///Field `PODR10` reader - Pmn Output Data
pub type Podr10R = crate::BitReader<Podr10>;
impl Podr10R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Podr10 {
        match self.bits {
            false => Podr10::_0,
            true => Podr10::_1,
        }
    }
    ///Low output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Podr10::_0
    }
    ///High output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Podr10::_1
    }
}
///Field `PODR10` writer - Pmn Output Data
pub type Podr10W<'a, REG> = crate::BitWriter<'a, REG, Podr10>;
impl<'a, REG> Podr10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Low output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Podr10::_0)
    }
    ///High output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Podr10::_1)
    }
}
/**Pmn Output Data

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Podr11 {
    ///0: Low output
    _0 = 0,
    ///1: High output
    _1 = 1,
}
impl From<Podr11> for bool {
    #[inline(always)]
    fn from(variant: Podr11) -> Self {
        variant as u8 != 0
    }
}
///Field `PODR11` reader - Pmn Output Data
pub type Podr11R = crate::BitReader<Podr11>;
impl Podr11R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Podr11 {
        match self.bits {
            false => Podr11::_0,
            true => Podr11::_1,
        }
    }
    ///Low output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Podr11::_0
    }
    ///High output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Podr11::_1
    }
}
///Field `PODR11` writer - Pmn Output Data
pub type Podr11W<'a, REG> = crate::BitWriter<'a, REG, Podr11>;
impl<'a, REG> Podr11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Low output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Podr11::_0)
    }
    ///High output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Podr11::_1)
    }
}
/**Pmn Output Data

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Podr12 {
    ///0: Low output
    _0 = 0,
    ///1: High output
    _1 = 1,
}
impl From<Podr12> for bool {
    #[inline(always)]
    fn from(variant: Podr12) -> Self {
        variant as u8 != 0
    }
}
///Field `PODR12` reader - Pmn Output Data
pub type Podr12R = crate::BitReader<Podr12>;
impl Podr12R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Podr12 {
        match self.bits {
            false => Podr12::_0,
            true => Podr12::_1,
        }
    }
    ///Low output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Podr12::_0
    }
    ///High output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Podr12::_1
    }
}
///Field `PODR12` writer - Pmn Output Data
pub type Podr12W<'a, REG> = crate::BitWriter<'a, REG, Podr12>;
impl<'a, REG> Podr12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Low output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Podr12::_0)
    }
    ///High output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Podr12::_1)
    }
}
/**Pmn Output Data

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Podr13 {
    ///0: Low output
    _0 = 0,
    ///1: High output
    _1 = 1,
}
impl From<Podr13> for bool {
    #[inline(always)]
    fn from(variant: Podr13) -> Self {
        variant as u8 != 0
    }
}
///Field `PODR13` reader - Pmn Output Data
pub type Podr13R = crate::BitReader<Podr13>;
impl Podr13R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Podr13 {
        match self.bits {
            false => Podr13::_0,
            true => Podr13::_1,
        }
    }
    ///Low output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Podr13::_0
    }
    ///High output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Podr13::_1
    }
}
///Field `PODR13` writer - Pmn Output Data
pub type Podr13W<'a, REG> = crate::BitWriter<'a, REG, Podr13>;
impl<'a, REG> Podr13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Low output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Podr13::_0)
    }
    ///High output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Podr13::_1)
    }
}
/**Pmn Output Data

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Podr14 {
    ///0: Low output
    _0 = 0,
    ///1: High output
    _1 = 1,
}
impl From<Podr14> for bool {
    #[inline(always)]
    fn from(variant: Podr14) -> Self {
        variant as u8 != 0
    }
}
///Field `PODR14` reader - Pmn Output Data
pub type Podr14R = crate::BitReader<Podr14>;
impl Podr14R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Podr14 {
        match self.bits {
            false => Podr14::_0,
            true => Podr14::_1,
        }
    }
    ///Low output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Podr14::_0
    }
    ///High output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Podr14::_1
    }
}
///Field `PODR14` writer - Pmn Output Data
pub type Podr14W<'a, REG> = crate::BitWriter<'a, REG, Podr14>;
impl<'a, REG> Podr14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Low output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Podr14::_0)
    }
    ///High output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Podr14::_1)
    }
}
/**Pmn Output Data

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Podr15 {
    ///0: Low output
    _0 = 0,
    ///1: High output
    _1 = 1,
}
impl From<Podr15> for bool {
    #[inline(always)]
    fn from(variant: Podr15) -> Self {
        variant as u8 != 0
    }
}
///Field `PODR15` reader - Pmn Output Data
pub type Podr15R = crate::BitReader<Podr15>;
impl Podr15R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Podr15 {
        match self.bits {
            false => Podr15::_0,
            true => Podr15::_1,
        }
    }
    ///Low output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Podr15::_0
    }
    ///High output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Podr15::_1
    }
}
///Field `PODR15` writer - Pmn Output Data
pub type Podr15W<'a, REG> = crate::BitWriter<'a, REG, Podr15>;
impl<'a, REG> Podr15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Low output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Podr15::_0)
    }
    ///High output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Podr15::_1)
    }
}
impl R {
    ///Bit 0 - Pmn Output Data
    #[inline(always)]
    pub fn podr00(&self) -> Podr00R {
        Podr00R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Pmn Output Data
    #[inline(always)]
    pub fn podr01(&self) -> Podr01R {
        Podr01R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Pmn Output Data
    #[inline(always)]
    pub fn podr02(&self) -> Podr02R {
        Podr02R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Pmn Output Data
    #[inline(always)]
    pub fn podr03(&self) -> Podr03R {
        Podr03R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Pmn Output Data
    #[inline(always)]
    pub fn podr04(&self) -> Podr04R {
        Podr04R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Pmn Output Data
    #[inline(always)]
    pub fn podr05(&self) -> Podr05R {
        Podr05R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Pmn Output Data
    #[inline(always)]
    pub fn podr06(&self) -> Podr06R {
        Podr06R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Pmn Output Data
    #[inline(always)]
    pub fn podr07(&self) -> Podr07R {
        Podr07R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Pmn Output Data
    #[inline(always)]
    pub fn podr08(&self) -> Podr08R {
        Podr08R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Pmn Output Data
    #[inline(always)]
    pub fn podr09(&self) -> Podr09R {
        Podr09R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Pmn Output Data
    #[inline(always)]
    pub fn podr10(&self) -> Podr10R {
        Podr10R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Pmn Output Data
    #[inline(always)]
    pub fn podr11(&self) -> Podr11R {
        Podr11R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Pmn Output Data
    #[inline(always)]
    pub fn podr12(&self) -> Podr12R {
        Podr12R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Pmn Output Data
    #[inline(always)]
    pub fn podr13(&self) -> Podr13R {
        Podr13R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Pmn Output Data
    #[inline(always)]
    pub fn podr14(&self) -> Podr14R {
        Podr14R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Pmn Output Data
    #[inline(always)]
    pub fn podr15(&self) -> Podr15R {
        Podr15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PODR")
            .field("podr00", &self.podr00())
            .field("podr01", &self.podr01())
            .field("podr02", &self.podr02())
            .field("podr03", &self.podr03())
            .field("podr04", &self.podr04())
            .field("podr05", &self.podr05())
            .field("podr06", &self.podr06())
            .field("podr07", &self.podr07())
            .field("podr08", &self.podr08())
            .field("podr09", &self.podr09())
            .field("podr10", &self.podr10())
            .field("podr11", &self.podr11())
            .field("podr12", &self.podr12())
            .field("podr13", &self.podr13())
            .field("podr14", &self.podr14())
            .field("podr15", &self.podr15())
            .finish()
    }
}
impl W {
    ///Bit 0 - Pmn Output Data
    #[inline(always)]
    pub fn podr00(&mut self) -> Podr00W<PodrSpec> {
        Podr00W::new(self, 0)
    }
    ///Bit 1 - Pmn Output Data
    #[inline(always)]
    pub fn podr01(&mut self) -> Podr01W<PodrSpec> {
        Podr01W::new(self, 1)
    }
    ///Bit 2 - Pmn Output Data
    #[inline(always)]
    pub fn podr02(&mut self) -> Podr02W<PodrSpec> {
        Podr02W::new(self, 2)
    }
    ///Bit 3 - Pmn Output Data
    #[inline(always)]
    pub fn podr03(&mut self) -> Podr03W<PodrSpec> {
        Podr03W::new(self, 3)
    }
    ///Bit 4 - Pmn Output Data
    #[inline(always)]
    pub fn podr04(&mut self) -> Podr04W<PodrSpec> {
        Podr04W::new(self, 4)
    }
    ///Bit 5 - Pmn Output Data
    #[inline(always)]
    pub fn podr05(&mut self) -> Podr05W<PodrSpec> {
        Podr05W::new(self, 5)
    }
    ///Bit 6 - Pmn Output Data
    #[inline(always)]
    pub fn podr06(&mut self) -> Podr06W<PodrSpec> {
        Podr06W::new(self, 6)
    }
    ///Bit 7 - Pmn Output Data
    #[inline(always)]
    pub fn podr07(&mut self) -> Podr07W<PodrSpec> {
        Podr07W::new(self, 7)
    }
    ///Bit 8 - Pmn Output Data
    #[inline(always)]
    pub fn podr08(&mut self) -> Podr08W<PodrSpec> {
        Podr08W::new(self, 8)
    }
    ///Bit 9 - Pmn Output Data
    #[inline(always)]
    pub fn podr09(&mut self) -> Podr09W<PodrSpec> {
        Podr09W::new(self, 9)
    }
    ///Bit 10 - Pmn Output Data
    #[inline(always)]
    pub fn podr10(&mut self) -> Podr10W<PodrSpec> {
        Podr10W::new(self, 10)
    }
    ///Bit 11 - Pmn Output Data
    #[inline(always)]
    pub fn podr11(&mut self) -> Podr11W<PodrSpec> {
        Podr11W::new(self, 11)
    }
    ///Bit 12 - Pmn Output Data
    #[inline(always)]
    pub fn podr12(&mut self) -> Podr12W<PodrSpec> {
        Podr12W::new(self, 12)
    }
    ///Bit 13 - Pmn Output Data
    #[inline(always)]
    pub fn podr13(&mut self) -> Podr13W<PodrSpec> {
        Podr13W::new(self, 13)
    }
    ///Bit 14 - Pmn Output Data
    #[inline(always)]
    pub fn podr14(&mut self) -> Podr14W<PodrSpec> {
        Podr14W::new(self, 14)
    }
    ///Bit 15 - Pmn Output Data
    #[inline(always)]
    pub fn podr15(&mut self) -> Podr15W<PodrSpec> {
        Podr15W::new(self, 15)
    }
}
/**Port Control Register 1

You can [`read`](crate::Reg::read) this register and get [`podr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`podr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PodrSpec;
impl crate::RegisterSpec for PodrSpec {
    type Ux = u16;
}
///`read()` method returns [`podr::R`](R) reader structure
impl crate::Readable for PodrSpec {}
///`write(|w| ..)` method takes [`podr::W`](W) writer structure
impl crate::Writable for PodrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PODR to value 0
impl crate::Resettable for PodrSpec {}
