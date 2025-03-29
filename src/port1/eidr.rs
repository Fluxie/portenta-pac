///Register `EIDR` reader
pub type R = crate::R<EidrSpec>;
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
    ///Bit 0 - Port Event Input Data
    #[inline(always)]
    pub fn eidr00(&self) -> Eidr00R {
        Eidr00R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Port Event Input Data
    #[inline(always)]
    pub fn eidr01(&self) -> Eidr01R {
        Eidr01R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Port Event Input Data
    #[inline(always)]
    pub fn eidr02(&self) -> Eidr02R {
        Eidr02R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Port Event Input Data
    #[inline(always)]
    pub fn eidr03(&self) -> Eidr03R {
        Eidr03R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Port Event Input Data
    #[inline(always)]
    pub fn eidr04(&self) -> Eidr04R {
        Eidr04R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Port Event Input Data
    #[inline(always)]
    pub fn eidr05(&self) -> Eidr05R {
        Eidr05R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Port Event Input Data
    #[inline(always)]
    pub fn eidr06(&self) -> Eidr06R {
        Eidr06R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Port Event Input Data
    #[inline(always)]
    pub fn eidr07(&self) -> Eidr07R {
        Eidr07R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Port Event Input Data
    #[inline(always)]
    pub fn eidr08(&self) -> Eidr08R {
        Eidr08R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Port Event Input Data
    #[inline(always)]
    pub fn eidr09(&self) -> Eidr09R {
        Eidr09R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Port Event Input Data
    #[inline(always)]
    pub fn eidr10(&self) -> Eidr10R {
        Eidr10R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Port Event Input Data
    #[inline(always)]
    pub fn eidr11(&self) -> Eidr11R {
        Eidr11R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Port Event Input Data
    #[inline(always)]
    pub fn eidr12(&self) -> Eidr12R {
        Eidr12R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Port Event Input Data
    #[inline(always)]
    pub fn eidr13(&self) -> Eidr13R {
        Eidr13R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Port Event Input Data
    #[inline(always)]
    pub fn eidr14(&self) -> Eidr14R {
        Eidr14R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Port Event Input Data
    #[inline(always)]
    pub fn eidr15(&self) -> Eidr15R {
        Eidr15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EIDR")
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

You can [`read`](crate::Reg::read) this register and get [`eidr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct EidrSpec;
impl crate::RegisterSpec for EidrSpec {
    type Ux = u16;
}
///`read()` method returns [`eidr::R`](R) reader structure
impl crate::Readable for EidrSpec {}
///`reset()` method sets EIDR to value 0
impl crate::Resettable for EidrSpec {}
