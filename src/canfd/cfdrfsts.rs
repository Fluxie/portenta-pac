///Register `CFDRFSTS%s` reader
pub type R = crate::R<CfdrfstsSpec>;
///Register `CFDRFSTS%s` writer
pub type W = crate::W<CfdrfstsSpec>;
/**RX FIFO Empty

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfemp {
    ///0: FIFO not empty
    _0 = 0,
    ///1: FIFO empty
    _1 = 1,
}
impl From<Rfemp> for bool {
    #[inline(always)]
    fn from(variant: Rfemp) -> Self {
        variant as u8 != 0
    }
}
///Field `RFEMP` reader - RX FIFO Empty
pub type RfempR = crate::BitReader<Rfemp>;
impl RfempR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rfemp {
        match self.bits {
            false => Rfemp::_0,
            true => Rfemp::_1,
        }
    }
    ///FIFO not empty
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rfemp::_0
    }
    ///FIFO empty
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rfemp::_1
    }
}
/**RX FIFO Full

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rffll {
    ///0: FIFO not full
    _0 = 0,
    ///1: FIFO full
    _1 = 1,
}
impl From<Rffll> for bool {
    #[inline(always)]
    fn from(variant: Rffll) -> Self {
        variant as u8 != 0
    }
}
///Field `RFFLL` reader - RX FIFO Full
pub type RffllR = crate::BitReader<Rffll>;
impl RffllR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rffll {
        match self.bits {
            false => Rffll::_0,
            true => Rffll::_1,
        }
    }
    ///FIFO not full
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rffll::_0
    }
    ///FIFO full
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rffll::_1
    }
}
/**RX FIFO Message Lost

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfmlt {
    ///0: No message lost in FIFO
    _0 = 0,
    ///1: FIFO message lost
    _1 = 1,
}
impl From<Rfmlt> for bool {
    #[inline(always)]
    fn from(variant: Rfmlt) -> Self {
        variant as u8 != 0
    }
}
///Field `RFMLT` reader - RX FIFO Message Lost
pub type RfmltR = crate::BitReader<Rfmlt>;
impl RfmltR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rfmlt {
        match self.bits {
            false => Rfmlt::_0,
            true => Rfmlt::_1,
        }
    }
    ///No message lost in FIFO
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rfmlt::_0
    }
    ///FIFO message lost
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rfmlt::_1
    }
}
///Field `RFMLT` writer - RX FIFO Message Lost
pub type RfmltW<'a, REG> = crate::BitWriter<'a, REG, Rfmlt>;
impl<'a, REG> RfmltW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No message lost in FIFO
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rfmlt::_0)
    }
    ///FIFO message lost
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rfmlt::_1)
    }
}
/**RX FIFO Interrupt Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfif {
    ///0: FIFO interrupt condition not satisfied
    _0 = 0,
    ///1: FIFO interrupt condition satisfied
    _1 = 1,
}
impl From<Rfif> for bool {
    #[inline(always)]
    fn from(variant: Rfif) -> Self {
        variant as u8 != 0
    }
}
///Field `RFIF` reader - RX FIFO Interrupt Flag
pub type RfifR = crate::BitReader<Rfif>;
impl RfifR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rfif {
        match self.bits {
            false => Rfif::_0,
            true => Rfif::_1,
        }
    }
    ///FIFO interrupt condition not satisfied
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rfif::_0
    }
    ///FIFO interrupt condition satisfied
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rfif::_1
    }
}
///Field `RFIF` writer - RX FIFO Interrupt Flag
pub type RfifW<'a, REG> = crate::BitWriter<'a, REG, Rfif>;
impl<'a, REG> RfifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///FIFO interrupt condition not satisfied
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rfif::_0)
    }
    ///FIFO interrupt condition satisfied
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rfif::_1)
    }
}
///Field `RFMC` reader - RX FIFO Message Count
pub type RfmcR = crate::FieldReader;
/**RX FIFO Full Interrupt Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rffif {
    ///0: FIFO full interrupt condition not satisfied
    _0 = 0,
    ///1: FIFO full interrupt condition satisfied
    _1 = 1,
}
impl From<Rffif> for bool {
    #[inline(always)]
    fn from(variant: Rffif) -> Self {
        variant as u8 != 0
    }
}
///Field `RFFIF` reader - RX FIFO Full Interrupt Flag
pub type RffifR = crate::BitReader<Rffif>;
impl RffifR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rffif {
        match self.bits {
            false => Rffif::_0,
            true => Rffif::_1,
        }
    }
    ///FIFO full interrupt condition not satisfied
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rffif::_0
    }
    ///FIFO full interrupt condition satisfied
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rffif::_1
    }
}
///Field `RFFIF` writer - RX FIFO Full Interrupt Flag
pub type RffifW<'a, REG> = crate::BitWriter<'a, REG, Rffif>;
impl<'a, REG> RffifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///FIFO full interrupt condition not satisfied
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rffif::_0)
    }
    ///FIFO full interrupt condition satisfied
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rffif::_1)
    }
}
impl R {
    ///Bit 0 - RX FIFO Empty
    #[inline(always)]
    pub fn rfemp(&self) -> RfempR {
        RfempR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - RX FIFO Full
    #[inline(always)]
    pub fn rffll(&self) -> RffllR {
        RffllR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - RX FIFO Message Lost
    #[inline(always)]
    pub fn rfmlt(&self) -> RfmltR {
        RfmltR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - RX FIFO Interrupt Flag
    #[inline(always)]
    pub fn rfif(&self) -> RfifR {
        RfifR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 8:15 - RX FIFO Message Count
    #[inline(always)]
    pub fn rfmc(&self) -> RfmcR {
        RfmcR::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bit 16 - RX FIFO Full Interrupt Flag
    #[inline(always)]
    pub fn rffif(&self) -> RffifR {
        RffifR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDRFSTS")
            .field("rfemp", &self.rfemp())
            .field("rffll", &self.rffll())
            .field("rfmlt", &self.rfmlt())
            .field("rfif", &self.rfif())
            .field("rfmc", &self.rfmc())
            .field("rffif", &self.rffif())
            .finish()
    }
}
impl W {
    ///Bit 2 - RX FIFO Message Lost
    #[inline(always)]
    pub fn rfmlt(&mut self) -> RfmltW<CfdrfstsSpec> {
        RfmltW::new(self, 2)
    }
    ///Bit 3 - RX FIFO Interrupt Flag
    #[inline(always)]
    pub fn rfif(&mut self) -> RfifW<CfdrfstsSpec> {
        RfifW::new(self, 3)
    }
    ///Bit 16 - RX FIFO Full Interrupt Flag
    #[inline(always)]
    pub fn rffif(&mut self) -> RffifW<CfdrfstsSpec> {
        RffifW::new(self, 16)
    }
}
/**RX FIFO Status Registers %s

You can [`read`](crate::Reg::read) this register and get [`cfdrfsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdrfsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdrfstsSpec;
impl crate::RegisterSpec for CfdrfstsSpec {
    type Ux = u32;
}
///`read()` method returns [`cfdrfsts::R`](R) reader structure
impl crate::Readable for CfdrfstsSpec {}
///`write(|w| ..)` method takes [`cfdrfsts::W`](W) writer structure
impl crate::Writable for CfdrfstsSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDRFSTS%s to value 0x01
impl crate::Resettable for CfdrfstsSpec {
    const RESET_VALUE: u32 = 0x01;
}
