///Register `ICUSARH` reader
pub type R = crate::R<IcusarhSpec>;
///Register `ICUSARH` writer
pub type W = crate::W<IcusarhSpec>;
/**Security attributes of registers for IELSR63 to IELSR32

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr32 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr32> for bool {
    #[inline(always)]
    fn from(variant: Saielsr32) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR32` reader - Security attributes of registers for IELSR63 to IELSR32
pub type Saielsr32R = crate::BitReader<Saielsr32>;
impl Saielsr32R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr32 {
        match self.bits {
            false => Saielsr32::_0,
            true => Saielsr32::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr32::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr32::_1
    }
}
///Field `SAIELSR32` writer - Security attributes of registers for IELSR63 to IELSR32
pub type Saielsr32W<'a, REG> = crate::BitWriter<'a, REG, Saielsr32>;
impl<'a, REG> Saielsr32W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr32::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr32::_1)
    }
}
/**Security attributes of registers for IELSR63 to IELSR32

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr33 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr33> for bool {
    #[inline(always)]
    fn from(variant: Saielsr33) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR33` reader - Security attributes of registers for IELSR63 to IELSR32
pub type Saielsr33R = crate::BitReader<Saielsr33>;
impl Saielsr33R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr33 {
        match self.bits {
            false => Saielsr33::_0,
            true => Saielsr33::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr33::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr33::_1
    }
}
///Field `SAIELSR33` writer - Security attributes of registers for IELSR63 to IELSR32
pub type Saielsr33W<'a, REG> = crate::BitWriter<'a, REG, Saielsr33>;
impl<'a, REG> Saielsr33W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr33::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr33::_1)
    }
}
/**Security attributes of registers for IELSR63 to IELSR32

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr34 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr34> for bool {
    #[inline(always)]
    fn from(variant: Saielsr34) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR34` reader - Security attributes of registers for IELSR63 to IELSR32
pub type Saielsr34R = crate::BitReader<Saielsr34>;
impl Saielsr34R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr34 {
        match self.bits {
            false => Saielsr34::_0,
            true => Saielsr34::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr34::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr34::_1
    }
}
///Field `SAIELSR34` writer - Security attributes of registers for IELSR63 to IELSR32
pub type Saielsr34W<'a, REG> = crate::BitWriter<'a, REG, Saielsr34>;
impl<'a, REG> Saielsr34W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr34::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr34::_1)
    }
}
/**Security attributes of registers for IELSR63 to IELSR32

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr35 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr35> for bool {
    #[inline(always)]
    fn from(variant: Saielsr35) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR35` reader - Security attributes of registers for IELSR63 to IELSR32
pub type Saielsr35R = crate::BitReader<Saielsr35>;
impl Saielsr35R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr35 {
        match self.bits {
            false => Saielsr35::_0,
            true => Saielsr35::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr35::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr35::_1
    }
}
///Field `SAIELSR35` writer - Security attributes of registers for IELSR63 to IELSR32
pub type Saielsr35W<'a, REG> = crate::BitWriter<'a, REG, Saielsr35>;
impl<'a, REG> Saielsr35W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr35::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr35::_1)
    }
}
/**Security attributes of registers for IELSR63 to IELSR32

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr36 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr36> for bool {
    #[inline(always)]
    fn from(variant: Saielsr36) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR36` reader - Security attributes of registers for IELSR63 to IELSR32
pub type Saielsr36R = crate::BitReader<Saielsr36>;
impl Saielsr36R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr36 {
        match self.bits {
            false => Saielsr36::_0,
            true => Saielsr36::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr36::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr36::_1
    }
}
///Field `SAIELSR36` writer - Security attributes of registers for IELSR63 to IELSR32
pub type Saielsr36W<'a, REG> = crate::BitWriter<'a, REG, Saielsr36>;
impl<'a, REG> Saielsr36W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr36::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr36::_1)
    }
}
/**Security attributes of registers for IELSR63 to IELSR32

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr37 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr37> for bool {
    #[inline(always)]
    fn from(variant: Saielsr37) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR37` reader - Security attributes of registers for IELSR63 to IELSR32
pub type Saielsr37R = crate::BitReader<Saielsr37>;
impl Saielsr37R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr37 {
        match self.bits {
            false => Saielsr37::_0,
            true => Saielsr37::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr37::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr37::_1
    }
}
///Field `SAIELSR37` writer - Security attributes of registers for IELSR63 to IELSR32
pub type Saielsr37W<'a, REG> = crate::BitWriter<'a, REG, Saielsr37>;
impl<'a, REG> Saielsr37W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr37::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr37::_1)
    }
}
/**Security attributes of registers for IELSR63 to IELSR32

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr38 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr38> for bool {
    #[inline(always)]
    fn from(variant: Saielsr38) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR38` reader - Security attributes of registers for IELSR63 to IELSR32
pub type Saielsr38R = crate::BitReader<Saielsr38>;
impl Saielsr38R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr38 {
        match self.bits {
            false => Saielsr38::_0,
            true => Saielsr38::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr38::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr38::_1
    }
}
///Field `SAIELSR38` writer - Security attributes of registers for IELSR63 to IELSR32
pub type Saielsr38W<'a, REG> = crate::BitWriter<'a, REG, Saielsr38>;
impl<'a, REG> Saielsr38W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr38::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr38::_1)
    }
}
/**Security attributes of registers for IELSR63 to IELSR32

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr39 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr39> for bool {
    #[inline(always)]
    fn from(variant: Saielsr39) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR39` reader - Security attributes of registers for IELSR63 to IELSR32
pub type Saielsr39R = crate::BitReader<Saielsr39>;
impl Saielsr39R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr39 {
        match self.bits {
            false => Saielsr39::_0,
            true => Saielsr39::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr39::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr39::_1
    }
}
///Field `SAIELSR39` writer - Security attributes of registers for IELSR63 to IELSR32
pub type Saielsr39W<'a, REG> = crate::BitWriter<'a, REG, Saielsr39>;
impl<'a, REG> Saielsr39W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr39::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr39::_1)
    }
}
/**Security attributes of registers for IELSR63 to IELSR32

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr40 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr40> for bool {
    #[inline(always)]
    fn from(variant: Saielsr40) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR40` reader - Security attributes of registers for IELSR63 to IELSR32
pub type Saielsr40R = crate::BitReader<Saielsr40>;
impl Saielsr40R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr40 {
        match self.bits {
            false => Saielsr40::_0,
            true => Saielsr40::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr40::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr40::_1
    }
}
///Field `SAIELSR40` writer - Security attributes of registers for IELSR63 to IELSR32
pub type Saielsr40W<'a, REG> = crate::BitWriter<'a, REG, Saielsr40>;
impl<'a, REG> Saielsr40W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr40::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr40::_1)
    }
}
/**Security attributes of registers for IELSR63 to IELSR32

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr41 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr41> for bool {
    #[inline(always)]
    fn from(variant: Saielsr41) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR41` reader - Security attributes of registers for IELSR63 to IELSR32
pub type Saielsr41R = crate::BitReader<Saielsr41>;
impl Saielsr41R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr41 {
        match self.bits {
            false => Saielsr41::_0,
            true => Saielsr41::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr41::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr41::_1
    }
}
///Field `SAIELSR41` writer - Security attributes of registers for IELSR63 to IELSR32
pub type Saielsr41W<'a, REG> = crate::BitWriter<'a, REG, Saielsr41>;
impl<'a, REG> Saielsr41W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr41::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr41::_1)
    }
}
/**Security attributes of registers for IELSR63 to IELSR32

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr42 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr42> for bool {
    #[inline(always)]
    fn from(variant: Saielsr42) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR42` reader - Security attributes of registers for IELSR63 to IELSR32
pub type Saielsr42R = crate::BitReader<Saielsr42>;
impl Saielsr42R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr42 {
        match self.bits {
            false => Saielsr42::_0,
            true => Saielsr42::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr42::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr42::_1
    }
}
///Field `SAIELSR42` writer - Security attributes of registers for IELSR63 to IELSR32
pub type Saielsr42W<'a, REG> = crate::BitWriter<'a, REG, Saielsr42>;
impl<'a, REG> Saielsr42W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr42::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr42::_1)
    }
}
/**Security attributes of registers for IELSR63 to IELSR32

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr43 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr43> for bool {
    #[inline(always)]
    fn from(variant: Saielsr43) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR43` reader - Security attributes of registers for IELSR63 to IELSR32
pub type Saielsr43R = crate::BitReader<Saielsr43>;
impl Saielsr43R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr43 {
        match self.bits {
            false => Saielsr43::_0,
            true => Saielsr43::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr43::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr43::_1
    }
}
///Field `SAIELSR43` writer - Security attributes of registers for IELSR63 to IELSR32
pub type Saielsr43W<'a, REG> = crate::BitWriter<'a, REG, Saielsr43>;
impl<'a, REG> Saielsr43W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr43::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr43::_1)
    }
}
/**Security attributes of registers for IELSR63 to IELSR32

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr44 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr44> for bool {
    #[inline(always)]
    fn from(variant: Saielsr44) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR44` reader - Security attributes of registers for IELSR63 to IELSR32
pub type Saielsr44R = crate::BitReader<Saielsr44>;
impl Saielsr44R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr44 {
        match self.bits {
            false => Saielsr44::_0,
            true => Saielsr44::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr44::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr44::_1
    }
}
///Field `SAIELSR44` writer - Security attributes of registers for IELSR63 to IELSR32
pub type Saielsr44W<'a, REG> = crate::BitWriter<'a, REG, Saielsr44>;
impl<'a, REG> Saielsr44W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr44::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr44::_1)
    }
}
/**Security attributes of registers for IELSR63 to IELSR32

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr45 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr45> for bool {
    #[inline(always)]
    fn from(variant: Saielsr45) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR45` reader - Security attributes of registers for IELSR63 to IELSR32
pub type Saielsr45R = crate::BitReader<Saielsr45>;
impl Saielsr45R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr45 {
        match self.bits {
            false => Saielsr45::_0,
            true => Saielsr45::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr45::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr45::_1
    }
}
///Field `SAIELSR45` writer - Security attributes of registers for IELSR63 to IELSR32
pub type Saielsr45W<'a, REG> = crate::BitWriter<'a, REG, Saielsr45>;
impl<'a, REG> Saielsr45W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr45::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr45::_1)
    }
}
/**Security attributes of registers for IELSR63 to IELSR32

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr46 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr46> for bool {
    #[inline(always)]
    fn from(variant: Saielsr46) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR46` reader - Security attributes of registers for IELSR63 to IELSR32
pub type Saielsr46R = crate::BitReader<Saielsr46>;
impl Saielsr46R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr46 {
        match self.bits {
            false => Saielsr46::_0,
            true => Saielsr46::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr46::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr46::_1
    }
}
///Field `SAIELSR46` writer - Security attributes of registers for IELSR63 to IELSR32
pub type Saielsr46W<'a, REG> = crate::BitWriter<'a, REG, Saielsr46>;
impl<'a, REG> Saielsr46W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr46::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr46::_1)
    }
}
/**Security attributes of registers for IELSR63 to IELSR32

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr47 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr47> for bool {
    #[inline(always)]
    fn from(variant: Saielsr47) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR47` reader - Security attributes of registers for IELSR63 to IELSR32
pub type Saielsr47R = crate::BitReader<Saielsr47>;
impl Saielsr47R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr47 {
        match self.bits {
            false => Saielsr47::_0,
            true => Saielsr47::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr47::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr47::_1
    }
}
///Field `SAIELSR47` writer - Security attributes of registers for IELSR63 to IELSR32
pub type Saielsr47W<'a, REG> = crate::BitWriter<'a, REG, Saielsr47>;
impl<'a, REG> Saielsr47W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr47::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr47::_1)
    }
}
/**Security attributes of registers for IELSR63 to IELSR32

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr48 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr48> for bool {
    #[inline(always)]
    fn from(variant: Saielsr48) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR48` reader - Security attributes of registers for IELSR63 to IELSR32
pub type Saielsr48R = crate::BitReader<Saielsr48>;
impl Saielsr48R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr48 {
        match self.bits {
            false => Saielsr48::_0,
            true => Saielsr48::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr48::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr48::_1
    }
}
///Field `SAIELSR48` writer - Security attributes of registers for IELSR63 to IELSR32
pub type Saielsr48W<'a, REG> = crate::BitWriter<'a, REG, Saielsr48>;
impl<'a, REG> Saielsr48W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr48::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr48::_1)
    }
}
/**Security attributes of registers for IELSR63 to IELSR32

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr49 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr49> for bool {
    #[inline(always)]
    fn from(variant: Saielsr49) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR49` reader - Security attributes of registers for IELSR63 to IELSR32
pub type Saielsr49R = crate::BitReader<Saielsr49>;
impl Saielsr49R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr49 {
        match self.bits {
            false => Saielsr49::_0,
            true => Saielsr49::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr49::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr49::_1
    }
}
///Field `SAIELSR49` writer - Security attributes of registers for IELSR63 to IELSR32
pub type Saielsr49W<'a, REG> = crate::BitWriter<'a, REG, Saielsr49>;
impl<'a, REG> Saielsr49W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr49::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr49::_1)
    }
}
/**Security attributes of registers for IELSR63 to IELSR32

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr50 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr50> for bool {
    #[inline(always)]
    fn from(variant: Saielsr50) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR50` reader - Security attributes of registers for IELSR63 to IELSR32
pub type Saielsr50R = crate::BitReader<Saielsr50>;
impl Saielsr50R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr50 {
        match self.bits {
            false => Saielsr50::_0,
            true => Saielsr50::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr50::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr50::_1
    }
}
///Field `SAIELSR50` writer - Security attributes of registers for IELSR63 to IELSR32
pub type Saielsr50W<'a, REG> = crate::BitWriter<'a, REG, Saielsr50>;
impl<'a, REG> Saielsr50W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr50::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr50::_1)
    }
}
/**Security attributes of registers for IELSR63 to IELSR32

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr51 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr51> for bool {
    #[inline(always)]
    fn from(variant: Saielsr51) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR51` reader - Security attributes of registers for IELSR63 to IELSR32
pub type Saielsr51R = crate::BitReader<Saielsr51>;
impl Saielsr51R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr51 {
        match self.bits {
            false => Saielsr51::_0,
            true => Saielsr51::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr51::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr51::_1
    }
}
///Field `SAIELSR51` writer - Security attributes of registers for IELSR63 to IELSR32
pub type Saielsr51W<'a, REG> = crate::BitWriter<'a, REG, Saielsr51>;
impl<'a, REG> Saielsr51W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr51::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr51::_1)
    }
}
/**Security attributes of registers for IELSR63 to IELSR32

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr52 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr52> for bool {
    #[inline(always)]
    fn from(variant: Saielsr52) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR52` reader - Security attributes of registers for IELSR63 to IELSR32
pub type Saielsr52R = crate::BitReader<Saielsr52>;
impl Saielsr52R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr52 {
        match self.bits {
            false => Saielsr52::_0,
            true => Saielsr52::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr52::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr52::_1
    }
}
///Field `SAIELSR52` writer - Security attributes of registers for IELSR63 to IELSR32
pub type Saielsr52W<'a, REG> = crate::BitWriter<'a, REG, Saielsr52>;
impl<'a, REG> Saielsr52W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr52::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr52::_1)
    }
}
/**Security attributes of registers for IELSR63 to IELSR32

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr53 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr53> for bool {
    #[inline(always)]
    fn from(variant: Saielsr53) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR53` reader - Security attributes of registers for IELSR63 to IELSR32
pub type Saielsr53R = crate::BitReader<Saielsr53>;
impl Saielsr53R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr53 {
        match self.bits {
            false => Saielsr53::_0,
            true => Saielsr53::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr53::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr53::_1
    }
}
///Field `SAIELSR53` writer - Security attributes of registers for IELSR63 to IELSR32
pub type Saielsr53W<'a, REG> = crate::BitWriter<'a, REG, Saielsr53>;
impl<'a, REG> Saielsr53W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr53::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr53::_1)
    }
}
/**Security attributes of registers for IELSR63 to IELSR32

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr54 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr54> for bool {
    #[inline(always)]
    fn from(variant: Saielsr54) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR54` reader - Security attributes of registers for IELSR63 to IELSR32
pub type Saielsr54R = crate::BitReader<Saielsr54>;
impl Saielsr54R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr54 {
        match self.bits {
            false => Saielsr54::_0,
            true => Saielsr54::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr54::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr54::_1
    }
}
///Field `SAIELSR54` writer - Security attributes of registers for IELSR63 to IELSR32
pub type Saielsr54W<'a, REG> = crate::BitWriter<'a, REG, Saielsr54>;
impl<'a, REG> Saielsr54W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr54::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr54::_1)
    }
}
/**Security attributes of registers for IELSR63 to IELSR32

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr55 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr55> for bool {
    #[inline(always)]
    fn from(variant: Saielsr55) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR55` reader - Security attributes of registers for IELSR63 to IELSR32
pub type Saielsr55R = crate::BitReader<Saielsr55>;
impl Saielsr55R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr55 {
        match self.bits {
            false => Saielsr55::_0,
            true => Saielsr55::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr55::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr55::_1
    }
}
///Field `SAIELSR55` writer - Security attributes of registers for IELSR63 to IELSR32
pub type Saielsr55W<'a, REG> = crate::BitWriter<'a, REG, Saielsr55>;
impl<'a, REG> Saielsr55W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr55::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr55::_1)
    }
}
/**Security attributes of registers for IELSR63 to IELSR32

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr56 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr56> for bool {
    #[inline(always)]
    fn from(variant: Saielsr56) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR56` reader - Security attributes of registers for IELSR63 to IELSR32
pub type Saielsr56R = crate::BitReader<Saielsr56>;
impl Saielsr56R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr56 {
        match self.bits {
            false => Saielsr56::_0,
            true => Saielsr56::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr56::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr56::_1
    }
}
///Field `SAIELSR56` writer - Security attributes of registers for IELSR63 to IELSR32
pub type Saielsr56W<'a, REG> = crate::BitWriter<'a, REG, Saielsr56>;
impl<'a, REG> Saielsr56W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr56::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr56::_1)
    }
}
/**Security attributes of registers for IELSR63 to IELSR32

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr57 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr57> for bool {
    #[inline(always)]
    fn from(variant: Saielsr57) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR57` reader - Security attributes of registers for IELSR63 to IELSR32
pub type Saielsr57R = crate::BitReader<Saielsr57>;
impl Saielsr57R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr57 {
        match self.bits {
            false => Saielsr57::_0,
            true => Saielsr57::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr57::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr57::_1
    }
}
///Field `SAIELSR57` writer - Security attributes of registers for IELSR63 to IELSR32
pub type Saielsr57W<'a, REG> = crate::BitWriter<'a, REG, Saielsr57>;
impl<'a, REG> Saielsr57W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr57::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr57::_1)
    }
}
/**Security attributes of registers for IELSR63 to IELSR32

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr58 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr58> for bool {
    #[inline(always)]
    fn from(variant: Saielsr58) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR58` reader - Security attributes of registers for IELSR63 to IELSR32
pub type Saielsr58R = crate::BitReader<Saielsr58>;
impl Saielsr58R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr58 {
        match self.bits {
            false => Saielsr58::_0,
            true => Saielsr58::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr58::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr58::_1
    }
}
///Field `SAIELSR58` writer - Security attributes of registers for IELSR63 to IELSR32
pub type Saielsr58W<'a, REG> = crate::BitWriter<'a, REG, Saielsr58>;
impl<'a, REG> Saielsr58W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr58::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr58::_1)
    }
}
/**Security attributes of registers for IELSR63 to IELSR32

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr59 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr59> for bool {
    #[inline(always)]
    fn from(variant: Saielsr59) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR59` reader - Security attributes of registers for IELSR63 to IELSR32
pub type Saielsr59R = crate::BitReader<Saielsr59>;
impl Saielsr59R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr59 {
        match self.bits {
            false => Saielsr59::_0,
            true => Saielsr59::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr59::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr59::_1
    }
}
///Field `SAIELSR59` writer - Security attributes of registers for IELSR63 to IELSR32
pub type Saielsr59W<'a, REG> = crate::BitWriter<'a, REG, Saielsr59>;
impl<'a, REG> Saielsr59W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr59::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr59::_1)
    }
}
/**Security attributes of registers for IELSR63 to IELSR32

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr60 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr60> for bool {
    #[inline(always)]
    fn from(variant: Saielsr60) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR60` reader - Security attributes of registers for IELSR63 to IELSR32
pub type Saielsr60R = crate::BitReader<Saielsr60>;
impl Saielsr60R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr60 {
        match self.bits {
            false => Saielsr60::_0,
            true => Saielsr60::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr60::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr60::_1
    }
}
///Field `SAIELSR60` writer - Security attributes of registers for IELSR63 to IELSR32
pub type Saielsr60W<'a, REG> = crate::BitWriter<'a, REG, Saielsr60>;
impl<'a, REG> Saielsr60W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr60::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr60::_1)
    }
}
/**Security attributes of registers for IELSR63 to IELSR32

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr61 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr61> for bool {
    #[inline(always)]
    fn from(variant: Saielsr61) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR61` reader - Security attributes of registers for IELSR63 to IELSR32
pub type Saielsr61R = crate::BitReader<Saielsr61>;
impl Saielsr61R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr61 {
        match self.bits {
            false => Saielsr61::_0,
            true => Saielsr61::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr61::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr61::_1
    }
}
///Field `SAIELSR61` writer - Security attributes of registers for IELSR63 to IELSR32
pub type Saielsr61W<'a, REG> = crate::BitWriter<'a, REG, Saielsr61>;
impl<'a, REG> Saielsr61W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr61::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr61::_1)
    }
}
/**Security attributes of registers for IELSR63 to IELSR32

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr62 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr62> for bool {
    #[inline(always)]
    fn from(variant: Saielsr62) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR62` reader - Security attributes of registers for IELSR63 to IELSR32
pub type Saielsr62R = crate::BitReader<Saielsr62>;
impl Saielsr62R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr62 {
        match self.bits {
            false => Saielsr62::_0,
            true => Saielsr62::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr62::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr62::_1
    }
}
///Field `SAIELSR62` writer - Security attributes of registers for IELSR63 to IELSR32
pub type Saielsr62W<'a, REG> = crate::BitWriter<'a, REG, Saielsr62>;
impl<'a, REG> Saielsr62W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr62::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr62::_1)
    }
}
/**Security attributes of registers for IELSR63 to IELSR32

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saielsr63 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saielsr63> for bool {
    #[inline(always)]
    fn from(variant: Saielsr63) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIELSR63` reader - Security attributes of registers for IELSR63 to IELSR32
pub type Saielsr63R = crate::BitReader<Saielsr63>;
impl Saielsr63R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saielsr63 {
        match self.bits {
            false => Saielsr63::_0,
            true => Saielsr63::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saielsr63::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saielsr63::_1
    }
}
///Field `SAIELSR63` writer - Security attributes of registers for IELSR63 to IELSR32
pub type Saielsr63W<'a, REG> = crate::BitWriter<'a, REG, Saielsr63>;
impl<'a, REG> Saielsr63W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr63::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saielsr63::_1)
    }
}
impl R {
    ///Bit 0 - Security attributes of registers for IELSR63 to IELSR32
    #[inline(always)]
    pub fn saielsr32(&self) -> Saielsr32R {
        Saielsr32R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Security attributes of registers for IELSR63 to IELSR32
    #[inline(always)]
    pub fn saielsr33(&self) -> Saielsr33R {
        Saielsr33R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Security attributes of registers for IELSR63 to IELSR32
    #[inline(always)]
    pub fn saielsr34(&self) -> Saielsr34R {
        Saielsr34R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Security attributes of registers for IELSR63 to IELSR32
    #[inline(always)]
    pub fn saielsr35(&self) -> Saielsr35R {
        Saielsr35R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Security attributes of registers for IELSR63 to IELSR32
    #[inline(always)]
    pub fn saielsr36(&self) -> Saielsr36R {
        Saielsr36R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Security attributes of registers for IELSR63 to IELSR32
    #[inline(always)]
    pub fn saielsr37(&self) -> Saielsr37R {
        Saielsr37R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Security attributes of registers for IELSR63 to IELSR32
    #[inline(always)]
    pub fn saielsr38(&self) -> Saielsr38R {
        Saielsr38R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Security attributes of registers for IELSR63 to IELSR32
    #[inline(always)]
    pub fn saielsr39(&self) -> Saielsr39R {
        Saielsr39R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Security attributes of registers for IELSR63 to IELSR32
    #[inline(always)]
    pub fn saielsr40(&self) -> Saielsr40R {
        Saielsr40R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Security attributes of registers for IELSR63 to IELSR32
    #[inline(always)]
    pub fn saielsr41(&self) -> Saielsr41R {
        Saielsr41R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Security attributes of registers for IELSR63 to IELSR32
    #[inline(always)]
    pub fn saielsr42(&self) -> Saielsr42R {
        Saielsr42R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Security attributes of registers for IELSR63 to IELSR32
    #[inline(always)]
    pub fn saielsr43(&self) -> Saielsr43R {
        Saielsr43R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Security attributes of registers for IELSR63 to IELSR32
    #[inline(always)]
    pub fn saielsr44(&self) -> Saielsr44R {
        Saielsr44R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Security attributes of registers for IELSR63 to IELSR32
    #[inline(always)]
    pub fn saielsr45(&self) -> Saielsr45R {
        Saielsr45R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Security attributes of registers for IELSR63 to IELSR32
    #[inline(always)]
    pub fn saielsr46(&self) -> Saielsr46R {
        Saielsr46R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Security attributes of registers for IELSR63 to IELSR32
    #[inline(always)]
    pub fn saielsr47(&self) -> Saielsr47R {
        Saielsr47R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Security attributes of registers for IELSR63 to IELSR32
    #[inline(always)]
    pub fn saielsr48(&self) -> Saielsr48R {
        Saielsr48R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Security attributes of registers for IELSR63 to IELSR32
    #[inline(always)]
    pub fn saielsr49(&self) -> Saielsr49R {
        Saielsr49R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Security attributes of registers for IELSR63 to IELSR32
    #[inline(always)]
    pub fn saielsr50(&self) -> Saielsr50R {
        Saielsr50R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Security attributes of registers for IELSR63 to IELSR32
    #[inline(always)]
    pub fn saielsr51(&self) -> Saielsr51R {
        Saielsr51R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Security attributes of registers for IELSR63 to IELSR32
    #[inline(always)]
    pub fn saielsr52(&self) -> Saielsr52R {
        Saielsr52R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Security attributes of registers for IELSR63 to IELSR32
    #[inline(always)]
    pub fn saielsr53(&self) -> Saielsr53R {
        Saielsr53R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Security attributes of registers for IELSR63 to IELSR32
    #[inline(always)]
    pub fn saielsr54(&self) -> Saielsr54R {
        Saielsr54R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Security attributes of registers for IELSR63 to IELSR32
    #[inline(always)]
    pub fn saielsr55(&self) -> Saielsr55R {
        Saielsr55R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Security attributes of registers for IELSR63 to IELSR32
    #[inline(always)]
    pub fn saielsr56(&self) -> Saielsr56R {
        Saielsr56R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Security attributes of registers for IELSR63 to IELSR32
    #[inline(always)]
    pub fn saielsr57(&self) -> Saielsr57R {
        Saielsr57R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Security attributes of registers for IELSR63 to IELSR32
    #[inline(always)]
    pub fn saielsr58(&self) -> Saielsr58R {
        Saielsr58R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Security attributes of registers for IELSR63 to IELSR32
    #[inline(always)]
    pub fn saielsr59(&self) -> Saielsr59R {
        Saielsr59R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Security attributes of registers for IELSR63 to IELSR32
    #[inline(always)]
    pub fn saielsr60(&self) -> Saielsr60R {
        Saielsr60R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Security attributes of registers for IELSR63 to IELSR32
    #[inline(always)]
    pub fn saielsr61(&self) -> Saielsr61R {
        Saielsr61R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Security attributes of registers for IELSR63 to IELSR32
    #[inline(always)]
    pub fn saielsr62(&self) -> Saielsr62R {
        Saielsr62R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Security attributes of registers for IELSR63 to IELSR32
    #[inline(always)]
    pub fn saielsr63(&self) -> Saielsr63R {
        Saielsr63R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICUSARH")
            .field("saielsr32", &self.saielsr32())
            .field("saielsr33", &self.saielsr33())
            .field("saielsr34", &self.saielsr34())
            .field("saielsr35", &self.saielsr35())
            .field("saielsr36", &self.saielsr36())
            .field("saielsr37", &self.saielsr37())
            .field("saielsr38", &self.saielsr38())
            .field("saielsr39", &self.saielsr39())
            .field("saielsr40", &self.saielsr40())
            .field("saielsr41", &self.saielsr41())
            .field("saielsr42", &self.saielsr42())
            .field("saielsr43", &self.saielsr43())
            .field("saielsr44", &self.saielsr44())
            .field("saielsr45", &self.saielsr45())
            .field("saielsr46", &self.saielsr46())
            .field("saielsr47", &self.saielsr47())
            .field("saielsr48", &self.saielsr48())
            .field("saielsr49", &self.saielsr49())
            .field("saielsr50", &self.saielsr50())
            .field("saielsr51", &self.saielsr51())
            .field("saielsr52", &self.saielsr52())
            .field("saielsr53", &self.saielsr53())
            .field("saielsr54", &self.saielsr54())
            .field("saielsr55", &self.saielsr55())
            .field("saielsr56", &self.saielsr56())
            .field("saielsr57", &self.saielsr57())
            .field("saielsr58", &self.saielsr58())
            .field("saielsr59", &self.saielsr59())
            .field("saielsr60", &self.saielsr60())
            .field("saielsr61", &self.saielsr61())
            .field("saielsr62", &self.saielsr62())
            .field("saielsr63", &self.saielsr63())
            .finish()
    }
}
impl W {
    ///Bit 0 - Security attributes of registers for IELSR63 to IELSR32
    #[inline(always)]
    pub fn saielsr32(&mut self) -> Saielsr32W<IcusarhSpec> {
        Saielsr32W::new(self, 0)
    }
    ///Bit 1 - Security attributes of registers for IELSR63 to IELSR32
    #[inline(always)]
    pub fn saielsr33(&mut self) -> Saielsr33W<IcusarhSpec> {
        Saielsr33W::new(self, 1)
    }
    ///Bit 2 - Security attributes of registers for IELSR63 to IELSR32
    #[inline(always)]
    pub fn saielsr34(&mut self) -> Saielsr34W<IcusarhSpec> {
        Saielsr34W::new(self, 2)
    }
    ///Bit 3 - Security attributes of registers for IELSR63 to IELSR32
    #[inline(always)]
    pub fn saielsr35(&mut self) -> Saielsr35W<IcusarhSpec> {
        Saielsr35W::new(self, 3)
    }
    ///Bit 4 - Security attributes of registers for IELSR63 to IELSR32
    #[inline(always)]
    pub fn saielsr36(&mut self) -> Saielsr36W<IcusarhSpec> {
        Saielsr36W::new(self, 4)
    }
    ///Bit 5 - Security attributes of registers for IELSR63 to IELSR32
    #[inline(always)]
    pub fn saielsr37(&mut self) -> Saielsr37W<IcusarhSpec> {
        Saielsr37W::new(self, 5)
    }
    ///Bit 6 - Security attributes of registers for IELSR63 to IELSR32
    #[inline(always)]
    pub fn saielsr38(&mut self) -> Saielsr38W<IcusarhSpec> {
        Saielsr38W::new(self, 6)
    }
    ///Bit 7 - Security attributes of registers for IELSR63 to IELSR32
    #[inline(always)]
    pub fn saielsr39(&mut self) -> Saielsr39W<IcusarhSpec> {
        Saielsr39W::new(self, 7)
    }
    ///Bit 8 - Security attributes of registers for IELSR63 to IELSR32
    #[inline(always)]
    pub fn saielsr40(&mut self) -> Saielsr40W<IcusarhSpec> {
        Saielsr40W::new(self, 8)
    }
    ///Bit 9 - Security attributes of registers for IELSR63 to IELSR32
    #[inline(always)]
    pub fn saielsr41(&mut self) -> Saielsr41W<IcusarhSpec> {
        Saielsr41W::new(self, 9)
    }
    ///Bit 10 - Security attributes of registers for IELSR63 to IELSR32
    #[inline(always)]
    pub fn saielsr42(&mut self) -> Saielsr42W<IcusarhSpec> {
        Saielsr42W::new(self, 10)
    }
    ///Bit 11 - Security attributes of registers for IELSR63 to IELSR32
    #[inline(always)]
    pub fn saielsr43(&mut self) -> Saielsr43W<IcusarhSpec> {
        Saielsr43W::new(self, 11)
    }
    ///Bit 12 - Security attributes of registers for IELSR63 to IELSR32
    #[inline(always)]
    pub fn saielsr44(&mut self) -> Saielsr44W<IcusarhSpec> {
        Saielsr44W::new(self, 12)
    }
    ///Bit 13 - Security attributes of registers for IELSR63 to IELSR32
    #[inline(always)]
    pub fn saielsr45(&mut self) -> Saielsr45W<IcusarhSpec> {
        Saielsr45W::new(self, 13)
    }
    ///Bit 14 - Security attributes of registers for IELSR63 to IELSR32
    #[inline(always)]
    pub fn saielsr46(&mut self) -> Saielsr46W<IcusarhSpec> {
        Saielsr46W::new(self, 14)
    }
    ///Bit 15 - Security attributes of registers for IELSR63 to IELSR32
    #[inline(always)]
    pub fn saielsr47(&mut self) -> Saielsr47W<IcusarhSpec> {
        Saielsr47W::new(self, 15)
    }
    ///Bit 16 - Security attributes of registers for IELSR63 to IELSR32
    #[inline(always)]
    pub fn saielsr48(&mut self) -> Saielsr48W<IcusarhSpec> {
        Saielsr48W::new(self, 16)
    }
    ///Bit 17 - Security attributes of registers for IELSR63 to IELSR32
    #[inline(always)]
    pub fn saielsr49(&mut self) -> Saielsr49W<IcusarhSpec> {
        Saielsr49W::new(self, 17)
    }
    ///Bit 18 - Security attributes of registers for IELSR63 to IELSR32
    #[inline(always)]
    pub fn saielsr50(&mut self) -> Saielsr50W<IcusarhSpec> {
        Saielsr50W::new(self, 18)
    }
    ///Bit 19 - Security attributes of registers for IELSR63 to IELSR32
    #[inline(always)]
    pub fn saielsr51(&mut self) -> Saielsr51W<IcusarhSpec> {
        Saielsr51W::new(self, 19)
    }
    ///Bit 20 - Security attributes of registers for IELSR63 to IELSR32
    #[inline(always)]
    pub fn saielsr52(&mut self) -> Saielsr52W<IcusarhSpec> {
        Saielsr52W::new(self, 20)
    }
    ///Bit 21 - Security attributes of registers for IELSR63 to IELSR32
    #[inline(always)]
    pub fn saielsr53(&mut self) -> Saielsr53W<IcusarhSpec> {
        Saielsr53W::new(self, 21)
    }
    ///Bit 22 - Security attributes of registers for IELSR63 to IELSR32
    #[inline(always)]
    pub fn saielsr54(&mut self) -> Saielsr54W<IcusarhSpec> {
        Saielsr54W::new(self, 22)
    }
    ///Bit 23 - Security attributes of registers for IELSR63 to IELSR32
    #[inline(always)]
    pub fn saielsr55(&mut self) -> Saielsr55W<IcusarhSpec> {
        Saielsr55W::new(self, 23)
    }
    ///Bit 24 - Security attributes of registers for IELSR63 to IELSR32
    #[inline(always)]
    pub fn saielsr56(&mut self) -> Saielsr56W<IcusarhSpec> {
        Saielsr56W::new(self, 24)
    }
    ///Bit 25 - Security attributes of registers for IELSR63 to IELSR32
    #[inline(always)]
    pub fn saielsr57(&mut self) -> Saielsr57W<IcusarhSpec> {
        Saielsr57W::new(self, 25)
    }
    ///Bit 26 - Security attributes of registers for IELSR63 to IELSR32
    #[inline(always)]
    pub fn saielsr58(&mut self) -> Saielsr58W<IcusarhSpec> {
        Saielsr58W::new(self, 26)
    }
    ///Bit 27 - Security attributes of registers for IELSR63 to IELSR32
    #[inline(always)]
    pub fn saielsr59(&mut self) -> Saielsr59W<IcusarhSpec> {
        Saielsr59W::new(self, 27)
    }
    ///Bit 28 - Security attributes of registers for IELSR63 to IELSR32
    #[inline(always)]
    pub fn saielsr60(&mut self) -> Saielsr60W<IcusarhSpec> {
        Saielsr60W::new(self, 28)
    }
    ///Bit 29 - Security attributes of registers for IELSR63 to IELSR32
    #[inline(always)]
    pub fn saielsr61(&mut self) -> Saielsr61W<IcusarhSpec> {
        Saielsr61W::new(self, 29)
    }
    ///Bit 30 - Security attributes of registers for IELSR63 to IELSR32
    #[inline(always)]
    pub fn saielsr62(&mut self) -> Saielsr62W<IcusarhSpec> {
        Saielsr62W::new(self, 30)
    }
    ///Bit 31 - Security attributes of registers for IELSR63 to IELSR32
    #[inline(always)]
    pub fn saielsr63(&mut self) -> Saielsr63W<IcusarhSpec> {
        Saielsr63W::new(self, 31)
    }
}
/**Interrupt Controller Unit Security Attribution Register H

You can [`read`](crate::Reg::read) this register and get [`icusarh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icusarh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IcusarhSpec;
impl crate::RegisterSpec for IcusarhSpec {
    type Ux = u32;
}
///`read()` method returns [`icusarh::R`](R) reader structure
impl crate::Readable for IcusarhSpec {}
///`write(|w| ..)` method takes [`icusarh::W`](W) writer structure
impl crate::Writable for IcusarhSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICUSARH to value 0xffff_ffff
impl crate::Resettable for IcusarhSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
