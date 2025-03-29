///Register `ADCMPANSR0` reader
pub type R = crate::R<Adcmpansr0Spec>;
///Register `ADCMPANSR0` writer
pub type W = crate::W<Adcmpansr0Spec>;
/**Compare Window A Channel Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpcha00 {
    ///0: Disable compare function for associated input channel
    _0 = 0,
    ///1: Enable compare function for associated input channel
    _1 = 1,
}
impl From<Cmpcha00> for bool {
    #[inline(always)]
    fn from(variant: Cmpcha00) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPCHA00` reader - Compare Window A Channel Select
pub type Cmpcha00R = crate::BitReader<Cmpcha00>;
impl Cmpcha00R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmpcha00 {
        match self.bits {
            false => Cmpcha00::_0,
            true => Cmpcha00::_1,
        }
    }
    ///Disable compare function for associated input channel
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpcha00::_0
    }
    ///Enable compare function for associated input channel
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpcha00::_1
    }
}
///Field `CMPCHA00` writer - Compare Window A Channel Select
pub type Cmpcha00W<'a, REG> = crate::BitWriter<'a, REG, Cmpcha00>;
impl<'a, REG> Cmpcha00W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable compare function for associated input channel
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha00::_0)
    }
    ///Enable compare function for associated input channel
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha00::_1)
    }
}
/**Compare Window A Channel Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpcha01 {
    ///0: Disable compare function for associated input channel
    _0 = 0,
    ///1: Enable compare function for associated input channel
    _1 = 1,
}
impl From<Cmpcha01> for bool {
    #[inline(always)]
    fn from(variant: Cmpcha01) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPCHA01` reader - Compare Window A Channel Select
pub type Cmpcha01R = crate::BitReader<Cmpcha01>;
impl Cmpcha01R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmpcha01 {
        match self.bits {
            false => Cmpcha01::_0,
            true => Cmpcha01::_1,
        }
    }
    ///Disable compare function for associated input channel
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpcha01::_0
    }
    ///Enable compare function for associated input channel
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpcha01::_1
    }
}
///Field `CMPCHA01` writer - Compare Window A Channel Select
pub type Cmpcha01W<'a, REG> = crate::BitWriter<'a, REG, Cmpcha01>;
impl<'a, REG> Cmpcha01W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable compare function for associated input channel
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha01::_0)
    }
    ///Enable compare function for associated input channel
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha01::_1)
    }
}
/**Compare Window A Channel Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpcha02 {
    ///0: Disable compare function for associated input channel
    _0 = 0,
    ///1: Enable compare function for associated input channel
    _1 = 1,
}
impl From<Cmpcha02> for bool {
    #[inline(always)]
    fn from(variant: Cmpcha02) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPCHA02` reader - Compare Window A Channel Select
pub type Cmpcha02R = crate::BitReader<Cmpcha02>;
impl Cmpcha02R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmpcha02 {
        match self.bits {
            false => Cmpcha02::_0,
            true => Cmpcha02::_1,
        }
    }
    ///Disable compare function for associated input channel
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpcha02::_0
    }
    ///Enable compare function for associated input channel
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpcha02::_1
    }
}
///Field `CMPCHA02` writer - Compare Window A Channel Select
pub type Cmpcha02W<'a, REG> = crate::BitWriter<'a, REG, Cmpcha02>;
impl<'a, REG> Cmpcha02W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable compare function for associated input channel
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha02::_0)
    }
    ///Enable compare function for associated input channel
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha02::_1)
    }
}
/**Compare Window A Channel Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpcha03 {
    ///0: Disable compare function for associated input channel
    _0 = 0,
    ///1: Enable compare function for associated input channel
    _1 = 1,
}
impl From<Cmpcha03> for bool {
    #[inline(always)]
    fn from(variant: Cmpcha03) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPCHA03` reader - Compare Window A Channel Select
pub type Cmpcha03R = crate::BitReader<Cmpcha03>;
impl Cmpcha03R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmpcha03 {
        match self.bits {
            false => Cmpcha03::_0,
            true => Cmpcha03::_1,
        }
    }
    ///Disable compare function for associated input channel
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpcha03::_0
    }
    ///Enable compare function for associated input channel
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpcha03::_1
    }
}
///Field `CMPCHA03` writer - Compare Window A Channel Select
pub type Cmpcha03W<'a, REG> = crate::BitWriter<'a, REG, Cmpcha03>;
impl<'a, REG> Cmpcha03W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable compare function for associated input channel
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha03::_0)
    }
    ///Enable compare function for associated input channel
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha03::_1)
    }
}
/**Compare Window A Channel Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpcha04 {
    ///0: Disable compare function for associated input channel
    _0 = 0,
    ///1: Enable compare function for associated input channel
    _1 = 1,
}
impl From<Cmpcha04> for bool {
    #[inline(always)]
    fn from(variant: Cmpcha04) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPCHA04` reader - Compare Window A Channel Select
pub type Cmpcha04R = crate::BitReader<Cmpcha04>;
impl Cmpcha04R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmpcha04 {
        match self.bits {
            false => Cmpcha04::_0,
            true => Cmpcha04::_1,
        }
    }
    ///Disable compare function for associated input channel
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpcha04::_0
    }
    ///Enable compare function for associated input channel
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpcha04::_1
    }
}
///Field `CMPCHA04` writer - Compare Window A Channel Select
pub type Cmpcha04W<'a, REG> = crate::BitWriter<'a, REG, Cmpcha04>;
impl<'a, REG> Cmpcha04W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable compare function for associated input channel
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha04::_0)
    }
    ///Enable compare function for associated input channel
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha04::_1)
    }
}
/**Compare Window A Channel Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpcha05 {
    ///0: Disable compare function for associated input channel
    _0 = 0,
    ///1: Enable compare function for associated input channel
    _1 = 1,
}
impl From<Cmpcha05> for bool {
    #[inline(always)]
    fn from(variant: Cmpcha05) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPCHA05` reader - Compare Window A Channel Select
pub type Cmpcha05R = crate::BitReader<Cmpcha05>;
impl Cmpcha05R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmpcha05 {
        match self.bits {
            false => Cmpcha05::_0,
            true => Cmpcha05::_1,
        }
    }
    ///Disable compare function for associated input channel
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpcha05::_0
    }
    ///Enable compare function for associated input channel
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpcha05::_1
    }
}
///Field `CMPCHA05` writer - Compare Window A Channel Select
pub type Cmpcha05W<'a, REG> = crate::BitWriter<'a, REG, Cmpcha05>;
impl<'a, REG> Cmpcha05W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable compare function for associated input channel
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha05::_0)
    }
    ///Enable compare function for associated input channel
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha05::_1)
    }
}
/**Compare Window A Channel Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpcha06 {
    ///0: Disable compare function for associated input channel
    _0 = 0,
    ///1: Enable compare function for associated input channel
    _1 = 1,
}
impl From<Cmpcha06> for bool {
    #[inline(always)]
    fn from(variant: Cmpcha06) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPCHA06` reader - Compare Window A Channel Select
pub type Cmpcha06R = crate::BitReader<Cmpcha06>;
impl Cmpcha06R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmpcha06 {
        match self.bits {
            false => Cmpcha06::_0,
            true => Cmpcha06::_1,
        }
    }
    ///Disable compare function for associated input channel
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpcha06::_0
    }
    ///Enable compare function for associated input channel
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpcha06::_1
    }
}
///Field `CMPCHA06` writer - Compare Window A Channel Select
pub type Cmpcha06W<'a, REG> = crate::BitWriter<'a, REG, Cmpcha06>;
impl<'a, REG> Cmpcha06W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable compare function for associated input channel
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha06::_0)
    }
    ///Enable compare function for associated input channel
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha06::_1)
    }
}
/**Compare Window A Channel Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpcha07 {
    ///0: Disable compare function for associated input channel
    _0 = 0,
    ///1: Enable compare function for associated input channel
    _1 = 1,
}
impl From<Cmpcha07> for bool {
    #[inline(always)]
    fn from(variant: Cmpcha07) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPCHA07` reader - Compare Window A Channel Select
pub type Cmpcha07R = crate::BitReader<Cmpcha07>;
impl Cmpcha07R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmpcha07 {
        match self.bits {
            false => Cmpcha07::_0,
            true => Cmpcha07::_1,
        }
    }
    ///Disable compare function for associated input channel
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpcha07::_0
    }
    ///Enable compare function for associated input channel
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpcha07::_1
    }
}
///Field `CMPCHA07` writer - Compare Window A Channel Select
pub type Cmpcha07W<'a, REG> = crate::BitWriter<'a, REG, Cmpcha07>;
impl<'a, REG> Cmpcha07W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable compare function for associated input channel
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha07::_0)
    }
    ///Enable compare function for associated input channel
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha07::_1)
    }
}
/**Compare Window A Channel Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpcha08 {
    ///0: Disable compare function for associated input channel
    _0 = 0,
    ///1: Enable compare function for associated input channel
    _1 = 1,
}
impl From<Cmpcha08> for bool {
    #[inline(always)]
    fn from(variant: Cmpcha08) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPCHA08` reader - Compare Window A Channel Select
pub type Cmpcha08R = crate::BitReader<Cmpcha08>;
impl Cmpcha08R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmpcha08 {
        match self.bits {
            false => Cmpcha08::_0,
            true => Cmpcha08::_1,
        }
    }
    ///Disable compare function for associated input channel
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpcha08::_0
    }
    ///Enable compare function for associated input channel
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpcha08::_1
    }
}
///Field `CMPCHA08` writer - Compare Window A Channel Select
pub type Cmpcha08W<'a, REG> = crate::BitWriter<'a, REG, Cmpcha08>;
impl<'a, REG> Cmpcha08W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable compare function for associated input channel
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha08::_0)
    }
    ///Enable compare function for associated input channel
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha08::_1)
    }
}
/**Compare Window A Channel Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpcha09 {
    ///0: Disable compare function for associated input channel
    _0 = 0,
    ///1: Enable compare function for associated input channel
    _1 = 1,
}
impl From<Cmpcha09> for bool {
    #[inline(always)]
    fn from(variant: Cmpcha09) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPCHA09` reader - Compare Window A Channel Select
pub type Cmpcha09R = crate::BitReader<Cmpcha09>;
impl Cmpcha09R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmpcha09 {
        match self.bits {
            false => Cmpcha09::_0,
            true => Cmpcha09::_1,
        }
    }
    ///Disable compare function for associated input channel
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpcha09::_0
    }
    ///Enable compare function for associated input channel
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpcha09::_1
    }
}
///Field `CMPCHA09` writer - Compare Window A Channel Select
pub type Cmpcha09W<'a, REG> = crate::BitWriter<'a, REG, Cmpcha09>;
impl<'a, REG> Cmpcha09W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable compare function for associated input channel
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha09::_0)
    }
    ///Enable compare function for associated input channel
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha09::_1)
    }
}
/**Compare Window A Channel Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpcha10 {
    ///0: Disable compare function for associated input channel
    _0 = 0,
    ///1: Enable compare function for associated input channel
    _1 = 1,
}
impl From<Cmpcha10> for bool {
    #[inline(always)]
    fn from(variant: Cmpcha10) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPCHA10` reader - Compare Window A Channel Select
pub type Cmpcha10R = crate::BitReader<Cmpcha10>;
impl Cmpcha10R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmpcha10 {
        match self.bits {
            false => Cmpcha10::_0,
            true => Cmpcha10::_1,
        }
    }
    ///Disable compare function for associated input channel
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpcha10::_0
    }
    ///Enable compare function for associated input channel
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpcha10::_1
    }
}
///Field `CMPCHA10` writer - Compare Window A Channel Select
pub type Cmpcha10W<'a, REG> = crate::BitWriter<'a, REG, Cmpcha10>;
impl<'a, REG> Cmpcha10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable compare function for associated input channel
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha10::_0)
    }
    ///Enable compare function for associated input channel
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha10::_1)
    }
}
/**Compare Window A Channel Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpcha12 {
    ///0: Disable compare function for associated input channel
    _0 = 0,
    ///1: Enable compare function for associated input channel
    _1 = 1,
}
impl From<Cmpcha12> for bool {
    #[inline(always)]
    fn from(variant: Cmpcha12) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPCHA12` reader - Compare Window A Channel Select
pub type Cmpcha12R = crate::BitReader<Cmpcha12>;
impl Cmpcha12R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmpcha12 {
        match self.bits {
            false => Cmpcha12::_0,
            true => Cmpcha12::_1,
        }
    }
    ///Disable compare function for associated input channel
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpcha12::_0
    }
    ///Enable compare function for associated input channel
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpcha12::_1
    }
}
///Field `CMPCHA12` writer - Compare Window A Channel Select
pub type Cmpcha12W<'a, REG> = crate::BitWriter<'a, REG, Cmpcha12>;
impl<'a, REG> Cmpcha12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable compare function for associated input channel
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha12::_0)
    }
    ///Enable compare function for associated input channel
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha12::_1)
    }
}
/**Compare Window A Channel Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpcha13 {
    ///0: Disable compare function for associated input channel
    _0 = 0,
    ///1: Enable compare function for associated input channel
    _1 = 1,
}
impl From<Cmpcha13> for bool {
    #[inline(always)]
    fn from(variant: Cmpcha13) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPCHA13` reader - Compare Window A Channel Select
pub type Cmpcha13R = crate::BitReader<Cmpcha13>;
impl Cmpcha13R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmpcha13 {
        match self.bits {
            false => Cmpcha13::_0,
            true => Cmpcha13::_1,
        }
    }
    ///Disable compare function for associated input channel
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpcha13::_0
    }
    ///Enable compare function for associated input channel
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpcha13::_1
    }
}
///Field `CMPCHA13` writer - Compare Window A Channel Select
pub type Cmpcha13W<'a, REG> = crate::BitWriter<'a, REG, Cmpcha13>;
impl<'a, REG> Cmpcha13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable compare function for associated input channel
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha13::_0)
    }
    ///Enable compare function for associated input channel
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpcha13::_1)
    }
}
impl R {
    ///Bit 0 - Compare Window A Channel Select
    #[inline(always)]
    pub fn cmpcha00(&self) -> Cmpcha00R {
        Cmpcha00R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Compare Window A Channel Select
    #[inline(always)]
    pub fn cmpcha01(&self) -> Cmpcha01R {
        Cmpcha01R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Compare Window A Channel Select
    #[inline(always)]
    pub fn cmpcha02(&self) -> Cmpcha02R {
        Cmpcha02R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Compare Window A Channel Select
    #[inline(always)]
    pub fn cmpcha03(&self) -> Cmpcha03R {
        Cmpcha03R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Compare Window A Channel Select
    #[inline(always)]
    pub fn cmpcha04(&self) -> Cmpcha04R {
        Cmpcha04R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Compare Window A Channel Select
    #[inline(always)]
    pub fn cmpcha05(&self) -> Cmpcha05R {
        Cmpcha05R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Compare Window A Channel Select
    #[inline(always)]
    pub fn cmpcha06(&self) -> Cmpcha06R {
        Cmpcha06R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Compare Window A Channel Select
    #[inline(always)]
    pub fn cmpcha07(&self) -> Cmpcha07R {
        Cmpcha07R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Compare Window A Channel Select
    #[inline(always)]
    pub fn cmpcha08(&self) -> Cmpcha08R {
        Cmpcha08R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Compare Window A Channel Select
    #[inline(always)]
    pub fn cmpcha09(&self) -> Cmpcha09R {
        Cmpcha09R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Compare Window A Channel Select
    #[inline(always)]
    pub fn cmpcha10(&self) -> Cmpcha10R {
        Cmpcha10R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 12 - Compare Window A Channel Select
    #[inline(always)]
    pub fn cmpcha12(&self) -> Cmpcha12R {
        Cmpcha12R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Compare Window A Channel Select
    #[inline(always)]
    pub fn cmpcha13(&self) -> Cmpcha13R {
        Cmpcha13R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCMPANSR0")
            .field("cmpcha00", &self.cmpcha00())
            .field("cmpcha01", &self.cmpcha01())
            .field("cmpcha02", &self.cmpcha02())
            .field("cmpcha03", &self.cmpcha03())
            .field("cmpcha04", &self.cmpcha04())
            .field("cmpcha05", &self.cmpcha05())
            .field("cmpcha06", &self.cmpcha06())
            .field("cmpcha07", &self.cmpcha07())
            .field("cmpcha08", &self.cmpcha08())
            .field("cmpcha09", &self.cmpcha09())
            .field("cmpcha10", &self.cmpcha10())
            .field("cmpcha12", &self.cmpcha12())
            .field("cmpcha13", &self.cmpcha13())
            .finish()
    }
}
impl W {
    ///Bit 0 - Compare Window A Channel Select
    #[inline(always)]
    pub fn cmpcha00(&mut self) -> Cmpcha00W<Adcmpansr0Spec> {
        Cmpcha00W::new(self, 0)
    }
    ///Bit 1 - Compare Window A Channel Select
    #[inline(always)]
    pub fn cmpcha01(&mut self) -> Cmpcha01W<Adcmpansr0Spec> {
        Cmpcha01W::new(self, 1)
    }
    ///Bit 2 - Compare Window A Channel Select
    #[inline(always)]
    pub fn cmpcha02(&mut self) -> Cmpcha02W<Adcmpansr0Spec> {
        Cmpcha02W::new(self, 2)
    }
    ///Bit 3 - Compare Window A Channel Select
    #[inline(always)]
    pub fn cmpcha03(&mut self) -> Cmpcha03W<Adcmpansr0Spec> {
        Cmpcha03W::new(self, 3)
    }
    ///Bit 4 - Compare Window A Channel Select
    #[inline(always)]
    pub fn cmpcha04(&mut self) -> Cmpcha04W<Adcmpansr0Spec> {
        Cmpcha04W::new(self, 4)
    }
    ///Bit 5 - Compare Window A Channel Select
    #[inline(always)]
    pub fn cmpcha05(&mut self) -> Cmpcha05W<Adcmpansr0Spec> {
        Cmpcha05W::new(self, 5)
    }
    ///Bit 6 - Compare Window A Channel Select
    #[inline(always)]
    pub fn cmpcha06(&mut self) -> Cmpcha06W<Adcmpansr0Spec> {
        Cmpcha06W::new(self, 6)
    }
    ///Bit 7 - Compare Window A Channel Select
    #[inline(always)]
    pub fn cmpcha07(&mut self) -> Cmpcha07W<Adcmpansr0Spec> {
        Cmpcha07W::new(self, 7)
    }
    ///Bit 8 - Compare Window A Channel Select
    #[inline(always)]
    pub fn cmpcha08(&mut self) -> Cmpcha08W<Adcmpansr0Spec> {
        Cmpcha08W::new(self, 8)
    }
    ///Bit 9 - Compare Window A Channel Select
    #[inline(always)]
    pub fn cmpcha09(&mut self) -> Cmpcha09W<Adcmpansr0Spec> {
        Cmpcha09W::new(self, 9)
    }
    ///Bit 10 - Compare Window A Channel Select
    #[inline(always)]
    pub fn cmpcha10(&mut self) -> Cmpcha10W<Adcmpansr0Spec> {
        Cmpcha10W::new(self, 10)
    }
    ///Bit 12 - Compare Window A Channel Select
    #[inline(always)]
    pub fn cmpcha12(&mut self) -> Cmpcha12W<Adcmpansr0Spec> {
        Cmpcha12W::new(self, 12)
    }
    ///Bit 13 - Compare Window A Channel Select
    #[inline(always)]
    pub fn cmpcha13(&mut self) -> Cmpcha13W<Adcmpansr0Spec> {
        Cmpcha13W::new(self, 13)
    }
}
/**A/D Compare Function Window A Channel Select Register 0

You can [`read`](crate::Reg::read) this register and get [`adcmpansr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpansr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Adcmpansr0Spec;
impl crate::RegisterSpec for Adcmpansr0Spec {
    type Ux = u16;
}
///`read()` method returns [`adcmpansr0::R`](R) reader structure
impl crate::Readable for Adcmpansr0Spec {}
///`write(|w| ..)` method takes [`adcmpansr0::W`](W) writer structure
impl crate::Writable for Adcmpansr0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADCMPANSR0 to value 0
impl crate::Resettable for Adcmpansr0Spec {}
