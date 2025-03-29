///Register `ICUSARG` reader
pub type R = crate::R<IcusargSpec>;
///Register `ICUSARG` writer
pub type W = crate::W<IcusargSpec>;
/**Security attributes of registers for IELSR31 to IELSR0

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr00 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr00> for bool {
    #[inline(always)]
    fn from(variant: Saielsr00) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR00` reader - Security attributes of registers for IELSR31 to IELSR0
pub type Saielsr00R = crate::BitReader<Saielsr00>;
impl Saielsr00R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr00 {
        match self.bits {
            false => Saielsr00::_0,
            true => Saielsr00::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr00::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr00::_1
    }
}
///Field `SAIELSR00` writer - Security attributes of registers for IELSR31 to IELSR0
pub type Saielsr00W<'a, REG> = crate::BitWriter<'a, REG, Saielsr00>;
impl<'a, REG> Saielsr00W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr00::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr00::_1)
    }
}
/**Security attributes of registers for IELSR31 to IELSR0

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr01 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr01> for bool {
    #[inline(always)]
    fn from(variant: Saielsr01) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR01` reader - Security attributes of registers for IELSR31 to IELSR0
pub type Saielsr01R = crate::BitReader<Saielsr01>;
impl Saielsr01R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr01 {
        match self.bits {
            false => Saielsr01::_0,
            true => Saielsr01::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr01::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr01::_1
    }
}
///Field `SAIELSR01` writer - Security attributes of registers for IELSR31 to IELSR0
pub type Saielsr01W<'a, REG> = crate::BitWriter<'a, REG, Saielsr01>;
impl<'a, REG> Saielsr01W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr01::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr01::_1)
    }
}
/**Security attributes of registers for IELSR31 to IELSR0

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr02 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr02> for bool {
    #[inline(always)]
    fn from(variant: Saielsr02) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR02` reader - Security attributes of registers for IELSR31 to IELSR0
pub type Saielsr02R = crate::BitReader<Saielsr02>;
impl Saielsr02R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr02 {
        match self.bits {
            false => Saielsr02::_0,
            true => Saielsr02::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr02::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr02::_1
    }
}
///Field `SAIELSR02` writer - Security attributes of registers for IELSR31 to IELSR0
pub type Saielsr02W<'a, REG> = crate::BitWriter<'a, REG, Saielsr02>;
impl<'a, REG> Saielsr02W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr02::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr02::_1)
    }
}
/**Security attributes of registers for IELSR31 to IELSR0

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr03 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr03> for bool {
    #[inline(always)]
    fn from(variant: Saielsr03) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR03` reader - Security attributes of registers for IELSR31 to IELSR0
pub type Saielsr03R = crate::BitReader<Saielsr03>;
impl Saielsr03R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr03 {
        match self.bits {
            false => Saielsr03::_0,
            true => Saielsr03::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr03::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr03::_1
    }
}
///Field `SAIELSR03` writer - Security attributes of registers for IELSR31 to IELSR0
pub type Saielsr03W<'a, REG> = crate::BitWriter<'a, REG, Saielsr03>;
impl<'a, REG> Saielsr03W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr03::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr03::_1)
    }
}
/**Security attributes of registers for IELSR31 to IELSR0

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr04 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr04> for bool {
    #[inline(always)]
    fn from(variant: Saielsr04) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR04` reader - Security attributes of registers for IELSR31 to IELSR0
pub type Saielsr04R = crate::BitReader<Saielsr04>;
impl Saielsr04R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr04 {
        match self.bits {
            false => Saielsr04::_0,
            true => Saielsr04::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr04::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr04::_1
    }
}
///Field `SAIELSR04` writer - Security attributes of registers for IELSR31 to IELSR0
pub type Saielsr04W<'a, REG> = crate::BitWriter<'a, REG, Saielsr04>;
impl<'a, REG> Saielsr04W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr04::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr04::_1)
    }
}
/**Security attributes of registers for IELSR31 to IELSR0

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr05 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr05> for bool {
    #[inline(always)]
    fn from(variant: Saielsr05) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR05` reader - Security attributes of registers for IELSR31 to IELSR0
pub type Saielsr05R = crate::BitReader<Saielsr05>;
impl Saielsr05R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr05 {
        match self.bits {
            false => Saielsr05::_0,
            true => Saielsr05::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr05::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr05::_1
    }
}
///Field `SAIELSR05` writer - Security attributes of registers for IELSR31 to IELSR0
pub type Saielsr05W<'a, REG> = crate::BitWriter<'a, REG, Saielsr05>;
impl<'a, REG> Saielsr05W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr05::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr05::_1)
    }
}
/**Security attributes of registers for IELSR31 to IELSR0

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr06 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr06> for bool {
    #[inline(always)]
    fn from(variant: Saielsr06) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR06` reader - Security attributes of registers for IELSR31 to IELSR0
pub type Saielsr06R = crate::BitReader<Saielsr06>;
impl Saielsr06R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr06 {
        match self.bits {
            false => Saielsr06::_0,
            true => Saielsr06::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr06::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr06::_1
    }
}
///Field `SAIELSR06` writer - Security attributes of registers for IELSR31 to IELSR0
pub type Saielsr06W<'a, REG> = crate::BitWriter<'a, REG, Saielsr06>;
impl<'a, REG> Saielsr06W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr06::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr06::_1)
    }
}
/**Security attributes of registers for IELSR31 to IELSR0

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr07 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr07> for bool {
    #[inline(always)]
    fn from(variant: Saielsr07) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR07` reader - Security attributes of registers for IELSR31 to IELSR0
pub type Saielsr07R = crate::BitReader<Saielsr07>;
impl Saielsr07R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr07 {
        match self.bits {
            false => Saielsr07::_0,
            true => Saielsr07::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr07::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr07::_1
    }
}
///Field `SAIELSR07` writer - Security attributes of registers for IELSR31 to IELSR0
pub type Saielsr07W<'a, REG> = crate::BitWriter<'a, REG, Saielsr07>;
impl<'a, REG> Saielsr07W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr07::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr07::_1)
    }
}
/**Security attributes of registers for IELSR31 to IELSR0

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr08 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr08> for bool {
    #[inline(always)]
    fn from(variant: Saielsr08) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR08` reader - Security attributes of registers for IELSR31 to IELSR0
pub type Saielsr08R = crate::BitReader<Saielsr08>;
impl Saielsr08R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr08 {
        match self.bits {
            false => Saielsr08::_0,
            true => Saielsr08::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr08::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr08::_1
    }
}
///Field `SAIELSR08` writer - Security attributes of registers for IELSR31 to IELSR0
pub type Saielsr08W<'a, REG> = crate::BitWriter<'a, REG, Saielsr08>;
impl<'a, REG> Saielsr08W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr08::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr08::_1)
    }
}
/**Security attributes of registers for IELSR31 to IELSR0

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr09 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr09> for bool {
    #[inline(always)]
    fn from(variant: Saielsr09) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR09` reader - Security attributes of registers for IELSR31 to IELSR0
pub type Saielsr09R = crate::BitReader<Saielsr09>;
impl Saielsr09R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr09 {
        match self.bits {
            false => Saielsr09::_0,
            true => Saielsr09::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr09::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr09::_1
    }
}
///Field `SAIELSR09` writer - Security attributes of registers for IELSR31 to IELSR0
pub type Saielsr09W<'a, REG> = crate::BitWriter<'a, REG, Saielsr09>;
impl<'a, REG> Saielsr09W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr09::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr09::_1)
    }
}
/**Security attributes of registers for IELSR31 to IELSR0

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr10 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr10> for bool {
    #[inline(always)]
    fn from(variant: Saielsr10) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR10` reader - Security attributes of registers for IELSR31 to IELSR0
pub type Saielsr10R = crate::BitReader<Saielsr10>;
impl Saielsr10R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr10 {
        match self.bits {
            false => Saielsr10::_0,
            true => Saielsr10::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr10::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr10::_1
    }
}
///Field `SAIELSR10` writer - Security attributes of registers for IELSR31 to IELSR0
pub type Saielsr10W<'a, REG> = crate::BitWriter<'a, REG, Saielsr10>;
impl<'a, REG> Saielsr10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr10::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr10::_1)
    }
}
/**Security attributes of registers for IELSR31 to IELSR0

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr11 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr11> for bool {
    #[inline(always)]
    fn from(variant: Saielsr11) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR11` reader - Security attributes of registers for IELSR31 to IELSR0
pub type Saielsr11R = crate::BitReader<Saielsr11>;
impl Saielsr11R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr11 {
        match self.bits {
            false => Saielsr11::_0,
            true => Saielsr11::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr11::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr11::_1
    }
}
///Field `SAIELSR11` writer - Security attributes of registers for IELSR31 to IELSR0
pub type Saielsr11W<'a, REG> = crate::BitWriter<'a, REG, Saielsr11>;
impl<'a, REG> Saielsr11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr11::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr11::_1)
    }
}
/**Security attributes of registers for IELSR31 to IELSR0

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr12 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr12> for bool {
    #[inline(always)]
    fn from(variant: Saielsr12) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR12` reader - Security attributes of registers for IELSR31 to IELSR0
pub type Saielsr12R = crate::BitReader<Saielsr12>;
impl Saielsr12R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr12 {
        match self.bits {
            false => Saielsr12::_0,
            true => Saielsr12::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr12::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr12::_1
    }
}
///Field `SAIELSR12` writer - Security attributes of registers for IELSR31 to IELSR0
pub type Saielsr12W<'a, REG> = crate::BitWriter<'a, REG, Saielsr12>;
impl<'a, REG> Saielsr12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr12::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr12::_1)
    }
}
/**Security attributes of registers for IELSR31 to IELSR0

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr13 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr13> for bool {
    #[inline(always)]
    fn from(variant: Saielsr13) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR13` reader - Security attributes of registers for IELSR31 to IELSR0
pub type Saielsr13R = crate::BitReader<Saielsr13>;
impl Saielsr13R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr13 {
        match self.bits {
            false => Saielsr13::_0,
            true => Saielsr13::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr13::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr13::_1
    }
}
///Field `SAIELSR13` writer - Security attributes of registers for IELSR31 to IELSR0
pub type Saielsr13W<'a, REG> = crate::BitWriter<'a, REG, Saielsr13>;
impl<'a, REG> Saielsr13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr13::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr13::_1)
    }
}
/**Security attributes of registers for IELSR31 to IELSR0

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr14 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr14> for bool {
    #[inline(always)]
    fn from(variant: Saielsr14) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR14` reader - Security attributes of registers for IELSR31 to IELSR0
pub type Saielsr14R = crate::BitReader<Saielsr14>;
impl Saielsr14R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr14 {
        match self.bits {
            false => Saielsr14::_0,
            true => Saielsr14::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr14::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr14::_1
    }
}
///Field `SAIELSR14` writer - Security attributes of registers for IELSR31 to IELSR0
pub type Saielsr14W<'a, REG> = crate::BitWriter<'a, REG, Saielsr14>;
impl<'a, REG> Saielsr14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr14::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr14::_1)
    }
}
/**Security attributes of registers for IELSR31 to IELSR0

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr15 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr15> for bool {
    #[inline(always)]
    fn from(variant: Saielsr15) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR15` reader - Security attributes of registers for IELSR31 to IELSR0
pub type Saielsr15R = crate::BitReader<Saielsr15>;
impl Saielsr15R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr15 {
        match self.bits {
            false => Saielsr15::_0,
            true => Saielsr15::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr15::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr15::_1
    }
}
///Field `SAIELSR15` writer - Security attributes of registers for IELSR31 to IELSR0
pub type Saielsr15W<'a, REG> = crate::BitWriter<'a, REG, Saielsr15>;
impl<'a, REG> Saielsr15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr15::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr15::_1)
    }
}
/**Security attributes of registers for IELSR31 to IELSR0

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr16 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr16> for bool {
    #[inline(always)]
    fn from(variant: Saielsr16) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR16` reader - Security attributes of registers for IELSR31 to IELSR0
pub type Saielsr16R = crate::BitReader<Saielsr16>;
impl Saielsr16R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr16 {
        match self.bits {
            false => Saielsr16::_0,
            true => Saielsr16::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr16::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr16::_1
    }
}
///Field `SAIELSR16` writer - Security attributes of registers for IELSR31 to IELSR0
pub type Saielsr16W<'a, REG> = crate::BitWriter<'a, REG, Saielsr16>;
impl<'a, REG> Saielsr16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr16::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr16::_1)
    }
}
/**Security attributes of registers for IELSR31 to IELSR0

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr17 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr17> for bool {
    #[inline(always)]
    fn from(variant: Saielsr17) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR17` reader - Security attributes of registers for IELSR31 to IELSR0
pub type Saielsr17R = crate::BitReader<Saielsr17>;
impl Saielsr17R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr17 {
        match self.bits {
            false => Saielsr17::_0,
            true => Saielsr17::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr17::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr17::_1
    }
}
///Field `SAIELSR17` writer - Security attributes of registers for IELSR31 to IELSR0
pub type Saielsr17W<'a, REG> = crate::BitWriter<'a, REG, Saielsr17>;
impl<'a, REG> Saielsr17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr17::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr17::_1)
    }
}
/**Security attributes of registers for IELSR31 to IELSR0

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr18 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr18> for bool {
    #[inline(always)]
    fn from(variant: Saielsr18) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR18` reader - Security attributes of registers for IELSR31 to IELSR0
pub type Saielsr18R = crate::BitReader<Saielsr18>;
impl Saielsr18R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr18 {
        match self.bits {
            false => Saielsr18::_0,
            true => Saielsr18::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr18::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr18::_1
    }
}
///Field `SAIELSR18` writer - Security attributes of registers for IELSR31 to IELSR0
pub type Saielsr18W<'a, REG> = crate::BitWriter<'a, REG, Saielsr18>;
impl<'a, REG> Saielsr18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr18::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr18::_1)
    }
}
/**Security attributes of registers for IELSR31 to IELSR0

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr19 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr19> for bool {
    #[inline(always)]
    fn from(variant: Saielsr19) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR19` reader - Security attributes of registers for IELSR31 to IELSR0
pub type Saielsr19R = crate::BitReader<Saielsr19>;
impl Saielsr19R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr19 {
        match self.bits {
            false => Saielsr19::_0,
            true => Saielsr19::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr19::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr19::_1
    }
}
///Field `SAIELSR19` writer - Security attributes of registers for IELSR31 to IELSR0
pub type Saielsr19W<'a, REG> = crate::BitWriter<'a, REG, Saielsr19>;
impl<'a, REG> Saielsr19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr19::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr19::_1)
    }
}
/**Security attributes of registers for IELSR31 to IELSR0

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr20 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr20> for bool {
    #[inline(always)]
    fn from(variant: Saielsr20) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR20` reader - Security attributes of registers for IELSR31 to IELSR0
pub type Saielsr20R = crate::BitReader<Saielsr20>;
impl Saielsr20R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr20 {
        match self.bits {
            false => Saielsr20::_0,
            true => Saielsr20::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr20::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr20::_1
    }
}
///Field `SAIELSR20` writer - Security attributes of registers for IELSR31 to IELSR0
pub type Saielsr20W<'a, REG> = crate::BitWriter<'a, REG, Saielsr20>;
impl<'a, REG> Saielsr20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr20::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr20::_1)
    }
}
/**Security attributes of registers for IELSR31 to IELSR0

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr21 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr21> for bool {
    #[inline(always)]
    fn from(variant: Saielsr21) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR21` reader - Security attributes of registers for IELSR31 to IELSR0
pub type Saielsr21R = crate::BitReader<Saielsr21>;
impl Saielsr21R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr21 {
        match self.bits {
            false => Saielsr21::_0,
            true => Saielsr21::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr21::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr21::_1
    }
}
///Field `SAIELSR21` writer - Security attributes of registers for IELSR31 to IELSR0
pub type Saielsr21W<'a, REG> = crate::BitWriter<'a, REG, Saielsr21>;
impl<'a, REG> Saielsr21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr21::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr21::_1)
    }
}
/**Security attributes of registers for IELSR31 to IELSR0

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr22 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr22> for bool {
    #[inline(always)]
    fn from(variant: Saielsr22) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR22` reader - Security attributes of registers for IELSR31 to IELSR0
pub type Saielsr22R = crate::BitReader<Saielsr22>;
impl Saielsr22R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr22 {
        match self.bits {
            false => Saielsr22::_0,
            true => Saielsr22::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr22::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr22::_1
    }
}
///Field `SAIELSR22` writer - Security attributes of registers for IELSR31 to IELSR0
pub type Saielsr22W<'a, REG> = crate::BitWriter<'a, REG, Saielsr22>;
impl<'a, REG> Saielsr22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr22::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr22::_1)
    }
}
/**Security attributes of registers for IELSR31 to IELSR0

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr23 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr23> for bool {
    #[inline(always)]
    fn from(variant: Saielsr23) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR23` reader - Security attributes of registers for IELSR31 to IELSR0
pub type Saielsr23R = crate::BitReader<Saielsr23>;
impl Saielsr23R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr23 {
        match self.bits {
            false => Saielsr23::_0,
            true => Saielsr23::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr23::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr23::_1
    }
}
///Field `SAIELSR23` writer - Security attributes of registers for IELSR31 to IELSR0
pub type Saielsr23W<'a, REG> = crate::BitWriter<'a, REG, Saielsr23>;
impl<'a, REG> Saielsr23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr23::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr23::_1)
    }
}
/**Security attributes of registers for IELSR31 to IELSR0

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr24 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr24> for bool {
    #[inline(always)]
    fn from(variant: Saielsr24) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR24` reader - Security attributes of registers for IELSR31 to IELSR0
pub type Saielsr24R = crate::BitReader<Saielsr24>;
impl Saielsr24R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr24 {
        match self.bits {
            false => Saielsr24::_0,
            true => Saielsr24::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr24::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr24::_1
    }
}
///Field `SAIELSR24` writer - Security attributes of registers for IELSR31 to IELSR0
pub type Saielsr24W<'a, REG> = crate::BitWriter<'a, REG, Saielsr24>;
impl<'a, REG> Saielsr24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr24::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr24::_1)
    }
}
/**Security attributes of registers for IELSR31 to IELSR0

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr25 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr25> for bool {
    #[inline(always)]
    fn from(variant: Saielsr25) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR25` reader - Security attributes of registers for IELSR31 to IELSR0
pub type Saielsr25R = crate::BitReader<Saielsr25>;
impl Saielsr25R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr25 {
        match self.bits {
            false => Saielsr25::_0,
            true => Saielsr25::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr25::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr25::_1
    }
}
///Field `SAIELSR25` writer - Security attributes of registers for IELSR31 to IELSR0
pub type Saielsr25W<'a, REG> = crate::BitWriter<'a, REG, Saielsr25>;
impl<'a, REG> Saielsr25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr25::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr25::_1)
    }
}
/**Security attributes of registers for IELSR31 to IELSR0

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr26 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr26> for bool {
    #[inline(always)]
    fn from(variant: Saielsr26) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR26` reader - Security attributes of registers for IELSR31 to IELSR0
pub type Saielsr26R = crate::BitReader<Saielsr26>;
impl Saielsr26R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr26 {
        match self.bits {
            false => Saielsr26::_0,
            true => Saielsr26::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr26::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr26::_1
    }
}
///Field `SAIELSR26` writer - Security attributes of registers for IELSR31 to IELSR0
pub type Saielsr26W<'a, REG> = crate::BitWriter<'a, REG, Saielsr26>;
impl<'a, REG> Saielsr26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr26::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr26::_1)
    }
}
/**Security attributes of registers for IELSR31 to IELSR0

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr27 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr27> for bool {
    #[inline(always)]
    fn from(variant: Saielsr27) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR27` reader - Security attributes of registers for IELSR31 to IELSR0
pub type Saielsr27R = crate::BitReader<Saielsr27>;
impl Saielsr27R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr27 {
        match self.bits {
            false => Saielsr27::_0,
            true => Saielsr27::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr27::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr27::_1
    }
}
///Field `SAIELSR27` writer - Security attributes of registers for IELSR31 to IELSR0
pub type Saielsr27W<'a, REG> = crate::BitWriter<'a, REG, Saielsr27>;
impl<'a, REG> Saielsr27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr27::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr27::_1)
    }
}
/**Security attributes of registers for IELSR31 to IELSR0

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr28 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr28> for bool {
    #[inline(always)]
    fn from(variant: Saielsr28) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR28` reader - Security attributes of registers for IELSR31 to IELSR0
pub type Saielsr28R = crate::BitReader<Saielsr28>;
impl Saielsr28R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr28 {
        match self.bits {
            false => Saielsr28::_0,
            true => Saielsr28::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr28::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr28::_1
    }
}
///Field `SAIELSR28` writer - Security attributes of registers for IELSR31 to IELSR0
pub type Saielsr28W<'a, REG> = crate::BitWriter<'a, REG, Saielsr28>;
impl<'a, REG> Saielsr28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr28::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr28::_1)
    }
}
/**Security attributes of registers for IELSR31 to IELSR0

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr29 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr29> for bool {
    #[inline(always)]
    fn from(variant: Saielsr29) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR29` reader - Security attributes of registers for IELSR31 to IELSR0
pub type Saielsr29R = crate::BitReader<Saielsr29>;
impl Saielsr29R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr29 {
        match self.bits {
            false => Saielsr29::_0,
            true => Saielsr29::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr29::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr29::_1
    }
}
///Field `SAIELSR29` writer - Security attributes of registers for IELSR31 to IELSR0
pub type Saielsr29W<'a, REG> = crate::BitWriter<'a, REG, Saielsr29>;
impl<'a, REG> Saielsr29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr29::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr29::_1)
    }
}
/**Security attributes of registers for IELSR31 to IELSR0

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr30 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr30> for bool {
    #[inline(always)]
    fn from(variant: Saielsr30) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR30` reader - Security attributes of registers for IELSR31 to IELSR0
pub type Saielsr30R = crate::BitReader<Saielsr30>;
impl Saielsr30R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr30 {
        match self.bits {
            false => Saielsr30::_0,
            true => Saielsr30::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr30::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr30::_1
    }
}
///Field `SAIELSR30` writer - Security attributes of registers for IELSR31 to IELSR0
pub type Saielsr30W<'a, REG> = crate::BitWriter<'a, REG, Saielsr30>;
impl<'a, REG> Saielsr30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr30::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr30::_1)
    }
}
/**Security attributes of registers for IELSR31 to IELSR0

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr31 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr31> for bool {
    #[inline(always)]
    fn from(variant: Saielsr31) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR31` reader - Security attributes of registers for IELSR31 to IELSR0
pub type Saielsr31R = crate::BitReader<Saielsr31>;
impl Saielsr31R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr31 {
        match self.bits {
            false => Saielsr31::_0,
            true => Saielsr31::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr31::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr31::_1
    }
}
///Field `SAIELSR31` writer - Security attributes of registers for IELSR31 to IELSR0
pub type Saielsr31W<'a, REG> = crate::BitWriter<'a, REG, Saielsr31>;
impl<'a, REG> Saielsr31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr31::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr31::_1)
    }
}
impl R {
    ///Bit 0 - Security attributes of registers for IELSR31 to IELSR0
    #[inline(always)]
    pub fn saielsr00(&self) -> Saielsr00R {
        Saielsr00R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Security attributes of registers for IELSR31 to IELSR0
    #[inline(always)]
    pub fn saielsr01(&self) -> Saielsr01R {
        Saielsr01R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Security attributes of registers for IELSR31 to IELSR0
    #[inline(always)]
    pub fn saielsr02(&self) -> Saielsr02R {
        Saielsr02R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Security attributes of registers for IELSR31 to IELSR0
    #[inline(always)]
    pub fn saielsr03(&self) -> Saielsr03R {
        Saielsr03R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Security attributes of registers for IELSR31 to IELSR0
    #[inline(always)]
    pub fn saielsr04(&self) -> Saielsr04R {
        Saielsr04R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Security attributes of registers for IELSR31 to IELSR0
    #[inline(always)]
    pub fn saielsr05(&self) -> Saielsr05R {
        Saielsr05R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Security attributes of registers for IELSR31 to IELSR0
    #[inline(always)]
    pub fn saielsr06(&self) -> Saielsr06R {
        Saielsr06R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Security attributes of registers for IELSR31 to IELSR0
    #[inline(always)]
    pub fn saielsr07(&self) -> Saielsr07R {
        Saielsr07R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Security attributes of registers for IELSR31 to IELSR0
    #[inline(always)]
    pub fn saielsr08(&self) -> Saielsr08R {
        Saielsr08R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Security attributes of registers for IELSR31 to IELSR0
    #[inline(always)]
    pub fn saielsr09(&self) -> Saielsr09R {
        Saielsr09R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Security attributes of registers for IELSR31 to IELSR0
    #[inline(always)]
    pub fn saielsr10(&self) -> Saielsr10R {
        Saielsr10R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Security attributes of registers for IELSR31 to IELSR0
    #[inline(always)]
    pub fn saielsr11(&self) -> Saielsr11R {
        Saielsr11R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Security attributes of registers for IELSR31 to IELSR0
    #[inline(always)]
    pub fn saielsr12(&self) -> Saielsr12R {
        Saielsr12R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Security attributes of registers for IELSR31 to IELSR0
    #[inline(always)]
    pub fn saielsr13(&self) -> Saielsr13R {
        Saielsr13R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Security attributes of registers for IELSR31 to IELSR0
    #[inline(always)]
    pub fn saielsr14(&self) -> Saielsr14R {
        Saielsr14R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Security attributes of registers for IELSR31 to IELSR0
    #[inline(always)]
    pub fn saielsr15(&self) -> Saielsr15R {
        Saielsr15R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Security attributes of registers for IELSR31 to IELSR0
    #[inline(always)]
    pub fn saielsr16(&self) -> Saielsr16R {
        Saielsr16R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Security attributes of registers for IELSR31 to IELSR0
    #[inline(always)]
    pub fn saielsr17(&self) -> Saielsr17R {
        Saielsr17R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Security attributes of registers for IELSR31 to IELSR0
    #[inline(always)]
    pub fn saielsr18(&self) -> Saielsr18R {
        Saielsr18R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Security attributes of registers for IELSR31 to IELSR0
    #[inline(always)]
    pub fn saielsr19(&self) -> Saielsr19R {
        Saielsr19R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Security attributes of registers for IELSR31 to IELSR0
    #[inline(always)]
    pub fn saielsr20(&self) -> Saielsr20R {
        Saielsr20R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Security attributes of registers for IELSR31 to IELSR0
    #[inline(always)]
    pub fn saielsr21(&self) -> Saielsr21R {
        Saielsr21R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Security attributes of registers for IELSR31 to IELSR0
    #[inline(always)]
    pub fn saielsr22(&self) -> Saielsr22R {
        Saielsr22R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Security attributes of registers for IELSR31 to IELSR0
    #[inline(always)]
    pub fn saielsr23(&self) -> Saielsr23R {
        Saielsr23R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Security attributes of registers for IELSR31 to IELSR0
    #[inline(always)]
    pub fn saielsr24(&self) -> Saielsr24R {
        Saielsr24R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Security attributes of registers for IELSR31 to IELSR0
    #[inline(always)]
    pub fn saielsr25(&self) -> Saielsr25R {
        Saielsr25R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Security attributes of registers for IELSR31 to IELSR0
    #[inline(always)]
    pub fn saielsr26(&self) -> Saielsr26R {
        Saielsr26R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Security attributes of registers for IELSR31 to IELSR0
    #[inline(always)]
    pub fn saielsr27(&self) -> Saielsr27R {
        Saielsr27R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Security attributes of registers for IELSR31 to IELSR0
    #[inline(always)]
    pub fn saielsr28(&self) -> Saielsr28R {
        Saielsr28R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Security attributes of registers for IELSR31 to IELSR0
    #[inline(always)]
    pub fn saielsr29(&self) -> Saielsr29R {
        Saielsr29R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Security attributes of registers for IELSR31 to IELSR0
    #[inline(always)]
    pub fn saielsr30(&self) -> Saielsr30R {
        Saielsr30R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Security attributes of registers for IELSR31 to IELSR0
    #[inline(always)]
    pub fn saielsr31(&self) -> Saielsr31R {
        Saielsr31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICUSARG")
            .field("saielsr00", &self.saielsr00())
            .field("saielsr01", &self.saielsr01())
            .field("saielsr02", &self.saielsr02())
            .field("saielsr03", &self.saielsr03())
            .field("saielsr04", &self.saielsr04())
            .field("saielsr05", &self.saielsr05())
            .field("saielsr06", &self.saielsr06())
            .field("saielsr07", &self.saielsr07())
            .field("saielsr08", &self.saielsr08())
            .field("saielsr09", &self.saielsr09())
            .field("saielsr10", &self.saielsr10())
            .field("saielsr11", &self.saielsr11())
            .field("saielsr12", &self.saielsr12())
            .field("saielsr13", &self.saielsr13())
            .field("saielsr14", &self.saielsr14())
            .field("saielsr15", &self.saielsr15())
            .field("saielsr16", &self.saielsr16())
            .field("saielsr17", &self.saielsr17())
            .field("saielsr18", &self.saielsr18())
            .field("saielsr19", &self.saielsr19())
            .field("saielsr20", &self.saielsr20())
            .field("saielsr21", &self.saielsr21())
            .field("saielsr22", &self.saielsr22())
            .field("saielsr23", &self.saielsr23())
            .field("saielsr24", &self.saielsr24())
            .field("saielsr25", &self.saielsr25())
            .field("saielsr26", &self.saielsr26())
            .field("saielsr27", &self.saielsr27())
            .field("saielsr28", &self.saielsr28())
            .field("saielsr29", &self.saielsr29())
            .field("saielsr30", &self.saielsr30())
            .field("saielsr31", &self.saielsr31())
            .finish()
    }
}
impl W {
    ///Bit 0 - Security attributes of registers for IELSR31 to IELSR0
    #[inline(always)]
    pub fn saielsr00(&mut self) -> Saielsr00W<IcusargSpec> {
        Saielsr00W::new(self, 0)
    }
    ///Bit 1 - Security attributes of registers for IELSR31 to IELSR0
    #[inline(always)]
    pub fn saielsr01(&mut self) -> Saielsr01W<IcusargSpec> {
        Saielsr01W::new(self, 1)
    }
    ///Bit 2 - Security attributes of registers for IELSR31 to IELSR0
    #[inline(always)]
    pub fn saielsr02(&mut self) -> Saielsr02W<IcusargSpec> {
        Saielsr02W::new(self, 2)
    }
    ///Bit 3 - Security attributes of registers for IELSR31 to IELSR0
    #[inline(always)]
    pub fn saielsr03(&mut self) -> Saielsr03W<IcusargSpec> {
        Saielsr03W::new(self, 3)
    }
    ///Bit 4 - Security attributes of registers for IELSR31 to IELSR0
    #[inline(always)]
    pub fn saielsr04(&mut self) -> Saielsr04W<IcusargSpec> {
        Saielsr04W::new(self, 4)
    }
    ///Bit 5 - Security attributes of registers for IELSR31 to IELSR0
    #[inline(always)]
    pub fn saielsr05(&mut self) -> Saielsr05W<IcusargSpec> {
        Saielsr05W::new(self, 5)
    }
    ///Bit 6 - Security attributes of registers for IELSR31 to IELSR0
    #[inline(always)]
    pub fn saielsr06(&mut self) -> Saielsr06W<IcusargSpec> {
        Saielsr06W::new(self, 6)
    }
    ///Bit 7 - Security attributes of registers for IELSR31 to IELSR0
    #[inline(always)]
    pub fn saielsr07(&mut self) -> Saielsr07W<IcusargSpec> {
        Saielsr07W::new(self, 7)
    }
    ///Bit 8 - Security attributes of registers for IELSR31 to IELSR0
    #[inline(always)]
    pub fn saielsr08(&mut self) -> Saielsr08W<IcusargSpec> {
        Saielsr08W::new(self, 8)
    }
    ///Bit 9 - Security attributes of registers for IELSR31 to IELSR0
    #[inline(always)]
    pub fn saielsr09(&mut self) -> Saielsr09W<IcusargSpec> {
        Saielsr09W::new(self, 9)
    }
    ///Bit 10 - Security attributes of registers for IELSR31 to IELSR0
    #[inline(always)]
    pub fn saielsr10(&mut self) -> Saielsr10W<IcusargSpec> {
        Saielsr10W::new(self, 10)
    }
    ///Bit 11 - Security attributes of registers for IELSR31 to IELSR0
    #[inline(always)]
    pub fn saielsr11(&mut self) -> Saielsr11W<IcusargSpec> {
        Saielsr11W::new(self, 11)
    }
    ///Bit 12 - Security attributes of registers for IELSR31 to IELSR0
    #[inline(always)]
    pub fn saielsr12(&mut self) -> Saielsr12W<IcusargSpec> {
        Saielsr12W::new(self, 12)
    }
    ///Bit 13 - Security attributes of registers for IELSR31 to IELSR0
    #[inline(always)]
    pub fn saielsr13(&mut self) -> Saielsr13W<IcusargSpec> {
        Saielsr13W::new(self, 13)
    }
    ///Bit 14 - Security attributes of registers for IELSR31 to IELSR0
    #[inline(always)]
    pub fn saielsr14(&mut self) -> Saielsr14W<IcusargSpec> {
        Saielsr14W::new(self, 14)
    }
    ///Bit 15 - Security attributes of registers for IELSR31 to IELSR0
    #[inline(always)]
    pub fn saielsr15(&mut self) -> Saielsr15W<IcusargSpec> {
        Saielsr15W::new(self, 15)
    }
    ///Bit 16 - Security attributes of registers for IELSR31 to IELSR0
    #[inline(always)]
    pub fn saielsr16(&mut self) -> Saielsr16W<IcusargSpec> {
        Saielsr16W::new(self, 16)
    }
    ///Bit 17 - Security attributes of registers for IELSR31 to IELSR0
    #[inline(always)]
    pub fn saielsr17(&mut self) -> Saielsr17W<IcusargSpec> {
        Saielsr17W::new(self, 17)
    }
    ///Bit 18 - Security attributes of registers for IELSR31 to IELSR0
    #[inline(always)]
    pub fn saielsr18(&mut self) -> Saielsr18W<IcusargSpec> {
        Saielsr18W::new(self, 18)
    }
    ///Bit 19 - Security attributes of registers for IELSR31 to IELSR0
    #[inline(always)]
    pub fn saielsr19(&mut self) -> Saielsr19W<IcusargSpec> {
        Saielsr19W::new(self, 19)
    }
    ///Bit 20 - Security attributes of registers for IELSR31 to IELSR0
    #[inline(always)]
    pub fn saielsr20(&mut self) -> Saielsr20W<IcusargSpec> {
        Saielsr20W::new(self, 20)
    }
    ///Bit 21 - Security attributes of registers for IELSR31 to IELSR0
    #[inline(always)]
    pub fn saielsr21(&mut self) -> Saielsr21W<IcusargSpec> {
        Saielsr21W::new(self, 21)
    }
    ///Bit 22 - Security attributes of registers for IELSR31 to IELSR0
    #[inline(always)]
    pub fn saielsr22(&mut self) -> Saielsr22W<IcusargSpec> {
        Saielsr22W::new(self, 22)
    }
    ///Bit 23 - Security attributes of registers for IELSR31 to IELSR0
    #[inline(always)]
    pub fn saielsr23(&mut self) -> Saielsr23W<IcusargSpec> {
        Saielsr23W::new(self, 23)
    }
    ///Bit 24 - Security attributes of registers for IELSR31 to IELSR0
    #[inline(always)]
    pub fn saielsr24(&mut self) -> Saielsr24W<IcusargSpec> {
        Saielsr24W::new(self, 24)
    }
    ///Bit 25 - Security attributes of registers for IELSR31 to IELSR0
    #[inline(always)]
    pub fn saielsr25(&mut self) -> Saielsr25W<IcusargSpec> {
        Saielsr25W::new(self, 25)
    }
    ///Bit 26 - Security attributes of registers for IELSR31 to IELSR0
    #[inline(always)]
    pub fn saielsr26(&mut self) -> Saielsr26W<IcusargSpec> {
        Saielsr26W::new(self, 26)
    }
    ///Bit 27 - Security attributes of registers for IELSR31 to IELSR0
    #[inline(always)]
    pub fn saielsr27(&mut self) -> Saielsr27W<IcusargSpec> {
        Saielsr27W::new(self, 27)
    }
    ///Bit 28 - Security attributes of registers for IELSR31 to IELSR0
    #[inline(always)]
    pub fn saielsr28(&mut self) -> Saielsr28W<IcusargSpec> {
        Saielsr28W::new(self, 28)
    }
    ///Bit 29 - Security attributes of registers for IELSR31 to IELSR0
    #[inline(always)]
    pub fn saielsr29(&mut self) -> Saielsr29W<IcusargSpec> {
        Saielsr29W::new(self, 29)
    }
    ///Bit 30 - Security attributes of registers for IELSR31 to IELSR0
    #[inline(always)]
    pub fn saielsr30(&mut self) -> Saielsr30W<IcusargSpec> {
        Saielsr30W::new(self, 30)
    }
    ///Bit 31 - Security attributes of registers for IELSR31 to IELSR0
    #[inline(always)]
    pub fn saielsr31(&mut self) -> Saielsr31W<IcusargSpec> {
        Saielsr31W::new(self, 31)
    }
}
/**Interrupt Controller Unit Security Attribution Register G

You can [`read`](crate::Reg::read) this register and get [`icusarg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icusarg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IcusargSpec;
impl crate::RegisterSpec for IcusargSpec {
    type Ux = u32;
}
///`read()` method returns [`icusarg::R`](R) reader structure
impl crate::Readable for IcusargSpec {}
///`write(|w| ..)` method takes [`icusarg::W`](W) writer structure
impl crate::Writable for IcusargSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICUSARG to value 0xffff_ffff
impl crate::Resettable for IcusargSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
