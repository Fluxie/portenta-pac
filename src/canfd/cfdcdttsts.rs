///Register `CFDCDTTSTS` reader
pub type R = crate::R<CfdcdttstsSpec>;
///Register `CFDCDTTSTS` writer
pub type W = crate::W<CfdcdttstsSpec>;
/**DMA TX Transfer Status for TXQ0 of Channel 0

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tq0dmasts0 {
    ///0: DMA transfer stopped
    _0 = 0,
    ///1: DMA transfer enabled
    _1 = 1,
}
impl From<Tq0dmasts0> for bool {
    #[inline(always)]
    fn from(variant: Tq0dmasts0) -> Self {
        variant as u8 != 0
    }
}
///Field `TQ0DMASTS0` reader - DMA TX Transfer Status for TXQ0 of Channel 0
pub type Tq0dmasts0R = crate::BitReader<Tq0dmasts0>;
impl Tq0dmasts0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tq0dmasts0 {
        match self.bits {
            false => Tq0dmasts0::_0,
            true => Tq0dmasts0::_1,
        }
    }
    ///DMA transfer stopped
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tq0dmasts0::_0
    }
    ///DMA transfer enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tq0dmasts0::_1
    }
}
/**DMA TX Transfer Status for TXQ0 of Channel 1

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tq0dmasts1 {
    ///0: DMA transfer stopped
    _0 = 0,
    ///1: DMA transfer enabled
    _1 = 1,
}
impl From<Tq0dmasts1> for bool {
    #[inline(always)]
    fn from(variant: Tq0dmasts1) -> Self {
        variant as u8 != 0
    }
}
///Field `TQ0DMASTS1` reader - DMA TX Transfer Status for TXQ0 of Channel 1
pub type Tq0dmasts1R = crate::BitReader<Tq0dmasts1>;
impl Tq0dmasts1R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tq0dmasts1 {
        match self.bits {
            false => Tq0dmasts1::_0,
            true => Tq0dmasts1::_1,
        }
    }
    ///DMA transfer stopped
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tq0dmasts1::_0
    }
    ///DMA transfer enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tq0dmasts1::_1
    }
}
/**DMA TX Transfer Status for TXQ3 of Channel 0

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tq3dmasts0 {
    ///0: DMA transfer stopped
    _0 = 0,
    ///1: DMA transfer enabled
    _1 = 1,
}
impl From<Tq3dmasts0> for bool {
    #[inline(always)]
    fn from(variant: Tq3dmasts0) -> Self {
        variant as u8 != 0
    }
}
///Field `TQ3DMASTS0` reader - DMA TX Transfer Status for TXQ3 of Channel 0
pub type Tq3dmasts0R = crate::BitReader<Tq3dmasts0>;
impl Tq3dmasts0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tq3dmasts0 {
        match self.bits {
            false => Tq3dmasts0::_0,
            true => Tq3dmasts0::_1,
        }
    }
    ///DMA transfer stopped
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tq3dmasts0::_0
    }
    ///DMA transfer enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tq3dmasts0::_1
    }
}
/**DMA TX Transfer Status for TXQ3 of Channel 1

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tq3dmasts1 {
    ///0: DMA transfer stopped
    _0 = 0,
    ///1: DMA transfer enabled
    _1 = 1,
}
impl From<Tq3dmasts1> for bool {
    #[inline(always)]
    fn from(variant: Tq3dmasts1) -> Self {
        variant as u8 != 0
    }
}
///Field `TQ3DMASTS1` reader - DMA TX Transfer Status for TXQ3 of Channel 1
pub type Tq3dmasts1R = crate::BitReader<Tq3dmasts1>;
impl Tq3dmasts1R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tq3dmasts1 {
        match self.bits {
            false => Tq3dmasts1::_0,
            true => Tq3dmasts1::_1,
        }
    }
    ///DMA transfer stopped
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tq3dmasts1::_0
    }
    ///DMA transfer enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tq3dmasts1::_1
    }
}
/**DMA TX Transfer Status only for Common FIFO 2 of Channel 0

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cfdmasts0 {
    ///0: DMA transfer stopped
    _0 = 0,
    ///1: DMA transfer enabled
    _1 = 1,
}
impl From<Cfdmasts0> for bool {
    #[inline(always)]
    fn from(variant: Cfdmasts0) -> Self {
        variant as u8 != 0
    }
}
///Field `CFDMASTS0` reader - DMA TX Transfer Status only for Common FIFO 2 of Channel 0
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
    ///DMA transfer enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cfdmasts0::_1
    }
}
/**DMA TX Transfer Status only for Common FIFO 2 of Channel 1

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cfdmasts1 {
    ///0: DMA transfer stopped
    _0 = 0,
    ///1: DMA transfer enabled
    _1 = 1,
}
impl From<Cfdmasts1> for bool {
    #[inline(always)]
    fn from(variant: Cfdmasts1) -> Self {
        variant as u8 != 0
    }
}
///Field `CFDMASTS1` reader - DMA TX Transfer Status only for Common FIFO 2 of Channel 1
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
    ///DMA transfer enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cfdmasts1::_1
    }
}
impl R {
    ///Bit 0 - DMA TX Transfer Status for TXQ0 of Channel 0
    #[inline(always)]
    pub fn tq0dmasts0(&self) -> Tq0dmasts0R {
        Tq0dmasts0R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DMA TX Transfer Status for TXQ0 of Channel 1
    #[inline(always)]
    pub fn tq0dmasts1(&self) -> Tq0dmasts1R {
        Tq0dmasts1R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 8 - DMA TX Transfer Status for TXQ3 of Channel 0
    #[inline(always)]
    pub fn tq3dmasts0(&self) -> Tq3dmasts0R {
        Tq3dmasts0R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - DMA TX Transfer Status for TXQ3 of Channel 1
    #[inline(always)]
    pub fn tq3dmasts1(&self) -> Tq3dmasts1R {
        Tq3dmasts1R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 16 - DMA TX Transfer Status only for Common FIFO 2 of Channel 0
    #[inline(always)]
    pub fn cfdmasts0(&self) -> Cfdmasts0R {
        Cfdmasts0R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - DMA TX Transfer Status only for Common FIFO 2 of Channel 1
    #[inline(always)]
    pub fn cfdmasts1(&self) -> Cfdmasts1R {
        Cfdmasts1R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDCDTTSTS")
            .field("tq0dmasts0", &self.tq0dmasts0())
            .field("tq0dmasts1", &self.tq0dmasts1())
            .field("tq3dmasts0", &self.tq3dmasts0())
            .field("tq3dmasts1", &self.tq3dmasts1())
            .field("cfdmasts0", &self.cfdmasts0())
            .field("cfdmasts1", &self.cfdmasts1())
            .finish()
    }
}
impl W {}
/**DMA TX Transfer Status Register

You can [`read`](crate::Reg::read) this register and get [`cfdcdttsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcdttsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdcdttstsSpec;
impl crate::RegisterSpec for CfdcdttstsSpec {
    type Ux = u32;
}
///`read()` method returns [`cfdcdttsts::R`](R) reader structure
impl crate::Readable for CfdcdttstsSpec {}
///`write(|w| ..)` method takes [`cfdcdttsts::W`](W) writer structure
impl crate::Writable for CfdcdttstsSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDCDTTSTS to value 0
impl crate::Resettable for CfdcdttstsSpec {}
