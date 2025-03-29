///Register `CFDCDTSTS` reader
pub type R = crate::R<CfdcdtstsSpec>;
/**DMA Transfer Status for RX FIFO 0

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfdmasts0 {
    ///0: DMA transfer stopped
    _0 = 0,
    ///1: DMA transfer on going
    _1 = 1,
}
impl From<Rfdmasts0> for bool {
    #[inline(always)]
    fn from(variant: Rfdmasts0) -> Self {
        variant as u8 != 0
    }
}
///Field `RFDMASTS0` reader - DMA Transfer Status for RX FIFO 0
pub type Rfdmasts0R = crate::BitReader<Rfdmasts0>;
impl Rfdmasts0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rfdmasts0 {
        match self.bits {
            false => Rfdmasts0::_0,
            true => Rfdmasts0::_1,
        }
    }
    ///DMA transfer stopped
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rfdmasts0::_0
    }
    ///DMA transfer on going
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rfdmasts0::_1
    }
}
/**DMA Transfer Status for RX FIFO 1

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfdmasts1 {
    ///0: DMA transfer stopped
    _0 = 0,
    ///1: DMA transfer on going
    _1 = 1,
}
impl From<Rfdmasts1> for bool {
    #[inline(always)]
    fn from(variant: Rfdmasts1) -> Self {
        variant as u8 != 0
    }
}
///Field `RFDMASTS1` reader - DMA Transfer Status for RX FIFO 1
pub type Rfdmasts1R = crate::BitReader<Rfdmasts1>;
impl Rfdmasts1R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rfdmasts1 {
        match self.bits {
            false => Rfdmasts1::_0,
            true => Rfdmasts1::_1,
        }
    }
    ///DMA transfer stopped
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rfdmasts1::_0
    }
    ///DMA transfer on going
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rfdmasts1::_1
    }
}
/**DMA Transfer Status for RX FIFO 2

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfdmasts2 {
    ///0: DMA transfer stopped
    _0 = 0,
    ///1: DMA transfer on going
    _1 = 1,
}
impl From<Rfdmasts2> for bool {
    #[inline(always)]
    fn from(variant: Rfdmasts2) -> Self {
        variant as u8 != 0
    }
}
///Field `RFDMASTS2` reader - DMA Transfer Status for RX FIFO 2
pub type Rfdmasts2R = crate::BitReader<Rfdmasts2>;
impl Rfdmasts2R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rfdmasts2 {
        match self.bits {
            false => Rfdmasts2::_0,
            true => Rfdmasts2::_1,
        }
    }
    ///DMA transfer stopped
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rfdmasts2::_0
    }
    ///DMA transfer on going
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rfdmasts2::_1
    }
}
/**DMA Transfer Status for RX FIFO 3

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfdmasts3 {
    ///0: DMA transfer stopped
    _0 = 0,
    ///1: DMA transfer on going
    _1 = 1,
}
impl From<Rfdmasts3> for bool {
    #[inline(always)]
    fn from(variant: Rfdmasts3) -> Self {
        variant as u8 != 0
    }
}
///Field `RFDMASTS3` reader - DMA Transfer Status for RX FIFO 3
pub type Rfdmasts3R = crate::BitReader<Rfdmasts3>;
impl Rfdmasts3R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rfdmasts3 {
        match self.bits {
            false => Rfdmasts3::_0,
            true => Rfdmasts3::_1,
        }
    }
    ///DMA transfer stopped
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rfdmasts3::_0
    }
    ///DMA transfer on going
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rfdmasts3::_1
    }
}
/**DMA Transfer Status for RX FIFO 4

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfdmasts4 {
    ///0: DMA transfer stopped
    _0 = 0,
    ///1: DMA transfer on going
    _1 = 1,
}
impl From<Rfdmasts4> for bool {
    #[inline(always)]
    fn from(variant: Rfdmasts4) -> Self {
        variant as u8 != 0
    }
}
///Field `RFDMASTS4` reader - DMA Transfer Status for RX FIFO 4
pub type Rfdmasts4R = crate::BitReader<Rfdmasts4>;
impl Rfdmasts4R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rfdmasts4 {
        match self.bits {
            false => Rfdmasts4::_0,
            true => Rfdmasts4::_1,
        }
    }
    ///DMA transfer stopped
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rfdmasts4::_0
    }
    ///DMA transfer on going
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rfdmasts4::_1
    }
}
/**DMA Transfer Status for RX FIFO 5

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfdmasts5 {
    ///0: DMA transfer stopped
    _0 = 0,
    ///1: DMA transfer on going
    _1 = 1,
}
impl From<Rfdmasts5> for bool {
    #[inline(always)]
    fn from(variant: Rfdmasts5) -> Self {
        variant as u8 != 0
    }
}
///Field `RFDMASTS5` reader - DMA Transfer Status for RX FIFO 5
pub type Rfdmasts5R = crate::BitReader<Rfdmasts5>;
impl Rfdmasts5R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rfdmasts5 {
        match self.bits {
            false => Rfdmasts5::_0,
            true => Rfdmasts5::_1,
        }
    }
    ///DMA transfer stopped
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rfdmasts5::_0
    }
    ///DMA transfer on going
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rfdmasts5::_1
    }
}
/**DMA Transfer Status for RX FIFO 6

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfdmasts6 {
    ///0: DMA transfer stopped
    _0 = 0,
    ///1: DMA transfer on going
    _1 = 1,
}
impl From<Rfdmasts6> for bool {
    #[inline(always)]
    fn from(variant: Rfdmasts6) -> Self {
        variant as u8 != 0
    }
}
///Field `RFDMASTS6` reader - DMA Transfer Status for RX FIFO 6
pub type Rfdmasts6R = crate::BitReader<Rfdmasts6>;
impl Rfdmasts6R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rfdmasts6 {
        match self.bits {
            false => Rfdmasts6::_0,
            true => Rfdmasts6::_1,
        }
    }
    ///DMA transfer stopped
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rfdmasts6::_0
    }
    ///DMA transfer on going
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rfdmasts6::_1
    }
}
/**DMA Transfer Status for RX FIFO 7

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfdmasts7 {
    ///0: DMA transfer stopped
    _0 = 0,
    ///1: DMA transfer on going
    _1 = 1,
}
impl From<Rfdmasts7> for bool {
    #[inline(always)]
    fn from(variant: Rfdmasts7) -> Self {
        variant as u8 != 0
    }
}
///Field `RFDMASTS7` reader - DMA Transfer Status for RX FIFO 7
pub type Rfdmasts7R = crate::BitReader<Rfdmasts7>;
impl Rfdmasts7R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rfdmasts7 {
        match self.bits {
            false => Rfdmasts7::_0,
            true => Rfdmasts7::_1,
        }
    }
    ///DMA transfer stopped
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rfdmasts7::_0
    }
    ///DMA transfer on going
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rfdmasts7::_1
    }
}
/**DMA Transfer Status only for Common FIFO 0 of Channel 0

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cfdmasts0 {
    ///0: DMA transfer stopped
    _0 = 0,
    ///1: DMA transfer on going
    _1 = 1,
}
impl From<Cfdmasts0> for bool {
    #[inline(always)]
    fn from(variant: Cfdmasts0) -> Self {
        variant as u8 != 0
    }
}
///Field `CFDMASTS0` reader - DMA Transfer Status only for Common FIFO 0 of Channel 0
pub type Cfdmasts0R = crate::BitReader<Cfdmasts0>;
impl Cfdmasts0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cfdmasts0 {
        match self.bits {
            false => Cfdmasts0::_0,
            true => Cfdmasts0::_1,
        }
    }
    ///DMA transfer stopped
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cfdmasts0::_0
    }
    ///DMA transfer on going
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cfdmasts0::_1
    }
}
/**DMA Transfer Status only for Common FIFO 0 of Channel 1

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cfdmasts1 {
    ///0: DMA transfer stopped
    _0 = 0,
    ///1: DMA transfer on going
    _1 = 1,
}
impl From<Cfdmasts1> for bool {
    #[inline(always)]
    fn from(variant: Cfdmasts1) -> Self {
        variant as u8 != 0
    }
}
///Field `CFDMASTS1` reader - DMA Transfer Status only for Common FIFO 0 of Channel 1
pub type Cfdmasts1R = crate::BitReader<Cfdmasts1>;
impl Cfdmasts1R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cfdmasts1 {
        match self.bits {
            false => Cfdmasts1::_0,
            true => Cfdmasts1::_1,
        }
    }
    ///DMA transfer stopped
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cfdmasts1::_0
    }
    ///DMA transfer on going
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cfdmasts1::_1
    }
}
impl R {
    ///Bit 0 - DMA Transfer Status for RX FIFO 0
    #[inline(always)]
    pub fn rfdmasts0(&self) -> Rfdmasts0R {
        Rfdmasts0R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DMA Transfer Status for RX FIFO 1
    #[inline(always)]
    pub fn rfdmasts1(&self) -> Rfdmasts1R {
        Rfdmasts1R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - DMA Transfer Status for RX FIFO 2
    #[inline(always)]
    pub fn rfdmasts2(&self) -> Rfdmasts2R {
        Rfdmasts2R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - DMA Transfer Status for RX FIFO 3
    #[inline(always)]
    pub fn rfdmasts3(&self) -> Rfdmasts3R {
        Rfdmasts3R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - DMA Transfer Status for RX FIFO 4
    #[inline(always)]
    pub fn rfdmasts4(&self) -> Rfdmasts4R {
        Rfdmasts4R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - DMA Transfer Status for RX FIFO 5
    #[inline(always)]
    pub fn rfdmasts5(&self) -> Rfdmasts5R {
        Rfdmasts5R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - DMA Transfer Status for RX FIFO 6
    #[inline(always)]
    pub fn rfdmasts6(&self) -> Rfdmasts6R {
        Rfdmasts6R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - DMA Transfer Status for RX FIFO 7
    #[inline(always)]
    pub fn rfdmasts7(&self) -> Rfdmasts7R {
        Rfdmasts7R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - DMA Transfer Status only for Common FIFO 0 of Channel 0
    #[inline(always)]
    pub fn cfdmasts0(&self) -> Cfdmasts0R {
        Cfdmasts0R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - DMA Transfer Status only for Common FIFO 0 of Channel 1
    #[inline(always)]
    pub fn cfdmasts1(&self) -> Cfdmasts1R {
        Cfdmasts1R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDCDTSTS")
            .field("rfdmasts0", &self.rfdmasts0())
            .field("rfdmasts1", &self.rfdmasts1())
            .field("rfdmasts2", &self.rfdmasts2())
            .field("rfdmasts3", &self.rfdmasts3())
            .field("rfdmasts4", &self.rfdmasts4())
            .field("rfdmasts5", &self.rfdmasts5())
            .field("rfdmasts6", &self.rfdmasts6())
            .field("rfdmasts7", &self.rfdmasts7())
            .field("cfdmasts0", &self.cfdmasts0())
            .field("cfdmasts1", &self.cfdmasts1())
            .finish()
    }
}
/**DMA Transfer Status Register

You can [`read`](crate::Reg::read) this register and get [`cfdcdtsts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdcdtstsSpec;
impl crate::RegisterSpec for CfdcdtstsSpec {
    type Ux = u32;
}
///`read()` method returns [`cfdcdtsts::R`](R) reader structure
impl crate::Readable for CfdcdtstsSpec {}
///`reset()` method sets CFDCDTSTS to value 0
impl crate::Resettable for CfdcdtstsSpec {}
