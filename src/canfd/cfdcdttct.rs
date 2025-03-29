///Register `CFDCDTTCT` reader
pub type R = crate::R<CfdcdttctSpec>;
///Register `CFDCDTTCT` writer
pub type W = crate::W<CfdcdttctSpec>;
/**DMA TX Transfer Enable for TXQ 0 of Channel 0

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tq0dmae0 {
    ///0: DMA TX transfer request disabled
    _0 = 0,
    ///1: DMA TX transfer request enabled
    _1 = 1,
}
impl From<Tq0dmae0> for bool {
    #[inline(always)]
    fn from(variant: Tq0dmae0) -> Self {
        variant as u8 != 0
    }
}
///Field `TQ0DMAE0` reader - DMA TX Transfer Enable for TXQ 0 of Channel 0
pub type Tq0dmae0R = crate::BitReader<Tq0dmae0>;
impl Tq0dmae0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tq0dmae0 {
        match self.bits {
            false => Tq0dmae0::_0,
            true => Tq0dmae0::_1,
        }
    }
    ///DMA TX transfer request disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tq0dmae0::_0
    }
    ///DMA TX transfer request enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tq0dmae0::_1
    }
}
///Field `TQ0DMAE0` writer - DMA TX Transfer Enable for TXQ 0 of Channel 0
pub type Tq0dmae0W<'a, REG> = crate::BitWriter<'a, REG, Tq0dmae0>;
impl<'a, REG> Tq0dmae0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DMA TX transfer request disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tq0dmae0::_0)
    }
    ///DMA TX transfer request enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tq0dmae0::_1)
    }
}
/**DMA TX Transfer Enable for TXQ 0 of Channel 1

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tq0dmae1 {
    ///0: DMA TX transfer request disabled
    _0 = 0,
    ///1: DMA TX transfer request enabled
    _1 = 1,
}
impl From<Tq0dmae1> for bool {
    #[inline(always)]
    fn from(variant: Tq0dmae1) -> Self {
        variant as u8 != 0
    }
}
///Field `TQ0DMAE1` reader - DMA TX Transfer Enable for TXQ 0 of Channel 1
pub type Tq0dmae1R = crate::BitReader<Tq0dmae1>;
impl Tq0dmae1R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tq0dmae1 {
        match self.bits {
            false => Tq0dmae1::_0,
            true => Tq0dmae1::_1,
        }
    }
    ///DMA TX transfer request disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tq0dmae1::_0
    }
    ///DMA TX transfer request enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tq0dmae1::_1
    }
}
///Field `TQ0DMAE1` writer - DMA TX Transfer Enable for TXQ 0 of Channel 1
pub type Tq0dmae1W<'a, REG> = crate::BitWriter<'a, REG, Tq0dmae1>;
impl<'a, REG> Tq0dmae1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DMA TX transfer request disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tq0dmae1::_0)
    }
    ///DMA TX transfer request enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tq0dmae1::_1)
    }
}
/**DMA TX Transfer Enable for TXQ 3 of Channel 0

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tq3dmae0 {
    ///0: DMA TX transfer request disabled
    _0 = 0,
    ///1: DMA TX transfer request enabled
    _1 = 1,
}
impl From<Tq3dmae0> for bool {
    #[inline(always)]
    fn from(variant: Tq3dmae0) -> Self {
        variant as u8 != 0
    }
}
///Field `TQ3DMAE0` reader - DMA TX Transfer Enable for TXQ 3 of Channel 0
pub type Tq3dmae0R = crate::BitReader<Tq3dmae0>;
impl Tq3dmae0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tq3dmae0 {
        match self.bits {
            false => Tq3dmae0::_0,
            true => Tq3dmae0::_1,
        }
    }
    ///DMA TX transfer request disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tq3dmae0::_0
    }
    ///DMA TX transfer request enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tq3dmae0::_1
    }
}
///Field `TQ3DMAE0` writer - DMA TX Transfer Enable for TXQ 3 of Channel 0
pub type Tq3dmae0W<'a, REG> = crate::BitWriter<'a, REG, Tq3dmae0>;
impl<'a, REG> Tq3dmae0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DMA TX transfer request disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tq3dmae0::_0)
    }
    ///DMA TX transfer request enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tq3dmae0::_1)
    }
}
/**DMA TX Transfer Enable for TXQ 3 of Channel 1

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tq3dmae1 {
    ///0: DMA TX transfer request disabled
    _0 = 0,
    ///1: DMA TX transfer request enabled
    _1 = 1,
}
impl From<Tq3dmae1> for bool {
    #[inline(always)]
    fn from(variant: Tq3dmae1) -> Self {
        variant as u8 != 0
    }
}
///Field `TQ3DMAE1` reader - DMA TX Transfer Enable for TXQ 3 of Channel 1
pub type Tq3dmae1R = crate::BitReader<Tq3dmae1>;
impl Tq3dmae1R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tq3dmae1 {
        match self.bits {
            false => Tq3dmae1::_0,
            true => Tq3dmae1::_1,
        }
    }
    ///DMA TX transfer request disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tq3dmae1::_0
    }
    ///DMA TX transfer request enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tq3dmae1::_1
    }
}
///Field `TQ3DMAE1` writer - DMA TX Transfer Enable for TXQ 3 of Channel 1
pub type Tq3dmae1W<'a, REG> = crate::BitWriter<'a, REG, Tq3dmae1>;
impl<'a, REG> Tq3dmae1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DMA TX transfer request disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tq3dmae1::_0)
    }
    ///DMA TX transfer request enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tq3dmae1::_1)
    }
}
/**DMA TX Transfer Enable for Common FIFO 2 of Channel 0

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cfdmae0 {
    ///0: DMA TX transfer request disabled
    _0 = 0,
    ///1: DMA TX transfer request enabled
    _1 = 1,
}
impl From<Cfdmae0> for bool {
    #[inline(always)]
    fn from(variant: Cfdmae0) -> Self {
        variant as u8 != 0
    }
}
///Field `CFDMAE0` reader - DMA TX Transfer Enable for Common FIFO 2 of Channel 0
pub type Cfdmae0R = crate::BitReader<Cfdmae0>;
impl Cfdmae0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cfdmae0 {
        match self.bits {
            false => Cfdmae0::_0,
            true => Cfdmae0::_1,
        }
    }
    ///DMA TX transfer request disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cfdmae0::_0
    }
    ///DMA TX transfer request enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cfdmae0::_1
    }
}
///Field `CFDMAE0` writer - DMA TX Transfer Enable for Common FIFO 2 of Channel 0
pub type Cfdmae0W<'a, REG> = crate::BitWriter<'a, REG, Cfdmae0>;
impl<'a, REG> Cfdmae0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DMA TX transfer request disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cfdmae0::_0)
    }
    ///DMA TX transfer request enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cfdmae0::_1)
    }
}
/**DMA TX Transfer Enable for Common FIFO 2 of Channel 1

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cfdmae1 {
    ///0: DMA TX transfer request disabled
    _0 = 0,
    ///1: DMA TX transfer request enabled
    _1 = 1,
}
impl From<Cfdmae1> for bool {
    #[inline(always)]
    fn from(variant: Cfdmae1) -> Self {
        variant as u8 != 0
    }
}
///Field `CFDMAE1` reader - DMA TX Transfer Enable for Common FIFO 2 of Channel 1
pub type Cfdmae1R = crate::BitReader<Cfdmae1>;
impl Cfdmae1R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cfdmae1 {
        match self.bits {
            false => Cfdmae1::_0,
            true => Cfdmae1::_1,
        }
    }
    ///DMA TX transfer request disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cfdmae1::_0
    }
    ///DMA TX transfer request enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cfdmae1::_1
    }
}
///Field `CFDMAE1` writer - DMA TX Transfer Enable for Common FIFO 2 of Channel 1
pub type Cfdmae1W<'a, REG> = crate::BitWriter<'a, REG, Cfdmae1>;
impl<'a, REG> Cfdmae1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DMA TX transfer request disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cfdmae1::_0)
    }
    ///DMA TX transfer request enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cfdmae1::_1)
    }
}
impl R {
    ///Bit 0 - DMA TX Transfer Enable for TXQ 0 of Channel 0
    #[inline(always)]
    pub fn tq0dmae0(&self) -> Tq0dmae0R {
        Tq0dmae0R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DMA TX Transfer Enable for TXQ 0 of Channel 1
    #[inline(always)]
    pub fn tq0dmae1(&self) -> Tq0dmae1R {
        Tq0dmae1R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 8 - DMA TX Transfer Enable for TXQ 3 of Channel 0
    #[inline(always)]
    pub fn tq3dmae0(&self) -> Tq3dmae0R {
        Tq3dmae0R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - DMA TX Transfer Enable for TXQ 3 of Channel 1
    #[inline(always)]
    pub fn tq3dmae1(&self) -> Tq3dmae1R {
        Tq3dmae1R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 16 - DMA TX Transfer Enable for Common FIFO 2 of Channel 0
    #[inline(always)]
    pub fn cfdmae0(&self) -> Cfdmae0R {
        Cfdmae0R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - DMA TX Transfer Enable for Common FIFO 2 of Channel 1
    #[inline(always)]
    pub fn cfdmae1(&self) -> Cfdmae1R {
        Cfdmae1R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDCDTTCT")
            .field("tq0dmae0", &self.tq0dmae0())
            .field("tq0dmae1", &self.tq0dmae1())
            .field("tq3dmae0", &self.tq3dmae0())
            .field("tq3dmae1", &self.tq3dmae1())
            .field("cfdmae0", &self.cfdmae0())
            .field("cfdmae1", &self.cfdmae1())
            .finish()
    }
}
impl W {
    ///Bit 0 - DMA TX Transfer Enable for TXQ 0 of Channel 0
    #[inline(always)]
    pub fn tq0dmae0(&mut self) -> Tq0dmae0W<CfdcdttctSpec> {
        Tq0dmae0W::new(self, 0)
    }
    ///Bit 1 - DMA TX Transfer Enable for TXQ 0 of Channel 1
    #[inline(always)]
    pub fn tq0dmae1(&mut self) -> Tq0dmae1W<CfdcdttctSpec> {
        Tq0dmae1W::new(self, 1)
    }
    ///Bit 8 - DMA TX Transfer Enable for TXQ 3 of Channel 0
    #[inline(always)]
    pub fn tq3dmae0(&mut self) -> Tq3dmae0W<CfdcdttctSpec> {
        Tq3dmae0W::new(self, 8)
    }
    ///Bit 9 - DMA TX Transfer Enable for TXQ 3 of Channel 1
    #[inline(always)]
    pub fn tq3dmae1(&mut self) -> Tq3dmae1W<CfdcdttctSpec> {
        Tq3dmae1W::new(self, 9)
    }
    ///Bit 16 - DMA TX Transfer Enable for Common FIFO 2 of Channel 0
    #[inline(always)]
    pub fn cfdmae0(&mut self) -> Cfdmae0W<CfdcdttctSpec> {
        Cfdmae0W::new(self, 16)
    }
    ///Bit 17 - DMA TX Transfer Enable for Common FIFO 2 of Channel 1
    #[inline(always)]
    pub fn cfdmae1(&mut self) -> Cfdmae1W<CfdcdttctSpec> {
        Cfdmae1W::new(self, 17)
    }
}
/**DMA TX Transfer Control Register

You can [`read`](crate::Reg::read) this register and get [`cfdcdttct::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcdttct::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdcdttctSpec;
impl crate::RegisterSpec for CfdcdttctSpec {
    type Ux = u32;
}
///`read()` method returns [`cfdcdttct::R`](R) reader structure
impl crate::Readable for CfdcdttctSpec {}
///`write(|w| ..)` method takes [`cfdcdttct::W`](W) writer structure
impl crate::Writable for CfdcdttctSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDCDTTCT to value 0
impl crate::Resettable for CfdcdttctSpec {}
