///Register `ADCMPLR0` reader
pub type R = crate::R<Adcmplr0Spec>;
///Register `ADCMPLR0` writer
pub type W = crate::W<Adcmplr0Spec>;
/**Compare Window A Comparison Condition Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmplcha00 {
    ///0: When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value
    _0 = 0,
    ///1: When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value
    _1 = 1,
}
impl From<Cmplcha00> for bool {
    #[inline(always)]
    fn from(variant: Cmplcha00) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPLCHA00` reader - Compare Window A Comparison Condition Select
pub type Cmplcha00R = crate::BitReader<Cmplcha00>;
impl Cmplcha00R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmplcha00 {
        match self.bits {
            false => Cmplcha00::_0,
            true => Cmplcha00::_1,
        }
    }
    ///When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmplcha00::_0
    }
    ///When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmplcha00::_1
    }
}
///Field `CMPLCHA00` writer - Compare Window A Comparison Condition Select
pub type Cmplcha00W<'a, REG> = crate::BitWriter<'a, REG, Cmplcha00>;
impl<'a, REG> Cmplcha00W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha00::_0)
    }
    ///When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha00::_1)
    }
}
/**Compare Window A Comparison Condition Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmplcha01 {
    ///0: When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value
    _0 = 0,
    ///1: When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value
    _1 = 1,
}
impl From<Cmplcha01> for bool {
    #[inline(always)]
    fn from(variant: Cmplcha01) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPLCHA01` reader - Compare Window A Comparison Condition Select
pub type Cmplcha01R = crate::BitReader<Cmplcha01>;
impl Cmplcha01R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmplcha01 {
        match self.bits {
            false => Cmplcha01::_0,
            true => Cmplcha01::_1,
        }
    }
    ///When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmplcha01::_0
    }
    ///When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmplcha01::_1
    }
}
///Field `CMPLCHA01` writer - Compare Window A Comparison Condition Select
pub type Cmplcha01W<'a, REG> = crate::BitWriter<'a, REG, Cmplcha01>;
impl<'a, REG> Cmplcha01W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha01::_0)
    }
    ///When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha01::_1)
    }
}
/**Compare Window A Comparison Condition Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmplcha02 {
    ///0: When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value
    _0 = 0,
    ///1: When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value
    _1 = 1,
}
impl From<Cmplcha02> for bool {
    #[inline(always)]
    fn from(variant: Cmplcha02) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPLCHA02` reader - Compare Window A Comparison Condition Select
pub type Cmplcha02R = crate::BitReader<Cmplcha02>;
impl Cmplcha02R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmplcha02 {
        match self.bits {
            false => Cmplcha02::_0,
            true => Cmplcha02::_1,
        }
    }
    ///When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmplcha02::_0
    }
    ///When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmplcha02::_1
    }
}
///Field `CMPLCHA02` writer - Compare Window A Comparison Condition Select
pub type Cmplcha02W<'a, REG> = crate::BitWriter<'a, REG, Cmplcha02>;
impl<'a, REG> Cmplcha02W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha02::_0)
    }
    ///When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha02::_1)
    }
}
/**Compare Window A Comparison Condition Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmplcha03 {
    ///0: When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value
    _0 = 0,
    ///1: When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value
    _1 = 1,
}
impl From<Cmplcha03> for bool {
    #[inline(always)]
    fn from(variant: Cmplcha03) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPLCHA03` reader - Compare Window A Comparison Condition Select
pub type Cmplcha03R = crate::BitReader<Cmplcha03>;
impl Cmplcha03R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmplcha03 {
        match self.bits {
            false => Cmplcha03::_0,
            true => Cmplcha03::_1,
        }
    }
    ///When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmplcha03::_0
    }
    ///When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmplcha03::_1
    }
}
///Field `CMPLCHA03` writer - Compare Window A Comparison Condition Select
pub type Cmplcha03W<'a, REG> = crate::BitWriter<'a, REG, Cmplcha03>;
impl<'a, REG> Cmplcha03W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha03::_0)
    }
    ///When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha03::_1)
    }
}
/**Compare Window A Comparison Condition Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmplcha04 {
    ///0: When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value
    _0 = 0,
    ///1: When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value
    _1 = 1,
}
impl From<Cmplcha04> for bool {
    #[inline(always)]
    fn from(variant: Cmplcha04) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPLCHA04` reader - Compare Window A Comparison Condition Select
pub type Cmplcha04R = crate::BitReader<Cmplcha04>;
impl Cmplcha04R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmplcha04 {
        match self.bits {
            false => Cmplcha04::_0,
            true => Cmplcha04::_1,
        }
    }
    ///When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmplcha04::_0
    }
    ///When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmplcha04::_1
    }
}
///Field `CMPLCHA04` writer - Compare Window A Comparison Condition Select
pub type Cmplcha04W<'a, REG> = crate::BitWriter<'a, REG, Cmplcha04>;
impl<'a, REG> Cmplcha04W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha04::_0)
    }
    ///When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha04::_1)
    }
}
/**Compare Window A Comparison Condition Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmplcha05 {
    ///0: When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value
    _0 = 0,
    ///1: When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value
    _1 = 1,
}
impl From<Cmplcha05> for bool {
    #[inline(always)]
    fn from(variant: Cmplcha05) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPLCHA05` reader - Compare Window A Comparison Condition Select
pub type Cmplcha05R = crate::BitReader<Cmplcha05>;
impl Cmplcha05R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmplcha05 {
        match self.bits {
            false => Cmplcha05::_0,
            true => Cmplcha05::_1,
        }
    }
    ///When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmplcha05::_0
    }
    ///When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmplcha05::_1
    }
}
///Field `CMPLCHA05` writer - Compare Window A Comparison Condition Select
pub type Cmplcha05W<'a, REG> = crate::BitWriter<'a, REG, Cmplcha05>;
impl<'a, REG> Cmplcha05W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha05::_0)
    }
    ///When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha05::_1)
    }
}
/**Compare Window A Comparison Condition Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmplcha06 {
    ///0: When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value
    _0 = 0,
    ///1: When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value
    _1 = 1,
}
impl From<Cmplcha06> for bool {
    #[inline(always)]
    fn from(variant: Cmplcha06) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPLCHA06` reader - Compare Window A Comparison Condition Select
pub type Cmplcha06R = crate::BitReader<Cmplcha06>;
impl Cmplcha06R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmplcha06 {
        match self.bits {
            false => Cmplcha06::_0,
            true => Cmplcha06::_1,
        }
    }
    ///When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmplcha06::_0
    }
    ///When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmplcha06::_1
    }
}
///Field `CMPLCHA06` writer - Compare Window A Comparison Condition Select
pub type Cmplcha06W<'a, REG> = crate::BitWriter<'a, REG, Cmplcha06>;
impl<'a, REG> Cmplcha06W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha06::_0)
    }
    ///When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha06::_1)
    }
}
/**Compare Window A Comparison Condition Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmplcha07 {
    ///0: When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value
    _0 = 0,
    ///1: When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value
    _1 = 1,
}
impl From<Cmplcha07> for bool {
    #[inline(always)]
    fn from(variant: Cmplcha07) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPLCHA07` reader - Compare Window A Comparison Condition Select
pub type Cmplcha07R = crate::BitReader<Cmplcha07>;
impl Cmplcha07R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmplcha07 {
        match self.bits {
            false => Cmplcha07::_0,
            true => Cmplcha07::_1,
        }
    }
    ///When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmplcha07::_0
    }
    ///When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmplcha07::_1
    }
}
///Field `CMPLCHA07` writer - Compare Window A Comparison Condition Select
pub type Cmplcha07W<'a, REG> = crate::BitWriter<'a, REG, Cmplcha07>;
impl<'a, REG> Cmplcha07W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha07::_0)
    }
    ///When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha07::_1)
    }
}
/**Compare Window A Comparison Condition Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmplcha08 {
    ///0: When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value
    _0 = 0,
    ///1: When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value
    _1 = 1,
}
impl From<Cmplcha08> for bool {
    #[inline(always)]
    fn from(variant: Cmplcha08) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPLCHA08` reader - Compare Window A Comparison Condition Select
pub type Cmplcha08R = crate::BitReader<Cmplcha08>;
impl Cmplcha08R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmplcha08 {
        match self.bits {
            false => Cmplcha08::_0,
            true => Cmplcha08::_1,
        }
    }
    ///When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmplcha08::_0
    }
    ///When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmplcha08::_1
    }
}
///Field `CMPLCHA08` writer - Compare Window A Comparison Condition Select
pub type Cmplcha08W<'a, REG> = crate::BitWriter<'a, REG, Cmplcha08>;
impl<'a, REG> Cmplcha08W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha08::_0)
    }
    ///When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha08::_1)
    }
}
/**Compare Window A Comparison Condition Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmplcha09 {
    ///0: When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value
    _0 = 0,
    ///1: When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value
    _1 = 1,
}
impl From<Cmplcha09> for bool {
    #[inline(always)]
    fn from(variant: Cmplcha09) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPLCHA09` reader - Compare Window A Comparison Condition Select
pub type Cmplcha09R = crate::BitReader<Cmplcha09>;
impl Cmplcha09R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmplcha09 {
        match self.bits {
            false => Cmplcha09::_0,
            true => Cmplcha09::_1,
        }
    }
    ///When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmplcha09::_0
    }
    ///When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmplcha09::_1
    }
}
///Field `CMPLCHA09` writer - Compare Window A Comparison Condition Select
pub type Cmplcha09W<'a, REG> = crate::BitWriter<'a, REG, Cmplcha09>;
impl<'a, REG> Cmplcha09W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha09::_0)
    }
    ///When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha09::_1)
    }
}
/**Compare Window A Comparison Condition Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmplcha10 {
    ///0: When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value
    _0 = 0,
    ///1: When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value
    _1 = 1,
}
impl From<Cmplcha10> for bool {
    #[inline(always)]
    fn from(variant: Cmplcha10) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPLCHA10` reader - Compare Window A Comparison Condition Select
pub type Cmplcha10R = crate::BitReader<Cmplcha10>;
impl Cmplcha10R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmplcha10 {
        match self.bits {
            false => Cmplcha10::_0,
            true => Cmplcha10::_1,
        }
    }
    ///When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmplcha10::_0
    }
    ///When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmplcha10::_1
    }
}
///Field `CMPLCHA10` writer - Compare Window A Comparison Condition Select
pub type Cmplcha10W<'a, REG> = crate::BitWriter<'a, REG, Cmplcha10>;
impl<'a, REG> Cmplcha10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha10::_0)
    }
    ///When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha10::_1)
    }
}
/**Compare Window A Comparison Condition Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmplcha12 {
    ///0: When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value
    _0 = 0,
    ///1: When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value
    _1 = 1,
}
impl From<Cmplcha12> for bool {
    #[inline(always)]
    fn from(variant: Cmplcha12) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPLCHA12` reader - Compare Window A Comparison Condition Select
pub type Cmplcha12R = crate::BitReader<Cmplcha12>;
impl Cmplcha12R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmplcha12 {
        match self.bits {
            false => Cmplcha12::_0,
            true => Cmplcha12::_1,
        }
    }
    ///When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmplcha12::_0
    }
    ///When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmplcha12::_1
    }
}
///Field `CMPLCHA12` writer - Compare Window A Comparison Condition Select
pub type Cmplcha12W<'a, REG> = crate::BitWriter<'a, REG, Cmplcha12>;
impl<'a, REG> Cmplcha12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha12::_0)
    }
    ///When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha12::_1)
    }
}
/**Compare Window A Comparison Condition Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmplcha13 {
    ///0: When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value
    _0 = 0,
    ///1: When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value
    _1 = 1,
}
impl From<Cmplcha13> for bool {
    #[inline(always)]
    fn from(variant: Cmplcha13) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPLCHA13` reader - Compare Window A Comparison Condition Select
pub type Cmplcha13R = crate::BitReader<Cmplcha13>;
impl Cmplcha13R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmplcha13 {
        match self.bits {
            false => Cmplcha13::_0,
            true => Cmplcha13::_1,
        }
    }
    ///When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmplcha13::_0
    }
    ///When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmplcha13::_1
    }
}
///Field `CMPLCHA13` writer - Compare Window A Comparison Condition Select
pub type Cmplcha13W<'a, REG> = crate::BitWriter<'a, REG, Cmplcha13>;
impl<'a, REG> Cmplcha13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha13::_0)
    }
    ///When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplcha13::_1)
    }
}
impl R {
    ///Bit 0 - Compare Window A Comparison Condition Select
    #[inline(always)]
    pub fn cmplcha00(&self) -> Cmplcha00R {
        Cmplcha00R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Compare Window A Comparison Condition Select
    #[inline(always)]
    pub fn cmplcha01(&self) -> Cmplcha01R {
        Cmplcha01R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Compare Window A Comparison Condition Select
    #[inline(always)]
    pub fn cmplcha02(&self) -> Cmplcha02R {
        Cmplcha02R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Compare Window A Comparison Condition Select
    #[inline(always)]
    pub fn cmplcha03(&self) -> Cmplcha03R {
        Cmplcha03R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Compare Window A Comparison Condition Select
    #[inline(always)]
    pub fn cmplcha04(&self) -> Cmplcha04R {
        Cmplcha04R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Compare Window A Comparison Condition Select
    #[inline(always)]
    pub fn cmplcha05(&self) -> Cmplcha05R {
        Cmplcha05R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Compare Window A Comparison Condition Select
    #[inline(always)]
    pub fn cmplcha06(&self) -> Cmplcha06R {
        Cmplcha06R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Compare Window A Comparison Condition Select
    #[inline(always)]
    pub fn cmplcha07(&self) -> Cmplcha07R {
        Cmplcha07R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Compare Window A Comparison Condition Select
    #[inline(always)]
    pub fn cmplcha08(&self) -> Cmplcha08R {
        Cmplcha08R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Compare Window A Comparison Condition Select
    #[inline(always)]
    pub fn cmplcha09(&self) -> Cmplcha09R {
        Cmplcha09R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Compare Window A Comparison Condition Select
    #[inline(always)]
    pub fn cmplcha10(&self) -> Cmplcha10R {
        Cmplcha10R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 12 - Compare Window A Comparison Condition Select
    #[inline(always)]
    pub fn cmplcha12(&self) -> Cmplcha12R {
        Cmplcha12R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Compare Window A Comparison Condition Select
    #[inline(always)]
    pub fn cmplcha13(&self) -> Cmplcha13R {
        Cmplcha13R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCMPLR0")
            .field("cmplcha00", &self.cmplcha00())
            .field("cmplcha01", &self.cmplcha01())
            .field("cmplcha02", &self.cmplcha02())
            .field("cmplcha03", &self.cmplcha03())
            .field("cmplcha04", &self.cmplcha04())
            .field("cmplcha05", &self.cmplcha05())
            .field("cmplcha06", &self.cmplcha06())
            .field("cmplcha07", &self.cmplcha07())
            .field("cmplcha08", &self.cmplcha08())
            .field("cmplcha09", &self.cmplcha09())
            .field("cmplcha10", &self.cmplcha10())
            .field("cmplcha12", &self.cmplcha12())
            .field("cmplcha13", &self.cmplcha13())
            .finish()
    }
}
impl W {
    ///Bit 0 - Compare Window A Comparison Condition Select
    #[inline(always)]
    pub fn cmplcha00(&mut self) -> Cmplcha00W<Adcmplr0Spec> {
        Cmplcha00W::new(self, 0)
    }
    ///Bit 1 - Compare Window A Comparison Condition Select
    #[inline(always)]
    pub fn cmplcha01(&mut self) -> Cmplcha01W<Adcmplr0Spec> {
        Cmplcha01W::new(self, 1)
    }
    ///Bit 2 - Compare Window A Comparison Condition Select
    #[inline(always)]
    pub fn cmplcha02(&mut self) -> Cmplcha02W<Adcmplr0Spec> {
        Cmplcha02W::new(self, 2)
    }
    ///Bit 3 - Compare Window A Comparison Condition Select
    #[inline(always)]
    pub fn cmplcha03(&mut self) -> Cmplcha03W<Adcmplr0Spec> {
        Cmplcha03W::new(self, 3)
    }
    ///Bit 4 - Compare Window A Comparison Condition Select
    #[inline(always)]
    pub fn cmplcha04(&mut self) -> Cmplcha04W<Adcmplr0Spec> {
        Cmplcha04W::new(self, 4)
    }
    ///Bit 5 - Compare Window A Comparison Condition Select
    #[inline(always)]
    pub fn cmplcha05(&mut self) -> Cmplcha05W<Adcmplr0Spec> {
        Cmplcha05W::new(self, 5)
    }
    ///Bit 6 - Compare Window A Comparison Condition Select
    #[inline(always)]
    pub fn cmplcha06(&mut self) -> Cmplcha06W<Adcmplr0Spec> {
        Cmplcha06W::new(self, 6)
    }
    ///Bit 7 - Compare Window A Comparison Condition Select
    #[inline(always)]
    pub fn cmplcha07(&mut self) -> Cmplcha07W<Adcmplr0Spec> {
        Cmplcha07W::new(self, 7)
    }
    ///Bit 8 - Compare Window A Comparison Condition Select
    #[inline(always)]
    pub fn cmplcha08(&mut self) -> Cmplcha08W<Adcmplr0Spec> {
        Cmplcha08W::new(self, 8)
    }
    ///Bit 9 - Compare Window A Comparison Condition Select
    #[inline(always)]
    pub fn cmplcha09(&mut self) -> Cmplcha09W<Adcmplr0Spec> {
        Cmplcha09W::new(self, 9)
    }
    ///Bit 10 - Compare Window A Comparison Condition Select
    #[inline(always)]
    pub fn cmplcha10(&mut self) -> Cmplcha10W<Adcmplr0Spec> {
        Cmplcha10W::new(self, 10)
    }
    ///Bit 12 - Compare Window A Comparison Condition Select
    #[inline(always)]
    pub fn cmplcha12(&mut self) -> Cmplcha12W<Adcmplr0Spec> {
        Cmplcha12W::new(self, 12)
    }
    ///Bit 13 - Compare Window A Comparison Condition Select
    #[inline(always)]
    pub fn cmplcha13(&mut self) -> Cmplcha13W<Adcmplr0Spec> {
        Cmplcha13W::new(self, 13)
    }
}
/**A/D Compare Function Window A Comparison Condition Setting Register 0

You can [`read`](crate::Reg::read) this register and get [`adcmplr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmplr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Adcmplr0Spec;
impl crate::RegisterSpec for Adcmplr0Spec {
    type Ux = u16;
}
///`read()` method returns [`adcmplr0::R`](R) reader structure
impl crate::Readable for Adcmplr0Spec {}
///`write(|w| ..)` method takes [`adcmplr0::W`](W) writer structure
impl crate::Writable for Adcmplr0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADCMPLR0 to value 0
impl crate::Resettable for Adcmplr0Spec {}
