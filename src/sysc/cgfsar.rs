///Register `CGFSAR` reader
pub type R = crate::R<CgfsarSpec>;
///Register `CGFSAR` writer
pub type W = crate::W<CgfsarSpec>;
/**Non Secure Attribute bit 00

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nonsec00 {
    ///0: Secure
    _0 = 0,
    ///1: Non Secure
    _1 = 1,
}
impl From<Nonsec00> for bool {
    #[inline(always)]
    fn from(variant: Nonsec00) -> Self {
        variant as u8 != 0
    }
}
///Field `NONSEC00` reader - Non Secure Attribute bit 00
pub type Nonsec00R = crate::BitReader<Nonsec00>;
impl Nonsec00R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Nonsec00 {
        match self.bits {
            false => Nonsec00::_0,
            true => Nonsec00::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Nonsec00::_0
    }
    ///Non Secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Nonsec00::_1
    }
}
///Field `NONSEC00` writer - Non Secure Attribute bit 00
pub type Nonsec00W<'a, REG> = crate::BitWriter<'a, REG, Nonsec00>;
impl<'a, REG> Nonsec00W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Nonsec00::_0)
    }
    ///Non Secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Nonsec00::_1)
    }
}
/**Non Secure Attribute bit 02

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nonsec02 {
    ///0: Secure
    _0 = 0,
    ///1: Non Secure
    _1 = 1,
}
impl From<Nonsec02> for bool {
    #[inline(always)]
    fn from(variant: Nonsec02) -> Self {
        variant as u8 != 0
    }
}
///Field `NONSEC02` reader - Non Secure Attribute bit 02
pub type Nonsec02R = crate::BitReader<Nonsec02>;
impl Nonsec02R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Nonsec02 {
        match self.bits {
            false => Nonsec02::_0,
            true => Nonsec02::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Nonsec02::_0
    }
    ///Non Secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Nonsec02::_1
    }
}
///Field `NONSEC02` writer - Non Secure Attribute bit 02
pub type Nonsec02W<'a, REG> = crate::BitWriter<'a, REG, Nonsec02>;
impl<'a, REG> Nonsec02W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Nonsec02::_0)
    }
    ///Non Secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Nonsec02::_1)
    }
}
/**Non Secure Attribute bit 03

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nonsec03 {
    ///0: Secure
    _0 = 0,
    ///1: Non Secure
    _1 = 1,
}
impl From<Nonsec03> for bool {
    #[inline(always)]
    fn from(variant: Nonsec03) -> Self {
        variant as u8 != 0
    }
}
///Field `NONSEC03` reader - Non Secure Attribute bit 03
pub type Nonsec03R = crate::BitReader<Nonsec03>;
impl Nonsec03R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Nonsec03 {
        match self.bits {
            false => Nonsec03::_0,
            true => Nonsec03::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Nonsec03::_0
    }
    ///Non Secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Nonsec03::_1
    }
}
///Field `NONSEC03` writer - Non Secure Attribute bit 03
pub type Nonsec03W<'a, REG> = crate::BitWriter<'a, REG, Nonsec03>;
impl<'a, REG> Nonsec03W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Nonsec03::_0)
    }
    ///Non Secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Nonsec03::_1)
    }
}
/**Non Secure Attribute bit 04

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nonsec04 {
    ///0: Secure
    _0 = 0,
    ///1: Non Secure
    _1 = 1,
}
impl From<Nonsec04> for bool {
    #[inline(always)]
    fn from(variant: Nonsec04) -> Self {
        variant as u8 != 0
    }
}
///Field `NONSEC04` reader - Non Secure Attribute bit 04
pub type Nonsec04R = crate::BitReader<Nonsec04>;
impl Nonsec04R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Nonsec04 {
        match self.bits {
            false => Nonsec04::_0,
            true => Nonsec04::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Nonsec04::_0
    }
    ///Non Secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Nonsec04::_1
    }
}
///Field `NONSEC04` writer - Non Secure Attribute bit 04
pub type Nonsec04W<'a, REG> = crate::BitWriter<'a, REG, Nonsec04>;
impl<'a, REG> Nonsec04W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Nonsec04::_0)
    }
    ///Non Secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Nonsec04::_1)
    }
}
/**Non Secure Attribute bit 05

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nonsec05 {
    ///0: Secure
    _0 = 0,
    ///1: Non Secure
    _1 = 1,
}
impl From<Nonsec05> for bool {
    #[inline(always)]
    fn from(variant: Nonsec05) -> Self {
        variant as u8 != 0
    }
}
///Field `NONSEC05` reader - Non Secure Attribute bit 05
pub type Nonsec05R = crate::BitReader<Nonsec05>;
impl Nonsec05R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Nonsec05 {
        match self.bits {
            false => Nonsec05::_0,
            true => Nonsec05::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Nonsec05::_0
    }
    ///Non Secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Nonsec05::_1
    }
}
///Field `NONSEC05` writer - Non Secure Attribute bit 05
pub type Nonsec05W<'a, REG> = crate::BitWriter<'a, REG, Nonsec05>;
impl<'a, REG> Nonsec05W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Nonsec05::_0)
    }
    ///Non Secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Nonsec05::_1)
    }
}
/**Non Secure Attribute bit 06

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nonsec06 {
    ///0: Secure
    _0 = 0,
    ///1: Non Secure
    _1 = 1,
}
impl From<Nonsec06> for bool {
    #[inline(always)]
    fn from(variant: Nonsec06) -> Self {
        variant as u8 != 0
    }
}
///Field `NONSEC06` reader - Non Secure Attribute bit 06
pub type Nonsec06R = crate::BitReader<Nonsec06>;
impl Nonsec06R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Nonsec06 {
        match self.bits {
            false => Nonsec06::_0,
            true => Nonsec06::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Nonsec06::_0
    }
    ///Non Secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Nonsec06::_1
    }
}
///Field `NONSEC06` writer - Non Secure Attribute bit 06
pub type Nonsec06W<'a, REG> = crate::BitWriter<'a, REG, Nonsec06>;
impl<'a, REG> Nonsec06W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Nonsec06::_0)
    }
    ///Non Secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Nonsec06::_1)
    }
}
/**Non Secure Attribute bit 07

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nonsec07 {
    ///0: Secure
    _0 = 0,
    ///1: Non Secure
    _1 = 1,
}
impl From<Nonsec07> for bool {
    #[inline(always)]
    fn from(variant: Nonsec07) -> Self {
        variant as u8 != 0
    }
}
///Field `NONSEC07` reader - Non Secure Attribute bit 07
pub type Nonsec07R = crate::BitReader<Nonsec07>;
impl Nonsec07R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Nonsec07 {
        match self.bits {
            false => Nonsec07::_0,
            true => Nonsec07::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Nonsec07::_0
    }
    ///Non Secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Nonsec07::_1
    }
}
///Field `NONSEC07` writer - Non Secure Attribute bit 07
pub type Nonsec07W<'a, REG> = crate::BitWriter<'a, REG, Nonsec07>;
impl<'a, REG> Nonsec07W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Nonsec07::_0)
    }
    ///Non Secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Nonsec07::_1)
    }
}
/**Non Secure Attribute bit 08

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nonsec08 {
    ///0: Secure
    _0 = 0,
    ///1: Non Secure
    _1 = 1,
}
impl From<Nonsec08> for bool {
    #[inline(always)]
    fn from(variant: Nonsec08) -> Self {
        variant as u8 != 0
    }
}
///Field `NONSEC08` reader - Non Secure Attribute bit 08
pub type Nonsec08R = crate::BitReader<Nonsec08>;
impl Nonsec08R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Nonsec08 {
        match self.bits {
            false => Nonsec08::_0,
            true => Nonsec08::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Nonsec08::_0
    }
    ///Non Secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Nonsec08::_1
    }
}
///Field `NONSEC08` writer - Non Secure Attribute bit 08
pub type Nonsec08W<'a, REG> = crate::BitWriter<'a, REG, Nonsec08>;
impl<'a, REG> Nonsec08W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Nonsec08::_0)
    }
    ///Non Secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Nonsec08::_1)
    }
}
/**Non Secure Attribute bit 09

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nonsec09 {
    ///0: Secure
    _0 = 0,
    ///1: Non Secure
    _1 = 1,
}
impl From<Nonsec09> for bool {
    #[inline(always)]
    fn from(variant: Nonsec09) -> Self {
        variant as u8 != 0
    }
}
///Field `NONSEC09` reader - Non Secure Attribute bit 09
pub type Nonsec09R = crate::BitReader<Nonsec09>;
impl Nonsec09R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Nonsec09 {
        match self.bits {
            false => Nonsec09::_0,
            true => Nonsec09::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Nonsec09::_0
    }
    ///Non Secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Nonsec09::_1
    }
}
///Field `NONSEC09` writer - Non Secure Attribute bit 09
pub type Nonsec09W<'a, REG> = crate::BitWriter<'a, REG, Nonsec09>;
impl<'a, REG> Nonsec09W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Nonsec09::_0)
    }
    ///Non Secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Nonsec09::_1)
    }
}
/**Non Secure Attribute bit 11

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nonsec11 {
    ///0: Secure
    _0 = 0,
    ///1: Non Secure
    _1 = 1,
}
impl From<Nonsec11> for bool {
    #[inline(always)]
    fn from(variant: Nonsec11) -> Self {
        variant as u8 != 0
    }
}
///Field `NONSEC11` reader - Non Secure Attribute bit 11
pub type Nonsec11R = crate::BitReader<Nonsec11>;
impl Nonsec11R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Nonsec11 {
        match self.bits {
            false => Nonsec11::_0,
            true => Nonsec11::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Nonsec11::_0
    }
    ///Non Secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Nonsec11::_1
    }
}
///Field `NONSEC11` writer - Non Secure Attribute bit 11
pub type Nonsec11W<'a, REG> = crate::BitWriter<'a, REG, Nonsec11>;
impl<'a, REG> Nonsec11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Nonsec11::_0)
    }
    ///Non Secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Nonsec11::_1)
    }
}
/**Non Secure Attribute bit 12

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nonsec12 {
    ///0: Secure
    _0 = 0,
    ///1: Non Secure
    _1 = 1,
}
impl From<Nonsec12> for bool {
    #[inline(always)]
    fn from(variant: Nonsec12) -> Self {
        variant as u8 != 0
    }
}
///Field `NONSEC12` reader - Non Secure Attribute bit 12
pub type Nonsec12R = crate::BitReader<Nonsec12>;
impl Nonsec12R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Nonsec12 {
        match self.bits {
            false => Nonsec12::_0,
            true => Nonsec12::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Nonsec12::_0
    }
    ///Non Secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Nonsec12::_1
    }
}
///Field `NONSEC12` writer - Non Secure Attribute bit 12
pub type Nonsec12W<'a, REG> = crate::BitWriter<'a, REG, Nonsec12>;
impl<'a, REG> Nonsec12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Nonsec12::_0)
    }
    ///Non Secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Nonsec12::_1)
    }
}
/**Non Secure Attribute bit 16

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nonsec16 {
    ///0: Secure
    _0 = 0,
    ///1: Non Secure
    _1 = 1,
}
impl From<Nonsec16> for bool {
    #[inline(always)]
    fn from(variant: Nonsec16) -> Self {
        variant as u8 != 0
    }
}
///Field `NONSEC16` reader - Non Secure Attribute bit 16
pub type Nonsec16R = crate::BitReader<Nonsec16>;
impl Nonsec16R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Nonsec16 {
        match self.bits {
            false => Nonsec16::_0,
            true => Nonsec16::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Nonsec16::_0
    }
    ///Non Secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Nonsec16::_1
    }
}
///Field `NONSEC16` writer - Non Secure Attribute bit 16
pub type Nonsec16W<'a, REG> = crate::BitWriter<'a, REG, Nonsec16>;
impl<'a, REG> Nonsec16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Nonsec16::_0)
    }
    ///Non Secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Nonsec16::_1)
    }
}
/**Non Secure Attribute bit 17

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nonsec17 {
    ///0: Secure
    _0 = 0,
    ///1: Non Secure
    _1 = 1,
}
impl From<Nonsec17> for bool {
    #[inline(always)]
    fn from(variant: Nonsec17) -> Self {
        variant as u8 != 0
    }
}
///Field `NONSEC17` reader - Non Secure Attribute bit 17
pub type Nonsec17R = crate::BitReader<Nonsec17>;
impl Nonsec17R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Nonsec17 {
        match self.bits {
            false => Nonsec17::_0,
            true => Nonsec17::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Nonsec17::_0
    }
    ///Non Secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Nonsec17::_1
    }
}
///Field `NONSEC17` writer - Non Secure Attribute bit 17
pub type Nonsec17W<'a, REG> = crate::BitWriter<'a, REG, Nonsec17>;
impl<'a, REG> Nonsec17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Nonsec17::_0)
    }
    ///Non Secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Nonsec17::_1)
    }
}
/**Non Secure Attribute bit 18

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nonsec18 {
    ///0: Secure
    _0 = 0,
    ///1: Non Secure
    _1 = 1,
}
impl From<Nonsec18> for bool {
    #[inline(always)]
    fn from(variant: Nonsec18) -> Self {
        variant as u8 != 0
    }
}
///Field `NONSEC18` reader - Non Secure Attribute bit 18
pub type Nonsec18R = crate::BitReader<Nonsec18>;
impl Nonsec18R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Nonsec18 {
        match self.bits {
            false => Nonsec18::_0,
            true => Nonsec18::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Nonsec18::_0
    }
    ///Non Secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Nonsec18::_1
    }
}
///Field `NONSEC18` writer - Non Secure Attribute bit 18
pub type Nonsec18W<'a, REG> = crate::BitWriter<'a, REG, Nonsec18>;
impl<'a, REG> Nonsec18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Nonsec18::_0)
    }
    ///Non Secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Nonsec18::_1)
    }
}
/**Non Secure Attribute bit 19

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nonsec19 {
    ///0: Secure
    _0 = 0,
    ///1: Non Secure
    _1 = 1,
}
impl From<Nonsec19> for bool {
    #[inline(always)]
    fn from(variant: Nonsec19) -> Self {
        variant as u8 != 0
    }
}
///Field `NONSEC19` reader - Non Secure Attribute bit 19
pub type Nonsec19R = crate::BitReader<Nonsec19>;
impl Nonsec19R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Nonsec19 {
        match self.bits {
            false => Nonsec19::_0,
            true => Nonsec19::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Nonsec19::_0
    }
    ///Non Secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Nonsec19::_1
    }
}
///Field `NONSEC19` writer - Non Secure Attribute bit 19
pub type Nonsec19W<'a, REG> = crate::BitWriter<'a, REG, Nonsec19>;
impl<'a, REG> Nonsec19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Nonsec19::_0)
    }
    ///Non Secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Nonsec19::_1)
    }
}
/**Non Secure Attribute bit 20

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nonsec20 {
    ///0: Secure
    _0 = 0,
    ///1: Non Secure
    _1 = 1,
}
impl From<Nonsec20> for bool {
    #[inline(always)]
    fn from(variant: Nonsec20) -> Self {
        variant as u8 != 0
    }
}
///Field `NONSEC20` reader - Non Secure Attribute bit 20
pub type Nonsec20R = crate::BitReader<Nonsec20>;
impl Nonsec20R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Nonsec20 {
        match self.bits {
            false => Nonsec20::_0,
            true => Nonsec20::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Nonsec20::_0
    }
    ///Non Secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Nonsec20::_1
    }
}
///Field `NONSEC20` writer - Non Secure Attribute bit 20
pub type Nonsec20W<'a, REG> = crate::BitWriter<'a, REG, Nonsec20>;
impl<'a, REG> Nonsec20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Nonsec20::_0)
    }
    ///Non Secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Nonsec20::_1)
    }
}
impl R {
    ///Bit 0 - Non Secure Attribute bit 00
    #[inline(always)]
    pub fn nonsec00(&self) -> Nonsec00R {
        Nonsec00R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - Non Secure Attribute bit 02
    #[inline(always)]
    pub fn nonsec02(&self) -> Nonsec02R {
        Nonsec02R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Non Secure Attribute bit 03
    #[inline(always)]
    pub fn nonsec03(&self) -> Nonsec03R {
        Nonsec03R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Non Secure Attribute bit 04
    #[inline(always)]
    pub fn nonsec04(&self) -> Nonsec04R {
        Nonsec04R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Non Secure Attribute bit 05
    #[inline(always)]
    pub fn nonsec05(&self) -> Nonsec05R {
        Nonsec05R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Non Secure Attribute bit 06
    #[inline(always)]
    pub fn nonsec06(&self) -> Nonsec06R {
        Nonsec06R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Non Secure Attribute bit 07
    #[inline(always)]
    pub fn nonsec07(&self) -> Nonsec07R {
        Nonsec07R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Non Secure Attribute bit 08
    #[inline(always)]
    pub fn nonsec08(&self) -> Nonsec08R {
        Nonsec08R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Non Secure Attribute bit 09
    #[inline(always)]
    pub fn nonsec09(&self) -> Nonsec09R {
        Nonsec09R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - Non Secure Attribute bit 11
    #[inline(always)]
    pub fn nonsec11(&self) -> Nonsec11R {
        Nonsec11R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Non Secure Attribute bit 12
    #[inline(always)]
    pub fn nonsec12(&self) -> Nonsec12R {
        Nonsec12R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 16 - Non Secure Attribute bit 16
    #[inline(always)]
    pub fn nonsec16(&self) -> Nonsec16R {
        Nonsec16R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Non Secure Attribute bit 17
    #[inline(always)]
    pub fn nonsec17(&self) -> Nonsec17R {
        Nonsec17R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Non Secure Attribute bit 18
    #[inline(always)]
    pub fn nonsec18(&self) -> Nonsec18R {
        Nonsec18R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Non Secure Attribute bit 19
    #[inline(always)]
    pub fn nonsec19(&self) -> Nonsec19R {
        Nonsec19R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Non Secure Attribute bit 20
    #[inline(always)]
    pub fn nonsec20(&self) -> Nonsec20R {
        Nonsec20R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CGFSAR")
            .field("nonsec00", &self.nonsec00())
            .field("nonsec02", &self.nonsec02())
            .field("nonsec03", &self.nonsec03())
            .field("nonsec04", &self.nonsec04())
            .field("nonsec05", &self.nonsec05())
            .field("nonsec06", &self.nonsec06())
            .field("nonsec07", &self.nonsec07())
            .field("nonsec08", &self.nonsec08())
            .field("nonsec09", &self.nonsec09())
            .field("nonsec11", &self.nonsec11())
            .field("nonsec12", &self.nonsec12())
            .field("nonsec16", &self.nonsec16())
            .field("nonsec17", &self.nonsec17())
            .field("nonsec18", &self.nonsec18())
            .field("nonsec19", &self.nonsec19())
            .field("nonsec20", &self.nonsec20())
            .finish()
    }
}
impl W {
    ///Bit 0 - Non Secure Attribute bit 00
    #[inline(always)]
    pub fn nonsec00(&mut self) -> Nonsec00W<CgfsarSpec> {
        Nonsec00W::new(self, 0)
    }
    ///Bit 2 - Non Secure Attribute bit 02
    #[inline(always)]
    pub fn nonsec02(&mut self) -> Nonsec02W<CgfsarSpec> {
        Nonsec02W::new(self, 2)
    }
    ///Bit 3 - Non Secure Attribute bit 03
    #[inline(always)]
    pub fn nonsec03(&mut self) -> Nonsec03W<CgfsarSpec> {
        Nonsec03W::new(self, 3)
    }
    ///Bit 4 - Non Secure Attribute bit 04
    #[inline(always)]
    pub fn nonsec04(&mut self) -> Nonsec04W<CgfsarSpec> {
        Nonsec04W::new(self, 4)
    }
    ///Bit 5 - Non Secure Attribute bit 05
    #[inline(always)]
    pub fn nonsec05(&mut self) -> Nonsec05W<CgfsarSpec> {
        Nonsec05W::new(self, 5)
    }
    ///Bit 6 - Non Secure Attribute bit 06
    #[inline(always)]
    pub fn nonsec06(&mut self) -> Nonsec06W<CgfsarSpec> {
        Nonsec06W::new(self, 6)
    }
    ///Bit 7 - Non Secure Attribute bit 07
    #[inline(always)]
    pub fn nonsec07(&mut self) -> Nonsec07W<CgfsarSpec> {
        Nonsec07W::new(self, 7)
    }
    ///Bit 8 - Non Secure Attribute bit 08
    #[inline(always)]
    pub fn nonsec08(&mut self) -> Nonsec08W<CgfsarSpec> {
        Nonsec08W::new(self, 8)
    }
    ///Bit 9 - Non Secure Attribute bit 09
    #[inline(always)]
    pub fn nonsec09(&mut self) -> Nonsec09W<CgfsarSpec> {
        Nonsec09W::new(self, 9)
    }
    ///Bit 11 - Non Secure Attribute bit 11
    #[inline(always)]
    pub fn nonsec11(&mut self) -> Nonsec11W<CgfsarSpec> {
        Nonsec11W::new(self, 11)
    }
    ///Bit 12 - Non Secure Attribute bit 12
    #[inline(always)]
    pub fn nonsec12(&mut self) -> Nonsec12W<CgfsarSpec> {
        Nonsec12W::new(self, 12)
    }
    ///Bit 16 - Non Secure Attribute bit 16
    #[inline(always)]
    pub fn nonsec16(&mut self) -> Nonsec16W<CgfsarSpec> {
        Nonsec16W::new(self, 16)
    }
    ///Bit 17 - Non Secure Attribute bit 17
    #[inline(always)]
    pub fn nonsec17(&mut self) -> Nonsec17W<CgfsarSpec> {
        Nonsec17W::new(self, 17)
    }
    ///Bit 18 - Non Secure Attribute bit 18
    #[inline(always)]
    pub fn nonsec18(&mut self) -> Nonsec18W<CgfsarSpec> {
        Nonsec18W::new(self, 18)
    }
    ///Bit 19 - Non Secure Attribute bit 19
    #[inline(always)]
    pub fn nonsec19(&mut self) -> Nonsec19W<CgfsarSpec> {
        Nonsec19W::new(self, 19)
    }
    ///Bit 20 - Non Secure Attribute bit 20
    #[inline(always)]
    pub fn nonsec20(&mut self) -> Nonsec20W<CgfsarSpec> {
        Nonsec20W::new(self, 20)
    }
}
/**Clock Generation Function Security Attribute Register

You can [`read`](crate::Reg::read) this register and get [`cgfsar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cgfsar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CgfsarSpec;
impl crate::RegisterSpec for CgfsarSpec {
    type Ux = u32;
}
///`read()` method returns [`cgfsar::R`](R) reader structure
impl crate::Readable for CgfsarSpec {}
///`write(|w| ..)` method takes [`cgfsar::W`](W) writer structure
impl crate::Writable for CgfsarSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CGFSAR to value 0xffff_ffff
impl crate::Resettable for CgfsarSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
