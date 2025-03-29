///Register `GTSTP` reader
pub type R = crate::R<GtstpSpec>;
///Register `GTSTP` writer
pub type W = crate::W<GtstpSpec>;
/**Channel n GTCNT Count Stop (n : the same as bit position value)

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cstop0 {
    ///0: GTCNT counter not stop
    _0 = 0,
    ///1: GTCNT counter stop
    _1 = 1,
}
impl From<Cstop0> for bool {
    #[inline(always)]
    fn from(variant: Cstop0) -> Self {
        variant as u8 != 0
    }
}
///Field `CSTOP0` reader - Channel n GTCNT Count Stop (n : the same as bit position value)
pub type Cstop0R = crate::BitReader<Cstop0>;
impl Cstop0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cstop0 {
        match self.bits {
            false => Cstop0::_0,
            true => Cstop0::_1,
        }
    }
    ///GTCNT counter not stop
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cstop0::_0
    }
    ///GTCNT counter stop
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cstop0::_1
    }
}
///Field `CSTOP0` writer - Channel n GTCNT Count Stop (n : the same as bit position value)
pub type Cstop0W<'a, REG> = crate::BitWriter<'a, REG, Cstop0>;
impl<'a, REG> Cstop0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///GTCNT counter not stop
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cstop0::_0)
    }
    ///GTCNT counter stop
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cstop0::_1)
    }
}
/**Channel n GTCNT Count Stop (n : the same as bit position value)

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cstop1 {
    ///0: GTCNT counter not stop
    _0 = 0,
    ///1: GTCNT counter stop
    _1 = 1,
}
impl From<Cstop1> for bool {
    #[inline(always)]
    fn from(variant: Cstop1) -> Self {
        variant as u8 != 0
    }
}
///Field `CSTOP1` reader - Channel n GTCNT Count Stop (n : the same as bit position value)
pub type Cstop1R = crate::BitReader<Cstop1>;
impl Cstop1R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cstop1 {
        match self.bits {
            false => Cstop1::_0,
            true => Cstop1::_1,
        }
    }
    ///GTCNT counter not stop
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cstop1::_0
    }
    ///GTCNT counter stop
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cstop1::_1
    }
}
///Field `CSTOP1` writer - Channel n GTCNT Count Stop (n : the same as bit position value)
pub type Cstop1W<'a, REG> = crate::BitWriter<'a, REG, Cstop1>;
impl<'a, REG> Cstop1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///GTCNT counter not stop
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cstop1::_0)
    }
    ///GTCNT counter stop
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cstop1::_1)
    }
}
/**Channel n GTCNT Count Stop (n : the same as bit position value)

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cstop2 {
    ///0: GTCNT counter not stop
    _0 = 0,
    ///1: GTCNT counter stop
    _1 = 1,
}
impl From<Cstop2> for bool {
    #[inline(always)]
    fn from(variant: Cstop2) -> Self {
        variant as u8 != 0
    }
}
///Field `CSTOP2` reader - Channel n GTCNT Count Stop (n : the same as bit position value)
pub type Cstop2R = crate::BitReader<Cstop2>;
impl Cstop2R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cstop2 {
        match self.bits {
            false => Cstop2::_0,
            true => Cstop2::_1,
        }
    }
    ///GTCNT counter not stop
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cstop2::_0
    }
    ///GTCNT counter stop
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cstop2::_1
    }
}
///Field `CSTOP2` writer - Channel n GTCNT Count Stop (n : the same as bit position value)
pub type Cstop2W<'a, REG> = crate::BitWriter<'a, REG, Cstop2>;
impl<'a, REG> Cstop2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///GTCNT counter not stop
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cstop2::_0)
    }
    ///GTCNT counter stop
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cstop2::_1)
    }
}
/**Channel n GTCNT Count Stop (n : the same as bit position value)

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cstop3 {
    ///0: GTCNT counter not stop
    _0 = 0,
    ///1: GTCNT counter stop
    _1 = 1,
}
impl From<Cstop3> for bool {
    #[inline(always)]
    fn from(variant: Cstop3) -> Self {
        variant as u8 != 0
    }
}
///Field `CSTOP3` reader - Channel n GTCNT Count Stop (n : the same as bit position value)
pub type Cstop3R = crate::BitReader<Cstop3>;
impl Cstop3R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cstop3 {
        match self.bits {
            false => Cstop3::_0,
            true => Cstop3::_1,
        }
    }
    ///GTCNT counter not stop
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cstop3::_0
    }
    ///GTCNT counter stop
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cstop3::_1
    }
}
///Field `CSTOP3` writer - Channel n GTCNT Count Stop (n : the same as bit position value)
pub type Cstop3W<'a, REG> = crate::BitWriter<'a, REG, Cstop3>;
impl<'a, REG> Cstop3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///GTCNT counter not stop
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cstop3::_0)
    }
    ///GTCNT counter stop
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cstop3::_1)
    }
}
/**Channel n GTCNT Count Stop (n : the same as bit position value)

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cstop4 {
    ///0: GTCNT counter not stop
    _0 = 0,
    ///1: GTCNT counter stop
    _1 = 1,
}
impl From<Cstop4> for bool {
    #[inline(always)]
    fn from(variant: Cstop4) -> Self {
        variant as u8 != 0
    }
}
///Field `CSTOP4` reader - Channel n GTCNT Count Stop (n : the same as bit position value)
pub type Cstop4R = crate::BitReader<Cstop4>;
impl Cstop4R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cstop4 {
        match self.bits {
            false => Cstop4::_0,
            true => Cstop4::_1,
        }
    }
    ///GTCNT counter not stop
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cstop4::_0
    }
    ///GTCNT counter stop
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cstop4::_1
    }
}
///Field `CSTOP4` writer - Channel n GTCNT Count Stop (n : the same as bit position value)
pub type Cstop4W<'a, REG> = crate::BitWriter<'a, REG, Cstop4>;
impl<'a, REG> Cstop4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///GTCNT counter not stop
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cstop4::_0)
    }
    ///GTCNT counter stop
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cstop4::_1)
    }
}
/**Channel n GTCNT Count Stop (n : the same as bit position value)

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cstop5 {
    ///0: GTCNT counter not stop
    _0 = 0,
    ///1: GTCNT counter stop
    _1 = 1,
}
impl From<Cstop5> for bool {
    #[inline(always)]
    fn from(variant: Cstop5) -> Self {
        variant as u8 != 0
    }
}
///Field `CSTOP5` reader - Channel n GTCNT Count Stop (n : the same as bit position value)
pub type Cstop5R = crate::BitReader<Cstop5>;
impl Cstop5R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cstop5 {
        match self.bits {
            false => Cstop5::_0,
            true => Cstop5::_1,
        }
    }
    ///GTCNT counter not stop
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cstop5::_0
    }
    ///GTCNT counter stop
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cstop5::_1
    }
}
///Field `CSTOP5` writer - Channel n GTCNT Count Stop (n : the same as bit position value)
pub type Cstop5W<'a, REG> = crate::BitWriter<'a, REG, Cstop5>;
impl<'a, REG> Cstop5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///GTCNT counter not stop
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cstop5::_0)
    }
    ///GTCNT counter stop
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cstop5::_1)
    }
}
/**Channel n GTCNT Count Stop (n : the same as bit position value)

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cstop6 {
    ///0: GTCNT counter not stop
    _0 = 0,
    ///1: GTCNT counter stop
    _1 = 1,
}
impl From<Cstop6> for bool {
    #[inline(always)]
    fn from(variant: Cstop6) -> Self {
        variant as u8 != 0
    }
}
///Field `CSTOP6` reader - Channel n GTCNT Count Stop (n : the same as bit position value)
pub type Cstop6R = crate::BitReader<Cstop6>;
impl Cstop6R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cstop6 {
        match self.bits {
            false => Cstop6::_0,
            true => Cstop6::_1,
        }
    }
    ///GTCNT counter not stop
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cstop6::_0
    }
    ///GTCNT counter stop
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cstop6::_1
    }
}
///Field `CSTOP6` writer - Channel n GTCNT Count Stop (n : the same as bit position value)
pub type Cstop6W<'a, REG> = crate::BitWriter<'a, REG, Cstop6>;
impl<'a, REG> Cstop6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///GTCNT counter not stop
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cstop6::_0)
    }
    ///GTCNT counter stop
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cstop6::_1)
    }
}
/**Channel n GTCNT Count Stop (n : the same as bit position value)

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cstop7 {
    ///0: GTCNT counter not stop
    _0 = 0,
    ///1: GTCNT counter stop
    _1 = 1,
}
impl From<Cstop7> for bool {
    #[inline(always)]
    fn from(variant: Cstop7) -> Self {
        variant as u8 != 0
    }
}
///Field `CSTOP7` reader - Channel n GTCNT Count Stop (n : the same as bit position value)
pub type Cstop7R = crate::BitReader<Cstop7>;
impl Cstop7R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cstop7 {
        match self.bits {
            false => Cstop7::_0,
            true => Cstop7::_1,
        }
    }
    ///GTCNT counter not stop
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cstop7::_0
    }
    ///GTCNT counter stop
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cstop7::_1
    }
}
///Field `CSTOP7` writer - Channel n GTCNT Count Stop (n : the same as bit position value)
pub type Cstop7W<'a, REG> = crate::BitWriter<'a, REG, Cstop7>;
impl<'a, REG> Cstop7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///GTCNT counter not stop
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cstop7::_0)
    }
    ///GTCNT counter stop
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cstop7::_1)
    }
}
/**Channel n GTCNT Count Stop (n : the same as bit position value)

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cstop8 {
    ///0: GTCNT counter not stop
    _0 = 0,
    ///1: GTCNT counter stop
    _1 = 1,
}
impl From<Cstop8> for bool {
    #[inline(always)]
    fn from(variant: Cstop8) -> Self {
        variant as u8 != 0
    }
}
///Field `CSTOP8` reader - Channel n GTCNT Count Stop (n : the same as bit position value)
pub type Cstop8R = crate::BitReader<Cstop8>;
impl Cstop8R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cstop8 {
        match self.bits {
            false => Cstop8::_0,
            true => Cstop8::_1,
        }
    }
    ///GTCNT counter not stop
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cstop8::_0
    }
    ///GTCNT counter stop
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cstop8::_1
    }
}
///Field `CSTOP8` writer - Channel n GTCNT Count Stop (n : the same as bit position value)
pub type Cstop8W<'a, REG> = crate::BitWriter<'a, REG, Cstop8>;
impl<'a, REG> Cstop8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///GTCNT counter not stop
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cstop8::_0)
    }
    ///GTCNT counter stop
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cstop8::_1)
    }
}
/**Channel n GTCNT Count Stop (n : the same as bit position value)

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cstop9 {
    ///0: GTCNT counter not stop
    _0 = 0,
    ///1: GTCNT counter stop
    _1 = 1,
}
impl From<Cstop9> for bool {
    #[inline(always)]
    fn from(variant: Cstop9) -> Self {
        variant as u8 != 0
    }
}
///Field `CSTOP9` reader - Channel n GTCNT Count Stop (n : the same as bit position value)
pub type Cstop9R = crate::BitReader<Cstop9>;
impl Cstop9R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cstop9 {
        match self.bits {
            false => Cstop9::_0,
            true => Cstop9::_1,
        }
    }
    ///GTCNT counter not stop
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cstop9::_0
    }
    ///GTCNT counter stop
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cstop9::_1
    }
}
///Field `CSTOP9` writer - Channel n GTCNT Count Stop (n : the same as bit position value)
pub type Cstop9W<'a, REG> = crate::BitWriter<'a, REG, Cstop9>;
impl<'a, REG> Cstop9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///GTCNT counter not stop
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cstop9::_0)
    }
    ///GTCNT counter stop
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cstop9::_1)
    }
}
impl R {
    ///Bit 0 - Channel n GTCNT Count Stop (n : the same as bit position value)
    #[inline(always)]
    pub fn cstop0(&self) -> Cstop0R {
        Cstop0R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Channel n GTCNT Count Stop (n : the same as bit position value)
    #[inline(always)]
    pub fn cstop1(&self) -> Cstop1R {
        Cstop1R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Channel n GTCNT Count Stop (n : the same as bit position value)
    #[inline(always)]
    pub fn cstop2(&self) -> Cstop2R {
        Cstop2R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Channel n GTCNT Count Stop (n : the same as bit position value)
    #[inline(always)]
    pub fn cstop3(&self) -> Cstop3R {
        Cstop3R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Channel n GTCNT Count Stop (n : the same as bit position value)
    #[inline(always)]
    pub fn cstop4(&self) -> Cstop4R {
        Cstop4R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Channel n GTCNT Count Stop (n : the same as bit position value)
    #[inline(always)]
    pub fn cstop5(&self) -> Cstop5R {
        Cstop5R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Channel n GTCNT Count Stop (n : the same as bit position value)
    #[inline(always)]
    pub fn cstop6(&self) -> Cstop6R {
        Cstop6R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Channel n GTCNT Count Stop (n : the same as bit position value)
    #[inline(always)]
    pub fn cstop7(&self) -> Cstop7R {
        Cstop7R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Channel n GTCNT Count Stop (n : the same as bit position value)
    #[inline(always)]
    pub fn cstop8(&self) -> Cstop8R {
        Cstop8R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Channel n GTCNT Count Stop (n : the same as bit position value)
    #[inline(always)]
    pub fn cstop9(&self) -> Cstop9R {
        Cstop9R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GTSTP")
            .field("cstop0", &self.cstop0())
            .field("cstop1", &self.cstop1())
            .field("cstop2", &self.cstop2())
            .field("cstop3", &self.cstop3())
            .field("cstop4", &self.cstop4())
            .field("cstop5", &self.cstop5())
            .field("cstop6", &self.cstop6())
            .field("cstop7", &self.cstop7())
            .field("cstop8", &self.cstop8())
            .field("cstop9", &self.cstop9())
            .finish()
    }
}
impl W {
    ///Bit 0 - Channel n GTCNT Count Stop (n : the same as bit position value)
    #[inline(always)]
    pub fn cstop0(&mut self) -> Cstop0W<GtstpSpec> {
        Cstop0W::new(self, 0)
    }
    ///Bit 1 - Channel n GTCNT Count Stop (n : the same as bit position value)
    #[inline(always)]
    pub fn cstop1(&mut self) -> Cstop1W<GtstpSpec> {
        Cstop1W::new(self, 1)
    }
    ///Bit 2 - Channel n GTCNT Count Stop (n : the same as bit position value)
    #[inline(always)]
    pub fn cstop2(&mut self) -> Cstop2W<GtstpSpec> {
        Cstop2W::new(self, 2)
    }
    ///Bit 3 - Channel n GTCNT Count Stop (n : the same as bit position value)
    #[inline(always)]
    pub fn cstop3(&mut self) -> Cstop3W<GtstpSpec> {
        Cstop3W::new(self, 3)
    }
    ///Bit 4 - Channel n GTCNT Count Stop (n : the same as bit position value)
    #[inline(always)]
    pub fn cstop4(&mut self) -> Cstop4W<GtstpSpec> {
        Cstop4W::new(self, 4)
    }
    ///Bit 5 - Channel n GTCNT Count Stop (n : the same as bit position value)
    #[inline(always)]
    pub fn cstop5(&mut self) -> Cstop5W<GtstpSpec> {
        Cstop5W::new(self, 5)
    }
    ///Bit 6 - Channel n GTCNT Count Stop (n : the same as bit position value)
    #[inline(always)]
    pub fn cstop6(&mut self) -> Cstop6W<GtstpSpec> {
        Cstop6W::new(self, 6)
    }
    ///Bit 7 - Channel n GTCNT Count Stop (n : the same as bit position value)
    #[inline(always)]
    pub fn cstop7(&mut self) -> Cstop7W<GtstpSpec> {
        Cstop7W::new(self, 7)
    }
    ///Bit 8 - Channel n GTCNT Count Stop (n : the same as bit position value)
    #[inline(always)]
    pub fn cstop8(&mut self) -> Cstop8W<GtstpSpec> {
        Cstop8W::new(self, 8)
    }
    ///Bit 9 - Channel n GTCNT Count Stop (n : the same as bit position value)
    #[inline(always)]
    pub fn cstop9(&mut self) -> Cstop9W<GtstpSpec> {
        Cstop9W::new(self, 9)
    }
}
/**General PWM Timer Software Stop Register

You can [`read`](crate::Reg::read) this register and get [`gtstp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtstp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GtstpSpec;
impl crate::RegisterSpec for GtstpSpec {
    type Ux = u32;
}
///`read()` method returns [`gtstp::R`](R) reader structure
impl crate::Readable for GtstpSpec {}
///`write(|w| ..)` method takes [`gtstp::W`](W) writer structure
impl crate::Writable for GtstpSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTSTP to value 0xffff_ffff
impl crate::Resettable for GtstpSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
