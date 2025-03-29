///Register `ADADS1` reader
pub type R = crate::R<Adads1Spec>;
///Register `ADADS1` writer
pub type W = crate::W<Adads1Spec>;
/**A/D-Converted Value Addition/Average Channel Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ads16 {
    ///0: Do not select associated input channel.
    _0 = 0,
    ///1: Select associated input channel.
    _1 = 1,
}
impl From<Ads16> for bool {
    #[inline(always)]
    fn from(variant: Ads16) -> Self {
        variant as u8 != 0
    }
}
///Field `ADS16` reader - A/D-Converted Value Addition/Average Channel Select
pub type Ads16R = crate::BitReader<Ads16>;
impl Ads16R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ads16 {
        match self.bits {
            false => Ads16::_0,
            true => Ads16::_1,
        }
    }
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ads16::_0
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ads16::_1
    }
}
///Field `ADS16` writer - A/D-Converted Value Addition/Average Channel Select
pub type Ads16W<'a, REG> = crate::BitWriter<'a, REG, Ads16>;
impl<'a, REG> Ads16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ads16::_0)
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ads16::_1)
    }
}
/**A/D-Converted Value Addition/Average Channel Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ads17 {
    ///0: Do not select associated input channel.
    _0 = 0,
    ///1: Select associated input channel.
    _1 = 1,
}
impl From<Ads17> for bool {
    #[inline(always)]
    fn from(variant: Ads17) -> Self {
        variant as u8 != 0
    }
}
///Field `ADS17` reader - A/D-Converted Value Addition/Average Channel Select
pub type Ads17R = crate::BitReader<Ads17>;
impl Ads17R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ads17 {
        match self.bits {
            false => Ads17::_0,
            true => Ads17::_1,
        }
    }
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ads17::_0
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ads17::_1
    }
}
///Field `ADS17` writer - A/D-Converted Value Addition/Average Channel Select
pub type Ads17W<'a, REG> = crate::BitWriter<'a, REG, Ads17>;
impl<'a, REG> Ads17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ads17::_0)
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ads17::_1)
    }
}
/**A/D-Converted Value Addition/Average Channel Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ads18 {
    ///0: Do not select associated input channel.
    _0 = 0,
    ///1: Select associated input channel.
    _1 = 1,
}
impl From<Ads18> for bool {
    #[inline(always)]
    fn from(variant: Ads18) -> Self {
        variant as u8 != 0
    }
}
///Field `ADS18` reader - A/D-Converted Value Addition/Average Channel Select
pub type Ads18R = crate::BitReader<Ads18>;
impl Ads18R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ads18 {
        match self.bits {
            false => Ads18::_0,
            true => Ads18::_1,
        }
    }
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ads18::_0
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ads18::_1
    }
}
///Field `ADS18` writer - A/D-Converted Value Addition/Average Channel Select
pub type Ads18W<'a, REG> = crate::BitWriter<'a, REG, Ads18>;
impl<'a, REG> Ads18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ads18::_0)
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ads18::_1)
    }
}
/**A/D-Converted Value Addition/Average Channel Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ads19 {
    ///0: Do not select associated input channel.
    _0 = 0,
    ///1: Select associated input channel.
    _1 = 1,
}
impl From<Ads19> for bool {
    #[inline(always)]
    fn from(variant: Ads19) -> Self {
        variant as u8 != 0
    }
}
///Field `ADS19` reader - A/D-Converted Value Addition/Average Channel Select
pub type Ads19R = crate::BitReader<Ads19>;
impl Ads19R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ads19 {
        match self.bits {
            false => Ads19::_0,
            true => Ads19::_1,
        }
    }
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ads19::_0
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ads19::_1
    }
}
///Field `ADS19` writer - A/D-Converted Value Addition/Average Channel Select
pub type Ads19W<'a, REG> = crate::BitWriter<'a, REG, Ads19>;
impl<'a, REG> Ads19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ads19::_0)
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ads19::_1)
    }
}
/**A/D-Converted Value Addition/Average Channel Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ads20 {
    ///0: Do not select associated input channel.
    _0 = 0,
    ///1: Select associated input channel.
    _1 = 1,
}
impl From<Ads20> for bool {
    #[inline(always)]
    fn from(variant: Ads20) -> Self {
        variant as u8 != 0
    }
}
///Field `ADS20` reader - A/D-Converted Value Addition/Average Channel Select
pub type Ads20R = crate::BitReader<Ads20>;
impl Ads20R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ads20 {
        match self.bits {
            false => Ads20::_0,
            true => Ads20::_1,
        }
    }
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ads20::_0
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ads20::_1
    }
}
///Field `ADS20` writer - A/D-Converted Value Addition/Average Channel Select
pub type Ads20W<'a, REG> = crate::BitWriter<'a, REG, Ads20>;
impl<'a, REG> Ads20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ads20::_0)
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ads20::_1)
    }
}
/**A/D-Converted Value Addition/Average Channel Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ads21 {
    ///0: Do not select associated input channel.
    _0 = 0,
    ///1: Select associated input channel.
    _1 = 1,
}
impl From<Ads21> for bool {
    #[inline(always)]
    fn from(variant: Ads21) -> Self {
        variant as u8 != 0
    }
}
///Field `ADS21` reader - A/D-Converted Value Addition/Average Channel Select
pub type Ads21R = crate::BitReader<Ads21>;
impl Ads21R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ads21 {
        match self.bits {
            false => Ads21::_0,
            true => Ads21::_1,
        }
    }
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ads21::_0
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ads21::_1
    }
}
///Field `ADS21` writer - A/D-Converted Value Addition/Average Channel Select
pub type Ads21W<'a, REG> = crate::BitWriter<'a, REG, Ads21>;
impl<'a, REG> Ads21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ads21::_0)
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ads21::_1)
    }
}
/**A/D-Converted Value Addition/Average Channel Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ads22 {
    ///0: Do not select associated input channel.
    _0 = 0,
    ///1: Select associated input channel.
    _1 = 1,
}
impl From<Ads22> for bool {
    #[inline(always)]
    fn from(variant: Ads22) -> Self {
        variant as u8 != 0
    }
}
///Field `ADS22` reader - A/D-Converted Value Addition/Average Channel Select
pub type Ads22R = crate::BitReader<Ads22>;
impl Ads22R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ads22 {
        match self.bits {
            false => Ads22::_0,
            true => Ads22::_1,
        }
    }
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ads22::_0
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ads22::_1
    }
}
///Field `ADS22` writer - A/D-Converted Value Addition/Average Channel Select
pub type Ads22W<'a, REG> = crate::BitWriter<'a, REG, Ads22>;
impl<'a, REG> Ads22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ads22::_0)
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ads22::_1)
    }
}
/**A/D-Converted Value Addition/Average Channel Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ads23 {
    ///0: Do not select associated input channel.
    _0 = 0,
    ///1: Select associated input channel.
    _1 = 1,
}
impl From<Ads23> for bool {
    #[inline(always)]
    fn from(variant: Ads23) -> Self {
        variant as u8 != 0
    }
}
///Field `ADS23` reader - A/D-Converted Value Addition/Average Channel Select
pub type Ads23R = crate::BitReader<Ads23>;
impl Ads23R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ads23 {
        match self.bits {
            false => Ads23::_0,
            true => Ads23::_1,
        }
    }
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ads23::_0
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ads23::_1
    }
}
///Field `ADS23` writer - A/D-Converted Value Addition/Average Channel Select
pub type Ads23W<'a, REG> = crate::BitWriter<'a, REG, Ads23>;
impl<'a, REG> Ads23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ads23::_0)
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ads23::_1)
    }
}
/**A/D-Converted Value Addition/Average Channel Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ads24 {
    ///0: Do not select associated input channel.
    _0 = 0,
    ///1: Select associated input channel.
    _1 = 1,
}
impl From<Ads24> for bool {
    #[inline(always)]
    fn from(variant: Ads24) -> Self {
        variant as u8 != 0
    }
}
///Field `ADS24` reader - A/D-Converted Value Addition/Average Channel Select
pub type Ads24R = crate::BitReader<Ads24>;
impl Ads24R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ads24 {
        match self.bits {
            false => Ads24::_0,
            true => Ads24::_1,
        }
    }
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ads24::_0
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ads24::_1
    }
}
///Field `ADS24` writer - A/D-Converted Value Addition/Average Channel Select
pub type Ads24W<'a, REG> = crate::BitWriter<'a, REG, Ads24>;
impl<'a, REG> Ads24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ads24::_0)
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ads24::_1)
    }
}
/**A/D-Converted Value Addition/Average Channel Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ads25 {
    ///0: Do not select associated input channel.
    _0 = 0,
    ///1: Select associated input channel.
    _1 = 1,
}
impl From<Ads25> for bool {
    #[inline(always)]
    fn from(variant: Ads25) -> Self {
        variant as u8 != 0
    }
}
///Field `ADS25` reader - A/D-Converted Value Addition/Average Channel Select
pub type Ads25R = crate::BitReader<Ads25>;
impl Ads25R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ads25 {
        match self.bits {
            false => Ads25::_0,
            true => Ads25::_1,
        }
    }
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ads25::_0
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ads25::_1
    }
}
///Field `ADS25` writer - A/D-Converted Value Addition/Average Channel Select
pub type Ads25W<'a, REG> = crate::BitWriter<'a, REG, Ads25>;
impl<'a, REG> Ads25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ads25::_0)
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ads25::_1)
    }
}
/**A/D-Converted Value Addition/Average Channel Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ads26 {
    ///0: Do not select associated input channel.
    _0 = 0,
    ///1: Select associated input channel.
    _1 = 1,
}
impl From<Ads26> for bool {
    #[inline(always)]
    fn from(variant: Ads26) -> Self {
        variant as u8 != 0
    }
}
///Field `ADS26` reader - A/D-Converted Value Addition/Average Channel Select
pub type Ads26R = crate::BitReader<Ads26>;
impl Ads26R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ads26 {
        match self.bits {
            false => Ads26::_0,
            true => Ads26::_1,
        }
    }
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ads26::_0
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ads26::_1
    }
}
///Field `ADS26` writer - A/D-Converted Value Addition/Average Channel Select
pub type Ads26W<'a, REG> = crate::BitWriter<'a, REG, Ads26>;
impl<'a, REG> Ads26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ads26::_0)
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ads26::_1)
    }
}
/**A/D-Converted Value Addition/Average Channel Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ads27 {
    ///0: Do not select associated input channel.
    _0 = 0,
    ///1: Select associated input channel.
    _1 = 1,
}
impl From<Ads27> for bool {
    #[inline(always)]
    fn from(variant: Ads27) -> Self {
        variant as u8 != 0
    }
}
///Field `ADS27` reader - A/D-Converted Value Addition/Average Channel Select
pub type Ads27R = crate::BitReader<Ads27>;
impl Ads27R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ads27 {
        match self.bits {
            false => Ads27::_0,
            true => Ads27::_1,
        }
    }
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ads27::_0
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ads27::_1
    }
}
///Field `ADS27` writer - A/D-Converted Value Addition/Average Channel Select
pub type Ads27W<'a, REG> = crate::BitWriter<'a, REG, Ads27>;
impl<'a, REG> Ads27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ads27::_0)
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ads27::_1)
    }
}
/**A/D-Converted Value Addition/Average Channel Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ads28 {
    ///0: Do not select associated input channel.
    _0 = 0,
    ///1: Select associated input channel.
    _1 = 1,
}
impl From<Ads28> for bool {
    #[inline(always)]
    fn from(variant: Ads28) -> Self {
        variant as u8 != 0
    }
}
///Field `ADS28` reader - A/D-Converted Value Addition/Average Channel Select
pub type Ads28R = crate::BitReader<Ads28>;
impl Ads28R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ads28 {
        match self.bits {
            false => Ads28::_0,
            true => Ads28::_1,
        }
    }
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ads28::_0
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ads28::_1
    }
}
///Field `ADS28` writer - A/D-Converted Value Addition/Average Channel Select
pub type Ads28W<'a, REG> = crate::BitWriter<'a, REG, Ads28>;
impl<'a, REG> Ads28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ads28::_0)
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ads28::_1)
    }
}
impl R {
    ///Bit 0 - A/D-Converted Value Addition/Average Channel Select
    #[inline(always)]
    pub fn ads16(&self) -> Ads16R {
        Ads16R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - A/D-Converted Value Addition/Average Channel Select
    #[inline(always)]
    pub fn ads17(&self) -> Ads17R {
        Ads17R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - A/D-Converted Value Addition/Average Channel Select
    #[inline(always)]
    pub fn ads18(&self) -> Ads18R {
        Ads18R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - A/D-Converted Value Addition/Average Channel Select
    #[inline(always)]
    pub fn ads19(&self) -> Ads19R {
        Ads19R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - A/D-Converted Value Addition/Average Channel Select
    #[inline(always)]
    pub fn ads20(&self) -> Ads20R {
        Ads20R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - A/D-Converted Value Addition/Average Channel Select
    #[inline(always)]
    pub fn ads21(&self) -> Ads21R {
        Ads21R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - A/D-Converted Value Addition/Average Channel Select
    #[inline(always)]
    pub fn ads22(&self) -> Ads22R {
        Ads22R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - A/D-Converted Value Addition/Average Channel Select
    #[inline(always)]
    pub fn ads23(&self) -> Ads23R {
        Ads23R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - A/D-Converted Value Addition/Average Channel Select
    #[inline(always)]
    pub fn ads24(&self) -> Ads24R {
        Ads24R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - A/D-Converted Value Addition/Average Channel Select
    #[inline(always)]
    pub fn ads25(&self) -> Ads25R {
        Ads25R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - A/D-Converted Value Addition/Average Channel Select
    #[inline(always)]
    pub fn ads26(&self) -> Ads26R {
        Ads26R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - A/D-Converted Value Addition/Average Channel Select
    #[inline(always)]
    pub fn ads27(&self) -> Ads27R {
        Ads27R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - A/D-Converted Value Addition/Average Channel Select
    #[inline(always)]
    pub fn ads28(&self) -> Ads28R {
        Ads28R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADADS1")
            .field("ads16", &self.ads16())
            .field("ads17", &self.ads17())
            .field("ads18", &self.ads18())
            .field("ads19", &self.ads19())
            .field("ads20", &self.ads20())
            .field("ads21", &self.ads21())
            .field("ads22", &self.ads22())
            .field("ads23", &self.ads23())
            .field("ads24", &self.ads24())
            .field("ads25", &self.ads25())
            .field("ads26", &self.ads26())
            .field("ads27", &self.ads27())
            .field("ads28", &self.ads28())
            .finish()
    }
}
impl W {
    ///Bit 0 - A/D-Converted Value Addition/Average Channel Select
    #[inline(always)]
    pub fn ads16(&mut self) -> Ads16W<Adads1Spec> {
        Ads16W::new(self, 0)
    }
    ///Bit 1 - A/D-Converted Value Addition/Average Channel Select
    #[inline(always)]
    pub fn ads17(&mut self) -> Ads17W<Adads1Spec> {
        Ads17W::new(self, 1)
    }
    ///Bit 2 - A/D-Converted Value Addition/Average Channel Select
    #[inline(always)]
    pub fn ads18(&mut self) -> Ads18W<Adads1Spec> {
        Ads18W::new(self, 2)
    }
    ///Bit 3 - A/D-Converted Value Addition/Average Channel Select
    #[inline(always)]
    pub fn ads19(&mut self) -> Ads19W<Adads1Spec> {
        Ads19W::new(self, 3)
    }
    ///Bit 4 - A/D-Converted Value Addition/Average Channel Select
    #[inline(always)]
    pub fn ads20(&mut self) -> Ads20W<Adads1Spec> {
        Ads20W::new(self, 4)
    }
    ///Bit 5 - A/D-Converted Value Addition/Average Channel Select
    #[inline(always)]
    pub fn ads21(&mut self) -> Ads21W<Adads1Spec> {
        Ads21W::new(self, 5)
    }
    ///Bit 6 - A/D-Converted Value Addition/Average Channel Select
    #[inline(always)]
    pub fn ads22(&mut self) -> Ads22W<Adads1Spec> {
        Ads22W::new(self, 6)
    }
    ///Bit 7 - A/D-Converted Value Addition/Average Channel Select
    #[inline(always)]
    pub fn ads23(&mut self) -> Ads23W<Adads1Spec> {
        Ads23W::new(self, 7)
    }
    ///Bit 8 - A/D-Converted Value Addition/Average Channel Select
    #[inline(always)]
    pub fn ads24(&mut self) -> Ads24W<Adads1Spec> {
        Ads24W::new(self, 8)
    }
    ///Bit 9 - A/D-Converted Value Addition/Average Channel Select
    #[inline(always)]
    pub fn ads25(&mut self) -> Ads25W<Adads1Spec> {
        Ads25W::new(self, 9)
    }
    ///Bit 10 - A/D-Converted Value Addition/Average Channel Select
    #[inline(always)]
    pub fn ads26(&mut self) -> Ads26W<Adads1Spec> {
        Ads26W::new(self, 10)
    }
    ///Bit 11 - A/D-Converted Value Addition/Average Channel Select
    #[inline(always)]
    pub fn ads27(&mut self) -> Ads27W<Adads1Spec> {
        Ads27W::new(self, 11)
    }
    ///Bit 12 - A/D-Converted Value Addition/Average Channel Select
    #[inline(always)]
    pub fn ads28(&mut self) -> Ads28W<Adads1Spec> {
        Ads28W::new(self, 12)
    }
}
/**A/D-Converted Value Addition/Average Channel Select Register 1

You can [`read`](crate::Reg::read) this register and get [`adads1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adads1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Adads1Spec;
impl crate::RegisterSpec for Adads1Spec {
    type Ux = u16;
}
///`read()` method returns [`adads1::R`](R) reader structure
impl crate::Readable for Adads1Spec {}
///`write(|w| ..)` method takes [`adads1::W`](W) writer structure
impl crate::Writable for Adads1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADADS1 to value 0
impl crate::Resettable for Adads1Spec {}
