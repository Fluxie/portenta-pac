///Register `ADCMPSR1` reader
pub type R = crate::R<Adcmpsr1Spec>;
///Register `ADCMPSR1` writer
pub type W = crate::W<Adcmpsr1Spec>;
/**Compare Window A Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpstcha16 {
    ///0: Comparison conditions are not met.
    _0 = 0,
    ///1: Comparison conditions are met.
    _1 = 1,
}
impl From<Cmpstcha16> for bool {
    #[inline(always)]
    fn from(variant: Cmpstcha16) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPSTCHA16` reader - Compare Window A Flag
pub type Cmpstcha16R = crate::BitReader<Cmpstcha16>;
impl Cmpstcha16R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmpstcha16 {
        match self.bits {
            false => Cmpstcha16::_0,
            true => Cmpstcha16::_1,
        }
    }
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpstcha16::_0
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpstcha16::_1
    }
}
///Field `CMPSTCHA16` writer - Compare Window A Flag
pub type Cmpstcha16W<'a, REG> = crate::BitWriter<'a, REG, Cmpstcha16>;
impl<'a, REG> Cmpstcha16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha16::_0)
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha16::_1)
    }
}
/**Compare Window A Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpstcha17 {
    ///0: Comparison conditions are not met.
    _0 = 0,
    ///1: Comparison conditions are met.
    _1 = 1,
}
impl From<Cmpstcha17> for bool {
    #[inline(always)]
    fn from(variant: Cmpstcha17) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPSTCHA17` reader - Compare Window A Flag
pub type Cmpstcha17R = crate::BitReader<Cmpstcha17>;
impl Cmpstcha17R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmpstcha17 {
        match self.bits {
            false => Cmpstcha17::_0,
            true => Cmpstcha17::_1,
        }
    }
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpstcha17::_0
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpstcha17::_1
    }
}
///Field `CMPSTCHA17` writer - Compare Window A Flag
pub type Cmpstcha17W<'a, REG> = crate::BitWriter<'a, REG, Cmpstcha17>;
impl<'a, REG> Cmpstcha17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha17::_0)
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha17::_1)
    }
}
/**Compare Window A Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpstcha18 {
    ///0: Comparison conditions are not met.
    _0 = 0,
    ///1: Comparison conditions are met.
    _1 = 1,
}
impl From<Cmpstcha18> for bool {
    #[inline(always)]
    fn from(variant: Cmpstcha18) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPSTCHA18` reader - Compare Window A Flag
pub type Cmpstcha18R = crate::BitReader<Cmpstcha18>;
impl Cmpstcha18R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmpstcha18 {
        match self.bits {
            false => Cmpstcha18::_0,
            true => Cmpstcha18::_1,
        }
    }
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpstcha18::_0
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpstcha18::_1
    }
}
///Field `CMPSTCHA18` writer - Compare Window A Flag
pub type Cmpstcha18W<'a, REG> = crate::BitWriter<'a, REG, Cmpstcha18>;
impl<'a, REG> Cmpstcha18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha18::_0)
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha18::_1)
    }
}
/**Compare Window A Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpstcha19 {
    ///0: Comparison conditions are not met.
    _0 = 0,
    ///1: Comparison conditions are met.
    _1 = 1,
}
impl From<Cmpstcha19> for bool {
    #[inline(always)]
    fn from(variant: Cmpstcha19) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPSTCHA19` reader - Compare Window A Flag
pub type Cmpstcha19R = crate::BitReader<Cmpstcha19>;
impl Cmpstcha19R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmpstcha19 {
        match self.bits {
            false => Cmpstcha19::_0,
            true => Cmpstcha19::_1,
        }
    }
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpstcha19::_0
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpstcha19::_1
    }
}
///Field `CMPSTCHA19` writer - Compare Window A Flag
pub type Cmpstcha19W<'a, REG> = crate::BitWriter<'a, REG, Cmpstcha19>;
impl<'a, REG> Cmpstcha19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha19::_0)
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha19::_1)
    }
}
/**Compare Window A Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpstcha20 {
    ///0: Comparison conditions are not met.
    _0 = 0,
    ///1: Comparison conditions are met.
    _1 = 1,
}
impl From<Cmpstcha20> for bool {
    #[inline(always)]
    fn from(variant: Cmpstcha20) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPSTCHA20` reader - Compare Window A Flag
pub type Cmpstcha20R = crate::BitReader<Cmpstcha20>;
impl Cmpstcha20R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmpstcha20 {
        match self.bits {
            false => Cmpstcha20::_0,
            true => Cmpstcha20::_1,
        }
    }
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpstcha20::_0
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpstcha20::_1
    }
}
///Field `CMPSTCHA20` writer - Compare Window A Flag
pub type Cmpstcha20W<'a, REG> = crate::BitWriter<'a, REG, Cmpstcha20>;
impl<'a, REG> Cmpstcha20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha20::_0)
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha20::_1)
    }
}
/**Compare Window A Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpstcha21 {
    ///0: Comparison conditions are not met.
    _0 = 0,
    ///1: Comparison conditions are met.
    _1 = 1,
}
impl From<Cmpstcha21> for bool {
    #[inline(always)]
    fn from(variant: Cmpstcha21) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPSTCHA21` reader - Compare Window A Flag
pub type Cmpstcha21R = crate::BitReader<Cmpstcha21>;
impl Cmpstcha21R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmpstcha21 {
        match self.bits {
            false => Cmpstcha21::_0,
            true => Cmpstcha21::_1,
        }
    }
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpstcha21::_0
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpstcha21::_1
    }
}
///Field `CMPSTCHA21` writer - Compare Window A Flag
pub type Cmpstcha21W<'a, REG> = crate::BitWriter<'a, REG, Cmpstcha21>;
impl<'a, REG> Cmpstcha21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha21::_0)
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha21::_1)
    }
}
/**Compare Window A Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpstcha22 {
    ///0: Comparison conditions are not met.
    _0 = 0,
    ///1: Comparison conditions are met.
    _1 = 1,
}
impl From<Cmpstcha22> for bool {
    #[inline(always)]
    fn from(variant: Cmpstcha22) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPSTCHA22` reader - Compare Window A Flag
pub type Cmpstcha22R = crate::BitReader<Cmpstcha22>;
impl Cmpstcha22R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmpstcha22 {
        match self.bits {
            false => Cmpstcha22::_0,
            true => Cmpstcha22::_1,
        }
    }
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpstcha22::_0
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpstcha22::_1
    }
}
///Field `CMPSTCHA22` writer - Compare Window A Flag
pub type Cmpstcha22W<'a, REG> = crate::BitWriter<'a, REG, Cmpstcha22>;
impl<'a, REG> Cmpstcha22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha22::_0)
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha22::_1)
    }
}
/**Compare Window A Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpstcha23 {
    ///0: Comparison conditions are not met.
    _0 = 0,
    ///1: Comparison conditions are met.
    _1 = 1,
}
impl From<Cmpstcha23> for bool {
    #[inline(always)]
    fn from(variant: Cmpstcha23) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPSTCHA23` reader - Compare Window A Flag
pub type Cmpstcha23R = crate::BitReader<Cmpstcha23>;
impl Cmpstcha23R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmpstcha23 {
        match self.bits {
            false => Cmpstcha23::_0,
            true => Cmpstcha23::_1,
        }
    }
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpstcha23::_0
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpstcha23::_1
    }
}
///Field `CMPSTCHA23` writer - Compare Window A Flag
pub type Cmpstcha23W<'a, REG> = crate::BitWriter<'a, REG, Cmpstcha23>;
impl<'a, REG> Cmpstcha23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha23::_0)
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha23::_1)
    }
}
/**Compare Window A Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpstcha24 {
    ///0: Comparison conditions are not met.
    _0 = 0,
    ///1: Comparison conditions are met.
    _1 = 1,
}
impl From<Cmpstcha24> for bool {
    #[inline(always)]
    fn from(variant: Cmpstcha24) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPSTCHA24` reader - Compare Window A Flag
pub type Cmpstcha24R = crate::BitReader<Cmpstcha24>;
impl Cmpstcha24R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmpstcha24 {
        match self.bits {
            false => Cmpstcha24::_0,
            true => Cmpstcha24::_1,
        }
    }
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpstcha24::_0
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpstcha24::_1
    }
}
///Field `CMPSTCHA24` writer - Compare Window A Flag
pub type Cmpstcha24W<'a, REG> = crate::BitWriter<'a, REG, Cmpstcha24>;
impl<'a, REG> Cmpstcha24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha24::_0)
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha24::_1)
    }
}
/**Compare Window A Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpstcha25 {
    ///0: Comparison conditions are not met.
    _0 = 0,
    ///1: Comparison conditions are met.
    _1 = 1,
}
impl From<Cmpstcha25> for bool {
    #[inline(always)]
    fn from(variant: Cmpstcha25) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPSTCHA25` reader - Compare Window A Flag
pub type Cmpstcha25R = crate::BitReader<Cmpstcha25>;
impl Cmpstcha25R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmpstcha25 {
        match self.bits {
            false => Cmpstcha25::_0,
            true => Cmpstcha25::_1,
        }
    }
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpstcha25::_0
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpstcha25::_1
    }
}
///Field `CMPSTCHA25` writer - Compare Window A Flag
pub type Cmpstcha25W<'a, REG> = crate::BitWriter<'a, REG, Cmpstcha25>;
impl<'a, REG> Cmpstcha25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha25::_0)
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha25::_1)
    }
}
/**Compare Window A Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpstcha26 {
    ///0: Comparison conditions are not met.
    _0 = 0,
    ///1: Comparison conditions are met.
    _1 = 1,
}
impl From<Cmpstcha26> for bool {
    #[inline(always)]
    fn from(variant: Cmpstcha26) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPSTCHA26` reader - Compare Window A Flag
pub type Cmpstcha26R = crate::BitReader<Cmpstcha26>;
impl Cmpstcha26R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmpstcha26 {
        match self.bits {
            false => Cmpstcha26::_0,
            true => Cmpstcha26::_1,
        }
    }
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpstcha26::_0
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpstcha26::_1
    }
}
///Field `CMPSTCHA26` writer - Compare Window A Flag
pub type Cmpstcha26W<'a, REG> = crate::BitWriter<'a, REG, Cmpstcha26>;
impl<'a, REG> Cmpstcha26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha26::_0)
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha26::_1)
    }
}
/**Compare Window A Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpstcha27 {
    ///0: Comparison conditions are not met.
    _0 = 0,
    ///1: Comparison conditions are met.
    _1 = 1,
}
impl From<Cmpstcha27> for bool {
    #[inline(always)]
    fn from(variant: Cmpstcha27) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPSTCHA27` reader - Compare Window A Flag
pub type Cmpstcha27R = crate::BitReader<Cmpstcha27>;
impl Cmpstcha27R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmpstcha27 {
        match self.bits {
            false => Cmpstcha27::_0,
            true => Cmpstcha27::_1,
        }
    }
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpstcha27::_0
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpstcha27::_1
    }
}
///Field `CMPSTCHA27` writer - Compare Window A Flag
pub type Cmpstcha27W<'a, REG> = crate::BitWriter<'a, REG, Cmpstcha27>;
impl<'a, REG> Cmpstcha27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha27::_0)
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha27::_1)
    }
}
/**Compare Window A Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpstcha28 {
    ///0: Comparison conditions are not met.
    _0 = 0,
    ///1: Comparison conditions are met.
    _1 = 1,
}
impl From<Cmpstcha28> for bool {
    #[inline(always)]
    fn from(variant: Cmpstcha28) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPSTCHA28` reader - Compare Window A Flag
pub type Cmpstcha28R = crate::BitReader<Cmpstcha28>;
impl Cmpstcha28R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmpstcha28 {
        match self.bits {
            false => Cmpstcha28::_0,
            true => Cmpstcha28::_1,
        }
    }
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpstcha28::_0
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpstcha28::_1
    }
}
///Field `CMPSTCHA28` writer - Compare Window A Flag
pub type Cmpstcha28W<'a, REG> = crate::BitWriter<'a, REG, Cmpstcha28>;
impl<'a, REG> Cmpstcha28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha28::_0)
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstcha28::_1)
    }
}
impl R {
    ///Bit 0 - Compare Window A Flag
    #[inline(always)]
    pub fn cmpstcha16(&self) -> Cmpstcha16R {
        Cmpstcha16R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Compare Window A Flag
    #[inline(always)]
    pub fn cmpstcha17(&self) -> Cmpstcha17R {
        Cmpstcha17R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Compare Window A Flag
    #[inline(always)]
    pub fn cmpstcha18(&self) -> Cmpstcha18R {
        Cmpstcha18R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Compare Window A Flag
    #[inline(always)]
    pub fn cmpstcha19(&self) -> Cmpstcha19R {
        Cmpstcha19R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Compare Window A Flag
    #[inline(always)]
    pub fn cmpstcha20(&self) -> Cmpstcha20R {
        Cmpstcha20R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Compare Window A Flag
    #[inline(always)]
    pub fn cmpstcha21(&self) -> Cmpstcha21R {
        Cmpstcha21R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Compare Window A Flag
    #[inline(always)]
    pub fn cmpstcha22(&self) -> Cmpstcha22R {
        Cmpstcha22R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Compare Window A Flag
    #[inline(always)]
    pub fn cmpstcha23(&self) -> Cmpstcha23R {
        Cmpstcha23R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Compare Window A Flag
    #[inline(always)]
    pub fn cmpstcha24(&self) -> Cmpstcha24R {
        Cmpstcha24R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Compare Window A Flag
    #[inline(always)]
    pub fn cmpstcha25(&self) -> Cmpstcha25R {
        Cmpstcha25R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Compare Window A Flag
    #[inline(always)]
    pub fn cmpstcha26(&self) -> Cmpstcha26R {
        Cmpstcha26R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Compare Window A Flag
    #[inline(always)]
    pub fn cmpstcha27(&self) -> Cmpstcha27R {
        Cmpstcha27R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Compare Window A Flag
    #[inline(always)]
    pub fn cmpstcha28(&self) -> Cmpstcha28R {
        Cmpstcha28R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCMPSR1")
            .field("cmpstcha16", &self.cmpstcha16())
            .field("cmpstcha17", &self.cmpstcha17())
            .field("cmpstcha18", &self.cmpstcha18())
            .field("cmpstcha19", &self.cmpstcha19())
            .field("cmpstcha20", &self.cmpstcha20())
            .field("cmpstcha21", &self.cmpstcha21())
            .field("cmpstcha22", &self.cmpstcha22())
            .field("cmpstcha23", &self.cmpstcha23())
            .field("cmpstcha24", &self.cmpstcha24())
            .field("cmpstcha25", &self.cmpstcha25())
            .field("cmpstcha26", &self.cmpstcha26())
            .field("cmpstcha27", &self.cmpstcha27())
            .field("cmpstcha28", &self.cmpstcha28())
            .finish()
    }
}
impl W {
    ///Bit 0 - Compare Window A Flag
    #[inline(always)]
    pub fn cmpstcha16(&mut self) -> Cmpstcha16W<Adcmpsr1Spec> {
        Cmpstcha16W::new(self, 0)
    }
    ///Bit 1 - Compare Window A Flag
    #[inline(always)]
    pub fn cmpstcha17(&mut self) -> Cmpstcha17W<Adcmpsr1Spec> {
        Cmpstcha17W::new(self, 1)
    }
    ///Bit 2 - Compare Window A Flag
    #[inline(always)]
    pub fn cmpstcha18(&mut self) -> Cmpstcha18W<Adcmpsr1Spec> {
        Cmpstcha18W::new(self, 2)
    }
    ///Bit 3 - Compare Window A Flag
    #[inline(always)]
    pub fn cmpstcha19(&mut self) -> Cmpstcha19W<Adcmpsr1Spec> {
        Cmpstcha19W::new(self, 3)
    }
    ///Bit 4 - Compare Window A Flag
    #[inline(always)]
    pub fn cmpstcha20(&mut self) -> Cmpstcha20W<Adcmpsr1Spec> {
        Cmpstcha20W::new(self, 4)
    }
    ///Bit 5 - Compare Window A Flag
    #[inline(always)]
    pub fn cmpstcha21(&mut self) -> Cmpstcha21W<Adcmpsr1Spec> {
        Cmpstcha21W::new(self, 5)
    }
    ///Bit 6 - Compare Window A Flag
    #[inline(always)]
    pub fn cmpstcha22(&mut self) -> Cmpstcha22W<Adcmpsr1Spec> {
        Cmpstcha22W::new(self, 6)
    }
    ///Bit 7 - Compare Window A Flag
    #[inline(always)]
    pub fn cmpstcha23(&mut self) -> Cmpstcha23W<Adcmpsr1Spec> {
        Cmpstcha23W::new(self, 7)
    }
    ///Bit 8 - Compare Window A Flag
    #[inline(always)]
    pub fn cmpstcha24(&mut self) -> Cmpstcha24W<Adcmpsr1Spec> {
        Cmpstcha24W::new(self, 8)
    }
    ///Bit 9 - Compare Window A Flag
    #[inline(always)]
    pub fn cmpstcha25(&mut self) -> Cmpstcha25W<Adcmpsr1Spec> {
        Cmpstcha25W::new(self, 9)
    }
    ///Bit 10 - Compare Window A Flag
    #[inline(always)]
    pub fn cmpstcha26(&mut self) -> Cmpstcha26W<Adcmpsr1Spec> {
        Cmpstcha26W::new(self, 10)
    }
    ///Bit 11 - Compare Window A Flag
    #[inline(always)]
    pub fn cmpstcha27(&mut self) -> Cmpstcha27W<Adcmpsr1Spec> {
        Cmpstcha27W::new(self, 11)
    }
    ///Bit 12 - Compare Window A Flag
    #[inline(always)]
    pub fn cmpstcha28(&mut self) -> Cmpstcha28W<Adcmpsr1Spec> {
        Cmpstcha28W::new(self, 12)
    }
}
/**A/D Compare Function Window A Channel Status Register1

You can [`read`](crate::Reg::read) this register and get [`adcmpsr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpsr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Adcmpsr1Spec;
impl crate::RegisterSpec for Adcmpsr1Spec {
    type Ux = u16;
}
///`read()` method returns [`adcmpsr1::R`](R) reader structure
impl crate::Readable for Adcmpsr1Spec {}
///`write(|w| ..)` method takes [`adcmpsr1::W`](W) writer structure
impl crate::Writable for Adcmpsr1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADCMPSR1 to value 0
impl crate::Resettable for Adcmpsr1Spec {}
