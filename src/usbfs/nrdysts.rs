///Register `NRDYSTS` reader
pub type R = crate::R<NrdystsSpec>;
///Register `NRDYSTS` writer
pub type W = crate::W<NrdystsSpec>;
/**NRDY Interrupt Status for Pipe 0

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe0nrdy {
    ///0: No NRDY interrupt occurred
    _0 = 0,
    ///1: NRDY interrupt occurred
    _1 = 1,
}
impl From<Pipe0nrdy> for bool {
    #[inline(always)]
    fn from(variant: Pipe0nrdy) -> Self {
        variant as u8 != 0
    }
}
///Field `PIPE0NRDY` reader - NRDY Interrupt Status for Pipe 0
pub type Pipe0nrdyR = crate::BitReader<Pipe0nrdy>;
impl Pipe0nrdyR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pipe0nrdy {
        match self.bits {
            false => Pipe0nrdy::_0,
            true => Pipe0nrdy::_1,
        }
    }
    ///No NRDY interrupt occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe0nrdy::_0
    }
    ///NRDY interrupt occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe0nrdy::_1
    }
}
///Field `PIPE0NRDY` writer - NRDY Interrupt Status for Pipe 0
pub type Pipe0nrdyW<'a, REG> = crate::BitWriter<'a, REG, Pipe0nrdy>;
impl<'a, REG> Pipe0nrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No NRDY interrupt occurred
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe0nrdy::_0)
    }
    ///NRDY interrupt occurred
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe0nrdy::_1)
    }
}
/**NRDY Interrupt Status for Pipe 1

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe1nrdy {
    ///0: No NRDY interrupt occurred
    _0 = 0,
    ///1: NRDY interrupt occurred
    _1 = 1,
}
impl From<Pipe1nrdy> for bool {
    #[inline(always)]
    fn from(variant: Pipe1nrdy) -> Self {
        variant as u8 != 0
    }
}
///Field `PIPE1NRDY` reader - NRDY Interrupt Status for Pipe 1
pub type Pipe1nrdyR = crate::BitReader<Pipe1nrdy>;
impl Pipe1nrdyR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pipe1nrdy {
        match self.bits {
            false => Pipe1nrdy::_0,
            true => Pipe1nrdy::_1,
        }
    }
    ///No NRDY interrupt occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe1nrdy::_0
    }
    ///NRDY interrupt occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe1nrdy::_1
    }
}
///Field `PIPE1NRDY` writer - NRDY Interrupt Status for Pipe 1
pub type Pipe1nrdyW<'a, REG> = crate::BitWriter<'a, REG, Pipe1nrdy>;
impl<'a, REG> Pipe1nrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No NRDY interrupt occurred
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe1nrdy::_0)
    }
    ///NRDY interrupt occurred
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe1nrdy::_1)
    }
}
/**NRDY Interrupt Status for Pipe 2

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe2nrdy {
    ///0: No NRDY interrupt occurred
    _0 = 0,
    ///1: NRDY interrupt occurred
    _1 = 1,
}
impl From<Pipe2nrdy> for bool {
    #[inline(always)]
    fn from(variant: Pipe2nrdy) -> Self {
        variant as u8 != 0
    }
}
///Field `PIPE2NRDY` reader - NRDY Interrupt Status for Pipe 2
pub type Pipe2nrdyR = crate::BitReader<Pipe2nrdy>;
impl Pipe2nrdyR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pipe2nrdy {
        match self.bits {
            false => Pipe2nrdy::_0,
            true => Pipe2nrdy::_1,
        }
    }
    ///No NRDY interrupt occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe2nrdy::_0
    }
    ///NRDY interrupt occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe2nrdy::_1
    }
}
///Field `PIPE2NRDY` writer - NRDY Interrupt Status for Pipe 2
pub type Pipe2nrdyW<'a, REG> = crate::BitWriter<'a, REG, Pipe2nrdy>;
impl<'a, REG> Pipe2nrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No NRDY interrupt occurred
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe2nrdy::_0)
    }
    ///NRDY interrupt occurred
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe2nrdy::_1)
    }
}
/**NRDY Interrupt Status for Pipe 3

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe3nrdy {
    ///0: No NRDY interrupt occurred
    _0 = 0,
    ///1: NRDY interrupt occurred
    _1 = 1,
}
impl From<Pipe3nrdy> for bool {
    #[inline(always)]
    fn from(variant: Pipe3nrdy) -> Self {
        variant as u8 != 0
    }
}
///Field `PIPE3NRDY` reader - NRDY Interrupt Status for Pipe 3
pub type Pipe3nrdyR = crate::BitReader<Pipe3nrdy>;
impl Pipe3nrdyR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pipe3nrdy {
        match self.bits {
            false => Pipe3nrdy::_0,
            true => Pipe3nrdy::_1,
        }
    }
    ///No NRDY interrupt occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe3nrdy::_0
    }
    ///NRDY interrupt occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe3nrdy::_1
    }
}
///Field `PIPE3NRDY` writer - NRDY Interrupt Status for Pipe 3
pub type Pipe3nrdyW<'a, REG> = crate::BitWriter<'a, REG, Pipe3nrdy>;
impl<'a, REG> Pipe3nrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No NRDY interrupt occurred
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe3nrdy::_0)
    }
    ///NRDY interrupt occurred
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe3nrdy::_1)
    }
}
/**NRDY Interrupt Status for Pipe 4

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe4nrdy {
    ///0: No NRDY interrupt occurred
    _0 = 0,
    ///1: NRDY interrupt occurred
    _1 = 1,
}
impl From<Pipe4nrdy> for bool {
    #[inline(always)]
    fn from(variant: Pipe4nrdy) -> Self {
        variant as u8 != 0
    }
}
///Field `PIPE4NRDY` reader - NRDY Interrupt Status for Pipe 4
pub type Pipe4nrdyR = crate::BitReader<Pipe4nrdy>;
impl Pipe4nrdyR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pipe4nrdy {
        match self.bits {
            false => Pipe4nrdy::_0,
            true => Pipe4nrdy::_1,
        }
    }
    ///No NRDY interrupt occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe4nrdy::_0
    }
    ///NRDY interrupt occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe4nrdy::_1
    }
}
///Field `PIPE4NRDY` writer - NRDY Interrupt Status for Pipe 4
pub type Pipe4nrdyW<'a, REG> = crate::BitWriter<'a, REG, Pipe4nrdy>;
impl<'a, REG> Pipe4nrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No NRDY interrupt occurred
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe4nrdy::_0)
    }
    ///NRDY interrupt occurred
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe4nrdy::_1)
    }
}
/**NRDY Interrupt Status for Pipe 5

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe5nrdy {
    ///0: No NRDY interrupt occurred
    _0 = 0,
    ///1: NRDY interrupt occurred
    _1 = 1,
}
impl From<Pipe5nrdy> for bool {
    #[inline(always)]
    fn from(variant: Pipe5nrdy) -> Self {
        variant as u8 != 0
    }
}
///Field `PIPE5NRDY` reader - NRDY Interrupt Status for Pipe 5
pub type Pipe5nrdyR = crate::BitReader<Pipe5nrdy>;
impl Pipe5nrdyR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pipe5nrdy {
        match self.bits {
            false => Pipe5nrdy::_0,
            true => Pipe5nrdy::_1,
        }
    }
    ///No NRDY interrupt occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe5nrdy::_0
    }
    ///NRDY interrupt occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe5nrdy::_1
    }
}
///Field `PIPE5NRDY` writer - NRDY Interrupt Status for Pipe 5
pub type Pipe5nrdyW<'a, REG> = crate::BitWriter<'a, REG, Pipe5nrdy>;
impl<'a, REG> Pipe5nrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No NRDY interrupt occurred
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe5nrdy::_0)
    }
    ///NRDY interrupt occurred
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe5nrdy::_1)
    }
}
/**NRDY Interrupt Status for Pipe 6

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe6nrdy {
    ///0: No NRDY interrupt occurred
    _0 = 0,
    ///1: NRDY interrupt occurred
    _1 = 1,
}
impl From<Pipe6nrdy> for bool {
    #[inline(always)]
    fn from(variant: Pipe6nrdy) -> Self {
        variant as u8 != 0
    }
}
///Field `PIPE6NRDY` reader - NRDY Interrupt Status for Pipe 6
pub type Pipe6nrdyR = crate::BitReader<Pipe6nrdy>;
impl Pipe6nrdyR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pipe6nrdy {
        match self.bits {
            false => Pipe6nrdy::_0,
            true => Pipe6nrdy::_1,
        }
    }
    ///No NRDY interrupt occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe6nrdy::_0
    }
    ///NRDY interrupt occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe6nrdy::_1
    }
}
///Field `PIPE6NRDY` writer - NRDY Interrupt Status for Pipe 6
pub type Pipe6nrdyW<'a, REG> = crate::BitWriter<'a, REG, Pipe6nrdy>;
impl<'a, REG> Pipe6nrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No NRDY interrupt occurred
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe6nrdy::_0)
    }
    ///NRDY interrupt occurred
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe6nrdy::_1)
    }
}
/**NRDY Interrupt Status for Pipe 7

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe7nrdy {
    ///0: No NRDY interrupt occurred
    _0 = 0,
    ///1: NRDY interrupt occurred
    _1 = 1,
}
impl From<Pipe7nrdy> for bool {
    #[inline(always)]
    fn from(variant: Pipe7nrdy) -> Self {
        variant as u8 != 0
    }
}
///Field `PIPE7NRDY` reader - NRDY Interrupt Status for Pipe 7
pub type Pipe7nrdyR = crate::BitReader<Pipe7nrdy>;
impl Pipe7nrdyR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pipe7nrdy {
        match self.bits {
            false => Pipe7nrdy::_0,
            true => Pipe7nrdy::_1,
        }
    }
    ///No NRDY interrupt occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe7nrdy::_0
    }
    ///NRDY interrupt occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe7nrdy::_1
    }
}
///Field `PIPE7NRDY` writer - NRDY Interrupt Status for Pipe 7
pub type Pipe7nrdyW<'a, REG> = crate::BitWriter<'a, REG, Pipe7nrdy>;
impl<'a, REG> Pipe7nrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No NRDY interrupt occurred
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe7nrdy::_0)
    }
    ///NRDY interrupt occurred
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe7nrdy::_1)
    }
}
/**NRDY Interrupt Status for Pipe 8

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe8nrdy {
    ///0: No NRDY interrupt occurred
    _0 = 0,
    ///1: NRDY interrupt occurred
    _1 = 1,
}
impl From<Pipe8nrdy> for bool {
    #[inline(always)]
    fn from(variant: Pipe8nrdy) -> Self {
        variant as u8 != 0
    }
}
///Field `PIPE8NRDY` reader - NRDY Interrupt Status for Pipe 8
pub type Pipe8nrdyR = crate::BitReader<Pipe8nrdy>;
impl Pipe8nrdyR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pipe8nrdy {
        match self.bits {
            false => Pipe8nrdy::_0,
            true => Pipe8nrdy::_1,
        }
    }
    ///No NRDY interrupt occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe8nrdy::_0
    }
    ///NRDY interrupt occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe8nrdy::_1
    }
}
///Field `PIPE8NRDY` writer - NRDY Interrupt Status for Pipe 8
pub type Pipe8nrdyW<'a, REG> = crate::BitWriter<'a, REG, Pipe8nrdy>;
impl<'a, REG> Pipe8nrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No NRDY interrupt occurred
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe8nrdy::_0)
    }
    ///NRDY interrupt occurred
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe8nrdy::_1)
    }
}
/**NRDY Interrupt Status for Pipe 9

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe9nrdy {
    ///0: No NRDY interrupt occurred
    _0 = 0,
    ///1: NRDY interrupt occurred
    _1 = 1,
}
impl From<Pipe9nrdy> for bool {
    #[inline(always)]
    fn from(variant: Pipe9nrdy) -> Self {
        variant as u8 != 0
    }
}
///Field `PIPE9NRDY` reader - NRDY Interrupt Status for Pipe 9
pub type Pipe9nrdyR = crate::BitReader<Pipe9nrdy>;
impl Pipe9nrdyR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pipe9nrdy {
        match self.bits {
            false => Pipe9nrdy::_0,
            true => Pipe9nrdy::_1,
        }
    }
    ///No NRDY interrupt occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe9nrdy::_0
    }
    ///NRDY interrupt occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe9nrdy::_1
    }
}
///Field `PIPE9NRDY` writer - NRDY Interrupt Status for Pipe 9
pub type Pipe9nrdyW<'a, REG> = crate::BitWriter<'a, REG, Pipe9nrdy>;
impl<'a, REG> Pipe9nrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No NRDY interrupt occurred
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe9nrdy::_0)
    }
    ///NRDY interrupt occurred
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe9nrdy::_1)
    }
}
impl R {
    ///Bit 0 - NRDY Interrupt Status for Pipe 0
    #[inline(always)]
    pub fn pipe0nrdy(&self) -> Pipe0nrdyR {
        Pipe0nrdyR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - NRDY Interrupt Status for Pipe 1
    #[inline(always)]
    pub fn pipe1nrdy(&self) -> Pipe1nrdyR {
        Pipe1nrdyR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - NRDY Interrupt Status for Pipe 2
    #[inline(always)]
    pub fn pipe2nrdy(&self) -> Pipe2nrdyR {
        Pipe2nrdyR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - NRDY Interrupt Status for Pipe 3
    #[inline(always)]
    pub fn pipe3nrdy(&self) -> Pipe3nrdyR {
        Pipe3nrdyR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - NRDY Interrupt Status for Pipe 4
    #[inline(always)]
    pub fn pipe4nrdy(&self) -> Pipe4nrdyR {
        Pipe4nrdyR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - NRDY Interrupt Status for Pipe 5
    #[inline(always)]
    pub fn pipe5nrdy(&self) -> Pipe5nrdyR {
        Pipe5nrdyR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - NRDY Interrupt Status for Pipe 6
    #[inline(always)]
    pub fn pipe6nrdy(&self) -> Pipe6nrdyR {
        Pipe6nrdyR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - NRDY Interrupt Status for Pipe 7
    #[inline(always)]
    pub fn pipe7nrdy(&self) -> Pipe7nrdyR {
        Pipe7nrdyR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - NRDY Interrupt Status for Pipe 8
    #[inline(always)]
    pub fn pipe8nrdy(&self) -> Pipe8nrdyR {
        Pipe8nrdyR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - NRDY Interrupt Status for Pipe 9
    #[inline(always)]
    pub fn pipe9nrdy(&self) -> Pipe9nrdyR {
        Pipe9nrdyR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NRDYSTS")
            .field("pipe0nrdy", &self.pipe0nrdy())
            .field("pipe1nrdy", &self.pipe1nrdy())
            .field("pipe2nrdy", &self.pipe2nrdy())
            .field("pipe3nrdy", &self.pipe3nrdy())
            .field("pipe4nrdy", &self.pipe4nrdy())
            .field("pipe5nrdy", &self.pipe5nrdy())
            .field("pipe6nrdy", &self.pipe6nrdy())
            .field("pipe7nrdy", &self.pipe7nrdy())
            .field("pipe8nrdy", &self.pipe8nrdy())
            .field("pipe9nrdy", &self.pipe9nrdy())
            .finish()
    }
}
impl W {
    ///Bit 0 - NRDY Interrupt Status for Pipe 0
    #[inline(always)]
    pub fn pipe0nrdy(&mut self) -> Pipe0nrdyW<NrdystsSpec> {
        Pipe0nrdyW::new(self, 0)
    }
    ///Bit 1 - NRDY Interrupt Status for Pipe 1
    #[inline(always)]
    pub fn pipe1nrdy(&mut self) -> Pipe1nrdyW<NrdystsSpec> {
        Pipe1nrdyW::new(self, 1)
    }
    ///Bit 2 - NRDY Interrupt Status for Pipe 2
    #[inline(always)]
    pub fn pipe2nrdy(&mut self) -> Pipe2nrdyW<NrdystsSpec> {
        Pipe2nrdyW::new(self, 2)
    }
    ///Bit 3 - NRDY Interrupt Status for Pipe 3
    #[inline(always)]
    pub fn pipe3nrdy(&mut self) -> Pipe3nrdyW<NrdystsSpec> {
        Pipe3nrdyW::new(self, 3)
    }
    ///Bit 4 - NRDY Interrupt Status for Pipe 4
    #[inline(always)]
    pub fn pipe4nrdy(&mut self) -> Pipe4nrdyW<NrdystsSpec> {
        Pipe4nrdyW::new(self, 4)
    }
    ///Bit 5 - NRDY Interrupt Status for Pipe 5
    #[inline(always)]
    pub fn pipe5nrdy(&mut self) -> Pipe5nrdyW<NrdystsSpec> {
        Pipe5nrdyW::new(self, 5)
    }
    ///Bit 6 - NRDY Interrupt Status for Pipe 6
    #[inline(always)]
    pub fn pipe6nrdy(&mut self) -> Pipe6nrdyW<NrdystsSpec> {
        Pipe6nrdyW::new(self, 6)
    }
    ///Bit 7 - NRDY Interrupt Status for Pipe 7
    #[inline(always)]
    pub fn pipe7nrdy(&mut self) -> Pipe7nrdyW<NrdystsSpec> {
        Pipe7nrdyW::new(self, 7)
    }
    ///Bit 8 - NRDY Interrupt Status for Pipe 8
    #[inline(always)]
    pub fn pipe8nrdy(&mut self) -> Pipe8nrdyW<NrdystsSpec> {
        Pipe8nrdyW::new(self, 8)
    }
    ///Bit 9 - NRDY Interrupt Status for Pipe 9
    #[inline(always)]
    pub fn pipe9nrdy(&mut self) -> Pipe9nrdyW<NrdystsSpec> {
        Pipe9nrdyW::new(self, 9)
    }
}
/**NRDY Interrupt Status Register

You can [`read`](crate::Reg::read) this register and get [`nrdysts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nrdysts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct NrdystsSpec;
impl crate::RegisterSpec for NrdystsSpec {
    type Ux = u16;
}
///`read()` method returns [`nrdysts::R`](R) reader structure
impl crate::Readable for NrdystsSpec {}
///`write(|w| ..)` method takes [`nrdysts::W`](W) writer structure
impl crate::Writable for NrdystsSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets NRDYSTS to value 0
impl crate::Resettable for NrdystsSpec {}
