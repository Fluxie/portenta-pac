///Register `BRDYENB` reader
pub type R = crate::R<BrdyenbSpec>;
///Register `BRDYENB` writer
pub type W = crate::W<BrdyenbSpec>;
/**BRDY Interrupt Enable for Pipe 0

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe0brdye {
    ///0: Disable interrupt request
    _0 = 0,
    ///1: Enable interrupt request
    _1 = 1,
}
impl From<Pipe0brdye> for bool {
    #[inline(always)]
    fn from(variant: Pipe0brdye) -> Self {
        variant as u8 != 0
    }
}
///Field `PIPE0BRDYE` reader - BRDY Interrupt Enable for Pipe 0
pub type Pipe0brdyeR = crate::BitReader<Pipe0brdye>;
impl Pipe0brdyeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pipe0brdye {
        match self.bits {
            false => Pipe0brdye::_0,
            true => Pipe0brdye::_1,
        }
    }
    ///Disable interrupt request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe0brdye::_0
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe0brdye::_1
    }
}
///Field `PIPE0BRDYE` writer - BRDY Interrupt Enable for Pipe 0
pub type Pipe0brdyeW<'a, REG> = crate::BitWriter<'a, REG, Pipe0brdye>;
impl<'a, REG> Pipe0brdyeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable interrupt request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe0brdye::_0)
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe0brdye::_1)
    }
}
/**BRDY Interrupt Enable for Pipe 1

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe1brdye {
    ///0: Disable interrupt request
    _0 = 0,
    ///1: Enable interrupt request
    _1 = 1,
}
impl From<Pipe1brdye> for bool {
    #[inline(always)]
    fn from(variant: Pipe1brdye) -> Self {
        variant as u8 != 0
    }
}
///Field `PIPE1BRDYE` reader - BRDY Interrupt Enable for Pipe 1
pub type Pipe1brdyeR = crate::BitReader<Pipe1brdye>;
impl Pipe1brdyeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pipe1brdye {
        match self.bits {
            false => Pipe1brdye::_0,
            true => Pipe1brdye::_1,
        }
    }
    ///Disable interrupt request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe1brdye::_0
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe1brdye::_1
    }
}
///Field `PIPE1BRDYE` writer - BRDY Interrupt Enable for Pipe 1
pub type Pipe1brdyeW<'a, REG> = crate::BitWriter<'a, REG, Pipe1brdye>;
impl<'a, REG> Pipe1brdyeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable interrupt request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe1brdye::_0)
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe1brdye::_1)
    }
}
/**BRDY Interrupt Enable for Pipe 2

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe2brdye {
    ///0: Disable interrupt request
    _0 = 0,
    ///1: Enable interrupt request
    _1 = 1,
}
impl From<Pipe2brdye> for bool {
    #[inline(always)]
    fn from(variant: Pipe2brdye) -> Self {
        variant as u8 != 0
    }
}
///Field `PIPE2BRDYE` reader - BRDY Interrupt Enable for Pipe 2
pub type Pipe2brdyeR = crate::BitReader<Pipe2brdye>;
impl Pipe2brdyeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pipe2brdye {
        match self.bits {
            false => Pipe2brdye::_0,
            true => Pipe2brdye::_1,
        }
    }
    ///Disable interrupt request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe2brdye::_0
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe2brdye::_1
    }
}
///Field `PIPE2BRDYE` writer - BRDY Interrupt Enable for Pipe 2
pub type Pipe2brdyeW<'a, REG> = crate::BitWriter<'a, REG, Pipe2brdye>;
impl<'a, REG> Pipe2brdyeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable interrupt request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe2brdye::_0)
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe2brdye::_1)
    }
}
/**BRDY Interrupt Enable for Pipe 3

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe3brdye {
    ///0: Disable interrupt request
    _0 = 0,
    ///1: Enable interrupt request
    _1 = 1,
}
impl From<Pipe3brdye> for bool {
    #[inline(always)]
    fn from(variant: Pipe3brdye) -> Self {
        variant as u8 != 0
    }
}
///Field `PIPE3BRDYE` reader - BRDY Interrupt Enable for Pipe 3
pub type Pipe3brdyeR = crate::BitReader<Pipe3brdye>;
impl Pipe3brdyeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pipe3brdye {
        match self.bits {
            false => Pipe3brdye::_0,
            true => Pipe3brdye::_1,
        }
    }
    ///Disable interrupt request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe3brdye::_0
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe3brdye::_1
    }
}
///Field `PIPE3BRDYE` writer - BRDY Interrupt Enable for Pipe 3
pub type Pipe3brdyeW<'a, REG> = crate::BitWriter<'a, REG, Pipe3brdye>;
impl<'a, REG> Pipe3brdyeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable interrupt request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe3brdye::_0)
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe3brdye::_1)
    }
}
/**BRDY Interrupt Enable for Pipe 4

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe4brdye {
    ///0: Disable interrupt request
    _0 = 0,
    ///1: Enable interrupt request
    _1 = 1,
}
impl From<Pipe4brdye> for bool {
    #[inline(always)]
    fn from(variant: Pipe4brdye) -> Self {
        variant as u8 != 0
    }
}
///Field `PIPE4BRDYE` reader - BRDY Interrupt Enable for Pipe 4
pub type Pipe4brdyeR = crate::BitReader<Pipe4brdye>;
impl Pipe4brdyeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pipe4brdye {
        match self.bits {
            false => Pipe4brdye::_0,
            true => Pipe4brdye::_1,
        }
    }
    ///Disable interrupt request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe4brdye::_0
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe4brdye::_1
    }
}
///Field `PIPE4BRDYE` writer - BRDY Interrupt Enable for Pipe 4
pub type Pipe4brdyeW<'a, REG> = crate::BitWriter<'a, REG, Pipe4brdye>;
impl<'a, REG> Pipe4brdyeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable interrupt request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe4brdye::_0)
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe4brdye::_1)
    }
}
/**BRDY Interrupt Enable for Pipe 5

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe5brdye {
    ///0: Disable interrupt request
    _0 = 0,
    ///1: Enable interrupt request
    _1 = 1,
}
impl From<Pipe5brdye> for bool {
    #[inline(always)]
    fn from(variant: Pipe5brdye) -> Self {
        variant as u8 != 0
    }
}
///Field `PIPE5BRDYE` reader - BRDY Interrupt Enable for Pipe 5
pub type Pipe5brdyeR = crate::BitReader<Pipe5brdye>;
impl Pipe5brdyeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pipe5brdye {
        match self.bits {
            false => Pipe5brdye::_0,
            true => Pipe5brdye::_1,
        }
    }
    ///Disable interrupt request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe5brdye::_0
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe5brdye::_1
    }
}
///Field `PIPE5BRDYE` writer - BRDY Interrupt Enable for Pipe 5
pub type Pipe5brdyeW<'a, REG> = crate::BitWriter<'a, REG, Pipe5brdye>;
impl<'a, REG> Pipe5brdyeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable interrupt request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe5brdye::_0)
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe5brdye::_1)
    }
}
/**BRDY Interrupt Enable for Pipe 6

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe6brdye {
    ///0: Disable interrupt request
    _0 = 0,
    ///1: Enable interrupt request
    _1 = 1,
}
impl From<Pipe6brdye> for bool {
    #[inline(always)]
    fn from(variant: Pipe6brdye) -> Self {
        variant as u8 != 0
    }
}
///Field `PIPE6BRDYE` reader - BRDY Interrupt Enable for Pipe 6
pub type Pipe6brdyeR = crate::BitReader<Pipe6brdye>;
impl Pipe6brdyeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pipe6brdye {
        match self.bits {
            false => Pipe6brdye::_0,
            true => Pipe6brdye::_1,
        }
    }
    ///Disable interrupt request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe6brdye::_0
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe6brdye::_1
    }
}
///Field `PIPE6BRDYE` writer - BRDY Interrupt Enable for Pipe 6
pub type Pipe6brdyeW<'a, REG> = crate::BitWriter<'a, REG, Pipe6brdye>;
impl<'a, REG> Pipe6brdyeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable interrupt request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe6brdye::_0)
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe6brdye::_1)
    }
}
/**BRDY Interrupt Enable for Pipe 7

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe7brdye {
    ///0: Disable interrupt request
    _0 = 0,
    ///1: Enable interrupt request
    _1 = 1,
}
impl From<Pipe7brdye> for bool {
    #[inline(always)]
    fn from(variant: Pipe7brdye) -> Self {
        variant as u8 != 0
    }
}
///Field `PIPE7BRDYE` reader - BRDY Interrupt Enable for Pipe 7
pub type Pipe7brdyeR = crate::BitReader<Pipe7brdye>;
impl Pipe7brdyeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pipe7brdye {
        match self.bits {
            false => Pipe7brdye::_0,
            true => Pipe7brdye::_1,
        }
    }
    ///Disable interrupt request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe7brdye::_0
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe7brdye::_1
    }
}
///Field `PIPE7BRDYE` writer - BRDY Interrupt Enable for Pipe 7
pub type Pipe7brdyeW<'a, REG> = crate::BitWriter<'a, REG, Pipe7brdye>;
impl<'a, REG> Pipe7brdyeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable interrupt request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe7brdye::_0)
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe7brdye::_1)
    }
}
/**BRDY Interrupt Enable for Pipe 8

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe8brdye {
    ///0: Disable interrupt request
    _0 = 0,
    ///1: Enable interrupt request
    _1 = 1,
}
impl From<Pipe8brdye> for bool {
    #[inline(always)]
    fn from(variant: Pipe8brdye) -> Self {
        variant as u8 != 0
    }
}
///Field `PIPE8BRDYE` reader - BRDY Interrupt Enable for Pipe 8
pub type Pipe8brdyeR = crate::BitReader<Pipe8brdye>;
impl Pipe8brdyeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pipe8brdye {
        match self.bits {
            false => Pipe8brdye::_0,
            true => Pipe8brdye::_1,
        }
    }
    ///Disable interrupt request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe8brdye::_0
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe8brdye::_1
    }
}
///Field `PIPE8BRDYE` writer - BRDY Interrupt Enable for Pipe 8
pub type Pipe8brdyeW<'a, REG> = crate::BitWriter<'a, REG, Pipe8brdye>;
impl<'a, REG> Pipe8brdyeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable interrupt request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe8brdye::_0)
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe8brdye::_1)
    }
}
/**BRDY Interrupt Enable for Pipe 9

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe9brdye {
    ///0: Disable interrupt request
    _0 = 0,
    ///1: Enable interrupt request
    _1 = 1,
}
impl From<Pipe9brdye> for bool {
    #[inline(always)]
    fn from(variant: Pipe9brdye) -> Self {
        variant as u8 != 0
    }
}
///Field `PIPE9BRDYE` reader - BRDY Interrupt Enable for Pipe 9
pub type Pipe9brdyeR = crate::BitReader<Pipe9brdye>;
impl Pipe9brdyeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pipe9brdye {
        match self.bits {
            false => Pipe9brdye::_0,
            true => Pipe9brdye::_1,
        }
    }
    ///Disable interrupt request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe9brdye::_0
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe9brdye::_1
    }
}
///Field `PIPE9BRDYE` writer - BRDY Interrupt Enable for Pipe 9
pub type Pipe9brdyeW<'a, REG> = crate::BitWriter<'a, REG, Pipe9brdye>;
impl<'a, REG> Pipe9brdyeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable interrupt request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe9brdye::_0)
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe9brdye::_1)
    }
}
impl R {
    ///Bit 0 - BRDY Interrupt Enable for Pipe 0
    #[inline(always)]
    pub fn pipe0brdye(&self) -> Pipe0brdyeR {
        Pipe0brdyeR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - BRDY Interrupt Enable for Pipe 1
    #[inline(always)]
    pub fn pipe1brdye(&self) -> Pipe1brdyeR {
        Pipe1brdyeR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - BRDY Interrupt Enable for Pipe 2
    #[inline(always)]
    pub fn pipe2brdye(&self) -> Pipe2brdyeR {
        Pipe2brdyeR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - BRDY Interrupt Enable for Pipe 3
    #[inline(always)]
    pub fn pipe3brdye(&self) -> Pipe3brdyeR {
        Pipe3brdyeR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - BRDY Interrupt Enable for Pipe 4
    #[inline(always)]
    pub fn pipe4brdye(&self) -> Pipe4brdyeR {
        Pipe4brdyeR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - BRDY Interrupt Enable for Pipe 5
    #[inline(always)]
    pub fn pipe5brdye(&self) -> Pipe5brdyeR {
        Pipe5brdyeR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - BRDY Interrupt Enable for Pipe 6
    #[inline(always)]
    pub fn pipe6brdye(&self) -> Pipe6brdyeR {
        Pipe6brdyeR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - BRDY Interrupt Enable for Pipe 7
    #[inline(always)]
    pub fn pipe7brdye(&self) -> Pipe7brdyeR {
        Pipe7brdyeR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - BRDY Interrupt Enable for Pipe 8
    #[inline(always)]
    pub fn pipe8brdye(&self) -> Pipe8brdyeR {
        Pipe8brdyeR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - BRDY Interrupt Enable for Pipe 9
    #[inline(always)]
    pub fn pipe9brdye(&self) -> Pipe9brdyeR {
        Pipe9brdyeR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BRDYENB")
            .field("pipe0brdye", &self.pipe0brdye())
            .field("pipe1brdye", &self.pipe1brdye())
            .field("pipe2brdye", &self.pipe2brdye())
            .field("pipe3brdye", &self.pipe3brdye())
            .field("pipe4brdye", &self.pipe4brdye())
            .field("pipe5brdye", &self.pipe5brdye())
            .field("pipe6brdye", &self.pipe6brdye())
            .field("pipe7brdye", &self.pipe7brdye())
            .field("pipe8brdye", &self.pipe8brdye())
            .field("pipe9brdye", &self.pipe9brdye())
            .finish()
    }
}
impl W {
    ///Bit 0 - BRDY Interrupt Enable for Pipe 0
    #[inline(always)]
    pub fn pipe0brdye(&mut self) -> Pipe0brdyeW<BrdyenbSpec> {
        Pipe0brdyeW::new(self, 0)
    }
    ///Bit 1 - BRDY Interrupt Enable for Pipe 1
    #[inline(always)]
    pub fn pipe1brdye(&mut self) -> Pipe1brdyeW<BrdyenbSpec> {
        Pipe1brdyeW::new(self, 1)
    }
    ///Bit 2 - BRDY Interrupt Enable for Pipe 2
    #[inline(always)]
    pub fn pipe2brdye(&mut self) -> Pipe2brdyeW<BrdyenbSpec> {
        Pipe2brdyeW::new(self, 2)
    }
    ///Bit 3 - BRDY Interrupt Enable for Pipe 3
    #[inline(always)]
    pub fn pipe3brdye(&mut self) -> Pipe3brdyeW<BrdyenbSpec> {
        Pipe3brdyeW::new(self, 3)
    }
    ///Bit 4 - BRDY Interrupt Enable for Pipe 4
    #[inline(always)]
    pub fn pipe4brdye(&mut self) -> Pipe4brdyeW<BrdyenbSpec> {
        Pipe4brdyeW::new(self, 4)
    }
    ///Bit 5 - BRDY Interrupt Enable for Pipe 5
    #[inline(always)]
    pub fn pipe5brdye(&mut self) -> Pipe5brdyeW<BrdyenbSpec> {
        Pipe5brdyeW::new(self, 5)
    }
    ///Bit 6 - BRDY Interrupt Enable for Pipe 6
    #[inline(always)]
    pub fn pipe6brdye(&mut self) -> Pipe6brdyeW<BrdyenbSpec> {
        Pipe6brdyeW::new(self, 6)
    }
    ///Bit 7 - BRDY Interrupt Enable for Pipe 7
    #[inline(always)]
    pub fn pipe7brdye(&mut self) -> Pipe7brdyeW<BrdyenbSpec> {
        Pipe7brdyeW::new(self, 7)
    }
    ///Bit 8 - BRDY Interrupt Enable for Pipe 8
    #[inline(always)]
    pub fn pipe8brdye(&mut self) -> Pipe8brdyeW<BrdyenbSpec> {
        Pipe8brdyeW::new(self, 8)
    }
    ///Bit 9 - BRDY Interrupt Enable for Pipe 9
    #[inline(always)]
    pub fn pipe9brdye(&mut self) -> Pipe9brdyeW<BrdyenbSpec> {
        Pipe9brdyeW::new(self, 9)
    }
}
/**BRDY Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`brdyenb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brdyenb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BrdyenbSpec;
impl crate::RegisterSpec for BrdyenbSpec {
    type Ux = u16;
}
///`read()` method returns [`brdyenb::R`](R) reader structure
impl crate::Readable for BrdyenbSpec {}
///`write(|w| ..)` method takes [`brdyenb::W`](W) writer structure
impl crate::Writable for BrdyenbSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BRDYENB to value 0
impl crate::Resettable for BrdyenbSpec {}
