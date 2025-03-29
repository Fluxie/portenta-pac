///Register `DPSIEGR0` reader
pub type R = crate::R<Dpsiegr0Spec>;
///Register `DPSIEGR0` writer
pub type W = crate::W<Dpsiegr0Spec>;
/**IRQ0-DS Pin Edge Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dirq0eg {
    ///0: A cancel request is generated at a falling edge
    _0 = 0,
    ///1: A cancel request is generated at a rising edge
    _1 = 1,
}
impl From<Dirq0eg> for bool {
    #[inline(always)]
    fn from(variant: Dirq0eg) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ0EG` reader - IRQ0-DS Pin Edge Select
pub type Dirq0egR = crate::BitReader<Dirq0eg>;
impl Dirq0egR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dirq0eg {
        match self.bits {
            false => Dirq0eg::_0,
            true => Dirq0eg::_1,
        }
    }
    ///A cancel request is generated at a falling edge
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dirq0eg::_0
    }
    ///A cancel request is generated at a rising edge
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dirq0eg::_1
    }
}
///Field `DIRQ0EG` writer - IRQ0-DS Pin Edge Select
pub type Dirq0egW<'a, REG> = crate::BitWriter<'a, REG, Dirq0eg>;
impl<'a, REG> Dirq0egW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A cancel request is generated at a falling edge
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq0eg::_0)
    }
    ///A cancel request is generated at a rising edge
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq0eg::_1)
    }
}
/**IRQ1-DS Pin Edge Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dirq1eg {
    ///0: A cancel request is generated at a falling edge
    _0 = 0,
    ///1: A cancel request is generated at a rising edge
    _1 = 1,
}
impl From<Dirq1eg> for bool {
    #[inline(always)]
    fn from(variant: Dirq1eg) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ1EG` reader - IRQ1-DS Pin Edge Select
pub type Dirq1egR = crate::BitReader<Dirq1eg>;
impl Dirq1egR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dirq1eg {
        match self.bits {
            false => Dirq1eg::_0,
            true => Dirq1eg::_1,
        }
    }
    ///A cancel request is generated at a falling edge
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dirq1eg::_0
    }
    ///A cancel request is generated at a rising edge
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dirq1eg::_1
    }
}
///Field `DIRQ1EG` writer - IRQ1-DS Pin Edge Select
pub type Dirq1egW<'a, REG> = crate::BitWriter<'a, REG, Dirq1eg>;
impl<'a, REG> Dirq1egW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A cancel request is generated at a falling edge
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq1eg::_0)
    }
    ///A cancel request is generated at a rising edge
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq1eg::_1)
    }
}
/**IRQ2-DS Pin Edge Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dirq2eg {
    ///0: A cancel request is generated at a falling edge
    _0 = 0,
    ///1: A cancel request is generated at a rising edge
    _1 = 1,
}
impl From<Dirq2eg> for bool {
    #[inline(always)]
    fn from(variant: Dirq2eg) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ2EG` reader - IRQ2-DS Pin Edge Select
pub type Dirq2egR = crate::BitReader<Dirq2eg>;
impl Dirq2egR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dirq2eg {
        match self.bits {
            false => Dirq2eg::_0,
            true => Dirq2eg::_1,
        }
    }
    ///A cancel request is generated at a falling edge
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dirq2eg::_0
    }
    ///A cancel request is generated at a rising edge
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dirq2eg::_1
    }
}
///Field `DIRQ2EG` writer - IRQ2-DS Pin Edge Select
pub type Dirq2egW<'a, REG> = crate::BitWriter<'a, REG, Dirq2eg>;
impl<'a, REG> Dirq2egW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A cancel request is generated at a falling edge
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq2eg::_0)
    }
    ///A cancel request is generated at a rising edge
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq2eg::_1)
    }
}
/**IRQ3-DS Pin Edge Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dirq3eg {
    ///0: A cancel request is generated at a falling edge
    _0 = 0,
    ///1: A cancel request is generated at a rising edge
    _1 = 1,
}
impl From<Dirq3eg> for bool {
    #[inline(always)]
    fn from(variant: Dirq3eg) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ3EG` reader - IRQ3-DS Pin Edge Select
pub type Dirq3egR = crate::BitReader<Dirq3eg>;
impl Dirq3egR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dirq3eg {
        match self.bits {
            false => Dirq3eg::_0,
            true => Dirq3eg::_1,
        }
    }
    ///A cancel request is generated at a falling edge
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dirq3eg::_0
    }
    ///A cancel request is generated at a rising edge
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dirq3eg::_1
    }
}
///Field `DIRQ3EG` writer - IRQ3-DS Pin Edge Select
pub type Dirq3egW<'a, REG> = crate::BitWriter<'a, REG, Dirq3eg>;
impl<'a, REG> Dirq3egW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A cancel request is generated at a falling edge
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq3eg::_0)
    }
    ///A cancel request is generated at a rising edge
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq3eg::_1)
    }
}
/**IRQ4-DS Pin Edge Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dirq4eg {
    ///0: A cancel request is generated at a falling edge
    _0 = 0,
    ///1: A cancel request is generated at a rising edge
    _1 = 1,
}
impl From<Dirq4eg> for bool {
    #[inline(always)]
    fn from(variant: Dirq4eg) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ4EG` reader - IRQ4-DS Pin Edge Select
pub type Dirq4egR = crate::BitReader<Dirq4eg>;
impl Dirq4egR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dirq4eg {
        match self.bits {
            false => Dirq4eg::_0,
            true => Dirq4eg::_1,
        }
    }
    ///A cancel request is generated at a falling edge
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dirq4eg::_0
    }
    ///A cancel request is generated at a rising edge
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dirq4eg::_1
    }
}
///Field `DIRQ4EG` writer - IRQ4-DS Pin Edge Select
pub type Dirq4egW<'a, REG> = crate::BitWriter<'a, REG, Dirq4eg>;
impl<'a, REG> Dirq4egW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A cancel request is generated at a falling edge
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq4eg::_0)
    }
    ///A cancel request is generated at a rising edge
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq4eg::_1)
    }
}
/**IRQ5-DS Pin Edge Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dirq5eg {
    ///0: A cancel request is generated at a falling edge
    _0 = 0,
    ///1: A cancel request is generated at a rising edge
    _1 = 1,
}
impl From<Dirq5eg> for bool {
    #[inline(always)]
    fn from(variant: Dirq5eg) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ5EG` reader - IRQ5-DS Pin Edge Select
pub type Dirq5egR = crate::BitReader<Dirq5eg>;
impl Dirq5egR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dirq5eg {
        match self.bits {
            false => Dirq5eg::_0,
            true => Dirq5eg::_1,
        }
    }
    ///A cancel request is generated at a falling edge
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dirq5eg::_0
    }
    ///A cancel request is generated at a rising edge
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dirq5eg::_1
    }
}
///Field `DIRQ5EG` writer - IRQ5-DS Pin Edge Select
pub type Dirq5egW<'a, REG> = crate::BitWriter<'a, REG, Dirq5eg>;
impl<'a, REG> Dirq5egW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A cancel request is generated at a falling edge
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq5eg::_0)
    }
    ///A cancel request is generated at a rising edge
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq5eg::_1)
    }
}
/**IRQ6-DS Pin Edge Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dirq6eg {
    ///0: A cancel request is generated at a falling edge
    _0 = 0,
    ///1: A cancel request is generated at a rising edge
    _1 = 1,
}
impl From<Dirq6eg> for bool {
    #[inline(always)]
    fn from(variant: Dirq6eg) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ6EG` reader - IRQ6-DS Pin Edge Select
pub type Dirq6egR = crate::BitReader<Dirq6eg>;
impl Dirq6egR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dirq6eg {
        match self.bits {
            false => Dirq6eg::_0,
            true => Dirq6eg::_1,
        }
    }
    ///A cancel request is generated at a falling edge
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dirq6eg::_0
    }
    ///A cancel request is generated at a rising edge
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dirq6eg::_1
    }
}
///Field `DIRQ6EG` writer - IRQ6-DS Pin Edge Select
pub type Dirq6egW<'a, REG> = crate::BitWriter<'a, REG, Dirq6eg>;
impl<'a, REG> Dirq6egW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A cancel request is generated at a falling edge
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq6eg::_0)
    }
    ///A cancel request is generated at a rising edge
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq6eg::_1)
    }
}
/**IRQ7-DS Pin Edge Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dirq7eg {
    ///0: A cancel request is generated at a falling edge
    _0 = 0,
    ///1: A cancel request is generated at a rising edge
    _1 = 1,
}
impl From<Dirq7eg> for bool {
    #[inline(always)]
    fn from(variant: Dirq7eg) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ7EG` reader - IRQ7-DS Pin Edge Select
pub type Dirq7egR = crate::BitReader<Dirq7eg>;
impl Dirq7egR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dirq7eg {
        match self.bits {
            false => Dirq7eg::_0,
            true => Dirq7eg::_1,
        }
    }
    ///A cancel request is generated at a falling edge
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dirq7eg::_0
    }
    ///A cancel request is generated at a rising edge
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dirq7eg::_1
    }
}
///Field `DIRQ7EG` writer - IRQ7-DS Pin Edge Select
pub type Dirq7egW<'a, REG> = crate::BitWriter<'a, REG, Dirq7eg>;
impl<'a, REG> Dirq7egW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A cancel request is generated at a falling edge
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq7eg::_0)
    }
    ///A cancel request is generated at a rising edge
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq7eg::_1)
    }
}
impl R {
    ///Bit 0 - IRQ0-DS Pin Edge Select
    #[inline(always)]
    pub fn dirq0eg(&self) -> Dirq0egR {
        Dirq0egR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - IRQ1-DS Pin Edge Select
    #[inline(always)]
    pub fn dirq1eg(&self) -> Dirq1egR {
        Dirq1egR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - IRQ2-DS Pin Edge Select
    #[inline(always)]
    pub fn dirq2eg(&self) -> Dirq2egR {
        Dirq2egR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - IRQ3-DS Pin Edge Select
    #[inline(always)]
    pub fn dirq3eg(&self) -> Dirq3egR {
        Dirq3egR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IRQ4-DS Pin Edge Select
    #[inline(always)]
    pub fn dirq4eg(&self) -> Dirq4egR {
        Dirq4egR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - IRQ5-DS Pin Edge Select
    #[inline(always)]
    pub fn dirq5eg(&self) -> Dirq5egR {
        Dirq5egR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - IRQ6-DS Pin Edge Select
    #[inline(always)]
    pub fn dirq6eg(&self) -> Dirq6egR {
        Dirq6egR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - IRQ7-DS Pin Edge Select
    #[inline(always)]
    pub fn dirq7eg(&self) -> Dirq7egR {
        Dirq7egR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DPSIEGR0")
            .field("dirq0eg", &self.dirq0eg())
            .field("dirq1eg", &self.dirq1eg())
            .field("dirq2eg", &self.dirq2eg())
            .field("dirq3eg", &self.dirq3eg())
            .field("dirq4eg", &self.dirq4eg())
            .field("dirq5eg", &self.dirq5eg())
            .field("dirq6eg", &self.dirq6eg())
            .field("dirq7eg", &self.dirq7eg())
            .finish()
    }
}
impl W {
    ///Bit 0 - IRQ0-DS Pin Edge Select
    #[inline(always)]
    pub fn dirq0eg(&mut self) -> Dirq0egW<Dpsiegr0Spec> {
        Dirq0egW::new(self, 0)
    }
    ///Bit 1 - IRQ1-DS Pin Edge Select
    #[inline(always)]
    pub fn dirq1eg(&mut self) -> Dirq1egW<Dpsiegr0Spec> {
        Dirq1egW::new(self, 1)
    }
    ///Bit 2 - IRQ2-DS Pin Edge Select
    #[inline(always)]
    pub fn dirq2eg(&mut self) -> Dirq2egW<Dpsiegr0Spec> {
        Dirq2egW::new(self, 2)
    }
    ///Bit 3 - IRQ3-DS Pin Edge Select
    #[inline(always)]
    pub fn dirq3eg(&mut self) -> Dirq3egW<Dpsiegr0Spec> {
        Dirq3egW::new(self, 3)
    }
    ///Bit 4 - IRQ4-DS Pin Edge Select
    #[inline(always)]
    pub fn dirq4eg(&mut self) -> Dirq4egW<Dpsiegr0Spec> {
        Dirq4egW::new(self, 4)
    }
    ///Bit 5 - IRQ5-DS Pin Edge Select
    #[inline(always)]
    pub fn dirq5eg(&mut self) -> Dirq5egW<Dpsiegr0Spec> {
        Dirq5egW::new(self, 5)
    }
    ///Bit 6 - IRQ6-DS Pin Edge Select
    #[inline(always)]
    pub fn dirq6eg(&mut self) -> Dirq6egW<Dpsiegr0Spec> {
        Dirq6egW::new(self, 6)
    }
    ///Bit 7 - IRQ7-DS Pin Edge Select
    #[inline(always)]
    pub fn dirq7eg(&mut self) -> Dirq7egW<Dpsiegr0Spec> {
        Dirq7egW::new(self, 7)
    }
}
/**Deep Software Standby Interrupt Edge Register 0

You can [`read`](crate::Reg::read) this register and get [`dpsiegr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpsiegr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Dpsiegr0Spec;
impl crate::RegisterSpec for Dpsiegr0Spec {
    type Ux = u8;
}
///`read()` method returns [`dpsiegr0::R`](R) reader structure
impl crate::Readable for Dpsiegr0Spec {}
///`write(|w| ..)` method takes [`dpsiegr0::W`](W) writer structure
impl crate::Writable for Dpsiegr0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DPSIEGR0 to value 0
impl crate::Resettable for Dpsiegr0Spec {}
