///Register `BEMPENB` reader
pub type R = crate::R<BempenbSpec>;
///Register `BEMPENB` writer
pub type W = crate::W<BempenbSpec>;
/**BEMP Interrupt Enable for Pipe 0

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe0bempe {
    ///0: Disable interrupt request
    _0 = 0,
    ///1: Enable interrupt request
    _1 = 1,
}
impl From<Pipe0bempe> for bool {
    #[inline(always)]
    fn from(variant: Pipe0bempe) -> Self {
        variant as u8 != 0
    }
}
///Field `PIPE0BEMPE` reader - BEMP Interrupt Enable for Pipe 0
pub type Pipe0bempeR = crate::BitReader<Pipe0bempe>;
impl Pipe0bempeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pipe0bempe {
        match self.bits {
            false => Pipe0bempe::_0,
            true => Pipe0bempe::_1,
        }
    }
    ///Disable interrupt request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe0bempe::_0
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe0bempe::_1
    }
}
///Field `PIPE0BEMPE` writer - BEMP Interrupt Enable for Pipe 0
pub type Pipe0bempeW<'a, REG> = crate::BitWriter<'a, REG, Pipe0bempe>;
impl<'a, REG> Pipe0bempeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable interrupt request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe0bempe::_0)
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe0bempe::_1)
    }
}
/**BEMP Interrupt Enable for Pipe 1

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe1bempe {
    ///0: Disable interrupt request
    _0 = 0,
    ///1: Enable interrupt request
    _1 = 1,
}
impl From<Pipe1bempe> for bool {
    #[inline(always)]
    fn from(variant: Pipe1bempe) -> Self {
        variant as u8 != 0
    }
}
///Field `PIPE1BEMPE` reader - BEMP Interrupt Enable for Pipe 1
pub type Pipe1bempeR = crate::BitReader<Pipe1bempe>;
impl Pipe1bempeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pipe1bempe {
        match self.bits {
            false => Pipe1bempe::_0,
            true => Pipe1bempe::_1,
        }
    }
    ///Disable interrupt request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe1bempe::_0
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe1bempe::_1
    }
}
///Field `PIPE1BEMPE` writer - BEMP Interrupt Enable for Pipe 1
pub type Pipe1bempeW<'a, REG> = crate::BitWriter<'a, REG, Pipe1bempe>;
impl<'a, REG> Pipe1bempeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable interrupt request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe1bempe::_0)
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe1bempe::_1)
    }
}
/**BEMP Interrupt Enable for Pipe 2

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe2bempe {
    ///0: Disable interrupt request
    _0 = 0,
    ///1: Enable interrupt request
    _1 = 1,
}
impl From<Pipe2bempe> for bool {
    #[inline(always)]
    fn from(variant: Pipe2bempe) -> Self {
        variant as u8 != 0
    }
}
///Field `PIPE2BEMPE` reader - BEMP Interrupt Enable for Pipe 2
pub type Pipe2bempeR = crate::BitReader<Pipe2bempe>;
impl Pipe2bempeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pipe2bempe {
        match self.bits {
            false => Pipe2bempe::_0,
            true => Pipe2bempe::_1,
        }
    }
    ///Disable interrupt request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe2bempe::_0
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe2bempe::_1
    }
}
///Field `PIPE2BEMPE` writer - BEMP Interrupt Enable for Pipe 2
pub type Pipe2bempeW<'a, REG> = crate::BitWriter<'a, REG, Pipe2bempe>;
impl<'a, REG> Pipe2bempeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable interrupt request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe2bempe::_0)
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe2bempe::_1)
    }
}
/**BEMP Interrupt Enable for Pipe 3

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe3bempe {
    ///0: Disable interrupt request
    _0 = 0,
    ///1: Enable interrupt request
    _1 = 1,
}
impl From<Pipe3bempe> for bool {
    #[inline(always)]
    fn from(variant: Pipe3bempe) -> Self {
        variant as u8 != 0
    }
}
///Field `PIPE3BEMPE` reader - BEMP Interrupt Enable for Pipe 3
pub type Pipe3bempeR = crate::BitReader<Pipe3bempe>;
impl Pipe3bempeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pipe3bempe {
        match self.bits {
            false => Pipe3bempe::_0,
            true => Pipe3bempe::_1,
        }
    }
    ///Disable interrupt request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe3bempe::_0
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe3bempe::_1
    }
}
///Field `PIPE3BEMPE` writer - BEMP Interrupt Enable for Pipe 3
pub type Pipe3bempeW<'a, REG> = crate::BitWriter<'a, REG, Pipe3bempe>;
impl<'a, REG> Pipe3bempeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable interrupt request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe3bempe::_0)
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe3bempe::_1)
    }
}
/**BEMP Interrupt Enable for Pipe 4

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe4bempe {
    ///0: Disable interrupt request
    _0 = 0,
    ///1: Enable interrupt request
    _1 = 1,
}
impl From<Pipe4bempe> for bool {
    #[inline(always)]
    fn from(variant: Pipe4bempe) -> Self {
        variant as u8 != 0
    }
}
///Field `PIPE4BEMPE` reader - BEMP Interrupt Enable for Pipe 4
pub type Pipe4bempeR = crate::BitReader<Pipe4bempe>;
impl Pipe4bempeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pipe4bempe {
        match self.bits {
            false => Pipe4bempe::_0,
            true => Pipe4bempe::_1,
        }
    }
    ///Disable interrupt request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe4bempe::_0
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe4bempe::_1
    }
}
///Field `PIPE4BEMPE` writer - BEMP Interrupt Enable for Pipe 4
pub type Pipe4bempeW<'a, REG> = crate::BitWriter<'a, REG, Pipe4bempe>;
impl<'a, REG> Pipe4bempeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable interrupt request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe4bempe::_0)
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe4bempe::_1)
    }
}
/**BEMP Interrupt Enable for Pipe 5

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe5bempe {
    ///0: Disable interrupt request
    _0 = 0,
    ///1: Enable interrupt request
    _1 = 1,
}
impl From<Pipe5bempe> for bool {
    #[inline(always)]
    fn from(variant: Pipe5bempe) -> Self {
        variant as u8 != 0
    }
}
///Field `PIPE5BEMPE` reader - BEMP Interrupt Enable for Pipe 5
pub type Pipe5bempeR = crate::BitReader<Pipe5bempe>;
impl Pipe5bempeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pipe5bempe {
        match self.bits {
            false => Pipe5bempe::_0,
            true => Pipe5bempe::_1,
        }
    }
    ///Disable interrupt request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe5bempe::_0
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe5bempe::_1
    }
}
///Field `PIPE5BEMPE` writer - BEMP Interrupt Enable for Pipe 5
pub type Pipe5bempeW<'a, REG> = crate::BitWriter<'a, REG, Pipe5bempe>;
impl<'a, REG> Pipe5bempeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable interrupt request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe5bempe::_0)
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe5bempe::_1)
    }
}
/**BEMP Interrupt Enable for Pipe 6

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe6bempe {
    ///0: Disable interrupt request
    _0 = 0,
    ///1: Enable interrupt request
    _1 = 1,
}
impl From<Pipe6bempe> for bool {
    #[inline(always)]
    fn from(variant: Pipe6bempe) -> Self {
        variant as u8 != 0
    }
}
///Field `PIPE6BEMPE` reader - BEMP Interrupt Enable for Pipe 6
pub type Pipe6bempeR = crate::BitReader<Pipe6bempe>;
impl Pipe6bempeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pipe6bempe {
        match self.bits {
            false => Pipe6bempe::_0,
            true => Pipe6bempe::_1,
        }
    }
    ///Disable interrupt request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe6bempe::_0
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe6bempe::_1
    }
}
///Field `PIPE6BEMPE` writer - BEMP Interrupt Enable for Pipe 6
pub type Pipe6bempeW<'a, REG> = crate::BitWriter<'a, REG, Pipe6bempe>;
impl<'a, REG> Pipe6bempeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable interrupt request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe6bempe::_0)
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe6bempe::_1)
    }
}
/**BEMP Interrupt Enable for Pipe 7

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe7bempe {
    ///0: Disable interrupt request
    _0 = 0,
    ///1: Enable interrupt request
    _1 = 1,
}
impl From<Pipe7bempe> for bool {
    #[inline(always)]
    fn from(variant: Pipe7bempe) -> Self {
        variant as u8 != 0
    }
}
///Field `PIPE7BEMPE` reader - BEMP Interrupt Enable for Pipe 7
pub type Pipe7bempeR = crate::BitReader<Pipe7bempe>;
impl Pipe7bempeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pipe7bempe {
        match self.bits {
            false => Pipe7bempe::_0,
            true => Pipe7bempe::_1,
        }
    }
    ///Disable interrupt request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe7bempe::_0
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe7bempe::_1
    }
}
///Field `PIPE7BEMPE` writer - BEMP Interrupt Enable for Pipe 7
pub type Pipe7bempeW<'a, REG> = crate::BitWriter<'a, REG, Pipe7bempe>;
impl<'a, REG> Pipe7bempeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable interrupt request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe7bempe::_0)
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe7bempe::_1)
    }
}
/**BEMP Interrupt Enable for Pipe 8

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe8bempe {
    ///0: Disable interrupt request
    _0 = 0,
    ///1: Enable interrupt request
    _1 = 1,
}
impl From<Pipe8bempe> for bool {
    #[inline(always)]
    fn from(variant: Pipe8bempe) -> Self {
        variant as u8 != 0
    }
}
///Field `PIPE8BEMPE` reader - BEMP Interrupt Enable for Pipe 8
pub type Pipe8bempeR = crate::BitReader<Pipe8bempe>;
impl Pipe8bempeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pipe8bempe {
        match self.bits {
            false => Pipe8bempe::_0,
            true => Pipe8bempe::_1,
        }
    }
    ///Disable interrupt request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe8bempe::_0
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe8bempe::_1
    }
}
///Field `PIPE8BEMPE` writer - BEMP Interrupt Enable for Pipe 8
pub type Pipe8bempeW<'a, REG> = crate::BitWriter<'a, REG, Pipe8bempe>;
impl<'a, REG> Pipe8bempeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable interrupt request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe8bempe::_0)
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe8bempe::_1)
    }
}
/**BEMP Interrupt Enable for Pipe 9

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe9bempe {
    ///0: Disable interrupt request
    _0 = 0,
    ///1: Enable interrupt request
    _1 = 1,
}
impl From<Pipe9bempe> for bool {
    #[inline(always)]
    fn from(variant: Pipe9bempe) -> Self {
        variant as u8 != 0
    }
}
///Field `PIPE9BEMPE` reader - BEMP Interrupt Enable for Pipe 9
pub type Pipe9bempeR = crate::BitReader<Pipe9bempe>;
impl Pipe9bempeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pipe9bempe {
        match self.bits {
            false => Pipe9bempe::_0,
            true => Pipe9bempe::_1,
        }
    }
    ///Disable interrupt request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pipe9bempe::_0
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pipe9bempe::_1
    }
}
///Field `PIPE9BEMPE` writer - BEMP Interrupt Enable for Pipe 9
pub type Pipe9bempeW<'a, REG> = crate::BitWriter<'a, REG, Pipe9bempe>;
impl<'a, REG> Pipe9bempeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable interrupt request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe9bempe::_0)
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pipe9bempe::_1)
    }
}
impl R {
    ///Bit 0 - BEMP Interrupt Enable for Pipe 0
    #[inline(always)]
    pub fn pipe0bempe(&self) -> Pipe0bempeR {
        Pipe0bempeR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - BEMP Interrupt Enable for Pipe 1
    #[inline(always)]
    pub fn pipe1bempe(&self) -> Pipe1bempeR {
        Pipe1bempeR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - BEMP Interrupt Enable for Pipe 2
    #[inline(always)]
    pub fn pipe2bempe(&self) -> Pipe2bempeR {
        Pipe2bempeR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - BEMP Interrupt Enable for Pipe 3
    #[inline(always)]
    pub fn pipe3bempe(&self) -> Pipe3bempeR {
        Pipe3bempeR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - BEMP Interrupt Enable for Pipe 4
    #[inline(always)]
    pub fn pipe4bempe(&self) -> Pipe4bempeR {
        Pipe4bempeR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - BEMP Interrupt Enable for Pipe 5
    #[inline(always)]
    pub fn pipe5bempe(&self) -> Pipe5bempeR {
        Pipe5bempeR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - BEMP Interrupt Enable for Pipe 6
    #[inline(always)]
    pub fn pipe6bempe(&self) -> Pipe6bempeR {
        Pipe6bempeR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - BEMP Interrupt Enable for Pipe 7
    #[inline(always)]
    pub fn pipe7bempe(&self) -> Pipe7bempeR {
        Pipe7bempeR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - BEMP Interrupt Enable for Pipe 8
    #[inline(always)]
    pub fn pipe8bempe(&self) -> Pipe8bempeR {
        Pipe8bempeR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - BEMP Interrupt Enable for Pipe 9
    #[inline(always)]
    pub fn pipe9bempe(&self) -> Pipe9bempeR {
        Pipe9bempeR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BEMPENB")
            .field("pipe0bempe", &self.pipe0bempe())
            .field("pipe1bempe", &self.pipe1bempe())
            .field("pipe2bempe", &self.pipe2bempe())
            .field("pipe3bempe", &self.pipe3bempe())
            .field("pipe4bempe", &self.pipe4bempe())
            .field("pipe5bempe", &self.pipe5bempe())
            .field("pipe6bempe", &self.pipe6bempe())
            .field("pipe7bempe", &self.pipe7bempe())
            .field("pipe8bempe", &self.pipe8bempe())
            .field("pipe9bempe", &self.pipe9bempe())
            .finish()
    }
}
impl W {
    ///Bit 0 - BEMP Interrupt Enable for Pipe 0
    #[inline(always)]
    pub fn pipe0bempe(&mut self) -> Pipe0bempeW<BempenbSpec> {
        Pipe0bempeW::new(self, 0)
    }
    ///Bit 1 - BEMP Interrupt Enable for Pipe 1
    #[inline(always)]
    pub fn pipe1bempe(&mut self) -> Pipe1bempeW<BempenbSpec> {
        Pipe1bempeW::new(self, 1)
    }
    ///Bit 2 - BEMP Interrupt Enable for Pipe 2
    #[inline(always)]
    pub fn pipe2bempe(&mut self) -> Pipe2bempeW<BempenbSpec> {
        Pipe2bempeW::new(self, 2)
    }
    ///Bit 3 - BEMP Interrupt Enable for Pipe 3
    #[inline(always)]
    pub fn pipe3bempe(&mut self) -> Pipe3bempeW<BempenbSpec> {
        Pipe3bempeW::new(self, 3)
    }
    ///Bit 4 - BEMP Interrupt Enable for Pipe 4
    #[inline(always)]
    pub fn pipe4bempe(&mut self) -> Pipe4bempeW<BempenbSpec> {
        Pipe4bempeW::new(self, 4)
    }
    ///Bit 5 - BEMP Interrupt Enable for Pipe 5
    #[inline(always)]
    pub fn pipe5bempe(&mut self) -> Pipe5bempeW<BempenbSpec> {
        Pipe5bempeW::new(self, 5)
    }
    ///Bit 6 - BEMP Interrupt Enable for Pipe 6
    #[inline(always)]
    pub fn pipe6bempe(&mut self) -> Pipe6bempeW<BempenbSpec> {
        Pipe6bempeW::new(self, 6)
    }
    ///Bit 7 - BEMP Interrupt Enable for Pipe 7
    #[inline(always)]
    pub fn pipe7bempe(&mut self) -> Pipe7bempeW<BempenbSpec> {
        Pipe7bempeW::new(self, 7)
    }
    ///Bit 8 - BEMP Interrupt Enable for Pipe 8
    #[inline(always)]
    pub fn pipe8bempe(&mut self) -> Pipe8bempeW<BempenbSpec> {
        Pipe8bempeW::new(self, 8)
    }
    ///Bit 9 - BEMP Interrupt Enable for Pipe 9
    #[inline(always)]
    pub fn pipe9bempe(&mut self) -> Pipe9bempeW<BempenbSpec> {
        Pipe9bempeW::new(self, 9)
    }
}
/**BEMP Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`bempenb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bempenb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BempenbSpec;
impl crate::RegisterSpec for BempenbSpec {
    type Ux = u16;
}
///`read()` method returns [`bempenb::R`](R) reader structure
impl crate::Readable for BempenbSpec {}
///`write(|w| ..)` method takes [`bempenb::W`](W) writer structure
impl crate::Writable for BempenbSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BEMPENB to value 0
impl crate::Resettable for BempenbSpec {}
