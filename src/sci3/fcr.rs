///Register `FCR` reader
pub type R = crate::R<FcrSpec>;
///Register `FCR` writer
pub type W = crate::W<FcrSpec>;
/**FIFO Mode Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fm {
    ///0: Non-FIFO mode. Selects TDR/RDR or TDRHL/RDRHL for communication.
    _0 = 0,
    ///1: FIFO mode. Selects FTDRHL/FRDRHL for communication.
    _1 = 1,
}
impl From<Fm> for bool {
    #[inline(always)]
    fn from(variant: Fm) -> Self {
        variant as u8 != 0
    }
}
///Field `FM` reader - FIFO Mode Select
pub type FmR = crate::BitReader<Fm>;
impl FmR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Fm {
        match self.bits {
            false => Fm::_0,
            true => Fm::_1,
        }
    }
    ///Non-FIFO mode. Selects TDR/RDR or TDRHL/RDRHL for communication.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Fm::_0
    }
    ///FIFO mode. Selects FTDRHL/FRDRHL for communication.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Fm::_1
    }
}
///Field `FM` writer - FIFO Mode Select
pub type FmW<'a, REG> = crate::BitWriter<'a, REG, Fm>;
impl<'a, REG> FmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Non-FIFO mode. Selects TDR/RDR or TDRHL/RDRHL for communication.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Fm::_0)
    }
    ///FIFO mode. Selects FTDRHL/FRDRHL for communication.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Fm::_1)
    }
}
/**Receive FIFO Data Register Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfrst {
    ///0: Do not reset FRDRHL
    _0 = 0,
    ///1: Reset FRDRHL
    _1 = 1,
}
impl From<Rfrst> for bool {
    #[inline(always)]
    fn from(variant: Rfrst) -> Self {
        variant as u8 != 0
    }
}
///Field `RFRST` reader - Receive FIFO Data Register Reset
pub type RfrstR = crate::BitReader<Rfrst>;
impl RfrstR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rfrst {
        match self.bits {
            false => Rfrst::_0,
            true => Rfrst::_1,
        }
    }
    ///Do not reset FRDRHL
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rfrst::_0
    }
    ///Reset FRDRHL
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rfrst::_1
    }
}
///Field `RFRST` writer - Receive FIFO Data Register Reset
pub type RfrstW<'a, REG> = crate::BitWriter<'a, REG, Rfrst>;
impl<'a, REG> RfrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not reset FRDRHL
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rfrst::_0)
    }
    ///Reset FRDRHL
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rfrst::_1)
    }
}
/**Transmit FIFO Data Register Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tfrst {
    ///0: Do not reset FTDRHL
    _0 = 0,
    ///1: Reset FTDRHL
    _1 = 1,
}
impl From<Tfrst> for bool {
    #[inline(always)]
    fn from(variant: Tfrst) -> Self {
        variant as u8 != 0
    }
}
///Field `TFRST` reader - Transmit FIFO Data Register Reset
pub type TfrstR = crate::BitReader<Tfrst>;
impl TfrstR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tfrst {
        match self.bits {
            false => Tfrst::_0,
            true => Tfrst::_1,
        }
    }
    ///Do not reset FTDRHL
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tfrst::_0
    }
    ///Reset FTDRHL
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tfrst::_1
    }
}
///Field `TFRST` writer - Transmit FIFO Data Register Reset
pub type TfrstW<'a, REG> = crate::BitWriter<'a, REG, Tfrst>;
impl<'a, REG> TfrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not reset FTDRHL
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tfrst::_0)
    }
    ///Reset FTDRHL
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tfrst::_1)
    }
}
/**Receive Data Ready Error Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dres {
    ///0: Receive data full interrupt (SCIn_RXI)
    _0 = 0,
    ///1: Receive error interrupt (SCIn_ERI)
    _1 = 1,
}
impl From<Dres> for bool {
    #[inline(always)]
    fn from(variant: Dres) -> Self {
        variant as u8 != 0
    }
}
///Field `DRES` reader - Receive Data Ready Error Select
pub type DresR = crate::BitReader<Dres>;
impl DresR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dres {
        match self.bits {
            false => Dres::_0,
            true => Dres::_1,
        }
    }
    ///Receive data full interrupt (SCIn_RXI)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dres::_0
    }
    ///Receive error interrupt (SCIn_ERI)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dres::_1
    }
}
///Field `DRES` writer - Receive Data Ready Error Select
pub type DresW<'a, REG> = crate::BitWriter<'a, REG, Dres>;
impl<'a, REG> DresW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Receive data full interrupt (SCIn_RXI)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dres::_0)
    }
    ///Receive error interrupt (SCIn_ERI)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dres::_1)
    }
}
///Field `TTRG` reader - Transmit FIFO Data Trigger Number
pub type TtrgR = crate::FieldReader;
///Field `TTRG` writer - Transmit FIFO Data Trigger Number
pub type TtrgW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `RTRG` reader - Receive FIFO Data Trigger Number
pub type RtrgR = crate::FieldReader;
///Field `RTRG` writer - Receive FIFO Data Trigger Number
pub type RtrgW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `RSTRG` reader - RTS Output Active Trigger Number Select
pub type RstrgR = crate::FieldReader;
///Field `RSTRG` writer - RTS Output Active Trigger Number Select
pub type RstrgW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bit 0 - FIFO Mode Select
    #[inline(always)]
    pub fn fm(&self) -> FmR {
        FmR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Receive FIFO Data Register Reset
    #[inline(always)]
    pub fn rfrst(&self) -> RfrstR {
        RfrstR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Transmit FIFO Data Register Reset
    #[inline(always)]
    pub fn tfrst(&self) -> TfrstR {
        TfrstR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Receive Data Ready Error Select
    #[inline(always)]
    pub fn dres(&self) -> DresR {
        DresR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:7 - Transmit FIFO Data Trigger Number
    #[inline(always)]
    pub fn ttrg(&self) -> TtrgR {
        TtrgR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - Receive FIFO Data Trigger Number
    #[inline(always)]
    pub fn rtrg(&self) -> RtrgR {
        RtrgR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - RTS Output Active Trigger Number Select
    #[inline(always)]
    pub fn rstrg(&self) -> RstrgR {
        RstrgR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCR")
            .field("fm", &self.fm())
            .field("rfrst", &self.rfrst())
            .field("tfrst", &self.tfrst())
            .field("dres", &self.dres())
            .field("ttrg", &self.ttrg())
            .field("rtrg", &self.rtrg())
            .field("rstrg", &self.rstrg())
            .finish()
    }
}
impl W {
    ///Bit 0 - FIFO Mode Select
    #[inline(always)]
    pub fn fm(&mut self) -> FmW<FcrSpec> {
        FmW::new(self, 0)
    }
    ///Bit 1 - Receive FIFO Data Register Reset
    #[inline(always)]
    pub fn rfrst(&mut self) -> RfrstW<FcrSpec> {
        RfrstW::new(self, 1)
    }
    ///Bit 2 - Transmit FIFO Data Register Reset
    #[inline(always)]
    pub fn tfrst(&mut self) -> TfrstW<FcrSpec> {
        TfrstW::new(self, 2)
    }
    ///Bit 3 - Receive Data Ready Error Select
    #[inline(always)]
    pub fn dres(&mut self) -> DresW<FcrSpec> {
        DresW::new(self, 3)
    }
    ///Bits 4:7 - Transmit FIFO Data Trigger Number
    #[inline(always)]
    pub fn ttrg(&mut self) -> TtrgW<FcrSpec> {
        TtrgW::new(self, 4)
    }
    ///Bits 8:11 - Receive FIFO Data Trigger Number
    #[inline(always)]
    pub fn rtrg(&mut self) -> RtrgW<FcrSpec> {
        RtrgW::new(self, 8)
    }
    ///Bits 12:15 - RTS Output Active Trigger Number Select
    #[inline(always)]
    pub fn rstrg(&mut self) -> RstrgW<FcrSpec> {
        RstrgW::new(self, 12)
    }
}
/**FIFO Control Register

You can [`read`](crate::Reg::read) this register and get [`fcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FcrSpec;
impl crate::RegisterSpec for FcrSpec {
    type Ux = u16;
}
///`read()` method returns [`fcr::R`](R) reader structure
impl crate::Readable for FcrSpec {}
///`write(|w| ..)` method takes [`fcr::W`](W) writer structure
impl crate::Writable for FcrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FCR to value 0xf800
impl crate::Resettable for FcrSpec {
    const RESET_VALUE: u16 = 0xf800;
}
