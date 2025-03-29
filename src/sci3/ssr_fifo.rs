///Register `SSR_FIFO` reader
pub type R = crate::R<SsrFifoSpec>;
///Register `SSR_FIFO` writer
pub type W = crate::W<SsrFifoSpec>;
/**Receive Data Ready Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dr {
    ///0: Receiving is in progress, or no received data remains in FRDRHL after successfully completed reception (receive FIFO empty)
    _0 = 0,
    ///1: Next receive data is not received for a period after normal receiving is complete, when the amount of data stored in the FIFO is equal to or less than the receive triggering number
    _1 = 1,
}
impl From<Dr> for bool {
    #[inline(always)]
    fn from(variant: Dr) -> Self {
        variant as u8 != 0
    }
}
///Field `DR` reader - Receive Data Ready Flag
pub type DrR = crate::BitReader<Dr>;
impl DrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dr {
        match self.bits {
            false => Dr::_0,
            true => Dr::_1,
        }
    }
    ///Receiving is in progress, or no received data remains in FRDRHL after successfully completed reception (receive FIFO empty)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dr::_0
    }
    ///Next receive data is not received for a period after normal receiving is complete, when the amount of data stored in the FIFO is equal to or less than the receive triggering number
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dr::_1
    }
}
///Field `DR` writer - Receive Data Ready Flag
pub type DrW<'a, REG> = crate::BitWriter<'a, REG, Dr>;
impl<'a, REG> DrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Receiving is in progress, or no received data remains in FRDRHL after successfully completed reception (receive FIFO empty)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dr::_0)
    }
    ///Next receive data is not received for a period after normal receiving is complete, when the amount of data stored in the FIFO is equal to or less than the receive triggering number
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dr::_1)
    }
}
/**Transmit End Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tend {
    ///0: A character is being transmitted
    _0 = 0,
    ///1: Character transfer is complete
    _1 = 1,
}
impl From<Tend> for bool {
    #[inline(always)]
    fn from(variant: Tend) -> Self {
        variant as u8 != 0
    }
}
///Field `TEND` reader - Transmit End Flag
pub type TendR = crate::BitReader<Tend>;
impl TendR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tend {
        match self.bits {
            false => Tend::_0,
            true => Tend::_1,
        }
    }
    ///A character is being transmitted
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tend::_0
    }
    ///Character transfer is complete
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tend::_1
    }
}
///Field `TEND` writer - Transmit End Flag
pub type TendW<'a, REG> = crate::BitWriter<'a, REG, Tend>;
impl<'a, REG> TendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A character is being transmitted
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tend::_0)
    }
    ///Character transfer is complete
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tend::_1)
    }
}
/**Parity Error Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Per {
    ///0: No parity error occurred
    _0 = 0,
    ///1: Parity error occurred
    _1 = 1,
}
impl From<Per> for bool {
    #[inline(always)]
    fn from(variant: Per) -> Self {
        variant as u8 != 0
    }
}
///Field `PER` reader - Parity Error Flag
pub type PerR = crate::BitReader<Per>;
impl PerR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Per {
        match self.bits {
            false => Per::_0,
            true => Per::_1,
        }
    }
    ///No parity error occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Per::_0
    }
    ///Parity error occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Per::_1
    }
}
///Field `PER` writer - Parity Error Flag
pub type PerW<'a, REG> = crate::BitWriter<'a, REG, Per>;
impl<'a, REG> PerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No parity error occurred
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Per::_0)
    }
    ///Parity error occurred
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Per::_1)
    }
}
/**Framing Error Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fer {
    ///0: No framing error occurred
    _0 = 0,
    ///1: Framing error occurred
    _1 = 1,
}
impl From<Fer> for bool {
    #[inline(always)]
    fn from(variant: Fer) -> Self {
        variant as u8 != 0
    }
}
///Field `FER` reader - Framing Error Flag
pub type FerR = crate::BitReader<Fer>;
impl FerR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Fer {
        match self.bits {
            false => Fer::_0,
            true => Fer::_1,
        }
    }
    ///No framing error occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Fer::_0
    }
    ///Framing error occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Fer::_1
    }
}
///Field `FER` writer - Framing Error Flag
pub type FerW<'a, REG> = crate::BitWriter<'a, REG, Fer>;
impl<'a, REG> FerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No framing error occurred
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Fer::_0)
    }
    ///Framing error occurred
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Fer::_1)
    }
}
/**Overrun Error Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Orer {
    ///0: No overrun error occurred
    _0 = 0,
    ///1: Overrun error occurred
    _1 = 1,
}
impl From<Orer> for bool {
    #[inline(always)]
    fn from(variant: Orer) -> Self {
        variant as u8 != 0
    }
}
///Field `ORER` reader - Overrun Error Flag
pub type OrerR = crate::BitReader<Orer>;
impl OrerR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Orer {
        match self.bits {
            false => Orer::_0,
            true => Orer::_1,
        }
    }
    ///No overrun error occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Orer::_0
    }
    ///Overrun error occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Orer::_1
    }
}
///Field `ORER` writer - Overrun Error Flag
pub type OrerW<'a, REG> = crate::BitWriter<'a, REG, Orer>;
impl<'a, REG> OrerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No overrun error occurred
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Orer::_0)
    }
    ///Overrun error occurred
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Orer::_1)
    }
}
/**Receive FIFO Data Full Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdf {
    ///0: The amount of receive data written in FRDRHL is less than the specified receive triggering number
    _0 = 0,
    ///1: The amount of receive data written in FRDRHL is equal to or greater than the specified receive triggering number
    _1 = 1,
}
impl From<Rdf> for bool {
    #[inline(always)]
    fn from(variant: Rdf) -> Self {
        variant as u8 != 0
    }
}
///Field `RDF` reader - Receive FIFO Data Full Flag
pub type RdfR = crate::BitReader<Rdf>;
impl RdfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rdf {
        match self.bits {
            false => Rdf::_0,
            true => Rdf::_1,
        }
    }
    ///The amount of receive data written in FRDRHL is less than the specified receive triggering number
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rdf::_0
    }
    ///The amount of receive data written in FRDRHL is equal to or greater than the specified receive triggering number
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rdf::_1
    }
}
///Field `RDF` writer - Receive FIFO Data Full Flag
pub type RdfW<'a, REG> = crate::BitWriter<'a, REG, Rdf>;
impl<'a, REG> RdfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The amount of receive data written in FRDRHL is less than the specified receive triggering number
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rdf::_0)
    }
    ///The amount of receive data written in FRDRHL is equal to or greater than the specified receive triggering number
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rdf::_1)
    }
}
/**Transmit FIFO Data Empty Flag

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tdfe {
    ///0: The amount of transmit data written in FTDRHL exceeds the specified transmit triggering number
    _0 = 0,
    ///1: The amount of transmit data written in FTDRHL is equal to or less than the specified transmit triggering number
    _1 = 1,
}
impl From<Tdfe> for bool {
    #[inline(always)]
    fn from(variant: Tdfe) -> Self {
        variant as u8 != 0
    }
}
///Field `TDFE` reader - Transmit FIFO Data Empty Flag
pub type TdfeR = crate::BitReader<Tdfe>;
impl TdfeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tdfe {
        match self.bits {
            false => Tdfe::_0,
            true => Tdfe::_1,
        }
    }
    ///The amount of transmit data written in FTDRHL exceeds the specified transmit triggering number
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tdfe::_0
    }
    ///The amount of transmit data written in FTDRHL is equal to or less than the specified transmit triggering number
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tdfe::_1
    }
}
///Field `TDFE` writer - Transmit FIFO Data Empty Flag
pub type TdfeW<'a, REG> = crate::BitWriter<'a, REG, Tdfe>;
impl<'a, REG> TdfeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The amount of transmit data written in FTDRHL exceeds the specified transmit triggering number
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tdfe::_0)
    }
    ///The amount of transmit data written in FTDRHL is equal to or less than the specified transmit triggering number
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tdfe::_1)
    }
}
impl R {
    ///Bit 0 - Receive Data Ready Flag
    #[inline(always)]
    pub fn dr(&self) -> DrR {
        DrR::new((self.bits & 1) != 0)
    }
    ///Bit 2 - Transmit End Flag
    #[inline(always)]
    pub fn tend(&self) -> TendR {
        TendR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Parity Error Flag
    #[inline(always)]
    pub fn per(&self) -> PerR {
        PerR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Framing Error Flag
    #[inline(always)]
    pub fn fer(&self) -> FerR {
        FerR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Overrun Error Flag
    #[inline(always)]
    pub fn orer(&self) -> OrerR {
        OrerR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Receive FIFO Data Full Flag
    #[inline(always)]
    pub fn rdf(&self) -> RdfR {
        RdfR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Transmit FIFO Data Empty Flag
    #[inline(always)]
    pub fn tdfe(&self) -> TdfeR {
        TdfeR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SSR_FIFO")
            .field("dr", &self.dr())
            .field("tend", &self.tend())
            .field("per", &self.per())
            .field("fer", &self.fer())
            .field("orer", &self.orer())
            .field("rdf", &self.rdf())
            .field("tdfe", &self.tdfe())
            .finish()
    }
}
impl W {
    ///Bit 0 - Receive Data Ready Flag
    #[inline(always)]
    pub fn dr(&mut self) -> DrW<SsrFifoSpec> {
        DrW::new(self, 0)
    }
    ///Bit 2 - Transmit End Flag
    #[inline(always)]
    pub fn tend(&mut self) -> TendW<SsrFifoSpec> {
        TendW::new(self, 2)
    }
    ///Bit 3 - Parity Error Flag
    #[inline(always)]
    pub fn per(&mut self) -> PerW<SsrFifoSpec> {
        PerW::new(self, 3)
    }
    ///Bit 4 - Framing Error Flag
    #[inline(always)]
    pub fn fer(&mut self) -> FerW<SsrFifoSpec> {
        FerW::new(self, 4)
    }
    ///Bit 5 - Overrun Error Flag
    #[inline(always)]
    pub fn orer(&mut self) -> OrerW<SsrFifoSpec> {
        OrerW::new(self, 5)
    }
    ///Bit 6 - Receive FIFO Data Full Flag
    #[inline(always)]
    pub fn rdf(&mut self) -> RdfW<SsrFifoSpec> {
        RdfW::new(self, 6)
    }
    ///Bit 7 - Transmit FIFO Data Empty Flag
    #[inline(always)]
    pub fn tdfe(&mut self) -> TdfeW<SsrFifoSpec> {
        TdfeW::new(self, 7)
    }
}
/**Serial Status Register for Non-Smart Card Interface and FIFO Mode (SCMR.SMIF = 0, FCR.FM = 1, and MMR.MANEN = 0)

You can [`read`](crate::Reg::read) this register and get [`ssr_fifo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssr_fifo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SsrFifoSpec;
impl crate::RegisterSpec for SsrFifoSpec {
    type Ux = u8;
}
///`read()` method returns [`ssr_fifo::R`](R) reader structure
impl crate::Readable for SsrFifoSpec {}
///`write(|w| ..)` method takes [`ssr_fifo::W`](W) writer structure
impl crate::Writable for SsrFifoSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SSR_FIFO to value 0x80
impl crate::Resettable for SsrFifoSpec {
    const RESET_VALUE: u8 = 0x80;
}
