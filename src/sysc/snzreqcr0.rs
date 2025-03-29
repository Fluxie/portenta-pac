///Register `SNZREQCR0` reader
pub type R = crate::R<Snzreqcr0Spec>;
///Register `SNZREQCR0` writer
pub type W = crate::W<Snzreqcr0Spec>;
/**Enable IRQ0 pin snooze request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snzreqen0 {
    ///0: Disable the snooze request
    _0 = 0,
    ///1: Enable the snooze request
    _1 = 1,
}
impl From<Snzreqen0> for bool {
    #[inline(always)]
    fn from(variant: Snzreqen0) -> Self {
        variant as u8 != 0
    }
}
///Field `SNZREQEN0` reader - Enable IRQ0 pin snooze request
pub type Snzreqen0R = crate::BitReader<Snzreqen0>;
impl Snzreqen0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Snzreqen0 {
        match self.bits {
            false => Snzreqen0::_0,
            true => Snzreqen0::_1,
        }
    }
    ///Disable the snooze request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Snzreqen0::_0
    }
    ///Enable the snooze request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Snzreqen0::_1
    }
}
///Field `SNZREQEN0` writer - Enable IRQ0 pin snooze request
pub type Snzreqen0W<'a, REG> = crate::BitWriter<'a, REG, Snzreqen0>;
impl<'a, REG> Snzreqen0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable the snooze request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen0::_0)
    }
    ///Enable the snooze request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen0::_1)
    }
}
/**Enable IRQ1 pin snooze request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snzreqen1 {
    ///0: Disable the snooze request
    _0 = 0,
    ///1: Enable the snooze request
    _1 = 1,
}
impl From<Snzreqen1> for bool {
    #[inline(always)]
    fn from(variant: Snzreqen1) -> Self {
        variant as u8 != 0
    }
}
///Field `SNZREQEN1` reader - Enable IRQ1 pin snooze request
pub type Snzreqen1R = crate::BitReader<Snzreqen1>;
impl Snzreqen1R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Snzreqen1 {
        match self.bits {
            false => Snzreqen1::_0,
            true => Snzreqen1::_1,
        }
    }
    ///Disable the snooze request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Snzreqen1::_0
    }
    ///Enable the snooze request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Snzreqen1::_1
    }
}
///Field `SNZREQEN1` writer - Enable IRQ1 pin snooze request
pub type Snzreqen1W<'a, REG> = crate::BitWriter<'a, REG, Snzreqen1>;
impl<'a, REG> Snzreqen1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable the snooze request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen1::_0)
    }
    ///Enable the snooze request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen1::_1)
    }
}
/**Enable IRQ2 pin snooze request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snzreqen2 {
    ///0: Disable the snooze request
    _0 = 0,
    ///1: Enable the snooze request
    _1 = 1,
}
impl From<Snzreqen2> for bool {
    #[inline(always)]
    fn from(variant: Snzreqen2) -> Self {
        variant as u8 != 0
    }
}
///Field `SNZREQEN2` reader - Enable IRQ2 pin snooze request
pub type Snzreqen2R = crate::BitReader<Snzreqen2>;
impl Snzreqen2R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Snzreqen2 {
        match self.bits {
            false => Snzreqen2::_0,
            true => Snzreqen2::_1,
        }
    }
    ///Disable the snooze request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Snzreqen2::_0
    }
    ///Enable the snooze request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Snzreqen2::_1
    }
}
///Field `SNZREQEN2` writer - Enable IRQ2 pin snooze request
pub type Snzreqen2W<'a, REG> = crate::BitWriter<'a, REG, Snzreqen2>;
impl<'a, REG> Snzreqen2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable the snooze request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen2::_0)
    }
    ///Enable the snooze request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen2::_1)
    }
}
/**Enable IRQ3 pin snooze request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snzreqen3 {
    ///0: Disable the snooze request
    _0 = 0,
    ///1: Enable the snooze request
    _1 = 1,
}
impl From<Snzreqen3> for bool {
    #[inline(always)]
    fn from(variant: Snzreqen3) -> Self {
        variant as u8 != 0
    }
}
///Field `SNZREQEN3` reader - Enable IRQ3 pin snooze request
pub type Snzreqen3R = crate::BitReader<Snzreqen3>;
impl Snzreqen3R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Snzreqen3 {
        match self.bits {
            false => Snzreqen3::_0,
            true => Snzreqen3::_1,
        }
    }
    ///Disable the snooze request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Snzreqen3::_0
    }
    ///Enable the snooze request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Snzreqen3::_1
    }
}
///Field `SNZREQEN3` writer - Enable IRQ3 pin snooze request
pub type Snzreqen3W<'a, REG> = crate::BitWriter<'a, REG, Snzreqen3>;
impl<'a, REG> Snzreqen3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable the snooze request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen3::_0)
    }
    ///Enable the snooze request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen3::_1)
    }
}
/**Enable IRQ4 pin snooze request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snzreqen4 {
    ///0: Disable the snooze request
    _0 = 0,
    ///1: Enable the snooze request
    _1 = 1,
}
impl From<Snzreqen4> for bool {
    #[inline(always)]
    fn from(variant: Snzreqen4) -> Self {
        variant as u8 != 0
    }
}
///Field `SNZREQEN4` reader - Enable IRQ4 pin snooze request
pub type Snzreqen4R = crate::BitReader<Snzreqen4>;
impl Snzreqen4R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Snzreqen4 {
        match self.bits {
            false => Snzreqen4::_0,
            true => Snzreqen4::_1,
        }
    }
    ///Disable the snooze request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Snzreqen4::_0
    }
    ///Enable the snooze request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Snzreqen4::_1
    }
}
///Field `SNZREQEN4` writer - Enable IRQ4 pin snooze request
pub type Snzreqen4W<'a, REG> = crate::BitWriter<'a, REG, Snzreqen4>;
impl<'a, REG> Snzreqen4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable the snooze request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen4::_0)
    }
    ///Enable the snooze request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen4::_1)
    }
}
/**Enable IRQ5 pin snooze request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snzreqen5 {
    ///0: Disable the snooze request
    _0 = 0,
    ///1: Enable the snooze request
    _1 = 1,
}
impl From<Snzreqen5> for bool {
    #[inline(always)]
    fn from(variant: Snzreqen5) -> Self {
        variant as u8 != 0
    }
}
///Field `SNZREQEN5` reader - Enable IRQ5 pin snooze request
pub type Snzreqen5R = crate::BitReader<Snzreqen5>;
impl Snzreqen5R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Snzreqen5 {
        match self.bits {
            false => Snzreqen5::_0,
            true => Snzreqen5::_1,
        }
    }
    ///Disable the snooze request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Snzreqen5::_0
    }
    ///Enable the snooze request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Snzreqen5::_1
    }
}
///Field `SNZREQEN5` writer - Enable IRQ5 pin snooze request
pub type Snzreqen5W<'a, REG> = crate::BitWriter<'a, REG, Snzreqen5>;
impl<'a, REG> Snzreqen5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable the snooze request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen5::_0)
    }
    ///Enable the snooze request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen5::_1)
    }
}
/**Enable IRQ6 pin snooze request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snzreqen6 {
    ///0: Disable the snooze request
    _0 = 0,
    ///1: Enable the snooze request
    _1 = 1,
}
impl From<Snzreqen6> for bool {
    #[inline(always)]
    fn from(variant: Snzreqen6) -> Self {
        variant as u8 != 0
    }
}
///Field `SNZREQEN6` reader - Enable IRQ6 pin snooze request
pub type Snzreqen6R = crate::BitReader<Snzreqen6>;
impl Snzreqen6R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Snzreqen6 {
        match self.bits {
            false => Snzreqen6::_0,
            true => Snzreqen6::_1,
        }
    }
    ///Disable the snooze request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Snzreqen6::_0
    }
    ///Enable the snooze request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Snzreqen6::_1
    }
}
///Field `SNZREQEN6` writer - Enable IRQ6 pin snooze request
pub type Snzreqen6W<'a, REG> = crate::BitWriter<'a, REG, Snzreqen6>;
impl<'a, REG> Snzreqen6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable the snooze request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen6::_0)
    }
    ///Enable the snooze request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen6::_1)
    }
}
/**Enable IRQ7 pin snooze request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snzreqen7 {
    ///0: Disable the snooze request
    _0 = 0,
    ///1: Enable the snooze request
    _1 = 1,
}
impl From<Snzreqen7> for bool {
    #[inline(always)]
    fn from(variant: Snzreqen7) -> Self {
        variant as u8 != 0
    }
}
///Field `SNZREQEN7` reader - Enable IRQ7 pin snooze request
pub type Snzreqen7R = crate::BitReader<Snzreqen7>;
impl Snzreqen7R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Snzreqen7 {
        match self.bits {
            false => Snzreqen7::_0,
            true => Snzreqen7::_1,
        }
    }
    ///Disable the snooze request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Snzreqen7::_0
    }
    ///Enable the snooze request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Snzreqen7::_1
    }
}
///Field `SNZREQEN7` writer - Enable IRQ7 pin snooze request
pub type Snzreqen7W<'a, REG> = crate::BitWriter<'a, REG, Snzreqen7>;
impl<'a, REG> Snzreqen7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable the snooze request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen7::_0)
    }
    ///Enable the snooze request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen7::_1)
    }
}
/**Enable IRQ8 pin snooze request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snzreqen8 {
    ///0: Disable the snooze request
    _0 = 0,
    ///1: Enable the snooze request
    _1 = 1,
}
impl From<Snzreqen8> for bool {
    #[inline(always)]
    fn from(variant: Snzreqen8) -> Self {
        variant as u8 != 0
    }
}
///Field `SNZREQEN8` reader - Enable IRQ8 pin snooze request
pub type Snzreqen8R = crate::BitReader<Snzreqen8>;
impl Snzreqen8R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Snzreqen8 {
        match self.bits {
            false => Snzreqen8::_0,
            true => Snzreqen8::_1,
        }
    }
    ///Disable the snooze request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Snzreqen8::_0
    }
    ///Enable the snooze request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Snzreqen8::_1
    }
}
///Field `SNZREQEN8` writer - Enable IRQ8 pin snooze request
pub type Snzreqen8W<'a, REG> = crate::BitWriter<'a, REG, Snzreqen8>;
impl<'a, REG> Snzreqen8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable the snooze request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen8::_0)
    }
    ///Enable the snooze request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen8::_1)
    }
}
/**Enable IRQ9 pin snooze request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snzreqen9 {
    ///0: Disable the snooze request
    _0 = 0,
    ///1: Enable the snooze request
    _1 = 1,
}
impl From<Snzreqen9> for bool {
    #[inline(always)]
    fn from(variant: Snzreqen9) -> Self {
        variant as u8 != 0
    }
}
///Field `SNZREQEN9` reader - Enable IRQ9 pin snooze request
pub type Snzreqen9R = crate::BitReader<Snzreqen9>;
impl Snzreqen9R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Snzreqen9 {
        match self.bits {
            false => Snzreqen9::_0,
            true => Snzreqen9::_1,
        }
    }
    ///Disable the snooze request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Snzreqen9::_0
    }
    ///Enable the snooze request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Snzreqen9::_1
    }
}
///Field `SNZREQEN9` writer - Enable IRQ9 pin snooze request
pub type Snzreqen9W<'a, REG> = crate::BitWriter<'a, REG, Snzreqen9>;
impl<'a, REG> Snzreqen9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable the snooze request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen9::_0)
    }
    ///Enable the snooze request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen9::_1)
    }
}
/**Enable IRQ10 pin snooze request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snzreqen10 {
    ///0: Disable the snooze request
    _0 = 0,
    ///1: Enable the snooze request
    _1 = 1,
}
impl From<Snzreqen10> for bool {
    #[inline(always)]
    fn from(variant: Snzreqen10) -> Self {
        variant as u8 != 0
    }
}
///Field `SNZREQEN10` reader - Enable IRQ10 pin snooze request
pub type Snzreqen10R = crate::BitReader<Snzreqen10>;
impl Snzreqen10R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Snzreqen10 {
        match self.bits {
            false => Snzreqen10::_0,
            true => Snzreqen10::_1,
        }
    }
    ///Disable the snooze request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Snzreqen10::_0
    }
    ///Enable the snooze request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Snzreqen10::_1
    }
}
///Field `SNZREQEN10` writer - Enable IRQ10 pin snooze request
pub type Snzreqen10W<'a, REG> = crate::BitWriter<'a, REG, Snzreqen10>;
impl<'a, REG> Snzreqen10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable the snooze request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen10::_0)
    }
    ///Enable the snooze request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen10::_1)
    }
}
/**Enable IRQ11 pin snooze request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snzreqen11 {
    ///0: Disable the snooze request
    _0 = 0,
    ///1: Enable the snooze request
    _1 = 1,
}
impl From<Snzreqen11> for bool {
    #[inline(always)]
    fn from(variant: Snzreqen11) -> Self {
        variant as u8 != 0
    }
}
///Field `SNZREQEN11` reader - Enable IRQ11 pin snooze request
pub type Snzreqen11R = crate::BitReader<Snzreqen11>;
impl Snzreqen11R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Snzreqen11 {
        match self.bits {
            false => Snzreqen11::_0,
            true => Snzreqen11::_1,
        }
    }
    ///Disable the snooze request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Snzreqen11::_0
    }
    ///Enable the snooze request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Snzreqen11::_1
    }
}
///Field `SNZREQEN11` writer - Enable IRQ11 pin snooze request
pub type Snzreqen11W<'a, REG> = crate::BitWriter<'a, REG, Snzreqen11>;
impl<'a, REG> Snzreqen11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable the snooze request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen11::_0)
    }
    ///Enable the snooze request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen11::_1)
    }
}
/**Enable IRQ12 pin snooze request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snzreqen12 {
    ///0: Disable the snooze request
    _0 = 0,
    ///1: Enable the snooze request
    _1 = 1,
}
impl From<Snzreqen12> for bool {
    #[inline(always)]
    fn from(variant: Snzreqen12) -> Self {
        variant as u8 != 0
    }
}
///Field `SNZREQEN12` reader - Enable IRQ12 pin snooze request
pub type Snzreqen12R = crate::BitReader<Snzreqen12>;
impl Snzreqen12R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Snzreqen12 {
        match self.bits {
            false => Snzreqen12::_0,
            true => Snzreqen12::_1,
        }
    }
    ///Disable the snooze request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Snzreqen12::_0
    }
    ///Enable the snooze request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Snzreqen12::_1
    }
}
///Field `SNZREQEN12` writer - Enable IRQ12 pin snooze request
pub type Snzreqen12W<'a, REG> = crate::BitWriter<'a, REG, Snzreqen12>;
impl<'a, REG> Snzreqen12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable the snooze request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen12::_0)
    }
    ///Enable the snooze request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen12::_1)
    }
}
/**Enable IRQ13 pin snooze request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snzreqen13 {
    ///0: Disable the snooze request
    _0 = 0,
    ///1: Enable the snooze request
    _1 = 1,
}
impl From<Snzreqen13> for bool {
    #[inline(always)]
    fn from(variant: Snzreqen13) -> Self {
        variant as u8 != 0
    }
}
///Field `SNZREQEN13` reader - Enable IRQ13 pin snooze request
pub type Snzreqen13R = crate::BitReader<Snzreqen13>;
impl Snzreqen13R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Snzreqen13 {
        match self.bits {
            false => Snzreqen13::_0,
            true => Snzreqen13::_1,
        }
    }
    ///Disable the snooze request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Snzreqen13::_0
    }
    ///Enable the snooze request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Snzreqen13::_1
    }
}
///Field `SNZREQEN13` writer - Enable IRQ13 pin snooze request
pub type Snzreqen13W<'a, REG> = crate::BitWriter<'a, REG, Snzreqen13>;
impl<'a, REG> Snzreqen13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable the snooze request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen13::_0)
    }
    ///Enable the snooze request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen13::_1)
    }
}
/**Enable IRQ14 pin snooze request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snzreqen14 {
    ///0: Disable the snooze request
    _0 = 0,
    ///1: Enable the snooze request
    _1 = 1,
}
impl From<Snzreqen14> for bool {
    #[inline(always)]
    fn from(variant: Snzreqen14) -> Self {
        variant as u8 != 0
    }
}
///Field `SNZREQEN14` reader - Enable IRQ14 pin snooze request
pub type Snzreqen14R = crate::BitReader<Snzreqen14>;
impl Snzreqen14R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Snzreqen14 {
        match self.bits {
            false => Snzreqen14::_0,
            true => Snzreqen14::_1,
        }
    }
    ///Disable the snooze request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Snzreqen14::_0
    }
    ///Enable the snooze request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Snzreqen14::_1
    }
}
///Field `SNZREQEN14` writer - Enable IRQ14 pin snooze request
pub type Snzreqen14W<'a, REG> = crate::BitWriter<'a, REG, Snzreqen14>;
impl<'a, REG> Snzreqen14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable the snooze request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen14::_0)
    }
    ///Enable the snooze request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen14::_1)
    }
}
/**Enable IRQ15 pin snooze request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snzreqen15 {
    ///0: Disable the snooze request
    _0 = 0,
    ///1: Enable the snooze request
    _1 = 1,
}
impl From<Snzreqen15> for bool {
    #[inline(always)]
    fn from(variant: Snzreqen15) -> Self {
        variant as u8 != 0
    }
}
///Field `SNZREQEN15` reader - Enable IRQ15 pin snooze request
pub type Snzreqen15R = crate::BitReader<Snzreqen15>;
impl Snzreqen15R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Snzreqen15 {
        match self.bits {
            false => Snzreqen15::_0,
            true => Snzreqen15::_1,
        }
    }
    ///Disable the snooze request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Snzreqen15::_0
    }
    ///Enable the snooze request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Snzreqen15::_1
    }
}
///Field `SNZREQEN15` writer - Enable IRQ15 pin snooze request
pub type Snzreqen15W<'a, REG> = crate::BitWriter<'a, REG, Snzreqen15>;
impl<'a, REG> Snzreqen15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable the snooze request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen15::_0)
    }
    ///Enable the snooze request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen15::_1)
    }
}
/**Enable RTC alarm snooze request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snzreqen24 {
    ///0: Disable the snooze request
    _0 = 0,
    ///1: Enable the snooze request
    _1 = 1,
}
impl From<Snzreqen24> for bool {
    #[inline(always)]
    fn from(variant: Snzreqen24) -> Self {
        variant as u8 != 0
    }
}
///Field `SNZREQEN24` reader - Enable RTC alarm snooze request
pub type Snzreqen24R = crate::BitReader<Snzreqen24>;
impl Snzreqen24R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Snzreqen24 {
        match self.bits {
            false => Snzreqen24::_0,
            true => Snzreqen24::_1,
        }
    }
    ///Disable the snooze request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Snzreqen24::_0
    }
    ///Enable the snooze request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Snzreqen24::_1
    }
}
///Field `SNZREQEN24` writer - Enable RTC alarm snooze request
pub type Snzreqen24W<'a, REG> = crate::BitWriter<'a, REG, Snzreqen24>;
impl<'a, REG> Snzreqen24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable the snooze request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen24::_0)
    }
    ///Enable the snooze request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen24::_1)
    }
}
/**Enable RTC period snooze request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snzreqen25 {
    ///0: Disable the snooze request
    _0 = 0,
    ///1: Enable the snooze request
    _1 = 1,
}
impl From<Snzreqen25> for bool {
    #[inline(always)]
    fn from(variant: Snzreqen25) -> Self {
        variant as u8 != 0
    }
}
///Field `SNZREQEN25` reader - Enable RTC period snooze request
pub type Snzreqen25R = crate::BitReader<Snzreqen25>;
impl Snzreqen25R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Snzreqen25 {
        match self.bits {
            false => Snzreqen25::_0,
            true => Snzreqen25::_1,
        }
    }
    ///Disable the snooze request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Snzreqen25::_0
    }
    ///Enable the snooze request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Snzreqen25::_1
    }
}
///Field `SNZREQEN25` writer - Enable RTC period snooze request
pub type Snzreqen25W<'a, REG> = crate::BitWriter<'a, REG, Snzreqen25>;
impl<'a, REG> Snzreqen25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable the snooze request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen25::_0)
    }
    ///Enable the snooze request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen25::_1)
    }
}
/**Enable AGT1 underflow snooze request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snzreqen28 {
    ///0: Disable the snooze request
    _0 = 0,
    ///1: Enable the snooze request
    _1 = 1,
}
impl From<Snzreqen28> for bool {
    #[inline(always)]
    fn from(variant: Snzreqen28) -> Self {
        variant as u8 != 0
    }
}
///Field `SNZREQEN28` reader - Enable AGT1 underflow snooze request
pub type Snzreqen28R = crate::BitReader<Snzreqen28>;
impl Snzreqen28R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Snzreqen28 {
        match self.bits {
            false => Snzreqen28::_0,
            true => Snzreqen28::_1,
        }
    }
    ///Disable the snooze request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Snzreqen28::_0
    }
    ///Enable the snooze request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Snzreqen28::_1
    }
}
///Field `SNZREQEN28` writer - Enable AGT1 underflow snooze request
pub type Snzreqen28W<'a, REG> = crate::BitWriter<'a, REG, Snzreqen28>;
impl<'a, REG> Snzreqen28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable the snooze request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen28::_0)
    }
    ///Enable the snooze request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen28::_1)
    }
}
/**Enable AGT1 compare match A snooze request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snzreqen29 {
    ///0: Disable the snooze request
    _0 = 0,
    ///1: Enable the snooze request
    _1 = 1,
}
impl From<Snzreqen29> for bool {
    #[inline(always)]
    fn from(variant: Snzreqen29) -> Self {
        variant as u8 != 0
    }
}
///Field `SNZREQEN29` reader - Enable AGT1 compare match A snooze request
pub type Snzreqen29R = crate::BitReader<Snzreqen29>;
impl Snzreqen29R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Snzreqen29 {
        match self.bits {
            false => Snzreqen29::_0,
            true => Snzreqen29::_1,
        }
    }
    ///Disable the snooze request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Snzreqen29::_0
    }
    ///Enable the snooze request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Snzreqen29::_1
    }
}
///Field `SNZREQEN29` writer - Enable AGT1 compare match A snooze request
pub type Snzreqen29W<'a, REG> = crate::BitWriter<'a, REG, Snzreqen29>;
impl<'a, REG> Snzreqen29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable the snooze request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen29::_0)
    }
    ///Enable the snooze request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen29::_1)
    }
}
/**Enable AGT1 compare match B snooze request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snzreqen30 {
    ///0: Disable the snooze request
    _0 = 0,
    ///1: Enable the snooze request
    _1 = 1,
}
impl From<Snzreqen30> for bool {
    #[inline(always)]
    fn from(variant: Snzreqen30) -> Self {
        variant as u8 != 0
    }
}
///Field `SNZREQEN30` reader - Enable AGT1 compare match B snooze request
pub type Snzreqen30R = crate::BitReader<Snzreqen30>;
impl Snzreqen30R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Snzreqen30 {
        match self.bits {
            false => Snzreqen30::_0,
            true => Snzreqen30::_1,
        }
    }
    ///Disable the snooze request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Snzreqen30::_0
    }
    ///Enable the snooze request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Snzreqen30::_1
    }
}
///Field `SNZREQEN30` writer - Enable AGT1 compare match B snooze request
pub type Snzreqen30W<'a, REG> = crate::BitWriter<'a, REG, Snzreqen30>;
impl<'a, REG> Snzreqen30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable the snooze request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen30::_0)
    }
    ///Enable the snooze request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen30::_1)
    }
}
impl R {
    ///Bit 0 - Enable IRQ0 pin snooze request
    #[inline(always)]
    pub fn snzreqen0(&self) -> Snzreqen0R {
        Snzreqen0R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Enable IRQ1 pin snooze request
    #[inline(always)]
    pub fn snzreqen1(&self) -> Snzreqen1R {
        Snzreqen1R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Enable IRQ2 pin snooze request
    #[inline(always)]
    pub fn snzreqen2(&self) -> Snzreqen2R {
        Snzreqen2R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Enable IRQ3 pin snooze request
    #[inline(always)]
    pub fn snzreqen3(&self) -> Snzreqen3R {
        Snzreqen3R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Enable IRQ4 pin snooze request
    #[inline(always)]
    pub fn snzreqen4(&self) -> Snzreqen4R {
        Snzreqen4R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Enable IRQ5 pin snooze request
    #[inline(always)]
    pub fn snzreqen5(&self) -> Snzreqen5R {
        Snzreqen5R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Enable IRQ6 pin snooze request
    #[inline(always)]
    pub fn snzreqen6(&self) -> Snzreqen6R {
        Snzreqen6R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Enable IRQ7 pin snooze request
    #[inline(always)]
    pub fn snzreqen7(&self) -> Snzreqen7R {
        Snzreqen7R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Enable IRQ8 pin snooze request
    #[inline(always)]
    pub fn snzreqen8(&self) -> Snzreqen8R {
        Snzreqen8R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Enable IRQ9 pin snooze request
    #[inline(always)]
    pub fn snzreqen9(&self) -> Snzreqen9R {
        Snzreqen9R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Enable IRQ10 pin snooze request
    #[inline(always)]
    pub fn snzreqen10(&self) -> Snzreqen10R {
        Snzreqen10R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Enable IRQ11 pin snooze request
    #[inline(always)]
    pub fn snzreqen11(&self) -> Snzreqen11R {
        Snzreqen11R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Enable IRQ12 pin snooze request
    #[inline(always)]
    pub fn snzreqen12(&self) -> Snzreqen12R {
        Snzreqen12R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Enable IRQ13 pin snooze request
    #[inline(always)]
    pub fn snzreqen13(&self) -> Snzreqen13R {
        Snzreqen13R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Enable IRQ14 pin snooze request
    #[inline(always)]
    pub fn snzreqen14(&self) -> Snzreqen14R {
        Snzreqen14R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Enable IRQ15 pin snooze request
    #[inline(always)]
    pub fn snzreqen15(&self) -> Snzreqen15R {
        Snzreqen15R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 24 - Enable RTC alarm snooze request
    #[inline(always)]
    pub fn snzreqen24(&self) -> Snzreqen24R {
        Snzreqen24R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Enable RTC period snooze request
    #[inline(always)]
    pub fn snzreqen25(&self) -> Snzreqen25R {
        Snzreqen25R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 28 - Enable AGT1 underflow snooze request
    #[inline(always)]
    pub fn snzreqen28(&self) -> Snzreqen28R {
        Snzreqen28R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Enable AGT1 compare match A snooze request
    #[inline(always)]
    pub fn snzreqen29(&self) -> Snzreqen29R {
        Snzreqen29R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Enable AGT1 compare match B snooze request
    #[inline(always)]
    pub fn snzreqen30(&self) -> Snzreqen30R {
        Snzreqen30R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SNZREQCR0")
            .field("snzreqen0", &self.snzreqen0())
            .field("snzreqen1", &self.snzreqen1())
            .field("snzreqen2", &self.snzreqen2())
            .field("snzreqen3", &self.snzreqen3())
            .field("snzreqen4", &self.snzreqen4())
            .field("snzreqen5", &self.snzreqen5())
            .field("snzreqen6", &self.snzreqen6())
            .field("snzreqen7", &self.snzreqen7())
            .field("snzreqen8", &self.snzreqen8())
            .field("snzreqen9", &self.snzreqen9())
            .field("snzreqen10", &self.snzreqen10())
            .field("snzreqen11", &self.snzreqen11())
            .field("snzreqen12", &self.snzreqen12())
            .field("snzreqen13", &self.snzreqen13())
            .field("snzreqen14", &self.snzreqen14())
            .field("snzreqen15", &self.snzreqen15())
            .field("snzreqen24", &self.snzreqen24())
            .field("snzreqen25", &self.snzreqen25())
            .field("snzreqen28", &self.snzreqen28())
            .field("snzreqen29", &self.snzreqen29())
            .field("snzreqen30", &self.snzreqen30())
            .finish()
    }
}
impl W {
    ///Bit 0 - Enable IRQ0 pin snooze request
    #[inline(always)]
    pub fn snzreqen0(&mut self) -> Snzreqen0W<Snzreqcr0Spec> {
        Snzreqen0W::new(self, 0)
    }
    ///Bit 1 - Enable IRQ1 pin snooze request
    #[inline(always)]
    pub fn snzreqen1(&mut self) -> Snzreqen1W<Snzreqcr0Spec> {
        Snzreqen1W::new(self, 1)
    }
    ///Bit 2 - Enable IRQ2 pin snooze request
    #[inline(always)]
    pub fn snzreqen2(&mut self) -> Snzreqen2W<Snzreqcr0Spec> {
        Snzreqen2W::new(self, 2)
    }
    ///Bit 3 - Enable IRQ3 pin snooze request
    #[inline(always)]
    pub fn snzreqen3(&mut self) -> Snzreqen3W<Snzreqcr0Spec> {
        Snzreqen3W::new(self, 3)
    }
    ///Bit 4 - Enable IRQ4 pin snooze request
    #[inline(always)]
    pub fn snzreqen4(&mut self) -> Snzreqen4W<Snzreqcr0Spec> {
        Snzreqen4W::new(self, 4)
    }
    ///Bit 5 - Enable IRQ5 pin snooze request
    #[inline(always)]
    pub fn snzreqen5(&mut self) -> Snzreqen5W<Snzreqcr0Spec> {
        Snzreqen5W::new(self, 5)
    }
    ///Bit 6 - Enable IRQ6 pin snooze request
    #[inline(always)]
    pub fn snzreqen6(&mut self) -> Snzreqen6W<Snzreqcr0Spec> {
        Snzreqen6W::new(self, 6)
    }
    ///Bit 7 - Enable IRQ7 pin snooze request
    #[inline(always)]
    pub fn snzreqen7(&mut self) -> Snzreqen7W<Snzreqcr0Spec> {
        Snzreqen7W::new(self, 7)
    }
    ///Bit 8 - Enable IRQ8 pin snooze request
    #[inline(always)]
    pub fn snzreqen8(&mut self) -> Snzreqen8W<Snzreqcr0Spec> {
        Snzreqen8W::new(self, 8)
    }
    ///Bit 9 - Enable IRQ9 pin snooze request
    #[inline(always)]
    pub fn snzreqen9(&mut self) -> Snzreqen9W<Snzreqcr0Spec> {
        Snzreqen9W::new(self, 9)
    }
    ///Bit 10 - Enable IRQ10 pin snooze request
    #[inline(always)]
    pub fn snzreqen10(&mut self) -> Snzreqen10W<Snzreqcr0Spec> {
        Snzreqen10W::new(self, 10)
    }
    ///Bit 11 - Enable IRQ11 pin snooze request
    #[inline(always)]
    pub fn snzreqen11(&mut self) -> Snzreqen11W<Snzreqcr0Spec> {
        Snzreqen11W::new(self, 11)
    }
    ///Bit 12 - Enable IRQ12 pin snooze request
    #[inline(always)]
    pub fn snzreqen12(&mut self) -> Snzreqen12W<Snzreqcr0Spec> {
        Snzreqen12W::new(self, 12)
    }
    ///Bit 13 - Enable IRQ13 pin snooze request
    #[inline(always)]
    pub fn snzreqen13(&mut self) -> Snzreqen13W<Snzreqcr0Spec> {
        Snzreqen13W::new(self, 13)
    }
    ///Bit 14 - Enable IRQ14 pin snooze request
    #[inline(always)]
    pub fn snzreqen14(&mut self) -> Snzreqen14W<Snzreqcr0Spec> {
        Snzreqen14W::new(self, 14)
    }
    ///Bit 15 - Enable IRQ15 pin snooze request
    #[inline(always)]
    pub fn snzreqen15(&mut self) -> Snzreqen15W<Snzreqcr0Spec> {
        Snzreqen15W::new(self, 15)
    }
    ///Bit 24 - Enable RTC alarm snooze request
    #[inline(always)]
    pub fn snzreqen24(&mut self) -> Snzreqen24W<Snzreqcr0Spec> {
        Snzreqen24W::new(self, 24)
    }
    ///Bit 25 - Enable RTC period snooze request
    #[inline(always)]
    pub fn snzreqen25(&mut self) -> Snzreqen25W<Snzreqcr0Spec> {
        Snzreqen25W::new(self, 25)
    }
    ///Bit 28 - Enable AGT1 underflow snooze request
    #[inline(always)]
    pub fn snzreqen28(&mut self) -> Snzreqen28W<Snzreqcr0Spec> {
        Snzreqen28W::new(self, 28)
    }
    ///Bit 29 - Enable AGT1 compare match A snooze request
    #[inline(always)]
    pub fn snzreqen29(&mut self) -> Snzreqen29W<Snzreqcr0Spec> {
        Snzreqen29W::new(self, 29)
    }
    ///Bit 30 - Enable AGT1 compare match B snooze request
    #[inline(always)]
    pub fn snzreqen30(&mut self) -> Snzreqen30W<Snzreqcr0Spec> {
        Snzreqen30W::new(self, 30)
    }
}
/**Snooze Request Control Register 0

You can [`read`](crate::Reg::read) this register and get [`snzreqcr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snzreqcr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Snzreqcr0Spec;
impl crate::RegisterSpec for Snzreqcr0Spec {
    type Ux = u32;
}
///`read()` method returns [`snzreqcr0::R`](R) reader structure
impl crate::Readable for Snzreqcr0Spec {}
///`write(|w| ..)` method takes [`snzreqcr0::W`](W) writer structure
impl crate::Writable for Snzreqcr0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SNZREQCR0 to value 0
impl crate::Resettable for Snzreqcr0Spec {}
