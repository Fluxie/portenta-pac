///Register `PCNTR1` reader
pub type R = crate::R<Pcntr1Spec>;
///Register `PCNTR1` writer
pub type W = crate::W<Pcntr1Spec>;
/**Pmn Direction

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdr00 {
    ///0: Input (functions as an input pin)
    _0 = 0,
    ///1: Output (functions as an output pin)
    _1 = 1,
}
impl From<Pdr00> for bool {
    #[inline(always)]
    fn from(variant: Pdr00) -> Self {
        variant as u8 != 0
    }
}
///Field `PDR00` reader - Pmn Direction
pub type Pdr00R = crate::BitReader<Pdr00>;
impl Pdr00R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pdr00 {
        match self.bits {
            false => Pdr00::_0,
            true => Pdr00::_1,
        }
    }
    ///Input (functions as an input pin)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pdr00::_0
    }
    ///Output (functions as an output pin)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pdr00::_1
    }
}
///Field `PDR00` writer - Pmn Direction
pub type Pdr00W<'a, REG> = crate::BitWriter<'a, REG, Pdr00>;
impl<'a, REG> Pdr00W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Input (functions as an input pin)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr00::_0)
    }
    ///Output (functions as an output pin)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr00::_1)
    }
}
/**Pmn Direction

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdr01 {
    ///0: Input (functions as an input pin)
    _0 = 0,
    ///1: Output (functions as an output pin)
    _1 = 1,
}
impl From<Pdr01> for bool {
    #[inline(always)]
    fn from(variant: Pdr01) -> Self {
        variant as u8 != 0
    }
}
///Field `PDR01` reader - Pmn Direction
pub type Pdr01R = crate::BitReader<Pdr01>;
impl Pdr01R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pdr01 {
        match self.bits {
            false => Pdr01::_0,
            true => Pdr01::_1,
        }
    }
    ///Input (functions as an input pin)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pdr01::_0
    }
    ///Output (functions as an output pin)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pdr01::_1
    }
}
///Field `PDR01` writer - Pmn Direction
pub type Pdr01W<'a, REG> = crate::BitWriter<'a, REG, Pdr01>;
impl<'a, REG> Pdr01W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Input (functions as an input pin)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr01::_0)
    }
    ///Output (functions as an output pin)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr01::_1)
    }
}
/**Pmn Direction

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdr02 {
    ///0: Input (functions as an input pin)
    _0 = 0,
    ///1: Output (functions as an output pin)
    _1 = 1,
}
impl From<Pdr02> for bool {
    #[inline(always)]
    fn from(variant: Pdr02) -> Self {
        variant as u8 != 0
    }
}
///Field `PDR02` reader - Pmn Direction
pub type Pdr02R = crate::BitReader<Pdr02>;
impl Pdr02R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pdr02 {
        match self.bits {
            false => Pdr02::_0,
            true => Pdr02::_1,
        }
    }
    ///Input (functions as an input pin)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pdr02::_0
    }
    ///Output (functions as an output pin)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pdr02::_1
    }
}
///Field `PDR02` writer - Pmn Direction
pub type Pdr02W<'a, REG> = crate::BitWriter<'a, REG, Pdr02>;
impl<'a, REG> Pdr02W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Input (functions as an input pin)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr02::_0)
    }
    ///Output (functions as an output pin)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr02::_1)
    }
}
/**Pmn Direction

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdr03 {
    ///0: Input (functions as an input pin)
    _0 = 0,
    ///1: Output (functions as an output pin)
    _1 = 1,
}
impl From<Pdr03> for bool {
    #[inline(always)]
    fn from(variant: Pdr03) -> Self {
        variant as u8 != 0
    }
}
///Field `PDR03` reader - Pmn Direction
pub type Pdr03R = crate::BitReader<Pdr03>;
impl Pdr03R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pdr03 {
        match self.bits {
            false => Pdr03::_0,
            true => Pdr03::_1,
        }
    }
    ///Input (functions as an input pin)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pdr03::_0
    }
    ///Output (functions as an output pin)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pdr03::_1
    }
}
///Field `PDR03` writer - Pmn Direction
pub type Pdr03W<'a, REG> = crate::BitWriter<'a, REG, Pdr03>;
impl<'a, REG> Pdr03W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Input (functions as an input pin)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr03::_0)
    }
    ///Output (functions as an output pin)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr03::_1)
    }
}
/**Pmn Direction

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdr04 {
    ///0: Input (functions as an input pin)
    _0 = 0,
    ///1: Output (functions as an output pin)
    _1 = 1,
}
impl From<Pdr04> for bool {
    #[inline(always)]
    fn from(variant: Pdr04) -> Self {
        variant as u8 != 0
    }
}
///Field `PDR04` reader - Pmn Direction
pub type Pdr04R = crate::BitReader<Pdr04>;
impl Pdr04R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pdr04 {
        match self.bits {
            false => Pdr04::_0,
            true => Pdr04::_1,
        }
    }
    ///Input (functions as an input pin)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pdr04::_0
    }
    ///Output (functions as an output pin)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pdr04::_1
    }
}
///Field `PDR04` writer - Pmn Direction
pub type Pdr04W<'a, REG> = crate::BitWriter<'a, REG, Pdr04>;
impl<'a, REG> Pdr04W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Input (functions as an input pin)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr04::_0)
    }
    ///Output (functions as an output pin)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr04::_1)
    }
}
/**Pmn Direction

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdr05 {
    ///0: Input (functions as an input pin)
    _0 = 0,
    ///1: Output (functions as an output pin)
    _1 = 1,
}
impl From<Pdr05> for bool {
    #[inline(always)]
    fn from(variant: Pdr05) -> Self {
        variant as u8 != 0
    }
}
///Field `PDR05` reader - Pmn Direction
pub type Pdr05R = crate::BitReader<Pdr05>;
impl Pdr05R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pdr05 {
        match self.bits {
            false => Pdr05::_0,
            true => Pdr05::_1,
        }
    }
    ///Input (functions as an input pin)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pdr05::_0
    }
    ///Output (functions as an output pin)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pdr05::_1
    }
}
///Field `PDR05` writer - Pmn Direction
pub type Pdr05W<'a, REG> = crate::BitWriter<'a, REG, Pdr05>;
impl<'a, REG> Pdr05W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Input (functions as an input pin)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr05::_0)
    }
    ///Output (functions as an output pin)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr05::_1)
    }
}
/**Pmn Direction

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdr06 {
    ///0: Input (functions as an input pin)
    _0 = 0,
    ///1: Output (functions as an output pin)
    _1 = 1,
}
impl From<Pdr06> for bool {
    #[inline(always)]
    fn from(variant: Pdr06) -> Self {
        variant as u8 != 0
    }
}
///Field `PDR06` reader - Pmn Direction
pub type Pdr06R = crate::BitReader<Pdr06>;
impl Pdr06R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pdr06 {
        match self.bits {
            false => Pdr06::_0,
            true => Pdr06::_1,
        }
    }
    ///Input (functions as an input pin)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pdr06::_0
    }
    ///Output (functions as an output pin)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pdr06::_1
    }
}
///Field `PDR06` writer - Pmn Direction
pub type Pdr06W<'a, REG> = crate::BitWriter<'a, REG, Pdr06>;
impl<'a, REG> Pdr06W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Input (functions as an input pin)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr06::_0)
    }
    ///Output (functions as an output pin)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr06::_1)
    }
}
/**Pmn Direction

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdr07 {
    ///0: Input (functions as an input pin)
    _0 = 0,
    ///1: Output (functions as an output pin)
    _1 = 1,
}
impl From<Pdr07> for bool {
    #[inline(always)]
    fn from(variant: Pdr07) -> Self {
        variant as u8 != 0
    }
}
///Field `PDR07` reader - Pmn Direction
pub type Pdr07R = crate::BitReader<Pdr07>;
impl Pdr07R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pdr07 {
        match self.bits {
            false => Pdr07::_0,
            true => Pdr07::_1,
        }
    }
    ///Input (functions as an input pin)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pdr07::_0
    }
    ///Output (functions as an output pin)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pdr07::_1
    }
}
///Field `PDR07` writer - Pmn Direction
pub type Pdr07W<'a, REG> = crate::BitWriter<'a, REG, Pdr07>;
impl<'a, REG> Pdr07W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Input (functions as an input pin)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr07::_0)
    }
    ///Output (functions as an output pin)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr07::_1)
    }
}
/**Pmn Direction

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdr08 {
    ///0: Input (functions as an input pin)
    _0 = 0,
    ///1: Output (functions as an output pin)
    _1 = 1,
}
impl From<Pdr08> for bool {
    #[inline(always)]
    fn from(variant: Pdr08) -> Self {
        variant as u8 != 0
    }
}
///Field `PDR08` reader - Pmn Direction
pub type Pdr08R = crate::BitReader<Pdr08>;
impl Pdr08R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pdr08 {
        match self.bits {
            false => Pdr08::_0,
            true => Pdr08::_1,
        }
    }
    ///Input (functions as an input pin)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pdr08::_0
    }
    ///Output (functions as an output pin)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pdr08::_1
    }
}
///Field `PDR08` writer - Pmn Direction
pub type Pdr08W<'a, REG> = crate::BitWriter<'a, REG, Pdr08>;
impl<'a, REG> Pdr08W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Input (functions as an input pin)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr08::_0)
    }
    ///Output (functions as an output pin)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr08::_1)
    }
}
/**Pmn Direction

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdr09 {
    ///0: Input (functions as an input pin)
    _0 = 0,
    ///1: Output (functions as an output pin)
    _1 = 1,
}
impl From<Pdr09> for bool {
    #[inline(always)]
    fn from(variant: Pdr09) -> Self {
        variant as u8 != 0
    }
}
///Field `PDR09` reader - Pmn Direction
pub type Pdr09R = crate::BitReader<Pdr09>;
impl Pdr09R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pdr09 {
        match self.bits {
            false => Pdr09::_0,
            true => Pdr09::_1,
        }
    }
    ///Input (functions as an input pin)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pdr09::_0
    }
    ///Output (functions as an output pin)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pdr09::_1
    }
}
///Field `PDR09` writer - Pmn Direction
pub type Pdr09W<'a, REG> = crate::BitWriter<'a, REG, Pdr09>;
impl<'a, REG> Pdr09W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Input (functions as an input pin)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr09::_0)
    }
    ///Output (functions as an output pin)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr09::_1)
    }
}
/**Pmn Direction

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdr10 {
    ///0: Input (functions as an input pin)
    _0 = 0,
    ///1: Output (functions as an output pin)
    _1 = 1,
}
impl From<Pdr10> for bool {
    #[inline(always)]
    fn from(variant: Pdr10) -> Self {
        variant as u8 != 0
    }
}
///Field `PDR10` reader - Pmn Direction
pub type Pdr10R = crate::BitReader<Pdr10>;
impl Pdr10R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pdr10 {
        match self.bits {
            false => Pdr10::_0,
            true => Pdr10::_1,
        }
    }
    ///Input (functions as an input pin)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pdr10::_0
    }
    ///Output (functions as an output pin)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pdr10::_1
    }
}
///Field `PDR10` writer - Pmn Direction
pub type Pdr10W<'a, REG> = crate::BitWriter<'a, REG, Pdr10>;
impl<'a, REG> Pdr10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Input (functions as an input pin)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr10::_0)
    }
    ///Output (functions as an output pin)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr10::_1)
    }
}
/**Pmn Direction

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdr11 {
    ///0: Input (functions as an input pin)
    _0 = 0,
    ///1: Output (functions as an output pin)
    _1 = 1,
}
impl From<Pdr11> for bool {
    #[inline(always)]
    fn from(variant: Pdr11) -> Self {
        variant as u8 != 0
    }
}
///Field `PDR11` reader - Pmn Direction
pub type Pdr11R = crate::BitReader<Pdr11>;
impl Pdr11R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pdr11 {
        match self.bits {
            false => Pdr11::_0,
            true => Pdr11::_1,
        }
    }
    ///Input (functions as an input pin)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pdr11::_0
    }
    ///Output (functions as an output pin)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pdr11::_1
    }
}
///Field `PDR11` writer - Pmn Direction
pub type Pdr11W<'a, REG> = crate::BitWriter<'a, REG, Pdr11>;
impl<'a, REG> Pdr11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Input (functions as an input pin)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr11::_0)
    }
    ///Output (functions as an output pin)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr11::_1)
    }
}
/**Pmn Direction

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdr12 {
    ///0: Input (functions as an input pin)
    _0 = 0,
    ///1: Output (functions as an output pin)
    _1 = 1,
}
impl From<Pdr12> for bool {
    #[inline(always)]
    fn from(variant: Pdr12) -> Self {
        variant as u8 != 0
    }
}
///Field `PDR12` reader - Pmn Direction
pub type Pdr12R = crate::BitReader<Pdr12>;
impl Pdr12R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pdr12 {
        match self.bits {
            false => Pdr12::_0,
            true => Pdr12::_1,
        }
    }
    ///Input (functions as an input pin)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pdr12::_0
    }
    ///Output (functions as an output pin)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pdr12::_1
    }
}
///Field `PDR12` writer - Pmn Direction
pub type Pdr12W<'a, REG> = crate::BitWriter<'a, REG, Pdr12>;
impl<'a, REG> Pdr12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Input (functions as an input pin)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr12::_0)
    }
    ///Output (functions as an output pin)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr12::_1)
    }
}
/**Pmn Direction

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdr13 {
    ///0: Input (functions as an input pin)
    _0 = 0,
    ///1: Output (functions as an output pin)
    _1 = 1,
}
impl From<Pdr13> for bool {
    #[inline(always)]
    fn from(variant: Pdr13) -> Self {
        variant as u8 != 0
    }
}
///Field `PDR13` reader - Pmn Direction
pub type Pdr13R = crate::BitReader<Pdr13>;
impl Pdr13R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pdr13 {
        match self.bits {
            false => Pdr13::_0,
            true => Pdr13::_1,
        }
    }
    ///Input (functions as an input pin)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pdr13::_0
    }
    ///Output (functions as an output pin)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pdr13::_1
    }
}
///Field `PDR13` writer - Pmn Direction
pub type Pdr13W<'a, REG> = crate::BitWriter<'a, REG, Pdr13>;
impl<'a, REG> Pdr13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Input (functions as an input pin)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr13::_0)
    }
    ///Output (functions as an output pin)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr13::_1)
    }
}
/**Pmn Direction

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdr14 {
    ///0: Input (functions as an input pin)
    _0 = 0,
    ///1: Output (functions as an output pin)
    _1 = 1,
}
impl From<Pdr14> for bool {
    #[inline(always)]
    fn from(variant: Pdr14) -> Self {
        variant as u8 != 0
    }
}
///Field `PDR14` reader - Pmn Direction
pub type Pdr14R = crate::BitReader<Pdr14>;
impl Pdr14R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pdr14 {
        match self.bits {
            false => Pdr14::_0,
            true => Pdr14::_1,
        }
    }
    ///Input (functions as an input pin)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pdr14::_0
    }
    ///Output (functions as an output pin)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pdr14::_1
    }
}
///Field `PDR14` writer - Pmn Direction
pub type Pdr14W<'a, REG> = crate::BitWriter<'a, REG, Pdr14>;
impl<'a, REG> Pdr14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Input (functions as an input pin)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr14::_0)
    }
    ///Output (functions as an output pin)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr14::_1)
    }
}
/**Pmn Direction

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdr15 {
    ///0: Input (functions as an input pin)
    _0 = 0,
    ///1: Output (functions as an output pin)
    _1 = 1,
}
impl From<Pdr15> for bool {
    #[inline(always)]
    fn from(variant: Pdr15) -> Self {
        variant as u8 != 0
    }
}
///Field `PDR15` reader - Pmn Direction
pub type Pdr15R = crate::BitReader<Pdr15>;
impl Pdr15R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pdr15 {
        match self.bits {
            false => Pdr15::_0,
            true => Pdr15::_1,
        }
    }
    ///Input (functions as an input pin)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pdr15::_0
    }
    ///Output (functions as an output pin)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pdr15::_1
    }
}
///Field `PDR15` writer - Pmn Direction
pub type Pdr15W<'a, REG> = crate::BitWriter<'a, REG, Pdr15>;
impl<'a, REG> Pdr15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Input (functions as an input pin)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr15::_0)
    }
    ///Output (functions as an output pin)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr15::_1)
    }
}
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
    ///Bit 0 - Pmn Direction
    #[inline(always)]
    pub fn pdr00(&self) -> Pdr00R {
        Pdr00R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Pmn Direction
    #[inline(always)]
    pub fn pdr01(&self) -> Pdr01R {
        Pdr01R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Pmn Direction
    #[inline(always)]
    pub fn pdr02(&self) -> Pdr02R {
        Pdr02R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Pmn Direction
    #[inline(always)]
    pub fn pdr03(&self) -> Pdr03R {
        Pdr03R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Pmn Direction
    #[inline(always)]
    pub fn pdr04(&self) -> Pdr04R {
        Pdr04R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Pmn Direction
    #[inline(always)]
    pub fn pdr05(&self) -> Pdr05R {
        Pdr05R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Pmn Direction
    #[inline(always)]
    pub fn pdr06(&self) -> Pdr06R {
        Pdr06R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Pmn Direction
    #[inline(always)]
    pub fn pdr07(&self) -> Pdr07R {
        Pdr07R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Pmn Direction
    #[inline(always)]
    pub fn pdr08(&self) -> Pdr08R {
        Pdr08R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Pmn Direction
    #[inline(always)]
    pub fn pdr09(&self) -> Pdr09R {
        Pdr09R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Pmn Direction
    #[inline(always)]
    pub fn pdr10(&self) -> Pdr10R {
        Pdr10R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Pmn Direction
    #[inline(always)]
    pub fn pdr11(&self) -> Pdr11R {
        Pdr11R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Pmn Direction
    #[inline(always)]
    pub fn pdr12(&self) -> Pdr12R {
        Pdr12R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Pmn Direction
    #[inline(always)]
    pub fn pdr13(&self) -> Pdr13R {
        Pdr13R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Pmn Direction
    #[inline(always)]
    pub fn pdr14(&self) -> Pdr14R {
        Pdr14R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Pmn Direction
    #[inline(always)]
    pub fn pdr15(&self) -> Pdr15R {
        Pdr15R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Pmn Output Data
    #[inline(always)]
    pub fn podr00(&self) -> Podr00R {
        Podr00R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Pmn Output Data
    #[inline(always)]
    pub fn podr01(&self) -> Podr01R {
        Podr01R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Pmn Output Data
    #[inline(always)]
    pub fn podr02(&self) -> Podr02R {
        Podr02R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Pmn Output Data
    #[inline(always)]
    pub fn podr03(&self) -> Podr03R {
        Podr03R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Pmn Output Data
    #[inline(always)]
    pub fn podr04(&self) -> Podr04R {
        Podr04R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Pmn Output Data
    #[inline(always)]
    pub fn podr05(&self) -> Podr05R {
        Podr05R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Pmn Output Data
    #[inline(always)]
    pub fn podr06(&self) -> Podr06R {
        Podr06R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Pmn Output Data
    #[inline(always)]
    pub fn podr07(&self) -> Podr07R {
        Podr07R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Pmn Output Data
    #[inline(always)]
    pub fn podr08(&self) -> Podr08R {
        Podr08R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Pmn Output Data
    #[inline(always)]
    pub fn podr09(&self) -> Podr09R {
        Podr09R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Pmn Output Data
    #[inline(always)]
    pub fn podr10(&self) -> Podr10R {
        Podr10R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Pmn Output Data
    #[inline(always)]
    pub fn podr11(&self) -> Podr11R {
        Podr11R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Pmn Output Data
    #[inline(always)]
    pub fn podr12(&self) -> Podr12R {
        Podr12R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Pmn Output Data
    #[inline(always)]
    pub fn podr13(&self) -> Podr13R {
        Podr13R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Pmn Output Data
    #[inline(always)]
    pub fn podr14(&self) -> Podr14R {
        Podr14R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Pmn Output Data
    #[inline(always)]
    pub fn podr15(&self) -> Podr15R {
        Podr15R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCNTR1")
            .field("pdr00", &self.pdr00())
            .field("pdr01", &self.pdr01())
            .field("pdr02", &self.pdr02())
            .field("pdr03", &self.pdr03())
            .field("pdr04", &self.pdr04())
            .field("pdr05", &self.pdr05())
            .field("pdr06", &self.pdr06())
            .field("pdr07", &self.pdr07())
            .field("pdr08", &self.pdr08())
            .field("pdr09", &self.pdr09())
            .field("pdr10", &self.pdr10())
            .field("pdr11", &self.pdr11())
            .field("pdr12", &self.pdr12())
            .field("pdr13", &self.pdr13())
            .field("pdr14", &self.pdr14())
            .field("pdr15", &self.pdr15())
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
    ///Bit 0 - Pmn Direction
    #[inline(always)]
    pub fn pdr00(&mut self) -> Pdr00W<Pcntr1Spec> {
        Pdr00W::new(self, 0)
    }
    ///Bit 1 - Pmn Direction
    #[inline(always)]
    pub fn pdr01(&mut self) -> Pdr01W<Pcntr1Spec> {
        Pdr01W::new(self, 1)
    }
    ///Bit 2 - Pmn Direction
    #[inline(always)]
    pub fn pdr02(&mut self) -> Pdr02W<Pcntr1Spec> {
        Pdr02W::new(self, 2)
    }
    ///Bit 3 - Pmn Direction
    #[inline(always)]
    pub fn pdr03(&mut self) -> Pdr03W<Pcntr1Spec> {
        Pdr03W::new(self, 3)
    }
    ///Bit 4 - Pmn Direction
    #[inline(always)]
    pub fn pdr04(&mut self) -> Pdr04W<Pcntr1Spec> {
        Pdr04W::new(self, 4)
    }
    ///Bit 5 - Pmn Direction
    #[inline(always)]
    pub fn pdr05(&mut self) -> Pdr05W<Pcntr1Spec> {
        Pdr05W::new(self, 5)
    }
    ///Bit 6 - Pmn Direction
    #[inline(always)]
    pub fn pdr06(&mut self) -> Pdr06W<Pcntr1Spec> {
        Pdr06W::new(self, 6)
    }
    ///Bit 7 - Pmn Direction
    #[inline(always)]
    pub fn pdr07(&mut self) -> Pdr07W<Pcntr1Spec> {
        Pdr07W::new(self, 7)
    }
    ///Bit 8 - Pmn Direction
    #[inline(always)]
    pub fn pdr08(&mut self) -> Pdr08W<Pcntr1Spec> {
        Pdr08W::new(self, 8)
    }
    ///Bit 9 - Pmn Direction
    #[inline(always)]
    pub fn pdr09(&mut self) -> Pdr09W<Pcntr1Spec> {
        Pdr09W::new(self, 9)
    }
    ///Bit 10 - Pmn Direction
    #[inline(always)]
    pub fn pdr10(&mut self) -> Pdr10W<Pcntr1Spec> {
        Pdr10W::new(self, 10)
    }
    ///Bit 11 - Pmn Direction
    #[inline(always)]
    pub fn pdr11(&mut self) -> Pdr11W<Pcntr1Spec> {
        Pdr11W::new(self, 11)
    }
    ///Bit 12 - Pmn Direction
    #[inline(always)]
    pub fn pdr12(&mut self) -> Pdr12W<Pcntr1Spec> {
        Pdr12W::new(self, 12)
    }
    ///Bit 13 - Pmn Direction
    #[inline(always)]
    pub fn pdr13(&mut self) -> Pdr13W<Pcntr1Spec> {
        Pdr13W::new(self, 13)
    }
    ///Bit 14 - Pmn Direction
    #[inline(always)]
    pub fn pdr14(&mut self) -> Pdr14W<Pcntr1Spec> {
        Pdr14W::new(self, 14)
    }
    ///Bit 15 - Pmn Direction
    #[inline(always)]
    pub fn pdr15(&mut self) -> Pdr15W<Pcntr1Spec> {
        Pdr15W::new(self, 15)
    }
    ///Bit 16 - Pmn Output Data
    #[inline(always)]
    pub fn podr00(&mut self) -> Podr00W<Pcntr1Spec> {
        Podr00W::new(self, 16)
    }
    ///Bit 17 - Pmn Output Data
    #[inline(always)]
    pub fn podr01(&mut self) -> Podr01W<Pcntr1Spec> {
        Podr01W::new(self, 17)
    }
    ///Bit 18 - Pmn Output Data
    #[inline(always)]
    pub fn podr02(&mut self) -> Podr02W<Pcntr1Spec> {
        Podr02W::new(self, 18)
    }
    ///Bit 19 - Pmn Output Data
    #[inline(always)]
    pub fn podr03(&mut self) -> Podr03W<Pcntr1Spec> {
        Podr03W::new(self, 19)
    }
    ///Bit 20 - Pmn Output Data
    #[inline(always)]
    pub fn podr04(&mut self) -> Podr04W<Pcntr1Spec> {
        Podr04W::new(self, 20)
    }
    ///Bit 21 - Pmn Output Data
    #[inline(always)]
    pub fn podr05(&mut self) -> Podr05W<Pcntr1Spec> {
        Podr05W::new(self, 21)
    }
    ///Bit 22 - Pmn Output Data
    #[inline(always)]
    pub fn podr06(&mut self) -> Podr06W<Pcntr1Spec> {
        Podr06W::new(self, 22)
    }
    ///Bit 23 - Pmn Output Data
    #[inline(always)]
    pub fn podr07(&mut self) -> Podr07W<Pcntr1Spec> {
        Podr07W::new(self, 23)
    }
    ///Bit 24 - Pmn Output Data
    #[inline(always)]
    pub fn podr08(&mut self) -> Podr08W<Pcntr1Spec> {
        Podr08W::new(self, 24)
    }
    ///Bit 25 - Pmn Output Data
    #[inline(always)]
    pub fn podr09(&mut self) -> Podr09W<Pcntr1Spec> {
        Podr09W::new(self, 25)
    }
    ///Bit 26 - Pmn Output Data
    #[inline(always)]
    pub fn podr10(&mut self) -> Podr10W<Pcntr1Spec> {
        Podr10W::new(self, 26)
    }
    ///Bit 27 - Pmn Output Data
    #[inline(always)]
    pub fn podr11(&mut self) -> Podr11W<Pcntr1Spec> {
        Podr11W::new(self, 27)
    }
    ///Bit 28 - Pmn Output Data
    #[inline(always)]
    pub fn podr12(&mut self) -> Podr12W<Pcntr1Spec> {
        Podr12W::new(self, 28)
    }
    ///Bit 29 - Pmn Output Data
    #[inline(always)]
    pub fn podr13(&mut self) -> Podr13W<Pcntr1Spec> {
        Podr13W::new(self, 29)
    }
    ///Bit 30 - Pmn Output Data
    #[inline(always)]
    pub fn podr14(&mut self) -> Podr14W<Pcntr1Spec> {
        Podr14W::new(self, 30)
    }
    ///Bit 31 - Pmn Output Data
    #[inline(always)]
    pub fn podr15(&mut self) -> Podr15W<Pcntr1Spec> {
        Podr15W::new(self, 31)
    }
}
/**Port Control Register 1

You can [`read`](crate::Reg::read) this register and get [`pcntr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcntr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Pcntr1Spec;
impl crate::RegisterSpec for Pcntr1Spec {
    type Ux = u32;
}
///`read()` method returns [`pcntr1::R`](R) reader structure
impl crate::Readable for Pcntr1Spec {}
///`write(|w| ..)` method takes [`pcntr1::W`](W) writer structure
impl crate::Writable for Pcntr1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PCNTR1 to value 0
impl crate::Resettable for Pcntr1Spec {}
