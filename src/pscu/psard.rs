///Register `PSARD` reader
pub type R = crate::R<PsardSpec>;
///Register `PSARD` writer
pub type W = crate::W<PsardSpec>;
/**AGT3 and the MSTPCRD.MSTPD0 bit security attribution

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psard0 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Psard0> for bool {
    #[inline(always)]
    fn from(variant: Psard0) -> Self {
        variant as u8 != 0
    }
}
///Field `PSARD0` reader - AGT3 and the MSTPCRD.MSTPD0 bit security attribution
pub type Psard0R = crate::BitReader<Psard0>;
impl Psard0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Psard0 {
        match self.bits {
            false => Psard0::_0,
            true => Psard0::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psard0::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psard0::_1
    }
}
///Field `PSARD0` writer - AGT3 and the MSTPCRD.MSTPD0 bit security attribution
pub type Psard0W<'a, REG> = crate::BitWriter<'a, REG, Psard0>;
impl<'a, REG> Psard0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psard0::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psard0::_1)
    }
}
/**AGT2 and the MSTPCRD.MSTPD1 bit security attribution

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psard1 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Psard1> for bool {
    #[inline(always)]
    fn from(variant: Psard1) -> Self {
        variant as u8 != 0
    }
}
///Field `PSARD1` reader - AGT2 and the MSTPCRD.MSTPD1 bit security attribution
pub type Psard1R = crate::BitReader<Psard1>;
impl Psard1R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Psard1 {
        match self.bits {
            false => Psard1::_0,
            true => Psard1::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psard1::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psard1::_1
    }
}
///Field `PSARD1` writer - AGT2 and the MSTPCRD.MSTPD1 bit security attribution
pub type Psard1W<'a, REG> = crate::BitWriter<'a, REG, Psard1>;
impl<'a, REG> Psard1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psard1::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psard1::_1)
    }
}
/**AGT1 and the MSTPCRD.MSTPD2 bit security attribution

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psard2 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Psard2> for bool {
    #[inline(always)]
    fn from(variant: Psard2) -> Self {
        variant as u8 != 0
    }
}
///Field `PSARD2` reader - AGT1 and the MSTPCRD.MSTPD2 bit security attribution
pub type Psard2R = crate::BitReader<Psard2>;
impl Psard2R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Psard2 {
        match self.bits {
            false => Psard2::_0,
            true => Psard2::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psard2::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psard2::_1
    }
}
///Field `PSARD2` writer - AGT1 and the MSTPCRD.MSTPD2 bit security attribution
pub type Psard2W<'a, REG> = crate::BitWriter<'a, REG, Psard2>;
impl<'a, REG> Psard2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psard2::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psard2::_1)
    }
}
/**AGT0 and the MSTPCRD.MSTPD3 bit security attribution

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psard3 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Psard3> for bool {
    #[inline(always)]
    fn from(variant: Psard3) -> Self {
        variant as u8 != 0
    }
}
///Field `PSARD3` reader - AGT0 and the MSTPCRD.MSTPD3 bit security attribution
pub type Psard3R = crate::BitReader<Psard3>;
impl Psard3R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Psard3 {
        match self.bits {
            false => Psard3::_0,
            true => Psard3::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psard3::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psard3::_1
    }
}
///Field `PSARD3` writer - AGT0 and the MSTPCRD.MSTPD3 bit security attribution
pub type Psard3W<'a, REG> = crate::BitWriter<'a, REG, Psard3>;
impl<'a, REG> Psard3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psard3::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psard3::_1)
    }
}
/**POEG Group D and the MSTPCRD.MSTPD11 bit security attribution

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psard11 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Psard11> for bool {
    #[inline(always)]
    fn from(variant: Psard11) -> Self {
        variant as u8 != 0
    }
}
///Field `PSARD11` reader - POEG Group D and the MSTPCRD.MSTPD11 bit security attribution
pub type Psard11R = crate::BitReader<Psard11>;
impl Psard11R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Psard11 {
        match self.bits {
            false => Psard11::_0,
            true => Psard11::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psard11::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psard11::_1
    }
}
///Field `PSARD11` writer - POEG Group D and the MSTPCRD.MSTPD11 bit security attribution
pub type Psard11W<'a, REG> = crate::BitWriter<'a, REG, Psard11>;
impl<'a, REG> Psard11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psard11::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psard11::_1)
    }
}
/**POEG Group C and the MSTPCRD.MSTPD12 bit security attribution

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psard12 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Psard12> for bool {
    #[inline(always)]
    fn from(variant: Psard12) -> Self {
        variant as u8 != 0
    }
}
///Field `PSARD12` reader - POEG Group C and the MSTPCRD.MSTPD12 bit security attribution
pub type Psard12R = crate::BitReader<Psard12>;
impl Psard12R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Psard12 {
        match self.bits {
            false => Psard12::_0,
            true => Psard12::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psard12::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psard12::_1
    }
}
///Field `PSARD12` writer - POEG Group C and the MSTPCRD.MSTPD12 bit security attribution
pub type Psard12W<'a, REG> = crate::BitWriter<'a, REG, Psard12>;
impl<'a, REG> Psard12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psard12::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psard12::_1)
    }
}
/**POEG Group B and the MSTPCRD.MSTPD13 bit security attribution

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psard13 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Psard13> for bool {
    #[inline(always)]
    fn from(variant: Psard13) -> Self {
        variant as u8 != 0
    }
}
///Field `PSARD13` reader - POEG Group B and the MSTPCRD.MSTPD13 bit security attribution
pub type Psard13R = crate::BitReader<Psard13>;
impl Psard13R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Psard13 {
        match self.bits {
            false => Psard13::_0,
            true => Psard13::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psard13::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psard13::_1
    }
}
///Field `PSARD13` writer - POEG Group B and the MSTPCRD.MSTPD13 bit security attribution
pub type Psard13W<'a, REG> = crate::BitWriter<'a, REG, Psard13>;
impl<'a, REG> Psard13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psard13::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psard13::_1)
    }
}
/**POEG Group A and the MSTPCRD.MSTPD14 bit security attribution

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psard14 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Psard14> for bool {
    #[inline(always)]
    fn from(variant: Psard14) -> Self {
        variant as u8 != 0
    }
}
///Field `PSARD14` reader - POEG Group A and the MSTPCRD.MSTPD14 bit security attribution
pub type Psard14R = crate::BitReader<Psard14>;
impl Psard14R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Psard14 {
        match self.bits {
            false => Psard14::_0,
            true => Psard14::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psard14::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psard14::_1
    }
}
///Field `PSARD14` writer - POEG Group A and the MSTPCRD.MSTPD14 bit security attribution
pub type Psard14W<'a, REG> = crate::BitWriter<'a, REG, Psard14>;
impl<'a, REG> Psard14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psard14::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psard14::_1)
    }
}
/**ADC121 and the MSTPCRD.MSTPD15 bit security attribution

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psard15 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Psard15> for bool {
    #[inline(always)]
    fn from(variant: Psard15) -> Self {
        variant as u8 != 0
    }
}
///Field `PSARD15` reader - ADC121 and the MSTPCRD.MSTPD15 bit security attribution
pub type Psard15R = crate::BitReader<Psard15>;
impl Psard15R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Psard15 {
        match self.bits {
            false => Psard15::_0,
            true => Psard15::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psard15::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psard15::_1
    }
}
///Field `PSARD15` writer - ADC121 and the MSTPCRD.MSTPD15 bit security attribution
pub type Psard15W<'a, REG> = crate::BitWriter<'a, REG, Psard15>;
impl<'a, REG> Psard15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psard15::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psard15::_1)
    }
}
/**ADC120 and the MSTPCRD.MSTPD16 bit security attribution

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psard16 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Psard16> for bool {
    #[inline(always)]
    fn from(variant: Psard16) -> Self {
        variant as u8 != 0
    }
}
///Field `PSARD16` reader - ADC120 and the MSTPCRD.MSTPD16 bit security attribution
pub type Psard16R = crate::BitReader<Psard16>;
impl Psard16R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Psard16 {
        match self.bits {
            false => Psard16::_0,
            true => Psard16::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psard16::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psard16::_1
    }
}
///Field `PSARD16` writer - ADC120 and the MSTPCRD.MSTPD16 bit security attribution
pub type Psard16W<'a, REG> = crate::BitWriter<'a, REG, Psard16>;
impl<'a, REG> Psard16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psard16::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psard16::_1)
    }
}
/**DAC12 and the MSTPCRD.MSTPD20 bit security attribution

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psard20 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Psard20> for bool {
    #[inline(always)]
    fn from(variant: Psard20) -> Self {
        variant as u8 != 0
    }
}
///Field `PSARD20` reader - DAC12 and the MSTPCRD.MSTPD20 bit security attribution
pub type Psard20R = crate::BitReader<Psard20>;
impl Psard20R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Psard20 {
        match self.bits {
            false => Psard20::_0,
            true => Psard20::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psard20::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psard20::_1
    }
}
///Field `PSARD20` writer - DAC12 and the MSTPCRD.MSTPD20 bit security attribution
pub type Psard20W<'a, REG> = crate::BitWriter<'a, REG, Psard20>;
impl<'a, REG> Psard20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psard20::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psard20::_1)
    }
}
/**TSN and the MSTPCRD.MSTPD22 bit security attribution

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psard22 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Psard22> for bool {
    #[inline(always)]
    fn from(variant: Psard22) -> Self {
        variant as u8 != 0
    }
}
///Field `PSARD22` reader - TSN and the MSTPCRD.MSTPD22 bit security attribution
pub type Psard22R = crate::BitReader<Psard22>;
impl Psard22R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Psard22 {
        match self.bits {
            false => Psard22::_0,
            true => Psard22::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psard22::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psard22::_1
    }
}
///Field `PSARD22` writer - TSN and the MSTPCRD.MSTPD22 bit security attribution
pub type Psard22W<'a, REG> = crate::BitWriter<'a, REG, Psard22>;
impl<'a, REG> Psard22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psard22::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psard22::_1)
    }
}
impl R {
    ///Bit 0 - AGT3 and the MSTPCRD.MSTPD0 bit security attribution
    #[inline(always)]
    pub fn psard0(&self) -> Psard0R {
        Psard0R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - AGT2 and the MSTPCRD.MSTPD1 bit security attribution
    #[inline(always)]
    pub fn psard1(&self) -> Psard1R {
        Psard1R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - AGT1 and the MSTPCRD.MSTPD2 bit security attribution
    #[inline(always)]
    pub fn psard2(&self) -> Psard2R {
        Psard2R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - AGT0 and the MSTPCRD.MSTPD3 bit security attribution
    #[inline(always)]
    pub fn psard3(&self) -> Psard3R {
        Psard3R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 11 - POEG Group D and the MSTPCRD.MSTPD11 bit security attribution
    #[inline(always)]
    pub fn psard11(&self) -> Psard11R {
        Psard11R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - POEG Group C and the MSTPCRD.MSTPD12 bit security attribution
    #[inline(always)]
    pub fn psard12(&self) -> Psard12R {
        Psard12R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - POEG Group B and the MSTPCRD.MSTPD13 bit security attribution
    #[inline(always)]
    pub fn psard13(&self) -> Psard13R {
        Psard13R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - POEG Group A and the MSTPCRD.MSTPD14 bit security attribution
    #[inline(always)]
    pub fn psard14(&self) -> Psard14R {
        Psard14R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - ADC121 and the MSTPCRD.MSTPD15 bit security attribution
    #[inline(always)]
    pub fn psard15(&self) -> Psard15R {
        Psard15R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - ADC120 and the MSTPCRD.MSTPD16 bit security attribution
    #[inline(always)]
    pub fn psard16(&self) -> Psard16R {
        Psard16R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 20 - DAC12 and the MSTPCRD.MSTPD20 bit security attribution
    #[inline(always)]
    pub fn psard20(&self) -> Psard20R {
        Psard20R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 22 - TSN and the MSTPCRD.MSTPD22 bit security attribution
    #[inline(always)]
    pub fn psard22(&self) -> Psard22R {
        Psard22R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSARD")
            .field("psard0", &self.psard0())
            .field("psard1", &self.psard1())
            .field("psard2", &self.psard2())
            .field("psard3", &self.psard3())
            .field("psard11", &self.psard11())
            .field("psard12", &self.psard12())
            .field("psard13", &self.psard13())
            .field("psard14", &self.psard14())
            .field("psard15", &self.psard15())
            .field("psard16", &self.psard16())
            .field("psard20", &self.psard20())
            .field("psard22", &self.psard22())
            .finish()
    }
}
impl W {
    ///Bit 0 - AGT3 and the MSTPCRD.MSTPD0 bit security attribution
    #[inline(always)]
    pub fn psard0(&mut self) -> Psard0W<PsardSpec> {
        Psard0W::new(self, 0)
    }
    ///Bit 1 - AGT2 and the MSTPCRD.MSTPD1 bit security attribution
    #[inline(always)]
    pub fn psard1(&mut self) -> Psard1W<PsardSpec> {
        Psard1W::new(self, 1)
    }
    ///Bit 2 - AGT1 and the MSTPCRD.MSTPD2 bit security attribution
    #[inline(always)]
    pub fn psard2(&mut self) -> Psard2W<PsardSpec> {
        Psard2W::new(self, 2)
    }
    ///Bit 3 - AGT0 and the MSTPCRD.MSTPD3 bit security attribution
    #[inline(always)]
    pub fn psard3(&mut self) -> Psard3W<PsardSpec> {
        Psard3W::new(self, 3)
    }
    ///Bit 11 - POEG Group D and the MSTPCRD.MSTPD11 bit security attribution
    #[inline(always)]
    pub fn psard11(&mut self) -> Psard11W<PsardSpec> {
        Psard11W::new(self, 11)
    }
    ///Bit 12 - POEG Group C and the MSTPCRD.MSTPD12 bit security attribution
    #[inline(always)]
    pub fn psard12(&mut self) -> Psard12W<PsardSpec> {
        Psard12W::new(self, 12)
    }
    ///Bit 13 - POEG Group B and the MSTPCRD.MSTPD13 bit security attribution
    #[inline(always)]
    pub fn psard13(&mut self) -> Psard13W<PsardSpec> {
        Psard13W::new(self, 13)
    }
    ///Bit 14 - POEG Group A and the MSTPCRD.MSTPD14 bit security attribution
    #[inline(always)]
    pub fn psard14(&mut self) -> Psard14W<PsardSpec> {
        Psard14W::new(self, 14)
    }
    ///Bit 15 - ADC121 and the MSTPCRD.MSTPD15 bit security attribution
    #[inline(always)]
    pub fn psard15(&mut self) -> Psard15W<PsardSpec> {
        Psard15W::new(self, 15)
    }
    ///Bit 16 - ADC120 and the MSTPCRD.MSTPD16 bit security attribution
    #[inline(always)]
    pub fn psard16(&mut self) -> Psard16W<PsardSpec> {
        Psard16W::new(self, 16)
    }
    ///Bit 20 - DAC12 and the MSTPCRD.MSTPD20 bit security attribution
    #[inline(always)]
    pub fn psard20(&mut self) -> Psard20W<PsardSpec> {
        Psard20W::new(self, 20)
    }
    ///Bit 22 - TSN and the MSTPCRD.MSTPD22 bit security attribution
    #[inline(always)]
    pub fn psard22(&mut self) -> Psard22W<PsardSpec> {
        Psard22W::new(self, 22)
    }
}
/**Peripheral Security Attribution Register D

You can [`read`](crate::Reg::read) this register and get [`psard::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psard::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PsardSpec;
impl crate::RegisterSpec for PsardSpec {
    type Ux = u32;
}
///`read()` method returns [`psard::R`](R) reader structure
impl crate::Readable for PsardSpec {}
///`write(|w| ..)` method takes [`psard::W`](W) writer structure
impl crate::Writable for PsardSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PSARD to value 0xffff_ffff
impl crate::Resettable for PsardSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
