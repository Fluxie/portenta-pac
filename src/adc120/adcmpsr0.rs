///Register `ADCMPSR0` reader
pub type R = crate::R<Adcmpsr0Spec>;
///Register `ADCMPSR0` writer
pub type W = crate::W<Adcmpsr0Spec>;
/**Compare Window A Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpstcha00 {
    ///0: Comparison conditions are not met.
    _0 = 0,
    ///1: Comparison conditions are met.
    _1 = 1,
}
impl From<Cmpstcha00> for bool {
    #[inline(always)]
    fn from(variant: Cmpstcha00) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPSTCHA00` reader - Compare Window A Flag
pub type Cmpstcha00R = crate::BitReader<Cmpstcha00>;
impl Cmpstcha00R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmpstcha00 {
        match self.bits {
            false => Cmpstcha00::_0,
            true => Cmpstcha00::_1,
        }
    }
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpstcha00::_0
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpstcha00::_1
    }
}
///Field `CMPSTCHA00` writer - Compare Window A Flag
pub type Cmpstcha00W<'a, REG> = crate::BitWriter<'a, REG, Cmpstcha00>;
impl<'a, REG> Cmpstcha00W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha00::_0)
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha00::_1)
    }
}
/**Compare Window A Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpstcha01 {
    ///0: Comparison conditions are not met.
    _0 = 0,
    ///1: Comparison conditions are met.
    _1 = 1,
}
impl From<Cmpstcha01> for bool {
    #[inline(always)]
    fn from(variant: Cmpstcha01) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPSTCHA01` reader - Compare Window A Flag
pub type Cmpstcha01R = crate::BitReader<Cmpstcha01>;
impl Cmpstcha01R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmpstcha01 {
        match self.bits {
            false => Cmpstcha01::_0,
            true => Cmpstcha01::_1,
        }
    }
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpstcha01::_0
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpstcha01::_1
    }
}
///Field `CMPSTCHA01` writer - Compare Window A Flag
pub type Cmpstcha01W<'a, REG> = crate::BitWriter<'a, REG, Cmpstcha01>;
impl<'a, REG> Cmpstcha01W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha01::_0)
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha01::_1)
    }
}
/**Compare Window A Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpstcha02 {
    ///0: Comparison conditions are not met.
    _0 = 0,
    ///1: Comparison conditions are met.
    _1 = 1,
}
impl From<Cmpstcha02> for bool {
    #[inline(always)]
    fn from(variant: Cmpstcha02) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPSTCHA02` reader - Compare Window A Flag
pub type Cmpstcha02R = crate::BitReader<Cmpstcha02>;
impl Cmpstcha02R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmpstcha02 {
        match self.bits {
            false => Cmpstcha02::_0,
            true => Cmpstcha02::_1,
        }
    }
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpstcha02::_0
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpstcha02::_1
    }
}
///Field `CMPSTCHA02` writer - Compare Window A Flag
pub type Cmpstcha02W<'a, REG> = crate::BitWriter<'a, REG, Cmpstcha02>;
impl<'a, REG> Cmpstcha02W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha02::_0)
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha02::_1)
    }
}
/**Compare Window A Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpstcha03 {
    ///0: Comparison conditions are not met.
    _0 = 0,
    ///1: Comparison conditions are met.
    _1 = 1,
}
impl From<Cmpstcha03> for bool {
    #[inline(always)]
    fn from(variant: Cmpstcha03) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPSTCHA03` reader - Compare Window A Flag
pub type Cmpstcha03R = crate::BitReader<Cmpstcha03>;
impl Cmpstcha03R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmpstcha03 {
        match self.bits {
            false => Cmpstcha03::_0,
            true => Cmpstcha03::_1,
        }
    }
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpstcha03::_0
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpstcha03::_1
    }
}
///Field `CMPSTCHA03` writer - Compare Window A Flag
pub type Cmpstcha03W<'a, REG> = crate::BitWriter<'a, REG, Cmpstcha03>;
impl<'a, REG> Cmpstcha03W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha03::_0)
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha03::_1)
    }
}
/**Compare Window A Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpstcha04 {
    ///0: Comparison conditions are not met.
    _0 = 0,
    ///1: Comparison conditions are met.
    _1 = 1,
}
impl From<Cmpstcha04> for bool {
    #[inline(always)]
    fn from(variant: Cmpstcha04) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPSTCHA04` reader - Compare Window A Flag
pub type Cmpstcha04R = crate::BitReader<Cmpstcha04>;
impl Cmpstcha04R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmpstcha04 {
        match self.bits {
            false => Cmpstcha04::_0,
            true => Cmpstcha04::_1,
        }
    }
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpstcha04::_0
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpstcha04::_1
    }
}
///Field `CMPSTCHA04` writer - Compare Window A Flag
pub type Cmpstcha04W<'a, REG> = crate::BitWriter<'a, REG, Cmpstcha04>;
impl<'a, REG> Cmpstcha04W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha04::_0)
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha04::_1)
    }
}
/**Compare Window A Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpstcha05 {
    ///0: Comparison conditions are not met.
    _0 = 0,
    ///1: Comparison conditions are met.
    _1 = 1,
}
impl From<Cmpstcha05> for bool {
    #[inline(always)]
    fn from(variant: Cmpstcha05) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPSTCHA05` reader - Compare Window A Flag
pub type Cmpstcha05R = crate::BitReader<Cmpstcha05>;
impl Cmpstcha05R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmpstcha05 {
        match self.bits {
            false => Cmpstcha05::_0,
            true => Cmpstcha05::_1,
        }
    }
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpstcha05::_0
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpstcha05::_1
    }
}
///Field `CMPSTCHA05` writer - Compare Window A Flag
pub type Cmpstcha05W<'a, REG> = crate::BitWriter<'a, REG, Cmpstcha05>;
impl<'a, REG> Cmpstcha05W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha05::_0)
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha05::_1)
    }
}
/**Compare Window A Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpstcha06 {
    ///0: Comparison conditions are not met.
    _0 = 0,
    ///1: Comparison conditions are met.
    _1 = 1,
}
impl From<Cmpstcha06> for bool {
    #[inline(always)]
    fn from(variant: Cmpstcha06) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPSTCHA06` reader - Compare Window A Flag
pub type Cmpstcha06R = crate::BitReader<Cmpstcha06>;
impl Cmpstcha06R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmpstcha06 {
        match self.bits {
            false => Cmpstcha06::_0,
            true => Cmpstcha06::_1,
        }
    }
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpstcha06::_0
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpstcha06::_1
    }
}
///Field `CMPSTCHA06` writer - Compare Window A Flag
pub type Cmpstcha06W<'a, REG> = crate::BitWriter<'a, REG, Cmpstcha06>;
impl<'a, REG> Cmpstcha06W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha06::_0)
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha06::_1)
    }
}
/**Compare Window A Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpstcha07 {
    ///0: Comparison conditions are not met.
    _0 = 0,
    ///1: Comparison conditions are met.
    _1 = 1,
}
impl From<Cmpstcha07> for bool {
    #[inline(always)]
    fn from(variant: Cmpstcha07) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPSTCHA07` reader - Compare Window A Flag
pub type Cmpstcha07R = crate::BitReader<Cmpstcha07>;
impl Cmpstcha07R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmpstcha07 {
        match self.bits {
            false => Cmpstcha07::_0,
            true => Cmpstcha07::_1,
        }
    }
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpstcha07::_0
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpstcha07::_1
    }
}
///Field `CMPSTCHA07` writer - Compare Window A Flag
pub type Cmpstcha07W<'a, REG> = crate::BitWriter<'a, REG, Cmpstcha07>;
impl<'a, REG> Cmpstcha07W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha07::_0)
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha07::_1)
    }
}
/**Compare Window A Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpstcha08 {
    ///0: Comparison conditions are not met.
    _0 = 0,
    ///1: Comparison conditions are met.
    _1 = 1,
}
impl From<Cmpstcha08> for bool {
    #[inline(always)]
    fn from(variant: Cmpstcha08) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPSTCHA08` reader - Compare Window A Flag
pub type Cmpstcha08R = crate::BitReader<Cmpstcha08>;
impl Cmpstcha08R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmpstcha08 {
        match self.bits {
            false => Cmpstcha08::_0,
            true => Cmpstcha08::_1,
        }
    }
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpstcha08::_0
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpstcha08::_1
    }
}
///Field `CMPSTCHA08` writer - Compare Window A Flag
pub type Cmpstcha08W<'a, REG> = crate::BitWriter<'a, REG, Cmpstcha08>;
impl<'a, REG> Cmpstcha08W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha08::_0)
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha08::_1)
    }
}
/**Compare Window A Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpstcha09 {
    ///0: Comparison conditions are not met.
    _0 = 0,
    ///1: Comparison conditions are met.
    _1 = 1,
}
impl From<Cmpstcha09> for bool {
    #[inline(always)]
    fn from(variant: Cmpstcha09) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPSTCHA09` reader - Compare Window A Flag
pub type Cmpstcha09R = crate::BitReader<Cmpstcha09>;
impl Cmpstcha09R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmpstcha09 {
        match self.bits {
            false => Cmpstcha09::_0,
            true => Cmpstcha09::_1,
        }
    }
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpstcha09::_0
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpstcha09::_1
    }
}
///Field `CMPSTCHA09` writer - Compare Window A Flag
pub type Cmpstcha09W<'a, REG> = crate::BitWriter<'a, REG, Cmpstcha09>;
impl<'a, REG> Cmpstcha09W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha09::_0)
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha09::_1)
    }
}
/**Compare Window A Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpstcha10 {
    ///0: Comparison conditions are not met.
    _0 = 0,
    ///1: Comparison conditions are met.
    _1 = 1,
}
impl From<Cmpstcha10> for bool {
    #[inline(always)]
    fn from(variant: Cmpstcha10) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPSTCHA10` reader - Compare Window A Flag
pub type Cmpstcha10R = crate::BitReader<Cmpstcha10>;
impl Cmpstcha10R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmpstcha10 {
        match self.bits {
            false => Cmpstcha10::_0,
            true => Cmpstcha10::_1,
        }
    }
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpstcha10::_0
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpstcha10::_1
    }
}
///Field `CMPSTCHA10` writer - Compare Window A Flag
pub type Cmpstcha10W<'a, REG> = crate::BitWriter<'a, REG, Cmpstcha10>;
impl<'a, REG> Cmpstcha10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha10::_0)
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha10::_1)
    }
}
/**Compare Window A Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpstcha12 {
    ///0: Comparison conditions are not met.
    _0 = 0,
    ///1: Comparison conditions are met.
    _1 = 1,
}
impl From<Cmpstcha12> for bool {
    #[inline(always)]
    fn from(variant: Cmpstcha12) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPSTCHA12` reader - Compare Window A Flag
pub type Cmpstcha12R = crate::BitReader<Cmpstcha12>;
impl Cmpstcha12R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmpstcha12 {
        match self.bits {
            false => Cmpstcha12::_0,
            true => Cmpstcha12::_1,
        }
    }
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpstcha12::_0
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpstcha12::_1
    }
}
///Field `CMPSTCHA12` writer - Compare Window A Flag
pub type Cmpstcha12W<'a, REG> = crate::BitWriter<'a, REG, Cmpstcha12>;
impl<'a, REG> Cmpstcha12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha12::_0)
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha12::_1)
    }
}
/**Compare Window A Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpstcha13 {
    ///0: Comparison conditions are not met.
    _0 = 0,
    ///1: Comparison conditions are met.
    _1 = 1,
}
impl From<Cmpstcha13> for bool {
    #[inline(always)]
    fn from(variant: Cmpstcha13) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPSTCHA13` reader - Compare Window A Flag
pub type Cmpstcha13R = crate::BitReader<Cmpstcha13>;
impl Cmpstcha13R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmpstcha13 {
        match self.bits {
            false => Cmpstcha13::_0,
            true => Cmpstcha13::_1,
        }
    }
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpstcha13::_0
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpstcha13::_1
    }
}
///Field `CMPSTCHA13` writer - Compare Window A Flag
pub type Cmpstcha13W<'a, REG> = crate::BitWriter<'a, REG, Cmpstcha13>;
impl<'a, REG> Cmpstcha13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha13::_0)
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha13::_1)
    }
}
impl R {
    ///Bit 0 - Compare Window A Flag
    #[inline(always)]
    pub fn cmpstcha00(&self) -> Cmpstcha00R {
        Cmpstcha00R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Compare Window A Flag
    #[inline(always)]
    pub fn cmpstcha01(&self) -> Cmpstcha01R {
        Cmpstcha01R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Compare Window A Flag
    #[inline(always)]
    pub fn cmpstcha02(&self) -> Cmpstcha02R {
        Cmpstcha02R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Compare Window A Flag
    #[inline(always)]
    pub fn cmpstcha03(&self) -> Cmpstcha03R {
        Cmpstcha03R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Compare Window A Flag
    #[inline(always)]
    pub fn cmpstcha04(&self) -> Cmpstcha04R {
        Cmpstcha04R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Compare Window A Flag
    #[inline(always)]
    pub fn cmpstcha05(&self) -> Cmpstcha05R {
        Cmpstcha05R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Compare Window A Flag
    #[inline(always)]
    pub fn cmpstcha06(&self) -> Cmpstcha06R {
        Cmpstcha06R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Compare Window A Flag
    #[inline(always)]
    pub fn cmpstcha07(&self) -> Cmpstcha07R {
        Cmpstcha07R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Compare Window A Flag
    #[inline(always)]
    pub fn cmpstcha08(&self) -> Cmpstcha08R {
        Cmpstcha08R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Compare Window A Flag
    #[inline(always)]
    pub fn cmpstcha09(&self) -> Cmpstcha09R {
        Cmpstcha09R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Compare Window A Flag
    #[inline(always)]
    pub fn cmpstcha10(&self) -> Cmpstcha10R {
        Cmpstcha10R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 12 - Compare Window A Flag
    #[inline(always)]
    pub fn cmpstcha12(&self) -> Cmpstcha12R {
        Cmpstcha12R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Compare Window A Flag
    #[inline(always)]
    pub fn cmpstcha13(&self) -> Cmpstcha13R {
        Cmpstcha13R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCMPSR0")
            .field("cmpstcha00", &self.cmpstcha00())
            .field("cmpstcha01", &self.cmpstcha01())
            .field("cmpstcha02", &self.cmpstcha02())
            .field("cmpstcha03", &self.cmpstcha03())
            .field("cmpstcha04", &self.cmpstcha04())
            .field("cmpstcha05", &self.cmpstcha05())
            .field("cmpstcha06", &self.cmpstcha06())
            .field("cmpstcha07", &self.cmpstcha07())
            .field("cmpstcha08", &self.cmpstcha08())
            .field("cmpstcha09", &self.cmpstcha09())
            .field("cmpstcha10", &self.cmpstcha10())
            .field("cmpstcha12", &self.cmpstcha12())
            .field("cmpstcha13", &self.cmpstcha13())
            .finish()
    }
}
impl W {
    ///Bit 0 - Compare Window A Flag
    #[inline(always)]
    pub fn cmpstcha00(&mut self) -> Cmpstcha00W<Adcmpsr0Spec> {
        Cmpstcha00W::new(self, 0)
    }
    ///Bit 1 - Compare Window A Flag
    #[inline(always)]
    pub fn cmpstcha01(&mut self) -> Cmpstcha01W<Adcmpsr0Spec> {
        Cmpstcha01W::new(self, 1)
    }
    ///Bit 2 - Compare Window A Flag
    #[inline(always)]
    pub fn cmpstcha02(&mut self) -> Cmpstcha02W<Adcmpsr0Spec> {
        Cmpstcha02W::new(self, 2)
    }
    ///Bit 3 - Compare Window A Flag
    #[inline(always)]
    pub fn cmpstcha03(&mut self) -> Cmpstcha03W<Adcmpsr0Spec> {
        Cmpstcha03W::new(self, 3)
    }
    ///Bit 4 - Compare Window A Flag
    #[inline(always)]
    pub fn cmpstcha04(&mut self) -> Cmpstcha04W<Adcmpsr0Spec> {
        Cmpstcha04W::new(self, 4)
    }
    ///Bit 5 - Compare Window A Flag
    #[inline(always)]
    pub fn cmpstcha05(&mut self) -> Cmpstcha05W<Adcmpsr0Spec> {
        Cmpstcha05W::new(self, 5)
    }
    ///Bit 6 - Compare Window A Flag
    #[inline(always)]
    pub fn cmpstcha06(&mut self) -> Cmpstcha06W<Adcmpsr0Spec> {
        Cmpstcha06W::new(self, 6)
    }
    ///Bit 7 - Compare Window A Flag
    #[inline(always)]
    pub fn cmpstcha07(&mut self) -> Cmpstcha07W<Adcmpsr0Spec> {
        Cmpstcha07W::new(self, 7)
    }
    ///Bit 8 - Compare Window A Flag
    #[inline(always)]
    pub fn cmpstcha08(&mut self) -> Cmpstcha08W<Adcmpsr0Spec> {
        Cmpstcha08W::new(self, 8)
    }
    ///Bit 9 - Compare Window A Flag
    #[inline(always)]
    pub fn cmpstcha09(&mut self) -> Cmpstcha09W<Adcmpsr0Spec> {
        Cmpstcha09W::new(self, 9)
    }
    ///Bit 10 - Compare Window A Flag
    #[inline(always)]
    pub fn cmpstcha10(&mut self) -> Cmpstcha10W<Adcmpsr0Spec> {
        Cmpstcha10W::new(self, 10)
    }
    ///Bit 12 - Compare Window A Flag
    #[inline(always)]
    pub fn cmpstcha12(&mut self) -> Cmpstcha12W<Adcmpsr0Spec> {
        Cmpstcha12W::new(self, 12)
    }
    ///Bit 13 - Compare Window A Flag
    #[inline(always)]
    pub fn cmpstcha13(&mut self) -> Cmpstcha13W<Adcmpsr0Spec> {
        Cmpstcha13W::new(self, 13)
    }
}
/**A/D Compare Function Window A Channel Status Register 0

You can [`read`](crate::Reg::read) this register and get [`adcmpsr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpsr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Adcmpsr0Spec;
impl crate::RegisterSpec for Adcmpsr0Spec {
    type Ux = u16;
}
///`read()` method returns [`adcmpsr0::R`](R) reader structure
impl crate::Readable for Adcmpsr0Spec {}
///`write(|w| ..)` method takes [`adcmpsr0::W`](W) writer structure
impl crate::Writable for Adcmpsr0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADCMPSR0 to value 0
impl crate::Resettable for Adcmpsr0Spec {}
