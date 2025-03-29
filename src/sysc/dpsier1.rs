///Register `DPSIER1` reader
pub type R = crate::R<Dpsier1Spec>;
///Register `DPSIER1` writer
pub type W = crate::W<Dpsier1Spec>;
/**IRQ8-DS Pin Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dirq8e {
    ///0: Cancelling Deep Software Standby mode is disabled
    _0 = 0,
    ///1: Cancelling Deep Software Standby mode is enabled
    _1 = 1,
}
impl From<Dirq8e> for bool {
    #[inline(always)]
    fn from(variant: Dirq8e) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ8E` reader - IRQ8-DS Pin Enable
pub type Dirq8eR = crate::BitReader<Dirq8e>;
impl Dirq8eR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dirq8e {
        match self.bits {
            false => Dirq8e::_0,
            true => Dirq8e::_1,
        }
    }
    ///Cancelling Deep Software Standby mode is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dirq8e::_0
    }
    ///Cancelling Deep Software Standby mode is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dirq8e::_1
    }
}
///Field `DIRQ8E` writer - IRQ8-DS Pin Enable
pub type Dirq8eW<'a, REG> = crate::BitWriter<'a, REG, Dirq8e>;
impl<'a, REG> Dirq8eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancelling Deep Software Standby mode is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq8e::_0)
    }
    ///Cancelling Deep Software Standby mode is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq8e::_1)
    }
}
/**IRQ9-DS Pin Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dirq9e {
    ///0: Cancelling Deep Software Standby mode is disabled
    _0 = 0,
    ///1: Cancelling Deep Software Standby mode is enabled
    _1 = 1,
}
impl From<Dirq9e> for bool {
    #[inline(always)]
    fn from(variant: Dirq9e) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ9E` reader - IRQ9-DS Pin Enable
pub type Dirq9eR = crate::BitReader<Dirq9e>;
impl Dirq9eR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dirq9e {
        match self.bits {
            false => Dirq9e::_0,
            true => Dirq9e::_1,
        }
    }
    ///Cancelling Deep Software Standby mode is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dirq9e::_0
    }
    ///Cancelling Deep Software Standby mode is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dirq9e::_1
    }
}
///Field `DIRQ9E` writer - IRQ9-DS Pin Enable
pub type Dirq9eW<'a, REG> = crate::BitWriter<'a, REG, Dirq9e>;
impl<'a, REG> Dirq9eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancelling Deep Software Standby mode is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq9e::_0)
    }
    ///Cancelling Deep Software Standby mode is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq9e::_1)
    }
}
/**IRQ10-DS Pin Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dirq10e {
    ///0: Cancelling Deep Software Standby mode is disabled
    _0 = 0,
    ///1: Cancelling Deep Software Standby mode is enabled
    _1 = 1,
}
impl From<Dirq10e> for bool {
    #[inline(always)]
    fn from(variant: Dirq10e) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ10E` reader - IRQ10-DS Pin Enable
pub type Dirq10eR = crate::BitReader<Dirq10e>;
impl Dirq10eR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dirq10e {
        match self.bits {
            false => Dirq10e::_0,
            true => Dirq10e::_1,
        }
    }
    ///Cancelling Deep Software Standby mode is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dirq10e::_0
    }
    ///Cancelling Deep Software Standby mode is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dirq10e::_1
    }
}
///Field `DIRQ10E` writer - IRQ10-DS Pin Enable
pub type Dirq10eW<'a, REG> = crate::BitWriter<'a, REG, Dirq10e>;
impl<'a, REG> Dirq10eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancelling Deep Software Standby mode is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq10e::_0)
    }
    ///Cancelling Deep Software Standby mode is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq10e::_1)
    }
}
/**IRQ11-DS Pin Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dirq11e {
    ///0: Cancelling Deep Software Standby mode is disabled
    _0 = 0,
    ///1: Cancelling Deep Software Standby mode is enabled
    _1 = 1,
}
impl From<Dirq11e> for bool {
    #[inline(always)]
    fn from(variant: Dirq11e) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ11E` reader - IRQ11-DS Pin Enable
pub type Dirq11eR = crate::BitReader<Dirq11e>;
impl Dirq11eR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dirq11e {
        match self.bits {
            false => Dirq11e::_0,
            true => Dirq11e::_1,
        }
    }
    ///Cancelling Deep Software Standby mode is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dirq11e::_0
    }
    ///Cancelling Deep Software Standby mode is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dirq11e::_1
    }
}
///Field `DIRQ11E` writer - IRQ11-DS Pin Enable
pub type Dirq11eW<'a, REG> = crate::BitWriter<'a, REG, Dirq11e>;
impl<'a, REG> Dirq11eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancelling Deep Software Standby mode is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq11e::_0)
    }
    ///Cancelling Deep Software Standby mode is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq11e::_1)
    }
}
/**IRQ12-DS Pin Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dirq12e {
    ///0: Cancelling Deep Software Standby mode is disabled
    _0 = 0,
    ///1: Cancelling Deep Software Standby mode is enabled
    _1 = 1,
}
impl From<Dirq12e> for bool {
    #[inline(always)]
    fn from(variant: Dirq12e) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ12E` reader - IRQ12-DS Pin Enable
pub type Dirq12eR = crate::BitReader<Dirq12e>;
impl Dirq12eR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dirq12e {
        match self.bits {
            false => Dirq12e::_0,
            true => Dirq12e::_1,
        }
    }
    ///Cancelling Deep Software Standby mode is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dirq12e::_0
    }
    ///Cancelling Deep Software Standby mode is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dirq12e::_1
    }
}
///Field `DIRQ12E` writer - IRQ12-DS Pin Enable
pub type Dirq12eW<'a, REG> = crate::BitWriter<'a, REG, Dirq12e>;
impl<'a, REG> Dirq12eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancelling Deep Software Standby mode is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq12e::_0)
    }
    ///Cancelling Deep Software Standby mode is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq12e::_1)
    }
}
/**IRQ13-DS Pin Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dirq13e {
    ///0: Cancelling Deep Software Standby mode is disabled
    _0 = 0,
    ///1: Cancelling Deep Software Standby mode is enabled
    _1 = 1,
}
impl From<Dirq13e> for bool {
    #[inline(always)]
    fn from(variant: Dirq13e) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ13E` reader - IRQ13-DS Pin Enable
pub type Dirq13eR = crate::BitReader<Dirq13e>;
impl Dirq13eR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dirq13e {
        match self.bits {
            false => Dirq13e::_0,
            true => Dirq13e::_1,
        }
    }
    ///Cancelling Deep Software Standby mode is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dirq13e::_0
    }
    ///Cancelling Deep Software Standby mode is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dirq13e::_1
    }
}
///Field `DIRQ13E` writer - IRQ13-DS Pin Enable
pub type Dirq13eW<'a, REG> = crate::BitWriter<'a, REG, Dirq13e>;
impl<'a, REG> Dirq13eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancelling Deep Software Standby mode is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq13e::_0)
    }
    ///Cancelling Deep Software Standby mode is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq13e::_1)
    }
}
/**IRQ14-DS Pin Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dirq14e {
    ///0: Cancelling Deep Software Standby mode is disabled
    _0 = 0,
    ///1: Cancelling Deep Software Standby mode is enabled
    _1 = 1,
}
impl From<Dirq14e> for bool {
    #[inline(always)]
    fn from(variant: Dirq14e) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ14E` reader - IRQ14-DS Pin Enable
pub type Dirq14eR = crate::BitReader<Dirq14e>;
impl Dirq14eR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dirq14e {
        match self.bits {
            false => Dirq14e::_0,
            true => Dirq14e::_1,
        }
    }
    ///Cancelling Deep Software Standby mode is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dirq14e::_0
    }
    ///Cancelling Deep Software Standby mode is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dirq14e::_1
    }
}
///Field `DIRQ14E` writer - IRQ14-DS Pin Enable
pub type Dirq14eW<'a, REG> = crate::BitWriter<'a, REG, Dirq14e>;
impl<'a, REG> Dirq14eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancelling Deep Software Standby mode is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq14e::_0)
    }
    ///Cancelling Deep Software Standby mode is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq14e::_1)
    }
}
/**IRQ15-DS Pin Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dirq15e {
    ///0: Cancelling Deep Software Standby mode is disabled
    _0 = 0,
    ///1: Cancelling Deep Software Standby mode is enabled
    _1 = 1,
}
impl From<Dirq15e> for bool {
    #[inline(always)]
    fn from(variant: Dirq15e) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ15E` reader - IRQ15-DS Pin Enable
pub type Dirq15eR = crate::BitReader<Dirq15e>;
impl Dirq15eR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dirq15e {
        match self.bits {
            false => Dirq15e::_0,
            true => Dirq15e::_1,
        }
    }
    ///Cancelling Deep Software Standby mode is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dirq15e::_0
    }
    ///Cancelling Deep Software Standby mode is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dirq15e::_1
    }
}
///Field `DIRQ15E` writer - IRQ15-DS Pin Enable
pub type Dirq15eW<'a, REG> = crate::BitWriter<'a, REG, Dirq15e>;
impl<'a, REG> Dirq15eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancelling Deep Software Standby mode is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq15e::_0)
    }
    ///Cancelling Deep Software Standby mode is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq15e::_1)
    }
}
impl R {
    ///Bit 0 - IRQ8-DS Pin Enable
    #[inline(always)]
    pub fn dirq8e(&self) -> Dirq8eR {
        Dirq8eR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - IRQ9-DS Pin Enable
    #[inline(always)]
    pub fn dirq9e(&self) -> Dirq9eR {
        Dirq9eR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - IRQ10-DS Pin Enable
    #[inline(always)]
    pub fn dirq10e(&self) -> Dirq10eR {
        Dirq10eR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - IRQ11-DS Pin Enable
    #[inline(always)]
    pub fn dirq11e(&self) -> Dirq11eR {
        Dirq11eR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IRQ12-DS Pin Enable
    #[inline(always)]
    pub fn dirq12e(&self) -> Dirq12eR {
        Dirq12eR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - IRQ13-DS Pin Enable
    #[inline(always)]
    pub fn dirq13e(&self) -> Dirq13eR {
        Dirq13eR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - IRQ14-DS Pin Enable
    #[inline(always)]
    pub fn dirq14e(&self) -> Dirq14eR {
        Dirq14eR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - IRQ15-DS Pin Enable
    #[inline(always)]
    pub fn dirq15e(&self) -> Dirq15eR {
        Dirq15eR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DPSIER1")
            .field("dirq8e", &self.dirq8e())
            .field("dirq9e", &self.dirq9e())
            .field("dirq10e", &self.dirq10e())
            .field("dirq11e", &self.dirq11e())
            .field("dirq12e", &self.dirq12e())
            .field("dirq13e", &self.dirq13e())
            .field("dirq14e", &self.dirq14e())
            .field("dirq15e", &self.dirq15e())
            .finish()
    }
}
impl W {
    ///Bit 0 - IRQ8-DS Pin Enable
    #[inline(always)]
    pub fn dirq8e(&mut self) -> Dirq8eW<Dpsier1Spec> {
        Dirq8eW::new(self, 0)
    }
    ///Bit 1 - IRQ9-DS Pin Enable
    #[inline(always)]
    pub fn dirq9e(&mut self) -> Dirq9eW<Dpsier1Spec> {
        Dirq9eW::new(self, 1)
    }
    ///Bit 2 - IRQ10-DS Pin Enable
    #[inline(always)]
    pub fn dirq10e(&mut self) -> Dirq10eW<Dpsier1Spec> {
        Dirq10eW::new(self, 2)
    }
    ///Bit 3 - IRQ11-DS Pin Enable
    #[inline(always)]
    pub fn dirq11e(&mut self) -> Dirq11eW<Dpsier1Spec> {
        Dirq11eW::new(self, 3)
    }
    ///Bit 4 - IRQ12-DS Pin Enable
    #[inline(always)]
    pub fn dirq12e(&mut self) -> Dirq12eW<Dpsier1Spec> {
        Dirq12eW::new(self, 4)
    }
    ///Bit 5 - IRQ13-DS Pin Enable
    #[inline(always)]
    pub fn dirq13e(&mut self) -> Dirq13eW<Dpsier1Spec> {
        Dirq13eW::new(self, 5)
    }
    ///Bit 6 - IRQ14-DS Pin Enable
    #[inline(always)]
    pub fn dirq14e(&mut self) -> Dirq14eW<Dpsier1Spec> {
        Dirq14eW::new(self, 6)
    }
    ///Bit 7 - IRQ15-DS Pin Enable
    #[inline(always)]
    pub fn dirq15e(&mut self) -> Dirq15eW<Dpsier1Spec> {
        Dirq15eW::new(self, 7)
    }
}
/**Deep Software Standby Interrupt Enable Register 1

You can [`read`](crate::Reg::read) this register and get [`dpsier1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpsier1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Dpsier1Spec;
impl crate::RegisterSpec for Dpsier1Spec {
    type Ux = u8;
}
///`read()` method returns [`dpsier1::R`](R) reader structure
impl crate::Readable for Dpsier1Spec {}
///`write(|w| ..)` method takes [`dpsier1::W`](W) writer structure
impl crate::Writable for Dpsier1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DPSIER1 to value 0
impl crate::Resettable for Dpsier1Spec {}
