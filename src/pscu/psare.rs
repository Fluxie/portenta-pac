///Register `PSARE` reader
pub type R = crate::R<PsareSpec>;
///Register `PSARE` writer
pub type W = crate::W<PsareSpec>;
/**WDT security attribution

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psare0 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Psare0> for bool {
    #[inline(always)]
    fn from(variant: Psare0) -> Self {
        variant as u8 != 0
    }
}
///Field `PSARE0` reader - WDT security attribution
pub type Psare0R = crate::BitReader<Psare0>;
impl Psare0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Psare0 {
        match self.bits {
            false => Psare0::_0,
            true => Psare0::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psare0::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psare0::_1
    }
}
///Field `PSARE0` writer - WDT security attribution
pub type Psare0W<'a, REG> = crate::BitWriter<'a, REG, Psare0>;
impl<'a, REG> Psare0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psare0::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psare0::_1)
    }
}
/**IWDT security attribution

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psare1 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Psare1> for bool {
    #[inline(always)]
    fn from(variant: Psare1) -> Self {
        variant as u8 != 0
    }
}
///Field `PSARE1` reader - IWDT security attribution
pub type Psare1R = crate::BitReader<Psare1>;
impl Psare1R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Psare1 {
        match self.bits {
            false => Psare1::_0,
            true => Psare1::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psare1::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psare1::_1
    }
}
///Field `PSARE1` writer - IWDT security attribution
pub type Psare1W<'a, REG> = crate::BitWriter<'a, REG, Psare1>;
impl<'a, REG> Psare1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psare1::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psare1::_1)
    }
}
/**RTC security attribution

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psare2 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Psare2> for bool {
    #[inline(always)]
    fn from(variant: Psare2) -> Self {
        variant as u8 != 0
    }
}
///Field `PSARE2` reader - RTC security attribution
pub type Psare2R = crate::BitReader<Psare2>;
impl Psare2R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Psare2 {
        match self.bits {
            false => Psare2::_0,
            true => Psare2::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psare2::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psare2::_1
    }
}
///Field `PSARE2` writer - RTC security attribution
pub type Psare2W<'a, REG> = crate::BitWriter<'a, REG, Psare2>;
impl<'a, REG> Psare2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psare2::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psare2::_1)
    }
}
/**AGT5 and the MSTPCRE.MSTPE14 bit security attribution

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psare14 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Psare14> for bool {
    #[inline(always)]
    fn from(variant: Psare14) -> Self {
        variant as u8 != 0
    }
}
///Field `PSARE14` reader - AGT5 and the MSTPCRE.MSTPE14 bit security attribution
pub type Psare14R = crate::BitReader<Psare14>;
impl Psare14R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Psare14 {
        match self.bits {
            false => Psare14::_0,
            true => Psare14::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psare14::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psare14::_1
    }
}
///Field `PSARE14` writer - AGT5 and the MSTPCRE.MSTPE14 bit security attribution
pub type Psare14W<'a, REG> = crate::BitWriter<'a, REG, Psare14>;
impl<'a, REG> Psare14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psare14::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psare14::_1)
    }
}
/**AGT4 and the MSTPCRE.MSTPE15 bit security attribution

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psare15 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Psare15> for bool {
    #[inline(always)]
    fn from(variant: Psare15) -> Self {
        variant as u8 != 0
    }
}
///Field `PSARE15` reader - AGT4 and the MSTPCRE.MSTPE15 bit security attribution
pub type Psare15R = crate::BitReader<Psare15>;
impl Psare15R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Psare15 {
        match self.bits {
            false => Psare15::_0,
            true => Psare15::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psare15::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psare15::_1
    }
}
///Field `PSARE15` writer - AGT4 and the MSTPCRE.MSTPE15 bit security attribution
pub type Psare15W<'a, REG> = crate::BitWriter<'a, REG, Psare15>;
impl<'a, REG> Psare15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psare15::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psare15::_1)
    }
}
/**GPT9 and the MSTPCRE.MSTPE22 bit security attribution

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psare22 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Psare22> for bool {
    #[inline(always)]
    fn from(variant: Psare22) -> Self {
        variant as u8 != 0
    }
}
///Field `PSARE22` reader - GPT9 and the MSTPCRE.MSTPE22 bit security attribution
pub type Psare22R = crate::BitReader<Psare22>;
impl Psare22R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Psare22 {
        match self.bits {
            false => Psare22::_0,
            true => Psare22::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psare22::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psare22::_1
    }
}
///Field `PSARE22` writer - GPT9 and the MSTPCRE.MSTPE22 bit security attribution
pub type Psare22W<'a, REG> = crate::BitWriter<'a, REG, Psare22>;
impl<'a, REG> Psare22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psare22::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psare22::_1)
    }
}
/**GPT8 and the MSTPCRE.MSTPE23 bit security attribution

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psare23 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Psare23> for bool {
    #[inline(always)]
    fn from(variant: Psare23) -> Self {
        variant as u8 != 0
    }
}
///Field `PSARE23` reader - GPT8 and the MSTPCRE.MSTPE23 bit security attribution
pub type Psare23R = crate::BitReader<Psare23>;
impl Psare23R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Psare23 {
        match self.bits {
            false => Psare23::_0,
            true => Psare23::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psare23::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psare23::_1
    }
}
///Field `PSARE23` writer - GPT8 and the MSTPCRE.MSTPE23 bit security attribution
pub type Psare23W<'a, REG> = crate::BitWriter<'a, REG, Psare23>;
impl<'a, REG> Psare23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psare23::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psare23::_1)
    }
}
/**GPT7 and the MSTPCRE.MSTPE24 bit security attribution

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psare24 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Psare24> for bool {
    #[inline(always)]
    fn from(variant: Psare24) -> Self {
        variant as u8 != 0
    }
}
///Field `PSARE24` reader - GPT7 and the MSTPCRE.MSTPE24 bit security attribution
pub type Psare24R = crate::BitReader<Psare24>;
impl Psare24R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Psare24 {
        match self.bits {
            false => Psare24::_0,
            true => Psare24::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psare24::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psare24::_1
    }
}
///Field `PSARE24` writer - GPT7 and the MSTPCRE.MSTPE24 bit security attribution
pub type Psare24W<'a, REG> = crate::BitWriter<'a, REG, Psare24>;
impl<'a, REG> Psare24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psare24::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psare24::_1)
    }
}
/**GPT6 and the MSTPCRE.MSTPE25 bit security attribution

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psare25 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Psare25> for bool {
    #[inline(always)]
    fn from(variant: Psare25) -> Self {
        variant as u8 != 0
    }
}
///Field `PSARE25` reader - GPT6 and the MSTPCRE.MSTPE25 bit security attribution
pub type Psare25R = crate::BitReader<Psare25>;
impl Psare25R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Psare25 {
        match self.bits {
            false => Psare25::_0,
            true => Psare25::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psare25::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psare25::_1
    }
}
///Field `PSARE25` writer - GPT6 and the MSTPCRE.MSTPE25 bit security attribution
pub type Psare25W<'a, REG> = crate::BitWriter<'a, REG, Psare25>;
impl<'a, REG> Psare25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psare25::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psare25::_1)
    }
}
/**GPT5 and the MSTPCRE.MSTPE26 bit security attribution

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psare26 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Psare26> for bool {
    #[inline(always)]
    fn from(variant: Psare26) -> Self {
        variant as u8 != 0
    }
}
///Field `PSARE26` reader - GPT5 and the MSTPCRE.MSTPE26 bit security attribution
pub type Psare26R = crate::BitReader<Psare26>;
impl Psare26R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Psare26 {
        match self.bits {
            false => Psare26::_0,
            true => Psare26::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psare26::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psare26::_1
    }
}
///Field `PSARE26` writer - GPT5 and the MSTPCRE.MSTPE26 bit security attribution
pub type Psare26W<'a, REG> = crate::BitWriter<'a, REG, Psare26>;
impl<'a, REG> Psare26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psare26::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psare26::_1)
    }
}
/**GPT4 and the MSTPCRE.MSTPE27 bit security attribution

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psare27 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Psare27> for bool {
    #[inline(always)]
    fn from(variant: Psare27) -> Self {
        variant as u8 != 0
    }
}
///Field `PSARE27` reader - GPT4 and the MSTPCRE.MSTPE27 bit security attribution
pub type Psare27R = crate::BitReader<Psare27>;
impl Psare27R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Psare27 {
        match self.bits {
            false => Psare27::_0,
            true => Psare27::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psare27::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psare27::_1
    }
}
///Field `PSARE27` writer - GPT4 and the MSTPCRE.MSTPE27 bit security attribution
pub type Psare27W<'a, REG> = crate::BitWriter<'a, REG, Psare27>;
impl<'a, REG> Psare27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psare27::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psare27::_1)
    }
}
/**GPT3 and the MSTPCRE.MSTPE28 bit security attribution

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psare28 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Psare28> for bool {
    #[inline(always)]
    fn from(variant: Psare28) -> Self {
        variant as u8 != 0
    }
}
///Field `PSARE28` reader - GPT3 and the MSTPCRE.MSTPE28 bit security attribution
pub type Psare28R = crate::BitReader<Psare28>;
impl Psare28R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Psare28 {
        match self.bits {
            false => Psare28::_0,
            true => Psare28::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psare28::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psare28::_1
    }
}
///Field `PSARE28` writer - GPT3 and the MSTPCRE.MSTPE28 bit security attribution
pub type Psare28W<'a, REG> = crate::BitWriter<'a, REG, Psare28>;
impl<'a, REG> Psare28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psare28::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psare28::_1)
    }
}
/**GPT2 and the MSTPCRE.MSTPE29 bit security attribution

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psare29 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Psare29> for bool {
    #[inline(always)]
    fn from(variant: Psare29) -> Self {
        variant as u8 != 0
    }
}
///Field `PSARE29` reader - GPT2 and the MSTPCRE.MSTPE29 bit security attribution
pub type Psare29R = crate::BitReader<Psare29>;
impl Psare29R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Psare29 {
        match self.bits {
            false => Psare29::_0,
            true => Psare29::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psare29::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psare29::_1
    }
}
///Field `PSARE29` writer - GPT2 and the MSTPCRE.MSTPE29 bit security attribution
pub type Psare29W<'a, REG> = crate::BitWriter<'a, REG, Psare29>;
impl<'a, REG> Psare29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psare29::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psare29::_1)
    }
}
/**GPT1 and the MSTPCRE.MSTPE30 bit security attribution

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psare30 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Psare30> for bool {
    #[inline(always)]
    fn from(variant: Psare30) -> Self {
        variant as u8 != 0
    }
}
///Field `PSARE30` reader - GPT1 and the MSTPCRE.MSTPE30 bit security attribution
pub type Psare30R = crate::BitReader<Psare30>;
impl Psare30R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Psare30 {
        match self.bits {
            false => Psare30::_0,
            true => Psare30::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psare30::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psare30::_1
    }
}
///Field `PSARE30` writer - GPT1 and the MSTPCRE.MSTPE30 bit security attribution
pub type Psare30W<'a, REG> = crate::BitWriter<'a, REG, Psare30>;
impl<'a, REG> Psare30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psare30::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psare30::_1)
    }
}
/**GPT0, GPT_OPS and the MSTPCRE.MSTPE31 bit security attribution

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psare31 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Psare31> for bool {
    #[inline(always)]
    fn from(variant: Psare31) -> Self {
        variant as u8 != 0
    }
}
///Field `PSARE31` reader - GPT0, GPT_OPS and the MSTPCRE.MSTPE31 bit security attribution
pub type Psare31R = crate::BitReader<Psare31>;
impl Psare31R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Psare31 {
        match self.bits {
            false => Psare31::_0,
            true => Psare31::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psare31::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psare31::_1
    }
}
///Field `PSARE31` writer - GPT0, GPT_OPS and the MSTPCRE.MSTPE31 bit security attribution
pub type Psare31W<'a, REG> = crate::BitWriter<'a, REG, Psare31>;
impl<'a, REG> Psare31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psare31::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psare31::_1)
    }
}
impl R {
    ///Bit 0 - WDT security attribution
    #[inline(always)]
    pub fn psare0(&self) -> Psare0R {
        Psare0R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - IWDT security attribution
    #[inline(always)]
    pub fn psare1(&self) -> Psare1R {
        Psare1R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - RTC security attribution
    #[inline(always)]
    pub fn psare2(&self) -> Psare2R {
        Psare2R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 14 - AGT5 and the MSTPCRE.MSTPE14 bit security attribution
    #[inline(always)]
    pub fn psare14(&self) -> Psare14R {
        Psare14R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - AGT4 and the MSTPCRE.MSTPE15 bit security attribution
    #[inline(always)]
    pub fn psare15(&self) -> Psare15R {
        Psare15R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 22 - GPT9 and the MSTPCRE.MSTPE22 bit security attribution
    #[inline(always)]
    pub fn psare22(&self) -> Psare22R {
        Psare22R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - GPT8 and the MSTPCRE.MSTPE23 bit security attribution
    #[inline(always)]
    pub fn psare23(&self) -> Psare23R {
        Psare23R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - GPT7 and the MSTPCRE.MSTPE24 bit security attribution
    #[inline(always)]
    pub fn psare24(&self) -> Psare24R {
        Psare24R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - GPT6 and the MSTPCRE.MSTPE25 bit security attribution
    #[inline(always)]
    pub fn psare25(&self) -> Psare25R {
        Psare25R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - GPT5 and the MSTPCRE.MSTPE26 bit security attribution
    #[inline(always)]
    pub fn psare26(&self) -> Psare26R {
        Psare26R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - GPT4 and the MSTPCRE.MSTPE27 bit security attribution
    #[inline(always)]
    pub fn psare27(&self) -> Psare27R {
        Psare27R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - GPT3 and the MSTPCRE.MSTPE28 bit security attribution
    #[inline(always)]
    pub fn psare28(&self) -> Psare28R {
        Psare28R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - GPT2 and the MSTPCRE.MSTPE29 bit security attribution
    #[inline(always)]
    pub fn psare29(&self) -> Psare29R {
        Psare29R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - GPT1 and the MSTPCRE.MSTPE30 bit security attribution
    #[inline(always)]
    pub fn psare30(&self) -> Psare30R {
        Psare30R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - GPT0, GPT_OPS and the MSTPCRE.MSTPE31 bit security attribution
    #[inline(always)]
    pub fn psare31(&self) -> Psare31R {
        Psare31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSARE")
            .field("psare0", &self.psare0())
            .field("psare1", &self.psare1())
            .field("psare2", &self.psare2())
            .field("psare14", &self.psare14())
            .field("psare15", &self.psare15())
            .field("psare22", &self.psare22())
            .field("psare23", &self.psare23())
            .field("psare24", &self.psare24())
            .field("psare25", &self.psare25())
            .field("psare26", &self.psare26())
            .field("psare27", &self.psare27())
            .field("psare28", &self.psare28())
            .field("psare29", &self.psare29())
            .field("psare30", &self.psare30())
            .field("psare31", &self.psare31())
            .finish()
    }
}
impl W {
    ///Bit 0 - WDT security attribution
    #[inline(always)]
    pub fn psare0(&mut self) -> Psare0W<PsareSpec> {
        Psare0W::new(self, 0)
    }
    ///Bit 1 - IWDT security attribution
    #[inline(always)]
    pub fn psare1(&mut self) -> Psare1W<PsareSpec> {
        Psare1W::new(self, 1)
    }
    ///Bit 2 - RTC security attribution
    #[inline(always)]
    pub fn psare2(&mut self) -> Psare2W<PsareSpec> {
        Psare2W::new(self, 2)
    }
    ///Bit 14 - AGT5 and the MSTPCRE.MSTPE14 bit security attribution
    #[inline(always)]
    pub fn psare14(&mut self) -> Psare14W<PsareSpec> {
        Psare14W::new(self, 14)
    }
    ///Bit 15 - AGT4 and the MSTPCRE.MSTPE15 bit security attribution
    #[inline(always)]
    pub fn psare15(&mut self) -> Psare15W<PsareSpec> {
        Psare15W::new(self, 15)
    }
    ///Bit 22 - GPT9 and the MSTPCRE.MSTPE22 bit security attribution
    #[inline(always)]
    pub fn psare22(&mut self) -> Psare22W<PsareSpec> {
        Psare22W::new(self, 22)
    }
    ///Bit 23 - GPT8 and the MSTPCRE.MSTPE23 bit security attribution
    #[inline(always)]
    pub fn psare23(&mut self) -> Psare23W<PsareSpec> {
        Psare23W::new(self, 23)
    }
    ///Bit 24 - GPT7 and the MSTPCRE.MSTPE24 bit security attribution
    #[inline(always)]
    pub fn psare24(&mut self) -> Psare24W<PsareSpec> {
        Psare24W::new(self, 24)
    }
    ///Bit 25 - GPT6 and the MSTPCRE.MSTPE25 bit security attribution
    #[inline(always)]
    pub fn psare25(&mut self) -> Psare25W<PsareSpec> {
        Psare25W::new(self, 25)
    }
    ///Bit 26 - GPT5 and the MSTPCRE.MSTPE26 bit security attribution
    #[inline(always)]
    pub fn psare26(&mut self) -> Psare26W<PsareSpec> {
        Psare26W::new(self, 26)
    }
    ///Bit 27 - GPT4 and the MSTPCRE.MSTPE27 bit security attribution
    #[inline(always)]
    pub fn psare27(&mut self) -> Psare27W<PsareSpec> {
        Psare27W::new(self, 27)
    }
    ///Bit 28 - GPT3 and the MSTPCRE.MSTPE28 bit security attribution
    #[inline(always)]
    pub fn psare28(&mut self) -> Psare28W<PsareSpec> {
        Psare28W::new(self, 28)
    }
    ///Bit 29 - GPT2 and the MSTPCRE.MSTPE29 bit security attribution
    #[inline(always)]
    pub fn psare29(&mut self) -> Psare29W<PsareSpec> {
        Psare29W::new(self, 29)
    }
    ///Bit 30 - GPT1 and the MSTPCRE.MSTPE30 bit security attribution
    #[inline(always)]
    pub fn psare30(&mut self) -> Psare30W<PsareSpec> {
        Psare30W::new(self, 30)
    }
    ///Bit 31 - GPT0, GPT_OPS and the MSTPCRE.MSTPE31 bit security attribution
    #[inline(always)]
    pub fn psare31(&mut self) -> Psare31W<PsareSpec> {
        Psare31W::new(self, 31)
    }
}
/**Peripheral Security Attribution Register E

You can [`read`](crate::Reg::read) this register and get [`psare::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psare::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PsareSpec;
impl crate::RegisterSpec for PsareSpec {
    type Ux = u32;
}
///`read()` method returns [`psare::R`](R) reader structure
impl crate::Readable for PsareSpec {}
///`write(|w| ..)` method takes [`psare::W`](W) writer structure
impl crate::Writable for PsareSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PSARE to value 0xffff_ffff
impl crate::Resettable for PsareSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
