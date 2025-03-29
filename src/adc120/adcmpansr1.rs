///Register `ADCMPANSR1` reader
pub type R = crate::R<Adcmpansr1Spec>;
///Register `ADCMPANSR1` writer
pub type W = crate::W<Adcmpansr1Spec>;
/**Compare Window A Channel Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpcha16 {
    ///0: Disable compare function for associated input channel
    _0 = 0,
    ///1: Enable compare function for associated input channel
    _1 = 1,
}
impl From<Cmpcha16> for bool {
    #[inline(always)]
    fn from(variant: Cmpcha16) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPCHA16` reader - Compare Window A Channel Select
pub type Cmpcha16R = crate::BitReader<Cmpcha16>;
impl Cmpcha16R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmpcha16 {
        match self.bits {
            false => Cmpcha16::_0,
            true => Cmpcha16::_1,
        }
    }
    ///Disable compare function for associated input channel
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpcha16::_0
    }
    ///Enable compare function for associated input channel
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpcha16::_1
    }
}
///Field `CMPCHA16` writer - Compare Window A Channel Select
pub type Cmpcha16W<'a, REG> = crate::BitWriter<'a, REG, Cmpcha16>;
impl<'a, REG> Cmpcha16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable compare function for associated input channel
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha16::_0)
    }
    ///Enable compare function for associated input channel
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha16::_1)
    }
}
/**Compare Window A Channel Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpcha17 {
    ///0: Disable compare function for associated input channel
    _0 = 0,
    ///1: Enable compare function for associated input channel
    _1 = 1,
}
impl From<Cmpcha17> for bool {
    #[inline(always)]
    fn from(variant: Cmpcha17) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPCHA17` reader - Compare Window A Channel Select
pub type Cmpcha17R = crate::BitReader<Cmpcha17>;
impl Cmpcha17R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmpcha17 {
        match self.bits {
            false => Cmpcha17::_0,
            true => Cmpcha17::_1,
        }
    }
    ///Disable compare function for associated input channel
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpcha17::_0
    }
    ///Enable compare function for associated input channel
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpcha17::_1
    }
}
///Field `CMPCHA17` writer - Compare Window A Channel Select
pub type Cmpcha17W<'a, REG> = crate::BitWriter<'a, REG, Cmpcha17>;
impl<'a, REG> Cmpcha17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable compare function for associated input channel
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha17::_0)
    }
    ///Enable compare function for associated input channel
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha17::_1)
    }
}
/**Compare Window A Channel Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpcha18 {
    ///0: Disable compare function for associated input channel
    _0 = 0,
    ///1: Enable compare function for associated input channel
    _1 = 1,
}
impl From<Cmpcha18> for bool {
    #[inline(always)]
    fn from(variant: Cmpcha18) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPCHA18` reader - Compare Window A Channel Select
pub type Cmpcha18R = crate::BitReader<Cmpcha18>;
impl Cmpcha18R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmpcha18 {
        match self.bits {
            false => Cmpcha18::_0,
            true => Cmpcha18::_1,
        }
    }
    ///Disable compare function for associated input channel
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpcha18::_0
    }
    ///Enable compare function for associated input channel
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpcha18::_1
    }
}
///Field `CMPCHA18` writer - Compare Window A Channel Select
pub type Cmpcha18W<'a, REG> = crate::BitWriter<'a, REG, Cmpcha18>;
impl<'a, REG> Cmpcha18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable compare function for associated input channel
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha18::_0)
    }
    ///Enable compare function for associated input channel
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha18::_1)
    }
}
/**Compare Window A Channel Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpcha19 {
    ///0: Disable compare function for associated input channel
    _0 = 0,
    ///1: Enable compare function for associated input channel
    _1 = 1,
}
impl From<Cmpcha19> for bool {
    #[inline(always)]
    fn from(variant: Cmpcha19) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPCHA19` reader - Compare Window A Channel Select
pub type Cmpcha19R = crate::BitReader<Cmpcha19>;
impl Cmpcha19R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmpcha19 {
        match self.bits {
            false => Cmpcha19::_0,
            true => Cmpcha19::_1,
        }
    }
    ///Disable compare function for associated input channel
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpcha19::_0
    }
    ///Enable compare function for associated input channel
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpcha19::_1
    }
}
///Field `CMPCHA19` writer - Compare Window A Channel Select
pub type Cmpcha19W<'a, REG> = crate::BitWriter<'a, REG, Cmpcha19>;
impl<'a, REG> Cmpcha19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable compare function for associated input channel
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha19::_0)
    }
    ///Enable compare function for associated input channel
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha19::_1)
    }
}
/**Compare Window A Channel Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpcha20 {
    ///0: Disable compare function for associated input channel
    _0 = 0,
    ///1: Enable compare function for associated input channel
    _1 = 1,
}
impl From<Cmpcha20> for bool {
    #[inline(always)]
    fn from(variant: Cmpcha20) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPCHA20` reader - Compare Window A Channel Select
pub type Cmpcha20R = crate::BitReader<Cmpcha20>;
impl Cmpcha20R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmpcha20 {
        match self.bits {
            false => Cmpcha20::_0,
            true => Cmpcha20::_1,
        }
    }
    ///Disable compare function for associated input channel
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpcha20::_0
    }
    ///Enable compare function for associated input channel
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpcha20::_1
    }
}
///Field `CMPCHA20` writer - Compare Window A Channel Select
pub type Cmpcha20W<'a, REG> = crate::BitWriter<'a, REG, Cmpcha20>;
impl<'a, REG> Cmpcha20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable compare function for associated input channel
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha20::_0)
    }
    ///Enable compare function for associated input channel
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha20::_1)
    }
}
/**Compare Window A Channel Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpcha21 {
    ///0: Disable compare function for associated input channel
    _0 = 0,
    ///1: Enable compare function for associated input channel
    _1 = 1,
}
impl From<Cmpcha21> for bool {
    #[inline(always)]
    fn from(variant: Cmpcha21) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPCHA21` reader - Compare Window A Channel Select
pub type Cmpcha21R = crate::BitReader<Cmpcha21>;
impl Cmpcha21R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmpcha21 {
        match self.bits {
            false => Cmpcha21::_0,
            true => Cmpcha21::_1,
        }
    }
    ///Disable compare function for associated input channel
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpcha21::_0
    }
    ///Enable compare function for associated input channel
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpcha21::_1
    }
}
///Field `CMPCHA21` writer - Compare Window A Channel Select
pub type Cmpcha21W<'a, REG> = crate::BitWriter<'a, REG, Cmpcha21>;
impl<'a, REG> Cmpcha21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable compare function for associated input channel
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha21::_0)
    }
    ///Enable compare function for associated input channel
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha21::_1)
    }
}
/**Compare Window A Channel Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpcha22 {
    ///0: Disable compare function for associated input channel
    _0 = 0,
    ///1: Enable compare function for associated input channel
    _1 = 1,
}
impl From<Cmpcha22> for bool {
    #[inline(always)]
    fn from(variant: Cmpcha22) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPCHA22` reader - Compare Window A Channel Select
pub type Cmpcha22R = crate::BitReader<Cmpcha22>;
impl Cmpcha22R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmpcha22 {
        match self.bits {
            false => Cmpcha22::_0,
            true => Cmpcha22::_1,
        }
    }
    ///Disable compare function for associated input channel
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpcha22::_0
    }
    ///Enable compare function for associated input channel
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpcha22::_1
    }
}
///Field `CMPCHA22` writer - Compare Window A Channel Select
pub type Cmpcha22W<'a, REG> = crate::BitWriter<'a, REG, Cmpcha22>;
impl<'a, REG> Cmpcha22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable compare function for associated input channel
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha22::_0)
    }
    ///Enable compare function for associated input channel
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha22::_1)
    }
}
/**Compare Window A Channel Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpcha23 {
    ///0: Disable compare function for associated input channel
    _0 = 0,
    ///1: Enable compare function for associated input channel
    _1 = 1,
}
impl From<Cmpcha23> for bool {
    #[inline(always)]
    fn from(variant: Cmpcha23) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPCHA23` reader - Compare Window A Channel Select
pub type Cmpcha23R = crate::BitReader<Cmpcha23>;
impl Cmpcha23R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmpcha23 {
        match self.bits {
            false => Cmpcha23::_0,
            true => Cmpcha23::_1,
        }
    }
    ///Disable compare function for associated input channel
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpcha23::_0
    }
    ///Enable compare function for associated input channel
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpcha23::_1
    }
}
///Field `CMPCHA23` writer - Compare Window A Channel Select
pub type Cmpcha23W<'a, REG> = crate::BitWriter<'a, REG, Cmpcha23>;
impl<'a, REG> Cmpcha23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable compare function for associated input channel
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha23::_0)
    }
    ///Enable compare function for associated input channel
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha23::_1)
    }
}
/**Compare Window A Channel Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpcha24 {
    ///0: Disable compare function for associated input channel
    _0 = 0,
    ///1: Enable compare function for associated input channel
    _1 = 1,
}
impl From<Cmpcha24> for bool {
    #[inline(always)]
    fn from(variant: Cmpcha24) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPCHA24` reader - Compare Window A Channel Select
pub type Cmpcha24R = crate::BitReader<Cmpcha24>;
impl Cmpcha24R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmpcha24 {
        match self.bits {
            false => Cmpcha24::_0,
            true => Cmpcha24::_1,
        }
    }
    ///Disable compare function for associated input channel
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpcha24::_0
    }
    ///Enable compare function for associated input channel
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpcha24::_1
    }
}
///Field `CMPCHA24` writer - Compare Window A Channel Select
pub type Cmpcha24W<'a, REG> = crate::BitWriter<'a, REG, Cmpcha24>;
impl<'a, REG> Cmpcha24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable compare function for associated input channel
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha24::_0)
    }
    ///Enable compare function for associated input channel
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha24::_1)
    }
}
/**Compare Window A Channel Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpcha25 {
    ///0: Disable compare function for associated input channel
    _0 = 0,
    ///1: Enable compare function for associated input channel
    _1 = 1,
}
impl From<Cmpcha25> for bool {
    #[inline(always)]
    fn from(variant: Cmpcha25) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPCHA25` reader - Compare Window A Channel Select
pub type Cmpcha25R = crate::BitReader<Cmpcha25>;
impl Cmpcha25R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmpcha25 {
        match self.bits {
            false => Cmpcha25::_0,
            true => Cmpcha25::_1,
        }
    }
    ///Disable compare function for associated input channel
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpcha25::_0
    }
    ///Enable compare function for associated input channel
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpcha25::_1
    }
}
///Field `CMPCHA25` writer - Compare Window A Channel Select
pub type Cmpcha25W<'a, REG> = crate::BitWriter<'a, REG, Cmpcha25>;
impl<'a, REG> Cmpcha25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable compare function for associated input channel
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha25::_0)
    }
    ///Enable compare function for associated input channel
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha25::_1)
    }
}
/**Compare Window A Channel Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpcha26 {
    ///0: Disable compare function for associated input channel
    _0 = 0,
    ///1: Enable compare function for associated input channel
    _1 = 1,
}
impl From<Cmpcha26> for bool {
    #[inline(always)]
    fn from(variant: Cmpcha26) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPCHA26` reader - Compare Window A Channel Select
pub type Cmpcha26R = crate::BitReader<Cmpcha26>;
impl Cmpcha26R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmpcha26 {
        match self.bits {
            false => Cmpcha26::_0,
            true => Cmpcha26::_1,
        }
    }
    ///Disable compare function for associated input channel
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpcha26::_0
    }
    ///Enable compare function for associated input channel
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpcha26::_1
    }
}
///Field `CMPCHA26` writer - Compare Window A Channel Select
pub type Cmpcha26W<'a, REG> = crate::BitWriter<'a, REG, Cmpcha26>;
impl<'a, REG> Cmpcha26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable compare function for associated input channel
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha26::_0)
    }
    ///Enable compare function for associated input channel
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha26::_1)
    }
}
/**Compare Window A Channel Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpcha27 {
    ///0: Disable compare function for associated input channel
    _0 = 0,
    ///1: Enable compare function for associated input channel
    _1 = 1,
}
impl From<Cmpcha27> for bool {
    #[inline(always)]
    fn from(variant: Cmpcha27) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPCHA27` reader - Compare Window A Channel Select
pub type Cmpcha27R = crate::BitReader<Cmpcha27>;
impl Cmpcha27R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmpcha27 {
        match self.bits {
            false => Cmpcha27::_0,
            true => Cmpcha27::_1,
        }
    }
    ///Disable compare function for associated input channel
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpcha27::_0
    }
    ///Enable compare function for associated input channel
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpcha27::_1
    }
}
///Field `CMPCHA27` writer - Compare Window A Channel Select
pub type Cmpcha27W<'a, REG> = crate::BitWriter<'a, REG, Cmpcha27>;
impl<'a, REG> Cmpcha27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable compare function for associated input channel
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha27::_0)
    }
    ///Enable compare function for associated input channel
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha27::_1)
    }
}
/**Compare Window A Channel Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpcha28 {
    ///0: Disable compare function for associated input channel
    _0 = 0,
    ///1: Enable compare function for associated input channel
    _1 = 1,
}
impl From<Cmpcha28> for bool {
    #[inline(always)]
    fn from(variant: Cmpcha28) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPCHA28` reader - Compare Window A Channel Select
pub type Cmpcha28R = crate::BitReader<Cmpcha28>;
impl Cmpcha28R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmpcha28 {
        match self.bits {
            false => Cmpcha28::_0,
            true => Cmpcha28::_1,
        }
    }
    ///Disable compare function for associated input channel
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpcha28::_0
    }
    ///Enable compare function for associated input channel
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpcha28::_1
    }
}
///Field `CMPCHA28` writer - Compare Window A Channel Select
pub type Cmpcha28W<'a, REG> = crate::BitWriter<'a, REG, Cmpcha28>;
impl<'a, REG> Cmpcha28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable compare function for associated input channel
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha28::_0)
    }
    ///Enable compare function for associated input channel
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha28::_1)
    }
}
impl R {
    ///Bit 0 - Compare Window A Channel Select
    #[inline(always)]
    pub fn cmpcha16(&self) -> Cmpcha16R {
        Cmpcha16R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Compare Window A Channel Select
    #[inline(always)]
    pub fn cmpcha17(&self) -> Cmpcha17R {
        Cmpcha17R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Compare Window A Channel Select
    #[inline(always)]
    pub fn cmpcha18(&self) -> Cmpcha18R {
        Cmpcha18R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Compare Window A Channel Select
    #[inline(always)]
    pub fn cmpcha19(&self) -> Cmpcha19R {
        Cmpcha19R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Compare Window A Channel Select
    #[inline(always)]
    pub fn cmpcha20(&self) -> Cmpcha20R {
        Cmpcha20R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Compare Window A Channel Select
    #[inline(always)]
    pub fn cmpcha21(&self) -> Cmpcha21R {
        Cmpcha21R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Compare Window A Channel Select
    #[inline(always)]
    pub fn cmpcha22(&self) -> Cmpcha22R {
        Cmpcha22R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Compare Window A Channel Select
    #[inline(always)]
    pub fn cmpcha23(&self) -> Cmpcha23R {
        Cmpcha23R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Compare Window A Channel Select
    #[inline(always)]
    pub fn cmpcha24(&self) -> Cmpcha24R {
        Cmpcha24R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Compare Window A Channel Select
    #[inline(always)]
    pub fn cmpcha25(&self) -> Cmpcha25R {
        Cmpcha25R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Compare Window A Channel Select
    #[inline(always)]
    pub fn cmpcha26(&self) -> Cmpcha26R {
        Cmpcha26R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Compare Window A Channel Select
    #[inline(always)]
    pub fn cmpcha27(&self) -> Cmpcha27R {
        Cmpcha27R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Compare Window A Channel Select
    #[inline(always)]
    pub fn cmpcha28(&self) -> Cmpcha28R {
        Cmpcha28R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCMPANSR1")
            .field("cmpcha16", &self.cmpcha16())
            .field("cmpcha17", &self.cmpcha17())
            .field("cmpcha18", &self.cmpcha18())
            .field("cmpcha19", &self.cmpcha19())
            .field("cmpcha20", &self.cmpcha20())
            .field("cmpcha21", &self.cmpcha21())
            .field("cmpcha22", &self.cmpcha22())
            .field("cmpcha23", &self.cmpcha23())
            .field("cmpcha24", &self.cmpcha24())
            .field("cmpcha25", &self.cmpcha25())
            .field("cmpcha26", &self.cmpcha26())
            .field("cmpcha27", &self.cmpcha27())
            .field("cmpcha28", &self.cmpcha28())
            .finish()
    }
}
impl W {
    ///Bit 0 - Compare Window A Channel Select
    #[inline(always)]
    pub fn cmpcha16(&mut self) -> Cmpcha16W<Adcmpansr1Spec> {
        Cmpcha16W::new(self, 0)
    }
    ///Bit 1 - Compare Window A Channel Select
    #[inline(always)]
    pub fn cmpcha17(&mut self) -> Cmpcha17W<Adcmpansr1Spec> {
        Cmpcha17W::new(self, 1)
    }
    ///Bit 2 - Compare Window A Channel Select
    #[inline(always)]
    pub fn cmpcha18(&mut self) -> Cmpcha18W<Adcmpansr1Spec> {
        Cmpcha18W::new(self, 2)
    }
    ///Bit 3 - Compare Window A Channel Select
    #[inline(always)]
    pub fn cmpcha19(&mut self) -> Cmpcha19W<Adcmpansr1Spec> {
        Cmpcha19W::new(self, 3)
    }
    ///Bit 4 - Compare Window A Channel Select
    #[inline(always)]
    pub fn cmpcha20(&mut self) -> Cmpcha20W<Adcmpansr1Spec> {
        Cmpcha20W::new(self, 4)
    }
    ///Bit 5 - Compare Window A Channel Select
    #[inline(always)]
    pub fn cmpcha21(&mut self) -> Cmpcha21W<Adcmpansr1Spec> {
        Cmpcha21W::new(self, 5)
    }
    ///Bit 6 - Compare Window A Channel Select
    #[inline(always)]
    pub fn cmpcha22(&mut self) -> Cmpcha22W<Adcmpansr1Spec> {
        Cmpcha22W::new(self, 6)
    }
    ///Bit 7 - Compare Window A Channel Select
    #[inline(always)]
    pub fn cmpcha23(&mut self) -> Cmpcha23W<Adcmpansr1Spec> {
        Cmpcha23W::new(self, 7)
    }
    ///Bit 8 - Compare Window A Channel Select
    #[inline(always)]
    pub fn cmpcha24(&mut self) -> Cmpcha24W<Adcmpansr1Spec> {
        Cmpcha24W::new(self, 8)
    }
    ///Bit 9 - Compare Window A Channel Select
    #[inline(always)]
    pub fn cmpcha25(&mut self) -> Cmpcha25W<Adcmpansr1Spec> {
        Cmpcha25W::new(self, 9)
    }
    ///Bit 10 - Compare Window A Channel Select
    #[inline(always)]
    pub fn cmpcha26(&mut self) -> Cmpcha26W<Adcmpansr1Spec> {
        Cmpcha26W::new(self, 10)
    }
    ///Bit 11 - Compare Window A Channel Select
    #[inline(always)]
    pub fn cmpcha27(&mut self) -> Cmpcha27W<Adcmpansr1Spec> {
        Cmpcha27W::new(self, 11)
    }
    ///Bit 12 - Compare Window A Channel Select
    #[inline(always)]
    pub fn cmpcha28(&mut self) -> Cmpcha28W<Adcmpansr1Spec> {
        Cmpcha28W::new(self, 12)
    }
}
/**A/D Compare Function Window A Channel Select Register 1

You can [`read`](crate::Reg::read) this register and get [`adcmpansr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpansr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Adcmpansr1Spec;
impl crate::RegisterSpec for Adcmpansr1Spec {
    type Ux = u16;
}
///`read()` method returns [`adcmpansr1::R`](R) reader structure
impl crate::Readable for Adcmpansr1Spec {}
///`write(|w| ..)` method takes [`adcmpansr1::W`](W) writer structure
impl crate::Writable for Adcmpansr1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADCMPANSR1 to value 0
impl crate::Resettable for Adcmpansr1Spec {}
