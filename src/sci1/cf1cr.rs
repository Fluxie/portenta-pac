///Register `CF1CR` reader
pub type R = crate::R<Cf1crSpec>;
///Register `CF1CR` writer
pub type W = crate::W<Cf1crSpec>;
/**Control Field 1 Bit 0 Compare Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cf1ce0 {
    ///0: Comparison with bit 0 of Control Field 1 is disabled.
    _0 = 0,
    ///1: Comparison with bit 0 of Control Field 1 is enabled.
    _1 = 1,
}
impl From<Cf1ce0> for bool {
    #[inline(always)]
    fn from(variant: Cf1ce0) -> Self {
        variant as u8 != 0
    }
}
///Field `CF1CE0` reader - Control Field 1 Bit 0 Compare Enable
pub type Cf1ce0R = crate::BitReader<Cf1ce0>;
impl Cf1ce0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cf1ce0 {
        match self.bits {
            false => Cf1ce0::_0,
            true => Cf1ce0::_1,
        }
    }
    ///Comparison with bit 0 of Control Field 1 is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cf1ce0::_0
    }
    ///Comparison with bit 0 of Control Field 1 is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cf1ce0::_1
    }
}
///Field `CF1CE0` writer - Control Field 1 Bit 0 Compare Enable
pub type Cf1ce0W<'a, REG> = crate::BitWriter<'a, REG, Cf1ce0>;
impl<'a, REG> Cf1ce0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison with bit 0 of Control Field 1 is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cf1ce0::_0)
    }
    ///Comparison with bit 0 of Control Field 1 is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cf1ce0::_1)
    }
}
/**Control Field 1 Bit 1 Compare Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cf1ce1 {
    ///0: Comparison with bit 1 of Control Field 1 is disabled.
    _0 = 0,
    ///1: Comparison with bit 1 of Control Field 1 is enabled.
    _1 = 1,
}
impl From<Cf1ce1> for bool {
    #[inline(always)]
    fn from(variant: Cf1ce1) -> Self {
        variant as u8 != 0
    }
}
///Field `CF1CE1` reader - Control Field 1 Bit 1 Compare Enable
pub type Cf1ce1R = crate::BitReader<Cf1ce1>;
impl Cf1ce1R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cf1ce1 {
        match self.bits {
            false => Cf1ce1::_0,
            true => Cf1ce1::_1,
        }
    }
    ///Comparison with bit 1 of Control Field 1 is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cf1ce1::_0
    }
    ///Comparison with bit 1 of Control Field 1 is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cf1ce1::_1
    }
}
///Field `CF1CE1` writer - Control Field 1 Bit 1 Compare Enable
pub type Cf1ce1W<'a, REG> = crate::BitWriter<'a, REG, Cf1ce1>;
impl<'a, REG> Cf1ce1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison with bit 1 of Control Field 1 is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cf1ce1::_0)
    }
    ///Comparison with bit 1 of Control Field 1 is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cf1ce1::_1)
    }
}
/**Control Field 1 Bit 2 Compare Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cf1ce2 {
    ///0: Comparison with bit 2 of Control Field 1 is disabled.
    _0 = 0,
    ///1: Comparison with bit 2 of Control Field 1 is enabled.
    _1 = 1,
}
impl From<Cf1ce2> for bool {
    #[inline(always)]
    fn from(variant: Cf1ce2) -> Self {
        variant as u8 != 0
    }
}
///Field `CF1CE2` reader - Control Field 1 Bit 2 Compare Enable
pub type Cf1ce2R = crate::BitReader<Cf1ce2>;
impl Cf1ce2R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cf1ce2 {
        match self.bits {
            false => Cf1ce2::_0,
            true => Cf1ce2::_1,
        }
    }
    ///Comparison with bit 2 of Control Field 1 is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cf1ce2::_0
    }
    ///Comparison with bit 2 of Control Field 1 is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cf1ce2::_1
    }
}
///Field `CF1CE2` writer - Control Field 1 Bit 2 Compare Enable
pub type Cf1ce2W<'a, REG> = crate::BitWriter<'a, REG, Cf1ce2>;
impl<'a, REG> Cf1ce2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison with bit 2 of Control Field 1 is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cf1ce2::_0)
    }
    ///Comparison with bit 2 of Control Field 1 is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cf1ce2::_1)
    }
}
/**Control Field 1 Bit 3 Compare Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cf1ce3 {
    ///0: Comparison with bit 3 of Control Field 1 is disabled.
    _0 = 0,
    ///1: Comparison with bit 3 of Control Field 1 is enabled.
    _1 = 1,
}
impl From<Cf1ce3> for bool {
    #[inline(always)]
    fn from(variant: Cf1ce3) -> Self {
        variant as u8 != 0
    }
}
///Field `CF1CE3` reader - Control Field 1 Bit 3 Compare Enable
pub type Cf1ce3R = crate::BitReader<Cf1ce3>;
impl Cf1ce3R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cf1ce3 {
        match self.bits {
            false => Cf1ce3::_0,
            true => Cf1ce3::_1,
        }
    }
    ///Comparison with bit 3 of Control Field 1 is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cf1ce3::_0
    }
    ///Comparison with bit 3 of Control Field 1 is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cf1ce3::_1
    }
}
///Field `CF1CE3` writer - Control Field 1 Bit 3 Compare Enable
pub type Cf1ce3W<'a, REG> = crate::BitWriter<'a, REG, Cf1ce3>;
impl<'a, REG> Cf1ce3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison with bit 3 of Control Field 1 is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cf1ce3::_0)
    }
    ///Comparison with bit 3 of Control Field 1 is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cf1ce3::_1)
    }
}
/**Control Field 1 Bit 4 Compare Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cf1ce4 {
    ///0: Comparison with bit 4 of Control Field 1 is disabled.
    _0 = 0,
    ///1: Comparison with bit 4 of Control Field 1 is enabled.
    _1 = 1,
}
impl From<Cf1ce4> for bool {
    #[inline(always)]
    fn from(variant: Cf1ce4) -> Self {
        variant as u8 != 0
    }
}
///Field `CF1CE4` reader - Control Field 1 Bit 4 Compare Enable
pub type Cf1ce4R = crate::BitReader<Cf1ce4>;
impl Cf1ce4R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cf1ce4 {
        match self.bits {
            false => Cf1ce4::_0,
            true => Cf1ce4::_1,
        }
    }
    ///Comparison with bit 4 of Control Field 1 is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cf1ce4::_0
    }
    ///Comparison with bit 4 of Control Field 1 is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cf1ce4::_1
    }
}
///Field `CF1CE4` writer - Control Field 1 Bit 4 Compare Enable
pub type Cf1ce4W<'a, REG> = crate::BitWriter<'a, REG, Cf1ce4>;
impl<'a, REG> Cf1ce4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison with bit 4 of Control Field 1 is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cf1ce4::_0)
    }
    ///Comparison with bit 4 of Control Field 1 is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cf1ce4::_1)
    }
}
/**Control Field 1 Bit 5 Compare Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cf1ce5 {
    ///0: Comparison with bit 5 of Control Field 1 is disabled.
    _0 = 0,
    ///1: Comparison with bit 5 of Control Field 1 is enabled.
    _1 = 1,
}
impl From<Cf1ce5> for bool {
    #[inline(always)]
    fn from(variant: Cf1ce5) -> Self {
        variant as u8 != 0
    }
}
///Field `CF1CE5` reader - Control Field 1 Bit 5 Compare Enable
pub type Cf1ce5R = crate::BitReader<Cf1ce5>;
impl Cf1ce5R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cf1ce5 {
        match self.bits {
            false => Cf1ce5::_0,
            true => Cf1ce5::_1,
        }
    }
    ///Comparison with bit 5 of Control Field 1 is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cf1ce5::_0
    }
    ///Comparison with bit 5 of Control Field 1 is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cf1ce5::_1
    }
}
///Field `CF1CE5` writer - Control Field 1 Bit 5 Compare Enable
pub type Cf1ce5W<'a, REG> = crate::BitWriter<'a, REG, Cf1ce5>;
impl<'a, REG> Cf1ce5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison with bit 5 of Control Field 1 is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cf1ce5::_0)
    }
    ///Comparison with bit 5 of Control Field 1 is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cf1ce5::_1)
    }
}
/**Control Field 1 Bit 6 Compare Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cf1ce6 {
    ///0: Comparison with bit 6 of Control Field 1 is disabled.
    _0 = 0,
    ///1: Comparison with bit 6 of Control Field 1 is enabled.
    _1 = 1,
}
impl From<Cf1ce6> for bool {
    #[inline(always)]
    fn from(variant: Cf1ce6) -> Self {
        variant as u8 != 0
    }
}
///Field `CF1CE6` reader - Control Field 1 Bit 6 Compare Enable
pub type Cf1ce6R = crate::BitReader<Cf1ce6>;
impl Cf1ce6R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cf1ce6 {
        match self.bits {
            false => Cf1ce6::_0,
            true => Cf1ce6::_1,
        }
    }
    ///Comparison with bit 6 of Control Field 1 is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cf1ce6::_0
    }
    ///Comparison with bit 6 of Control Field 1 is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cf1ce6::_1
    }
}
///Field `CF1CE6` writer - Control Field 1 Bit 6 Compare Enable
pub type Cf1ce6W<'a, REG> = crate::BitWriter<'a, REG, Cf1ce6>;
impl<'a, REG> Cf1ce6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison with bit 6 of Control Field 1 is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cf1ce6::_0)
    }
    ///Comparison with bit 6 of Control Field 1 is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cf1ce6::_1)
    }
}
/**Control Field 1 Bit 7 Compare Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cf1ce7 {
    ///0: Comparison with bit 7 of Control Field 1 is disabled.
    _0 = 0,
    ///1: Comparison with bit 7 of Control Field 1 is enabled.
    _1 = 1,
}
impl From<Cf1ce7> for bool {
    #[inline(always)]
    fn from(variant: Cf1ce7) -> Self {
        variant as u8 != 0
    }
}
///Field `CF1CE7` reader - Control Field 1 Bit 7 Compare Enable
pub type Cf1ce7R = crate::BitReader<Cf1ce7>;
impl Cf1ce7R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cf1ce7 {
        match self.bits {
            false => Cf1ce7::_0,
            true => Cf1ce7::_1,
        }
    }
    ///Comparison with bit 7 of Control Field 1 is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cf1ce7::_0
    }
    ///Comparison with bit 7 of Control Field 1 is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cf1ce7::_1
    }
}
///Field `CF1CE7` writer - Control Field 1 Bit 7 Compare Enable
pub type Cf1ce7W<'a, REG> = crate::BitWriter<'a, REG, Cf1ce7>;
impl<'a, REG> Cf1ce7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison with bit 7 of Control Field 1 is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cf1ce7::_0)
    }
    ///Comparison with bit 7 of Control Field 1 is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cf1ce7::_1)
    }
}
impl R {
    ///Bit 0 - Control Field 1 Bit 0 Compare Enable
    #[inline(always)]
    pub fn cf1ce0(&self) -> Cf1ce0R {
        Cf1ce0R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Control Field 1 Bit 1 Compare Enable
    #[inline(always)]
    pub fn cf1ce1(&self) -> Cf1ce1R {
        Cf1ce1R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Control Field 1 Bit 2 Compare Enable
    #[inline(always)]
    pub fn cf1ce2(&self) -> Cf1ce2R {
        Cf1ce2R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Control Field 1 Bit 3 Compare Enable
    #[inline(always)]
    pub fn cf1ce3(&self) -> Cf1ce3R {
        Cf1ce3R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Control Field 1 Bit 4 Compare Enable
    #[inline(always)]
    pub fn cf1ce4(&self) -> Cf1ce4R {
        Cf1ce4R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Control Field 1 Bit 5 Compare Enable
    #[inline(always)]
    pub fn cf1ce5(&self) -> Cf1ce5R {
        Cf1ce5R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Control Field 1 Bit 6 Compare Enable
    #[inline(always)]
    pub fn cf1ce6(&self) -> Cf1ce6R {
        Cf1ce6R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Control Field 1 Bit 7 Compare Enable
    #[inline(always)]
    pub fn cf1ce7(&self) -> Cf1ce7R {
        Cf1ce7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CF1CR")
            .field("cf1ce0", &self.cf1ce0())
            .field("cf1ce1", &self.cf1ce1())
            .field("cf1ce2", &self.cf1ce2())
            .field("cf1ce3", &self.cf1ce3())
            .field("cf1ce4", &self.cf1ce4())
            .field("cf1ce5", &self.cf1ce5())
            .field("cf1ce6", &self.cf1ce6())
            .field("cf1ce7", &self.cf1ce7())
            .finish()
    }
}
impl W {
    ///Bit 0 - Control Field 1 Bit 0 Compare Enable
    #[inline(always)]
    pub fn cf1ce0(&mut self) -> Cf1ce0W<Cf1crSpec> {
        Cf1ce0W::new(self, 0)
    }
    ///Bit 1 - Control Field 1 Bit 1 Compare Enable
    #[inline(always)]
    pub fn cf1ce1(&mut self) -> Cf1ce1W<Cf1crSpec> {
        Cf1ce1W::new(self, 1)
    }
    ///Bit 2 - Control Field 1 Bit 2 Compare Enable
    #[inline(always)]
    pub fn cf1ce2(&mut self) -> Cf1ce2W<Cf1crSpec> {
        Cf1ce2W::new(self, 2)
    }
    ///Bit 3 - Control Field 1 Bit 3 Compare Enable
    #[inline(always)]
    pub fn cf1ce3(&mut self) -> Cf1ce3W<Cf1crSpec> {
        Cf1ce3W::new(self, 3)
    }
    ///Bit 4 - Control Field 1 Bit 4 Compare Enable
    #[inline(always)]
    pub fn cf1ce4(&mut self) -> Cf1ce4W<Cf1crSpec> {
        Cf1ce4W::new(self, 4)
    }
    ///Bit 5 - Control Field 1 Bit 5 Compare Enable
    #[inline(always)]
    pub fn cf1ce5(&mut self) -> Cf1ce5W<Cf1crSpec> {
        Cf1ce5W::new(self, 5)
    }
    ///Bit 6 - Control Field 1 Bit 6 Compare Enable
    #[inline(always)]
    pub fn cf1ce6(&mut self) -> Cf1ce6W<Cf1crSpec> {
        Cf1ce6W::new(self, 6)
    }
    ///Bit 7 - Control Field 1 Bit 7 Compare Enable
    #[inline(always)]
    pub fn cf1ce7(&mut self) -> Cf1ce7W<Cf1crSpec> {
        Cf1ce7W::new(self, 7)
    }
}
/**Control Field 1 Compare Enable Register

You can [`read`](crate::Reg::read) this register and get [`cf1cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cf1cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Cf1crSpec;
impl crate::RegisterSpec for Cf1crSpec {
    type Ux = u8;
}
///`read()` method returns [`cf1cr::R`](R) reader structure
impl crate::Readable for Cf1crSpec {}
///`write(|w| ..)` method takes [`cf1cr::W`](W) writer structure
impl crate::Writable for Cf1crSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CF1CR to value 0
impl crate::Resettable for Cf1crSpec {}
