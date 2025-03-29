///Register `DPFSAR` reader
pub type R = crate::R<DpfsarSpec>;
///Register `DPFSAR` writer
pub type W = crate::W<DpfsarSpec>;
/**Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dpfsa0 {
    ///0: Secure
    _0 = 0,
    ///1: Non Secure
    _1 = 1,
}
impl From<Dpfsa0> for bool {
    #[inline(always)]
    fn from(variant: Dpfsa0) -> Self {
        variant as u8 != 0
    }
}
///Field `DPFSA0` reader - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)
pub type Dpfsa0R = crate::BitReader<Dpfsa0>;
impl Dpfsa0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dpfsa0 {
        match self.bits {
            false => Dpfsa0::_0,
            true => Dpfsa0::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dpfsa0::_0
    }
    ///Non Secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dpfsa0::_1
    }
}
///Field `DPFSA0` writer - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)
pub type Dpfsa0W<'a, REG> = crate::BitWriter<'a, REG, Dpfsa0>;
impl<'a, REG> Dpfsa0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dpfsa0::_0)
    }
    ///Non Secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dpfsa0::_1)
    }
}
/**Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dpfsa1 {
    ///0: Secure
    _0 = 0,
    ///1: Non Secure
    _1 = 1,
}
impl From<Dpfsa1> for bool {
    #[inline(always)]
    fn from(variant: Dpfsa1) -> Self {
        variant as u8 != 0
    }
}
///Field `DPFSA1` reader - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)
pub type Dpfsa1R = crate::BitReader<Dpfsa1>;
impl Dpfsa1R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dpfsa1 {
        match self.bits {
            false => Dpfsa1::_0,
            true => Dpfsa1::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dpfsa1::_0
    }
    ///Non Secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dpfsa1::_1
    }
}
///Field `DPFSA1` writer - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)
pub type Dpfsa1W<'a, REG> = crate::BitWriter<'a, REG, Dpfsa1>;
impl<'a, REG> Dpfsa1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dpfsa1::_0)
    }
    ///Non Secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dpfsa1::_1)
    }
}
/**Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dpfsa2 {
    ///0: Secure
    _0 = 0,
    ///1: Non Secure
    _1 = 1,
}
impl From<Dpfsa2> for bool {
    #[inline(always)]
    fn from(variant: Dpfsa2) -> Self {
        variant as u8 != 0
    }
}
///Field `DPFSA2` reader - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)
pub type Dpfsa2R = crate::BitReader<Dpfsa2>;
impl Dpfsa2R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dpfsa2 {
        match self.bits {
            false => Dpfsa2::_0,
            true => Dpfsa2::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dpfsa2::_0
    }
    ///Non Secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dpfsa2::_1
    }
}
///Field `DPFSA2` writer - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)
pub type Dpfsa2W<'a, REG> = crate::BitWriter<'a, REG, Dpfsa2>;
impl<'a, REG> Dpfsa2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dpfsa2::_0)
    }
    ///Non Secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dpfsa2::_1)
    }
}
/**Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dpfsa3 {
    ///0: Secure
    _0 = 0,
    ///1: Non Secure
    _1 = 1,
}
impl From<Dpfsa3> for bool {
    #[inline(always)]
    fn from(variant: Dpfsa3) -> Self {
        variant as u8 != 0
    }
}
///Field `DPFSA3` reader - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)
pub type Dpfsa3R = crate::BitReader<Dpfsa3>;
impl Dpfsa3R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dpfsa3 {
        match self.bits {
            false => Dpfsa3::_0,
            true => Dpfsa3::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dpfsa3::_0
    }
    ///Non Secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dpfsa3::_1
    }
}
///Field `DPFSA3` writer - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)
pub type Dpfsa3W<'a, REG> = crate::BitWriter<'a, REG, Dpfsa3>;
impl<'a, REG> Dpfsa3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dpfsa3::_0)
    }
    ///Non Secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dpfsa3::_1)
    }
}
/**Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dpfsa4 {
    ///0: Secure
    _0 = 0,
    ///1: Non Secure
    _1 = 1,
}
impl From<Dpfsa4> for bool {
    #[inline(always)]
    fn from(variant: Dpfsa4) -> Self {
        variant as u8 != 0
    }
}
///Field `DPFSA4` reader - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)
pub type Dpfsa4R = crate::BitReader<Dpfsa4>;
impl Dpfsa4R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dpfsa4 {
        match self.bits {
            false => Dpfsa4::_0,
            true => Dpfsa4::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dpfsa4::_0
    }
    ///Non Secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dpfsa4::_1
    }
}
///Field `DPFSA4` writer - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)
pub type Dpfsa4W<'a, REG> = crate::BitWriter<'a, REG, Dpfsa4>;
impl<'a, REG> Dpfsa4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dpfsa4::_0)
    }
    ///Non Secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dpfsa4::_1)
    }
}
/**Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dpfsa5 {
    ///0: Secure
    _0 = 0,
    ///1: Non Secure
    _1 = 1,
}
impl From<Dpfsa5> for bool {
    #[inline(always)]
    fn from(variant: Dpfsa5) -> Self {
        variant as u8 != 0
    }
}
///Field `DPFSA5` reader - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)
pub type Dpfsa5R = crate::BitReader<Dpfsa5>;
impl Dpfsa5R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dpfsa5 {
        match self.bits {
            false => Dpfsa5::_0,
            true => Dpfsa5::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dpfsa5::_0
    }
    ///Non Secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dpfsa5::_1
    }
}
///Field `DPFSA5` writer - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)
pub type Dpfsa5W<'a, REG> = crate::BitWriter<'a, REG, Dpfsa5>;
impl<'a, REG> Dpfsa5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dpfsa5::_0)
    }
    ///Non Secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dpfsa5::_1)
    }
}
/**Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dpfsa6 {
    ///0: Secure
    _0 = 0,
    ///1: Non Secure
    _1 = 1,
}
impl From<Dpfsa6> for bool {
    #[inline(always)]
    fn from(variant: Dpfsa6) -> Self {
        variant as u8 != 0
    }
}
///Field `DPFSA6` reader - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)
pub type Dpfsa6R = crate::BitReader<Dpfsa6>;
impl Dpfsa6R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dpfsa6 {
        match self.bits {
            false => Dpfsa6::_0,
            true => Dpfsa6::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dpfsa6::_0
    }
    ///Non Secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dpfsa6::_1
    }
}
///Field `DPFSA6` writer - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)
pub type Dpfsa6W<'a, REG> = crate::BitWriter<'a, REG, Dpfsa6>;
impl<'a, REG> Dpfsa6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dpfsa6::_0)
    }
    ///Non Secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dpfsa6::_1)
    }
}
/**Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dpfsa7 {
    ///0: Secure
    _0 = 0,
    ///1: Non Secure
    _1 = 1,
}
impl From<Dpfsa7> for bool {
    #[inline(always)]
    fn from(variant: Dpfsa7) -> Self {
        variant as u8 != 0
    }
}
///Field `DPFSA7` reader - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)
pub type Dpfsa7R = crate::BitReader<Dpfsa7>;
impl Dpfsa7R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dpfsa7 {
        match self.bits {
            false => Dpfsa7::_0,
            true => Dpfsa7::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dpfsa7::_0
    }
    ///Non Secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dpfsa7::_1
    }
}
///Field `DPFSA7` writer - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)
pub type Dpfsa7W<'a, REG> = crate::BitWriter<'a, REG, Dpfsa7>;
impl<'a, REG> Dpfsa7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dpfsa7::_0)
    }
    ///Non Secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dpfsa7::_1)
    }
}
/**Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dpfsa08 {
    ///0: Secure
    _0 = 0,
    ///1: Non Secure
    _1 = 1,
}
impl From<Dpfsa08> for bool {
    #[inline(always)]
    fn from(variant: Dpfsa08) -> Self {
        variant as u8 != 0
    }
}
///Field `DPFSA08` reader - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)
pub type Dpfsa08R = crate::BitReader<Dpfsa08>;
impl Dpfsa08R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dpfsa08 {
        match self.bits {
            false => Dpfsa08::_0,
            true => Dpfsa08::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dpfsa08::_0
    }
    ///Non Secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dpfsa08::_1
    }
}
///Field `DPFSA08` writer - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)
pub type Dpfsa08W<'a, REG> = crate::BitWriter<'a, REG, Dpfsa08>;
impl<'a, REG> Dpfsa08W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dpfsa08::_0)
    }
    ///Non Secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dpfsa08::_1)
    }
}
/**Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dpfsa09 {
    ///0: Secure
    _0 = 0,
    ///1: Non Secure
    _1 = 1,
}
impl From<Dpfsa09> for bool {
    #[inline(always)]
    fn from(variant: Dpfsa09) -> Self {
        variant as u8 != 0
    }
}
///Field `DPFSA09` reader - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)
pub type Dpfsa09R = crate::BitReader<Dpfsa09>;
impl Dpfsa09R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dpfsa09 {
        match self.bits {
            false => Dpfsa09::_0,
            true => Dpfsa09::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dpfsa09::_0
    }
    ///Non Secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dpfsa09::_1
    }
}
///Field `DPFSA09` writer - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)
pub type Dpfsa09W<'a, REG> = crate::BitWriter<'a, REG, Dpfsa09>;
impl<'a, REG> Dpfsa09W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dpfsa09::_0)
    }
    ///Non Secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dpfsa09::_1)
    }
}
/**Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dpfsa10 {
    ///0: Secure
    _0 = 0,
    ///1: Non Secure
    _1 = 1,
}
impl From<Dpfsa10> for bool {
    #[inline(always)]
    fn from(variant: Dpfsa10) -> Self {
        variant as u8 != 0
    }
}
///Field `DPFSA10` reader - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)
pub type Dpfsa10R = crate::BitReader<Dpfsa10>;
impl Dpfsa10R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dpfsa10 {
        match self.bits {
            false => Dpfsa10::_0,
            true => Dpfsa10::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dpfsa10::_0
    }
    ///Non Secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dpfsa10::_1
    }
}
///Field `DPFSA10` writer - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)
pub type Dpfsa10W<'a, REG> = crate::BitWriter<'a, REG, Dpfsa10>;
impl<'a, REG> Dpfsa10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dpfsa10::_0)
    }
    ///Non Secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dpfsa10::_1)
    }
}
/**Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dpfsa11 {
    ///0: Secure
    _0 = 0,
    ///1: Non Secure
    _1 = 1,
}
impl From<Dpfsa11> for bool {
    #[inline(always)]
    fn from(variant: Dpfsa11) -> Self {
        variant as u8 != 0
    }
}
///Field `DPFSA11` reader - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)
pub type Dpfsa11R = crate::BitReader<Dpfsa11>;
impl Dpfsa11R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dpfsa11 {
        match self.bits {
            false => Dpfsa11::_0,
            true => Dpfsa11::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dpfsa11::_0
    }
    ///Non Secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dpfsa11::_1
    }
}
///Field `DPFSA11` writer - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)
pub type Dpfsa11W<'a, REG> = crate::BitWriter<'a, REG, Dpfsa11>;
impl<'a, REG> Dpfsa11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dpfsa11::_0)
    }
    ///Non Secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dpfsa11::_1)
    }
}
/**Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dpfsa12 {
    ///0: Secure
    _0 = 0,
    ///1: Non Secure
    _1 = 1,
}
impl From<Dpfsa12> for bool {
    #[inline(always)]
    fn from(variant: Dpfsa12) -> Self {
        variant as u8 != 0
    }
}
///Field `DPFSA12` reader - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)
pub type Dpfsa12R = crate::BitReader<Dpfsa12>;
impl Dpfsa12R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dpfsa12 {
        match self.bits {
            false => Dpfsa12::_0,
            true => Dpfsa12::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dpfsa12::_0
    }
    ///Non Secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dpfsa12::_1
    }
}
///Field `DPFSA12` writer - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)
pub type Dpfsa12W<'a, REG> = crate::BitWriter<'a, REG, Dpfsa12>;
impl<'a, REG> Dpfsa12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dpfsa12::_0)
    }
    ///Non Secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dpfsa12::_1)
    }
}
/**Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dpfsa13 {
    ///0: Secure
    _0 = 0,
    ///1: Non Secure
    _1 = 1,
}
impl From<Dpfsa13> for bool {
    #[inline(always)]
    fn from(variant: Dpfsa13) -> Self {
        variant as u8 != 0
    }
}
///Field `DPFSA13` reader - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)
pub type Dpfsa13R = crate::BitReader<Dpfsa13>;
impl Dpfsa13R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dpfsa13 {
        match self.bits {
            false => Dpfsa13::_0,
            true => Dpfsa13::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dpfsa13::_0
    }
    ///Non Secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dpfsa13::_1
    }
}
///Field `DPFSA13` writer - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)
pub type Dpfsa13W<'a, REG> = crate::BitWriter<'a, REG, Dpfsa13>;
impl<'a, REG> Dpfsa13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dpfsa13::_0)
    }
    ///Non Secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dpfsa13::_1)
    }
}
/**Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dpfsa14 {
    ///0: Secure
    _0 = 0,
    ///1: Non Secure
    _1 = 1,
}
impl From<Dpfsa14> for bool {
    #[inline(always)]
    fn from(variant: Dpfsa14) -> Self {
        variant as u8 != 0
    }
}
///Field `DPFSA14` reader - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)
pub type Dpfsa14R = crate::BitReader<Dpfsa14>;
impl Dpfsa14R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dpfsa14 {
        match self.bits {
            false => Dpfsa14::_0,
            true => Dpfsa14::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dpfsa14::_0
    }
    ///Non Secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dpfsa14::_1
    }
}
///Field `DPFSA14` writer - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)
pub type Dpfsa14W<'a, REG> = crate::BitWriter<'a, REG, Dpfsa14>;
impl<'a, REG> Dpfsa14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dpfsa14::_0)
    }
    ///Non Secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dpfsa14::_1)
    }
}
/**Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dpfsa15 {
    ///0: Secure
    _0 = 0,
    ///1: Non Secure
    _1 = 1,
}
impl From<Dpfsa15> for bool {
    #[inline(always)]
    fn from(variant: Dpfsa15) -> Self {
        variant as u8 != 0
    }
}
///Field `DPFSA15` reader - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)
pub type Dpfsa15R = crate::BitReader<Dpfsa15>;
impl Dpfsa15R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dpfsa15 {
        match self.bits {
            false => Dpfsa15::_0,
            true => Dpfsa15::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dpfsa15::_0
    }
    ///Non Secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dpfsa15::_1
    }
}
///Field `DPFSA15` writer - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)
pub type Dpfsa15W<'a, REG> = crate::BitWriter<'a, REG, Dpfsa15>;
impl<'a, REG> Dpfsa15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dpfsa15::_0)
    }
    ///Non Secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dpfsa15::_1)
    }
}
/**Deep Software Standby Interrupt Factor Security Attribute bit 16

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dpfsa16 {
    ///0: Secure
    _0 = 0,
    ///1: Non Secure
    _1 = 1,
}
impl From<Dpfsa16> for bool {
    #[inline(always)]
    fn from(variant: Dpfsa16) -> Self {
        variant as u8 != 0
    }
}
///Field `DPFSA16` reader - Deep Software Standby Interrupt Factor Security Attribute bit 16
pub type Dpfsa16R = crate::BitReader<Dpfsa16>;
impl Dpfsa16R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dpfsa16 {
        match self.bits {
            false => Dpfsa16::_0,
            true => Dpfsa16::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dpfsa16::_0
    }
    ///Non Secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dpfsa16::_1
    }
}
///Field `DPFSA16` writer - Deep Software Standby Interrupt Factor Security Attribute bit 16
pub type Dpfsa16W<'a, REG> = crate::BitWriter<'a, REG, Dpfsa16>;
impl<'a, REG> Dpfsa16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dpfsa16::_0)
    }
    ///Non Secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dpfsa16::_1)
    }
}
/**Deep Software Standby Interrupt Factor Security Attribute bit 17

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dpfsa17 {
    ///0: Secure
    _0 = 0,
    ///1: Non Secure
    _1 = 1,
}
impl From<Dpfsa17> for bool {
    #[inline(always)]
    fn from(variant: Dpfsa17) -> Self {
        variant as u8 != 0
    }
}
///Field `DPFSA17` reader - Deep Software Standby Interrupt Factor Security Attribute bit 17
pub type Dpfsa17R = crate::BitReader<Dpfsa17>;
impl Dpfsa17R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dpfsa17 {
        match self.bits {
            false => Dpfsa17::_0,
            true => Dpfsa17::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dpfsa17::_0
    }
    ///Non Secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dpfsa17::_1
    }
}
///Field `DPFSA17` writer - Deep Software Standby Interrupt Factor Security Attribute bit 17
pub type Dpfsa17W<'a, REG> = crate::BitWriter<'a, REG, Dpfsa17>;
impl<'a, REG> Dpfsa17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dpfsa17::_0)
    }
    ///Non Secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dpfsa17::_1)
    }
}
/**Deep Software Standby Interrupt Factor Security Attribute bit 18

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dpfsa18 {
    ///0: Secure
    _0 = 0,
    ///1: Non Secure
    _1 = 1,
}
impl From<Dpfsa18> for bool {
    #[inline(always)]
    fn from(variant: Dpfsa18) -> Self {
        variant as u8 != 0
    }
}
///Field `DPFSA18` reader - Deep Software Standby Interrupt Factor Security Attribute bit 18
pub type Dpfsa18R = crate::BitReader<Dpfsa18>;
impl Dpfsa18R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dpfsa18 {
        match self.bits {
            false => Dpfsa18::_0,
            true => Dpfsa18::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dpfsa18::_0
    }
    ///Non Secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dpfsa18::_1
    }
}
///Field `DPFSA18` writer - Deep Software Standby Interrupt Factor Security Attribute bit 18
pub type Dpfsa18W<'a, REG> = crate::BitWriter<'a, REG, Dpfsa18>;
impl<'a, REG> Dpfsa18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dpfsa18::_0)
    }
    ///Non Secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dpfsa18::_1)
    }
}
/**Deep Software Standby Interrupt Factor Security Attribute bit 19

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dpfsa19 {
    ///0: Secure
    _0 = 0,
    ///1: Non Secure
    _1 = 1,
}
impl From<Dpfsa19> for bool {
    #[inline(always)]
    fn from(variant: Dpfsa19) -> Self {
        variant as u8 != 0
    }
}
///Field `DPFSA19` reader - Deep Software Standby Interrupt Factor Security Attribute bit 19
pub type Dpfsa19R = crate::BitReader<Dpfsa19>;
impl Dpfsa19R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dpfsa19 {
        match self.bits {
            false => Dpfsa19::_0,
            true => Dpfsa19::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dpfsa19::_0
    }
    ///Non Secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dpfsa19::_1
    }
}
///Field `DPFSA19` writer - Deep Software Standby Interrupt Factor Security Attribute bit 19
pub type Dpfsa19W<'a, REG> = crate::BitWriter<'a, REG, Dpfsa19>;
impl<'a, REG> Dpfsa19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dpfsa19::_0)
    }
    ///Non Secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dpfsa19::_1)
    }
}
/**Deep Software Standby Interrupt Factor Security Attribute bit 20

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dpfsa20 {
    ///0: Secure
    _0 = 0,
    ///1: Non Secure
    _1 = 1,
}
impl From<Dpfsa20> for bool {
    #[inline(always)]
    fn from(variant: Dpfsa20) -> Self {
        variant as u8 != 0
    }
}
///Field `DPFSA20` reader - Deep Software Standby Interrupt Factor Security Attribute bit 20
pub type Dpfsa20R = crate::BitReader<Dpfsa20>;
impl Dpfsa20R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dpfsa20 {
        match self.bits {
            false => Dpfsa20::_0,
            true => Dpfsa20::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dpfsa20::_0
    }
    ///Non Secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dpfsa20::_1
    }
}
///Field `DPFSA20` writer - Deep Software Standby Interrupt Factor Security Attribute bit 20
pub type Dpfsa20W<'a, REG> = crate::BitWriter<'a, REG, Dpfsa20>;
impl<'a, REG> Dpfsa20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dpfsa20::_0)
    }
    ///Non Secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dpfsa20::_1)
    }
}
/**Deep Software Standby Interrupt Factor Security Attribute bit 24

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dpfsa24 {
    ///0: Secure
    _0 = 0,
    ///1: Non Secure
    _1 = 1,
}
impl From<Dpfsa24> for bool {
    #[inline(always)]
    fn from(variant: Dpfsa24) -> Self {
        variant as u8 != 0
    }
}
///Field `DPFSA24` reader - Deep Software Standby Interrupt Factor Security Attribute bit 24
pub type Dpfsa24R = crate::BitReader<Dpfsa24>;
impl Dpfsa24R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dpfsa24 {
        match self.bits {
            false => Dpfsa24::_0,
            true => Dpfsa24::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dpfsa24::_0
    }
    ///Non Secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dpfsa24::_1
    }
}
///Field `DPFSA24` writer - Deep Software Standby Interrupt Factor Security Attribute bit 24
pub type Dpfsa24W<'a, REG> = crate::BitWriter<'a, REG, Dpfsa24>;
impl<'a, REG> Dpfsa24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dpfsa24::_0)
    }
    ///Non Secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dpfsa24::_1)
    }
}
/**Deep Software Standby Interrupt Factor Security Attribute bit 25

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dpfsa25 {
    ///0: Secure
    _0 = 0,
    ///1: Non Secure
    _1 = 1,
}
impl From<Dpfsa25> for bool {
    #[inline(always)]
    fn from(variant: Dpfsa25) -> Self {
        variant as u8 != 0
    }
}
///Field `DPFSA25` reader - Deep Software Standby Interrupt Factor Security Attribute bit 25
pub type Dpfsa25R = crate::BitReader<Dpfsa25>;
impl Dpfsa25R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dpfsa25 {
        match self.bits {
            false => Dpfsa25::_0,
            true => Dpfsa25::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dpfsa25::_0
    }
    ///Non Secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dpfsa25::_1
    }
}
///Field `DPFSA25` writer - Deep Software Standby Interrupt Factor Security Attribute bit 25
pub type Dpfsa25W<'a, REG> = crate::BitWriter<'a, REG, Dpfsa25>;
impl<'a, REG> Dpfsa25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dpfsa25::_0)
    }
    ///Non Secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dpfsa25::_1)
    }
}
/**Deep Software Standby Interrupt Factor Security Attribute bit 26

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dpfsa26 {
    ///0: Secure
    _0 = 0,
    ///1: Non Secure
    _1 = 1,
}
impl From<Dpfsa26> for bool {
    #[inline(always)]
    fn from(variant: Dpfsa26) -> Self {
        variant as u8 != 0
    }
}
///Field `DPFSA26` reader - Deep Software Standby Interrupt Factor Security Attribute bit 26
pub type Dpfsa26R = crate::BitReader<Dpfsa26>;
impl Dpfsa26R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dpfsa26 {
        match self.bits {
            false => Dpfsa26::_0,
            true => Dpfsa26::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dpfsa26::_0
    }
    ///Non Secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dpfsa26::_1
    }
}
///Field `DPFSA26` writer - Deep Software Standby Interrupt Factor Security Attribute bit 26
pub type Dpfsa26W<'a, REG> = crate::BitWriter<'a, REG, Dpfsa26>;
impl<'a, REG> Dpfsa26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dpfsa26::_0)
    }
    ///Non Secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dpfsa26::_1)
    }
}
/**Deep Software Standby Interrupt Factor Security Attribute bit 27

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dpfsa27 {
    ///0: Secure
    _0 = 0,
    ///1: Non Secure
    _1 = 1,
}
impl From<Dpfsa27> for bool {
    #[inline(always)]
    fn from(variant: Dpfsa27) -> Self {
        variant as u8 != 0
    }
}
///Field `DPFSA27` reader - Deep Software Standby Interrupt Factor Security Attribute bit 27
pub type Dpfsa27R = crate::BitReader<Dpfsa27>;
impl Dpfsa27R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dpfsa27 {
        match self.bits {
            false => Dpfsa27::_0,
            true => Dpfsa27::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dpfsa27::_0
    }
    ///Non Secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dpfsa27::_1
    }
}
///Field `DPFSA27` writer - Deep Software Standby Interrupt Factor Security Attribute bit 27
pub type Dpfsa27W<'a, REG> = crate::BitWriter<'a, REG, Dpfsa27>;
impl<'a, REG> Dpfsa27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dpfsa27::_0)
    }
    ///Non Secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dpfsa27::_1)
    }
}
impl R {
    ///Bit 0 - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)
    #[inline(always)]
    pub fn dpfsa0(&self) -> Dpfsa0R {
        Dpfsa0R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)
    #[inline(always)]
    pub fn dpfsa1(&self) -> Dpfsa1R {
        Dpfsa1R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)
    #[inline(always)]
    pub fn dpfsa2(&self) -> Dpfsa2R {
        Dpfsa2R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)
    #[inline(always)]
    pub fn dpfsa3(&self) -> Dpfsa3R {
        Dpfsa3R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)
    #[inline(always)]
    pub fn dpfsa4(&self) -> Dpfsa4R {
        Dpfsa4R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)
    #[inline(always)]
    pub fn dpfsa5(&self) -> Dpfsa5R {
        Dpfsa5R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)
    #[inline(always)]
    pub fn dpfsa6(&self) -> Dpfsa6R {
        Dpfsa6R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)
    #[inline(always)]
    pub fn dpfsa7(&self) -> Dpfsa7R {
        Dpfsa7R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)
    #[inline(always)]
    pub fn dpfsa08(&self) -> Dpfsa08R {
        Dpfsa08R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)
    #[inline(always)]
    pub fn dpfsa09(&self) -> Dpfsa09R {
        Dpfsa09R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)
    #[inline(always)]
    pub fn dpfsa10(&self) -> Dpfsa10R {
        Dpfsa10R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)
    #[inline(always)]
    pub fn dpfsa11(&self) -> Dpfsa11R {
        Dpfsa11R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)
    #[inline(always)]
    pub fn dpfsa12(&self) -> Dpfsa12R {
        Dpfsa12R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)
    #[inline(always)]
    pub fn dpfsa13(&self) -> Dpfsa13R {
        Dpfsa13R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)
    #[inline(always)]
    pub fn dpfsa14(&self) -> Dpfsa14R {
        Dpfsa14R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)
    #[inline(always)]
    pub fn dpfsa15(&self) -> Dpfsa15R {
        Dpfsa15R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Deep Software Standby Interrupt Factor Security Attribute bit 16
    #[inline(always)]
    pub fn dpfsa16(&self) -> Dpfsa16R {
        Dpfsa16R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Deep Software Standby Interrupt Factor Security Attribute bit 17
    #[inline(always)]
    pub fn dpfsa17(&self) -> Dpfsa17R {
        Dpfsa17R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Deep Software Standby Interrupt Factor Security Attribute bit 18
    #[inline(always)]
    pub fn dpfsa18(&self) -> Dpfsa18R {
        Dpfsa18R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Deep Software Standby Interrupt Factor Security Attribute bit 19
    #[inline(always)]
    pub fn dpfsa19(&self) -> Dpfsa19R {
        Dpfsa19R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Deep Software Standby Interrupt Factor Security Attribute bit 20
    #[inline(always)]
    pub fn dpfsa20(&self) -> Dpfsa20R {
        Dpfsa20R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 24 - Deep Software Standby Interrupt Factor Security Attribute bit 24
    #[inline(always)]
    pub fn dpfsa24(&self) -> Dpfsa24R {
        Dpfsa24R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Deep Software Standby Interrupt Factor Security Attribute bit 25
    #[inline(always)]
    pub fn dpfsa25(&self) -> Dpfsa25R {
        Dpfsa25R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Deep Software Standby Interrupt Factor Security Attribute bit 26
    #[inline(always)]
    pub fn dpfsa26(&self) -> Dpfsa26R {
        Dpfsa26R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Deep Software Standby Interrupt Factor Security Attribute bit 27
    #[inline(always)]
    pub fn dpfsa27(&self) -> Dpfsa27R {
        Dpfsa27R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DPFSAR")
            .field("dpfsa0", &self.dpfsa0())
            .field("dpfsa1", &self.dpfsa1())
            .field("dpfsa2", &self.dpfsa2())
            .field("dpfsa3", &self.dpfsa3())
            .field("dpfsa4", &self.dpfsa4())
            .field("dpfsa5", &self.dpfsa5())
            .field("dpfsa6", &self.dpfsa6())
            .field("dpfsa7", &self.dpfsa7())
            .field("dpfsa08", &self.dpfsa08())
            .field("dpfsa09", &self.dpfsa09())
            .field("dpfsa10", &self.dpfsa10())
            .field("dpfsa11", &self.dpfsa11())
            .field("dpfsa12", &self.dpfsa12())
            .field("dpfsa13", &self.dpfsa13())
            .field("dpfsa14", &self.dpfsa14())
            .field("dpfsa15", &self.dpfsa15())
            .field("dpfsa16", &self.dpfsa16())
            .field("dpfsa17", &self.dpfsa17())
            .field("dpfsa18", &self.dpfsa18())
            .field("dpfsa19", &self.dpfsa19())
            .field("dpfsa20", &self.dpfsa20())
            .field("dpfsa24", &self.dpfsa24())
            .field("dpfsa25", &self.dpfsa25())
            .field("dpfsa26", &self.dpfsa26())
            .field("dpfsa27", &self.dpfsa27())
            .finish()
    }
}
impl W {
    ///Bit 0 - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)
    #[inline(always)]
    pub fn dpfsa0(&mut self) -> Dpfsa0W<DpfsarSpec> {
        Dpfsa0W::new(self, 0)
    }
    ///Bit 1 - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)
    #[inline(always)]
    pub fn dpfsa1(&mut self) -> Dpfsa1W<DpfsarSpec> {
        Dpfsa1W::new(self, 1)
    }
    ///Bit 2 - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)
    #[inline(always)]
    pub fn dpfsa2(&mut self) -> Dpfsa2W<DpfsarSpec> {
        Dpfsa2W::new(self, 2)
    }
    ///Bit 3 - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)
    #[inline(always)]
    pub fn dpfsa3(&mut self) -> Dpfsa3W<DpfsarSpec> {
        Dpfsa3W::new(self, 3)
    }
    ///Bit 4 - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)
    #[inline(always)]
    pub fn dpfsa4(&mut self) -> Dpfsa4W<DpfsarSpec> {
        Dpfsa4W::new(self, 4)
    }
    ///Bit 5 - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)
    #[inline(always)]
    pub fn dpfsa5(&mut self) -> Dpfsa5W<DpfsarSpec> {
        Dpfsa5W::new(self, 5)
    }
    ///Bit 6 - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)
    #[inline(always)]
    pub fn dpfsa6(&mut self) -> Dpfsa6W<DpfsarSpec> {
        Dpfsa6W::new(self, 6)
    }
    ///Bit 7 - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 0 to 7)
    #[inline(always)]
    pub fn dpfsa7(&mut self) -> Dpfsa7W<DpfsarSpec> {
        Dpfsa7W::new(self, 7)
    }
    ///Bit 8 - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)
    #[inline(always)]
    pub fn dpfsa08(&mut self) -> Dpfsa08W<DpfsarSpec> {
        Dpfsa08W::new(self, 8)
    }
    ///Bit 9 - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)
    #[inline(always)]
    pub fn dpfsa09(&mut self) -> Dpfsa09W<DpfsarSpec> {
        Dpfsa09W::new(self, 9)
    }
    ///Bit 10 - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)
    #[inline(always)]
    pub fn dpfsa10(&mut self) -> Dpfsa10W<DpfsarSpec> {
        Dpfsa10W::new(self, 10)
    }
    ///Bit 11 - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)
    #[inline(always)]
    pub fn dpfsa11(&mut self) -> Dpfsa11W<DpfsarSpec> {
        Dpfsa11W::new(self, 11)
    }
    ///Bit 12 - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)
    #[inline(always)]
    pub fn dpfsa12(&mut self) -> Dpfsa12W<DpfsarSpec> {
        Dpfsa12W::new(self, 12)
    }
    ///Bit 13 - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)
    #[inline(always)]
    pub fn dpfsa13(&mut self) -> Dpfsa13W<DpfsarSpec> {
        Dpfsa13W::new(self, 13)
    }
    ///Bit 14 - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)
    #[inline(always)]
    pub fn dpfsa14(&mut self) -> Dpfsa14W<DpfsarSpec> {
        Dpfsa14W::new(self, 14)
    }
    ///Bit 15 - Deep Software Standby Interrupt Factor Security Attribute bit n (n = 8 to 15)
    #[inline(always)]
    pub fn dpfsa15(&mut self) -> Dpfsa15W<DpfsarSpec> {
        Dpfsa15W::new(self, 15)
    }
    ///Bit 16 - Deep Software Standby Interrupt Factor Security Attribute bit 16
    #[inline(always)]
    pub fn dpfsa16(&mut self) -> Dpfsa16W<DpfsarSpec> {
        Dpfsa16W::new(self, 16)
    }
    ///Bit 17 - Deep Software Standby Interrupt Factor Security Attribute bit 17
    #[inline(always)]
    pub fn dpfsa17(&mut self) -> Dpfsa17W<DpfsarSpec> {
        Dpfsa17W::new(self, 17)
    }
    ///Bit 18 - Deep Software Standby Interrupt Factor Security Attribute bit 18
    #[inline(always)]
    pub fn dpfsa18(&mut self) -> Dpfsa18W<DpfsarSpec> {
        Dpfsa18W::new(self, 18)
    }
    ///Bit 19 - Deep Software Standby Interrupt Factor Security Attribute bit 19
    #[inline(always)]
    pub fn dpfsa19(&mut self) -> Dpfsa19W<DpfsarSpec> {
        Dpfsa19W::new(self, 19)
    }
    ///Bit 20 - Deep Software Standby Interrupt Factor Security Attribute bit 20
    #[inline(always)]
    pub fn dpfsa20(&mut self) -> Dpfsa20W<DpfsarSpec> {
        Dpfsa20W::new(self, 20)
    }
    ///Bit 24 - Deep Software Standby Interrupt Factor Security Attribute bit 24
    #[inline(always)]
    pub fn dpfsa24(&mut self) -> Dpfsa24W<DpfsarSpec> {
        Dpfsa24W::new(self, 24)
    }
    ///Bit 25 - Deep Software Standby Interrupt Factor Security Attribute bit 25
    #[inline(always)]
    pub fn dpfsa25(&mut self) -> Dpfsa25W<DpfsarSpec> {
        Dpfsa25W::new(self, 25)
    }
    ///Bit 26 - Deep Software Standby Interrupt Factor Security Attribute bit 26
    #[inline(always)]
    pub fn dpfsa26(&mut self) -> Dpfsa26W<DpfsarSpec> {
        Dpfsa26W::new(self, 26)
    }
    ///Bit 27 - Deep Software Standby Interrupt Factor Security Attribute bit 27
    #[inline(always)]
    pub fn dpfsa27(&mut self) -> Dpfsa27W<DpfsarSpec> {
        Dpfsa27W::new(self, 27)
    }
}
/**Deep Software Standby Interrupt Factor Security Attribution Register

You can [`read`](crate::Reg::read) this register and get [`dpfsar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpfsar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DpfsarSpec;
impl crate::RegisterSpec for DpfsarSpec {
    type Ux = u32;
}
///`read()` method returns [`dpfsar::R`](R) reader structure
impl crate::Readable for DpfsarSpec {}
///`write(|w| ..)` method takes [`dpfsar::W`](W) writer structure
impl crate::Writable for DpfsarSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DPFSAR to value 0xffff_ffff
impl crate::Resettable for DpfsarSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
