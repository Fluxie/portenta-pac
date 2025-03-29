///Register `CFDTHLCC%s` reader
pub type R = crate::R<CfdthlccSpec>;
///Register `CFDTHLCC%s` writer
pub type W = crate::W<CfdthlccSpec>;
/**TX History List Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Thle {
    ///0: TX History List disabled
    _0 = 0,
    ///1: TX History List enabled
    _1 = 1,
}
impl From<Thle> for bool {
    #[inline(always)]
    fn from(variant: Thle) -> Self {
        variant as u8 != 0
    }
}
///Field `THLE` reader - TX History List Enable
pub type ThleR = crate::BitReader<Thle>;
impl ThleR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Thle {
        match self.bits {
            false => Thle::_0,
            true => Thle::_1,
        }
    }
    ///TX History List disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Thle::_0
    }
    ///TX History List enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Thle::_1
    }
}
///Field `THLE` writer - TX History List Enable
pub type ThleW<'a, REG> = crate::BitWriter<'a, REG, Thle>;
impl<'a, REG> ThleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///TX History List disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Thle::_0)
    }
    ///TX History List enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Thle::_1)
    }
}
/**TX History List Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Thlie {
    ///0: TX History List Interrupt disabled
    _0 = 0,
    ///1: TX History List Interrupt enabled
    _1 = 1,
}
impl From<Thlie> for bool {
    #[inline(always)]
    fn from(variant: Thlie) -> Self {
        variant as u8 != 0
    }
}
///Field `THLIE` reader - TX History List Interrupt Enable
pub type ThlieR = crate::BitReader<Thlie>;
impl ThlieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Thlie {
        match self.bits {
            false => Thlie::_0,
            true => Thlie::_1,
        }
    }
    ///TX History List Interrupt disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Thlie::_0
    }
    ///TX History List Interrupt enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Thlie::_1
    }
}
///Field `THLIE` writer - TX History List Interrupt Enable
pub type ThlieW<'a, REG> = crate::BitWriter<'a, REG, Thlie>;
impl<'a, REG> ThlieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///TX History List Interrupt disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Thlie::_0)
    }
    ///TX History List Interrupt enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Thlie::_1)
    }
}
/**TX History List Interrupt Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Thlim {
    ///0: Interrupt generated if TX History List level reaches ¾ of the TX History List depth
    _0 = 0,
    ///1: Interrupt generated for every successfully stored entry
    _1 = 1,
}
impl From<Thlim> for bool {
    #[inline(always)]
    fn from(variant: Thlim) -> Self {
        variant as u8 != 0
    }
}
///Field `THLIM` reader - TX History List Interrupt Mode
pub type ThlimR = crate::BitReader<Thlim>;
impl ThlimR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Thlim {
        match self.bits {
            false => Thlim::_0,
            true => Thlim::_1,
        }
    }
    ///Interrupt generated if TX History List level reaches ¾ of the TX History List depth
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Thlim::_0
    }
    ///Interrupt generated for every successfully stored entry
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Thlim::_1
    }
}
///Field `THLIM` writer - TX History List Interrupt Mode
pub type ThlimW<'a, REG> = crate::BitWriter<'a, REG, Thlim>;
impl<'a, REG> ThlimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt generated if TX History List level reaches ¾ of the TX History List depth
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Thlim::_0)
    }
    ///Interrupt generated for every successfully stored entry
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Thlim::_1)
    }
}
/**TX History List Dedicated TX Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Thldte {
    ///0: TX FIFO + TX Queue
    _0 = 0,
    ///1: Flat TX MB + TX FIFO + TX Queue
    _1 = 1,
}
impl From<Thldte> for bool {
    #[inline(always)]
    fn from(variant: Thldte) -> Self {
        variant as u8 != 0
    }
}
///Field `THLDTE` reader - TX History List Dedicated TX Enable
pub type ThldteR = crate::BitReader<Thldte>;
impl ThldteR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Thldte {
        match self.bits {
            false => Thldte::_0,
            true => Thldte::_1,
        }
    }
    ///TX FIFO + TX Queue
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Thldte::_0
    }
    ///Flat TX MB + TX FIFO + TX Queue
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Thldte::_1
    }
}
///Field `THLDTE` writer - TX History List Dedicated TX Enable
pub type ThldteW<'a, REG> = crate::BitWriter<'a, REG, Thldte>;
impl<'a, REG> ThldteW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///TX FIFO + TX Queue
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Thldte::_0)
    }
    ///Flat TX MB + TX FIFO + TX Queue
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Thldte::_1)
    }
}
/**TX History List Dedicated Gateway Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Thldge {
    ///0: Not dedicated Gateway FIFO + Gateway TX Queue
    _0 = 0,
    ///1: Dedicated Gateway FIFO + Gateway TX Queue
    _1 = 1,
}
impl From<Thldge> for bool {
    #[inline(always)]
    fn from(variant: Thldge) -> Self {
        variant as u8 != 0
    }
}
///Field `THLDGE` reader - TX History List Dedicated Gateway Enable
pub type ThldgeR = crate::BitReader<Thldge>;
impl ThldgeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Thldge {
        match self.bits {
            false => Thldge::_0,
            true => Thldge::_1,
        }
    }
    ///Not dedicated Gateway FIFO + Gateway TX Queue
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Thldge::_0
    }
    ///Dedicated Gateway FIFO + Gateway TX Queue
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Thldge::_1
    }
}
///Field `THLDGE` writer - TX History List Dedicated Gateway Enable
pub type ThldgeW<'a, REG> = crate::BitWriter<'a, REG, Thldge>;
impl<'a, REG> ThldgeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Not dedicated Gateway FIFO + Gateway TX Queue
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Thldge::_0)
    }
    ///Dedicated Gateway FIFO + Gateway TX Queue
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Thldge::_1)
    }
}
impl R {
    ///Bit 0 - TX History List Enable
    #[inline(always)]
    pub fn thle(&self) -> ThleR {
        ThleR::new((self.bits & 1) != 0)
    }
    ///Bit 8 - TX History List Interrupt Enable
    #[inline(always)]
    pub fn thlie(&self) -> ThlieR {
        ThlieR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - TX History List Interrupt Mode
    #[inline(always)]
    pub fn thlim(&self) -> ThlimR {
        ThlimR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - TX History List Dedicated TX Enable
    #[inline(always)]
    pub fn thldte(&self) -> ThldteR {
        ThldteR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - TX History List Dedicated Gateway Enable
    #[inline(always)]
    pub fn thldge(&self) -> ThldgeR {
        ThldgeR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDTHLCC")
            .field("thle", &self.thle())
            .field("thlie", &self.thlie())
            .field("thlim", &self.thlim())
            .field("thldte", &self.thldte())
            .field("thldge", &self.thldge())
            .finish()
    }
}
impl W {
    ///Bit 0 - TX History List Enable
    #[inline(always)]
    pub fn thle(&mut self) -> ThleW<CfdthlccSpec> {
        ThleW::new(self, 0)
    }
    ///Bit 8 - TX History List Interrupt Enable
    #[inline(always)]
    pub fn thlie(&mut self) -> ThlieW<CfdthlccSpec> {
        ThlieW::new(self, 8)
    }
    ///Bit 9 - TX History List Interrupt Mode
    #[inline(always)]
    pub fn thlim(&mut self) -> ThlimW<CfdthlccSpec> {
        ThlimW::new(self, 9)
    }
    ///Bit 10 - TX History List Dedicated TX Enable
    #[inline(always)]
    pub fn thldte(&mut self) -> ThldteW<CfdthlccSpec> {
        ThldteW::new(self, 10)
    }
    ///Bit 11 - TX History List Dedicated Gateway Enable
    #[inline(always)]
    pub fn thldge(&mut self) -> ThldgeW<CfdthlccSpec> {
        ThldgeW::new(self, 11)
    }
}
/**TX History List Configuration/Control Register %s

You can [`read`](crate::Reg::read) this register and get [`cfdthlcc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdthlcc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdthlccSpec;
impl crate::RegisterSpec for CfdthlccSpec {
    type Ux = u32;
}
///`read()` method returns [`cfdthlcc::R`](R) reader structure
impl crate::Readable for CfdthlccSpec {}
///`write(|w| ..)` method takes [`cfdthlcc::W`](W) writer structure
impl crate::Writable for CfdthlccSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDTHLCC%s to value 0
impl crate::Resettable for CfdthlccSpec {}
