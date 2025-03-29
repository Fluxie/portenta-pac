///Register `GTCLR` writer
pub type W = crate::W<GtclrSpec>;
/**Channel n GTCNT Count Clear (n : the same as bit position value)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cclr0 {
    ///0: GTCNT counter is not cleared
    _0 = 0,
    ///1: GTCNT counter is cleared
    _1 = 1,
}
impl From<Cclr0> for bool {
    #[inline(always)]
    fn from(variant: Cclr0) -> Self {
        variant as u8 != 0
    }
}
///Field `CCLR0` writer - Channel n GTCNT Count Clear (n : the same as bit position value)
pub type Cclr0W<'a, REG> = crate::BitWriter<'a, REG, Cclr0>;
impl<'a, REG> Cclr0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///GTCNT counter is not cleared
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cclr0::_0)
    }
    ///GTCNT counter is cleared
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cclr0::_1)
    }
}
/**Channel n GTCNT Count Clear (n : the same as bit position value)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cclr1 {
    ///0: GTCNT counter is not cleared
    _0 = 0,
    ///1: GTCNT counter is cleared
    _1 = 1,
}
impl From<Cclr1> for bool {
    #[inline(always)]
    fn from(variant: Cclr1) -> Self {
        variant as u8 != 0
    }
}
///Field `CCLR1` writer - Channel n GTCNT Count Clear (n : the same as bit position value)
pub type Cclr1W<'a, REG> = crate::BitWriter<'a, REG, Cclr1>;
impl<'a, REG> Cclr1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///GTCNT counter is not cleared
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cclr1::_0)
    }
    ///GTCNT counter is cleared
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cclr1::_1)
    }
}
/**Channel n GTCNT Count Clear (n : the same as bit position value)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cclr2 {
    ///0: GTCNT counter is not cleared
    _0 = 0,
    ///1: GTCNT counter is cleared
    _1 = 1,
}
impl From<Cclr2> for bool {
    #[inline(always)]
    fn from(variant: Cclr2) -> Self {
        variant as u8 != 0
    }
}
///Field `CCLR2` writer - Channel n GTCNT Count Clear (n : the same as bit position value)
pub type Cclr2W<'a, REG> = crate::BitWriter<'a, REG, Cclr2>;
impl<'a, REG> Cclr2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///GTCNT counter is not cleared
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cclr2::_0)
    }
    ///GTCNT counter is cleared
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cclr2::_1)
    }
}
/**Channel n GTCNT Count Clear (n : the same as bit position value)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cclr3 {
    ///0: GTCNT counter is not cleared
    _0 = 0,
    ///1: GTCNT counter is cleared
    _1 = 1,
}
impl From<Cclr3> for bool {
    #[inline(always)]
    fn from(variant: Cclr3) -> Self {
        variant as u8 != 0
    }
}
///Field `CCLR3` writer - Channel n GTCNT Count Clear (n : the same as bit position value)
pub type Cclr3W<'a, REG> = crate::BitWriter<'a, REG, Cclr3>;
impl<'a, REG> Cclr3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///GTCNT counter is not cleared
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cclr3::_0)
    }
    ///GTCNT counter is cleared
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cclr3::_1)
    }
}
/**Channel n GTCNT Count Clear (n : the same as bit position value)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cclr4 {
    ///0: GTCNT counter is not cleared
    _0 = 0,
    ///1: GTCNT counter is cleared
    _1 = 1,
}
impl From<Cclr4> for bool {
    #[inline(always)]
    fn from(variant: Cclr4) -> Self {
        variant as u8 != 0
    }
}
///Field `CCLR4` writer - Channel n GTCNT Count Clear (n : the same as bit position value)
pub type Cclr4W<'a, REG> = crate::BitWriter<'a, REG, Cclr4>;
impl<'a, REG> Cclr4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///GTCNT counter is not cleared
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cclr4::_0)
    }
    ///GTCNT counter is cleared
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cclr4::_1)
    }
}
/**Channel n GTCNT Count Clear (n : the same as bit position value)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cclr5 {
    ///0: GTCNT counter is not cleared
    _0 = 0,
    ///1: GTCNT counter is cleared
    _1 = 1,
}
impl From<Cclr5> for bool {
    #[inline(always)]
    fn from(variant: Cclr5) -> Self {
        variant as u8 != 0
    }
}
///Field `CCLR5` writer - Channel n GTCNT Count Clear (n : the same as bit position value)
pub type Cclr5W<'a, REG> = crate::BitWriter<'a, REG, Cclr5>;
impl<'a, REG> Cclr5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///GTCNT counter is not cleared
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cclr5::_0)
    }
    ///GTCNT counter is cleared
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cclr5::_1)
    }
}
/**Channel n GTCNT Count Clear (n : the same as bit position value)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cclr6 {
    ///0: GTCNT counter is not cleared
    _0 = 0,
    ///1: GTCNT counter is cleared
    _1 = 1,
}
impl From<Cclr6> for bool {
    #[inline(always)]
    fn from(variant: Cclr6) -> Self {
        variant as u8 != 0
    }
}
///Field `CCLR6` writer - Channel n GTCNT Count Clear (n : the same as bit position value)
pub type Cclr6W<'a, REG> = crate::BitWriter<'a, REG, Cclr6>;
impl<'a, REG> Cclr6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///GTCNT counter is not cleared
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cclr6::_0)
    }
    ///GTCNT counter is cleared
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cclr6::_1)
    }
}
/**Channel n GTCNT Count Clear (n : the same as bit position value)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cclr7 {
    ///0: GTCNT counter is not cleared
    _0 = 0,
    ///1: GTCNT counter is cleared
    _1 = 1,
}
impl From<Cclr7> for bool {
    #[inline(always)]
    fn from(variant: Cclr7) -> Self {
        variant as u8 != 0
    }
}
///Field `CCLR7` writer - Channel n GTCNT Count Clear (n : the same as bit position value)
pub type Cclr7W<'a, REG> = crate::BitWriter<'a, REG, Cclr7>;
impl<'a, REG> Cclr7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///GTCNT counter is not cleared
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cclr7::_0)
    }
    ///GTCNT counter is cleared
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cclr7::_1)
    }
}
/**Channel n GTCNT Count Clear (n : the same as bit position value)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cclr8 {
    ///0: GTCNT counter is not cleared
    _0 = 0,
    ///1: GTCNT counter is cleared
    _1 = 1,
}
impl From<Cclr8> for bool {
    #[inline(always)]
    fn from(variant: Cclr8) -> Self {
        variant as u8 != 0
    }
}
///Field `CCLR8` writer - Channel n GTCNT Count Clear (n : the same as bit position value)
pub type Cclr8W<'a, REG> = crate::BitWriter<'a, REG, Cclr8>;
impl<'a, REG> Cclr8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///GTCNT counter is not cleared
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cclr8::_0)
    }
    ///GTCNT counter is cleared
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cclr8::_1)
    }
}
/**Channel n GTCNT Count Clear (n : the same as bit position value)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cclr9 {
    ///0: GTCNT counter is not cleared
    _0 = 0,
    ///1: GTCNT counter is cleared
    _1 = 1,
}
impl From<Cclr9> for bool {
    #[inline(always)]
    fn from(variant: Cclr9) -> Self {
        variant as u8 != 0
    }
}
///Field `CCLR9` writer - Channel n GTCNT Count Clear (n : the same as bit position value)
pub type Cclr9W<'a, REG> = crate::BitWriter<'a, REG, Cclr9>;
impl<'a, REG> Cclr9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///GTCNT counter is not cleared
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cclr9::_0)
    }
    ///GTCNT counter is cleared
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cclr9::_1)
    }
}
impl core::fmt::Debug for crate::generic::Reg<GtclrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Channel n GTCNT Count Clear (n : the same as bit position value)
    #[inline(always)]
    pub fn cclr0(&mut self) -> Cclr0W<GtclrSpec> {
        Cclr0W::new(self, 0)
    }
    ///Bit 1 - Channel n GTCNT Count Clear (n : the same as bit position value)
    #[inline(always)]
    pub fn cclr1(&mut self) -> Cclr1W<GtclrSpec> {
        Cclr1W::new(self, 1)
    }
    ///Bit 2 - Channel n GTCNT Count Clear (n : the same as bit position value)
    #[inline(always)]
    pub fn cclr2(&mut self) -> Cclr2W<GtclrSpec> {
        Cclr2W::new(self, 2)
    }
    ///Bit 3 - Channel n GTCNT Count Clear (n : the same as bit position value)
    #[inline(always)]
    pub fn cclr3(&mut self) -> Cclr3W<GtclrSpec> {
        Cclr3W::new(self, 3)
    }
    ///Bit 4 - Channel n GTCNT Count Clear (n : the same as bit position value)
    #[inline(always)]
    pub fn cclr4(&mut self) -> Cclr4W<GtclrSpec> {
        Cclr4W::new(self, 4)
    }
    ///Bit 5 - Channel n GTCNT Count Clear (n : the same as bit position value)
    #[inline(always)]
    pub fn cclr5(&mut self) -> Cclr5W<GtclrSpec> {
        Cclr5W::new(self, 5)
    }
    ///Bit 6 - Channel n GTCNT Count Clear (n : the same as bit position value)
    #[inline(always)]
    pub fn cclr6(&mut self) -> Cclr6W<GtclrSpec> {
        Cclr6W::new(self, 6)
    }
    ///Bit 7 - Channel n GTCNT Count Clear (n : the same as bit position value)
    #[inline(always)]
    pub fn cclr7(&mut self) -> Cclr7W<GtclrSpec> {
        Cclr7W::new(self, 7)
    }
    ///Bit 8 - Channel n GTCNT Count Clear (n : the same as bit position value)
    #[inline(always)]
    pub fn cclr8(&mut self) -> Cclr8W<GtclrSpec> {
        Cclr8W::new(self, 8)
    }
    ///Bit 9 - Channel n GTCNT Count Clear (n : the same as bit position value)
    #[inline(always)]
    pub fn cclr9(&mut self) -> Cclr9W<GtclrSpec> {
        Cclr9W::new(self, 9)
    }
}
/**General PWM Timer Software Clear Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GtclrSpec;
impl crate::RegisterSpec for GtclrSpec {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`gtclr::W`](W) writer structure
impl crate::Writable for GtclrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTCLR to value 0
impl crate::Resettable for GtclrSpec {}
