///Register `PCNTR4` reader
pub type R = crate::R<Pcntr4Spec>;
///Register `PCNTR4` writer
pub type W = crate::W<Pcntr4Spec>;
/**Pmn Event Output Set

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eosr00 {
    ///0: No effect on output
    _0 = 0,
    ///1: High output
    _1 = 1,
}
impl From<Eosr00> for bool {
    #[inline(always)]
    fn from(variant: Eosr00) -> Self {
        variant as u8 != 0
    }
}
///Field `EOSR00` reader - Pmn Event Output Set
pub type Eosr00R = crate::BitReader<Eosr00>;
impl Eosr00R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eosr00 {
        match self.bits {
            false => Eosr00::_0,
            true => Eosr00::_1,
        }
    }
    ///No effect on output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eosr00::_0
    }
    ///High output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eosr00::_1
    }
}
///Field `EOSR00` writer - Pmn Event Output Set
pub type Eosr00W<'a, REG> = crate::BitWriter<'a, REG, Eosr00>;
impl<'a, REG> Eosr00W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eosr00::_0)
    }
    ///High output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eosr00::_1)
    }
}
/**Pmn Event Output Set

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eosr01 {
    ///0: No effect on output
    _0 = 0,
    ///1: High output
    _1 = 1,
}
impl From<Eosr01> for bool {
    #[inline(always)]
    fn from(variant: Eosr01) -> Self {
        variant as u8 != 0
    }
}
///Field `EOSR01` reader - Pmn Event Output Set
pub type Eosr01R = crate::BitReader<Eosr01>;
impl Eosr01R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eosr01 {
        match self.bits {
            false => Eosr01::_0,
            true => Eosr01::_1,
        }
    }
    ///No effect on output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eosr01::_0
    }
    ///High output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eosr01::_1
    }
}
///Field `EOSR01` writer - Pmn Event Output Set
pub type Eosr01W<'a, REG> = crate::BitWriter<'a, REG, Eosr01>;
impl<'a, REG> Eosr01W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eosr01::_0)
    }
    ///High output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eosr01::_1)
    }
}
/**Pmn Event Output Set

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eosr02 {
    ///0: No effect on output
    _0 = 0,
    ///1: High output
    _1 = 1,
}
impl From<Eosr02> for bool {
    #[inline(always)]
    fn from(variant: Eosr02) -> Self {
        variant as u8 != 0
    }
}
///Field `EOSR02` reader - Pmn Event Output Set
pub type Eosr02R = crate::BitReader<Eosr02>;
impl Eosr02R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eosr02 {
        match self.bits {
            false => Eosr02::_0,
            true => Eosr02::_1,
        }
    }
    ///No effect on output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eosr02::_0
    }
    ///High output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eosr02::_1
    }
}
///Field `EOSR02` writer - Pmn Event Output Set
pub type Eosr02W<'a, REG> = crate::BitWriter<'a, REG, Eosr02>;
impl<'a, REG> Eosr02W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eosr02::_0)
    }
    ///High output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eosr02::_1)
    }
}
/**Pmn Event Output Set

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eosr03 {
    ///0: No effect on output
    _0 = 0,
    ///1: High output
    _1 = 1,
}
impl From<Eosr03> for bool {
    #[inline(always)]
    fn from(variant: Eosr03) -> Self {
        variant as u8 != 0
    }
}
///Field `EOSR03` reader - Pmn Event Output Set
pub type Eosr03R = crate::BitReader<Eosr03>;
impl Eosr03R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eosr03 {
        match self.bits {
            false => Eosr03::_0,
            true => Eosr03::_1,
        }
    }
    ///No effect on output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eosr03::_0
    }
    ///High output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eosr03::_1
    }
}
///Field `EOSR03` writer - Pmn Event Output Set
pub type Eosr03W<'a, REG> = crate::BitWriter<'a, REG, Eosr03>;
impl<'a, REG> Eosr03W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eosr03::_0)
    }
    ///High output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eosr03::_1)
    }
}
/**Pmn Event Output Set

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eosr04 {
    ///0: No effect on output
    _0 = 0,
    ///1: High output
    _1 = 1,
}
impl From<Eosr04> for bool {
    #[inline(always)]
    fn from(variant: Eosr04) -> Self {
        variant as u8 != 0
    }
}
///Field `EOSR04` reader - Pmn Event Output Set
pub type Eosr04R = crate::BitReader<Eosr04>;
impl Eosr04R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eosr04 {
        match self.bits {
            false => Eosr04::_0,
            true => Eosr04::_1,
        }
    }
    ///No effect on output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eosr04::_0
    }
    ///High output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eosr04::_1
    }
}
///Field `EOSR04` writer - Pmn Event Output Set
pub type Eosr04W<'a, REG> = crate::BitWriter<'a, REG, Eosr04>;
impl<'a, REG> Eosr04W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eosr04::_0)
    }
    ///High output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eosr04::_1)
    }
}
/**Pmn Event Output Set

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eosr05 {
    ///0: No effect on output
    _0 = 0,
    ///1: High output
    _1 = 1,
}
impl From<Eosr05> for bool {
    #[inline(always)]
    fn from(variant: Eosr05) -> Self {
        variant as u8 != 0
    }
}
///Field `EOSR05` reader - Pmn Event Output Set
pub type Eosr05R = crate::BitReader<Eosr05>;
impl Eosr05R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eosr05 {
        match self.bits {
            false => Eosr05::_0,
            true => Eosr05::_1,
        }
    }
    ///No effect on output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eosr05::_0
    }
    ///High output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eosr05::_1
    }
}
///Field `EOSR05` writer - Pmn Event Output Set
pub type Eosr05W<'a, REG> = crate::BitWriter<'a, REG, Eosr05>;
impl<'a, REG> Eosr05W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eosr05::_0)
    }
    ///High output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eosr05::_1)
    }
}
/**Pmn Event Output Set

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eosr06 {
    ///0: No effect on output
    _0 = 0,
    ///1: High output
    _1 = 1,
}
impl From<Eosr06> for bool {
    #[inline(always)]
    fn from(variant: Eosr06) -> Self {
        variant as u8 != 0
    }
}
///Field `EOSR06` reader - Pmn Event Output Set
pub type Eosr06R = crate::BitReader<Eosr06>;
impl Eosr06R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eosr06 {
        match self.bits {
            false => Eosr06::_0,
            true => Eosr06::_1,
        }
    }
    ///No effect on output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eosr06::_0
    }
    ///High output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eosr06::_1
    }
}
///Field `EOSR06` writer - Pmn Event Output Set
pub type Eosr06W<'a, REG> = crate::BitWriter<'a, REG, Eosr06>;
impl<'a, REG> Eosr06W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eosr06::_0)
    }
    ///High output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eosr06::_1)
    }
}
/**Pmn Event Output Set

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eosr07 {
    ///0: No effect on output
    _0 = 0,
    ///1: High output
    _1 = 1,
}
impl From<Eosr07> for bool {
    #[inline(always)]
    fn from(variant: Eosr07) -> Self {
        variant as u8 != 0
    }
}
///Field `EOSR07` reader - Pmn Event Output Set
pub type Eosr07R = crate::BitReader<Eosr07>;
impl Eosr07R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eosr07 {
        match self.bits {
            false => Eosr07::_0,
            true => Eosr07::_1,
        }
    }
    ///No effect on output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eosr07::_0
    }
    ///High output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eosr07::_1
    }
}
///Field `EOSR07` writer - Pmn Event Output Set
pub type Eosr07W<'a, REG> = crate::BitWriter<'a, REG, Eosr07>;
impl<'a, REG> Eosr07W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eosr07::_0)
    }
    ///High output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eosr07::_1)
    }
}
/**Pmn Event Output Set

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eosr08 {
    ///0: No effect on output
    _0 = 0,
    ///1: High output
    _1 = 1,
}
impl From<Eosr08> for bool {
    #[inline(always)]
    fn from(variant: Eosr08) -> Self {
        variant as u8 != 0
    }
}
///Field `EOSR08` reader - Pmn Event Output Set
pub type Eosr08R = crate::BitReader<Eosr08>;
impl Eosr08R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eosr08 {
        match self.bits {
            false => Eosr08::_0,
            true => Eosr08::_1,
        }
    }
    ///No effect on output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eosr08::_0
    }
    ///High output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eosr08::_1
    }
}
///Field `EOSR08` writer - Pmn Event Output Set
pub type Eosr08W<'a, REG> = crate::BitWriter<'a, REG, Eosr08>;
impl<'a, REG> Eosr08W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eosr08::_0)
    }
    ///High output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eosr08::_1)
    }
}
/**Pmn Event Output Set

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eosr09 {
    ///0: No effect on output
    _0 = 0,
    ///1: High output
    _1 = 1,
}
impl From<Eosr09> for bool {
    #[inline(always)]
    fn from(variant: Eosr09) -> Self {
        variant as u8 != 0
    }
}
///Field `EOSR09` reader - Pmn Event Output Set
pub type Eosr09R = crate::BitReader<Eosr09>;
impl Eosr09R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eosr09 {
        match self.bits {
            false => Eosr09::_0,
            true => Eosr09::_1,
        }
    }
    ///No effect on output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eosr09::_0
    }
    ///High output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eosr09::_1
    }
}
///Field `EOSR09` writer - Pmn Event Output Set
pub type Eosr09W<'a, REG> = crate::BitWriter<'a, REG, Eosr09>;
impl<'a, REG> Eosr09W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eosr09::_0)
    }
    ///High output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eosr09::_1)
    }
}
/**Pmn Event Output Set

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eosr10 {
    ///0: No effect on output
    _0 = 0,
    ///1: High output
    _1 = 1,
}
impl From<Eosr10> for bool {
    #[inline(always)]
    fn from(variant: Eosr10) -> Self {
        variant as u8 != 0
    }
}
///Field `EOSR10` reader - Pmn Event Output Set
pub type Eosr10R = crate::BitReader<Eosr10>;
impl Eosr10R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eosr10 {
        match self.bits {
            false => Eosr10::_0,
            true => Eosr10::_1,
        }
    }
    ///No effect on output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eosr10::_0
    }
    ///High output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eosr10::_1
    }
}
///Field `EOSR10` writer - Pmn Event Output Set
pub type Eosr10W<'a, REG> = crate::BitWriter<'a, REG, Eosr10>;
impl<'a, REG> Eosr10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eosr10::_0)
    }
    ///High output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eosr10::_1)
    }
}
/**Pmn Event Output Set

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eosr11 {
    ///0: No effect on output
    _0 = 0,
    ///1: High output
    _1 = 1,
}
impl From<Eosr11> for bool {
    #[inline(always)]
    fn from(variant: Eosr11) -> Self {
        variant as u8 != 0
    }
}
///Field `EOSR11` reader - Pmn Event Output Set
pub type Eosr11R = crate::BitReader<Eosr11>;
impl Eosr11R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eosr11 {
        match self.bits {
            false => Eosr11::_0,
            true => Eosr11::_1,
        }
    }
    ///No effect on output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eosr11::_0
    }
    ///High output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eosr11::_1
    }
}
///Field `EOSR11` writer - Pmn Event Output Set
pub type Eosr11W<'a, REG> = crate::BitWriter<'a, REG, Eosr11>;
impl<'a, REG> Eosr11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eosr11::_0)
    }
    ///High output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eosr11::_1)
    }
}
/**Pmn Event Output Set

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eosr12 {
    ///0: No effect on output
    _0 = 0,
    ///1: High output
    _1 = 1,
}
impl From<Eosr12> for bool {
    #[inline(always)]
    fn from(variant: Eosr12) -> Self {
        variant as u8 != 0
    }
}
///Field `EOSR12` reader - Pmn Event Output Set
pub type Eosr12R = crate::BitReader<Eosr12>;
impl Eosr12R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eosr12 {
        match self.bits {
            false => Eosr12::_0,
            true => Eosr12::_1,
        }
    }
    ///No effect on output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eosr12::_0
    }
    ///High output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eosr12::_1
    }
}
///Field `EOSR12` writer - Pmn Event Output Set
pub type Eosr12W<'a, REG> = crate::BitWriter<'a, REG, Eosr12>;
impl<'a, REG> Eosr12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eosr12::_0)
    }
    ///High output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eosr12::_1)
    }
}
/**Pmn Event Output Set

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eosr13 {
    ///0: No effect on output
    _0 = 0,
    ///1: High output
    _1 = 1,
}
impl From<Eosr13> for bool {
    #[inline(always)]
    fn from(variant: Eosr13) -> Self {
        variant as u8 != 0
    }
}
///Field `EOSR13` reader - Pmn Event Output Set
pub type Eosr13R = crate::BitReader<Eosr13>;
impl Eosr13R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eosr13 {
        match self.bits {
            false => Eosr13::_0,
            true => Eosr13::_1,
        }
    }
    ///No effect on output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eosr13::_0
    }
    ///High output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eosr13::_1
    }
}
///Field `EOSR13` writer - Pmn Event Output Set
pub type Eosr13W<'a, REG> = crate::BitWriter<'a, REG, Eosr13>;
impl<'a, REG> Eosr13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eosr13::_0)
    }
    ///High output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eosr13::_1)
    }
}
/**Pmn Event Output Set

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eosr14 {
    ///0: No effect on output
    _0 = 0,
    ///1: High output
    _1 = 1,
}
impl From<Eosr14> for bool {
    #[inline(always)]
    fn from(variant: Eosr14) -> Self {
        variant as u8 != 0
    }
}
///Field `EOSR14` reader - Pmn Event Output Set
pub type Eosr14R = crate::BitReader<Eosr14>;
impl Eosr14R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eosr14 {
        match self.bits {
            false => Eosr14::_0,
            true => Eosr14::_1,
        }
    }
    ///No effect on output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eosr14::_0
    }
    ///High output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eosr14::_1
    }
}
///Field `EOSR14` writer - Pmn Event Output Set
pub type Eosr14W<'a, REG> = crate::BitWriter<'a, REG, Eosr14>;
impl<'a, REG> Eosr14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eosr14::_0)
    }
    ///High output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eosr14::_1)
    }
}
/**Pmn Event Output Set

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eosr15 {
    ///0: No effect on output
    _0 = 0,
    ///1: High output
    _1 = 1,
}
impl From<Eosr15> for bool {
    #[inline(always)]
    fn from(variant: Eosr15) -> Self {
        variant as u8 != 0
    }
}
///Field `EOSR15` reader - Pmn Event Output Set
pub type Eosr15R = crate::BitReader<Eosr15>;
impl Eosr15R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eosr15 {
        match self.bits {
            false => Eosr15::_0,
            true => Eosr15::_1,
        }
    }
    ///No effect on output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eosr15::_0
    }
    ///High output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eosr15::_1
    }
}
///Field `EOSR15` writer - Pmn Event Output Set
pub type Eosr15W<'a, REG> = crate::BitWriter<'a, REG, Eosr15>;
impl<'a, REG> Eosr15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eosr15::_0)
    }
    ///High output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eosr15::_1)
    }
}
/**Pmn Event Output Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eorr00 {
    ///0: No effect on output
    _0 = 0,
    ///1: Low output
    _1 = 1,
}
impl From<Eorr00> for bool {
    #[inline(always)]
    fn from(variant: Eorr00) -> Self {
        variant as u8 != 0
    }
}
///Field `EORR00` reader - Pmn Event Output Reset
pub type Eorr00R = crate::BitReader<Eorr00>;
impl Eorr00R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eorr00 {
        match self.bits {
            false => Eorr00::_0,
            true => Eorr00::_1,
        }
    }
    ///No effect on output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eorr00::_0
    }
    ///Low output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eorr00::_1
    }
}
///Field `EORR00` writer - Pmn Event Output Reset
pub type Eorr00W<'a, REG> = crate::BitWriter<'a, REG, Eorr00>;
impl<'a, REG> Eorr00W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr00::_0)
    }
    ///Low output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr00::_1)
    }
}
/**Pmn Event Output Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eorr01 {
    ///0: No effect on output
    _0 = 0,
    ///1: Low output
    _1 = 1,
}
impl From<Eorr01> for bool {
    #[inline(always)]
    fn from(variant: Eorr01) -> Self {
        variant as u8 != 0
    }
}
///Field `EORR01` reader - Pmn Event Output Reset
pub type Eorr01R = crate::BitReader<Eorr01>;
impl Eorr01R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eorr01 {
        match self.bits {
            false => Eorr01::_0,
            true => Eorr01::_1,
        }
    }
    ///No effect on output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eorr01::_0
    }
    ///Low output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eorr01::_1
    }
}
///Field `EORR01` writer - Pmn Event Output Reset
pub type Eorr01W<'a, REG> = crate::BitWriter<'a, REG, Eorr01>;
impl<'a, REG> Eorr01W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr01::_0)
    }
    ///Low output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr01::_1)
    }
}
/**Pmn Event Output Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eorr02 {
    ///0: No effect on output
    _0 = 0,
    ///1: Low output
    _1 = 1,
}
impl From<Eorr02> for bool {
    #[inline(always)]
    fn from(variant: Eorr02) -> Self {
        variant as u8 != 0
    }
}
///Field `EORR02` reader - Pmn Event Output Reset
pub type Eorr02R = crate::BitReader<Eorr02>;
impl Eorr02R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eorr02 {
        match self.bits {
            false => Eorr02::_0,
            true => Eorr02::_1,
        }
    }
    ///No effect on output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eorr02::_0
    }
    ///Low output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eorr02::_1
    }
}
///Field `EORR02` writer - Pmn Event Output Reset
pub type Eorr02W<'a, REG> = crate::BitWriter<'a, REG, Eorr02>;
impl<'a, REG> Eorr02W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr02::_0)
    }
    ///Low output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr02::_1)
    }
}
/**Pmn Event Output Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eorr03 {
    ///0: No effect on output
    _0 = 0,
    ///1: Low output
    _1 = 1,
}
impl From<Eorr03> for bool {
    #[inline(always)]
    fn from(variant: Eorr03) -> Self {
        variant as u8 != 0
    }
}
///Field `EORR03` reader - Pmn Event Output Reset
pub type Eorr03R = crate::BitReader<Eorr03>;
impl Eorr03R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eorr03 {
        match self.bits {
            false => Eorr03::_0,
            true => Eorr03::_1,
        }
    }
    ///No effect on output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eorr03::_0
    }
    ///Low output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eorr03::_1
    }
}
///Field `EORR03` writer - Pmn Event Output Reset
pub type Eorr03W<'a, REG> = crate::BitWriter<'a, REG, Eorr03>;
impl<'a, REG> Eorr03W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr03::_0)
    }
    ///Low output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr03::_1)
    }
}
/**Pmn Event Output Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eorr04 {
    ///0: No effect on output
    _0 = 0,
    ///1: Low output
    _1 = 1,
}
impl From<Eorr04> for bool {
    #[inline(always)]
    fn from(variant: Eorr04) -> Self {
        variant as u8 != 0
    }
}
///Field `EORR04` reader - Pmn Event Output Reset
pub type Eorr04R = crate::BitReader<Eorr04>;
impl Eorr04R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eorr04 {
        match self.bits {
            false => Eorr04::_0,
            true => Eorr04::_1,
        }
    }
    ///No effect on output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eorr04::_0
    }
    ///Low output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eorr04::_1
    }
}
///Field `EORR04` writer - Pmn Event Output Reset
pub type Eorr04W<'a, REG> = crate::BitWriter<'a, REG, Eorr04>;
impl<'a, REG> Eorr04W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr04::_0)
    }
    ///Low output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr04::_1)
    }
}
/**Pmn Event Output Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eorr05 {
    ///0: No effect on output
    _0 = 0,
    ///1: Low output
    _1 = 1,
}
impl From<Eorr05> for bool {
    #[inline(always)]
    fn from(variant: Eorr05) -> Self {
        variant as u8 != 0
    }
}
///Field `EORR05` reader - Pmn Event Output Reset
pub type Eorr05R = crate::BitReader<Eorr05>;
impl Eorr05R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eorr05 {
        match self.bits {
            false => Eorr05::_0,
            true => Eorr05::_1,
        }
    }
    ///No effect on output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eorr05::_0
    }
    ///Low output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eorr05::_1
    }
}
///Field `EORR05` writer - Pmn Event Output Reset
pub type Eorr05W<'a, REG> = crate::BitWriter<'a, REG, Eorr05>;
impl<'a, REG> Eorr05W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr05::_0)
    }
    ///Low output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr05::_1)
    }
}
/**Pmn Event Output Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eorr06 {
    ///0: No effect on output
    _0 = 0,
    ///1: Low output
    _1 = 1,
}
impl From<Eorr06> for bool {
    #[inline(always)]
    fn from(variant: Eorr06) -> Self {
        variant as u8 != 0
    }
}
///Field `EORR06` reader - Pmn Event Output Reset
pub type Eorr06R = crate::BitReader<Eorr06>;
impl Eorr06R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eorr06 {
        match self.bits {
            false => Eorr06::_0,
            true => Eorr06::_1,
        }
    }
    ///No effect on output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eorr06::_0
    }
    ///Low output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eorr06::_1
    }
}
///Field `EORR06` writer - Pmn Event Output Reset
pub type Eorr06W<'a, REG> = crate::BitWriter<'a, REG, Eorr06>;
impl<'a, REG> Eorr06W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr06::_0)
    }
    ///Low output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr06::_1)
    }
}
/**Pmn Event Output Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eorr07 {
    ///0: No effect on output
    _0 = 0,
    ///1: Low output
    _1 = 1,
}
impl From<Eorr07> for bool {
    #[inline(always)]
    fn from(variant: Eorr07) -> Self {
        variant as u8 != 0
    }
}
///Field `EORR07` reader - Pmn Event Output Reset
pub type Eorr07R = crate::BitReader<Eorr07>;
impl Eorr07R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eorr07 {
        match self.bits {
            false => Eorr07::_0,
            true => Eorr07::_1,
        }
    }
    ///No effect on output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eorr07::_0
    }
    ///Low output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eorr07::_1
    }
}
///Field `EORR07` writer - Pmn Event Output Reset
pub type Eorr07W<'a, REG> = crate::BitWriter<'a, REG, Eorr07>;
impl<'a, REG> Eorr07W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr07::_0)
    }
    ///Low output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr07::_1)
    }
}
/**Pmn Event Output Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eorr08 {
    ///0: No effect on output
    _0 = 0,
    ///1: Low output
    _1 = 1,
}
impl From<Eorr08> for bool {
    #[inline(always)]
    fn from(variant: Eorr08) -> Self {
        variant as u8 != 0
    }
}
///Field `EORR08` reader - Pmn Event Output Reset
pub type Eorr08R = crate::BitReader<Eorr08>;
impl Eorr08R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eorr08 {
        match self.bits {
            false => Eorr08::_0,
            true => Eorr08::_1,
        }
    }
    ///No effect on output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eorr08::_0
    }
    ///Low output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eorr08::_1
    }
}
///Field `EORR08` writer - Pmn Event Output Reset
pub type Eorr08W<'a, REG> = crate::BitWriter<'a, REG, Eorr08>;
impl<'a, REG> Eorr08W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr08::_0)
    }
    ///Low output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr08::_1)
    }
}
/**Pmn Event Output Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eorr09 {
    ///0: No effect on output
    _0 = 0,
    ///1: Low output
    _1 = 1,
}
impl From<Eorr09> for bool {
    #[inline(always)]
    fn from(variant: Eorr09) -> Self {
        variant as u8 != 0
    }
}
///Field `EORR09` reader - Pmn Event Output Reset
pub type Eorr09R = crate::BitReader<Eorr09>;
impl Eorr09R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eorr09 {
        match self.bits {
            false => Eorr09::_0,
            true => Eorr09::_1,
        }
    }
    ///No effect on output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eorr09::_0
    }
    ///Low output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eorr09::_1
    }
}
///Field `EORR09` writer - Pmn Event Output Reset
pub type Eorr09W<'a, REG> = crate::BitWriter<'a, REG, Eorr09>;
impl<'a, REG> Eorr09W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr09::_0)
    }
    ///Low output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr09::_1)
    }
}
/**Pmn Event Output Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eorr10 {
    ///0: No effect on output
    _0 = 0,
    ///1: Low output
    _1 = 1,
}
impl From<Eorr10> for bool {
    #[inline(always)]
    fn from(variant: Eorr10) -> Self {
        variant as u8 != 0
    }
}
///Field `EORR10` reader - Pmn Event Output Reset
pub type Eorr10R = crate::BitReader<Eorr10>;
impl Eorr10R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eorr10 {
        match self.bits {
            false => Eorr10::_0,
            true => Eorr10::_1,
        }
    }
    ///No effect on output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eorr10::_0
    }
    ///Low output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eorr10::_1
    }
}
///Field `EORR10` writer - Pmn Event Output Reset
pub type Eorr10W<'a, REG> = crate::BitWriter<'a, REG, Eorr10>;
impl<'a, REG> Eorr10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr10::_0)
    }
    ///Low output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr10::_1)
    }
}
/**Pmn Event Output Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eorr11 {
    ///0: No effect on output
    _0 = 0,
    ///1: Low output
    _1 = 1,
}
impl From<Eorr11> for bool {
    #[inline(always)]
    fn from(variant: Eorr11) -> Self {
        variant as u8 != 0
    }
}
///Field `EORR11` reader - Pmn Event Output Reset
pub type Eorr11R = crate::BitReader<Eorr11>;
impl Eorr11R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eorr11 {
        match self.bits {
            false => Eorr11::_0,
            true => Eorr11::_1,
        }
    }
    ///No effect on output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eorr11::_0
    }
    ///Low output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eorr11::_1
    }
}
///Field `EORR11` writer - Pmn Event Output Reset
pub type Eorr11W<'a, REG> = crate::BitWriter<'a, REG, Eorr11>;
impl<'a, REG> Eorr11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr11::_0)
    }
    ///Low output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr11::_1)
    }
}
/**Pmn Event Output Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eorr12 {
    ///0: No effect on output
    _0 = 0,
    ///1: Low output
    _1 = 1,
}
impl From<Eorr12> for bool {
    #[inline(always)]
    fn from(variant: Eorr12) -> Self {
        variant as u8 != 0
    }
}
///Field `EORR12` reader - Pmn Event Output Reset
pub type Eorr12R = crate::BitReader<Eorr12>;
impl Eorr12R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eorr12 {
        match self.bits {
            false => Eorr12::_0,
            true => Eorr12::_1,
        }
    }
    ///No effect on output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eorr12::_0
    }
    ///Low output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eorr12::_1
    }
}
///Field `EORR12` writer - Pmn Event Output Reset
pub type Eorr12W<'a, REG> = crate::BitWriter<'a, REG, Eorr12>;
impl<'a, REG> Eorr12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr12::_0)
    }
    ///Low output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr12::_1)
    }
}
/**Pmn Event Output Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eorr13 {
    ///0: No effect on output
    _0 = 0,
    ///1: Low output
    _1 = 1,
}
impl From<Eorr13> for bool {
    #[inline(always)]
    fn from(variant: Eorr13) -> Self {
        variant as u8 != 0
    }
}
///Field `EORR13` reader - Pmn Event Output Reset
pub type Eorr13R = crate::BitReader<Eorr13>;
impl Eorr13R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eorr13 {
        match self.bits {
            false => Eorr13::_0,
            true => Eorr13::_1,
        }
    }
    ///No effect on output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eorr13::_0
    }
    ///Low output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eorr13::_1
    }
}
///Field `EORR13` writer - Pmn Event Output Reset
pub type Eorr13W<'a, REG> = crate::BitWriter<'a, REG, Eorr13>;
impl<'a, REG> Eorr13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr13::_0)
    }
    ///Low output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr13::_1)
    }
}
/**Pmn Event Output Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eorr14 {
    ///0: No effect on output
    _0 = 0,
    ///1: Low output
    _1 = 1,
}
impl From<Eorr14> for bool {
    #[inline(always)]
    fn from(variant: Eorr14) -> Self {
        variant as u8 != 0
    }
}
///Field `EORR14` reader - Pmn Event Output Reset
pub type Eorr14R = crate::BitReader<Eorr14>;
impl Eorr14R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eorr14 {
        match self.bits {
            false => Eorr14::_0,
            true => Eorr14::_1,
        }
    }
    ///No effect on output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eorr14::_0
    }
    ///Low output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eorr14::_1
    }
}
///Field `EORR14` writer - Pmn Event Output Reset
pub type Eorr14W<'a, REG> = crate::BitWriter<'a, REG, Eorr14>;
impl<'a, REG> Eorr14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr14::_0)
    }
    ///Low output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr14::_1)
    }
}
/**Pmn Event Output Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eorr15 {
    ///0: No effect on output
    _0 = 0,
    ///1: Low output
    _1 = 1,
}
impl From<Eorr15> for bool {
    #[inline(always)]
    fn from(variant: Eorr15) -> Self {
        variant as u8 != 0
    }
}
///Field `EORR15` reader - Pmn Event Output Reset
pub type Eorr15R = crate::BitReader<Eorr15>;
impl Eorr15R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eorr15 {
        match self.bits {
            false => Eorr15::_0,
            true => Eorr15::_1,
        }
    }
    ///No effect on output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eorr15::_0
    }
    ///Low output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eorr15::_1
    }
}
///Field `EORR15` writer - Pmn Event Output Reset
pub type Eorr15W<'a, REG> = crate::BitWriter<'a, REG, Eorr15>;
impl<'a, REG> Eorr15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr15::_0)
    }
    ///Low output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr15::_1)
    }
}
impl R {
    ///Bit 0 - Pmn Event Output Set
    #[inline(always)]
    pub fn eosr00(&self) -> Eosr00R {
        Eosr00R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Pmn Event Output Set
    #[inline(always)]
    pub fn eosr01(&self) -> Eosr01R {
        Eosr01R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Pmn Event Output Set
    #[inline(always)]
    pub fn eosr02(&self) -> Eosr02R {
        Eosr02R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Pmn Event Output Set
    #[inline(always)]
    pub fn eosr03(&self) -> Eosr03R {
        Eosr03R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Pmn Event Output Set
    #[inline(always)]
    pub fn eosr04(&self) -> Eosr04R {
        Eosr04R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Pmn Event Output Set
    #[inline(always)]
    pub fn eosr05(&self) -> Eosr05R {
        Eosr05R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Pmn Event Output Set
    #[inline(always)]
    pub fn eosr06(&self) -> Eosr06R {
        Eosr06R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Pmn Event Output Set
    #[inline(always)]
    pub fn eosr07(&self) -> Eosr07R {
        Eosr07R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Pmn Event Output Set
    #[inline(always)]
    pub fn eosr08(&self) -> Eosr08R {
        Eosr08R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Pmn Event Output Set
    #[inline(always)]
    pub fn eosr09(&self) -> Eosr09R {
        Eosr09R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Pmn Event Output Set
    #[inline(always)]
    pub fn eosr10(&self) -> Eosr10R {
        Eosr10R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Pmn Event Output Set
    #[inline(always)]
    pub fn eosr11(&self) -> Eosr11R {
        Eosr11R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Pmn Event Output Set
    #[inline(always)]
    pub fn eosr12(&self) -> Eosr12R {
        Eosr12R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Pmn Event Output Set
    #[inline(always)]
    pub fn eosr13(&self) -> Eosr13R {
        Eosr13R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Pmn Event Output Set
    #[inline(always)]
    pub fn eosr14(&self) -> Eosr14R {
        Eosr14R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Pmn Event Output Set
    #[inline(always)]
    pub fn eosr15(&self) -> Eosr15R {
        Eosr15R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Pmn Event Output Reset
    #[inline(always)]
    pub fn eorr00(&self) -> Eorr00R {
        Eorr00R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Pmn Event Output Reset
    #[inline(always)]
    pub fn eorr01(&self) -> Eorr01R {
        Eorr01R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Pmn Event Output Reset
    #[inline(always)]
    pub fn eorr02(&self) -> Eorr02R {
        Eorr02R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Pmn Event Output Reset
    #[inline(always)]
    pub fn eorr03(&self) -> Eorr03R {
        Eorr03R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Pmn Event Output Reset
    #[inline(always)]
    pub fn eorr04(&self) -> Eorr04R {
        Eorr04R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Pmn Event Output Reset
    #[inline(always)]
    pub fn eorr05(&self) -> Eorr05R {
        Eorr05R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Pmn Event Output Reset
    #[inline(always)]
    pub fn eorr06(&self) -> Eorr06R {
        Eorr06R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Pmn Event Output Reset
    #[inline(always)]
    pub fn eorr07(&self) -> Eorr07R {
        Eorr07R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Pmn Event Output Reset
    #[inline(always)]
    pub fn eorr08(&self) -> Eorr08R {
        Eorr08R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Pmn Event Output Reset
    #[inline(always)]
    pub fn eorr09(&self) -> Eorr09R {
        Eorr09R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Pmn Event Output Reset
    #[inline(always)]
    pub fn eorr10(&self) -> Eorr10R {
        Eorr10R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Pmn Event Output Reset
    #[inline(always)]
    pub fn eorr11(&self) -> Eorr11R {
        Eorr11R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Pmn Event Output Reset
    #[inline(always)]
    pub fn eorr12(&self) -> Eorr12R {
        Eorr12R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Pmn Event Output Reset
    #[inline(always)]
    pub fn eorr13(&self) -> Eorr13R {
        Eorr13R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Pmn Event Output Reset
    #[inline(always)]
    pub fn eorr14(&self) -> Eorr14R {
        Eorr14R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Pmn Event Output Reset
    #[inline(always)]
    pub fn eorr15(&self) -> Eorr15R {
        Eorr15R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCNTR4")
            .field("eosr00", &self.eosr00())
            .field("eosr01", &self.eosr01())
            .field("eosr02", &self.eosr02())
            .field("eosr03", &self.eosr03())
            .field("eosr04", &self.eosr04())
            .field("eosr05", &self.eosr05())
            .field("eosr06", &self.eosr06())
            .field("eosr07", &self.eosr07())
            .field("eosr08", &self.eosr08())
            .field("eosr09", &self.eosr09())
            .field("eosr10", &self.eosr10())
            .field("eosr11", &self.eosr11())
            .field("eosr12", &self.eosr12())
            .field("eosr13", &self.eosr13())
            .field("eosr14", &self.eosr14())
            .field("eosr15", &self.eosr15())
            .field("eorr00", &self.eorr00())
            .field("eorr01", &self.eorr01())
            .field("eorr02", &self.eorr02())
            .field("eorr03", &self.eorr03())
            .field("eorr04", &self.eorr04())
            .field("eorr05", &self.eorr05())
            .field("eorr06", &self.eorr06())
            .field("eorr07", &self.eorr07())
            .field("eorr08", &self.eorr08())
            .field("eorr09", &self.eorr09())
            .field("eorr10", &self.eorr10())
            .field("eorr11", &self.eorr11())
            .field("eorr12", &self.eorr12())
            .field("eorr13", &self.eorr13())
            .field("eorr14", &self.eorr14())
            .field("eorr15", &self.eorr15())
            .finish()
    }
}
impl W {
    ///Bit 0 - Pmn Event Output Set
    #[inline(always)]
    pub fn eosr00(&mut self) -> Eosr00W<Pcntr4Spec> {
        Eosr00W::new(self, 0)
    }
    ///Bit 1 - Pmn Event Output Set
    #[inline(always)]
    pub fn eosr01(&mut self) -> Eosr01W<Pcntr4Spec> {
        Eosr01W::new(self, 1)
    }
    ///Bit 2 - Pmn Event Output Set
    #[inline(always)]
    pub fn eosr02(&mut self) -> Eosr02W<Pcntr4Spec> {
        Eosr02W::new(self, 2)
    }
    ///Bit 3 - Pmn Event Output Set
    #[inline(always)]
    pub fn eosr03(&mut self) -> Eosr03W<Pcntr4Spec> {
        Eosr03W::new(self, 3)
    }
    ///Bit 4 - Pmn Event Output Set
    #[inline(always)]
    pub fn eosr04(&mut self) -> Eosr04W<Pcntr4Spec> {
        Eosr04W::new(self, 4)
    }
    ///Bit 5 - Pmn Event Output Set
    #[inline(always)]
    pub fn eosr05(&mut self) -> Eosr05W<Pcntr4Spec> {
        Eosr05W::new(self, 5)
    }
    ///Bit 6 - Pmn Event Output Set
    #[inline(always)]
    pub fn eosr06(&mut self) -> Eosr06W<Pcntr4Spec> {
        Eosr06W::new(self, 6)
    }
    ///Bit 7 - Pmn Event Output Set
    #[inline(always)]
    pub fn eosr07(&mut self) -> Eosr07W<Pcntr4Spec> {
        Eosr07W::new(self, 7)
    }
    ///Bit 8 - Pmn Event Output Set
    #[inline(always)]
    pub fn eosr08(&mut self) -> Eosr08W<Pcntr4Spec> {
        Eosr08W::new(self, 8)
    }
    ///Bit 9 - Pmn Event Output Set
    #[inline(always)]
    pub fn eosr09(&mut self) -> Eosr09W<Pcntr4Spec> {
        Eosr09W::new(self, 9)
    }
    ///Bit 10 - Pmn Event Output Set
    #[inline(always)]
    pub fn eosr10(&mut self) -> Eosr10W<Pcntr4Spec> {
        Eosr10W::new(self, 10)
    }
    ///Bit 11 - Pmn Event Output Set
    #[inline(always)]
    pub fn eosr11(&mut self) -> Eosr11W<Pcntr4Spec> {
        Eosr11W::new(self, 11)
    }
    ///Bit 12 - Pmn Event Output Set
    #[inline(always)]
    pub fn eosr12(&mut self) -> Eosr12W<Pcntr4Spec> {
        Eosr12W::new(self, 12)
    }
    ///Bit 13 - Pmn Event Output Set
    #[inline(always)]
    pub fn eosr13(&mut self) -> Eosr13W<Pcntr4Spec> {
        Eosr13W::new(self, 13)
    }
    ///Bit 14 - Pmn Event Output Set
    #[inline(always)]
    pub fn eosr14(&mut self) -> Eosr14W<Pcntr4Spec> {
        Eosr14W::new(self, 14)
    }
    ///Bit 15 - Pmn Event Output Set
    #[inline(always)]
    pub fn eosr15(&mut self) -> Eosr15W<Pcntr4Spec> {
        Eosr15W::new(self, 15)
    }
    ///Bit 16 - Pmn Event Output Reset
    #[inline(always)]
    pub fn eorr00(&mut self) -> Eorr00W<Pcntr4Spec> {
        Eorr00W::new(self, 16)
    }
    ///Bit 17 - Pmn Event Output Reset
    #[inline(always)]
    pub fn eorr01(&mut self) -> Eorr01W<Pcntr4Spec> {
        Eorr01W::new(self, 17)
    }
    ///Bit 18 - Pmn Event Output Reset
    #[inline(always)]
    pub fn eorr02(&mut self) -> Eorr02W<Pcntr4Spec> {
        Eorr02W::new(self, 18)
    }
    ///Bit 19 - Pmn Event Output Reset
    #[inline(always)]
    pub fn eorr03(&mut self) -> Eorr03W<Pcntr4Spec> {
        Eorr03W::new(self, 19)
    }
    ///Bit 20 - Pmn Event Output Reset
    #[inline(always)]
    pub fn eorr04(&mut self) -> Eorr04W<Pcntr4Spec> {
        Eorr04W::new(self, 20)
    }
    ///Bit 21 - Pmn Event Output Reset
    #[inline(always)]
    pub fn eorr05(&mut self) -> Eorr05W<Pcntr4Spec> {
        Eorr05W::new(self, 21)
    }
    ///Bit 22 - Pmn Event Output Reset
    #[inline(always)]
    pub fn eorr06(&mut self) -> Eorr06W<Pcntr4Spec> {
        Eorr06W::new(self, 22)
    }
    ///Bit 23 - Pmn Event Output Reset
    #[inline(always)]
    pub fn eorr07(&mut self) -> Eorr07W<Pcntr4Spec> {
        Eorr07W::new(self, 23)
    }
    ///Bit 24 - Pmn Event Output Reset
    #[inline(always)]
    pub fn eorr08(&mut self) -> Eorr08W<Pcntr4Spec> {
        Eorr08W::new(self, 24)
    }
    ///Bit 25 - Pmn Event Output Reset
    #[inline(always)]
    pub fn eorr09(&mut self) -> Eorr09W<Pcntr4Spec> {
        Eorr09W::new(self, 25)
    }
    ///Bit 26 - Pmn Event Output Reset
    #[inline(always)]
    pub fn eorr10(&mut self) -> Eorr10W<Pcntr4Spec> {
        Eorr10W::new(self, 26)
    }
    ///Bit 27 - Pmn Event Output Reset
    #[inline(always)]
    pub fn eorr11(&mut self) -> Eorr11W<Pcntr4Spec> {
        Eorr11W::new(self, 27)
    }
    ///Bit 28 - Pmn Event Output Reset
    #[inline(always)]
    pub fn eorr12(&mut self) -> Eorr12W<Pcntr4Spec> {
        Eorr12W::new(self, 28)
    }
    ///Bit 29 - Pmn Event Output Reset
    #[inline(always)]
    pub fn eorr13(&mut self) -> Eorr13W<Pcntr4Spec> {
        Eorr13W::new(self, 29)
    }
    ///Bit 30 - Pmn Event Output Reset
    #[inline(always)]
    pub fn eorr14(&mut self) -> Eorr14W<Pcntr4Spec> {
        Eorr14W::new(self, 30)
    }
    ///Bit 31 - Pmn Event Output Reset
    #[inline(always)]
    pub fn eorr15(&mut self) -> Eorr15W<Pcntr4Spec> {
        Eorr15W::new(self, 31)
    }
}
/**Port Control Register 4

You can [`read`](crate::Reg::read) this register and get [`pcntr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcntr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Pcntr4Spec;
impl crate::RegisterSpec for Pcntr4Spec {
    type Ux = u32;
}
///`read()` method returns [`pcntr4::R`](R) reader structure
impl crate::Readable for Pcntr4Spec {}
///`write(|w| ..)` method takes [`pcntr4::W`](W) writer structure
impl crate::Writable for Pcntr4Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PCNTR4 to value 0
impl crate::Resettable for Pcntr4Spec {}
