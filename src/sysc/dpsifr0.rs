///Register `DPSIFR0` reader
pub type R = crate::R<Dpsifr0Spec>;
///Register `DPSIFR0` writer
pub type W = crate::W<Dpsifr0Spec>;
/**IRQ0-DS Pin Deep Software Standby Cancel Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dirq0f {
    ///0: The cancel request is not generated
    _0 = 0,
    ///1: The cancel request is generated
    _1 = 1,
}
impl From<Dirq0f> for bool {
    #[inline(always)]
    fn from(variant: Dirq0f) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ0F` reader - IRQ0-DS Pin Deep Software Standby Cancel Flag
pub type Dirq0fR = crate::BitReader<Dirq0f>;
impl Dirq0fR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dirq0f {
        match self.bits {
            false => Dirq0f::_0,
            true => Dirq0f::_1,
        }
    }
    ///The cancel request is not generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dirq0f::_0
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dirq0f::_1
    }
}
///Field `DIRQ0F` writer - IRQ0-DS Pin Deep Software Standby Cancel Flag
pub type Dirq0fW<'a, REG> = crate::BitWriter<'a, REG, Dirq0f>;
impl<'a, REG> Dirq0fW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The cancel request is not generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq0f::_0)
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq0f::_1)
    }
}
/**IRQ1-DS Pin Deep Software Standby Cancel Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dirq1f {
    ///0: The cancel request is not generated
    _0 = 0,
    ///1: The cancel request is generated
    _1 = 1,
}
impl From<Dirq1f> for bool {
    #[inline(always)]
    fn from(variant: Dirq1f) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ1F` reader - IRQ1-DS Pin Deep Software Standby Cancel Flag
pub type Dirq1fR = crate::BitReader<Dirq1f>;
impl Dirq1fR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dirq1f {
        match self.bits {
            false => Dirq1f::_0,
            true => Dirq1f::_1,
        }
    }
    ///The cancel request is not generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dirq1f::_0
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dirq1f::_1
    }
}
///Field `DIRQ1F` writer - IRQ1-DS Pin Deep Software Standby Cancel Flag
pub type Dirq1fW<'a, REG> = crate::BitWriter<'a, REG, Dirq1f>;
impl<'a, REG> Dirq1fW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The cancel request is not generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq1f::_0)
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq1f::_1)
    }
}
/**IRQ2-DS Pin Deep Software Standby Cancel Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dirq2f {
    ///0: The cancel request is not generated
    _0 = 0,
    ///1: The cancel request is generated
    _1 = 1,
}
impl From<Dirq2f> for bool {
    #[inline(always)]
    fn from(variant: Dirq2f) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ2F` reader - IRQ2-DS Pin Deep Software Standby Cancel Flag
pub type Dirq2fR = crate::BitReader<Dirq2f>;
impl Dirq2fR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dirq2f {
        match self.bits {
            false => Dirq2f::_0,
            true => Dirq2f::_1,
        }
    }
    ///The cancel request is not generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dirq2f::_0
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dirq2f::_1
    }
}
///Field `DIRQ2F` writer - IRQ2-DS Pin Deep Software Standby Cancel Flag
pub type Dirq2fW<'a, REG> = crate::BitWriter<'a, REG, Dirq2f>;
impl<'a, REG> Dirq2fW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The cancel request is not generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq2f::_0)
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq2f::_1)
    }
}
/**IRQ3-DS Pin Deep Software Standby Cancel Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dirq3f {
    ///0: The cancel request is not generated
    _0 = 0,
    ///1: The cancel request is generated
    _1 = 1,
}
impl From<Dirq3f> for bool {
    #[inline(always)]
    fn from(variant: Dirq3f) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ3F` reader - IRQ3-DS Pin Deep Software Standby Cancel Flag
pub type Dirq3fR = crate::BitReader<Dirq3f>;
impl Dirq3fR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dirq3f {
        match self.bits {
            false => Dirq3f::_0,
            true => Dirq3f::_1,
        }
    }
    ///The cancel request is not generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dirq3f::_0
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dirq3f::_1
    }
}
///Field `DIRQ3F` writer - IRQ3-DS Pin Deep Software Standby Cancel Flag
pub type Dirq3fW<'a, REG> = crate::BitWriter<'a, REG, Dirq3f>;
impl<'a, REG> Dirq3fW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The cancel request is not generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq3f::_0)
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq3f::_1)
    }
}
/**IRQ4-DS Pin Deep Software Standby Cancel Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dirq4f {
    ///0: The cancel request is not generated
    _0 = 0,
    ///1: The cancel request is generated
    _1 = 1,
}
impl From<Dirq4f> for bool {
    #[inline(always)]
    fn from(variant: Dirq4f) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ4F` reader - IRQ4-DS Pin Deep Software Standby Cancel Flag
pub type Dirq4fR = crate::BitReader<Dirq4f>;
impl Dirq4fR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dirq4f {
        match self.bits {
            false => Dirq4f::_0,
            true => Dirq4f::_1,
        }
    }
    ///The cancel request is not generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dirq4f::_0
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dirq4f::_1
    }
}
///Field `DIRQ4F` writer - IRQ4-DS Pin Deep Software Standby Cancel Flag
pub type Dirq4fW<'a, REG> = crate::BitWriter<'a, REG, Dirq4f>;
impl<'a, REG> Dirq4fW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The cancel request is not generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq4f::_0)
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq4f::_1)
    }
}
/**IRQ5-DS Pin Deep Software Standby Cancel Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dirq5f {
    ///0: The cancel request is not generated
    _0 = 0,
    ///1: The cancel request is generated
    _1 = 1,
}
impl From<Dirq5f> for bool {
    #[inline(always)]
    fn from(variant: Dirq5f) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ5F` reader - IRQ5-DS Pin Deep Software Standby Cancel Flag
pub type Dirq5fR = crate::BitReader<Dirq5f>;
impl Dirq5fR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dirq5f {
        match self.bits {
            false => Dirq5f::_0,
            true => Dirq5f::_1,
        }
    }
    ///The cancel request is not generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dirq5f::_0
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dirq5f::_1
    }
}
///Field `DIRQ5F` writer - IRQ5-DS Pin Deep Software Standby Cancel Flag
pub type Dirq5fW<'a, REG> = crate::BitWriter<'a, REG, Dirq5f>;
impl<'a, REG> Dirq5fW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The cancel request is not generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq5f::_0)
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq5f::_1)
    }
}
/**IRQ6-DS Pin Deep Software Standby Cancel Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dirq6f {
    ///0: The cancel request is not generated
    _0 = 0,
    ///1: The cancel request is generated
    _1 = 1,
}
impl From<Dirq6f> for bool {
    #[inline(always)]
    fn from(variant: Dirq6f) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ6F` reader - IRQ6-DS Pin Deep Software Standby Cancel Flag
pub type Dirq6fR = crate::BitReader<Dirq6f>;
impl Dirq6fR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dirq6f {
        match self.bits {
            false => Dirq6f::_0,
            true => Dirq6f::_1,
        }
    }
    ///The cancel request is not generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dirq6f::_0
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dirq6f::_1
    }
}
///Field `DIRQ6F` writer - IRQ6-DS Pin Deep Software Standby Cancel Flag
pub type Dirq6fW<'a, REG> = crate::BitWriter<'a, REG, Dirq6f>;
impl<'a, REG> Dirq6fW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The cancel request is not generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq6f::_0)
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq6f::_1)
    }
}
/**IRQ7-DS Pin Deep Software Standby Cancel Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dirq7f {
    ///0: The cancel request is not generated
    _0 = 0,
    ///1: The cancel request is generated
    _1 = 1,
}
impl From<Dirq7f> for bool {
    #[inline(always)]
    fn from(variant: Dirq7f) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ7F` reader - IRQ7-DS Pin Deep Software Standby Cancel Flag
pub type Dirq7fR = crate::BitReader<Dirq7f>;
impl Dirq7fR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dirq7f {
        match self.bits {
            false => Dirq7f::_0,
            true => Dirq7f::_1,
        }
    }
    ///The cancel request is not generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dirq7f::_0
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dirq7f::_1
    }
}
///Field `DIRQ7F` writer - IRQ7-DS Pin Deep Software Standby Cancel Flag
pub type Dirq7fW<'a, REG> = crate::BitWriter<'a, REG, Dirq7f>;
impl<'a, REG> Dirq7fW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The cancel request is not generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq7f::_0)
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dirq7f::_1)
    }
}
impl R {
    ///Bit 0 - IRQ0-DS Pin Deep Software Standby Cancel Flag
    #[inline(always)]
    pub fn dirq0f(&self) -> Dirq0fR {
        Dirq0fR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - IRQ1-DS Pin Deep Software Standby Cancel Flag
    #[inline(always)]
    pub fn dirq1f(&self) -> Dirq1fR {
        Dirq1fR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - IRQ2-DS Pin Deep Software Standby Cancel Flag
    #[inline(always)]
    pub fn dirq2f(&self) -> Dirq2fR {
        Dirq2fR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - IRQ3-DS Pin Deep Software Standby Cancel Flag
    #[inline(always)]
    pub fn dirq3f(&self) -> Dirq3fR {
        Dirq3fR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IRQ4-DS Pin Deep Software Standby Cancel Flag
    #[inline(always)]
    pub fn dirq4f(&self) -> Dirq4fR {
        Dirq4fR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - IRQ5-DS Pin Deep Software Standby Cancel Flag
    #[inline(always)]
    pub fn dirq5f(&self) -> Dirq5fR {
        Dirq5fR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - IRQ6-DS Pin Deep Software Standby Cancel Flag
    #[inline(always)]
    pub fn dirq6f(&self) -> Dirq6fR {
        Dirq6fR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - IRQ7-DS Pin Deep Software Standby Cancel Flag
    #[inline(always)]
    pub fn dirq7f(&self) -> Dirq7fR {
        Dirq7fR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DPSIFR0")
            .field("dirq0f", &self.dirq0f())
            .field("dirq1f", &self.dirq1f())
            .field("dirq2f", &self.dirq2f())
            .field("dirq3f", &self.dirq3f())
            .field("dirq4f", &self.dirq4f())
            .field("dirq5f", &self.dirq5f())
            .field("dirq6f", &self.dirq6f())
            .field("dirq7f", &self.dirq7f())
            .finish()
    }
}
impl W {
    ///Bit 0 - IRQ0-DS Pin Deep Software Standby Cancel Flag
    #[inline(always)]
    pub fn dirq0f(&mut self) -> Dirq0fW<Dpsifr0Spec> {
        Dirq0fW::new(self, 0)
    }
    ///Bit 1 - IRQ1-DS Pin Deep Software Standby Cancel Flag
    #[inline(always)]
    pub fn dirq1f(&mut self) -> Dirq1fW<Dpsifr0Spec> {
        Dirq1fW::new(self, 1)
    }
    ///Bit 2 - IRQ2-DS Pin Deep Software Standby Cancel Flag
    #[inline(always)]
    pub fn dirq2f(&mut self) -> Dirq2fW<Dpsifr0Spec> {
        Dirq2fW::new(self, 2)
    }
    ///Bit 3 - IRQ3-DS Pin Deep Software Standby Cancel Flag
    #[inline(always)]
    pub fn dirq3f(&mut self) -> Dirq3fW<Dpsifr0Spec> {
        Dirq3fW::new(self, 3)
    }
    ///Bit 4 - IRQ4-DS Pin Deep Software Standby Cancel Flag
    #[inline(always)]
    pub fn dirq4f(&mut self) -> Dirq4fW<Dpsifr0Spec> {
        Dirq4fW::new(self, 4)
    }
    ///Bit 5 - IRQ5-DS Pin Deep Software Standby Cancel Flag
    #[inline(always)]
    pub fn dirq5f(&mut self) -> Dirq5fW<Dpsifr0Spec> {
        Dirq5fW::new(self, 5)
    }
    ///Bit 6 - IRQ6-DS Pin Deep Software Standby Cancel Flag
    #[inline(always)]
    pub fn dirq6f(&mut self) -> Dirq6fW<Dpsifr0Spec> {
        Dirq6fW::new(self, 6)
    }
    ///Bit 7 - IRQ7-DS Pin Deep Software Standby Cancel Flag
    #[inline(always)]
    pub fn dirq7f(&mut self) -> Dirq7fW<Dpsifr0Spec> {
        Dirq7fW::new(self, 7)
    }
}
/**Deep Software Standby Interrupt Flag Register 0

You can [`read`](crate::Reg::read) this register and get [`dpsifr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpsifr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Dpsifr0Spec;
impl crate::RegisterSpec for Dpsifr0Spec {
    type Ux = u8;
}
///`read()` method returns [`dpsifr0::R`](R) reader structure
impl crate::Readable for Dpsifr0Spec {}
///`write(|w| ..)` method takes [`dpsifr0::W`](W) writer structure
impl crate::Writable for Dpsifr0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DPSIFR0 to value 0
impl crate::Resettable for Dpsifr0Spec {}
