///Register `CFDTXQSTS1%s` reader
pub type R = crate::R<Cfdtxqsts1Spec>;
///Register `CFDTXQSTS1%s` writer
pub type W = crate::W<Cfdtxqsts1Spec>;
/**TX Queue Empty

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txqemp {
    ///0: TX Queue not empty
    _0 = 0,
    ///1: TX Queue empty
    _1 = 1,
}
impl From<Txqemp> for bool {
    #[inline(always)]
    fn from(variant: Txqemp) -> Self {
        variant as u8 != 0
    }
}
///Field `TXQEMP` reader - TX Queue Empty
pub type TxqempR = crate::BitReader<Txqemp>;
impl TxqempR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Txqemp {
        match self.bits {
            false => Txqemp::_0,
            true => Txqemp::_1,
        }
    }
    ///TX Queue not empty
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Txqemp::_0
    }
    ///TX Queue empty
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Txqemp::_1
    }
}
/**TX Queue Full

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txqfll {
    ///0: TX Queue not full
    _0 = 0,
    ///1: TX Queue full
    _1 = 1,
}
impl From<Txqfll> for bool {
    #[inline(always)]
    fn from(variant: Txqfll) -> Self {
        variant as u8 != 0
    }
}
///Field `TXQFLL` reader - TX Queue Full
pub type TxqfllR = crate::BitReader<Txqfll>;
impl TxqfllR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Txqfll {
        match self.bits {
            false => Txqfll::_0,
            true => Txqfll::_1,
        }
    }
    ///TX Queue not full
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Txqfll::_0
    }
    ///TX Queue full
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Txqfll::_1
    }
}
/**TX Queue TX Interrupt Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txqtxif {
    ///0: TX Queue interrupt condition not satisfied after a frame TX
    _0 = 0,
    ///1: TX Queue interrupt condition satisfied after a frame TX
    _1 = 1,
}
impl From<Txqtxif> for bool {
    #[inline(always)]
    fn from(variant: Txqtxif) -> Self {
        variant as u8 != 0
    }
}
///Field `TXQTXIF` reader - TX Queue TX Interrupt Flag
pub type TxqtxifR = crate::BitReader<Txqtxif>;
impl TxqtxifR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Txqtxif {
        match self.bits {
            false => Txqtxif::_0,
            true => Txqtxif::_1,
        }
    }
    ///TX Queue interrupt condition not satisfied after a frame TX
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Txqtxif::_0
    }
    ///TX Queue interrupt condition satisfied after a frame TX
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Txqtxif::_1
    }
}
///Field `TXQTXIF` writer - TX Queue TX Interrupt Flag
pub type TxqtxifW<'a, REG> = crate::BitWriter<'a, REG, Txqtxif>;
impl<'a, REG> TxqtxifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///TX Queue interrupt condition not satisfied after a frame TX
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Txqtxif::_0)
    }
    ///TX Queue interrupt condition satisfied after a frame TX
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Txqtxif::_1)
    }
}
///Field `TXQMC` reader - TX Queue Message Count
pub type TxqmcR = crate::FieldReader;
///Field `TXQFIF` reader - TXQ Full Interrupt Flag
pub type TxqfifR = crate::BitReader;
///Field `TXQFIF` writer - TXQ Full Interrupt Flag
pub type TxqfifW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXQOFRXIF` reader - TXQ One Frame Reception Interrupt Flag
pub type TxqofrxifR = crate::BitReader;
///Field `TXQOFRXIF` writer - TXQ One Frame Reception Interrupt Flag
pub type TxqofrxifW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXQOFTXIF` reader - TXQ One Frame Transmission Interrupt Flag
pub type TxqoftxifR = crate::BitReader;
///Field `TXQOFTXIF` writer - TXQ One Frame Transmission Interrupt Flag
pub type TxqoftxifW<'a, REG> = crate::BitWriter<'a, REG>;
/**TXQ Message Lost

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txqmlt {
    ///0: No message lost in TXQ
    _0 = 0,
    ///1: TXQ message lost
    _1 = 1,
}
impl From<Txqmlt> for bool {
    #[inline(always)]
    fn from(variant: Txqmlt) -> Self {
        variant as u8 != 0
    }
}
///Field `TXQMLT` reader - TXQ Message Lost
pub type TxqmltR = crate::BitReader<Txqmlt>;
impl TxqmltR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Txqmlt {
        match self.bits {
            false => Txqmlt::_0,
            true => Txqmlt::_1,
        }
    }
    ///No message lost in TXQ
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Txqmlt::_0
    }
    ///TXQ message lost
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Txqmlt::_1
    }
}
///Field `TXQMLT` writer - TXQ Message Lost
pub type TxqmltW<'a, REG> = crate::BitWriter<'a, REG, Txqmlt>;
impl<'a, REG> TxqmltW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No message lost in TXQ
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Txqmlt::_0)
    }
    ///TXQ message lost
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Txqmlt::_1)
    }
}
impl R {
    ///Bit 0 - TX Queue Empty
    #[inline(always)]
    pub fn txqemp(&self) -> TxqempR {
        TxqempR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TX Queue Full
    #[inline(always)]
    pub fn txqfll(&self) -> TxqfllR {
        TxqfllR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TX Queue TX Interrupt Flag
    #[inline(always)]
    pub fn txqtxif(&self) -> TxqtxifR {
        TxqtxifR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 8:13 - TX Queue Message Count
    #[inline(always)]
    pub fn txqmc(&self) -> TxqmcR {
        TxqmcR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    ///Bit 16 - TXQ Full Interrupt Flag
    #[inline(always)]
    pub fn txqfif(&self) -> TxqfifR {
        TxqfifR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - TXQ One Frame Reception Interrupt Flag
    #[inline(always)]
    pub fn txqofrxif(&self) -> TxqofrxifR {
        TxqofrxifR::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - TXQ One Frame Transmission Interrupt Flag
    #[inline(always)]
    pub fn txqoftxif(&self) -> TxqoftxifR {
        TxqoftxifR::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - TXQ Message Lost
    #[inline(always)]
    pub fn txqmlt(&self) -> TxqmltR {
        TxqmltR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDTXQSTS1")
            .field("txqemp", &self.txqemp())
            .field("txqfll", &self.txqfll())
            .field("txqtxif", &self.txqtxif())
            .field("txqmc", &self.txqmc())
            .field("txqfif", &self.txqfif())
            .field("txqofrxif", &self.txqofrxif())
            .field("txqoftxif", &self.txqoftxif())
            .field("txqmlt", &self.txqmlt())
            .finish()
    }
}
impl W {
    ///Bit 2 - TX Queue TX Interrupt Flag
    #[inline(always)]
    pub fn txqtxif(&mut self) -> TxqtxifW<Cfdtxqsts1Spec> {
        TxqtxifW::new(self, 2)
    }
    ///Bit 16 - TXQ Full Interrupt Flag
    #[inline(always)]
    pub fn txqfif(&mut self) -> TxqfifW<Cfdtxqsts1Spec> {
        TxqfifW::new(self, 16)
    }
    ///Bit 17 - TXQ One Frame Reception Interrupt Flag
    #[inline(always)]
    pub fn txqofrxif(&mut self) -> TxqofrxifW<Cfdtxqsts1Spec> {
        TxqofrxifW::new(self, 17)
    }
    ///Bit 18 - TXQ One Frame Transmission Interrupt Flag
    #[inline(always)]
    pub fn txqoftxif(&mut self) -> TxqoftxifW<Cfdtxqsts1Spec> {
        TxqoftxifW::new(self, 18)
    }
    ///Bit 19 - TXQ Message Lost
    #[inline(always)]
    pub fn txqmlt(&mut self) -> TxqmltW<Cfdtxqsts1Spec> {
        TxqmltW::new(self, 19)
    }
}
/**TX Queue Status Registers 1%s

You can [`read`](crate::Reg::read) this register and get [`cfdtxqsts1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtxqsts1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Cfdtxqsts1Spec;
impl crate::RegisterSpec for Cfdtxqsts1Spec {
    type Ux = u32;
}
///`read()` method returns [`cfdtxqsts1::R`](R) reader structure
impl crate::Readable for Cfdtxqsts1Spec {}
///`write(|w| ..)` method takes [`cfdtxqsts1::W`](W) writer structure
impl crate::Writable for Cfdtxqsts1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDTXQSTS1%s to value 0x01
impl crate::Resettable for Cfdtxqsts1Spec {
    const RESET_VALUE: u32 = 0x01;
}
