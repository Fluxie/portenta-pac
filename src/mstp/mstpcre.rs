///Register `MSTPCRE` reader
pub type R = crate::R<MstpcreSpec>;
///Register `MSTPCRE` writer
pub type W = crate::W<MstpcreSpec>;
/**Low Power Asynchronous General Purpose Timer 5 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpe14 {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<Mstpe14> for bool {
    #[inline(always)]
    fn from(variant: Mstpe14) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPE14` reader - Low Power Asynchronous General Purpose Timer 5 Module Stop
pub type Mstpe14R = crate::BitReader<Mstpe14>;
impl Mstpe14R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mstpe14 {
        match self.bits {
            false => Mstpe14::_0,
            true => Mstpe14::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpe14::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpe14::_1
    }
}
///Field `MSTPE14` writer - Low Power Asynchronous General Purpose Timer 5 Module Stop
pub type Mstpe14W<'a, REG> = crate::BitWriter<'a, REG, Mstpe14>;
impl<'a, REG> Mstpe14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpe14::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpe14::_1)
    }
}
/**Low Power Asynchronous General Purpose Timer 4 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpe15 {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<Mstpe15> for bool {
    #[inline(always)]
    fn from(variant: Mstpe15) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPE15` reader - Low Power Asynchronous General Purpose Timer 4 Module Stop
pub type Mstpe15R = crate::BitReader<Mstpe15>;
impl Mstpe15R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mstpe15 {
        match self.bits {
            false => Mstpe15::_0,
            true => Mstpe15::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpe15::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpe15::_1
    }
}
///Field `MSTPE15` writer - Low Power Asynchronous General Purpose Timer 4 Module Stop
pub type Mstpe15W<'a, REG> = crate::BitWriter<'a, REG, Mstpe15>;
impl<'a, REG> Mstpe15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpe15::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpe15::_1)
    }
}
/**GPT9 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpe22 {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<Mstpe22> for bool {
    #[inline(always)]
    fn from(variant: Mstpe22) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPE22` reader - GPT9 Module Stop
pub type Mstpe22R = crate::BitReader<Mstpe22>;
impl Mstpe22R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mstpe22 {
        match self.bits {
            false => Mstpe22::_0,
            true => Mstpe22::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpe22::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpe22::_1
    }
}
///Field `MSTPE22` writer - GPT9 Module Stop
pub type Mstpe22W<'a, REG> = crate::BitWriter<'a, REG, Mstpe22>;
impl<'a, REG> Mstpe22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpe22::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpe22::_1)
    }
}
/**GPT8 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpe23 {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<Mstpe23> for bool {
    #[inline(always)]
    fn from(variant: Mstpe23) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPE23` reader - GPT8 Module Stop
pub type Mstpe23R = crate::BitReader<Mstpe23>;
impl Mstpe23R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mstpe23 {
        match self.bits {
            false => Mstpe23::_0,
            true => Mstpe23::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpe23::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpe23::_1
    }
}
///Field `MSTPE23` writer - GPT8 Module Stop
pub type Mstpe23W<'a, REG> = crate::BitWriter<'a, REG, Mstpe23>;
impl<'a, REG> Mstpe23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpe23::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpe23::_1)
    }
}
/**GPT7 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpe24 {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<Mstpe24> for bool {
    #[inline(always)]
    fn from(variant: Mstpe24) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPE24` reader - GPT7 Module Stop
pub type Mstpe24R = crate::BitReader<Mstpe24>;
impl Mstpe24R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mstpe24 {
        match self.bits {
            false => Mstpe24::_0,
            true => Mstpe24::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpe24::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpe24::_1
    }
}
///Field `MSTPE24` writer - GPT7 Module Stop
pub type Mstpe24W<'a, REG> = crate::BitWriter<'a, REG, Mstpe24>;
impl<'a, REG> Mstpe24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpe24::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpe24::_1)
    }
}
/**GPT6 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpe25 {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<Mstpe25> for bool {
    #[inline(always)]
    fn from(variant: Mstpe25) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPE25` reader - GPT6 Module Stop
pub type Mstpe25R = crate::BitReader<Mstpe25>;
impl Mstpe25R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mstpe25 {
        match self.bits {
            false => Mstpe25::_0,
            true => Mstpe25::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpe25::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpe25::_1
    }
}
///Field `MSTPE25` writer - GPT6 Module Stop
pub type Mstpe25W<'a, REG> = crate::BitWriter<'a, REG, Mstpe25>;
impl<'a, REG> Mstpe25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpe25::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpe25::_1)
    }
}
/**GPT5 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpe26 {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<Mstpe26> for bool {
    #[inline(always)]
    fn from(variant: Mstpe26) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPE26` reader - GPT5 Module Stop
pub type Mstpe26R = crate::BitReader<Mstpe26>;
impl Mstpe26R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mstpe26 {
        match self.bits {
            false => Mstpe26::_0,
            true => Mstpe26::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpe26::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpe26::_1
    }
}
///Field `MSTPE26` writer - GPT5 Module Stop
pub type Mstpe26W<'a, REG> = crate::BitWriter<'a, REG, Mstpe26>;
impl<'a, REG> Mstpe26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpe26::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpe26::_1)
    }
}
/**GPT4 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpe27 {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<Mstpe27> for bool {
    #[inline(always)]
    fn from(variant: Mstpe27) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPE27` reader - GPT4 Module Stop
pub type Mstpe27R = crate::BitReader<Mstpe27>;
impl Mstpe27R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mstpe27 {
        match self.bits {
            false => Mstpe27::_0,
            true => Mstpe27::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpe27::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpe27::_1
    }
}
///Field `MSTPE27` writer - GPT4 Module Stop
pub type Mstpe27W<'a, REG> = crate::BitWriter<'a, REG, Mstpe27>;
impl<'a, REG> Mstpe27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpe27::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpe27::_1)
    }
}
/**GPT3 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpe28 {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<Mstpe28> for bool {
    #[inline(always)]
    fn from(variant: Mstpe28) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPE28` reader - GPT3 Module Stop
pub type Mstpe28R = crate::BitReader<Mstpe28>;
impl Mstpe28R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mstpe28 {
        match self.bits {
            false => Mstpe28::_0,
            true => Mstpe28::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpe28::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpe28::_1
    }
}
///Field `MSTPE28` writer - GPT3 Module Stop
pub type Mstpe28W<'a, REG> = crate::BitWriter<'a, REG, Mstpe28>;
impl<'a, REG> Mstpe28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpe28::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpe28::_1)
    }
}
/**GPT2 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpe29 {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<Mstpe29> for bool {
    #[inline(always)]
    fn from(variant: Mstpe29) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPE29` reader - GPT2 Module Stop
pub type Mstpe29R = crate::BitReader<Mstpe29>;
impl Mstpe29R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mstpe29 {
        match self.bits {
            false => Mstpe29::_0,
            true => Mstpe29::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpe29::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpe29::_1
    }
}
///Field `MSTPE29` writer - GPT2 Module Stop
pub type Mstpe29W<'a, REG> = crate::BitWriter<'a, REG, Mstpe29>;
impl<'a, REG> Mstpe29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpe29::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpe29::_1)
    }
}
/**GPT1 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpe30 {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<Mstpe30> for bool {
    #[inline(always)]
    fn from(variant: Mstpe30) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPE30` reader - GPT1 Module Stop
pub type Mstpe30R = crate::BitReader<Mstpe30>;
impl Mstpe30R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mstpe30 {
        match self.bits {
            false => Mstpe30::_0,
            true => Mstpe30::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpe30::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpe30::_1
    }
}
///Field `MSTPE30` writer - GPT1 Module Stop
pub type Mstpe30W<'a, REG> = crate::BitWriter<'a, REG, Mstpe30>;
impl<'a, REG> Mstpe30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpe30::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpe30::_1)
    }
}
/**GPT0 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpe31 {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<Mstpe31> for bool {
    #[inline(always)]
    fn from(variant: Mstpe31) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPE31` reader - GPT0 Module Stop
pub type Mstpe31R = crate::BitReader<Mstpe31>;
impl Mstpe31R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mstpe31 {
        match self.bits {
            false => Mstpe31::_0,
            true => Mstpe31::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpe31::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpe31::_1
    }
}
///Field `MSTPE31` writer - GPT0 Module Stop
pub type Mstpe31W<'a, REG> = crate::BitWriter<'a, REG, Mstpe31>;
impl<'a, REG> Mstpe31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpe31::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpe31::_1)
    }
}
impl R {
    ///Bit 14 - Low Power Asynchronous General Purpose Timer 5 Module Stop
    #[inline(always)]
    pub fn mstpe14(&self) -> Mstpe14R {
        Mstpe14R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Low Power Asynchronous General Purpose Timer 4 Module Stop
    #[inline(always)]
    pub fn mstpe15(&self) -> Mstpe15R {
        Mstpe15R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 22 - GPT9 Module Stop
    #[inline(always)]
    pub fn mstpe22(&self) -> Mstpe22R {
        Mstpe22R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - GPT8 Module Stop
    #[inline(always)]
    pub fn mstpe23(&self) -> Mstpe23R {
        Mstpe23R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - GPT7 Module Stop
    #[inline(always)]
    pub fn mstpe24(&self) -> Mstpe24R {
        Mstpe24R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - GPT6 Module Stop
    #[inline(always)]
    pub fn mstpe25(&self) -> Mstpe25R {
        Mstpe25R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - GPT5 Module Stop
    #[inline(always)]
    pub fn mstpe26(&self) -> Mstpe26R {
        Mstpe26R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - GPT4 Module Stop
    #[inline(always)]
    pub fn mstpe27(&self) -> Mstpe27R {
        Mstpe27R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - GPT3 Module Stop
    #[inline(always)]
    pub fn mstpe28(&self) -> Mstpe28R {
        Mstpe28R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - GPT2 Module Stop
    #[inline(always)]
    pub fn mstpe29(&self) -> Mstpe29R {
        Mstpe29R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - GPT1 Module Stop
    #[inline(always)]
    pub fn mstpe30(&self) -> Mstpe30R {
        Mstpe30R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - GPT0 Module Stop
    #[inline(always)]
    pub fn mstpe31(&self) -> Mstpe31R {
        Mstpe31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MSTPCRE")
            .field("mstpe14", &self.mstpe14())
            .field("mstpe15", &self.mstpe15())
            .field("mstpe22", &self.mstpe22())
            .field("mstpe23", &self.mstpe23())
            .field("mstpe24", &self.mstpe24())
            .field("mstpe25", &self.mstpe25())
            .field("mstpe26", &self.mstpe26())
            .field("mstpe27", &self.mstpe27())
            .field("mstpe28", &self.mstpe28())
            .field("mstpe29", &self.mstpe29())
            .field("mstpe30", &self.mstpe30())
            .field("mstpe31", &self.mstpe31())
            .finish()
    }
}
impl W {
    ///Bit 14 - Low Power Asynchronous General Purpose Timer 5 Module Stop
    #[inline(always)]
    pub fn mstpe14(&mut self) -> Mstpe14W<MstpcreSpec> {
        Mstpe14W::new(self, 14)
    }
    ///Bit 15 - Low Power Asynchronous General Purpose Timer 4 Module Stop
    #[inline(always)]
    pub fn mstpe15(&mut self) -> Mstpe15W<MstpcreSpec> {
        Mstpe15W::new(self, 15)
    }
    ///Bit 22 - GPT9 Module Stop
    #[inline(always)]
    pub fn mstpe22(&mut self) -> Mstpe22W<MstpcreSpec> {
        Mstpe22W::new(self, 22)
    }
    ///Bit 23 - GPT8 Module Stop
    #[inline(always)]
    pub fn mstpe23(&mut self) -> Mstpe23W<MstpcreSpec> {
        Mstpe23W::new(self, 23)
    }
    ///Bit 24 - GPT7 Module Stop
    #[inline(always)]
    pub fn mstpe24(&mut self) -> Mstpe24W<MstpcreSpec> {
        Mstpe24W::new(self, 24)
    }
    ///Bit 25 - GPT6 Module Stop
    #[inline(always)]
    pub fn mstpe25(&mut self) -> Mstpe25W<MstpcreSpec> {
        Mstpe25W::new(self, 25)
    }
    ///Bit 26 - GPT5 Module Stop
    #[inline(always)]
    pub fn mstpe26(&mut self) -> Mstpe26W<MstpcreSpec> {
        Mstpe26W::new(self, 26)
    }
    ///Bit 27 - GPT4 Module Stop
    #[inline(always)]
    pub fn mstpe27(&mut self) -> Mstpe27W<MstpcreSpec> {
        Mstpe27W::new(self, 27)
    }
    ///Bit 28 - GPT3 Module Stop
    #[inline(always)]
    pub fn mstpe28(&mut self) -> Mstpe28W<MstpcreSpec> {
        Mstpe28W::new(self, 28)
    }
    ///Bit 29 - GPT2 Module Stop
    #[inline(always)]
    pub fn mstpe29(&mut self) -> Mstpe29W<MstpcreSpec> {
        Mstpe29W::new(self, 29)
    }
    ///Bit 30 - GPT1 Module Stop
    #[inline(always)]
    pub fn mstpe30(&mut self) -> Mstpe30W<MstpcreSpec> {
        Mstpe30W::new(self, 30)
    }
    ///Bit 31 - GPT0 Module Stop
    #[inline(always)]
    pub fn mstpe31(&mut self) -> Mstpe31W<MstpcreSpec> {
        Mstpe31W::new(self, 31)
    }
}
/**Module Stop Control Register E

You can [`read`](crate::Reg::read) this register and get [`mstpcre::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mstpcre::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MstpcreSpec;
impl crate::RegisterSpec for MstpcreSpec {
    type Ux = u32;
}
///`read()` method returns [`mstpcre::R`](R) reader structure
impl crate::Readable for MstpcreSpec {}
///`write(|w| ..)` method takes [`mstpcre::W`](W) writer structure
impl crate::Writable for MstpcreSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MSTPCRE to value 0xffff_ffff
impl crate::Resettable for MstpcreSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
