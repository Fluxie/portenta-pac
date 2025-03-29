///Register `CADR` reader
pub type R = crate::R<CadrSpec>;
///Register `CADR` writer
pub type W = crate::W<CadrSpec>;
/**Local Address at Address 0 (TV)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adr00 {
    ///0: Does not set as local address.
    _0 = 0,
    ///1: Sets as local address.
    _1 = 1,
}
impl From<Adr00> for bool {
    #[inline(always)]
    fn from(variant: Adr00) -> Self {
        variant as u8 != 0
    }
}
///Field `ADR00` reader - Local Address at Address 0 (TV)
pub type Adr00R = crate::BitReader<Adr00>;
impl Adr00R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Adr00 {
        match self.bits {
            false => Adr00::_0,
            true => Adr00::_1,
        }
    }
    ///Does not set as local address.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Adr00::_0
    }
    ///Sets as local address.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Adr00::_1
    }
}
///Field `ADR00` writer - Local Address at Address 0 (TV)
pub type Adr00W<'a, REG> = crate::BitWriter<'a, REG, Adr00>;
impl<'a, REG> Adr00W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Does not set as local address.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Adr00::_0)
    }
    ///Sets as local address.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Adr00::_1)
    }
}
/**Local Address Setting at Address 1 (recording device 1)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adr01 {
    ///0: Does not set as local address.
    _0 = 0,
    ///1: Sets as local address.
    _1 = 1,
}
impl From<Adr01> for bool {
    #[inline(always)]
    fn from(variant: Adr01) -> Self {
        variant as u8 != 0
    }
}
///Field `ADR01` reader - Local Address Setting at Address 1 (recording device 1)
pub type Adr01R = crate::BitReader<Adr01>;
impl Adr01R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Adr01 {
        match self.bits {
            false => Adr01::_0,
            true => Adr01::_1,
        }
    }
    ///Does not set as local address.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Adr01::_0
    }
    ///Sets as local address.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Adr01::_1
    }
}
///Field `ADR01` writer - Local Address Setting at Address 1 (recording device 1)
pub type Adr01W<'a, REG> = crate::BitWriter<'a, REG, Adr01>;
impl<'a, REG> Adr01W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Does not set as local address.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Adr01::_0)
    }
    ///Sets as local address.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Adr01::_1)
    }
}
/**Local Address Setting at Address 2 (recording device 2)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adr02 {
    ///0: Does not set as local address.
    _0 = 0,
    ///1: Sets as local address.
    _1 = 1,
}
impl From<Adr02> for bool {
    #[inline(always)]
    fn from(variant: Adr02) -> Self {
        variant as u8 != 0
    }
}
///Field `ADR02` reader - Local Address Setting at Address 2 (recording device 2)
pub type Adr02R = crate::BitReader<Adr02>;
impl Adr02R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Adr02 {
        match self.bits {
            false => Adr02::_0,
            true => Adr02::_1,
        }
    }
    ///Does not set as local address.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Adr02::_0
    }
    ///Sets as local address.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Adr02::_1
    }
}
///Field `ADR02` writer - Local Address Setting at Address 2 (recording device 2)
pub type Adr02W<'a, REG> = crate::BitWriter<'a, REG, Adr02>;
impl<'a, REG> Adr02W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Does not set as local address.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Adr02::_0)
    }
    ///Sets as local address.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Adr02::_1)
    }
}
/**Local Address Setting at Address 3 (tuner 1)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adr03 {
    ///0: Does not set as local address.
    _0 = 0,
    ///1: Sets as local address.
    _1 = 1,
}
impl From<Adr03> for bool {
    #[inline(always)]
    fn from(variant: Adr03) -> Self {
        variant as u8 != 0
    }
}
///Field `ADR03` reader - Local Address Setting at Address 3 (tuner 1)
pub type Adr03R = crate::BitReader<Adr03>;
impl Adr03R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Adr03 {
        match self.bits {
            false => Adr03::_0,
            true => Adr03::_1,
        }
    }
    ///Does not set as local address.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Adr03::_0
    }
    ///Sets as local address.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Adr03::_1
    }
}
///Field `ADR03` writer - Local Address Setting at Address 3 (tuner 1)
pub type Adr03W<'a, REG> = crate::BitWriter<'a, REG, Adr03>;
impl<'a, REG> Adr03W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Does not set as local address.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Adr03::_0)
    }
    ///Sets as local address.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Adr03::_1)
    }
}
/**Local Address Setting at Address 4 (playback device 1)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adr04 {
    ///0: Does not set as local address.
    _0 = 0,
    ///1: Sets as local address.
    _1 = 1,
}
impl From<Adr04> for bool {
    #[inline(always)]
    fn from(variant: Adr04) -> Self {
        variant as u8 != 0
    }
}
///Field `ADR04` reader - Local Address Setting at Address 4 (playback device 1)
pub type Adr04R = crate::BitReader<Adr04>;
impl Adr04R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Adr04 {
        match self.bits {
            false => Adr04::_0,
            true => Adr04::_1,
        }
    }
    ///Does not set as local address.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Adr04::_0
    }
    ///Sets as local address.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Adr04::_1
    }
}
///Field `ADR04` writer - Local Address Setting at Address 4 (playback device 1)
pub type Adr04W<'a, REG> = crate::BitWriter<'a, REG, Adr04>;
impl<'a, REG> Adr04W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Does not set as local address.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Adr04::_0)
    }
    ///Sets as local address.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Adr04::_1)
    }
}
/**Local Address Setting at Address 5 (audio system)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adr05 {
    ///0: Does not set as local address.
    _0 = 0,
    ///1: Sets as local address.
    _1 = 1,
}
impl From<Adr05> for bool {
    #[inline(always)]
    fn from(variant: Adr05) -> Self {
        variant as u8 != 0
    }
}
///Field `ADR05` reader - Local Address Setting at Address 5 (audio system)
pub type Adr05R = crate::BitReader<Adr05>;
impl Adr05R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Adr05 {
        match self.bits {
            false => Adr05::_0,
            true => Adr05::_1,
        }
    }
    ///Does not set as local address.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Adr05::_0
    }
    ///Sets as local address.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Adr05::_1
    }
}
///Field `ADR05` writer - Local Address Setting at Address 5 (audio system)
pub type Adr05W<'a, REG> = crate::BitWriter<'a, REG, Adr05>;
impl<'a, REG> Adr05W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Does not set as local address.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Adr05::_0)
    }
    ///Sets as local address.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Adr05::_1)
    }
}
/**Local Address Setting at Address 6 (tuner 2)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adr06 {
    ///0: Does not set as local address.
    _0 = 0,
    ///1: Sets as local address.
    _1 = 1,
}
impl From<Adr06> for bool {
    #[inline(always)]
    fn from(variant: Adr06) -> Self {
        variant as u8 != 0
    }
}
///Field `ADR06` reader - Local Address Setting at Address 6 (tuner 2)
pub type Adr06R = crate::BitReader<Adr06>;
impl Adr06R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Adr06 {
        match self.bits {
            false => Adr06::_0,
            true => Adr06::_1,
        }
    }
    ///Does not set as local address.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Adr06::_0
    }
    ///Sets as local address.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Adr06::_1
    }
}
///Field `ADR06` writer - Local Address Setting at Address 6 (tuner 2)
pub type Adr06W<'a, REG> = crate::BitWriter<'a, REG, Adr06>;
impl<'a, REG> Adr06W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Does not set as local address.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Adr06::_0)
    }
    ///Sets as local address.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Adr06::_1)
    }
}
/**Local Address Setting at Address 7 (tuner 3)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adr07 {
    ///0: Does not set as local address.
    _0 = 0,
    ///1: Sets as local address.
    _1 = 1,
}
impl From<Adr07> for bool {
    #[inline(always)]
    fn from(variant: Adr07) -> Self {
        variant as u8 != 0
    }
}
///Field `ADR07` reader - Local Address Setting at Address 7 (tuner 3)
pub type Adr07R = crate::BitReader<Adr07>;
impl Adr07R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Adr07 {
        match self.bits {
            false => Adr07::_0,
            true => Adr07::_1,
        }
    }
    ///Does not set as local address.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Adr07::_0
    }
    ///Sets as local address.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Adr07::_1
    }
}
///Field `ADR07` writer - Local Address Setting at Address 7 (tuner 3)
pub type Adr07W<'a, REG> = crate::BitWriter<'a, REG, Adr07>;
impl<'a, REG> Adr07W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Does not set as local address.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Adr07::_0)
    }
    ///Sets as local address.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Adr07::_1)
    }
}
/**Local Address Setting at Address 8 (playback device 2)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adr08 {
    ///0: Does not set as local address.
    _0 = 0,
    ///1: Sets as local address.
    _1 = 1,
}
impl From<Adr08> for bool {
    #[inline(always)]
    fn from(variant: Adr08) -> Self {
        variant as u8 != 0
    }
}
///Field `ADR08` reader - Local Address Setting at Address 8 (playback device 2)
pub type Adr08R = crate::BitReader<Adr08>;
impl Adr08R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Adr08 {
        match self.bits {
            false => Adr08::_0,
            true => Adr08::_1,
        }
    }
    ///Does not set as local address.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Adr08::_0
    }
    ///Sets as local address.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Adr08::_1
    }
}
///Field `ADR08` writer - Local Address Setting at Address 8 (playback device 2)
pub type Adr08W<'a, REG> = crate::BitWriter<'a, REG, Adr08>;
impl<'a, REG> Adr08W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Does not set as local address.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Adr08::_0)
    }
    ///Sets as local address.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Adr08::_1)
    }
}
/**Local Address Setting at Address 9 (recording device 3)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adr09 {
    ///0: Does not set as local address.
    _0 = 0,
    ///1: Sets as local address.
    _1 = 1,
}
impl From<Adr09> for bool {
    #[inline(always)]
    fn from(variant: Adr09) -> Self {
        variant as u8 != 0
    }
}
///Field `ADR09` reader - Local Address Setting at Address 9 (recording device 3)
pub type Adr09R = crate::BitReader<Adr09>;
impl Adr09R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Adr09 {
        match self.bits {
            false => Adr09::_0,
            true => Adr09::_1,
        }
    }
    ///Does not set as local address.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Adr09::_0
    }
    ///Sets as local address.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Adr09::_1
    }
}
///Field `ADR09` writer - Local Address Setting at Address 9 (recording device 3)
pub type Adr09W<'a, REG> = crate::BitWriter<'a, REG, Adr09>;
impl<'a, REG> Adr09W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Does not set as local address.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Adr09::_0)
    }
    ///Sets as local address.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Adr09::_1)
    }
}
/**Local Address Setting at Address 10 (tuner 4)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adr10 {
    ///0: Does not set as local address.
    _0 = 0,
    ///1: Sets as local address.
    _1 = 1,
}
impl From<Adr10> for bool {
    #[inline(always)]
    fn from(variant: Adr10) -> Self {
        variant as u8 != 0
    }
}
///Field `ADR10` reader - Local Address Setting at Address 10 (tuner 4)
pub type Adr10R = crate::BitReader<Adr10>;
impl Adr10R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Adr10 {
        match self.bits {
            false => Adr10::_0,
            true => Adr10::_1,
        }
    }
    ///Does not set as local address.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Adr10::_0
    }
    ///Sets as local address.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Adr10::_1
    }
}
///Field `ADR10` writer - Local Address Setting at Address 10 (tuner 4)
pub type Adr10W<'a, REG> = crate::BitWriter<'a, REG, Adr10>;
impl<'a, REG> Adr10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Does not set as local address.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Adr10::_0)
    }
    ///Sets as local address.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Adr10::_1)
    }
}
/**Local Address Setting at Address 11 (playback device 3)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adr11 {
    ///0: Does not set as local address.
    _0 = 0,
    ///1: Sets as local address.
    _1 = 1,
}
impl From<Adr11> for bool {
    #[inline(always)]
    fn from(variant: Adr11) -> Self {
        variant as u8 != 0
    }
}
///Field `ADR11` reader - Local Address Setting at Address 11 (playback device 3)
pub type Adr11R = crate::BitReader<Adr11>;
impl Adr11R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Adr11 {
        match self.bits {
            false => Adr11::_0,
            true => Adr11::_1,
        }
    }
    ///Does not set as local address.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Adr11::_0
    }
    ///Sets as local address.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Adr11::_1
    }
}
///Field `ADR11` writer - Local Address Setting at Address 11 (playback device 3)
pub type Adr11W<'a, REG> = crate::BitWriter<'a, REG, Adr11>;
impl<'a, REG> Adr11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Does not set as local address.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Adr11::_0)
    }
    ///Sets as local address.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Adr11::_1)
    }
}
/**Local Address Setting at Address 12 (reserved)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adr12 {
    ///0: Does not set as local address.
    _0 = 0,
    ///1: Sets as local address.
    _1 = 1,
}
impl From<Adr12> for bool {
    #[inline(always)]
    fn from(variant: Adr12) -> Self {
        variant as u8 != 0
    }
}
///Field `ADR12` reader - Local Address Setting at Address 12 (reserved)
pub type Adr12R = crate::BitReader<Adr12>;
impl Adr12R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Adr12 {
        match self.bits {
            false => Adr12::_0,
            true => Adr12::_1,
        }
    }
    ///Does not set as local address.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Adr12::_0
    }
    ///Sets as local address.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Adr12::_1
    }
}
///Field `ADR12` writer - Local Address Setting at Address 12 (reserved)
pub type Adr12W<'a, REG> = crate::BitWriter<'a, REG, Adr12>;
impl<'a, REG> Adr12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Does not set as local address.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Adr12::_0)
    }
    ///Sets as local address.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Adr12::_1)
    }
}
/**Local Address Setting at Address 13 (reserved)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adr13 {
    ///0: Does not set as local address.
    _0 = 0,
    ///1: Sets as local address.
    _1 = 1,
}
impl From<Adr13> for bool {
    #[inline(always)]
    fn from(variant: Adr13) -> Self {
        variant as u8 != 0
    }
}
///Field `ADR13` reader - Local Address Setting at Address 13 (reserved)
pub type Adr13R = crate::BitReader<Adr13>;
impl Adr13R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Adr13 {
        match self.bits {
            false => Adr13::_0,
            true => Adr13::_1,
        }
    }
    ///Does not set as local address.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Adr13::_0
    }
    ///Sets as local address.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Adr13::_1
    }
}
///Field `ADR13` writer - Local Address Setting at Address 13 (reserved)
pub type Adr13W<'a, REG> = crate::BitWriter<'a, REG, Adr13>;
impl<'a, REG> Adr13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Does not set as local address.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Adr13::_0)
    }
    ///Sets as local address.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Adr13::_1)
    }
}
/**Local Address Setting at Address 14 (specific use)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adr14 {
    ///0: Does not set as local address.
    _0 = 0,
    ///1: Sets as local address.
    _1 = 1,
}
impl From<Adr14> for bool {
    #[inline(always)]
    fn from(variant: Adr14) -> Self {
        variant as u8 != 0
    }
}
///Field `ADR14` reader - Local Address Setting at Address 14 (specific use)
pub type Adr14R = crate::BitReader<Adr14>;
impl Adr14R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Adr14 {
        match self.bits {
            false => Adr14::_0,
            true => Adr14::_1,
        }
    }
    ///Does not set as local address.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Adr14::_0
    }
    ///Sets as local address.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Adr14::_1
    }
}
///Field `ADR14` writer - Local Address Setting at Address 14 (specific use)
pub type Adr14W<'a, REG> = crate::BitWriter<'a, REG, Adr14>;
impl<'a, REG> Adr14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Does not set as local address.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Adr14::_0)
    }
    ///Sets as local address.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Adr14::_1)
    }
}
impl R {
    ///Bit 0 - Local Address at Address 0 (TV)
    #[inline(always)]
    pub fn adr00(&self) -> Adr00R {
        Adr00R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Local Address Setting at Address 1 (recording device 1)
    #[inline(always)]
    pub fn adr01(&self) -> Adr01R {
        Adr01R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Local Address Setting at Address 2 (recording device 2)
    #[inline(always)]
    pub fn adr02(&self) -> Adr02R {
        Adr02R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Local Address Setting at Address 3 (tuner 1)
    #[inline(always)]
    pub fn adr03(&self) -> Adr03R {
        Adr03R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Local Address Setting at Address 4 (playback device 1)
    #[inline(always)]
    pub fn adr04(&self) -> Adr04R {
        Adr04R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Local Address Setting at Address 5 (audio system)
    #[inline(always)]
    pub fn adr05(&self) -> Adr05R {
        Adr05R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Local Address Setting at Address 6 (tuner 2)
    #[inline(always)]
    pub fn adr06(&self) -> Adr06R {
        Adr06R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Local Address Setting at Address 7 (tuner 3)
    #[inline(always)]
    pub fn adr07(&self) -> Adr07R {
        Adr07R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Local Address Setting at Address 8 (playback device 2)
    #[inline(always)]
    pub fn adr08(&self) -> Adr08R {
        Adr08R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Local Address Setting at Address 9 (recording device 3)
    #[inline(always)]
    pub fn adr09(&self) -> Adr09R {
        Adr09R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Local Address Setting at Address 10 (tuner 4)
    #[inline(always)]
    pub fn adr10(&self) -> Adr10R {
        Adr10R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Local Address Setting at Address 11 (playback device 3)
    #[inline(always)]
    pub fn adr11(&self) -> Adr11R {
        Adr11R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Local Address Setting at Address 12 (reserved)
    #[inline(always)]
    pub fn adr12(&self) -> Adr12R {
        Adr12R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Local Address Setting at Address 13 (reserved)
    #[inline(always)]
    pub fn adr13(&self) -> Adr13R {
        Adr13R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Local Address Setting at Address 14 (specific use)
    #[inline(always)]
    pub fn adr14(&self) -> Adr14R {
        Adr14R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CADR")
            .field("adr00", &self.adr00())
            .field("adr01", &self.adr01())
            .field("adr02", &self.adr02())
            .field("adr03", &self.adr03())
            .field("adr04", &self.adr04())
            .field("adr05", &self.adr05())
            .field("adr06", &self.adr06())
            .field("adr07", &self.adr07())
            .field("adr08", &self.adr08())
            .field("adr09", &self.adr09())
            .field("adr10", &self.adr10())
            .field("adr11", &self.adr11())
            .field("adr12", &self.adr12())
            .field("adr13", &self.adr13())
            .field("adr14", &self.adr14())
            .finish()
    }
}
impl W {
    ///Bit 0 - Local Address at Address 0 (TV)
    #[inline(always)]
    pub fn adr00(&mut self) -> Adr00W<CadrSpec> {
        Adr00W::new(self, 0)
    }
    ///Bit 1 - Local Address Setting at Address 1 (recording device 1)
    #[inline(always)]
    pub fn adr01(&mut self) -> Adr01W<CadrSpec> {
        Adr01W::new(self, 1)
    }
    ///Bit 2 - Local Address Setting at Address 2 (recording device 2)
    #[inline(always)]
    pub fn adr02(&mut self) -> Adr02W<CadrSpec> {
        Adr02W::new(self, 2)
    }
    ///Bit 3 - Local Address Setting at Address 3 (tuner 1)
    #[inline(always)]
    pub fn adr03(&mut self) -> Adr03W<CadrSpec> {
        Adr03W::new(self, 3)
    }
    ///Bit 4 - Local Address Setting at Address 4 (playback device 1)
    #[inline(always)]
    pub fn adr04(&mut self) -> Adr04W<CadrSpec> {
        Adr04W::new(self, 4)
    }
    ///Bit 5 - Local Address Setting at Address 5 (audio system)
    #[inline(always)]
    pub fn adr05(&mut self) -> Adr05W<CadrSpec> {
        Adr05W::new(self, 5)
    }
    ///Bit 6 - Local Address Setting at Address 6 (tuner 2)
    #[inline(always)]
    pub fn adr06(&mut self) -> Adr06W<CadrSpec> {
        Adr06W::new(self, 6)
    }
    ///Bit 7 - Local Address Setting at Address 7 (tuner 3)
    #[inline(always)]
    pub fn adr07(&mut self) -> Adr07W<CadrSpec> {
        Adr07W::new(self, 7)
    }
    ///Bit 8 - Local Address Setting at Address 8 (playback device 2)
    #[inline(always)]
    pub fn adr08(&mut self) -> Adr08W<CadrSpec> {
        Adr08W::new(self, 8)
    }
    ///Bit 9 - Local Address Setting at Address 9 (recording device 3)
    #[inline(always)]
    pub fn adr09(&mut self) -> Adr09W<CadrSpec> {
        Adr09W::new(self, 9)
    }
    ///Bit 10 - Local Address Setting at Address 10 (tuner 4)
    #[inline(always)]
    pub fn adr10(&mut self) -> Adr10W<CadrSpec> {
        Adr10W::new(self, 10)
    }
    ///Bit 11 - Local Address Setting at Address 11 (playback device 3)
    #[inline(always)]
    pub fn adr11(&mut self) -> Adr11W<CadrSpec> {
        Adr11W::new(self, 11)
    }
    ///Bit 12 - Local Address Setting at Address 12 (reserved)
    #[inline(always)]
    pub fn adr12(&mut self) -> Adr12W<CadrSpec> {
        Adr12W::new(self, 12)
    }
    ///Bit 13 - Local Address Setting at Address 13 (reserved)
    #[inline(always)]
    pub fn adr13(&mut self) -> Adr13W<CadrSpec> {
        Adr13W::new(self, 13)
    }
    ///Bit 14 - Local Address Setting at Address 14 (specific use)
    #[inline(always)]
    pub fn adr14(&mut self) -> Adr14W<CadrSpec> {
        Adr14W::new(self, 14)
    }
}
/**CEC Local Address Setting Register

You can [`read`](crate::Reg::read) this register and get [`cadr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cadr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CadrSpec;
impl crate::RegisterSpec for CadrSpec {
    type Ux = u16;
}
///`read()` method returns [`cadr::R`](R) reader structure
impl crate::Readable for CadrSpec {}
///`write(|w| ..)` method takes [`cadr::W`](W) writer structure
impl crate::Writable for CadrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CADR to value 0
impl crate::Resettable for CadrSpec {}
