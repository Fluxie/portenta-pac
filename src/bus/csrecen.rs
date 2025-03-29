///Register `CSRECEN` reader
pub type R = crate::R<CsrecenSpec>;
///Register `CSRECEN` writer
pub type W = crate::W<CsrecenSpec>;
/**Separate Bus Recovery Cycle Insertion Enable 0

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rcven0 {
    ///0: Recovery cycle insertion is disabled.
    _0 = 0,
    ///1: Recovery cycle insertion is enabled.
    _1 = 1,
}
impl From<Rcven0> for bool {
    #[inline(always)]
    fn from(variant: Rcven0) -> Self {
        variant as u8 != 0
    }
}
///Field `RCVEN0` reader - Separate Bus Recovery Cycle Insertion Enable 0
pub type Rcven0R = crate::BitReader<Rcven0>;
impl Rcven0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rcven0 {
        match self.bits {
            false => Rcven0::_0,
            true => Rcven0::_1,
        }
    }
    ///Recovery cycle insertion is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rcven0::_0
    }
    ///Recovery cycle insertion is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rcven0::_1
    }
}
///Field `RCVEN0` writer - Separate Bus Recovery Cycle Insertion Enable 0
pub type Rcven0W<'a, REG> = crate::BitWriter<'a, REG, Rcven0>;
impl<'a, REG> Rcven0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Recovery cycle insertion is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rcven0::_0)
    }
    ///Recovery cycle insertion is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rcven0::_1)
    }
}
/**Separate Bus Recovery Cycle Insertion Enable 1

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rcven1 {
    ///0: Recovery cycle insertion is disabled.
    _0 = 0,
    ///1: Recovery cycle insertion is enabled.
    _1 = 1,
}
impl From<Rcven1> for bool {
    #[inline(always)]
    fn from(variant: Rcven1) -> Self {
        variant as u8 != 0
    }
}
///Field `RCVEN1` reader - Separate Bus Recovery Cycle Insertion Enable 1
pub type Rcven1R = crate::BitReader<Rcven1>;
impl Rcven1R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rcven1 {
        match self.bits {
            false => Rcven1::_0,
            true => Rcven1::_1,
        }
    }
    ///Recovery cycle insertion is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rcven1::_0
    }
    ///Recovery cycle insertion is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rcven1::_1
    }
}
///Field `RCVEN1` writer - Separate Bus Recovery Cycle Insertion Enable 1
pub type Rcven1W<'a, REG> = crate::BitWriter<'a, REG, Rcven1>;
impl<'a, REG> Rcven1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Recovery cycle insertion is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rcven1::_0)
    }
    ///Recovery cycle insertion is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rcven1::_1)
    }
}
/**Separate Bus Recovery Cycle Insertion Enable 2

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rcven2 {
    ///0: Recovery cycle insertion is disabled.
    _0 = 0,
    ///1: Recovery cycle insertion is enabled.
    _1 = 1,
}
impl From<Rcven2> for bool {
    #[inline(always)]
    fn from(variant: Rcven2) -> Self {
        variant as u8 != 0
    }
}
///Field `RCVEN2` reader - Separate Bus Recovery Cycle Insertion Enable 2
pub type Rcven2R = crate::BitReader<Rcven2>;
impl Rcven2R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rcven2 {
        match self.bits {
            false => Rcven2::_0,
            true => Rcven2::_1,
        }
    }
    ///Recovery cycle insertion is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rcven2::_0
    }
    ///Recovery cycle insertion is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rcven2::_1
    }
}
///Field `RCVEN2` writer - Separate Bus Recovery Cycle Insertion Enable 2
pub type Rcven2W<'a, REG> = crate::BitWriter<'a, REG, Rcven2>;
impl<'a, REG> Rcven2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Recovery cycle insertion is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rcven2::_0)
    }
    ///Recovery cycle insertion is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rcven2::_1)
    }
}
/**Separate Bus Recovery Cycle Insertion Enable 3

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rcven3 {
    ///0: Recovery cycle insertion is disabled.
    _0 = 0,
    ///1: Recovery cycle insertion is enabled.
    _1 = 1,
}
impl From<Rcven3> for bool {
    #[inline(always)]
    fn from(variant: Rcven3) -> Self {
        variant as u8 != 0
    }
}
///Field `RCVEN3` reader - Separate Bus Recovery Cycle Insertion Enable 3
pub type Rcven3R = crate::BitReader<Rcven3>;
impl Rcven3R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rcven3 {
        match self.bits {
            false => Rcven3::_0,
            true => Rcven3::_1,
        }
    }
    ///Recovery cycle insertion is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rcven3::_0
    }
    ///Recovery cycle insertion is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rcven3::_1
    }
}
///Field `RCVEN3` writer - Separate Bus Recovery Cycle Insertion Enable 3
pub type Rcven3W<'a, REG> = crate::BitWriter<'a, REG, Rcven3>;
impl<'a, REG> Rcven3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Recovery cycle insertion is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rcven3::_0)
    }
    ///Recovery cycle insertion is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rcven3::_1)
    }
}
/**Separate Bus Recovery Cycle Insertion Enable 4

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rcven4 {
    ///0: Recovery cycle insertion is disabled.
    _0 = 0,
    ///1: Recovery cycle insertion is enabled.
    _1 = 1,
}
impl From<Rcven4> for bool {
    #[inline(always)]
    fn from(variant: Rcven4) -> Self {
        variant as u8 != 0
    }
}
///Field `RCVEN4` reader - Separate Bus Recovery Cycle Insertion Enable 4
pub type Rcven4R = crate::BitReader<Rcven4>;
impl Rcven4R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rcven4 {
        match self.bits {
            false => Rcven4::_0,
            true => Rcven4::_1,
        }
    }
    ///Recovery cycle insertion is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rcven4::_0
    }
    ///Recovery cycle insertion is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rcven4::_1
    }
}
///Field `RCVEN4` writer - Separate Bus Recovery Cycle Insertion Enable 4
pub type Rcven4W<'a, REG> = crate::BitWriter<'a, REG, Rcven4>;
impl<'a, REG> Rcven4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Recovery cycle insertion is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rcven4::_0)
    }
    ///Recovery cycle insertion is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rcven4::_1)
    }
}
/**Separate Bus Recovery Cycle Insertion Enable 5

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rcven5 {
    ///0: Recovery cycle insertion is disabled.
    _0 = 0,
    ///1: Recovery cycle insertion is enabled.
    _1 = 1,
}
impl From<Rcven5> for bool {
    #[inline(always)]
    fn from(variant: Rcven5) -> Self {
        variant as u8 != 0
    }
}
///Field `RCVEN5` reader - Separate Bus Recovery Cycle Insertion Enable 5
pub type Rcven5R = crate::BitReader<Rcven5>;
impl Rcven5R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rcven5 {
        match self.bits {
            false => Rcven5::_0,
            true => Rcven5::_1,
        }
    }
    ///Recovery cycle insertion is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rcven5::_0
    }
    ///Recovery cycle insertion is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rcven5::_1
    }
}
///Field `RCVEN5` writer - Separate Bus Recovery Cycle Insertion Enable 5
pub type Rcven5W<'a, REG> = crate::BitWriter<'a, REG, Rcven5>;
impl<'a, REG> Rcven5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Recovery cycle insertion is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rcven5::_0)
    }
    ///Recovery cycle insertion is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rcven5::_1)
    }
}
/**Separate Bus Recovery Cycle Insertion Enable 6

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rcven6 {
    ///0: Recovery cycle insertion is disabled.
    _0 = 0,
    ///1: Recovery cycle insertion is enabled.
    _1 = 1,
}
impl From<Rcven6> for bool {
    #[inline(always)]
    fn from(variant: Rcven6) -> Self {
        variant as u8 != 0
    }
}
///Field `RCVEN6` reader - Separate Bus Recovery Cycle Insertion Enable 6
pub type Rcven6R = crate::BitReader<Rcven6>;
impl Rcven6R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rcven6 {
        match self.bits {
            false => Rcven6::_0,
            true => Rcven6::_1,
        }
    }
    ///Recovery cycle insertion is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rcven6::_0
    }
    ///Recovery cycle insertion is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rcven6::_1
    }
}
///Field `RCVEN6` writer - Separate Bus Recovery Cycle Insertion Enable 6
pub type Rcven6W<'a, REG> = crate::BitWriter<'a, REG, Rcven6>;
impl<'a, REG> Rcven6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Recovery cycle insertion is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rcven6::_0)
    }
    ///Recovery cycle insertion is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rcven6::_1)
    }
}
/**Separate Bus Recovery Cycle Insertion Enable 7

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rcven7 {
    ///0: Recovery cycle insertion is disabled.
    _0 = 0,
    ///1: Recovery cycle insertion is enabled.
    _1 = 1,
}
impl From<Rcven7> for bool {
    #[inline(always)]
    fn from(variant: Rcven7) -> Self {
        variant as u8 != 0
    }
}
///Field `RCVEN7` reader - Separate Bus Recovery Cycle Insertion Enable 7
pub type Rcven7R = crate::BitReader<Rcven7>;
impl Rcven7R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rcven7 {
        match self.bits {
            false => Rcven7::_0,
            true => Rcven7::_1,
        }
    }
    ///Recovery cycle insertion is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rcven7::_0
    }
    ///Recovery cycle insertion is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rcven7::_1
    }
}
///Field `RCVEN7` writer - Separate Bus Recovery Cycle Insertion Enable 7
pub type Rcven7W<'a, REG> = crate::BitWriter<'a, REG, Rcven7>;
impl<'a, REG> Rcven7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Recovery cycle insertion is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rcven7::_0)
    }
    ///Recovery cycle insertion is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rcven7::_1)
    }
}
/**Multiplexed Bus Recovery Cycle Insertion Enable 0

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rcvenm0 {
    ///0: Recovery cycle insertion is disabled.
    _0 = 0,
    ///1: Recovery cycle insertion is enabled.
    _1 = 1,
}
impl From<Rcvenm0> for bool {
    #[inline(always)]
    fn from(variant: Rcvenm0) -> Self {
        variant as u8 != 0
    }
}
///Field `RCVENM0` reader - Multiplexed Bus Recovery Cycle Insertion Enable 0
pub type Rcvenm0R = crate::BitReader<Rcvenm0>;
impl Rcvenm0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rcvenm0 {
        match self.bits {
            false => Rcvenm0::_0,
            true => Rcvenm0::_1,
        }
    }
    ///Recovery cycle insertion is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rcvenm0::_0
    }
    ///Recovery cycle insertion is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rcvenm0::_1
    }
}
///Field `RCVENM0` writer - Multiplexed Bus Recovery Cycle Insertion Enable 0
pub type Rcvenm0W<'a, REG> = crate::BitWriter<'a, REG, Rcvenm0>;
impl<'a, REG> Rcvenm0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Recovery cycle insertion is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rcvenm0::_0)
    }
    ///Recovery cycle insertion is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rcvenm0::_1)
    }
}
/**Multiplexed Bus Recovery Cycle Insertion Enable 1

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rcvenm1 {
    ///0: Recovery cycle insertion is disabled.
    _0 = 0,
    ///1: Recovery cycle insertion is enabled.
    _1 = 1,
}
impl From<Rcvenm1> for bool {
    #[inline(always)]
    fn from(variant: Rcvenm1) -> Self {
        variant as u8 != 0
    }
}
///Field `RCVENM1` reader - Multiplexed Bus Recovery Cycle Insertion Enable 1
pub type Rcvenm1R = crate::BitReader<Rcvenm1>;
impl Rcvenm1R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rcvenm1 {
        match self.bits {
            false => Rcvenm1::_0,
            true => Rcvenm1::_1,
        }
    }
    ///Recovery cycle insertion is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rcvenm1::_0
    }
    ///Recovery cycle insertion is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rcvenm1::_1
    }
}
///Field `RCVENM1` writer - Multiplexed Bus Recovery Cycle Insertion Enable 1
pub type Rcvenm1W<'a, REG> = crate::BitWriter<'a, REG, Rcvenm1>;
impl<'a, REG> Rcvenm1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Recovery cycle insertion is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rcvenm1::_0)
    }
    ///Recovery cycle insertion is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rcvenm1::_1)
    }
}
/**Multiplexed Bus Recovery Cycle Insertion Enable 2

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rcvenm2 {
    ///0: Recovery cycle insertion is disabled.
    _0 = 0,
    ///1: Recovery cycle insertion is enabled.
    _1 = 1,
}
impl From<Rcvenm2> for bool {
    #[inline(always)]
    fn from(variant: Rcvenm2) -> Self {
        variant as u8 != 0
    }
}
///Field `RCVENM2` reader - Multiplexed Bus Recovery Cycle Insertion Enable 2
pub type Rcvenm2R = crate::BitReader<Rcvenm2>;
impl Rcvenm2R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rcvenm2 {
        match self.bits {
            false => Rcvenm2::_0,
            true => Rcvenm2::_1,
        }
    }
    ///Recovery cycle insertion is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rcvenm2::_0
    }
    ///Recovery cycle insertion is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rcvenm2::_1
    }
}
///Field `RCVENM2` writer - Multiplexed Bus Recovery Cycle Insertion Enable 2
pub type Rcvenm2W<'a, REG> = crate::BitWriter<'a, REG, Rcvenm2>;
impl<'a, REG> Rcvenm2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Recovery cycle insertion is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rcvenm2::_0)
    }
    ///Recovery cycle insertion is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rcvenm2::_1)
    }
}
/**Multiplexed Bus Recovery Cycle Insertion Enable 3

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rcvenm3 {
    ///0: Recovery cycle insertion is disabled.
    _0 = 0,
    ///1: Recovery cycle insertion is enabled.
    _1 = 1,
}
impl From<Rcvenm3> for bool {
    #[inline(always)]
    fn from(variant: Rcvenm3) -> Self {
        variant as u8 != 0
    }
}
///Field `RCVENM3` reader - Multiplexed Bus Recovery Cycle Insertion Enable 3
pub type Rcvenm3R = crate::BitReader<Rcvenm3>;
impl Rcvenm3R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rcvenm3 {
        match self.bits {
            false => Rcvenm3::_0,
            true => Rcvenm3::_1,
        }
    }
    ///Recovery cycle insertion is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rcvenm3::_0
    }
    ///Recovery cycle insertion is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rcvenm3::_1
    }
}
///Field `RCVENM3` writer - Multiplexed Bus Recovery Cycle Insertion Enable 3
pub type Rcvenm3W<'a, REG> = crate::BitWriter<'a, REG, Rcvenm3>;
impl<'a, REG> Rcvenm3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Recovery cycle insertion is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rcvenm3::_0)
    }
    ///Recovery cycle insertion is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rcvenm3::_1)
    }
}
/**Multiplexed Bus Recovery Cycle Insertion Enable 4

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rcvenm4 {
    ///0: Recovery cycle insertion is disabled.
    _0 = 0,
    ///1: Recovery cycle insertion is enabled.
    _1 = 1,
}
impl From<Rcvenm4> for bool {
    #[inline(always)]
    fn from(variant: Rcvenm4) -> Self {
        variant as u8 != 0
    }
}
///Field `RCVENM4` reader - Multiplexed Bus Recovery Cycle Insertion Enable 4
pub type Rcvenm4R = crate::BitReader<Rcvenm4>;
impl Rcvenm4R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rcvenm4 {
        match self.bits {
            false => Rcvenm4::_0,
            true => Rcvenm4::_1,
        }
    }
    ///Recovery cycle insertion is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rcvenm4::_0
    }
    ///Recovery cycle insertion is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rcvenm4::_1
    }
}
///Field `RCVENM4` writer - Multiplexed Bus Recovery Cycle Insertion Enable 4
pub type Rcvenm4W<'a, REG> = crate::BitWriter<'a, REG, Rcvenm4>;
impl<'a, REG> Rcvenm4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Recovery cycle insertion is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rcvenm4::_0)
    }
    ///Recovery cycle insertion is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rcvenm4::_1)
    }
}
/**Multiplexed Bus Recovery Cycle Insertion Enable 5

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rcvenm5 {
    ///0: Recovery cycle insertion is disabled.
    _0 = 0,
    ///1: Recovery cycle insertion is enabled.
    _1 = 1,
}
impl From<Rcvenm5> for bool {
    #[inline(always)]
    fn from(variant: Rcvenm5) -> Self {
        variant as u8 != 0
    }
}
///Field `RCVENM5` reader - Multiplexed Bus Recovery Cycle Insertion Enable 5
pub type Rcvenm5R = crate::BitReader<Rcvenm5>;
impl Rcvenm5R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rcvenm5 {
        match self.bits {
            false => Rcvenm5::_0,
            true => Rcvenm5::_1,
        }
    }
    ///Recovery cycle insertion is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rcvenm5::_0
    }
    ///Recovery cycle insertion is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rcvenm5::_1
    }
}
///Field `RCVENM5` writer - Multiplexed Bus Recovery Cycle Insertion Enable 5
pub type Rcvenm5W<'a, REG> = crate::BitWriter<'a, REG, Rcvenm5>;
impl<'a, REG> Rcvenm5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Recovery cycle insertion is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rcvenm5::_0)
    }
    ///Recovery cycle insertion is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rcvenm5::_1)
    }
}
/**Multiplexed Bus Recovery Cycle Insertion Enable 6

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rcvenm6 {
    ///0: Recovery cycle insertion is disabled.
    _0 = 0,
    ///1: Recovery cycle insertion is enabled.
    _1 = 1,
}
impl From<Rcvenm6> for bool {
    #[inline(always)]
    fn from(variant: Rcvenm6) -> Self {
        variant as u8 != 0
    }
}
///Field `RCVENM6` reader - Multiplexed Bus Recovery Cycle Insertion Enable 6
pub type Rcvenm6R = crate::BitReader<Rcvenm6>;
impl Rcvenm6R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rcvenm6 {
        match self.bits {
            false => Rcvenm6::_0,
            true => Rcvenm6::_1,
        }
    }
    ///Recovery cycle insertion is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rcvenm6::_0
    }
    ///Recovery cycle insertion is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rcvenm6::_1
    }
}
///Field `RCVENM6` writer - Multiplexed Bus Recovery Cycle Insertion Enable 6
pub type Rcvenm6W<'a, REG> = crate::BitWriter<'a, REG, Rcvenm6>;
impl<'a, REG> Rcvenm6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Recovery cycle insertion is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rcvenm6::_0)
    }
    ///Recovery cycle insertion is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rcvenm6::_1)
    }
}
/**Multiplexed Bus Recovery Cycle Insertion Enable 7

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rcvenm7 {
    ///0: Recovery cycle insertion is disabled.
    _0 = 0,
    ///1: Recovery cycle insertion is enabled.
    _1 = 1,
}
impl From<Rcvenm7> for bool {
    #[inline(always)]
    fn from(variant: Rcvenm7) -> Self {
        variant as u8 != 0
    }
}
///Field `RCVENM7` reader - Multiplexed Bus Recovery Cycle Insertion Enable 7
pub type Rcvenm7R = crate::BitReader<Rcvenm7>;
impl Rcvenm7R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rcvenm7 {
        match self.bits {
            false => Rcvenm7::_0,
            true => Rcvenm7::_1,
        }
    }
    ///Recovery cycle insertion is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rcvenm7::_0
    }
    ///Recovery cycle insertion is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rcvenm7::_1
    }
}
///Field `RCVENM7` writer - Multiplexed Bus Recovery Cycle Insertion Enable 7
pub type Rcvenm7W<'a, REG> = crate::BitWriter<'a, REG, Rcvenm7>;
impl<'a, REG> Rcvenm7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Recovery cycle insertion is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rcvenm7::_0)
    }
    ///Recovery cycle insertion is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rcvenm7::_1)
    }
}
impl R {
    ///Bit 0 - Separate Bus Recovery Cycle Insertion Enable 0
    #[inline(always)]
    pub fn rcven0(&self) -> Rcven0R {
        Rcven0R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Separate Bus Recovery Cycle Insertion Enable 1
    #[inline(always)]
    pub fn rcven1(&self) -> Rcven1R {
        Rcven1R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Separate Bus Recovery Cycle Insertion Enable 2
    #[inline(always)]
    pub fn rcven2(&self) -> Rcven2R {
        Rcven2R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Separate Bus Recovery Cycle Insertion Enable 3
    #[inline(always)]
    pub fn rcven3(&self) -> Rcven3R {
        Rcven3R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Separate Bus Recovery Cycle Insertion Enable 4
    #[inline(always)]
    pub fn rcven4(&self) -> Rcven4R {
        Rcven4R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Separate Bus Recovery Cycle Insertion Enable 5
    #[inline(always)]
    pub fn rcven5(&self) -> Rcven5R {
        Rcven5R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Separate Bus Recovery Cycle Insertion Enable 6
    #[inline(always)]
    pub fn rcven6(&self) -> Rcven6R {
        Rcven6R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Separate Bus Recovery Cycle Insertion Enable 7
    #[inline(always)]
    pub fn rcven7(&self) -> Rcven7R {
        Rcven7R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Multiplexed Bus Recovery Cycle Insertion Enable 0
    #[inline(always)]
    pub fn rcvenm0(&self) -> Rcvenm0R {
        Rcvenm0R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Multiplexed Bus Recovery Cycle Insertion Enable 1
    #[inline(always)]
    pub fn rcvenm1(&self) -> Rcvenm1R {
        Rcvenm1R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Multiplexed Bus Recovery Cycle Insertion Enable 2
    #[inline(always)]
    pub fn rcvenm2(&self) -> Rcvenm2R {
        Rcvenm2R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Multiplexed Bus Recovery Cycle Insertion Enable 3
    #[inline(always)]
    pub fn rcvenm3(&self) -> Rcvenm3R {
        Rcvenm3R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Multiplexed Bus Recovery Cycle Insertion Enable 4
    #[inline(always)]
    pub fn rcvenm4(&self) -> Rcvenm4R {
        Rcvenm4R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Multiplexed Bus Recovery Cycle Insertion Enable 5
    #[inline(always)]
    pub fn rcvenm5(&self) -> Rcvenm5R {
        Rcvenm5R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Multiplexed Bus Recovery Cycle Insertion Enable 6
    #[inline(always)]
    pub fn rcvenm6(&self) -> Rcvenm6R {
        Rcvenm6R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Multiplexed Bus Recovery Cycle Insertion Enable 7
    #[inline(always)]
    pub fn rcvenm7(&self) -> Rcvenm7R {
        Rcvenm7R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSRECEN")
            .field("rcven0", &self.rcven0())
            .field("rcven1", &self.rcven1())
            .field("rcven2", &self.rcven2())
            .field("rcven3", &self.rcven3())
            .field("rcven4", &self.rcven4())
            .field("rcven5", &self.rcven5())
            .field("rcven6", &self.rcven6())
            .field("rcven7", &self.rcven7())
            .field("rcvenm0", &self.rcvenm0())
            .field("rcvenm1", &self.rcvenm1())
            .field("rcvenm2", &self.rcvenm2())
            .field("rcvenm3", &self.rcvenm3())
            .field("rcvenm4", &self.rcvenm4())
            .field("rcvenm5", &self.rcvenm5())
            .field("rcvenm6", &self.rcvenm6())
            .field("rcvenm7", &self.rcvenm7())
            .finish()
    }
}
impl W {
    ///Bit 0 - Separate Bus Recovery Cycle Insertion Enable 0
    #[inline(always)]
    pub fn rcven0(&mut self) -> Rcven0W<CsrecenSpec> {
        Rcven0W::new(self, 0)
    }
    ///Bit 1 - Separate Bus Recovery Cycle Insertion Enable 1
    #[inline(always)]
    pub fn rcven1(&mut self) -> Rcven1W<CsrecenSpec> {
        Rcven1W::new(self, 1)
    }
    ///Bit 2 - Separate Bus Recovery Cycle Insertion Enable 2
    #[inline(always)]
    pub fn rcven2(&mut self) -> Rcven2W<CsrecenSpec> {
        Rcven2W::new(self, 2)
    }
    ///Bit 3 - Separate Bus Recovery Cycle Insertion Enable 3
    #[inline(always)]
    pub fn rcven3(&mut self) -> Rcven3W<CsrecenSpec> {
        Rcven3W::new(self, 3)
    }
    ///Bit 4 - Separate Bus Recovery Cycle Insertion Enable 4
    #[inline(always)]
    pub fn rcven4(&mut self) -> Rcven4W<CsrecenSpec> {
        Rcven4W::new(self, 4)
    }
    ///Bit 5 - Separate Bus Recovery Cycle Insertion Enable 5
    #[inline(always)]
    pub fn rcven5(&mut self) -> Rcven5W<CsrecenSpec> {
        Rcven5W::new(self, 5)
    }
    ///Bit 6 - Separate Bus Recovery Cycle Insertion Enable 6
    #[inline(always)]
    pub fn rcven6(&mut self) -> Rcven6W<CsrecenSpec> {
        Rcven6W::new(self, 6)
    }
    ///Bit 7 - Separate Bus Recovery Cycle Insertion Enable 7
    #[inline(always)]
    pub fn rcven7(&mut self) -> Rcven7W<CsrecenSpec> {
        Rcven7W::new(self, 7)
    }
    ///Bit 8 - Multiplexed Bus Recovery Cycle Insertion Enable 0
    #[inline(always)]
    pub fn rcvenm0(&mut self) -> Rcvenm0W<CsrecenSpec> {
        Rcvenm0W::new(self, 8)
    }
    ///Bit 9 - Multiplexed Bus Recovery Cycle Insertion Enable 1
    #[inline(always)]
    pub fn rcvenm1(&mut self) -> Rcvenm1W<CsrecenSpec> {
        Rcvenm1W::new(self, 9)
    }
    ///Bit 10 - Multiplexed Bus Recovery Cycle Insertion Enable 2
    #[inline(always)]
    pub fn rcvenm2(&mut self) -> Rcvenm2W<CsrecenSpec> {
        Rcvenm2W::new(self, 10)
    }
    ///Bit 11 - Multiplexed Bus Recovery Cycle Insertion Enable 3
    #[inline(always)]
    pub fn rcvenm3(&mut self) -> Rcvenm3W<CsrecenSpec> {
        Rcvenm3W::new(self, 11)
    }
    ///Bit 12 - Multiplexed Bus Recovery Cycle Insertion Enable 4
    #[inline(always)]
    pub fn rcvenm4(&mut self) -> Rcvenm4W<CsrecenSpec> {
        Rcvenm4W::new(self, 12)
    }
    ///Bit 13 - Multiplexed Bus Recovery Cycle Insertion Enable 5
    #[inline(always)]
    pub fn rcvenm5(&mut self) -> Rcvenm5W<CsrecenSpec> {
        Rcvenm5W::new(self, 13)
    }
    ///Bit 14 - Multiplexed Bus Recovery Cycle Insertion Enable 6
    #[inline(always)]
    pub fn rcvenm6(&mut self) -> Rcvenm6W<CsrecenSpec> {
        Rcvenm6W::new(self, 14)
    }
    ///Bit 15 - Multiplexed Bus Recovery Cycle Insertion Enable 7
    #[inline(always)]
    pub fn rcvenm7(&mut self) -> Rcvenm7W<CsrecenSpec> {
        Rcvenm7W::new(self, 15)
    }
}
/**CS Recovery Cycle Insertion Enable Register

You can [`read`](crate::Reg::read) this register and get [`csrecen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csrecen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CsrecenSpec;
impl crate::RegisterSpec for CsrecenSpec {
    type Ux = u16;
}
///`read()` method returns [`csrecen::R`](R) reader structure
impl crate::Readable for CsrecenSpec {}
///`write(|w| ..)` method takes [`csrecen::W`](W) writer structure
impl crate::Writable for CsrecenSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSRECEN to value 0x3e3e
impl crate::Resettable for CsrecenSpec {
    const RESET_VALUE: u16 = 0x3e3e;
}
