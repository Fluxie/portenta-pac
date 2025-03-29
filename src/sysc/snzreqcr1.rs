///Register `SNZREQCR1` reader
pub type R = crate::R<Snzreqcr1Spec>;
///Register `SNZREQCR1` writer
pub type W = crate::W<Snzreqcr1Spec>;
/**Enable AGT3 underflow snooze request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snzreqen0 {
    ///0: Disable the snooze request
    _0 = 0,
    ///1: Enable the snooze request
    _1 = 1,
}
impl From<Snzreqen0> for bool {
    #[inline(always)]
    fn from(variant: Snzreqen0) -> Self {
        variant as u8 != 0
    }
}
///Field `SNZREQEN0` reader - Enable AGT3 underflow snooze request
pub type Snzreqen0R = crate::BitReader<Snzreqen0>;
impl Snzreqen0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Snzreqen0 {
        match self.bits {
            false => Snzreqen0::_0,
            true => Snzreqen0::_1,
        }
    }
    ///Disable the snooze request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Snzreqen0::_0
    }
    ///Enable the snooze request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Snzreqen0::_1
    }
}
///Field `SNZREQEN0` writer - Enable AGT3 underflow snooze request
pub type Snzreqen0W<'a, REG> = crate::BitWriter<'a, REG, Snzreqen0>;
impl<'a, REG> Snzreqen0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable the snooze request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen0::_0)
    }
    ///Enable the snooze request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen0::_1)
    }
}
/**Enable AGT3 compare match A snooze request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snzreqen1 {
    ///0: Disable the snooze request
    _0 = 0,
    ///1: Enable the snooze request
    _1 = 1,
}
impl From<Snzreqen1> for bool {
    #[inline(always)]
    fn from(variant: Snzreqen1) -> Self {
        variant as u8 != 0
    }
}
///Field `SNZREQEN1` reader - Enable AGT3 compare match A snooze request
pub type Snzreqen1R = crate::BitReader<Snzreqen1>;
impl Snzreqen1R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Snzreqen1 {
        match self.bits {
            false => Snzreqen1::_0,
            true => Snzreqen1::_1,
        }
    }
    ///Disable the snooze request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Snzreqen1::_0
    }
    ///Enable the snooze request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Snzreqen1::_1
    }
}
///Field `SNZREQEN1` writer - Enable AGT3 compare match A snooze request
pub type Snzreqen1W<'a, REG> = crate::BitWriter<'a, REG, Snzreqen1>;
impl<'a, REG> Snzreqen1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable the snooze request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen1::_0)
    }
    ///Enable the snooze request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen1::_1)
    }
}
/**Enable AGT3 compare match B snooze request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snzreqen2 {
    ///0: Disable the snooze request
    _0 = 0,
    ///1: Enable the snooze request
    _1 = 1,
}
impl From<Snzreqen2> for bool {
    #[inline(always)]
    fn from(variant: Snzreqen2) -> Self {
        variant as u8 != 0
    }
}
///Field `SNZREQEN2` reader - Enable AGT3 compare match B snooze request
pub type Snzreqen2R = crate::BitReader<Snzreqen2>;
impl Snzreqen2R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Snzreqen2 {
        match self.bits {
            false => Snzreqen2::_0,
            true => Snzreqen2::_1,
        }
    }
    ///Disable the snooze request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Snzreqen2::_0
    }
    ///Enable the snooze request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Snzreqen2::_1
    }
}
///Field `SNZREQEN2` writer - Enable AGT3 compare match B snooze request
pub type Snzreqen2W<'a, REG> = crate::BitWriter<'a, REG, Snzreqen2>;
impl<'a, REG> Snzreqen2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable the snooze request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen2::_0)
    }
    ///Enable the snooze request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen2::_1)
    }
}
impl R {
    ///Bit 0 - Enable AGT3 underflow snooze request
    #[inline(always)]
    pub fn snzreqen0(&self) -> Snzreqen0R {
        Snzreqen0R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Enable AGT3 compare match A snooze request
    #[inline(always)]
    pub fn snzreqen1(&self) -> Snzreqen1R {
        Snzreqen1R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Enable AGT3 compare match B snooze request
    #[inline(always)]
    pub fn snzreqen2(&self) -> Snzreqen2R {
        Snzreqen2R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SNZREQCR1")
            .field("snzreqen0", &self.snzreqen0())
            .field("snzreqen1", &self.snzreqen1())
            .field("snzreqen2", &self.snzreqen2())
            .finish()
    }
}
impl W {
    ///Bit 0 - Enable AGT3 underflow snooze request
    #[inline(always)]
    pub fn snzreqen0(&mut self) -> Snzreqen0W<Snzreqcr1Spec> {
        Snzreqen0W::new(self, 0)
    }
    ///Bit 1 - Enable AGT3 compare match A snooze request
    #[inline(always)]
    pub fn snzreqen1(&mut self) -> Snzreqen1W<Snzreqcr1Spec> {
        Snzreqen1W::new(self, 1)
    }
    ///Bit 2 - Enable AGT3 compare match B snooze request
    #[inline(always)]
    pub fn snzreqen2(&mut self) -> Snzreqen2W<Snzreqcr1Spec> {
        Snzreqen2W::new(self, 2)
    }
}
/**Snooze Request Control Register 1

You can [`read`](crate::Reg::read) this register and get [`snzreqcr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snzreqcr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Snzreqcr1Spec;
impl crate::RegisterSpec for Snzreqcr1Spec {
    type Ux = u32;
}
///`read()` method returns [`snzreqcr1::R`](R) reader structure
impl crate::Readable for Snzreqcr1Spec {}
///`write(|w| ..)` method takes [`snzreqcr1::W`](W) writer structure
impl crate::Writable for Snzreqcr1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SNZREQCR1 to value 0
impl crate::Resettable for Snzreqcr1Spec {}
