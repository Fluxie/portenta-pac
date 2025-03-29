///Register `ICUSARI` reader
pub type R = crate::R<IcusariSpec>;
///Register `ICUSARI` writer
pub type W = crate::W<IcusariSpec>;
/**Security attributes of registers for IELSR95 to IELSR64

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr64 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr64> for bool {
    #[inline(always)]
    fn from(variant: Saielsr64) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR64` reader - Security attributes of registers for IELSR95 to IELSR64
pub type Saielsr64R = crate::BitReader<Saielsr64>;
impl Saielsr64R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr64 {
        match self.bits {
            false => Saielsr64::_0,
            true => Saielsr64::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr64::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr64::_1
    }
}
///Field `SAIELSR64` writer - Security attributes of registers for IELSR95 to IELSR64
pub type Saielsr64W<'a, REG> = crate::BitWriter<'a, REG, Saielsr64>;
impl<'a, REG> Saielsr64W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr64::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr64::_1)
    }
}
/**Security attributes of registers for IELSR95 to IELSR64

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr65 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr65> for bool {
    #[inline(always)]
    fn from(variant: Saielsr65) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR65` reader - Security attributes of registers for IELSR95 to IELSR64
pub type Saielsr65R = crate::BitReader<Saielsr65>;
impl Saielsr65R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr65 {
        match self.bits {
            false => Saielsr65::_0,
            true => Saielsr65::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr65::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr65::_1
    }
}
///Field `SAIELSR65` writer - Security attributes of registers for IELSR95 to IELSR64
pub type Saielsr65W<'a, REG> = crate::BitWriter<'a, REG, Saielsr65>;
impl<'a, REG> Saielsr65W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr65::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr65::_1)
    }
}
/**Security attributes of registers for IELSR95 to IELSR64

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr66 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr66> for bool {
    #[inline(always)]
    fn from(variant: Saielsr66) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR66` reader - Security attributes of registers for IELSR95 to IELSR64
pub type Saielsr66R = crate::BitReader<Saielsr66>;
impl Saielsr66R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr66 {
        match self.bits {
            false => Saielsr66::_0,
            true => Saielsr66::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr66::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr66::_1
    }
}
///Field `SAIELSR66` writer - Security attributes of registers for IELSR95 to IELSR64
pub type Saielsr66W<'a, REG> = crate::BitWriter<'a, REG, Saielsr66>;
impl<'a, REG> Saielsr66W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr66::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr66::_1)
    }
}
/**Security attributes of registers for IELSR95 to IELSR64

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr67 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr67> for bool {
    #[inline(always)]
    fn from(variant: Saielsr67) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR67` reader - Security attributes of registers for IELSR95 to IELSR64
pub type Saielsr67R = crate::BitReader<Saielsr67>;
impl Saielsr67R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr67 {
        match self.bits {
            false => Saielsr67::_0,
            true => Saielsr67::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr67::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr67::_1
    }
}
///Field `SAIELSR67` writer - Security attributes of registers for IELSR95 to IELSR64
pub type Saielsr67W<'a, REG> = crate::BitWriter<'a, REG, Saielsr67>;
impl<'a, REG> Saielsr67W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr67::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr67::_1)
    }
}
/**Security attributes of registers for IELSR95 to IELSR64

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr68 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr68> for bool {
    #[inline(always)]
    fn from(variant: Saielsr68) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR68` reader - Security attributes of registers for IELSR95 to IELSR64
pub type Saielsr68R = crate::BitReader<Saielsr68>;
impl Saielsr68R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr68 {
        match self.bits {
            false => Saielsr68::_0,
            true => Saielsr68::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr68::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr68::_1
    }
}
///Field `SAIELSR68` writer - Security attributes of registers for IELSR95 to IELSR64
pub type Saielsr68W<'a, REG> = crate::BitWriter<'a, REG, Saielsr68>;
impl<'a, REG> Saielsr68W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr68::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr68::_1)
    }
}
/**Security attributes of registers for IELSR95 to IELSR64

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr69 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr69> for bool {
    #[inline(always)]
    fn from(variant: Saielsr69) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR69` reader - Security attributes of registers for IELSR95 to IELSR64
pub type Saielsr69R = crate::BitReader<Saielsr69>;
impl Saielsr69R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr69 {
        match self.bits {
            false => Saielsr69::_0,
            true => Saielsr69::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr69::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr69::_1
    }
}
///Field `SAIELSR69` writer - Security attributes of registers for IELSR95 to IELSR64
pub type Saielsr69W<'a, REG> = crate::BitWriter<'a, REG, Saielsr69>;
impl<'a, REG> Saielsr69W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr69::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr69::_1)
    }
}
/**Security attributes of registers for IELSR95 to IELSR64

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr70 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr70> for bool {
    #[inline(always)]
    fn from(variant: Saielsr70) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR70` reader - Security attributes of registers for IELSR95 to IELSR64
pub type Saielsr70R = crate::BitReader<Saielsr70>;
impl Saielsr70R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr70 {
        match self.bits {
            false => Saielsr70::_0,
            true => Saielsr70::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr70::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr70::_1
    }
}
///Field `SAIELSR70` writer - Security attributes of registers for IELSR95 to IELSR64
pub type Saielsr70W<'a, REG> = crate::BitWriter<'a, REG, Saielsr70>;
impl<'a, REG> Saielsr70W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr70::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr70::_1)
    }
}
/**Security attributes of registers for IELSR95 to IELSR64

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr71 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr71> for bool {
    #[inline(always)]
    fn from(variant: Saielsr71) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR71` reader - Security attributes of registers for IELSR95 to IELSR64
pub type Saielsr71R = crate::BitReader<Saielsr71>;
impl Saielsr71R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr71 {
        match self.bits {
            false => Saielsr71::_0,
            true => Saielsr71::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr71::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr71::_1
    }
}
///Field `SAIELSR71` writer - Security attributes of registers for IELSR95 to IELSR64
pub type Saielsr71W<'a, REG> = crate::BitWriter<'a, REG, Saielsr71>;
impl<'a, REG> Saielsr71W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr71::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr71::_1)
    }
}
/**Security attributes of registers for IELSR95 to IELSR64

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr72 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr72> for bool {
    #[inline(always)]
    fn from(variant: Saielsr72) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR72` reader - Security attributes of registers for IELSR95 to IELSR64
pub type Saielsr72R = crate::BitReader<Saielsr72>;
impl Saielsr72R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr72 {
        match self.bits {
            false => Saielsr72::_0,
            true => Saielsr72::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr72::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr72::_1
    }
}
///Field `SAIELSR72` writer - Security attributes of registers for IELSR95 to IELSR64
pub type Saielsr72W<'a, REG> = crate::BitWriter<'a, REG, Saielsr72>;
impl<'a, REG> Saielsr72W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr72::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr72::_1)
    }
}
/**Security attributes of registers for IELSR95 to IELSR64

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr73 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr73> for bool {
    #[inline(always)]
    fn from(variant: Saielsr73) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR73` reader - Security attributes of registers for IELSR95 to IELSR64
pub type Saielsr73R = crate::BitReader<Saielsr73>;
impl Saielsr73R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr73 {
        match self.bits {
            false => Saielsr73::_0,
            true => Saielsr73::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr73::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr73::_1
    }
}
///Field `SAIELSR73` writer - Security attributes of registers for IELSR95 to IELSR64
pub type Saielsr73W<'a, REG> = crate::BitWriter<'a, REG, Saielsr73>;
impl<'a, REG> Saielsr73W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr73::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr73::_1)
    }
}
/**Security attributes of registers for IELSR95 to IELSR64

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr74 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr74> for bool {
    #[inline(always)]
    fn from(variant: Saielsr74) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR74` reader - Security attributes of registers for IELSR95 to IELSR64
pub type Saielsr74R = crate::BitReader<Saielsr74>;
impl Saielsr74R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr74 {
        match self.bits {
            false => Saielsr74::_0,
            true => Saielsr74::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr74::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr74::_1
    }
}
///Field `SAIELSR74` writer - Security attributes of registers for IELSR95 to IELSR64
pub type Saielsr74W<'a, REG> = crate::BitWriter<'a, REG, Saielsr74>;
impl<'a, REG> Saielsr74W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr74::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr74::_1)
    }
}
/**Security attributes of registers for IELSR95 to IELSR64

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr75 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr75> for bool {
    #[inline(always)]
    fn from(variant: Saielsr75) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR75` reader - Security attributes of registers for IELSR95 to IELSR64
pub type Saielsr75R = crate::BitReader<Saielsr75>;
impl Saielsr75R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr75 {
        match self.bits {
            false => Saielsr75::_0,
            true => Saielsr75::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr75::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr75::_1
    }
}
///Field `SAIELSR75` writer - Security attributes of registers for IELSR95 to IELSR64
pub type Saielsr75W<'a, REG> = crate::BitWriter<'a, REG, Saielsr75>;
impl<'a, REG> Saielsr75W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr75::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr75::_1)
    }
}
/**Security attributes of registers for IELSR95 to IELSR64

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr76 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr76> for bool {
    #[inline(always)]
    fn from(variant: Saielsr76) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR76` reader - Security attributes of registers for IELSR95 to IELSR64
pub type Saielsr76R = crate::BitReader<Saielsr76>;
impl Saielsr76R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr76 {
        match self.bits {
            false => Saielsr76::_0,
            true => Saielsr76::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr76::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr76::_1
    }
}
///Field `SAIELSR76` writer - Security attributes of registers for IELSR95 to IELSR64
pub type Saielsr76W<'a, REG> = crate::BitWriter<'a, REG, Saielsr76>;
impl<'a, REG> Saielsr76W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr76::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr76::_1)
    }
}
/**Security attributes of registers for IELSR95 to IELSR64

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr77 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr77> for bool {
    #[inline(always)]
    fn from(variant: Saielsr77) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR77` reader - Security attributes of registers for IELSR95 to IELSR64
pub type Saielsr77R = crate::BitReader<Saielsr77>;
impl Saielsr77R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr77 {
        match self.bits {
            false => Saielsr77::_0,
            true => Saielsr77::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr77::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr77::_1
    }
}
///Field `SAIELSR77` writer - Security attributes of registers for IELSR95 to IELSR64
pub type Saielsr77W<'a, REG> = crate::BitWriter<'a, REG, Saielsr77>;
impl<'a, REG> Saielsr77W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr77::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr77::_1)
    }
}
/**Security attributes of registers for IELSR95 to IELSR64

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr78 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr78> for bool {
    #[inline(always)]
    fn from(variant: Saielsr78) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR78` reader - Security attributes of registers for IELSR95 to IELSR64
pub type Saielsr78R = crate::BitReader<Saielsr78>;
impl Saielsr78R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr78 {
        match self.bits {
            false => Saielsr78::_0,
            true => Saielsr78::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr78::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr78::_1
    }
}
///Field `SAIELSR78` writer - Security attributes of registers for IELSR95 to IELSR64
pub type Saielsr78W<'a, REG> = crate::BitWriter<'a, REG, Saielsr78>;
impl<'a, REG> Saielsr78W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr78::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr78::_1)
    }
}
/**Security attributes of registers for IELSR95 to IELSR64

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr79 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr79> for bool {
    #[inline(always)]
    fn from(variant: Saielsr79) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR79` reader - Security attributes of registers for IELSR95 to IELSR64
pub type Saielsr79R = crate::BitReader<Saielsr79>;
impl Saielsr79R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr79 {
        match self.bits {
            false => Saielsr79::_0,
            true => Saielsr79::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr79::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr79::_1
    }
}
///Field `SAIELSR79` writer - Security attributes of registers for IELSR95 to IELSR64
pub type Saielsr79W<'a, REG> = crate::BitWriter<'a, REG, Saielsr79>;
impl<'a, REG> Saielsr79W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr79::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr79::_1)
    }
}
/**Security attributes of registers for IELSR95 to IELSR64

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr80 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr80> for bool {
    #[inline(always)]
    fn from(variant: Saielsr80) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR80` reader - Security attributes of registers for IELSR95 to IELSR64
pub type Saielsr80R = crate::BitReader<Saielsr80>;
impl Saielsr80R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr80 {
        match self.bits {
            false => Saielsr80::_0,
            true => Saielsr80::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr80::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr80::_1
    }
}
///Field `SAIELSR80` writer - Security attributes of registers for IELSR95 to IELSR64
pub type Saielsr80W<'a, REG> = crate::BitWriter<'a, REG, Saielsr80>;
impl<'a, REG> Saielsr80W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr80::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr80::_1)
    }
}
/**Security attributes of registers for IELSR95 to IELSR64

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr81 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr81> for bool {
    #[inline(always)]
    fn from(variant: Saielsr81) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR81` reader - Security attributes of registers for IELSR95 to IELSR64
pub type Saielsr81R = crate::BitReader<Saielsr81>;
impl Saielsr81R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr81 {
        match self.bits {
            false => Saielsr81::_0,
            true => Saielsr81::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr81::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr81::_1
    }
}
///Field `SAIELSR81` writer - Security attributes of registers for IELSR95 to IELSR64
pub type Saielsr81W<'a, REG> = crate::BitWriter<'a, REG, Saielsr81>;
impl<'a, REG> Saielsr81W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr81::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr81::_1)
    }
}
/**Security attributes of registers for IELSR95 to IELSR64

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr82 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr82> for bool {
    #[inline(always)]
    fn from(variant: Saielsr82) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR82` reader - Security attributes of registers for IELSR95 to IELSR64
pub type Saielsr82R = crate::BitReader<Saielsr82>;
impl Saielsr82R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr82 {
        match self.bits {
            false => Saielsr82::_0,
            true => Saielsr82::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr82::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr82::_1
    }
}
///Field `SAIELSR82` writer - Security attributes of registers for IELSR95 to IELSR64
pub type Saielsr82W<'a, REG> = crate::BitWriter<'a, REG, Saielsr82>;
impl<'a, REG> Saielsr82W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr82::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr82::_1)
    }
}
/**Security attributes of registers for IELSR95 to IELSR64

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr83 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr83> for bool {
    #[inline(always)]
    fn from(variant: Saielsr83) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR83` reader - Security attributes of registers for IELSR95 to IELSR64
pub type Saielsr83R = crate::BitReader<Saielsr83>;
impl Saielsr83R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr83 {
        match self.bits {
            false => Saielsr83::_0,
            true => Saielsr83::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr83::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr83::_1
    }
}
///Field `SAIELSR83` writer - Security attributes of registers for IELSR95 to IELSR64
pub type Saielsr83W<'a, REG> = crate::BitWriter<'a, REG, Saielsr83>;
impl<'a, REG> Saielsr83W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr83::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr83::_1)
    }
}
/**Security attributes of registers for IELSR95 to IELSR64

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr84 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr84> for bool {
    #[inline(always)]
    fn from(variant: Saielsr84) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR84` reader - Security attributes of registers for IELSR95 to IELSR64
pub type Saielsr84R = crate::BitReader<Saielsr84>;
impl Saielsr84R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr84 {
        match self.bits {
            false => Saielsr84::_0,
            true => Saielsr84::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr84::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr84::_1
    }
}
///Field `SAIELSR84` writer - Security attributes of registers for IELSR95 to IELSR64
pub type Saielsr84W<'a, REG> = crate::BitWriter<'a, REG, Saielsr84>;
impl<'a, REG> Saielsr84W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr84::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr84::_1)
    }
}
/**Security attributes of registers for IELSR95 to IELSR64

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr85 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr85> for bool {
    #[inline(always)]
    fn from(variant: Saielsr85) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR85` reader - Security attributes of registers for IELSR95 to IELSR64
pub type Saielsr85R = crate::BitReader<Saielsr85>;
impl Saielsr85R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr85 {
        match self.bits {
            false => Saielsr85::_0,
            true => Saielsr85::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr85::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr85::_1
    }
}
///Field `SAIELSR85` writer - Security attributes of registers for IELSR95 to IELSR64
pub type Saielsr85W<'a, REG> = crate::BitWriter<'a, REG, Saielsr85>;
impl<'a, REG> Saielsr85W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr85::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr85::_1)
    }
}
/**Security attributes of registers for IELSR95 to IELSR64

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr86 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr86> for bool {
    #[inline(always)]
    fn from(variant: Saielsr86) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR86` reader - Security attributes of registers for IELSR95 to IELSR64
pub type Saielsr86R = crate::BitReader<Saielsr86>;
impl Saielsr86R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr86 {
        match self.bits {
            false => Saielsr86::_0,
            true => Saielsr86::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr86::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr86::_1
    }
}
///Field `SAIELSR86` writer - Security attributes of registers for IELSR95 to IELSR64
pub type Saielsr86W<'a, REG> = crate::BitWriter<'a, REG, Saielsr86>;
impl<'a, REG> Saielsr86W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr86::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr86::_1)
    }
}
/**Security attributes of registers for IELSR95 to IELSR64

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr87 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr87> for bool {
    #[inline(always)]
    fn from(variant: Saielsr87) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR87` reader - Security attributes of registers for IELSR95 to IELSR64
pub type Saielsr87R = crate::BitReader<Saielsr87>;
impl Saielsr87R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr87 {
        match self.bits {
            false => Saielsr87::_0,
            true => Saielsr87::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr87::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr87::_1
    }
}
///Field `SAIELSR87` writer - Security attributes of registers for IELSR95 to IELSR64
pub type Saielsr87W<'a, REG> = crate::BitWriter<'a, REG, Saielsr87>;
impl<'a, REG> Saielsr87W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr87::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr87::_1)
    }
}
/**Security attributes of registers for IELSR95 to IELSR64

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr88 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr88> for bool {
    #[inline(always)]
    fn from(variant: Saielsr88) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR88` reader - Security attributes of registers for IELSR95 to IELSR64
pub type Saielsr88R = crate::BitReader<Saielsr88>;
impl Saielsr88R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr88 {
        match self.bits {
            false => Saielsr88::_0,
            true => Saielsr88::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr88::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr88::_1
    }
}
///Field `SAIELSR88` writer - Security attributes of registers for IELSR95 to IELSR64
pub type Saielsr88W<'a, REG> = crate::BitWriter<'a, REG, Saielsr88>;
impl<'a, REG> Saielsr88W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr88::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr88::_1)
    }
}
/**Security attributes of registers for IELSR95 to IELSR64

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr89 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr89> for bool {
    #[inline(always)]
    fn from(variant: Saielsr89) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR89` reader - Security attributes of registers for IELSR95 to IELSR64
pub type Saielsr89R = crate::BitReader<Saielsr89>;
impl Saielsr89R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr89 {
        match self.bits {
            false => Saielsr89::_0,
            true => Saielsr89::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr89::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr89::_1
    }
}
///Field `SAIELSR89` writer - Security attributes of registers for IELSR95 to IELSR64
pub type Saielsr89W<'a, REG> = crate::BitWriter<'a, REG, Saielsr89>;
impl<'a, REG> Saielsr89W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr89::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr89::_1)
    }
}
/**Security attributes of registers for IELSR95 to IELSR64

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr90 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr90> for bool {
    #[inline(always)]
    fn from(variant: Saielsr90) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR90` reader - Security attributes of registers for IELSR95 to IELSR64
pub type Saielsr90R = crate::BitReader<Saielsr90>;
impl Saielsr90R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr90 {
        match self.bits {
            false => Saielsr90::_0,
            true => Saielsr90::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr90::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr90::_1
    }
}
///Field `SAIELSR90` writer - Security attributes of registers for IELSR95 to IELSR64
pub type Saielsr90W<'a, REG> = crate::BitWriter<'a, REG, Saielsr90>;
impl<'a, REG> Saielsr90W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr90::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr90::_1)
    }
}
/**Security attributes of registers for IELSR95 to IELSR64

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr91 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr91> for bool {
    #[inline(always)]
    fn from(variant: Saielsr91) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR91` reader - Security attributes of registers for IELSR95 to IELSR64
pub type Saielsr91R = crate::BitReader<Saielsr91>;
impl Saielsr91R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr91 {
        match self.bits {
            false => Saielsr91::_0,
            true => Saielsr91::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr91::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr91::_1
    }
}
///Field `SAIELSR91` writer - Security attributes of registers for IELSR95 to IELSR64
pub type Saielsr91W<'a, REG> = crate::BitWriter<'a, REG, Saielsr91>;
impl<'a, REG> Saielsr91W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr91::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr91::_1)
    }
}
/**Security attributes of registers for IELSR95 to IELSR64

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr92 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr92> for bool {
    #[inline(always)]
    fn from(variant: Saielsr92) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR92` reader - Security attributes of registers for IELSR95 to IELSR64
pub type Saielsr92R = crate::BitReader<Saielsr92>;
impl Saielsr92R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr92 {
        match self.bits {
            false => Saielsr92::_0,
            true => Saielsr92::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr92::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr92::_1
    }
}
///Field `SAIELSR92` writer - Security attributes of registers for IELSR95 to IELSR64
pub type Saielsr92W<'a, REG> = crate::BitWriter<'a, REG, Saielsr92>;
impl<'a, REG> Saielsr92W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr92::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr92::_1)
    }
}
/**Security attributes of registers for IELSR95 to IELSR64

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr93 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr93> for bool {
    #[inline(always)]
    fn from(variant: Saielsr93) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR93` reader - Security attributes of registers for IELSR95 to IELSR64
pub type Saielsr93R = crate::BitReader<Saielsr93>;
impl Saielsr93R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr93 {
        match self.bits {
            false => Saielsr93::_0,
            true => Saielsr93::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr93::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr93::_1
    }
}
///Field `SAIELSR93` writer - Security attributes of registers for IELSR95 to IELSR64
pub type Saielsr93W<'a, REG> = crate::BitWriter<'a, REG, Saielsr93>;
impl<'a, REG> Saielsr93W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr93::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr93::_1)
    }
}
/**Security attributes of registers for IELSR95 to IELSR64

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr94 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr94> for bool {
    #[inline(always)]
    fn from(variant: Saielsr94) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR94` reader - Security attributes of registers for IELSR95 to IELSR64
pub type Saielsr94R = crate::BitReader<Saielsr94>;
impl Saielsr94R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr94 {
        match self.bits {
            false => Saielsr94::_0,
            true => Saielsr94::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr94::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr94::_1
    }
}
///Field `SAIELSR94` writer - Security attributes of registers for IELSR95 to IELSR64
pub type Saielsr94W<'a, REG> = crate::BitWriter<'a, REG, Saielsr94>;
impl<'a, REG> Saielsr94W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr94::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr94::_1)
    }
}
/**Security attributes of registers for IELSR95 to IELSR64

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr95 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr95> for bool {
    #[inline(always)]
    fn from(variant: Saielsr95) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR95` reader - Security attributes of registers for IELSR95 to IELSR64
pub type Saielsr95R = crate::BitReader<Saielsr95>;
impl Saielsr95R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr95 {
        match self.bits {
            false => Saielsr95::_0,
            true => Saielsr95::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr95::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr95::_1
    }
}
///Field `SAIELSR95` writer - Security attributes of registers for IELSR95 to IELSR64
pub type Saielsr95W<'a, REG> = crate::BitWriter<'a, REG, Saielsr95>;
impl<'a, REG> Saielsr95W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr95::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr95::_1)
    }
}
impl R {
    ///Bit 0 - Security attributes of registers for IELSR95 to IELSR64
    #[inline(always)]
    pub fn saielsr64(&self) -> Saielsr64R {
        Saielsr64R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Security attributes of registers for IELSR95 to IELSR64
    #[inline(always)]
    pub fn saielsr65(&self) -> Saielsr65R {
        Saielsr65R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Security attributes of registers for IELSR95 to IELSR64
    #[inline(always)]
    pub fn saielsr66(&self) -> Saielsr66R {
        Saielsr66R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Security attributes of registers for IELSR95 to IELSR64
    #[inline(always)]
    pub fn saielsr67(&self) -> Saielsr67R {
        Saielsr67R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Security attributes of registers for IELSR95 to IELSR64
    #[inline(always)]
    pub fn saielsr68(&self) -> Saielsr68R {
        Saielsr68R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Security attributes of registers for IELSR95 to IELSR64
    #[inline(always)]
    pub fn saielsr69(&self) -> Saielsr69R {
        Saielsr69R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Security attributes of registers for IELSR95 to IELSR64
    #[inline(always)]
    pub fn saielsr70(&self) -> Saielsr70R {
        Saielsr70R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Security attributes of registers for IELSR95 to IELSR64
    #[inline(always)]
    pub fn saielsr71(&self) -> Saielsr71R {
        Saielsr71R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Security attributes of registers for IELSR95 to IELSR64
    #[inline(always)]
    pub fn saielsr72(&self) -> Saielsr72R {
        Saielsr72R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Security attributes of registers for IELSR95 to IELSR64
    #[inline(always)]
    pub fn saielsr73(&self) -> Saielsr73R {
        Saielsr73R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Security attributes of registers for IELSR95 to IELSR64
    #[inline(always)]
    pub fn saielsr74(&self) -> Saielsr74R {
        Saielsr74R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Security attributes of registers for IELSR95 to IELSR64
    #[inline(always)]
    pub fn saielsr75(&self) -> Saielsr75R {
        Saielsr75R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Security attributes of registers for IELSR95 to IELSR64
    #[inline(always)]
    pub fn saielsr76(&self) -> Saielsr76R {
        Saielsr76R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Security attributes of registers for IELSR95 to IELSR64
    #[inline(always)]
    pub fn saielsr77(&self) -> Saielsr77R {
        Saielsr77R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Security attributes of registers for IELSR95 to IELSR64
    #[inline(always)]
    pub fn saielsr78(&self) -> Saielsr78R {
        Saielsr78R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Security attributes of registers for IELSR95 to IELSR64
    #[inline(always)]
    pub fn saielsr79(&self) -> Saielsr79R {
        Saielsr79R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Security attributes of registers for IELSR95 to IELSR64
    #[inline(always)]
    pub fn saielsr80(&self) -> Saielsr80R {
        Saielsr80R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Security attributes of registers for IELSR95 to IELSR64
    #[inline(always)]
    pub fn saielsr81(&self) -> Saielsr81R {
        Saielsr81R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Security attributes of registers for IELSR95 to IELSR64
    #[inline(always)]
    pub fn saielsr82(&self) -> Saielsr82R {
        Saielsr82R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Security attributes of registers for IELSR95 to IELSR64
    #[inline(always)]
    pub fn saielsr83(&self) -> Saielsr83R {
        Saielsr83R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Security attributes of registers for IELSR95 to IELSR64
    #[inline(always)]
    pub fn saielsr84(&self) -> Saielsr84R {
        Saielsr84R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Security attributes of registers for IELSR95 to IELSR64
    #[inline(always)]
    pub fn saielsr85(&self) -> Saielsr85R {
        Saielsr85R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Security attributes of registers for IELSR95 to IELSR64
    #[inline(always)]
    pub fn saielsr86(&self) -> Saielsr86R {
        Saielsr86R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Security attributes of registers for IELSR95 to IELSR64
    #[inline(always)]
    pub fn saielsr87(&self) -> Saielsr87R {
        Saielsr87R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Security attributes of registers for IELSR95 to IELSR64
    #[inline(always)]
    pub fn saielsr88(&self) -> Saielsr88R {
        Saielsr88R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Security attributes of registers for IELSR95 to IELSR64
    #[inline(always)]
    pub fn saielsr89(&self) -> Saielsr89R {
        Saielsr89R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Security attributes of registers for IELSR95 to IELSR64
    #[inline(always)]
    pub fn saielsr90(&self) -> Saielsr90R {
        Saielsr90R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Security attributes of registers for IELSR95 to IELSR64
    #[inline(always)]
    pub fn saielsr91(&self) -> Saielsr91R {
        Saielsr91R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Security attributes of registers for IELSR95 to IELSR64
    #[inline(always)]
    pub fn saielsr92(&self) -> Saielsr92R {
        Saielsr92R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Security attributes of registers for IELSR95 to IELSR64
    #[inline(always)]
    pub fn saielsr93(&self) -> Saielsr93R {
        Saielsr93R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Security attributes of registers for IELSR95 to IELSR64
    #[inline(always)]
    pub fn saielsr94(&self) -> Saielsr94R {
        Saielsr94R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Security attributes of registers for IELSR95 to IELSR64
    #[inline(always)]
    pub fn saielsr95(&self) -> Saielsr95R {
        Saielsr95R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICUSARI")
            .field("saielsr64", &self.saielsr64())
            .field("saielsr65", &self.saielsr65())
            .field("saielsr66", &self.saielsr66())
            .field("saielsr67", &self.saielsr67())
            .field("saielsr68", &self.saielsr68())
            .field("saielsr69", &self.saielsr69())
            .field("saielsr70", &self.saielsr70())
            .field("saielsr71", &self.saielsr71())
            .field("saielsr72", &self.saielsr72())
            .field("saielsr73", &self.saielsr73())
            .field("saielsr74", &self.saielsr74())
            .field("saielsr75", &self.saielsr75())
            .field("saielsr76", &self.saielsr76())
            .field("saielsr77", &self.saielsr77())
            .field("saielsr78", &self.saielsr78())
            .field("saielsr79", &self.saielsr79())
            .field("saielsr80", &self.saielsr80())
            .field("saielsr81", &self.saielsr81())
            .field("saielsr82", &self.saielsr82())
            .field("saielsr83", &self.saielsr83())
            .field("saielsr84", &self.saielsr84())
            .field("saielsr85", &self.saielsr85())
            .field("saielsr86", &self.saielsr86())
            .field("saielsr87", &self.saielsr87())
            .field("saielsr88", &self.saielsr88())
            .field("saielsr89", &self.saielsr89())
            .field("saielsr90", &self.saielsr90())
            .field("saielsr91", &self.saielsr91())
            .field("saielsr92", &self.saielsr92())
            .field("saielsr93", &self.saielsr93())
            .field("saielsr94", &self.saielsr94())
            .field("saielsr95", &self.saielsr95())
            .finish()
    }
}
impl W {
    ///Bit 0 - Security attributes of registers for IELSR95 to IELSR64
    #[inline(always)]
    pub fn saielsr64(&mut self) -> Saielsr64W<IcusariSpec> {
        Saielsr64W::new(self, 0)
    }
    ///Bit 1 - Security attributes of registers for IELSR95 to IELSR64
    #[inline(always)]
    pub fn saielsr65(&mut self) -> Saielsr65W<IcusariSpec> {
        Saielsr65W::new(self, 1)
    }
    ///Bit 2 - Security attributes of registers for IELSR95 to IELSR64
    #[inline(always)]
    pub fn saielsr66(&mut self) -> Saielsr66W<IcusariSpec> {
        Saielsr66W::new(self, 2)
    }
    ///Bit 3 - Security attributes of registers for IELSR95 to IELSR64
    #[inline(always)]
    pub fn saielsr67(&mut self) -> Saielsr67W<IcusariSpec> {
        Saielsr67W::new(self, 3)
    }
    ///Bit 4 - Security attributes of registers for IELSR95 to IELSR64
    #[inline(always)]
    pub fn saielsr68(&mut self) -> Saielsr68W<IcusariSpec> {
        Saielsr68W::new(self, 4)
    }
    ///Bit 5 - Security attributes of registers for IELSR95 to IELSR64
    #[inline(always)]
    pub fn saielsr69(&mut self) -> Saielsr69W<IcusariSpec> {
        Saielsr69W::new(self, 5)
    }
    ///Bit 6 - Security attributes of registers for IELSR95 to IELSR64
    #[inline(always)]
    pub fn saielsr70(&mut self) -> Saielsr70W<IcusariSpec> {
        Saielsr70W::new(self, 6)
    }
    ///Bit 7 - Security attributes of registers for IELSR95 to IELSR64
    #[inline(always)]
    pub fn saielsr71(&mut self) -> Saielsr71W<IcusariSpec> {
        Saielsr71W::new(self, 7)
    }
    ///Bit 8 - Security attributes of registers for IELSR95 to IELSR64
    #[inline(always)]
    pub fn saielsr72(&mut self) -> Saielsr72W<IcusariSpec> {
        Saielsr72W::new(self, 8)
    }
    ///Bit 9 - Security attributes of registers for IELSR95 to IELSR64
    #[inline(always)]
    pub fn saielsr73(&mut self) -> Saielsr73W<IcusariSpec> {
        Saielsr73W::new(self, 9)
    }
    ///Bit 10 - Security attributes of registers for IELSR95 to IELSR64
    #[inline(always)]
    pub fn saielsr74(&mut self) -> Saielsr74W<IcusariSpec> {
        Saielsr74W::new(self, 10)
    }
    ///Bit 11 - Security attributes of registers for IELSR95 to IELSR64
    #[inline(always)]
    pub fn saielsr75(&mut self) -> Saielsr75W<IcusariSpec> {
        Saielsr75W::new(self, 11)
    }
    ///Bit 12 - Security attributes of registers for IELSR95 to IELSR64
    #[inline(always)]
    pub fn saielsr76(&mut self) -> Saielsr76W<IcusariSpec> {
        Saielsr76W::new(self, 12)
    }
    ///Bit 13 - Security attributes of registers for IELSR95 to IELSR64
    #[inline(always)]
    pub fn saielsr77(&mut self) -> Saielsr77W<IcusariSpec> {
        Saielsr77W::new(self, 13)
    }
    ///Bit 14 - Security attributes of registers for IELSR95 to IELSR64
    #[inline(always)]
    pub fn saielsr78(&mut self) -> Saielsr78W<IcusariSpec> {
        Saielsr78W::new(self, 14)
    }
    ///Bit 15 - Security attributes of registers for IELSR95 to IELSR64
    #[inline(always)]
    pub fn saielsr79(&mut self) -> Saielsr79W<IcusariSpec> {
        Saielsr79W::new(self, 15)
    }
    ///Bit 16 - Security attributes of registers for IELSR95 to IELSR64
    #[inline(always)]
    pub fn saielsr80(&mut self) -> Saielsr80W<IcusariSpec> {
        Saielsr80W::new(self, 16)
    }
    ///Bit 17 - Security attributes of registers for IELSR95 to IELSR64
    #[inline(always)]
    pub fn saielsr81(&mut self) -> Saielsr81W<IcusariSpec> {
        Saielsr81W::new(self, 17)
    }
    ///Bit 18 - Security attributes of registers for IELSR95 to IELSR64
    #[inline(always)]
    pub fn saielsr82(&mut self) -> Saielsr82W<IcusariSpec> {
        Saielsr82W::new(self, 18)
    }
    ///Bit 19 - Security attributes of registers for IELSR95 to IELSR64
    #[inline(always)]
    pub fn saielsr83(&mut self) -> Saielsr83W<IcusariSpec> {
        Saielsr83W::new(self, 19)
    }
    ///Bit 20 - Security attributes of registers for IELSR95 to IELSR64
    #[inline(always)]
    pub fn saielsr84(&mut self) -> Saielsr84W<IcusariSpec> {
        Saielsr84W::new(self, 20)
    }
    ///Bit 21 - Security attributes of registers for IELSR95 to IELSR64
    #[inline(always)]
    pub fn saielsr85(&mut self) -> Saielsr85W<IcusariSpec> {
        Saielsr85W::new(self, 21)
    }
    ///Bit 22 - Security attributes of registers for IELSR95 to IELSR64
    #[inline(always)]
    pub fn saielsr86(&mut self) -> Saielsr86W<IcusariSpec> {
        Saielsr86W::new(self, 22)
    }
    ///Bit 23 - Security attributes of registers for IELSR95 to IELSR64
    #[inline(always)]
    pub fn saielsr87(&mut self) -> Saielsr87W<IcusariSpec> {
        Saielsr87W::new(self, 23)
    }
    ///Bit 24 - Security attributes of registers for IELSR95 to IELSR64
    #[inline(always)]
    pub fn saielsr88(&mut self) -> Saielsr88W<IcusariSpec> {
        Saielsr88W::new(self, 24)
    }
    ///Bit 25 - Security attributes of registers for IELSR95 to IELSR64
    #[inline(always)]
    pub fn saielsr89(&mut self) -> Saielsr89W<IcusariSpec> {
        Saielsr89W::new(self, 25)
    }
    ///Bit 26 - Security attributes of registers for IELSR95 to IELSR64
    #[inline(always)]
    pub fn saielsr90(&mut self) -> Saielsr90W<IcusariSpec> {
        Saielsr90W::new(self, 26)
    }
    ///Bit 27 - Security attributes of registers for IELSR95 to IELSR64
    #[inline(always)]
    pub fn saielsr91(&mut self) -> Saielsr91W<IcusariSpec> {
        Saielsr91W::new(self, 27)
    }
    ///Bit 28 - Security attributes of registers for IELSR95 to IELSR64
    #[inline(always)]
    pub fn saielsr92(&mut self) -> Saielsr92W<IcusariSpec> {
        Saielsr92W::new(self, 28)
    }
    ///Bit 29 - Security attributes of registers for IELSR95 to IELSR64
    #[inline(always)]
    pub fn saielsr93(&mut self) -> Saielsr93W<IcusariSpec> {
        Saielsr93W::new(self, 29)
    }
    ///Bit 30 - Security attributes of registers for IELSR95 to IELSR64
    #[inline(always)]
    pub fn saielsr94(&mut self) -> Saielsr94W<IcusariSpec> {
        Saielsr94W::new(self, 30)
    }
    ///Bit 31 - Security attributes of registers for IELSR95 to IELSR64
    #[inline(always)]
    pub fn saielsr95(&mut self) -> Saielsr95W<IcusariSpec> {
        Saielsr95W::new(self, 31)
    }
}
/**Interrupt Controller Unit Security Attribution Register I

You can [`read`](crate::Reg::read) this register and get [`icusari::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icusari::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IcusariSpec;
impl crate::RegisterSpec for IcusariSpec {
    type Ux = u32;
}
///`read()` method returns [`icusari::R`](R) reader structure
impl crate::Readable for IcusariSpec {}
///`write(|w| ..)` method takes [`icusari::W`](W) writer structure
impl crate::Writable for IcusariSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICUSARI to value 0xffff_ffff
impl crate::Resettable for IcusariSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
