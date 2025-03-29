///Register `DPSIFR1` reader
pub type R = crate::R<Dpsifr1Spec>;
///Register `DPSIFR1` writer
pub type W = crate::W<Dpsifr1Spec>;
/**IRQ8-DS Pin Deep Software Standby Cancel Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dirq8f {
    ///0: The cancel request is not generated
    _0 = 0,
    ///1: The cancel request is generated
    _1 = 1,
}
impl From<Dirq8f> for bool {
    #[inline(always)]
    fn from(variant: Dirq8f) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ8F` reader - IRQ8-DS Pin Deep Software Standby Cancel Flag
pub type Dirq8fR = crate::BitReader<Dirq8f>;
impl Dirq8fR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dirq8f {
        match self.bits {
            false => Dirq8f::_0,
            true => Dirq8f::_1,
        }
    }
    ///The cancel request is not generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dirq8f::_0
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dirq8f::_1
    }
}
///Field `DIRQ8F` writer - IRQ8-DS Pin Deep Software Standby Cancel Flag
pub type Dirq8fW<'a, REG> = crate::BitWriter<'a, REG, Dirq8f>;
impl<'a, REG> Dirq8fW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The cancel request is not generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq8f::_0)
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq8f::_1)
    }
}
/**IRQ9-DS Pin Deep Software Standby Cancel Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dirq9f {
    ///0: The cancel request is not generated
    _0 = 0,
    ///1: The cancel request is generated
    _1 = 1,
}
impl From<Dirq9f> for bool {
    #[inline(always)]
    fn from(variant: Dirq9f) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ9F` reader - IRQ9-DS Pin Deep Software Standby Cancel Flag
pub type Dirq9fR = crate::BitReader<Dirq9f>;
impl Dirq9fR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dirq9f {
        match self.bits {
            false => Dirq9f::_0,
            true => Dirq9f::_1,
        }
    }
    ///The cancel request is not generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dirq9f::_0
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dirq9f::_1
    }
}
///Field `DIRQ9F` writer - IRQ9-DS Pin Deep Software Standby Cancel Flag
pub type Dirq9fW<'a, REG> = crate::BitWriter<'a, REG, Dirq9f>;
impl<'a, REG> Dirq9fW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The cancel request is not generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq9f::_0)
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq9f::_1)
    }
}
/**IRQ10-DS Pin Deep Software Standby Cancel Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dirq10f {
    ///0: The cancel request is not generated
    _0 = 0,
    ///1: The cancel request is generated
    _1 = 1,
}
impl From<Dirq10f> for bool {
    #[inline(always)]
    fn from(variant: Dirq10f) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ10F` reader - IRQ10-DS Pin Deep Software Standby Cancel Flag
pub type Dirq10fR = crate::BitReader<Dirq10f>;
impl Dirq10fR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dirq10f {
        match self.bits {
            false => Dirq10f::_0,
            true => Dirq10f::_1,
        }
    }
    ///The cancel request is not generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dirq10f::_0
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dirq10f::_1
    }
}
///Field `DIRQ10F` writer - IRQ10-DS Pin Deep Software Standby Cancel Flag
pub type Dirq10fW<'a, REG> = crate::BitWriter<'a, REG, Dirq10f>;
impl<'a, REG> Dirq10fW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The cancel request is not generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq10f::_0)
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq10f::_1)
    }
}
/**IRQ11-DS Pin Deep Software Standby Cancel Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dirq11f {
    ///0: The cancel request is not generated
    _0 = 0,
    ///1: The cancel request is generated
    _1 = 1,
}
impl From<Dirq11f> for bool {
    #[inline(always)]
    fn from(variant: Dirq11f) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ11F` reader - IRQ11-DS Pin Deep Software Standby Cancel Flag
pub type Dirq11fR = crate::BitReader<Dirq11f>;
impl Dirq11fR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dirq11f {
        match self.bits {
            false => Dirq11f::_0,
            true => Dirq11f::_1,
        }
    }
    ///The cancel request is not generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dirq11f::_0
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dirq11f::_1
    }
}
///Field `DIRQ11F` writer - IRQ11-DS Pin Deep Software Standby Cancel Flag
pub type Dirq11fW<'a, REG> = crate::BitWriter<'a, REG, Dirq11f>;
impl<'a, REG> Dirq11fW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The cancel request is not generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq11f::_0)
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq11f::_1)
    }
}
/**IRQ12-DS Pin Deep Software Standby Cancel Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dirq12f {
    ///0: The cancel request is not generated
    _0 = 0,
    ///1: The cancel request is generated
    _1 = 1,
}
impl From<Dirq12f> for bool {
    #[inline(always)]
    fn from(variant: Dirq12f) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ12F` reader - IRQ12-DS Pin Deep Software Standby Cancel Flag
pub type Dirq12fR = crate::BitReader<Dirq12f>;
impl Dirq12fR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dirq12f {
        match self.bits {
            false => Dirq12f::_0,
            true => Dirq12f::_1,
        }
    }
    ///The cancel request is not generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dirq12f::_0
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dirq12f::_1
    }
}
///Field `DIRQ12F` writer - IRQ12-DS Pin Deep Software Standby Cancel Flag
pub type Dirq12fW<'a, REG> = crate::BitWriter<'a, REG, Dirq12f>;
impl<'a, REG> Dirq12fW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The cancel request is not generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq12f::_0)
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq12f::_1)
    }
}
/**IRQ13-DS Pin Deep Software Standby Cancel Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dirq13f {
    ///0: The cancel request is not generated
    _0 = 0,
    ///1: The cancel request is generated
    _1 = 1,
}
impl From<Dirq13f> for bool {
    #[inline(always)]
    fn from(variant: Dirq13f) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ13F` reader - IRQ13-DS Pin Deep Software Standby Cancel Flag
pub type Dirq13fR = crate::BitReader<Dirq13f>;
impl Dirq13fR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dirq13f {
        match self.bits {
            false => Dirq13f::_0,
            true => Dirq13f::_1,
        }
    }
    ///The cancel request is not generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dirq13f::_0
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dirq13f::_1
    }
}
///Field `DIRQ13F` writer - IRQ13-DS Pin Deep Software Standby Cancel Flag
pub type Dirq13fW<'a, REG> = crate::BitWriter<'a, REG, Dirq13f>;
impl<'a, REG> Dirq13fW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The cancel request is not generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq13f::_0)
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq13f::_1)
    }
}
/**IRQ14-DS Pin Deep Software Standby Cancel Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dirq14f {
    ///0: The cancel request is not generated
    _0 = 0,
    ///1: The cancel request is generated
    _1 = 1,
}
impl From<Dirq14f> for bool {
    #[inline(always)]
    fn from(variant: Dirq14f) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ14F` reader - IRQ14-DS Pin Deep Software Standby Cancel Flag
pub type Dirq14fR = crate::BitReader<Dirq14f>;
impl Dirq14fR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dirq14f {
        match self.bits {
            false => Dirq14f::_0,
            true => Dirq14f::_1,
        }
    }
    ///The cancel request is not generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dirq14f::_0
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dirq14f::_1
    }
}
///Field `DIRQ14F` writer - IRQ14-DS Pin Deep Software Standby Cancel Flag
pub type Dirq14fW<'a, REG> = crate::BitWriter<'a, REG, Dirq14f>;
impl<'a, REG> Dirq14fW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The cancel request is not generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq14f::_0)
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq14f::_1)
    }
}
/**IRQ15-DS Pin Deep Software Standby Cancel Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dirq15f {
    ///0: The cancel request is not generated
    _0 = 0,
    ///1: The cancel request is generated
    _1 = 1,
}
impl From<Dirq15f> for bool {
    #[inline(always)]
    fn from(variant: Dirq15f) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ15F` reader - IRQ15-DS Pin Deep Software Standby Cancel Flag
pub type Dirq15fR = crate::BitReader<Dirq15f>;
impl Dirq15fR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dirq15f {
        match self.bits {
            false => Dirq15f::_0,
            true => Dirq15f::_1,
        }
    }
    ///The cancel request is not generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dirq15f::_0
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dirq15f::_1
    }
}
///Field `DIRQ15F` writer - IRQ15-DS Pin Deep Software Standby Cancel Flag
pub type Dirq15fW<'a, REG> = crate::BitWriter<'a, REG, Dirq15f>;
impl<'a, REG> Dirq15fW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The cancel request is not generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq15f::_0)
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq15f::_1)
    }
}
impl R {
    ///Bit 0 - IRQ8-DS Pin Deep Software Standby Cancel Flag
    #[inline(always)]
    pub fn dirq8f(&self) -> Dirq8fR {
        Dirq8fR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - IRQ9-DS Pin Deep Software Standby Cancel Flag
    #[inline(always)]
    pub fn dirq9f(&self) -> Dirq9fR {
        Dirq9fR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - IRQ10-DS Pin Deep Software Standby Cancel Flag
    #[inline(always)]
    pub fn dirq10f(&self) -> Dirq10fR {
        Dirq10fR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - IRQ11-DS Pin Deep Software Standby Cancel Flag
    #[inline(always)]
    pub fn dirq11f(&self) -> Dirq11fR {
        Dirq11fR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IRQ12-DS Pin Deep Software Standby Cancel Flag
    #[inline(always)]
    pub fn dirq12f(&self) -> Dirq12fR {
        Dirq12fR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - IRQ13-DS Pin Deep Software Standby Cancel Flag
    #[inline(always)]
    pub fn dirq13f(&self) -> Dirq13fR {
        Dirq13fR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - IRQ14-DS Pin Deep Software Standby Cancel Flag
    #[inline(always)]
    pub fn dirq14f(&self) -> Dirq14fR {
        Dirq14fR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - IRQ15-DS Pin Deep Software Standby Cancel Flag
    #[inline(always)]
    pub fn dirq15f(&self) -> Dirq15fR {
        Dirq15fR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DPSIFR1")
            .field("dirq8f", &self.dirq8f())
            .field("dirq9f", &self.dirq9f())
            .field("dirq10f", &self.dirq10f())
            .field("dirq11f", &self.dirq11f())
            .field("dirq12f", &self.dirq12f())
            .field("dirq13f", &self.dirq13f())
            .field("dirq14f", &self.dirq14f())
            .field("dirq15f", &self.dirq15f())
            .finish()
    }
}
impl W {
    ///Bit 0 - IRQ8-DS Pin Deep Software Standby Cancel Flag
    #[inline(always)]
    pub fn dirq8f(&mut self) -> Dirq8fW<Dpsifr1Spec> {
        Dirq8fW::new(self, 0)
    }
    ///Bit 1 - IRQ9-DS Pin Deep Software Standby Cancel Flag
    #[inline(always)]
    pub fn dirq9f(&mut self) -> Dirq9fW<Dpsifr1Spec> {
        Dirq9fW::new(self, 1)
    }
    ///Bit 2 - IRQ10-DS Pin Deep Software Standby Cancel Flag
    #[inline(always)]
    pub fn dirq10f(&mut self) -> Dirq10fW<Dpsifr1Spec> {
        Dirq10fW::new(self, 2)
    }
    ///Bit 3 - IRQ11-DS Pin Deep Software Standby Cancel Flag
    #[inline(always)]
    pub fn dirq11f(&mut self) -> Dirq11fW<Dpsifr1Spec> {
        Dirq11fW::new(self, 3)
    }
    ///Bit 4 - IRQ12-DS Pin Deep Software Standby Cancel Flag
    #[inline(always)]
    pub fn dirq12f(&mut self) -> Dirq12fW<Dpsifr1Spec> {
        Dirq12fW::new(self, 4)
    }
    ///Bit 5 - IRQ13-DS Pin Deep Software Standby Cancel Flag
    #[inline(always)]
    pub fn dirq13f(&mut self) -> Dirq13fW<Dpsifr1Spec> {
        Dirq13fW::new(self, 5)
    }
    ///Bit 6 - IRQ14-DS Pin Deep Software Standby Cancel Flag
    #[inline(always)]
    pub fn dirq14f(&mut self) -> Dirq14fW<Dpsifr1Spec> {
        Dirq14fW::new(self, 6)
    }
    ///Bit 7 - IRQ15-DS Pin Deep Software Standby Cancel Flag
    #[inline(always)]
    pub fn dirq15f(&mut self) -> Dirq15fW<Dpsifr1Spec> {
        Dirq15fW::new(self, 7)
    }
}
/**Deep Software Standby Interrupt Flag Register 1

You can [`read`](crate::Reg::read) this register and get [`dpsifr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpsifr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Dpsifr1Spec;
impl crate::RegisterSpec for Dpsifr1Spec {
    type Ux = u8;
}
///`read()` method returns [`dpsifr1::R`](R) reader structure
impl crate::Readable for Dpsifr1Spec {}
///`write(|w| ..)` method takes [`dpsifr1::W`](W) writer structure
impl crate::Writable for Dpsifr1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DPSIFR1 to value 0
impl crate::Resettable for Dpsifr1Spec {}
