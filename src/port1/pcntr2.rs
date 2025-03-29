///Register `PCNTR2` reader
pub type R = crate::R<Pcntr2Spec>;
/**Pmn State

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pidr00 {
    ///0: Low level
    _0 = 0,
    ///1: High level
    _1 = 1,
}
impl From<Pidr00> for bool {
    #[inline(always)]
    fn from(variant: Pidr00) -> Self {
        variant as u8 != 0
    }
}
///Field `PIDR00` reader - Pmn State
pub type Pidr00R = crate::BitReader<Pidr00>;
impl Pidr00R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pidr00 {
        match self.bits {
            false => Pidr00::_0,
            true => Pidr00::_1,
        }
    }
    ///Low level
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pidr00::_0
    }
    ///High level
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pidr00::_1
    }
}
/**Pmn State

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pidr01 {
    ///0: Low level
    _0 = 0,
    ///1: High level
    _1 = 1,
}
impl From<Pidr01> for bool {
    #[inline(always)]
    fn from(variant: Pidr01) -> Self {
        variant as u8 != 0
    }
}
///Field `PIDR01` reader - Pmn State
pub type Pidr01R = crate::BitReader<Pidr01>;
impl Pidr01R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pidr01 {
        match self.bits {
            false => Pidr01::_0,
            true => Pidr01::_1,
        }
    }
    ///Low level
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pidr01::_0
    }
    ///High level
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pidr01::_1
    }
}
/**Pmn State

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pidr02 {
    ///0: Low level
    _0 = 0,
    ///1: High level
    _1 = 1,
}
impl From<Pidr02> for bool {
    #[inline(always)]
    fn from(variant: Pidr02) -> Self {
        variant as u8 != 0
    }
}
///Field `PIDR02` reader - Pmn State
pub type Pidr02R = crate::BitReader<Pidr02>;
impl Pidr02R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pidr02 {
        match self.bits {
            false => Pidr02::_0,
            true => Pidr02::_1,
        }
    }
    ///Low level
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pidr02::_0
    }
    ///High level
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pidr02::_1
    }
}
/**Pmn State

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pidr03 {
    ///0: Low level
    _0 = 0,
    ///1: High level
    _1 = 1,
}
impl From<Pidr03> for bool {
    #[inline(always)]
    fn from(variant: Pidr03) -> Self {
        variant as u8 != 0
    }
}
///Field `PIDR03` reader - Pmn State
pub type Pidr03R = crate::BitReader<Pidr03>;
impl Pidr03R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pidr03 {
        match self.bits {
            false => Pidr03::_0,
            true => Pidr03::_1,
        }
    }
    ///Low level
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pidr03::_0
    }
    ///High level
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pidr03::_1
    }
}
/**Pmn State

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pidr04 {
    ///0: Low level
    _0 = 0,
    ///1: High level
    _1 = 1,
}
impl From<Pidr04> for bool {
    #[inline(always)]
    fn from(variant: Pidr04) -> Self {
        variant as u8 != 0
    }
}
///Field `PIDR04` reader - Pmn State
pub type Pidr04R = crate::BitReader<Pidr04>;
impl Pidr04R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pidr04 {
        match self.bits {
            false => Pidr04::_0,
            true => Pidr04::_1,
        }
    }
    ///Low level
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pidr04::_0
    }
    ///High level
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pidr04::_1
    }
}
/**Pmn State

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pidr05 {
    ///0: Low level
    _0 = 0,
    ///1: High level
    _1 = 1,
}
impl From<Pidr05> for bool {
    #[inline(always)]
    fn from(variant: Pidr05) -> Self {
        variant as u8 != 0
    }
}
///Field `PIDR05` reader - Pmn State
pub type Pidr05R = crate::BitReader<Pidr05>;
impl Pidr05R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pidr05 {
        match self.bits {
            false => Pidr05::_0,
            true => Pidr05::_1,
        }
    }
    ///Low level
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pidr05::_0
    }
    ///High level
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pidr05::_1
    }
}
/**Pmn State

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pidr06 {
    ///0: Low level
    _0 = 0,
    ///1: High level
    _1 = 1,
}
impl From<Pidr06> for bool {
    #[inline(always)]
    fn from(variant: Pidr06) -> Self {
        variant as u8 != 0
    }
}
///Field `PIDR06` reader - Pmn State
pub type Pidr06R = crate::BitReader<Pidr06>;
impl Pidr06R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pidr06 {
        match self.bits {
            false => Pidr06::_0,
            true => Pidr06::_1,
        }
    }
    ///Low level
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pidr06::_0
    }
    ///High level
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pidr06::_1
    }
}
/**Pmn State

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pidr07 {
    ///0: Low level
    _0 = 0,
    ///1: High level
    _1 = 1,
}
impl From<Pidr07> for bool {
    #[inline(always)]
    fn from(variant: Pidr07) -> Self {
        variant as u8 != 0
    }
}
///Field `PIDR07` reader - Pmn State
pub type Pidr07R = crate::BitReader<Pidr07>;
impl Pidr07R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pidr07 {
        match self.bits {
            false => Pidr07::_0,
            true => Pidr07::_1,
        }
    }
    ///Low level
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pidr07::_0
    }
    ///High level
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pidr07::_1
    }
}
/**Pmn State

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pidr08 {
    ///0: Low level
    _0 = 0,
    ///1: High level
    _1 = 1,
}
impl From<Pidr08> for bool {
    #[inline(always)]
    fn from(variant: Pidr08) -> Self {
        variant as u8 != 0
    }
}
///Field `PIDR08` reader - Pmn State
pub type Pidr08R = crate::BitReader<Pidr08>;
impl Pidr08R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pidr08 {
        match self.bits {
            false => Pidr08::_0,
            true => Pidr08::_1,
        }
    }
    ///Low level
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pidr08::_0
    }
    ///High level
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pidr08::_1
    }
}
/**Pmn State

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pidr09 {
    ///0: Low level
    _0 = 0,
    ///1: High level
    _1 = 1,
}
impl From<Pidr09> for bool {
    #[inline(always)]
    fn from(variant: Pidr09) -> Self {
        variant as u8 != 0
    }
}
///Field `PIDR09` reader - Pmn State
pub type Pidr09R = crate::BitReader<Pidr09>;
impl Pidr09R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pidr09 {
        match self.bits {
            false => Pidr09::_0,
            true => Pidr09::_1,
        }
    }
    ///Low level
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pidr09::_0
    }
    ///High level
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pidr09::_1
    }
}
/**Pmn State

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pidr10 {
    ///0: Low level
    _0 = 0,
    ///1: High level
    _1 = 1,
}
impl From<Pidr10> for bool {
    #[inline(always)]
    fn from(variant: Pidr10) -> Self {
        variant as u8 != 0
    }
}
///Field `PIDR10` reader - Pmn State
pub type Pidr10R = crate::BitReader<Pidr10>;
impl Pidr10R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pidr10 {
        match self.bits {
            false => Pidr10::_0,
            true => Pidr10::_1,
        }
    }
    ///Low level
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pidr10::_0
    }
    ///High level
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pidr10::_1
    }
}
/**Pmn State

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pidr11 {
    ///0: Low level
    _0 = 0,
    ///1: High level
    _1 = 1,
}
impl From<Pidr11> for bool {
    #[inline(always)]
    fn from(variant: Pidr11) -> Self {
        variant as u8 != 0
    }
}
///Field `PIDR11` reader - Pmn State
pub type Pidr11R = crate::BitReader<Pidr11>;
impl Pidr11R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pidr11 {
        match self.bits {
            false => Pidr11::_0,
            true => Pidr11::_1,
        }
    }
    ///Low level
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pidr11::_0
    }
    ///High level
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pidr11::_1
    }
}
/**Pmn State

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pidr12 {
    ///0: Low level
    _0 = 0,
    ///1: High level
    _1 = 1,
}
impl From<Pidr12> for bool {
    #[inline(always)]
    fn from(variant: Pidr12) -> Self {
        variant as u8 != 0
    }
}
///Field `PIDR12` reader - Pmn State
pub type Pidr12R = crate::BitReader<Pidr12>;
impl Pidr12R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pidr12 {
        match self.bits {
            false => Pidr12::_0,
            true => Pidr12::_1,
        }
    }
    ///Low level
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pidr12::_0
    }
    ///High level
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pidr12::_1
    }
}
/**Pmn State

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pidr13 {
    ///0: Low level
    _0 = 0,
    ///1: High level
    _1 = 1,
}
impl From<Pidr13> for bool {
    #[inline(always)]
    fn from(variant: Pidr13) -> Self {
        variant as u8 != 0
    }
}
///Field `PIDR13` reader - Pmn State
pub type Pidr13R = crate::BitReader<Pidr13>;
impl Pidr13R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pidr13 {
        match self.bits {
            false => Pidr13::_0,
            true => Pidr13::_1,
        }
    }
    ///Low level
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pidr13::_0
    }
    ///High level
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pidr13::_1
    }
}
/**Pmn State

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pidr14 {
    ///0: Low level
    _0 = 0,
    ///1: High level
    _1 = 1,
}
impl From<Pidr14> for bool {
    #[inline(always)]
    fn from(variant: Pidr14) -> Self {
        variant as u8 != 0
    }
}
///Field `PIDR14` reader - Pmn State
pub type Pidr14R = crate::BitReader<Pidr14>;
impl Pidr14R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pidr14 {
        match self.bits {
            false => Pidr14::_0,
            true => Pidr14::_1,
        }
    }
    ///Low level
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pidr14::_0
    }
    ///High level
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pidr14::_1
    }
}
/**Pmn State

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pidr15 {
    ///0: Low level
    _0 = 0,
    ///1: High level
    _1 = 1,
}
impl From<Pidr15> for bool {
    #[inline(always)]
    fn from(variant: Pidr15) -> Self {
        variant as u8 != 0
    }
}
///Field `PIDR15` reader - Pmn State
pub type Pidr15R = crate::BitReader<Pidr15>;
impl Pidr15R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pidr15 {
        match self.bits {
            false => Pidr15::_0,
            true => Pidr15::_1,
        }
    }
    ///Low level
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pidr15::_0
    }
    ///High level
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pidr15::_1
    }
}
/**Port Event Input Data

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eidr00 {
    ///0: Low input
    _0 = 0,
    ///1: High input
    _1 = 1,
}
impl From<Eidr00> for bool {
    #[inline(always)]
    fn from(variant: Eidr00) -> Self {
        variant as u8 != 0
    }
}
///Field `EIDR00` reader - Port Event Input Data
pub type Eidr00R = crate::BitReader<Eidr00>;
impl Eidr00R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eidr00 {
        match self.bits {
            false => Eidr00::_0,
            true => Eidr00::_1,
        }
    }
    ///Low input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eidr00::_0
    }
    ///High input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eidr00::_1
    }
}
/**Port Event Input Data

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eidr01 {
    ///0: Low input
    _0 = 0,
    ///1: High input
    _1 = 1,
}
impl From<Eidr01> for bool {
    #[inline(always)]
    fn from(variant: Eidr01) -> Self {
        variant as u8 != 0
    }
}
///Field `EIDR01` reader - Port Event Input Data
pub type Eidr01R = crate::BitReader<Eidr01>;
impl Eidr01R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eidr01 {
        match self.bits {
            false => Eidr01::_0,
            true => Eidr01::_1,
        }
    }
    ///Low input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eidr01::_0
    }
    ///High input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eidr01::_1
    }
}
/**Port Event Input Data

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eidr02 {
    ///0: Low input
    _0 = 0,
    ///1: High input
    _1 = 1,
}
impl From<Eidr02> for bool {
    #[inline(always)]
    fn from(variant: Eidr02) -> Self {
        variant as u8 != 0
    }
}
///Field `EIDR02` reader - Port Event Input Data
pub type Eidr02R = crate::BitReader<Eidr02>;
impl Eidr02R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eidr02 {
        match self.bits {
            false => Eidr02::_0,
            true => Eidr02::_1,
        }
    }
    ///Low input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eidr02::_0
    }
    ///High input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eidr02::_1
    }
}
/**Port Event Input Data

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eidr03 {
    ///0: Low input
    _0 = 0,
    ///1: High input
    _1 = 1,
}
impl From<Eidr03> for bool {
    #[inline(always)]
    fn from(variant: Eidr03) -> Self {
        variant as u8 != 0
    }
}
///Field `EIDR03` reader - Port Event Input Data
pub type Eidr03R = crate::BitReader<Eidr03>;
impl Eidr03R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eidr03 {
        match self.bits {
            false => Eidr03::_0,
            true => Eidr03::_1,
        }
    }
    ///Low input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eidr03::_0
    }
    ///High input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eidr03::_1
    }
}
/**Port Event Input Data

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eidr04 {
    ///0: Low input
    _0 = 0,
    ///1: High input
    _1 = 1,
}
impl From<Eidr04> for bool {
    #[inline(always)]
    fn from(variant: Eidr04) -> Self {
        variant as u8 != 0
    }
}
///Field `EIDR04` reader - Port Event Input Data
pub type Eidr04R = crate::BitReader<Eidr04>;
impl Eidr04R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eidr04 {
        match self.bits {
            false => Eidr04::_0,
            true => Eidr04::_1,
        }
    }
    ///Low input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eidr04::_0
    }
    ///High input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eidr04::_1
    }
}
/**Port Event Input Data

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eidr05 {
    ///0: Low input
    _0 = 0,
    ///1: High input
    _1 = 1,
}
impl From<Eidr05> for bool {
    #[inline(always)]
    fn from(variant: Eidr05) -> Self {
        variant as u8 != 0
    }
}
///Field `EIDR05` reader - Port Event Input Data
pub type Eidr05R = crate::BitReader<Eidr05>;
impl Eidr05R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eidr05 {
        match self.bits {
            false => Eidr05::_0,
            true => Eidr05::_1,
        }
    }
    ///Low input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eidr05::_0
    }
    ///High input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eidr05::_1
    }
}
/**Port Event Input Data

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eidr06 {
    ///0: Low input
    _0 = 0,
    ///1: High input
    _1 = 1,
}
impl From<Eidr06> for bool {
    #[inline(always)]
    fn from(variant: Eidr06) -> Self {
        variant as u8 != 0
    }
}
///Field `EIDR06` reader - Port Event Input Data
pub type Eidr06R = crate::BitReader<Eidr06>;
impl Eidr06R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eidr06 {
        match self.bits {
            false => Eidr06::_0,
            true => Eidr06::_1,
        }
    }
    ///Low input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eidr06::_0
    }
    ///High input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eidr06::_1
    }
}
/**Port Event Input Data

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eidr07 {
    ///0: Low input
    _0 = 0,
    ///1: High input
    _1 = 1,
}
impl From<Eidr07> for bool {
    #[inline(always)]
    fn from(variant: Eidr07) -> Self {
        variant as u8 != 0
    }
}
///Field `EIDR07` reader - Port Event Input Data
pub type Eidr07R = crate::BitReader<Eidr07>;
impl Eidr07R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eidr07 {
        match self.bits {
            false => Eidr07::_0,
            true => Eidr07::_1,
        }
    }
    ///Low input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eidr07::_0
    }
    ///High input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eidr07::_1
    }
}
/**Port Event Input Data

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eidr08 {
    ///0: Low input
    _0 = 0,
    ///1: High input
    _1 = 1,
}
impl From<Eidr08> for bool {
    #[inline(always)]
    fn from(variant: Eidr08) -> Self {
        variant as u8 != 0
    }
}
///Field `EIDR08` reader - Port Event Input Data
pub type Eidr08R = crate::BitReader<Eidr08>;
impl Eidr08R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eidr08 {
        match self.bits {
            false => Eidr08::_0,
            true => Eidr08::_1,
        }
    }
    ///Low input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eidr08::_0
    }
    ///High input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eidr08::_1
    }
}
/**Port Event Input Data

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eidr09 {
    ///0: Low input
    _0 = 0,
    ///1: High input
    _1 = 1,
}
impl From<Eidr09> for bool {
    #[inline(always)]
    fn from(variant: Eidr09) -> Self {
        variant as u8 != 0
    }
}
///Field `EIDR09` reader - Port Event Input Data
pub type Eidr09R = crate::BitReader<Eidr09>;
impl Eidr09R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eidr09 {
        match self.bits {
            false => Eidr09::_0,
            true => Eidr09::_1,
        }
    }
    ///Low input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eidr09::_0
    }
    ///High input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eidr09::_1
    }
}
/**Port Event Input Data

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eidr10 {
    ///0: Low input
    _0 = 0,
    ///1: High input
    _1 = 1,
}
impl From<Eidr10> for bool {
    #[inline(always)]
    fn from(variant: Eidr10) -> Self {
        variant as u8 != 0
    }
}
///Field `EIDR10` reader - Port Event Input Data
pub type Eidr10R = crate::BitReader<Eidr10>;
impl Eidr10R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eidr10 {
        match self.bits {
            false => Eidr10::_0,
            true => Eidr10::_1,
        }
    }
    ///Low input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eidr10::_0
    }
    ///High input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eidr10::_1
    }
}
/**Port Event Input Data

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eidr11 {
    ///0: Low input
    _0 = 0,
    ///1: High input
    _1 = 1,
}
impl From<Eidr11> for bool {
    #[inline(always)]
    fn from(variant: Eidr11) -> Self {
        variant as u8 != 0
    }
}
///Field `EIDR11` reader - Port Event Input Data
pub type Eidr11R = crate::BitReader<Eidr11>;
impl Eidr11R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eidr11 {
        match self.bits {
            false => Eidr11::_0,
            true => Eidr11::_1,
        }
    }
    ///Low input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eidr11::_0
    }
    ///High input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eidr11::_1
    }
}
/**Port Event Input Data

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eidr12 {
    ///0: Low input
    _0 = 0,
    ///1: High input
    _1 = 1,
}
impl From<Eidr12> for bool {
    #[inline(always)]
    fn from(variant: Eidr12) -> Self {
        variant as u8 != 0
    }
}
///Field `EIDR12` reader - Port Event Input Data
pub type Eidr12R = crate::BitReader<Eidr12>;
impl Eidr12R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eidr12 {
        match self.bits {
            false => Eidr12::_0,
            true => Eidr12::_1,
        }
    }
    ///Low input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eidr12::_0
    }
    ///High input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eidr12::_1
    }
}
/**Port Event Input Data

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eidr13 {
    ///0: Low input
    _0 = 0,
    ///1: High input
    _1 = 1,
}
impl From<Eidr13> for bool {
    #[inline(always)]
    fn from(variant: Eidr13) -> Self {
        variant as u8 != 0
    }
}
///Field `EIDR13` reader - Port Event Input Data
pub type Eidr13R = crate::BitReader<Eidr13>;
impl Eidr13R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eidr13 {
        match self.bits {
            false => Eidr13::_0,
            true => Eidr13::_1,
        }
    }
    ///Low input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eidr13::_0
    }
    ///High input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eidr13::_1
    }
}
/**Port Event Input Data

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eidr14 {
    ///0: Low input
    _0 = 0,
    ///1: High input
    _1 = 1,
}
impl From<Eidr14> for bool {
    #[inline(always)]
    fn from(variant: Eidr14) -> Self {
        variant as u8 != 0
    }
}
///Field `EIDR14` reader - Port Event Input Data
pub type Eidr14R = crate::BitReader<Eidr14>;
impl Eidr14R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eidr14 {
        match self.bits {
            false => Eidr14::_0,
            true => Eidr14::_1,
        }
    }
    ///Low input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eidr14::_0
    }
    ///High input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eidr14::_1
    }
}
/**Port Event Input Data

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eidr15 {
    ///0: Low input
    _0 = 0,
    ///1: High input
    _1 = 1,
}
impl From<Eidr15> for bool {
    #[inline(always)]
    fn from(variant: Eidr15) -> Self {
        variant as u8 != 0
    }
}
///Field `EIDR15` reader - Port Event Input Data
pub type Eidr15R = crate::BitReader<Eidr15>;
impl Eidr15R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eidr15 {
        match self.bits {
            false => Eidr15::_0,
            true => Eidr15::_1,
        }
    }
    ///Low input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eidr15::_0
    }
    ///High input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eidr15::_1
    }
}
impl R {
    ///Bit 0 - Pmn State
    #[inline(always)]
    pub fn pidr00(&self) -> Pidr00R {
        Pidr00R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Pmn State
    #[inline(always)]
    pub fn pidr01(&self) -> Pidr01R {
        Pidr01R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Pmn State
    #[inline(always)]
    pub fn pidr02(&self) -> Pidr02R {
        Pidr02R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Pmn State
    #[inline(always)]
    pub fn pidr03(&self) -> Pidr03R {
        Pidr03R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Pmn State
    #[inline(always)]
    pub fn pidr04(&self) -> Pidr04R {
        Pidr04R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Pmn State
    #[inline(always)]
    pub fn pidr05(&self) -> Pidr05R {
        Pidr05R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Pmn State
    #[inline(always)]
    pub fn pidr06(&self) -> Pidr06R {
        Pidr06R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Pmn State
    #[inline(always)]
    pub fn pidr07(&self) -> Pidr07R {
        Pidr07R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Pmn State
    #[inline(always)]
    pub fn pidr08(&self) -> Pidr08R {
        Pidr08R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Pmn State
    #[inline(always)]
    pub fn pidr09(&self) -> Pidr09R {
        Pidr09R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Pmn State
    #[inline(always)]
    pub fn pidr10(&self) -> Pidr10R {
        Pidr10R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Pmn State
    #[inline(always)]
    pub fn pidr11(&self) -> Pidr11R {
        Pidr11R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Pmn State
    #[inline(always)]
    pub fn pidr12(&self) -> Pidr12R {
        Pidr12R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Pmn State
    #[inline(always)]
    pub fn pidr13(&self) -> Pidr13R {
        Pidr13R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Pmn State
    #[inline(always)]
    pub fn pidr14(&self) -> Pidr14R {
        Pidr14R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Pmn State
    #[inline(always)]
    pub fn pidr15(&self) -> Pidr15R {
        Pidr15R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Port Event Input Data
    #[inline(always)]
    pub fn eidr00(&self) -> Eidr00R {
        Eidr00R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Port Event Input Data
    #[inline(always)]
    pub fn eidr01(&self) -> Eidr01R {
        Eidr01R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Port Event Input Data
    #[inline(always)]
    pub fn eidr02(&self) -> Eidr02R {
        Eidr02R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Port Event Input Data
    #[inline(always)]
    pub fn eidr03(&self) -> Eidr03R {
        Eidr03R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Port Event Input Data
    #[inline(always)]
    pub fn eidr04(&self) -> Eidr04R {
        Eidr04R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Port Event Input Data
    #[inline(always)]
    pub fn eidr05(&self) -> Eidr05R {
        Eidr05R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Port Event Input Data
    #[inline(always)]
    pub fn eidr06(&self) -> Eidr06R {
        Eidr06R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Port Event Input Data
    #[inline(always)]
    pub fn eidr07(&self) -> Eidr07R {
        Eidr07R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Port Event Input Data
    #[inline(always)]
    pub fn eidr08(&self) -> Eidr08R {
        Eidr08R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Port Event Input Data
    #[inline(always)]
    pub fn eidr09(&self) -> Eidr09R {
        Eidr09R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Port Event Input Data
    #[inline(always)]
    pub fn eidr10(&self) -> Eidr10R {
        Eidr10R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Port Event Input Data
    #[inline(always)]
    pub fn eidr11(&self) -> Eidr11R {
        Eidr11R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Port Event Input Data
    #[inline(always)]
    pub fn eidr12(&self) -> Eidr12R {
        Eidr12R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Port Event Input Data
    #[inline(always)]
    pub fn eidr13(&self) -> Eidr13R {
        Eidr13R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Port Event Input Data
    #[inline(always)]
    pub fn eidr14(&self) -> Eidr14R {
        Eidr14R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Port Event Input Data
    #[inline(always)]
    pub fn eidr15(&self) -> Eidr15R {
        Eidr15R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCNTR2")
            .field("pidr00", &self.pidr00())
            .field("pidr01", &self.pidr01())
            .field("pidr02", &self.pidr02())
            .field("pidr03", &self.pidr03())
            .field("pidr04", &self.pidr04())
            .field("pidr05", &self.pidr05())
            .field("pidr06", &self.pidr06())
            .field("pidr07", &self.pidr07())
            .field("pidr08", &self.pidr08())
            .field("pidr09", &self.pidr09())
            .field("pidr10", &self.pidr10())
            .field("pidr11", &self.pidr11())
            .field("pidr12", &self.pidr12())
            .field("pidr13", &self.pidr13())
            .field("pidr14", &self.pidr14())
            .field("pidr15", &self.pidr15())
            .field("eidr00", &self.eidr00())
            .field("eidr01", &self.eidr01())
            .field("eidr02", &self.eidr02())
            .field("eidr03", &self.eidr03())
            .field("eidr04", &self.eidr04())
            .field("eidr05", &self.eidr05())
            .field("eidr06", &self.eidr06())
            .field("eidr07", &self.eidr07())
            .field("eidr08", &self.eidr08())
            .field("eidr09", &self.eidr09())
            .field("eidr10", &self.eidr10())
            .field("eidr11", &self.eidr11())
            .field("eidr12", &self.eidr12())
            .field("eidr13", &self.eidr13())
            .field("eidr14", &self.eidr14())
            .field("eidr15", &self.eidr15())
            .finish()
    }
}
/**Port Control Register 2

You can [`read`](crate::Reg::read) this register and get [`pcntr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Pcntr2Spec;
impl crate::RegisterSpec for Pcntr2Spec {
    type Ux = u32;
}
///`read()` method returns [`pcntr2::R`](R) reader structure
impl crate::Readable for Pcntr2Spec {}
///`reset()` method sets PCNTR2 to value 0
impl crate::Resettable for Pcntr2Spec {}
