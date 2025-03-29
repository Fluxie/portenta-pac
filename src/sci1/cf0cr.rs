///Register `CF0CR` reader
pub type R = crate::R<Cf0crSpec>;
///Register `CF0CR` writer
pub type W = crate::W<Cf0crSpec>;
/**Control Field 0 Bit 0 Compare Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cf0ce0 {
    ///0: Comparison with bit 0 of Control Field 0 is disabled.
    _0 = 0,
    ///1: Comparison with bit 0 of Control Field 0 is enabled.
    _1 = 1,
}
impl From<Cf0ce0> for bool {
    #[inline(always)]
    fn from(variant: Cf0ce0) -> Self {
        variant as u8 != 0
    }
}
///Field `CF0CE0` reader - Control Field 0 Bit 0 Compare Enable
pub type Cf0ce0R = crate::BitReader<Cf0ce0>;
impl Cf0ce0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cf0ce0 {
        match self.bits {
            false => Cf0ce0::_0,
            true => Cf0ce0::_1,
        }
    }
    ///Comparison with bit 0 of Control Field 0 is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cf0ce0::_0
    }
    ///Comparison with bit 0 of Control Field 0 is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cf0ce0::_1
    }
}
///Field `CF0CE0` writer - Control Field 0 Bit 0 Compare Enable
pub type Cf0ce0W<'a, REG> = crate::BitWriter<'a, REG, Cf0ce0>;
impl<'a, REG> Cf0ce0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison with bit 0 of Control Field 0 is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cf0ce0::_0)
    }
    ///Comparison with bit 0 of Control Field 0 is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cf0ce0::_1)
    }
}
/**Control Field 1 Bit 0 Compare Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cf0ce1 {
    ///0: Comparison with bit 1 of Control Field 0 is disabled.
    _0 = 0,
    ///1: Comparison with bit 1 of Control Field 0 is enabled.
    _1 = 1,
}
impl From<Cf0ce1> for bool {
    #[inline(always)]
    fn from(variant: Cf0ce1) -> Self {
        variant as u8 != 0
    }
}
///Field `CF0CE1` reader - Control Field 1 Bit 0 Compare Enable
pub type Cf0ce1R = crate::BitReader<Cf0ce1>;
impl Cf0ce1R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cf0ce1 {
        match self.bits {
            false => Cf0ce1::_0,
            true => Cf0ce1::_1,
        }
    }
    ///Comparison with bit 1 of Control Field 0 is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cf0ce1::_0
    }
    ///Comparison with bit 1 of Control Field 0 is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cf0ce1::_1
    }
}
///Field `CF0CE1` writer - Control Field 1 Bit 0 Compare Enable
pub type Cf0ce1W<'a, REG> = crate::BitWriter<'a, REG, Cf0ce1>;
impl<'a, REG> Cf0ce1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison with bit 1 of Control Field 0 is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cf0ce1::_0)
    }
    ///Comparison with bit 1 of Control Field 0 is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cf0ce1::_1)
    }
}
/**Control Field 2 Bit 0 Compare Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cf0ce2 {
    ///0: Comparison with bit 2 of Control Field 0 is disabled.
    _0 = 0,
    ///1: Comparison with bit 2 of Control Field 0 is enabled.
    _1 = 1,
}
impl From<Cf0ce2> for bool {
    #[inline(always)]
    fn from(variant: Cf0ce2) -> Self {
        variant as u8 != 0
    }
}
///Field `CF0CE2` reader - Control Field 2 Bit 0 Compare Enable
pub type Cf0ce2R = crate::BitReader<Cf0ce2>;
impl Cf0ce2R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cf0ce2 {
        match self.bits {
            false => Cf0ce2::_0,
            true => Cf0ce2::_1,
        }
    }
    ///Comparison with bit 2 of Control Field 0 is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cf0ce2::_0
    }
    ///Comparison with bit 2 of Control Field 0 is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cf0ce2::_1
    }
}
///Field `CF0CE2` writer - Control Field 2 Bit 0 Compare Enable
pub type Cf0ce2W<'a, REG> = crate::BitWriter<'a, REG, Cf0ce2>;
impl<'a, REG> Cf0ce2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison with bit 2 of Control Field 0 is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cf0ce2::_0)
    }
    ///Comparison with bit 2 of Control Field 0 is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cf0ce2::_1)
    }
}
/**Control Field 3 Bit 0 Compare Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cf0ce3 {
    ///0: Comparison with bit 3 of Control Field 0 is disabled.
    _0 = 0,
    ///1: Comparison with bit 3 of Control Field 0 is enabled.
    _1 = 1,
}
impl From<Cf0ce3> for bool {
    #[inline(always)]
    fn from(variant: Cf0ce3) -> Self {
        variant as u8 != 0
    }
}
///Field `CF0CE3` reader - Control Field 3 Bit 0 Compare Enable
pub type Cf0ce3R = crate::BitReader<Cf0ce3>;
impl Cf0ce3R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cf0ce3 {
        match self.bits {
            false => Cf0ce3::_0,
            true => Cf0ce3::_1,
        }
    }
    ///Comparison with bit 3 of Control Field 0 is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cf0ce3::_0
    }
    ///Comparison with bit 3 of Control Field 0 is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cf0ce3::_1
    }
}
///Field `CF0CE3` writer - Control Field 3 Bit 0 Compare Enable
pub type Cf0ce3W<'a, REG> = crate::BitWriter<'a, REG, Cf0ce3>;
impl<'a, REG> Cf0ce3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison with bit 3 of Control Field 0 is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cf0ce3::_0)
    }
    ///Comparison with bit 3 of Control Field 0 is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cf0ce3::_1)
    }
}
/**Control Field 4 Bit 0 Compare Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cf0ce4 {
    ///0: Comparison with bit 4 of Control Field 0 is disabled.
    _0 = 0,
    ///1: Comparison with bit 4 of Control Field 0 is enabled.
    _1 = 1,
}
impl From<Cf0ce4> for bool {
    #[inline(always)]
    fn from(variant: Cf0ce4) -> Self {
        variant as u8 != 0
    }
}
///Field `CF0CE4` reader - Control Field 4 Bit 0 Compare Enable
pub type Cf0ce4R = crate::BitReader<Cf0ce4>;
impl Cf0ce4R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cf0ce4 {
        match self.bits {
            false => Cf0ce4::_0,
            true => Cf0ce4::_1,
        }
    }
    ///Comparison with bit 4 of Control Field 0 is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cf0ce4::_0
    }
    ///Comparison with bit 4 of Control Field 0 is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cf0ce4::_1
    }
}
///Field `CF0CE4` writer - Control Field 4 Bit 0 Compare Enable
pub type Cf0ce4W<'a, REG> = crate::BitWriter<'a, REG, Cf0ce4>;
impl<'a, REG> Cf0ce4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison with bit 4 of Control Field 0 is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cf0ce4::_0)
    }
    ///Comparison with bit 4 of Control Field 0 is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cf0ce4::_1)
    }
}
/**Control Field 5 Bit 0 Compare Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cf0ce5 {
    ///0: Comparison with bit 5 of Control Field 0 is disabled.
    _0 = 0,
    ///1: Comparison with bit 5 of Control Field 0 is enabled.
    _1 = 1,
}
impl From<Cf0ce5> for bool {
    #[inline(always)]
    fn from(variant: Cf0ce5) -> Self {
        variant as u8 != 0
    }
}
///Field `CF0CE5` reader - Control Field 5 Bit 0 Compare Enable
pub type Cf0ce5R = crate::BitReader<Cf0ce5>;
impl Cf0ce5R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cf0ce5 {
        match self.bits {
            false => Cf0ce5::_0,
            true => Cf0ce5::_1,
        }
    }
    ///Comparison with bit 5 of Control Field 0 is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cf0ce5::_0
    }
    ///Comparison with bit 5 of Control Field 0 is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cf0ce5::_1
    }
}
///Field `CF0CE5` writer - Control Field 5 Bit 0 Compare Enable
pub type Cf0ce5W<'a, REG> = crate::BitWriter<'a, REG, Cf0ce5>;
impl<'a, REG> Cf0ce5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison with bit 5 of Control Field 0 is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cf0ce5::_0)
    }
    ///Comparison with bit 5 of Control Field 0 is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cf0ce5::_1)
    }
}
/**Control Field 6 Bit 0 Compare Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cf0ce6 {
    ///0: Comparison with bit 6 of Control Field 0 is disabled.
    _0 = 0,
    ///1: Comparison with bit 6 of Control Field 0 is enabled.
    _1 = 1,
}
impl From<Cf0ce6> for bool {
    #[inline(always)]
    fn from(variant: Cf0ce6) -> Self {
        variant as u8 != 0
    }
}
///Field `CF0CE6` reader - Control Field 6 Bit 0 Compare Enable
pub type Cf0ce6R = crate::BitReader<Cf0ce6>;
impl Cf0ce6R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cf0ce6 {
        match self.bits {
            false => Cf0ce6::_0,
            true => Cf0ce6::_1,
        }
    }
    ///Comparison with bit 6 of Control Field 0 is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cf0ce6::_0
    }
    ///Comparison with bit 6 of Control Field 0 is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cf0ce6::_1
    }
}
///Field `CF0CE6` writer - Control Field 6 Bit 0 Compare Enable
pub type Cf0ce6W<'a, REG> = crate::BitWriter<'a, REG, Cf0ce6>;
impl<'a, REG> Cf0ce6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison with bit 6 of Control Field 0 is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cf0ce6::_0)
    }
    ///Comparison with bit 6 of Control Field 0 is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cf0ce6::_1)
    }
}
/**Control Field 7 Bit 0 Compare Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cf0ce7 {
    ///0: Comparison with bit 7 of Control Field 0 is disabled.
    _0 = 0,
    ///1: Comparison with bit 7 of Control Field 0 is enabled.
    _1 = 1,
}
impl From<Cf0ce7> for bool {
    #[inline(always)]
    fn from(variant: Cf0ce7) -> Self {
        variant as u8 != 0
    }
}
///Field `CF0CE7` reader - Control Field 7 Bit 0 Compare Enable
pub type Cf0ce7R = crate::BitReader<Cf0ce7>;
impl Cf0ce7R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cf0ce7 {
        match self.bits {
            false => Cf0ce7::_0,
            true => Cf0ce7::_1,
        }
    }
    ///Comparison with bit 7 of Control Field 0 is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cf0ce7::_0
    }
    ///Comparison with bit 7 of Control Field 0 is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cf0ce7::_1
    }
}
///Field `CF0CE7` writer - Control Field 7 Bit 0 Compare Enable
pub type Cf0ce7W<'a, REG> = crate::BitWriter<'a, REG, Cf0ce7>;
impl<'a, REG> Cf0ce7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison with bit 7 of Control Field 0 is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cf0ce7::_0)
    }
    ///Comparison with bit 7 of Control Field 0 is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cf0ce7::_1)
    }
}
impl R {
    ///Bit 0 - Control Field 0 Bit 0 Compare Enable
    #[inline(always)]
    pub fn cf0ce0(&self) -> Cf0ce0R {
        Cf0ce0R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Control Field 1 Bit 0 Compare Enable
    #[inline(always)]
    pub fn cf0ce1(&self) -> Cf0ce1R {
        Cf0ce1R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Control Field 2 Bit 0 Compare Enable
    #[inline(always)]
    pub fn cf0ce2(&self) -> Cf0ce2R {
        Cf0ce2R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Control Field 3 Bit 0 Compare Enable
    #[inline(always)]
    pub fn cf0ce3(&self) -> Cf0ce3R {
        Cf0ce3R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Control Field 4 Bit 0 Compare Enable
    #[inline(always)]
    pub fn cf0ce4(&self) -> Cf0ce4R {
        Cf0ce4R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Control Field 5 Bit 0 Compare Enable
    #[inline(always)]
    pub fn cf0ce5(&self) -> Cf0ce5R {
        Cf0ce5R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Control Field 6 Bit 0 Compare Enable
    #[inline(always)]
    pub fn cf0ce6(&self) -> Cf0ce6R {
        Cf0ce6R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Control Field 7 Bit 0 Compare Enable
    #[inline(always)]
    pub fn cf0ce7(&self) -> Cf0ce7R {
        Cf0ce7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CF0CR")
            .field("cf0ce0", &self.cf0ce0())
            .field("cf0ce1", &self.cf0ce1())
            .field("cf0ce2", &self.cf0ce2())
            .field("cf0ce3", &self.cf0ce3())
            .field("cf0ce4", &self.cf0ce4())
            .field("cf0ce5", &self.cf0ce5())
            .field("cf0ce6", &self.cf0ce6())
            .field("cf0ce7", &self.cf0ce7())
            .finish()
    }
}
impl W {
    ///Bit 0 - Control Field 0 Bit 0 Compare Enable
    #[inline(always)]
    pub fn cf0ce0(&mut self) -> Cf0ce0W<Cf0crSpec> {
        Cf0ce0W::new(self, 0)
    }
    ///Bit 1 - Control Field 1 Bit 0 Compare Enable
    #[inline(always)]
    pub fn cf0ce1(&mut self) -> Cf0ce1W<Cf0crSpec> {
        Cf0ce1W::new(self, 1)
    }
    ///Bit 2 - Control Field 2 Bit 0 Compare Enable
    #[inline(always)]
    pub fn cf0ce2(&mut self) -> Cf0ce2W<Cf0crSpec> {
        Cf0ce2W::new(self, 2)
    }
    ///Bit 3 - Control Field 3 Bit 0 Compare Enable
    #[inline(always)]
    pub fn cf0ce3(&mut self) -> Cf0ce3W<Cf0crSpec> {
        Cf0ce3W::new(self, 3)
    }
    ///Bit 4 - Control Field 4 Bit 0 Compare Enable
    #[inline(always)]
    pub fn cf0ce4(&mut self) -> Cf0ce4W<Cf0crSpec> {
        Cf0ce4W::new(self, 4)
    }
    ///Bit 5 - Control Field 5 Bit 0 Compare Enable
    #[inline(always)]
    pub fn cf0ce5(&mut self) -> Cf0ce5W<Cf0crSpec> {
        Cf0ce5W::new(self, 5)
    }
    ///Bit 6 - Control Field 6 Bit 0 Compare Enable
    #[inline(always)]
    pub fn cf0ce6(&mut self) -> Cf0ce6W<Cf0crSpec> {
        Cf0ce6W::new(self, 6)
    }
    ///Bit 7 - Control Field 7 Bit 0 Compare Enable
    #[inline(always)]
    pub fn cf0ce7(&mut self) -> Cf0ce7W<Cf0crSpec> {
        Cf0ce7W::new(self, 7)
    }
}
/**Control Field 0 Compare Enable Register

You can [`read`](crate::Reg::read) this register and get [`cf0cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cf0cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Cf0crSpec;
impl crate::RegisterSpec for Cf0crSpec {
    type Ux = u8;
}
///`read()` method returns [`cf0cr::R`](R) reader structure
impl crate::Readable for Cf0crSpec {}
///`write(|w| ..)` method takes [`cf0cr::W`](W) writer structure
impl crate::Writable for Cf0crSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CF0CR to value 0
impl crate::Resettable for Cf0crSpec {}
