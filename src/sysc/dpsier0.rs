///Register `DPSIER0` reader
pub type R = crate::R<Dpsier0Spec>;
///Register `DPSIER0` writer
pub type W = crate::W<Dpsier0Spec>;
/**IRQ0-DS Pin Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dirq0e {
    ///0: Cancelling Deep Software Standby mode is disabled
    _0 = 0,
    ///1: Cancelling Deep Software Standby mode is enabled
    _1 = 1,
}
impl From<Dirq0e> for bool {
    #[inline(always)]
    fn from(variant: Dirq0e) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ0E` reader - IRQ0-DS Pin Enable
pub type Dirq0eR = crate::BitReader<Dirq0e>;
impl Dirq0eR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dirq0e {
        match self.bits {
            false => Dirq0e::_0,
            true => Dirq0e::_1,
        }
    }
    ///Cancelling Deep Software Standby mode is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dirq0e::_0
    }
    ///Cancelling Deep Software Standby mode is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dirq0e::_1
    }
}
///Field `DIRQ0E` writer - IRQ0-DS Pin Enable
pub type Dirq0eW<'a, REG> = crate::BitWriter<'a, REG, Dirq0e>;
impl<'a, REG> Dirq0eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancelling Deep Software Standby mode is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq0e::_0)
    }
    ///Cancelling Deep Software Standby mode is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq0e::_1)
    }
}
/**IRQ1-DS Pin Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dirq1e {
    ///0: Cancelling Deep Software Standby mode is disabled
    _0 = 0,
    ///1: Cancelling Deep Software Standby mode is enabled
    _1 = 1,
}
impl From<Dirq1e> for bool {
    #[inline(always)]
    fn from(variant: Dirq1e) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ1E` reader - IRQ1-DS Pin Enable
pub type Dirq1eR = crate::BitReader<Dirq1e>;
impl Dirq1eR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dirq1e {
        match self.bits {
            false => Dirq1e::_0,
            true => Dirq1e::_1,
        }
    }
    ///Cancelling Deep Software Standby mode is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dirq1e::_0
    }
    ///Cancelling Deep Software Standby mode is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dirq1e::_1
    }
}
///Field `DIRQ1E` writer - IRQ1-DS Pin Enable
pub type Dirq1eW<'a, REG> = crate::BitWriter<'a, REG, Dirq1e>;
impl<'a, REG> Dirq1eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancelling Deep Software Standby mode is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq1e::_0)
    }
    ///Cancelling Deep Software Standby mode is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq1e::_1)
    }
}
/**IRQ2-DS Pin Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dirq2e {
    ///0: Cancelling Deep Software Standby mode is disabled
    _0 = 0,
    ///1: Cancelling Deep Software Standby mode is enabled
    _1 = 1,
}
impl From<Dirq2e> for bool {
    #[inline(always)]
    fn from(variant: Dirq2e) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ2E` reader - IRQ2-DS Pin Enable
pub type Dirq2eR = crate::BitReader<Dirq2e>;
impl Dirq2eR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dirq2e {
        match self.bits {
            false => Dirq2e::_0,
            true => Dirq2e::_1,
        }
    }
    ///Cancelling Deep Software Standby mode is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dirq2e::_0
    }
    ///Cancelling Deep Software Standby mode is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dirq2e::_1
    }
}
///Field `DIRQ2E` writer - IRQ2-DS Pin Enable
pub type Dirq2eW<'a, REG> = crate::BitWriter<'a, REG, Dirq2e>;
impl<'a, REG> Dirq2eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancelling Deep Software Standby mode is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq2e::_0)
    }
    ///Cancelling Deep Software Standby mode is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq2e::_1)
    }
}
/**IRQ3-DS Pin Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dirq3e {
    ///0: Cancelling Deep Software Standby mode is disabled
    _0 = 0,
    ///1: Cancelling Deep Software Standby mode is enabled
    _1 = 1,
}
impl From<Dirq3e> for bool {
    #[inline(always)]
    fn from(variant: Dirq3e) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ3E` reader - IRQ3-DS Pin Enable
pub type Dirq3eR = crate::BitReader<Dirq3e>;
impl Dirq3eR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dirq3e {
        match self.bits {
            false => Dirq3e::_0,
            true => Dirq3e::_1,
        }
    }
    ///Cancelling Deep Software Standby mode is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dirq3e::_0
    }
    ///Cancelling Deep Software Standby mode is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dirq3e::_1
    }
}
///Field `DIRQ3E` writer - IRQ3-DS Pin Enable
pub type Dirq3eW<'a, REG> = crate::BitWriter<'a, REG, Dirq3e>;
impl<'a, REG> Dirq3eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancelling Deep Software Standby mode is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq3e::_0)
    }
    ///Cancelling Deep Software Standby mode is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq3e::_1)
    }
}
/**IRQ4-DS Pin Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dirq4e {
    ///0: Cancelling Deep Software Standby mode is disabled
    _0 = 0,
    ///1: Cancelling Deep Software Standby mode is enabled
    _1 = 1,
}
impl From<Dirq4e> for bool {
    #[inline(always)]
    fn from(variant: Dirq4e) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ4E` reader - IRQ4-DS Pin Enable
pub type Dirq4eR = crate::BitReader<Dirq4e>;
impl Dirq4eR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dirq4e {
        match self.bits {
            false => Dirq4e::_0,
            true => Dirq4e::_1,
        }
    }
    ///Cancelling Deep Software Standby mode is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dirq4e::_0
    }
    ///Cancelling Deep Software Standby mode is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dirq4e::_1
    }
}
///Field `DIRQ4E` writer - IRQ4-DS Pin Enable
pub type Dirq4eW<'a, REG> = crate::BitWriter<'a, REG, Dirq4e>;
impl<'a, REG> Dirq4eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancelling Deep Software Standby mode is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq4e::_0)
    }
    ///Cancelling Deep Software Standby mode is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq4e::_1)
    }
}
/**IRQ5-DS Pin Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dirq5e {
    ///0: Cancelling Deep Software Standby mode is disabled
    _0 = 0,
    ///1: Cancelling Deep Software Standby mode is enabled
    _1 = 1,
}
impl From<Dirq5e> for bool {
    #[inline(always)]
    fn from(variant: Dirq5e) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ5E` reader - IRQ5-DS Pin Enable
pub type Dirq5eR = crate::BitReader<Dirq5e>;
impl Dirq5eR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dirq5e {
        match self.bits {
            false => Dirq5e::_0,
            true => Dirq5e::_1,
        }
    }
    ///Cancelling Deep Software Standby mode is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dirq5e::_0
    }
    ///Cancelling Deep Software Standby mode is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dirq5e::_1
    }
}
///Field `DIRQ5E` writer - IRQ5-DS Pin Enable
pub type Dirq5eW<'a, REG> = crate::BitWriter<'a, REG, Dirq5e>;
impl<'a, REG> Dirq5eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancelling Deep Software Standby mode is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq5e::_0)
    }
    ///Cancelling Deep Software Standby mode is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq5e::_1)
    }
}
/**IRQ6-DS Pin Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dirq6e {
    ///0: Cancelling Deep Software Standby mode is disabled
    _0 = 0,
    ///1: Cancelling Deep Software Standby mode is enabled
    _1 = 1,
}
impl From<Dirq6e> for bool {
    #[inline(always)]
    fn from(variant: Dirq6e) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ6E` reader - IRQ6-DS Pin Enable
pub type Dirq6eR = crate::BitReader<Dirq6e>;
impl Dirq6eR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dirq6e {
        match self.bits {
            false => Dirq6e::_0,
            true => Dirq6e::_1,
        }
    }
    ///Cancelling Deep Software Standby mode is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dirq6e::_0
    }
    ///Cancelling Deep Software Standby mode is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dirq6e::_1
    }
}
///Field `DIRQ6E` writer - IRQ6-DS Pin Enable
pub type Dirq6eW<'a, REG> = crate::BitWriter<'a, REG, Dirq6e>;
impl<'a, REG> Dirq6eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancelling Deep Software Standby mode is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq6e::_0)
    }
    ///Cancelling Deep Software Standby mode is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq6e::_1)
    }
}
/**IRQ7-DS Pin Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dirq7e {
    ///0: Cancelling Deep Software Standby mode is disabled
    _0 = 0,
    ///1: Cancelling Deep Software Standby mode is enabled
    _1 = 1,
}
impl From<Dirq7e> for bool {
    #[inline(always)]
    fn from(variant: Dirq7e) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ7E` reader - IRQ7-DS Pin Enable
pub type Dirq7eR = crate::BitReader<Dirq7e>;
impl Dirq7eR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dirq7e {
        match self.bits {
            false => Dirq7e::_0,
            true => Dirq7e::_1,
        }
    }
    ///Cancelling Deep Software Standby mode is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dirq7e::_0
    }
    ///Cancelling Deep Software Standby mode is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dirq7e::_1
    }
}
///Field `DIRQ7E` writer - IRQ7-DS Pin Enable
pub type Dirq7eW<'a, REG> = crate::BitWriter<'a, REG, Dirq7e>;
impl<'a, REG> Dirq7eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancelling Deep Software Standby mode is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq7e::_0)
    }
    ///Cancelling Deep Software Standby mode is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq7e::_1)
    }
}
impl R {
    ///Bit 0 - IRQ0-DS Pin Enable
    #[inline(always)]
    pub fn dirq0e(&self) -> Dirq0eR {
        Dirq0eR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - IRQ1-DS Pin Enable
    #[inline(always)]
    pub fn dirq1e(&self) -> Dirq1eR {
        Dirq1eR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - IRQ2-DS Pin Enable
    #[inline(always)]
    pub fn dirq2e(&self) -> Dirq2eR {
        Dirq2eR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - IRQ3-DS Pin Enable
    #[inline(always)]
    pub fn dirq3e(&self) -> Dirq3eR {
        Dirq3eR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IRQ4-DS Pin Enable
    #[inline(always)]
    pub fn dirq4e(&self) -> Dirq4eR {
        Dirq4eR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - IRQ5-DS Pin Enable
    #[inline(always)]
    pub fn dirq5e(&self) -> Dirq5eR {
        Dirq5eR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - IRQ6-DS Pin Enable
    #[inline(always)]
    pub fn dirq6e(&self) -> Dirq6eR {
        Dirq6eR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - IRQ7-DS Pin Enable
    #[inline(always)]
    pub fn dirq7e(&self) -> Dirq7eR {
        Dirq7eR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DPSIER0")
            .field("dirq0e", &self.dirq0e())
            .field("dirq1e", &self.dirq1e())
            .field("dirq2e", &self.dirq2e())
            .field("dirq3e", &self.dirq3e())
            .field("dirq4e", &self.dirq4e())
            .field("dirq5e", &self.dirq5e())
            .field("dirq6e", &self.dirq6e())
            .field("dirq7e", &self.dirq7e())
            .finish()
    }
}
impl W {
    ///Bit 0 - IRQ0-DS Pin Enable
    #[inline(always)]
    pub fn dirq0e(&mut self) -> Dirq0eW<Dpsier0Spec> {
        Dirq0eW::new(self, 0)
    }
    ///Bit 1 - IRQ1-DS Pin Enable
    #[inline(always)]
    pub fn dirq1e(&mut self) -> Dirq1eW<Dpsier0Spec> {
        Dirq1eW::new(self, 1)
    }
    ///Bit 2 - IRQ2-DS Pin Enable
    #[inline(always)]
    pub fn dirq2e(&mut self) -> Dirq2eW<Dpsier0Spec> {
        Dirq2eW::new(self, 2)
    }
    ///Bit 3 - IRQ3-DS Pin Enable
    #[inline(always)]
    pub fn dirq3e(&mut self) -> Dirq3eW<Dpsier0Spec> {
        Dirq3eW::new(self, 3)
    }
    ///Bit 4 - IRQ4-DS Pin Enable
    #[inline(always)]
    pub fn dirq4e(&mut self) -> Dirq4eW<Dpsier0Spec> {
        Dirq4eW::new(self, 4)
    }
    ///Bit 5 - IRQ5-DS Pin Enable
    #[inline(always)]
    pub fn dirq5e(&mut self) -> Dirq5eW<Dpsier0Spec> {
        Dirq5eW::new(self, 5)
    }
    ///Bit 6 - IRQ6-DS Pin Enable
    #[inline(always)]
    pub fn dirq6e(&mut self) -> Dirq6eW<Dpsier0Spec> {
        Dirq6eW::new(self, 6)
    }
    ///Bit 7 - IRQ7-DS Pin Enable
    #[inline(always)]
    pub fn dirq7e(&mut self) -> Dirq7eW<Dpsier0Spec> {
        Dirq7eW::new(self, 7)
    }
}
/**Deep Software Standby Interrupt Enable Register 0

You can [`read`](crate::Reg::read) this register and get [`dpsier0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpsier0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Dpsier0Spec;
impl crate::RegisterSpec for Dpsier0Spec {
    type Ux = u8;
}
///`read()` method returns [`dpsier0::R`](R) reader structure
impl crate::Readable for Dpsier0Spec {}
///`write(|w| ..)` method takes [`dpsier0::W`](W) writer structure
impl crate::Writable for Dpsier0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DPSIER0 to value 0
impl crate::Resettable for Dpsier0Spec {}
