///Register `CFDCDTCT` reader
pub type R = crate::R<CfdcdtctSpec>;
///Register `CFDCDTCT` writer
pub type W = crate::W<CfdcdtctSpec>;
/**DMA Transfer Enable for RXFIFO 0

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfdmae0 {
    ///0: DMA transfer request disabled
    _0 = 0,
    ///1: DMA transfer request enabled
    _1 = 1,
}
impl From<Rfdmae0> for bool {
    #[inline(always)]
    fn from(variant: Rfdmae0) -> Self {
        variant as u8 != 0
    }
}
///Field `RFDMAE0` reader - DMA Transfer Enable for RXFIFO 0
pub type Rfdmae0R = crate::BitReader<Rfdmae0>;
impl Rfdmae0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rfdmae0 {
        match self.bits {
            false => Rfdmae0::_0,
            true => Rfdmae0::_1,
        }
    }
    ///DMA transfer request disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rfdmae0::_0
    }
    ///DMA transfer request enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rfdmae0::_1
    }
}
///Field `RFDMAE0` writer - DMA Transfer Enable for RXFIFO 0
pub type Rfdmae0W<'a, REG> = crate::BitWriter<'a, REG, Rfdmae0>;
impl<'a, REG> Rfdmae0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DMA transfer request disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rfdmae0::_0)
    }
    ///DMA transfer request enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rfdmae0::_1)
    }
}
/**DMA Transfer Enable for RXFIFO 1

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfdmae1 {
    ///0: DMA transfer request disabled
    _0 = 0,
    ///1: DMA transfer request enabled
    _1 = 1,
}
impl From<Rfdmae1> for bool {
    #[inline(always)]
    fn from(variant: Rfdmae1) -> Self {
        variant as u8 != 0
    }
}
///Field `RFDMAE1` reader - DMA Transfer Enable for RXFIFO 1
pub type Rfdmae1R = crate::BitReader<Rfdmae1>;
impl Rfdmae1R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rfdmae1 {
        match self.bits {
            false => Rfdmae1::_0,
            true => Rfdmae1::_1,
        }
    }
    ///DMA transfer request disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rfdmae1::_0
    }
    ///DMA transfer request enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rfdmae1::_1
    }
}
///Field `RFDMAE1` writer - DMA Transfer Enable for RXFIFO 1
pub type Rfdmae1W<'a, REG> = crate::BitWriter<'a, REG, Rfdmae1>;
impl<'a, REG> Rfdmae1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DMA transfer request disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rfdmae1::_0)
    }
    ///DMA transfer request enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rfdmae1::_1)
    }
}
/**DMA Transfer Enable for RXFIFO 2

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfdmae2 {
    ///0: DMA transfer request disabled
    _0 = 0,
    ///1: DMA transfer request enabled
    _1 = 1,
}
impl From<Rfdmae2> for bool {
    #[inline(always)]
    fn from(variant: Rfdmae2) -> Self {
        variant as u8 != 0
    }
}
///Field `RFDMAE2` reader - DMA Transfer Enable for RXFIFO 2
pub type Rfdmae2R = crate::BitReader<Rfdmae2>;
impl Rfdmae2R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rfdmae2 {
        match self.bits {
            false => Rfdmae2::_0,
            true => Rfdmae2::_1,
        }
    }
    ///DMA transfer request disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rfdmae2::_0
    }
    ///DMA transfer request enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rfdmae2::_1
    }
}
///Field `RFDMAE2` writer - DMA Transfer Enable for RXFIFO 2
pub type Rfdmae2W<'a, REG> = crate::BitWriter<'a, REG, Rfdmae2>;
impl<'a, REG> Rfdmae2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DMA transfer request disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rfdmae2::_0)
    }
    ///DMA transfer request enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rfdmae2::_1)
    }
}
/**DMA Transfer Enable for RXFIFO 3

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfdmae3 {
    ///0: DMA transfer request disabled
    _0 = 0,
    ///1: DMA transfer request enabled
    _1 = 1,
}
impl From<Rfdmae3> for bool {
    #[inline(always)]
    fn from(variant: Rfdmae3) -> Self {
        variant as u8 != 0
    }
}
///Field `RFDMAE3` reader - DMA Transfer Enable for RXFIFO 3
pub type Rfdmae3R = crate::BitReader<Rfdmae3>;
impl Rfdmae3R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rfdmae3 {
        match self.bits {
            false => Rfdmae3::_0,
            true => Rfdmae3::_1,
        }
    }
    ///DMA transfer request disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rfdmae3::_0
    }
    ///DMA transfer request enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rfdmae3::_1
    }
}
///Field `RFDMAE3` writer - DMA Transfer Enable for RXFIFO 3
pub type Rfdmae3W<'a, REG> = crate::BitWriter<'a, REG, Rfdmae3>;
impl<'a, REG> Rfdmae3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DMA transfer request disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rfdmae3::_0)
    }
    ///DMA transfer request enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rfdmae3::_1)
    }
}
/**DMA Transfer Enable for RXFIFO 4

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfdmae4 {
    ///0: DMA transfer request disabled
    _0 = 0,
    ///1: DMA transfer request enabled
    _1 = 1,
}
impl From<Rfdmae4> for bool {
    #[inline(always)]
    fn from(variant: Rfdmae4) -> Self {
        variant as u8 != 0
    }
}
///Field `RFDMAE4` reader - DMA Transfer Enable for RXFIFO 4
pub type Rfdmae4R = crate::BitReader<Rfdmae4>;
impl Rfdmae4R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rfdmae4 {
        match self.bits {
            false => Rfdmae4::_0,
            true => Rfdmae4::_1,
        }
    }
    ///DMA transfer request disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rfdmae4::_0
    }
    ///DMA transfer request enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rfdmae4::_1
    }
}
///Field `RFDMAE4` writer - DMA Transfer Enable for RXFIFO 4
pub type Rfdmae4W<'a, REG> = crate::BitWriter<'a, REG, Rfdmae4>;
impl<'a, REG> Rfdmae4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DMA transfer request disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rfdmae4::_0)
    }
    ///DMA transfer request enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rfdmae4::_1)
    }
}
/**DMA Transfer Enable for RXFIFO 5

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfdmae5 {
    ///0: DMA transfer request disabled
    _0 = 0,
    ///1: DMA transfer request enabled
    _1 = 1,
}
impl From<Rfdmae5> for bool {
    #[inline(always)]
    fn from(variant: Rfdmae5) -> Self {
        variant as u8 != 0
    }
}
///Field `RFDMAE5` reader - DMA Transfer Enable for RXFIFO 5
pub type Rfdmae5R = crate::BitReader<Rfdmae5>;
impl Rfdmae5R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rfdmae5 {
        match self.bits {
            false => Rfdmae5::_0,
            true => Rfdmae5::_1,
        }
    }
    ///DMA transfer request disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rfdmae5::_0
    }
    ///DMA transfer request enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rfdmae5::_1
    }
}
///Field `RFDMAE5` writer - DMA Transfer Enable for RXFIFO 5
pub type Rfdmae5W<'a, REG> = crate::BitWriter<'a, REG, Rfdmae5>;
impl<'a, REG> Rfdmae5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DMA transfer request disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rfdmae5::_0)
    }
    ///DMA transfer request enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rfdmae5::_1)
    }
}
/**DMA Transfer Enable for RXFIFO 6

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfdmae6 {
    ///0: DMA transfer request disabled
    _0 = 0,
    ///1: DMA transfer request enabled
    _1 = 1,
}
impl From<Rfdmae6> for bool {
    #[inline(always)]
    fn from(variant: Rfdmae6) -> Self {
        variant as u8 != 0
    }
}
///Field `RFDMAE6` reader - DMA Transfer Enable for RXFIFO 6
pub type Rfdmae6R = crate::BitReader<Rfdmae6>;
impl Rfdmae6R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rfdmae6 {
        match self.bits {
            false => Rfdmae6::_0,
            true => Rfdmae6::_1,
        }
    }
    ///DMA transfer request disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rfdmae6::_0
    }
    ///DMA transfer request enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rfdmae6::_1
    }
}
///Field `RFDMAE6` writer - DMA Transfer Enable for RXFIFO 6
pub type Rfdmae6W<'a, REG> = crate::BitWriter<'a, REG, Rfdmae6>;
impl<'a, REG> Rfdmae6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DMA transfer request disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rfdmae6::_0)
    }
    ///DMA transfer request enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rfdmae6::_1)
    }
}
/**DMA Transfer Enable for RXFIFO 7

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfdmae7 {
    ///0: DMA transfer request disabled
    _0 = 0,
    ///1: DMA transfer request enabled
    _1 = 1,
}
impl From<Rfdmae7> for bool {
    #[inline(always)]
    fn from(variant: Rfdmae7) -> Self {
        variant as u8 != 0
    }
}
///Field `RFDMAE7` reader - DMA Transfer Enable for RXFIFO 7
pub type Rfdmae7R = crate::BitReader<Rfdmae7>;
impl Rfdmae7R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rfdmae7 {
        match self.bits {
            false => Rfdmae7::_0,
            true => Rfdmae7::_1,
        }
    }
    ///DMA transfer request disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rfdmae7::_0
    }
    ///DMA transfer request enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rfdmae7::_1
    }
}
///Field `RFDMAE7` writer - DMA Transfer Enable for RXFIFO 7
pub type Rfdmae7W<'a, REG> = crate::BitWriter<'a, REG, Rfdmae7>;
impl<'a, REG> Rfdmae7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DMA transfer request disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rfdmae7::_0)
    }
    ///DMA transfer request enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rfdmae7::_1)
    }
}
/**DMA Transfer Enable for Common FIFO 0 of Channel 0

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cfdmae0 {
    ///0: DMA transfer request disabled
    _0 = 0,
    ///1: DMA transfer request enabled
    _1 = 1,
}
impl From<Cfdmae0> for bool {
    #[inline(always)]
    fn from(variant: Cfdmae0) -> Self {
        variant as u8 != 0
    }
}
///Field `CFDMAE0` reader - DMA Transfer Enable for Common FIFO 0 of Channel 0
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
    ///DMA transfer request disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cfdmae0::_0
    }
    ///DMA transfer request enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cfdmae0::_1
    }
}
///Field `CFDMAE0` writer - DMA Transfer Enable for Common FIFO 0 of Channel 0
pub type Cfdmae0W<'a, REG> = crate::BitWriter<'a, REG, Cfdmae0>;
impl<'a, REG> Cfdmae0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DMA transfer request disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cfdmae0::_0)
    }
    ///DMA transfer request enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cfdmae0::_1)
    }
}
/**DMA Transfer Enable for Common FIFO 0 of Channel 1

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cfdmae1 {
    ///0: DMA transfer request disabled
    _0 = 0,
    ///1: DMA transfer request enabled
    _1 = 1,
}
impl From<Cfdmae1> for bool {
    #[inline(always)]
    fn from(variant: Cfdmae1) -> Self {
        variant as u8 != 0
    }
}
///Field `CFDMAE1` reader - DMA Transfer Enable for Common FIFO 0 of Channel 1
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
    ///DMA transfer request disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cfdmae1::_0
    }
    ///DMA transfer request enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cfdmae1::_1
    }
}
///Field `CFDMAE1` writer - DMA Transfer Enable for Common FIFO 0 of Channel 1
pub type Cfdmae1W<'a, REG> = crate::BitWriter<'a, REG, Cfdmae1>;
impl<'a, REG> Cfdmae1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DMA transfer request disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cfdmae1::_0)
    }
    ///DMA transfer request enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cfdmae1::_1)
    }
}
impl R {
    ///Bit 0 - DMA Transfer Enable for RXFIFO 0
    #[inline(always)]
    pub fn rfdmae0(&self) -> Rfdmae0R {
        Rfdmae0R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DMA Transfer Enable for RXFIFO 1
    #[inline(always)]
    pub fn rfdmae1(&self) -> Rfdmae1R {
        Rfdmae1R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - DMA Transfer Enable for RXFIFO 2
    #[inline(always)]
    pub fn rfdmae2(&self) -> Rfdmae2R {
        Rfdmae2R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - DMA Transfer Enable for RXFIFO 3
    #[inline(always)]
    pub fn rfdmae3(&self) -> Rfdmae3R {
        Rfdmae3R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - DMA Transfer Enable for RXFIFO 4
    #[inline(always)]
    pub fn rfdmae4(&self) -> Rfdmae4R {
        Rfdmae4R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - DMA Transfer Enable for RXFIFO 5
    #[inline(always)]
    pub fn rfdmae5(&self) -> Rfdmae5R {
        Rfdmae5R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - DMA Transfer Enable for RXFIFO 6
    #[inline(always)]
    pub fn rfdmae6(&self) -> Rfdmae6R {
        Rfdmae6R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - DMA Transfer Enable for RXFIFO 7
    #[inline(always)]
    pub fn rfdmae7(&self) -> Rfdmae7R {
        Rfdmae7R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - DMA Transfer Enable for Common FIFO 0 of Channel 0
    #[inline(always)]
    pub fn cfdmae0(&self) -> Cfdmae0R {
        Cfdmae0R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - DMA Transfer Enable for Common FIFO 0 of Channel 1
    #[inline(always)]
    pub fn cfdmae1(&self) -> Cfdmae1R {
        Cfdmae1R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDCDTCT")
            .field("rfdmae0", &self.rfdmae0())
            .field("rfdmae1", &self.rfdmae1())
            .field("rfdmae2", &self.rfdmae2())
            .field("rfdmae3", &self.rfdmae3())
            .field("rfdmae4", &self.rfdmae4())
            .field("rfdmae5", &self.rfdmae5())
            .field("rfdmae6", &self.rfdmae6())
            .field("rfdmae7", &self.rfdmae7())
            .field("cfdmae0", &self.cfdmae0())
            .field("cfdmae1", &self.cfdmae1())
            .finish()
    }
}
impl W {
    ///Bit 0 - DMA Transfer Enable for RXFIFO 0
    #[inline(always)]
    pub fn rfdmae0(&mut self) -> Rfdmae0W<CfdcdtctSpec> {
        Rfdmae0W::new(self, 0)
    }
    ///Bit 1 - DMA Transfer Enable for RXFIFO 1
    #[inline(always)]
    pub fn rfdmae1(&mut self) -> Rfdmae1W<CfdcdtctSpec> {
        Rfdmae1W::new(self, 1)
    }
    ///Bit 2 - DMA Transfer Enable for RXFIFO 2
    #[inline(always)]
    pub fn rfdmae2(&mut self) -> Rfdmae2W<CfdcdtctSpec> {
        Rfdmae2W::new(self, 2)
    }
    ///Bit 3 - DMA Transfer Enable for RXFIFO 3
    #[inline(always)]
    pub fn rfdmae3(&mut self) -> Rfdmae3W<CfdcdtctSpec> {
        Rfdmae3W::new(self, 3)
    }
    ///Bit 4 - DMA Transfer Enable for RXFIFO 4
    #[inline(always)]
    pub fn rfdmae4(&mut self) -> Rfdmae4W<CfdcdtctSpec> {
        Rfdmae4W::new(self, 4)
    }
    ///Bit 5 - DMA Transfer Enable for RXFIFO 5
    #[inline(always)]
    pub fn rfdmae5(&mut self) -> Rfdmae5W<CfdcdtctSpec> {
        Rfdmae5W::new(self, 5)
    }
    ///Bit 6 - DMA Transfer Enable for RXFIFO 6
    #[inline(always)]
    pub fn rfdmae6(&mut self) -> Rfdmae6W<CfdcdtctSpec> {
        Rfdmae6W::new(self, 6)
    }
    ///Bit 7 - DMA Transfer Enable for RXFIFO 7
    #[inline(always)]
    pub fn rfdmae7(&mut self) -> Rfdmae7W<CfdcdtctSpec> {
        Rfdmae7W::new(self, 7)
    }
    ///Bit 8 - DMA Transfer Enable for Common FIFO 0 of Channel 0
    #[inline(always)]
    pub fn cfdmae0(&mut self) -> Cfdmae0W<CfdcdtctSpec> {
        Cfdmae0W::new(self, 8)
    }
    ///Bit 9 - DMA Transfer Enable for Common FIFO 0 of Channel 1
    #[inline(always)]
    pub fn cfdmae1(&mut self) -> Cfdmae1W<CfdcdtctSpec> {
        Cfdmae1W::new(self, 9)
    }
}
/**DMA Transfer Control Register

You can [`read`](crate::Reg::read) this register and get [`cfdcdtct::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcdtct::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdcdtctSpec;
impl crate::RegisterSpec for CfdcdtctSpec {
    type Ux = u32;
}
///`read()` method returns [`cfdcdtct::R`](R) reader structure
impl crate::Readable for CfdcdtctSpec {}
///`write(|w| ..)` method takes [`cfdcdtct::W`](W) writer structure
impl crate::Writable for CfdcdtctSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDCDTCT to value 0
impl crate::Resettable for CfdcdtctSpec {}
