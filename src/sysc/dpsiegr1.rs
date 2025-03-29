///Register `DPSIEGR1` reader
pub type R = crate::R<Dpsiegr1Spec>;
///Register `DPSIEGR1` writer
pub type W = crate::W<Dpsiegr1Spec>;
/**IRQ8-DS Pin Edge Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dirq8eg {
    ///0: A cancel request is generated at a falling edge.
    _0 = 0,
    ///1: A cancel request is generated at a rising edge.
    _1 = 1,
}
impl From<Dirq8eg> for bool {
    #[inline(always)]
    fn from(variant: Dirq8eg) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ8EG` reader - IRQ8-DS Pin Edge Select
pub type Dirq8egR = crate::BitReader<Dirq8eg>;
impl Dirq8egR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dirq8eg {
        match self.bits {
            false => Dirq8eg::_0,
            true => Dirq8eg::_1,
        }
    }
    ///A cancel request is generated at a falling edge.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dirq8eg::_0
    }
    ///A cancel request is generated at a rising edge.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dirq8eg::_1
    }
}
///Field `DIRQ8EG` writer - IRQ8-DS Pin Edge Select
pub type Dirq8egW<'a, REG> = crate::BitWriter<'a, REG, Dirq8eg>;
impl<'a, REG> Dirq8egW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A cancel request is generated at a falling edge.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq8eg::_0)
    }
    ///A cancel request is generated at a rising edge.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq8eg::_1)
    }
}
/**IRQ9-DS Pin Edge Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dirq9eg {
    ///0: A cancel request is generated at a falling edge.
    _0 = 0,
    ///1: A cancel request is generated at a rising edge.
    _1 = 1,
}
impl From<Dirq9eg> for bool {
    #[inline(always)]
    fn from(variant: Dirq9eg) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ9EG` reader - IRQ9-DS Pin Edge Select
pub type Dirq9egR = crate::BitReader<Dirq9eg>;
impl Dirq9egR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dirq9eg {
        match self.bits {
            false => Dirq9eg::_0,
            true => Dirq9eg::_1,
        }
    }
    ///A cancel request is generated at a falling edge.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dirq9eg::_0
    }
    ///A cancel request is generated at a rising edge.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dirq9eg::_1
    }
}
///Field `DIRQ9EG` writer - IRQ9-DS Pin Edge Select
pub type Dirq9egW<'a, REG> = crate::BitWriter<'a, REG, Dirq9eg>;
impl<'a, REG> Dirq9egW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A cancel request is generated at a falling edge.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq9eg::_0)
    }
    ///A cancel request is generated at a rising edge.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq9eg::_1)
    }
}
/**IRQ10-DS Pin Edge Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dirq10eg {
    ///0: A cancel request is generated at a falling edge.
    _0 = 0,
    ///1: A cancel request is generated at a rising edge
    _1 = 1,
}
impl From<Dirq10eg> for bool {
    #[inline(always)]
    fn from(variant: Dirq10eg) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ10EG` reader - IRQ10-DS Pin Edge Select
pub type Dirq10egR = crate::BitReader<Dirq10eg>;
impl Dirq10egR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dirq10eg {
        match self.bits {
            false => Dirq10eg::_0,
            true => Dirq10eg::_1,
        }
    }
    ///A cancel request is generated at a falling edge.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dirq10eg::_0
    }
    ///A cancel request is generated at a rising edge
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dirq10eg::_1
    }
}
///Field `DIRQ10EG` writer - IRQ10-DS Pin Edge Select
pub type Dirq10egW<'a, REG> = crate::BitWriter<'a, REG, Dirq10eg>;
impl<'a, REG> Dirq10egW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A cancel request is generated at a falling edge.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq10eg::_0)
    }
    ///A cancel request is generated at a rising edge
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq10eg::_1)
    }
}
/**IRQ11-DS Pin Edge Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dirq11eg {
    ///0: A cancel request is generated at a falling edge.
    _0 = 0,
    ///1: A cancel request is generated at a rising edge.
    _1 = 1,
}
impl From<Dirq11eg> for bool {
    #[inline(always)]
    fn from(variant: Dirq11eg) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ11EG` reader - IRQ11-DS Pin Edge Select
pub type Dirq11egR = crate::BitReader<Dirq11eg>;
impl Dirq11egR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dirq11eg {
        match self.bits {
            false => Dirq11eg::_0,
            true => Dirq11eg::_1,
        }
    }
    ///A cancel request is generated at a falling edge.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dirq11eg::_0
    }
    ///A cancel request is generated at a rising edge.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dirq11eg::_1
    }
}
///Field `DIRQ11EG` writer - IRQ11-DS Pin Edge Select
pub type Dirq11egW<'a, REG> = crate::BitWriter<'a, REG, Dirq11eg>;
impl<'a, REG> Dirq11egW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A cancel request is generated at a falling edge.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq11eg::_0)
    }
    ///A cancel request is generated at a rising edge.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq11eg::_1)
    }
}
/**IRQ12-DS Pin Edge Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dirq12eg {
    ///0: A cancel request is generated at a falling edge.
    _0 = 0,
    ///1: A cancel request is generated at a rising edge.
    _1 = 1,
}
impl From<Dirq12eg> for bool {
    #[inline(always)]
    fn from(variant: Dirq12eg) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ12EG` reader - IRQ12-DS Pin Edge Select
pub type Dirq12egR = crate::BitReader<Dirq12eg>;
impl Dirq12egR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dirq12eg {
        match self.bits {
            false => Dirq12eg::_0,
            true => Dirq12eg::_1,
        }
    }
    ///A cancel request is generated at a falling edge.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dirq12eg::_0
    }
    ///A cancel request is generated at a rising edge.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dirq12eg::_1
    }
}
///Field `DIRQ12EG` writer - IRQ12-DS Pin Edge Select
pub type Dirq12egW<'a, REG> = crate::BitWriter<'a, REG, Dirq12eg>;
impl<'a, REG> Dirq12egW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A cancel request is generated at a falling edge.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq12eg::_0)
    }
    ///A cancel request is generated at a rising edge.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq12eg::_1)
    }
}
/**IRQ13-DS Pin Edge Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dirq13eg {
    ///0: A cancel request is generated at a falling edge.
    _0 = 0,
    ///1: A cancel request is generated at a rising edge.
    _1 = 1,
}
impl From<Dirq13eg> for bool {
    #[inline(always)]
    fn from(variant: Dirq13eg) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ13EG` reader - IRQ13-DS Pin Edge Select
pub type Dirq13egR = crate::BitReader<Dirq13eg>;
impl Dirq13egR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dirq13eg {
        match self.bits {
            false => Dirq13eg::_0,
            true => Dirq13eg::_1,
        }
    }
    ///A cancel request is generated at a falling edge.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dirq13eg::_0
    }
    ///A cancel request is generated at a rising edge.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dirq13eg::_1
    }
}
///Field `DIRQ13EG` writer - IRQ13-DS Pin Edge Select
pub type Dirq13egW<'a, REG> = crate::BitWriter<'a, REG, Dirq13eg>;
impl<'a, REG> Dirq13egW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A cancel request is generated at a falling edge.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq13eg::_0)
    }
    ///A cancel request is generated at a rising edge.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq13eg::_1)
    }
}
/**IRQ14-DS Pin Edge Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dirq14eg {
    ///0: A cancel request is generated at a falling edge.
    _0 = 0,
    ///1: A cancel request is generated at a rising edge.
    _1 = 1,
}
impl From<Dirq14eg> for bool {
    #[inline(always)]
    fn from(variant: Dirq14eg) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ14EG` reader - IRQ14-DS Pin Edge Select
pub type Dirq14egR = crate::BitReader<Dirq14eg>;
impl Dirq14egR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dirq14eg {
        match self.bits {
            false => Dirq14eg::_0,
            true => Dirq14eg::_1,
        }
    }
    ///A cancel request is generated at a falling edge.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dirq14eg::_0
    }
    ///A cancel request is generated at a rising edge.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dirq14eg::_1
    }
}
///Field `DIRQ14EG` writer - IRQ14-DS Pin Edge Select
pub type Dirq14egW<'a, REG> = crate::BitWriter<'a, REG, Dirq14eg>;
impl<'a, REG> Dirq14egW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A cancel request is generated at a falling edge.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq14eg::_0)
    }
    ///A cancel request is generated at a rising edge.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq14eg::_1)
    }
}
/**IRQ15-DS Pin Edge Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dirq15eg {
    ///0: A cancel request is generated at a falling edge.
    _0 = 0,
    ///1: A cancel request is generated at a rising edge.
    _1 = 1,
}
impl From<Dirq15eg> for bool {
    #[inline(always)]
    fn from(variant: Dirq15eg) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ15EG` reader - IRQ15-DS Pin Edge Select
pub type Dirq15egR = crate::BitReader<Dirq15eg>;
impl Dirq15egR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dirq15eg {
        match self.bits {
            false => Dirq15eg::_0,
            true => Dirq15eg::_1,
        }
    }
    ///A cancel request is generated at a falling edge.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dirq15eg::_0
    }
    ///A cancel request is generated at a rising edge.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dirq15eg::_1
    }
}
///Field `DIRQ15EG` writer - IRQ15-DS Pin Edge Select
pub type Dirq15egW<'a, REG> = crate::BitWriter<'a, REG, Dirq15eg>;
impl<'a, REG> Dirq15egW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A cancel request is generated at a falling edge.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq15eg::_0)
    }
    ///A cancel request is generated at a rising edge.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq15eg::_1)
    }
}
impl R {
    ///Bit 0 - IRQ8-DS Pin Edge Select
    #[inline(always)]
    pub fn dirq8eg(&self) -> Dirq8egR {
        Dirq8egR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - IRQ9-DS Pin Edge Select
    #[inline(always)]
    pub fn dirq9eg(&self) -> Dirq9egR {
        Dirq9egR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - IRQ10-DS Pin Edge Select
    #[inline(always)]
    pub fn dirq10eg(&self) -> Dirq10egR {
        Dirq10egR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - IRQ11-DS Pin Edge Select
    #[inline(always)]
    pub fn dirq11eg(&self) -> Dirq11egR {
        Dirq11egR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IRQ12-DS Pin Edge Select
    #[inline(always)]
    pub fn dirq12eg(&self) -> Dirq12egR {
        Dirq12egR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - IRQ13-DS Pin Edge Select
    #[inline(always)]
    pub fn dirq13eg(&self) -> Dirq13egR {
        Dirq13egR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - IRQ14-DS Pin Edge Select
    #[inline(always)]
    pub fn dirq14eg(&self) -> Dirq14egR {
        Dirq14egR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - IRQ15-DS Pin Edge Select
    #[inline(always)]
    pub fn dirq15eg(&self) -> Dirq15egR {
        Dirq15egR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DPSIEGR1")
            .field("dirq8eg", &self.dirq8eg())
            .field("dirq9eg", &self.dirq9eg())
            .field("dirq10eg", &self.dirq10eg())
            .field("dirq11eg", &self.dirq11eg())
            .field("dirq12eg", &self.dirq12eg())
            .field("dirq13eg", &self.dirq13eg())
            .field("dirq14eg", &self.dirq14eg())
            .field("dirq15eg", &self.dirq15eg())
            .finish()
    }
}
impl W {
    ///Bit 0 - IRQ8-DS Pin Edge Select
    #[inline(always)]
    pub fn dirq8eg(&mut self) -> Dirq8egW<Dpsiegr1Spec> {
        Dirq8egW::new(self, 0)
    }
    ///Bit 1 - IRQ9-DS Pin Edge Select
    #[inline(always)]
    pub fn dirq9eg(&mut self) -> Dirq9egW<Dpsiegr1Spec> {
        Dirq9egW::new(self, 1)
    }
    ///Bit 2 - IRQ10-DS Pin Edge Select
    #[inline(always)]
    pub fn dirq10eg(&mut self) -> Dirq10egW<Dpsiegr1Spec> {
        Dirq10egW::new(self, 2)
    }
    ///Bit 3 - IRQ11-DS Pin Edge Select
    #[inline(always)]
    pub fn dirq11eg(&mut self) -> Dirq11egW<Dpsiegr1Spec> {
        Dirq11egW::new(self, 3)
    }
    ///Bit 4 - IRQ12-DS Pin Edge Select
    #[inline(always)]
    pub fn dirq12eg(&mut self) -> Dirq12egW<Dpsiegr1Spec> {
        Dirq12egW::new(self, 4)
    }
    ///Bit 5 - IRQ13-DS Pin Edge Select
    #[inline(always)]
    pub fn dirq13eg(&mut self) -> Dirq13egW<Dpsiegr1Spec> {
        Dirq13egW::new(self, 5)
    }
    ///Bit 6 - IRQ14-DS Pin Edge Select
    #[inline(always)]
    pub fn dirq14eg(&mut self) -> Dirq14egW<Dpsiegr1Spec> {
        Dirq14egW::new(self, 6)
    }
    ///Bit 7 - IRQ15-DS Pin Edge Select
    #[inline(always)]
    pub fn dirq15eg(&mut self) -> Dirq15egW<Dpsiegr1Spec> {
        Dirq15egW::new(self, 7)
    }
}
/**Deep Software Standby Interrupt Edge Register 1

You can [`read`](crate::Reg::read) this register and get [`dpsiegr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpsiegr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Dpsiegr1Spec;
impl crate::RegisterSpec for Dpsiegr1Spec {
    type Ux = u8;
}
///`read()` method returns [`dpsiegr1::R`](R) reader structure
impl crate::Readable for Dpsiegr1Spec {}
///`write(|w| ..)` method takes [`dpsiegr1::W`](W) writer structure
impl crate::Writable for Dpsiegr1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DPSIEGR1 to value 0
impl crate::Resettable for Dpsiegr1Spec {}
