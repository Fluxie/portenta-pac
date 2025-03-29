///Register `ICUSARA` reader
pub type R = crate::R<IcusaraSpec>;
///Register `ICUSARA` writer
pub type W = crate::W<IcusaraSpec>;
/**Security attributes of registers for the IRQCRn register

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sairqcr00 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Sairqcr00> for bool {
    #[inline(always)]
    fn from(variant: Sairqcr00) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIRQCR00` reader - Security attributes of registers for the IRQCRn register
pub type Sairqcr00R = crate::BitReader<Sairqcr00>;
impl Sairqcr00R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sairqcr00 {
        match self.bits {
            false => Sairqcr00::_0,
            true => Sairqcr00::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sairqcr00::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sairqcr00::_1
    }
}
///Field `SAIRQCR00` writer - Security attributes of registers for the IRQCRn register
pub type Sairqcr00W<'a, REG> = crate::BitWriter<'a, REG, Sairqcr00>;
impl<'a, REG> Sairqcr00W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sairqcr00::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sairqcr00::_1)
    }
}
/**Security attributes of registers for the IRQCRn register

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sairqcr01 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Sairqcr01> for bool {
    #[inline(always)]
    fn from(variant: Sairqcr01) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIRQCR01` reader - Security attributes of registers for the IRQCRn register
pub type Sairqcr01R = crate::BitReader<Sairqcr01>;
impl Sairqcr01R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sairqcr01 {
        match self.bits {
            false => Sairqcr01::_0,
            true => Sairqcr01::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sairqcr01::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sairqcr01::_1
    }
}
///Field `SAIRQCR01` writer - Security attributes of registers for the IRQCRn register
pub type Sairqcr01W<'a, REG> = crate::BitWriter<'a, REG, Sairqcr01>;
impl<'a, REG> Sairqcr01W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sairqcr01::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sairqcr01::_1)
    }
}
/**Security attributes of registers for the IRQCRn register

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sairqcr02 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Sairqcr02> for bool {
    #[inline(always)]
    fn from(variant: Sairqcr02) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIRQCR02` reader - Security attributes of registers for the IRQCRn register
pub type Sairqcr02R = crate::BitReader<Sairqcr02>;
impl Sairqcr02R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sairqcr02 {
        match self.bits {
            false => Sairqcr02::_0,
            true => Sairqcr02::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sairqcr02::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sairqcr02::_1
    }
}
///Field `SAIRQCR02` writer - Security attributes of registers for the IRQCRn register
pub type Sairqcr02W<'a, REG> = crate::BitWriter<'a, REG, Sairqcr02>;
impl<'a, REG> Sairqcr02W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sairqcr02::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sairqcr02::_1)
    }
}
/**Security attributes of registers for the IRQCRn register

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sairqcr03 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Sairqcr03> for bool {
    #[inline(always)]
    fn from(variant: Sairqcr03) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIRQCR03` reader - Security attributes of registers for the IRQCRn register
pub type Sairqcr03R = crate::BitReader<Sairqcr03>;
impl Sairqcr03R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sairqcr03 {
        match self.bits {
            false => Sairqcr03::_0,
            true => Sairqcr03::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sairqcr03::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sairqcr03::_1
    }
}
///Field `SAIRQCR03` writer - Security attributes of registers for the IRQCRn register
pub type Sairqcr03W<'a, REG> = crate::BitWriter<'a, REG, Sairqcr03>;
impl<'a, REG> Sairqcr03W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sairqcr03::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sairqcr03::_1)
    }
}
/**Security attributes of registers for the IRQCRn register

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sairqcr04 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Sairqcr04> for bool {
    #[inline(always)]
    fn from(variant: Sairqcr04) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIRQCR04` reader - Security attributes of registers for the IRQCRn register
pub type Sairqcr04R = crate::BitReader<Sairqcr04>;
impl Sairqcr04R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sairqcr04 {
        match self.bits {
            false => Sairqcr04::_0,
            true => Sairqcr04::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sairqcr04::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sairqcr04::_1
    }
}
///Field `SAIRQCR04` writer - Security attributes of registers for the IRQCRn register
pub type Sairqcr04W<'a, REG> = crate::BitWriter<'a, REG, Sairqcr04>;
impl<'a, REG> Sairqcr04W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sairqcr04::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sairqcr04::_1)
    }
}
/**Security attributes of registers for the IRQCRn register

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sairqcr05 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Sairqcr05> for bool {
    #[inline(always)]
    fn from(variant: Sairqcr05) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIRQCR05` reader - Security attributes of registers for the IRQCRn register
pub type Sairqcr05R = crate::BitReader<Sairqcr05>;
impl Sairqcr05R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sairqcr05 {
        match self.bits {
            false => Sairqcr05::_0,
            true => Sairqcr05::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sairqcr05::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sairqcr05::_1
    }
}
///Field `SAIRQCR05` writer - Security attributes of registers for the IRQCRn register
pub type Sairqcr05W<'a, REG> = crate::BitWriter<'a, REG, Sairqcr05>;
impl<'a, REG> Sairqcr05W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sairqcr05::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sairqcr05::_1)
    }
}
/**Security attributes of registers for the IRQCRn register

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sairqcr06 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Sairqcr06> for bool {
    #[inline(always)]
    fn from(variant: Sairqcr06) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIRQCR06` reader - Security attributes of registers for the IRQCRn register
pub type Sairqcr06R = crate::BitReader<Sairqcr06>;
impl Sairqcr06R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sairqcr06 {
        match self.bits {
            false => Sairqcr06::_0,
            true => Sairqcr06::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sairqcr06::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sairqcr06::_1
    }
}
///Field `SAIRQCR06` writer - Security attributes of registers for the IRQCRn register
pub type Sairqcr06W<'a, REG> = crate::BitWriter<'a, REG, Sairqcr06>;
impl<'a, REG> Sairqcr06W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sairqcr06::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sairqcr06::_1)
    }
}
/**Security attributes of registers for the IRQCRn register

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sairqcr07 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Sairqcr07> for bool {
    #[inline(always)]
    fn from(variant: Sairqcr07) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIRQCR07` reader - Security attributes of registers for the IRQCRn register
pub type Sairqcr07R = crate::BitReader<Sairqcr07>;
impl Sairqcr07R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sairqcr07 {
        match self.bits {
            false => Sairqcr07::_0,
            true => Sairqcr07::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sairqcr07::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sairqcr07::_1
    }
}
///Field `SAIRQCR07` writer - Security attributes of registers for the IRQCRn register
pub type Sairqcr07W<'a, REG> = crate::BitWriter<'a, REG, Sairqcr07>;
impl<'a, REG> Sairqcr07W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sairqcr07::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sairqcr07::_1)
    }
}
/**Security attributes of registers for the IRQCRn register

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sairqcr08 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Sairqcr08> for bool {
    #[inline(always)]
    fn from(variant: Sairqcr08) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIRQCR08` reader - Security attributes of registers for the IRQCRn register
pub type Sairqcr08R = crate::BitReader<Sairqcr08>;
impl Sairqcr08R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sairqcr08 {
        match self.bits {
            false => Sairqcr08::_0,
            true => Sairqcr08::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sairqcr08::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sairqcr08::_1
    }
}
///Field `SAIRQCR08` writer - Security attributes of registers for the IRQCRn register
pub type Sairqcr08W<'a, REG> = crate::BitWriter<'a, REG, Sairqcr08>;
impl<'a, REG> Sairqcr08W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sairqcr08::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sairqcr08::_1)
    }
}
/**Security attributes of registers for the IRQCRn register

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sairqcr09 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Sairqcr09> for bool {
    #[inline(always)]
    fn from(variant: Sairqcr09) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIRQCR09` reader - Security attributes of registers for the IRQCRn register
pub type Sairqcr09R = crate::BitReader<Sairqcr09>;
impl Sairqcr09R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sairqcr09 {
        match self.bits {
            false => Sairqcr09::_0,
            true => Sairqcr09::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sairqcr09::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sairqcr09::_1
    }
}
///Field `SAIRQCR09` writer - Security attributes of registers for the IRQCRn register
pub type Sairqcr09W<'a, REG> = crate::BitWriter<'a, REG, Sairqcr09>;
impl<'a, REG> Sairqcr09W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sairqcr09::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sairqcr09::_1)
    }
}
/**Security attributes of registers for the IRQCRn register

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sairqcr10 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Sairqcr10> for bool {
    #[inline(always)]
    fn from(variant: Sairqcr10) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIRQCR10` reader - Security attributes of registers for the IRQCRn register
pub type Sairqcr10R = crate::BitReader<Sairqcr10>;
impl Sairqcr10R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sairqcr10 {
        match self.bits {
            false => Sairqcr10::_0,
            true => Sairqcr10::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sairqcr10::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sairqcr10::_1
    }
}
///Field `SAIRQCR10` writer - Security attributes of registers for the IRQCRn register
pub type Sairqcr10W<'a, REG> = crate::BitWriter<'a, REG, Sairqcr10>;
impl<'a, REG> Sairqcr10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sairqcr10::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sairqcr10::_1)
    }
}
/**Security attributes of registers for the IRQCRn register

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sairqcr11 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Sairqcr11> for bool {
    #[inline(always)]
    fn from(variant: Sairqcr11) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIRQCR11` reader - Security attributes of registers for the IRQCRn register
pub type Sairqcr11R = crate::BitReader<Sairqcr11>;
impl Sairqcr11R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sairqcr11 {
        match self.bits {
            false => Sairqcr11::_0,
            true => Sairqcr11::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sairqcr11::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sairqcr11::_1
    }
}
///Field `SAIRQCR11` writer - Security attributes of registers for the IRQCRn register
pub type Sairqcr11W<'a, REG> = crate::BitWriter<'a, REG, Sairqcr11>;
impl<'a, REG> Sairqcr11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sairqcr11::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sairqcr11::_1)
    }
}
/**Security attributes of registers for the IRQCRn register

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sairqcr12 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Sairqcr12> for bool {
    #[inline(always)]
    fn from(variant: Sairqcr12) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIRQCR12` reader - Security attributes of registers for the IRQCRn register
pub type Sairqcr12R = crate::BitReader<Sairqcr12>;
impl Sairqcr12R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sairqcr12 {
        match self.bits {
            false => Sairqcr12::_0,
            true => Sairqcr12::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sairqcr12::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sairqcr12::_1
    }
}
///Field `SAIRQCR12` writer - Security attributes of registers for the IRQCRn register
pub type Sairqcr12W<'a, REG> = crate::BitWriter<'a, REG, Sairqcr12>;
impl<'a, REG> Sairqcr12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sairqcr12::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sairqcr12::_1)
    }
}
/**Security attributes of registers for the IRQCRn register

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sairqcr13 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Sairqcr13> for bool {
    #[inline(always)]
    fn from(variant: Sairqcr13) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIRQCR13` reader - Security attributes of registers for the IRQCRn register
pub type Sairqcr13R = crate::BitReader<Sairqcr13>;
impl Sairqcr13R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sairqcr13 {
        match self.bits {
            false => Sairqcr13::_0,
            true => Sairqcr13::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sairqcr13::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sairqcr13::_1
    }
}
///Field `SAIRQCR13` writer - Security attributes of registers for the IRQCRn register
pub type Sairqcr13W<'a, REG> = crate::BitWriter<'a, REG, Sairqcr13>;
impl<'a, REG> Sairqcr13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sairqcr13::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sairqcr13::_1)
    }
}
/**Security attributes of registers for the IRQCRn register

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sairqcr14 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Sairqcr14> for bool {
    #[inline(always)]
    fn from(variant: Sairqcr14) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIRQCR14` reader - Security attributes of registers for the IRQCRn register
pub type Sairqcr14R = crate::BitReader<Sairqcr14>;
impl Sairqcr14R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sairqcr14 {
        match self.bits {
            false => Sairqcr14::_0,
            true => Sairqcr14::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sairqcr14::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sairqcr14::_1
    }
}
///Field `SAIRQCR14` writer - Security attributes of registers for the IRQCRn register
pub type Sairqcr14W<'a, REG> = crate::BitWriter<'a, REG, Sairqcr14>;
impl<'a, REG> Sairqcr14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sairqcr14::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sairqcr14::_1)
    }
}
/**Security attributes of registers for the IRQCRn register

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sairqcr15 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Sairqcr15> for bool {
    #[inline(always)]
    fn from(variant: Sairqcr15) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIRQCR15` reader - Security attributes of registers for the IRQCRn register
pub type Sairqcr15R = crate::BitReader<Sairqcr15>;
impl Sairqcr15R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sairqcr15 {
        match self.bits {
            false => Sairqcr15::_0,
            true => Sairqcr15::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sairqcr15::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sairqcr15::_1
    }
}
///Field `SAIRQCR15` writer - Security attributes of registers for the IRQCRn register
pub type Sairqcr15W<'a, REG> = crate::BitWriter<'a, REG, Sairqcr15>;
impl<'a, REG> Sairqcr15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sairqcr15::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sairqcr15::_1)
    }
}
impl R {
    ///Bit 0 - Security attributes of registers for the IRQCRn register
    #[inline(always)]
    pub fn sairqcr00(&self) -> Sairqcr00R {
        Sairqcr00R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Security attributes of registers for the IRQCRn register
    #[inline(always)]
    pub fn sairqcr01(&self) -> Sairqcr01R {
        Sairqcr01R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Security attributes of registers for the IRQCRn register
    #[inline(always)]
    pub fn sairqcr02(&self) -> Sairqcr02R {
        Sairqcr02R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Security attributes of registers for the IRQCRn register
    #[inline(always)]
    pub fn sairqcr03(&self) -> Sairqcr03R {
        Sairqcr03R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Security attributes of registers for the IRQCRn register
    #[inline(always)]
    pub fn sairqcr04(&self) -> Sairqcr04R {
        Sairqcr04R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Security attributes of registers for the IRQCRn register
    #[inline(always)]
    pub fn sairqcr05(&self) -> Sairqcr05R {
        Sairqcr05R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Security attributes of registers for the IRQCRn register
    #[inline(always)]
    pub fn sairqcr06(&self) -> Sairqcr06R {
        Sairqcr06R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Security attributes of registers for the IRQCRn register
    #[inline(always)]
    pub fn sairqcr07(&self) -> Sairqcr07R {
        Sairqcr07R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Security attributes of registers for the IRQCRn register
    #[inline(always)]
    pub fn sairqcr08(&self) -> Sairqcr08R {
        Sairqcr08R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Security attributes of registers for the IRQCRn register
    #[inline(always)]
    pub fn sairqcr09(&self) -> Sairqcr09R {
        Sairqcr09R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Security attributes of registers for the IRQCRn register
    #[inline(always)]
    pub fn sairqcr10(&self) -> Sairqcr10R {
        Sairqcr10R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Security attributes of registers for the IRQCRn register
    #[inline(always)]
    pub fn sairqcr11(&self) -> Sairqcr11R {
        Sairqcr11R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Security attributes of registers for the IRQCRn register
    #[inline(always)]
    pub fn sairqcr12(&self) -> Sairqcr12R {
        Sairqcr12R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Security attributes of registers for the IRQCRn register
    #[inline(always)]
    pub fn sairqcr13(&self) -> Sairqcr13R {
        Sairqcr13R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Security attributes of registers for the IRQCRn register
    #[inline(always)]
    pub fn sairqcr14(&self) -> Sairqcr14R {
        Sairqcr14R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Security attributes of registers for the IRQCRn register
    #[inline(always)]
    pub fn sairqcr15(&self) -> Sairqcr15R {
        Sairqcr15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICUSARA")
            .field("sairqcr00", &self.sairqcr00())
            .field("sairqcr01", &self.sairqcr01())
            .field("sairqcr02", &self.sairqcr02())
            .field("sairqcr03", &self.sairqcr03())
            .field("sairqcr04", &self.sairqcr04())
            .field("sairqcr05", &self.sairqcr05())
            .field("sairqcr06", &self.sairqcr06())
            .field("sairqcr07", &self.sairqcr07())
            .field("sairqcr08", &self.sairqcr08())
            .field("sairqcr09", &self.sairqcr09())
            .field("sairqcr10", &self.sairqcr10())
            .field("sairqcr11", &self.sairqcr11())
            .field("sairqcr12", &self.sairqcr12())
            .field("sairqcr13", &self.sairqcr13())
            .field("sairqcr14", &self.sairqcr14())
            .field("sairqcr15", &self.sairqcr15())
            .finish()
    }
}
impl W {
    ///Bit 0 - Security attributes of registers for the IRQCRn register
    #[inline(always)]
    pub fn sairqcr00(&mut self) -> Sairqcr00W<IcusaraSpec> {
        Sairqcr00W::new(self, 0)
    }
    ///Bit 1 - Security attributes of registers for the IRQCRn register
    #[inline(always)]
    pub fn sairqcr01(&mut self) -> Sairqcr01W<IcusaraSpec> {
        Sairqcr01W::new(self, 1)
    }
    ///Bit 2 - Security attributes of registers for the IRQCRn register
    #[inline(always)]
    pub fn sairqcr02(&mut self) -> Sairqcr02W<IcusaraSpec> {
        Sairqcr02W::new(self, 2)
    }
    ///Bit 3 - Security attributes of registers for the IRQCRn register
    #[inline(always)]
    pub fn sairqcr03(&mut self) -> Sairqcr03W<IcusaraSpec> {
        Sairqcr03W::new(self, 3)
    }
    ///Bit 4 - Security attributes of registers for the IRQCRn register
    #[inline(always)]
    pub fn sairqcr04(&mut self) -> Sairqcr04W<IcusaraSpec> {
        Sairqcr04W::new(self, 4)
    }
    ///Bit 5 - Security attributes of registers for the IRQCRn register
    #[inline(always)]
    pub fn sairqcr05(&mut self) -> Sairqcr05W<IcusaraSpec> {
        Sairqcr05W::new(self, 5)
    }
    ///Bit 6 - Security attributes of registers for the IRQCRn register
    #[inline(always)]
    pub fn sairqcr06(&mut self) -> Sairqcr06W<IcusaraSpec> {
        Sairqcr06W::new(self, 6)
    }
    ///Bit 7 - Security attributes of registers for the IRQCRn register
    #[inline(always)]
    pub fn sairqcr07(&mut self) -> Sairqcr07W<IcusaraSpec> {
        Sairqcr07W::new(self, 7)
    }
    ///Bit 8 - Security attributes of registers for the IRQCRn register
    #[inline(always)]
    pub fn sairqcr08(&mut self) -> Sairqcr08W<IcusaraSpec> {
        Sairqcr08W::new(self, 8)
    }
    ///Bit 9 - Security attributes of registers for the IRQCRn register
    #[inline(always)]
    pub fn sairqcr09(&mut self) -> Sairqcr09W<IcusaraSpec> {
        Sairqcr09W::new(self, 9)
    }
    ///Bit 10 - Security attributes of registers for the IRQCRn register
    #[inline(always)]
    pub fn sairqcr10(&mut self) -> Sairqcr10W<IcusaraSpec> {
        Sairqcr10W::new(self, 10)
    }
    ///Bit 11 - Security attributes of registers for the IRQCRn register
    #[inline(always)]
    pub fn sairqcr11(&mut self) -> Sairqcr11W<IcusaraSpec> {
        Sairqcr11W::new(self, 11)
    }
    ///Bit 12 - Security attributes of registers for the IRQCRn register
    #[inline(always)]
    pub fn sairqcr12(&mut self) -> Sairqcr12W<IcusaraSpec> {
        Sairqcr12W::new(self, 12)
    }
    ///Bit 13 - Security attributes of registers for the IRQCRn register
    #[inline(always)]
    pub fn sairqcr13(&mut self) -> Sairqcr13W<IcusaraSpec> {
        Sairqcr13W::new(self, 13)
    }
    ///Bit 14 - Security attributes of registers for the IRQCRn register
    #[inline(always)]
    pub fn sairqcr14(&mut self) -> Sairqcr14W<IcusaraSpec> {
        Sairqcr14W::new(self, 14)
    }
    ///Bit 15 - Security attributes of registers for the IRQCRn register
    #[inline(always)]
    pub fn sairqcr15(&mut self) -> Sairqcr15W<IcusaraSpec> {
        Sairqcr15W::new(self, 15)
    }
}
/**Interrupt Controller Unit Security Attribution Register A

You can [`read`](crate::Reg::read) this register and get [`icusara::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icusara::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IcusaraSpec;
impl crate::RegisterSpec for IcusaraSpec {
    type Ux = u32;
}
///`read()` method returns [`icusara::R`](R) reader structure
impl crate::Readable for IcusaraSpec {}
///`write(|w| ..)` method takes [`icusara::W`](W) writer structure
impl crate::Writable for IcusaraSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICUSARA to value 0xffff_ffff
impl crate::Resettable for IcusaraSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
