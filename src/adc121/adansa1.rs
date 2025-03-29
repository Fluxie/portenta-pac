///Register `ADANSA1` reader
pub type R = crate::R<Adansa1Spec>;
///Register `ADANSA1` writer
pub type W = crate::W<Adansa1Spec>;
/**A/D Conversion Channels Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansa16 {
    ///0: Do not select associated input channel.
    _0 = 0,
    ///1: Select associated input channel.
    _1 = 1,
}
impl From<Ansa16> for bool {
    #[inline(always)]
    fn from(variant: Ansa16) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSA16` reader - A/D Conversion Channels Select
pub type Ansa16R = crate::BitReader<Ansa16>;
impl Ansa16R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ansa16 {
        match self.bits {
            false => Ansa16::_0,
            true => Ansa16::_1,
        }
    }
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansa16::_0
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansa16::_1
    }
}
///Field `ANSA16` writer - A/D Conversion Channels Select
pub type Ansa16W<'a, REG> = crate::BitWriter<'a, REG, Ansa16>;
impl<'a, REG> Ansa16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa16::_0)
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa16::_1)
    }
}
/**A/D Conversion Channels Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansa17 {
    ///0: Do not select associated input channel.
    _0 = 0,
    ///1: Select associated input channel.
    _1 = 1,
}
impl From<Ansa17> for bool {
    #[inline(always)]
    fn from(variant: Ansa17) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSA17` reader - A/D Conversion Channels Select
pub type Ansa17R = crate::BitReader<Ansa17>;
impl Ansa17R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ansa17 {
        match self.bits {
            false => Ansa17::_0,
            true => Ansa17::_1,
        }
    }
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansa17::_0
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansa17::_1
    }
}
///Field `ANSA17` writer - A/D Conversion Channels Select
pub type Ansa17W<'a, REG> = crate::BitWriter<'a, REG, Ansa17>;
impl<'a, REG> Ansa17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa17::_0)
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa17::_1)
    }
}
/**A/D Conversion Channels Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansa18 {
    ///0: Do not select associated input channel.
    _0 = 0,
    ///1: Select associated input channel.
    _1 = 1,
}
impl From<Ansa18> for bool {
    #[inline(always)]
    fn from(variant: Ansa18) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSA18` reader - A/D Conversion Channels Select
pub type Ansa18R = crate::BitReader<Ansa18>;
impl Ansa18R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ansa18 {
        match self.bits {
            false => Ansa18::_0,
            true => Ansa18::_1,
        }
    }
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansa18::_0
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansa18::_1
    }
}
///Field `ANSA18` writer - A/D Conversion Channels Select
pub type Ansa18W<'a, REG> = crate::BitWriter<'a, REG, Ansa18>;
impl<'a, REG> Ansa18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa18::_0)
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa18::_1)
    }
}
/**A/D Conversion Channels Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansa19 {
    ///0: Do not select associated input channel.
    _0 = 0,
    ///1: Select associated input channel.
    _1 = 1,
}
impl From<Ansa19> for bool {
    #[inline(always)]
    fn from(variant: Ansa19) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSA19` reader - A/D Conversion Channels Select
pub type Ansa19R = crate::BitReader<Ansa19>;
impl Ansa19R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ansa19 {
        match self.bits {
            false => Ansa19::_0,
            true => Ansa19::_1,
        }
    }
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansa19::_0
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansa19::_1
    }
}
///Field `ANSA19` writer - A/D Conversion Channels Select
pub type Ansa19W<'a, REG> = crate::BitWriter<'a, REG, Ansa19>;
impl<'a, REG> Ansa19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa19::_0)
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa19::_1)
    }
}
/**A/D Conversion Channels Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansa20 {
    ///0: Do not select associated input channel.
    _0 = 0,
    ///1: Select associated input channel.
    _1 = 1,
}
impl From<Ansa20> for bool {
    #[inline(always)]
    fn from(variant: Ansa20) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSA20` reader - A/D Conversion Channels Select
pub type Ansa20R = crate::BitReader<Ansa20>;
impl Ansa20R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ansa20 {
        match self.bits {
            false => Ansa20::_0,
            true => Ansa20::_1,
        }
    }
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansa20::_0
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansa20::_1
    }
}
///Field `ANSA20` writer - A/D Conversion Channels Select
pub type Ansa20W<'a, REG> = crate::BitWriter<'a, REG, Ansa20>;
impl<'a, REG> Ansa20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa20::_0)
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa20::_1)
    }
}
/**A/D Conversion Channels Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansa21 {
    ///0: Do not select associated input channel.
    _0 = 0,
    ///1: Select associated input channel.
    _1 = 1,
}
impl From<Ansa21> for bool {
    #[inline(always)]
    fn from(variant: Ansa21) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSA21` reader - A/D Conversion Channels Select
pub type Ansa21R = crate::BitReader<Ansa21>;
impl Ansa21R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ansa21 {
        match self.bits {
            false => Ansa21::_0,
            true => Ansa21::_1,
        }
    }
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansa21::_0
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansa21::_1
    }
}
///Field `ANSA21` writer - A/D Conversion Channels Select
pub type Ansa21W<'a, REG> = crate::BitWriter<'a, REG, Ansa21>;
impl<'a, REG> Ansa21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa21::_0)
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa21::_1)
    }
}
/**A/D Conversion Channels Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansa22 {
    ///0: Do not select associated input channel.
    _0 = 0,
    ///1: Select associated input channel.
    _1 = 1,
}
impl From<Ansa22> for bool {
    #[inline(always)]
    fn from(variant: Ansa22) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSA22` reader - A/D Conversion Channels Select
pub type Ansa22R = crate::BitReader<Ansa22>;
impl Ansa22R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ansa22 {
        match self.bits {
            false => Ansa22::_0,
            true => Ansa22::_1,
        }
    }
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansa22::_0
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansa22::_1
    }
}
///Field `ANSA22` writer - A/D Conversion Channels Select
pub type Ansa22W<'a, REG> = crate::BitWriter<'a, REG, Ansa22>;
impl<'a, REG> Ansa22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa22::_0)
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa22::_1)
    }
}
/**A/D Conversion Channels Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansa23 {
    ///0: Do not select associated input channel.
    _0 = 0,
    ///1: Select associated input channel.
    _1 = 1,
}
impl From<Ansa23> for bool {
    #[inline(always)]
    fn from(variant: Ansa23) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSA23` reader - A/D Conversion Channels Select
pub type Ansa23R = crate::BitReader<Ansa23>;
impl Ansa23R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ansa23 {
        match self.bits {
            false => Ansa23::_0,
            true => Ansa23::_1,
        }
    }
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansa23::_0
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansa23::_1
    }
}
///Field `ANSA23` writer - A/D Conversion Channels Select
pub type Ansa23W<'a, REG> = crate::BitWriter<'a, REG, Ansa23>;
impl<'a, REG> Ansa23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa23::_0)
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa23::_1)
    }
}
/**A/D Conversion Channels Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansa24 {
    ///0: Do not select associated input channel.
    _0 = 0,
    ///1: Select associated input channel.
    _1 = 1,
}
impl From<Ansa24> for bool {
    #[inline(always)]
    fn from(variant: Ansa24) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSA24` reader - A/D Conversion Channels Select
pub type Ansa24R = crate::BitReader<Ansa24>;
impl Ansa24R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ansa24 {
        match self.bits {
            false => Ansa24::_0,
            true => Ansa24::_1,
        }
    }
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansa24::_0
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansa24::_1
    }
}
///Field `ANSA24` writer - A/D Conversion Channels Select
pub type Ansa24W<'a, REG> = crate::BitWriter<'a, REG, Ansa24>;
impl<'a, REG> Ansa24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa24::_0)
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa24::_1)
    }
}
/**A/D Conversion Channels Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansa25 {
    ///0: Do not select associated input channel.
    _0 = 0,
    ///1: Select associated input channel.
    _1 = 1,
}
impl From<Ansa25> for bool {
    #[inline(always)]
    fn from(variant: Ansa25) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSA25` reader - A/D Conversion Channels Select
pub type Ansa25R = crate::BitReader<Ansa25>;
impl Ansa25R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ansa25 {
        match self.bits {
            false => Ansa25::_0,
            true => Ansa25::_1,
        }
    }
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansa25::_0
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansa25::_1
    }
}
///Field `ANSA25` writer - A/D Conversion Channels Select
pub type Ansa25W<'a, REG> = crate::BitWriter<'a, REG, Ansa25>;
impl<'a, REG> Ansa25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa25::_0)
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa25::_1)
    }
}
/**A/D Conversion Channels Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansa26 {
    ///0: Do not select associated input channel.
    _0 = 0,
    ///1: Select associated input channel.
    _1 = 1,
}
impl From<Ansa26> for bool {
    #[inline(always)]
    fn from(variant: Ansa26) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSA26` reader - A/D Conversion Channels Select
pub type Ansa26R = crate::BitReader<Ansa26>;
impl Ansa26R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ansa26 {
        match self.bits {
            false => Ansa26::_0,
            true => Ansa26::_1,
        }
    }
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansa26::_0
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansa26::_1
    }
}
///Field `ANSA26` writer - A/D Conversion Channels Select
pub type Ansa26W<'a, REG> = crate::BitWriter<'a, REG, Ansa26>;
impl<'a, REG> Ansa26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa26::_0)
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa26::_1)
    }
}
/**A/D Conversion Channels Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansa27 {
    ///0: Do not select associated input channel.
    _0 = 0,
    ///1: Select associated input channel.
    _1 = 1,
}
impl From<Ansa27> for bool {
    #[inline(always)]
    fn from(variant: Ansa27) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSA27` reader - A/D Conversion Channels Select
pub type Ansa27R = crate::BitReader<Ansa27>;
impl Ansa27R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ansa27 {
        match self.bits {
            false => Ansa27::_0,
            true => Ansa27::_1,
        }
    }
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansa27::_0
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansa27::_1
    }
}
///Field `ANSA27` writer - A/D Conversion Channels Select
pub type Ansa27W<'a, REG> = crate::BitWriter<'a, REG, Ansa27>;
impl<'a, REG> Ansa27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa27::_0)
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa27::_1)
    }
}
/**A/D Conversion Channels Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansa28 {
    ///0: Do not select associated input channel.
    _0 = 0,
    ///1: Select associated input channel.
    _1 = 1,
}
impl From<Ansa28> for bool {
    #[inline(always)]
    fn from(variant: Ansa28) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSA28` reader - A/D Conversion Channels Select
pub type Ansa28R = crate::BitReader<Ansa28>;
impl Ansa28R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ansa28 {
        match self.bits {
            false => Ansa28::_0,
            true => Ansa28::_1,
        }
    }
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansa28::_0
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansa28::_1
    }
}
///Field `ANSA28` writer - A/D Conversion Channels Select
pub type Ansa28W<'a, REG> = crate::BitWriter<'a, REG, Ansa28>;
impl<'a, REG> Ansa28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa28::_0)
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa28::_1)
    }
}
impl R {
    ///Bit 0 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansa16(&self) -> Ansa16R {
        Ansa16R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansa17(&self) -> Ansa17R {
        Ansa17R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansa18(&self) -> Ansa18R {
        Ansa18R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansa19(&self) -> Ansa19R {
        Ansa19R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansa20(&self) -> Ansa20R {
        Ansa20R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansa21(&self) -> Ansa21R {
        Ansa21R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansa22(&self) -> Ansa22R {
        Ansa22R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansa23(&self) -> Ansa23R {
        Ansa23R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansa24(&self) -> Ansa24R {
        Ansa24R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansa25(&self) -> Ansa25R {
        Ansa25R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansa26(&self) -> Ansa26R {
        Ansa26R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansa27(&self) -> Ansa27R {
        Ansa27R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansa28(&self) -> Ansa28R {
        Ansa28R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADANSA1")
            .field("ansa16", &self.ansa16())
            .field("ansa17", &self.ansa17())
            .field("ansa18", &self.ansa18())
            .field("ansa19", &self.ansa19())
            .field("ansa20", &self.ansa20())
            .field("ansa21", &self.ansa21())
            .field("ansa22", &self.ansa22())
            .field("ansa23", &self.ansa23())
            .field("ansa24", &self.ansa24())
            .field("ansa25", &self.ansa25())
            .field("ansa26", &self.ansa26())
            .field("ansa27", &self.ansa27())
            .field("ansa28", &self.ansa28())
            .finish()
    }
}
impl W {
    ///Bit 0 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansa16(&mut self) -> Ansa16W<Adansa1Spec> {
        Ansa16W::new(self, 0)
    }
    ///Bit 1 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansa17(&mut self) -> Ansa17W<Adansa1Spec> {
        Ansa17W::new(self, 1)
    }
    ///Bit 2 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansa18(&mut self) -> Ansa18W<Adansa1Spec> {
        Ansa18W::new(self, 2)
    }
    ///Bit 3 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansa19(&mut self) -> Ansa19W<Adansa1Spec> {
        Ansa19W::new(self, 3)
    }
    ///Bit 4 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansa20(&mut self) -> Ansa20W<Adansa1Spec> {
        Ansa20W::new(self, 4)
    }
    ///Bit 5 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansa21(&mut self) -> Ansa21W<Adansa1Spec> {
        Ansa21W::new(self, 5)
    }
    ///Bit 6 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansa22(&mut self) -> Ansa22W<Adansa1Spec> {
        Ansa22W::new(self, 6)
    }
    ///Bit 7 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansa23(&mut self) -> Ansa23W<Adansa1Spec> {
        Ansa23W::new(self, 7)
    }
    ///Bit 8 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansa24(&mut self) -> Ansa24W<Adansa1Spec> {
        Ansa24W::new(self, 8)
    }
    ///Bit 9 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansa25(&mut self) -> Ansa25W<Adansa1Spec> {
        Ansa25W::new(self, 9)
    }
    ///Bit 10 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansa26(&mut self) -> Ansa26W<Adansa1Spec> {
        Ansa26W::new(self, 10)
    }
    ///Bit 11 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansa27(&mut self) -> Ansa27W<Adansa1Spec> {
        Ansa27W::new(self, 11)
    }
    ///Bit 12 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansa28(&mut self) -> Ansa28W<Adansa1Spec> {
        Ansa28W::new(self, 12)
    }
}
/**A/D Channel Select Register A1

You can [`read`](crate::Reg::read) this register and get [`adansa1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adansa1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Adansa1Spec;
impl crate::RegisterSpec for Adansa1Spec {
    type Ux = u16;
}
///`read()` method returns [`adansa1::R`](R) reader structure
impl crate::Readable for Adansa1Spec {}
///`write(|w| ..)` method takes [`adansa1::W`](W) writer structure
impl crate::Writable for Adansa1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADANSA1 to value 0
impl crate::Resettable for Adansa1Spec {}
