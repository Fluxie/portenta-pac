///Register `MSTPCRD` reader
pub type R = crate::R<MstpcrdSpec>;
///Register `MSTPCRD` writer
pub type W = crate::W<MstpcrdSpec>;
/**Low Power Asynchronous General Purpose Timer 3 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpd0 {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<Mstpd0> for bool {
    #[inline(always)]
    fn from(variant: Mstpd0) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPD0` reader - Low Power Asynchronous General Purpose Timer 3 Module Stop
pub type Mstpd0R = crate::BitReader<Mstpd0>;
impl Mstpd0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mstpd0 {
        match self.bits {
            false => Mstpd0::_0,
            true => Mstpd0::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpd0::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpd0::_1
    }
}
///Field `MSTPD0` writer - Low Power Asynchronous General Purpose Timer 3 Module Stop
pub type Mstpd0W<'a, REG> = crate::BitWriter<'a, REG, Mstpd0>;
impl<'a, REG> Mstpd0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpd0::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpd0::_1)
    }
}
/**Low Power Asynchronous General Purpose Timer 2 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpd1 {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<Mstpd1> for bool {
    #[inline(always)]
    fn from(variant: Mstpd1) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPD1` reader - Low Power Asynchronous General Purpose Timer 2 Module Stop
pub type Mstpd1R = crate::BitReader<Mstpd1>;
impl Mstpd1R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mstpd1 {
        match self.bits {
            false => Mstpd1::_0,
            true => Mstpd1::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpd1::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpd1::_1
    }
}
///Field `MSTPD1` writer - Low Power Asynchronous General Purpose Timer 2 Module Stop
pub type Mstpd1W<'a, REG> = crate::BitWriter<'a, REG, Mstpd1>;
impl<'a, REG> Mstpd1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpd1::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpd1::_1)
    }
}
/**Low Power Asynchronous General Purpose Timer 1 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpd2 {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<Mstpd2> for bool {
    #[inline(always)]
    fn from(variant: Mstpd2) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPD2` reader - Low Power Asynchronous General Purpose Timer 1 Module Stop
pub type Mstpd2R = crate::BitReader<Mstpd2>;
impl Mstpd2R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mstpd2 {
        match self.bits {
            false => Mstpd2::_0,
            true => Mstpd2::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpd2::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpd2::_1
    }
}
///Field `MSTPD2` writer - Low Power Asynchronous General Purpose Timer 1 Module Stop
pub type Mstpd2W<'a, REG> = crate::BitWriter<'a, REG, Mstpd2>;
impl<'a, REG> Mstpd2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpd2::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpd2::_1)
    }
}
/**Low Power Asynchronous General Purpose Timer 0 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpd3 {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<Mstpd3> for bool {
    #[inline(always)]
    fn from(variant: Mstpd3) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPD3` reader - Low Power Asynchronous General Purpose Timer 0 Module Stop
pub type Mstpd3R = crate::BitReader<Mstpd3>;
impl Mstpd3R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mstpd3 {
        match self.bits {
            false => Mstpd3::_0,
            true => Mstpd3::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpd3::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpd3::_1
    }
}
///Field `MSTPD3` writer - Low Power Asynchronous General Purpose Timer 0 Module Stop
pub type Mstpd3W<'a, REG> = crate::BitWriter<'a, REG, Mstpd3>;
impl<'a, REG> Mstpd3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpd3::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpd3::_1)
    }
}
/**Port Output Enable for GPT Group D Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpd11 {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<Mstpd11> for bool {
    #[inline(always)]
    fn from(variant: Mstpd11) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPD11` reader - Port Output Enable for GPT Group D Module Stop
pub type Mstpd11R = crate::BitReader<Mstpd11>;
impl Mstpd11R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mstpd11 {
        match self.bits {
            false => Mstpd11::_0,
            true => Mstpd11::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpd11::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpd11::_1
    }
}
///Field `MSTPD11` writer - Port Output Enable for GPT Group D Module Stop
pub type Mstpd11W<'a, REG> = crate::BitWriter<'a, REG, Mstpd11>;
impl<'a, REG> Mstpd11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpd11::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpd11::_1)
    }
}
/**Port Output Enable for GPT Group C Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpd12 {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<Mstpd12> for bool {
    #[inline(always)]
    fn from(variant: Mstpd12) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPD12` reader - Port Output Enable for GPT Group C Module Stop
pub type Mstpd12R = crate::BitReader<Mstpd12>;
impl Mstpd12R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mstpd12 {
        match self.bits {
            false => Mstpd12::_0,
            true => Mstpd12::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpd12::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpd12::_1
    }
}
///Field `MSTPD12` writer - Port Output Enable for GPT Group C Module Stop
pub type Mstpd12W<'a, REG> = crate::BitWriter<'a, REG, Mstpd12>;
impl<'a, REG> Mstpd12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpd12::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpd12::_1)
    }
}
/**Port Output Enable for GPT Group B Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpd13 {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<Mstpd13> for bool {
    #[inline(always)]
    fn from(variant: Mstpd13) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPD13` reader - Port Output Enable for GPT Group B Module Stop
pub type Mstpd13R = crate::BitReader<Mstpd13>;
impl Mstpd13R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mstpd13 {
        match self.bits {
            false => Mstpd13::_0,
            true => Mstpd13::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpd13::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpd13::_1
    }
}
///Field `MSTPD13` writer - Port Output Enable for GPT Group B Module Stop
pub type Mstpd13W<'a, REG> = crate::BitWriter<'a, REG, Mstpd13>;
impl<'a, REG> Mstpd13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpd13::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpd13::_1)
    }
}
/**Port Output Enable for GPT Group A Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpd14 {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<Mstpd14> for bool {
    #[inline(always)]
    fn from(variant: Mstpd14) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPD14` reader - Port Output Enable for GPT Group A Module Stop
pub type Mstpd14R = crate::BitReader<Mstpd14>;
impl Mstpd14R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mstpd14 {
        match self.bits {
            false => Mstpd14::_0,
            true => Mstpd14::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpd14::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpd14::_1
    }
}
///Field `MSTPD14` writer - Port Output Enable for GPT Group A Module Stop
pub type Mstpd14W<'a, REG> = crate::BitWriter<'a, REG, Mstpd14>;
impl<'a, REG> Mstpd14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpd14::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpd14::_1)
    }
}
/**12-bit A/D Converter 1 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpd15 {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<Mstpd15> for bool {
    #[inline(always)]
    fn from(variant: Mstpd15) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPD15` reader - 12-bit A/D Converter 1 Module Stop
pub type Mstpd15R = crate::BitReader<Mstpd15>;
impl Mstpd15R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mstpd15 {
        match self.bits {
            false => Mstpd15::_0,
            true => Mstpd15::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpd15::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpd15::_1
    }
}
///Field `MSTPD15` writer - 12-bit A/D Converter 1 Module Stop
pub type Mstpd15W<'a, REG> = crate::BitWriter<'a, REG, Mstpd15>;
impl<'a, REG> Mstpd15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpd15::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpd15::_1)
    }
}
/**12-bit A/D Converter 0 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpd16 {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<Mstpd16> for bool {
    #[inline(always)]
    fn from(variant: Mstpd16) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPD16` reader - 12-bit A/D Converter 0 Module Stop
pub type Mstpd16R = crate::BitReader<Mstpd16>;
impl Mstpd16R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mstpd16 {
        match self.bits {
            false => Mstpd16::_0,
            true => Mstpd16::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpd16::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpd16::_1
    }
}
///Field `MSTPD16` writer - 12-bit A/D Converter 0 Module Stop
pub type Mstpd16W<'a, REG> = crate::BitWriter<'a, REG, Mstpd16>;
impl<'a, REG> Mstpd16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpd16::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpd16::_1)
    }
}
/**12-bit D/A Converter Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpd20 {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<Mstpd20> for bool {
    #[inline(always)]
    fn from(variant: Mstpd20) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPD20` reader - 12-bit D/A Converter Module Stop
pub type Mstpd20R = crate::BitReader<Mstpd20>;
impl Mstpd20R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mstpd20 {
        match self.bits {
            false => Mstpd20::_0,
            true => Mstpd20::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpd20::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpd20::_1
    }
}
///Field `MSTPD20` writer - 12-bit D/A Converter Module Stop
pub type Mstpd20W<'a, REG> = crate::BitWriter<'a, REG, Mstpd20>;
impl<'a, REG> Mstpd20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpd20::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpd20::_1)
    }
}
/**Temperature Sensor Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpd22 {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<Mstpd22> for bool {
    #[inline(always)]
    fn from(variant: Mstpd22) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPD22` reader - Temperature Sensor Module Stop
pub type Mstpd22R = crate::BitReader<Mstpd22>;
impl Mstpd22R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mstpd22 {
        match self.bits {
            false => Mstpd22::_0,
            true => Mstpd22::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpd22::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpd22::_1
    }
}
///Field `MSTPD22` writer - Temperature Sensor Module Stop
pub type Mstpd22W<'a, REG> = crate::BitWriter<'a, REG, Mstpd22>;
impl<'a, REG> Mstpd22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpd22::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpd22::_1)
    }
}
impl R {
    ///Bit 0 - Low Power Asynchronous General Purpose Timer 3 Module Stop
    #[inline(always)]
    pub fn mstpd0(&self) -> Mstpd0R {
        Mstpd0R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Low Power Asynchronous General Purpose Timer 2 Module Stop
    #[inline(always)]
    pub fn mstpd1(&self) -> Mstpd1R {
        Mstpd1R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Low Power Asynchronous General Purpose Timer 1 Module Stop
    #[inline(always)]
    pub fn mstpd2(&self) -> Mstpd2R {
        Mstpd2R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Low Power Asynchronous General Purpose Timer 0 Module Stop
    #[inline(always)]
    pub fn mstpd3(&self) -> Mstpd3R {
        Mstpd3R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 11 - Port Output Enable for GPT Group D Module Stop
    #[inline(always)]
    pub fn mstpd11(&self) -> Mstpd11R {
        Mstpd11R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Port Output Enable for GPT Group C Module Stop
    #[inline(always)]
    pub fn mstpd12(&self) -> Mstpd12R {
        Mstpd12R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Port Output Enable for GPT Group B Module Stop
    #[inline(always)]
    pub fn mstpd13(&self) -> Mstpd13R {
        Mstpd13R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Port Output Enable for GPT Group A Module Stop
    #[inline(always)]
    pub fn mstpd14(&self) -> Mstpd14R {
        Mstpd14R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - 12-bit A/D Converter 1 Module Stop
    #[inline(always)]
    pub fn mstpd15(&self) -> Mstpd15R {
        Mstpd15R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - 12-bit A/D Converter 0 Module Stop
    #[inline(always)]
    pub fn mstpd16(&self) -> Mstpd16R {
        Mstpd16R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 20 - 12-bit D/A Converter Module Stop
    #[inline(always)]
    pub fn mstpd20(&self) -> Mstpd20R {
        Mstpd20R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 22 - Temperature Sensor Module Stop
    #[inline(always)]
    pub fn mstpd22(&self) -> Mstpd22R {
        Mstpd22R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MSTPCRD")
            .field("mstpd0", &self.mstpd0())
            .field("mstpd1", &self.mstpd1())
            .field("mstpd2", &self.mstpd2())
            .field("mstpd3", &self.mstpd3())
            .field("mstpd11", &self.mstpd11())
            .field("mstpd12", &self.mstpd12())
            .field("mstpd13", &self.mstpd13())
            .field("mstpd14", &self.mstpd14())
            .field("mstpd15", &self.mstpd15())
            .field("mstpd16", &self.mstpd16())
            .field("mstpd20", &self.mstpd20())
            .field("mstpd22", &self.mstpd22())
            .finish()
    }
}
impl W {
    ///Bit 0 - Low Power Asynchronous General Purpose Timer 3 Module Stop
    #[inline(always)]
    pub fn mstpd0(&mut self) -> Mstpd0W<MstpcrdSpec> {
        Mstpd0W::new(self, 0)
    }
    ///Bit 1 - Low Power Asynchronous General Purpose Timer 2 Module Stop
    #[inline(always)]
    pub fn mstpd1(&mut self) -> Mstpd1W<MstpcrdSpec> {
        Mstpd1W::new(self, 1)
    }
    ///Bit 2 - Low Power Asynchronous General Purpose Timer 1 Module Stop
    #[inline(always)]
    pub fn mstpd2(&mut self) -> Mstpd2W<MstpcrdSpec> {
        Mstpd2W::new(self, 2)
    }
    ///Bit 3 - Low Power Asynchronous General Purpose Timer 0 Module Stop
    #[inline(always)]
    pub fn mstpd3(&mut self) -> Mstpd3W<MstpcrdSpec> {
        Mstpd3W::new(self, 3)
    }
    ///Bit 11 - Port Output Enable for GPT Group D Module Stop
    #[inline(always)]
    pub fn mstpd11(&mut self) -> Mstpd11W<MstpcrdSpec> {
        Mstpd11W::new(self, 11)
    }
    ///Bit 12 - Port Output Enable for GPT Group C Module Stop
    #[inline(always)]
    pub fn mstpd12(&mut self) -> Mstpd12W<MstpcrdSpec> {
        Mstpd12W::new(self, 12)
    }
    ///Bit 13 - Port Output Enable for GPT Group B Module Stop
    #[inline(always)]
    pub fn mstpd13(&mut self) -> Mstpd13W<MstpcrdSpec> {
        Mstpd13W::new(self, 13)
    }
    ///Bit 14 - Port Output Enable for GPT Group A Module Stop
    #[inline(always)]
    pub fn mstpd14(&mut self) -> Mstpd14W<MstpcrdSpec> {
        Mstpd14W::new(self, 14)
    }
    ///Bit 15 - 12-bit A/D Converter 1 Module Stop
    #[inline(always)]
    pub fn mstpd15(&mut self) -> Mstpd15W<MstpcrdSpec> {
        Mstpd15W::new(self, 15)
    }
    ///Bit 16 - 12-bit A/D Converter 0 Module Stop
    #[inline(always)]
    pub fn mstpd16(&mut self) -> Mstpd16W<MstpcrdSpec> {
        Mstpd16W::new(self, 16)
    }
    ///Bit 20 - 12-bit D/A Converter Module Stop
    #[inline(always)]
    pub fn mstpd20(&mut self) -> Mstpd20W<MstpcrdSpec> {
        Mstpd20W::new(self, 20)
    }
    ///Bit 22 - Temperature Sensor Module Stop
    #[inline(always)]
    pub fn mstpd22(&mut self) -> Mstpd22W<MstpcrdSpec> {
        Mstpd22W::new(self, 22)
    }
}
/**Module Stop Control Register D

You can [`read`](crate::Reg::read) this register and get [`mstpcrd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mstpcrd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MstpcrdSpec;
impl crate::RegisterSpec for MstpcrdSpec {
    type Ux = u32;
}
///`read()` method returns [`mstpcrd::R`](R) reader structure
impl crate::Readable for MstpcrdSpec {}
///`write(|w| ..)` method takes [`mstpcrd::W`](W) writer structure
impl crate::Writable for MstpcrdSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MSTPCRD to value 0xffff_ffff
impl crate::Resettable for MstpcrdSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
