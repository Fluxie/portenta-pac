///Register `ADANSB1` reader
pub type R = crate::R<Adansb1Spec>;
///Register `ADANSB1` writer
pub type W = crate::W<Adansb1Spec>;
/**A/D Conversion Channels Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansb16 {
    ///0: Do not select associated input channel.
    _0 = 0,
    ///1: Select associated input channel.
    _1 = 1,
}
impl From<Ansb16> for bool {
    #[inline(always)]
    fn from(variant: Ansb16) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSB16` reader - A/D Conversion Channels Select
pub type Ansb16R = crate::BitReader<Ansb16>;
impl Ansb16R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ansb16 {
        match self.bits {
            false => Ansb16::_0,
            true => Ansb16::_1,
        }
    }
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansb16::_0
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansb16::_1
    }
}
///Field `ANSB16` writer - A/D Conversion Channels Select
pub type Ansb16W<'a, REG> = crate::BitWriter<'a, REG, Ansb16>;
impl<'a, REG> Ansb16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb16::_0)
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb16::_1)
    }
}
/**A/D Conversion Channels Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansb17 {
    ///0: Do not select associated input channel.
    _0 = 0,
    ///1: Select associated input channel.
    _1 = 1,
}
impl From<Ansb17> for bool {
    #[inline(always)]
    fn from(variant: Ansb17) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSB17` reader - A/D Conversion Channels Select
pub type Ansb17R = crate::BitReader<Ansb17>;
impl Ansb17R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ansb17 {
        match self.bits {
            false => Ansb17::_0,
            true => Ansb17::_1,
        }
    }
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansb17::_0
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansb17::_1
    }
}
///Field `ANSB17` writer - A/D Conversion Channels Select
pub type Ansb17W<'a, REG> = crate::BitWriter<'a, REG, Ansb17>;
impl<'a, REG> Ansb17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb17::_0)
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb17::_1)
    }
}
/**A/D Conversion Channels Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansb18 {
    ///0: Do not select associated input channel.
    _0 = 0,
    ///1: Select associated input channel.
    _1 = 1,
}
impl From<Ansb18> for bool {
    #[inline(always)]
    fn from(variant: Ansb18) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSB18` reader - A/D Conversion Channels Select
pub type Ansb18R = crate::BitReader<Ansb18>;
impl Ansb18R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ansb18 {
        match self.bits {
            false => Ansb18::_0,
            true => Ansb18::_1,
        }
    }
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansb18::_0
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansb18::_1
    }
}
///Field `ANSB18` writer - A/D Conversion Channels Select
pub type Ansb18W<'a, REG> = crate::BitWriter<'a, REG, Ansb18>;
impl<'a, REG> Ansb18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb18::_0)
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb18::_1)
    }
}
/**A/D Conversion Channels Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansb19 {
    ///0: Do not select associated input channel.
    _0 = 0,
    ///1: Select associated input channel.
    _1 = 1,
}
impl From<Ansb19> for bool {
    #[inline(always)]
    fn from(variant: Ansb19) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSB19` reader - A/D Conversion Channels Select
pub type Ansb19R = crate::BitReader<Ansb19>;
impl Ansb19R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ansb19 {
        match self.bits {
            false => Ansb19::_0,
            true => Ansb19::_1,
        }
    }
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansb19::_0
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansb19::_1
    }
}
///Field `ANSB19` writer - A/D Conversion Channels Select
pub type Ansb19W<'a, REG> = crate::BitWriter<'a, REG, Ansb19>;
impl<'a, REG> Ansb19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb19::_0)
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb19::_1)
    }
}
/**A/D Conversion Channels Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansb20 {
    ///0: Do not select associated input channel.
    _0 = 0,
    ///1: Select associated input channel.
    _1 = 1,
}
impl From<Ansb20> for bool {
    #[inline(always)]
    fn from(variant: Ansb20) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSB20` reader - A/D Conversion Channels Select
pub type Ansb20R = crate::BitReader<Ansb20>;
impl Ansb20R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ansb20 {
        match self.bits {
            false => Ansb20::_0,
            true => Ansb20::_1,
        }
    }
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansb20::_0
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansb20::_1
    }
}
///Field `ANSB20` writer - A/D Conversion Channels Select
pub type Ansb20W<'a, REG> = crate::BitWriter<'a, REG, Ansb20>;
impl<'a, REG> Ansb20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb20::_0)
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb20::_1)
    }
}
/**A/D Conversion Channels Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansb21 {
    ///0: Do not select associated input channel.
    _0 = 0,
    ///1: Select associated input channel.
    _1 = 1,
}
impl From<Ansb21> for bool {
    #[inline(always)]
    fn from(variant: Ansb21) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSB21` reader - A/D Conversion Channels Select
pub type Ansb21R = crate::BitReader<Ansb21>;
impl Ansb21R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ansb21 {
        match self.bits {
            false => Ansb21::_0,
            true => Ansb21::_1,
        }
    }
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansb21::_0
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansb21::_1
    }
}
///Field `ANSB21` writer - A/D Conversion Channels Select
pub type Ansb21W<'a, REG> = crate::BitWriter<'a, REG, Ansb21>;
impl<'a, REG> Ansb21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb21::_0)
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb21::_1)
    }
}
/**A/D Conversion Channels Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansb22 {
    ///0: Do not select associated input channel.
    _0 = 0,
    ///1: Select associated input channel.
    _1 = 1,
}
impl From<Ansb22> for bool {
    #[inline(always)]
    fn from(variant: Ansb22) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSB22` reader - A/D Conversion Channels Select
pub type Ansb22R = crate::BitReader<Ansb22>;
impl Ansb22R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ansb22 {
        match self.bits {
            false => Ansb22::_0,
            true => Ansb22::_1,
        }
    }
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansb22::_0
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansb22::_1
    }
}
///Field `ANSB22` writer - A/D Conversion Channels Select
pub type Ansb22W<'a, REG> = crate::BitWriter<'a, REG, Ansb22>;
impl<'a, REG> Ansb22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb22::_0)
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb22::_1)
    }
}
/**A/D Conversion Channels Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansb23 {
    ///0: Do not select associated input channel.
    _0 = 0,
    ///1: Select associated input channel.
    _1 = 1,
}
impl From<Ansb23> for bool {
    #[inline(always)]
    fn from(variant: Ansb23) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSB23` reader - A/D Conversion Channels Select
pub type Ansb23R = crate::BitReader<Ansb23>;
impl Ansb23R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ansb23 {
        match self.bits {
            false => Ansb23::_0,
            true => Ansb23::_1,
        }
    }
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansb23::_0
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansb23::_1
    }
}
///Field `ANSB23` writer - A/D Conversion Channels Select
pub type Ansb23W<'a, REG> = crate::BitWriter<'a, REG, Ansb23>;
impl<'a, REG> Ansb23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb23::_0)
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb23::_1)
    }
}
/**A/D Conversion Channels Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansb24 {
    ///0: Do not select associated input channel.
    _0 = 0,
    ///1: Select associated input channel.
    _1 = 1,
}
impl From<Ansb24> for bool {
    #[inline(always)]
    fn from(variant: Ansb24) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSB24` reader - A/D Conversion Channels Select
pub type Ansb24R = crate::BitReader<Ansb24>;
impl Ansb24R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ansb24 {
        match self.bits {
            false => Ansb24::_0,
            true => Ansb24::_1,
        }
    }
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansb24::_0
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansb24::_1
    }
}
///Field `ANSB24` writer - A/D Conversion Channels Select
pub type Ansb24W<'a, REG> = crate::BitWriter<'a, REG, Ansb24>;
impl<'a, REG> Ansb24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb24::_0)
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb24::_1)
    }
}
/**A/D Conversion Channels Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansb25 {
    ///0: Do not select associated input channel.
    _0 = 0,
    ///1: Select associated input channel.
    _1 = 1,
}
impl From<Ansb25> for bool {
    #[inline(always)]
    fn from(variant: Ansb25) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSB25` reader - A/D Conversion Channels Select
pub type Ansb25R = crate::BitReader<Ansb25>;
impl Ansb25R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ansb25 {
        match self.bits {
            false => Ansb25::_0,
            true => Ansb25::_1,
        }
    }
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansb25::_0
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansb25::_1
    }
}
///Field `ANSB25` writer - A/D Conversion Channels Select
pub type Ansb25W<'a, REG> = crate::BitWriter<'a, REG, Ansb25>;
impl<'a, REG> Ansb25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb25::_0)
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb25::_1)
    }
}
/**A/D Conversion Channels Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansb26 {
    ///0: Do not select associated input channel.
    _0 = 0,
    ///1: Select associated input channel.
    _1 = 1,
}
impl From<Ansb26> for bool {
    #[inline(always)]
    fn from(variant: Ansb26) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSB26` reader - A/D Conversion Channels Select
pub type Ansb26R = crate::BitReader<Ansb26>;
impl Ansb26R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ansb26 {
        match self.bits {
            false => Ansb26::_0,
            true => Ansb26::_1,
        }
    }
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansb26::_0
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansb26::_1
    }
}
///Field `ANSB26` writer - A/D Conversion Channels Select
pub type Ansb26W<'a, REG> = crate::BitWriter<'a, REG, Ansb26>;
impl<'a, REG> Ansb26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb26::_0)
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb26::_1)
    }
}
/**A/D Conversion Channels Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansb27 {
    ///0: Do not select associated input channel.
    _0 = 0,
    ///1: Select associated input channel.
    _1 = 1,
}
impl From<Ansb27> for bool {
    #[inline(always)]
    fn from(variant: Ansb27) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSB27` reader - A/D Conversion Channels Select
pub type Ansb27R = crate::BitReader<Ansb27>;
impl Ansb27R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ansb27 {
        match self.bits {
            false => Ansb27::_0,
            true => Ansb27::_1,
        }
    }
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansb27::_0
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansb27::_1
    }
}
///Field `ANSB27` writer - A/D Conversion Channels Select
pub type Ansb27W<'a, REG> = crate::BitWriter<'a, REG, Ansb27>;
impl<'a, REG> Ansb27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb27::_0)
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb27::_1)
    }
}
/**A/D Conversion Channels Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansb28 {
    ///0: Do not select associated input channel.
    _0 = 0,
    ///1: Select associated input channel.
    _1 = 1,
}
impl From<Ansb28> for bool {
    #[inline(always)]
    fn from(variant: Ansb28) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSB28` reader - A/D Conversion Channels Select
pub type Ansb28R = crate::BitReader<Ansb28>;
impl Ansb28R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ansb28 {
        match self.bits {
            false => Ansb28::_0,
            true => Ansb28::_1,
        }
    }
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansb28::_0
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansb28::_1
    }
}
///Field `ANSB28` writer - A/D Conversion Channels Select
pub type Ansb28W<'a, REG> = crate::BitWriter<'a, REG, Ansb28>;
impl<'a, REG> Ansb28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb28::_0)
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb28::_1)
    }
}
impl R {
    ///Bit 0 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansb16(&self) -> Ansb16R {
        Ansb16R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansb17(&self) -> Ansb17R {
        Ansb17R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansb18(&self) -> Ansb18R {
        Ansb18R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansb19(&self) -> Ansb19R {
        Ansb19R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansb20(&self) -> Ansb20R {
        Ansb20R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansb21(&self) -> Ansb21R {
        Ansb21R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansb22(&self) -> Ansb22R {
        Ansb22R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansb23(&self) -> Ansb23R {
        Ansb23R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansb24(&self) -> Ansb24R {
        Ansb24R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansb25(&self) -> Ansb25R {
        Ansb25R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansb26(&self) -> Ansb26R {
        Ansb26R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansb27(&self) -> Ansb27R {
        Ansb27R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansb28(&self) -> Ansb28R {
        Ansb28R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADANSB1")
            .field("ansb16", &self.ansb16())
            .field("ansb17", &self.ansb17())
            .field("ansb18", &self.ansb18())
            .field("ansb19", &self.ansb19())
            .field("ansb20", &self.ansb20())
            .field("ansb21", &self.ansb21())
            .field("ansb22", &self.ansb22())
            .field("ansb23", &self.ansb23())
            .field("ansb24", &self.ansb24())
            .field("ansb25", &self.ansb25())
            .field("ansb26", &self.ansb26())
            .field("ansb27", &self.ansb27())
            .field("ansb28", &self.ansb28())
            .finish()
    }
}
impl W {
    ///Bit 0 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansb16(&mut self) -> Ansb16W<Adansb1Spec> {
        Ansb16W::new(self, 0)
    }
    ///Bit 1 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansb17(&mut self) -> Ansb17W<Adansb1Spec> {
        Ansb17W::new(self, 1)
    }
    ///Bit 2 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansb18(&mut self) -> Ansb18W<Adansb1Spec> {
        Ansb18W::new(self, 2)
    }
    ///Bit 3 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansb19(&mut self) -> Ansb19W<Adansb1Spec> {
        Ansb19W::new(self, 3)
    }
    ///Bit 4 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansb20(&mut self) -> Ansb20W<Adansb1Spec> {
        Ansb20W::new(self, 4)
    }
    ///Bit 5 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansb21(&mut self) -> Ansb21W<Adansb1Spec> {
        Ansb21W::new(self, 5)
    }
    ///Bit 6 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansb22(&mut self) -> Ansb22W<Adansb1Spec> {
        Ansb22W::new(self, 6)
    }
    ///Bit 7 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansb23(&mut self) -> Ansb23W<Adansb1Spec> {
        Ansb23W::new(self, 7)
    }
    ///Bit 8 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansb24(&mut self) -> Ansb24W<Adansb1Spec> {
        Ansb24W::new(self, 8)
    }
    ///Bit 9 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansb25(&mut self) -> Ansb25W<Adansb1Spec> {
        Ansb25W::new(self, 9)
    }
    ///Bit 10 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansb26(&mut self) -> Ansb26W<Adansb1Spec> {
        Ansb26W::new(self, 10)
    }
    ///Bit 11 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansb27(&mut self) -> Ansb27W<Adansb1Spec> {
        Ansb27W::new(self, 11)
    }
    ///Bit 12 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansb28(&mut self) -> Ansb28W<Adansb1Spec> {
        Ansb28W::new(self, 12)
    }
}
/**A/D Channel Select Register B1

You can [`read`](crate::Reg::read) this register and get [`adansb1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adansb1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Adansb1Spec;
impl crate::RegisterSpec for Adansb1Spec {
    type Ux = u16;
}
///`read()` method returns [`adansb1::R`](R) reader structure
impl crate::Readable for Adansb1Spec {}
///`write(|w| ..)` method takes [`adansb1::W`](W) writer structure
impl crate::Writable for Adansb1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADANSB1 to value 0
impl crate::Resettable for Adansb1Spec {}
