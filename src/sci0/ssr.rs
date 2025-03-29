///Register `SSR` reader
pub type R = crate::R<SsrSpec>;
///Register `SSR` writer
pub type W = crate::W<SsrSpec>;
/**Multi-Processor Bit Transfer

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mpbt {
    ///0: Data transmission cycle
    _0 = 0,
    ///1: ID transmission cycle
    _1 = 1,
}
impl From<Mpbt> for bool {
    #[inline(always)]
    fn from(variant: Mpbt) -> Self {
        variant as u8 != 0
    }
}
///Field `MPBT` reader - Multi-Processor Bit Transfer
pub type MpbtR = crate::BitReader<Mpbt>;
impl MpbtR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mpbt {
        match self.bits {
            false => Mpbt::_0,
            true => Mpbt::_1,
        }
    }
    ///Data transmission cycle
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mpbt::_0
    }
    ///ID transmission cycle
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mpbt::_1
    }
}
///Field `MPBT` writer - Multi-Processor Bit Transfer
pub type MpbtW<'a, REG> = crate::BitWriter<'a, REG, Mpbt>;
impl<'a, REG> MpbtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Data transmission cycle
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mpbt::_0)
    }
    ///ID transmission cycle
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mpbt::_1)
    }
}
/**Multi-Processor

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mpb {
    ///0: Data transmission cycle
    _0 = 0,
    ///1: ID transmission cycle
    _1 = 1,
}
impl From<Mpb> for bool {
    #[inline(always)]
    fn from(variant: Mpb) -> Self {
        variant as u8 != 0
    }
}
///Field `MPB` reader - Multi-Processor
pub type MpbR = crate::BitReader<Mpb>;
impl MpbR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mpb {
        match self.bits {
            false => Mpb::_0,
            true => Mpb::_1,
        }
    }
    ///Data transmission cycle
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mpb::_0
    }
    ///ID transmission cycle
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mpb::_1
    }
}
/**Transmit End Flag

Value on reset: 1*/
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
/**Receive Data Full Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdrf {
    ///0: No received data in RDR register
    _0 = 0,
    ///1: Received data in RDR register
    _1 = 1,
}
impl From<Rdrf> for bool {
    #[inline(always)]
    fn from(variant: Rdrf) -> Self {
        variant as u8 != 0
    }
}
///Field `RDRF` reader - Receive Data Full Flag
pub type RdrfR = crate::BitReader<Rdrf>;
impl RdrfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rdrf {
        match self.bits {
            false => Rdrf::_0,
            true => Rdrf::_1,
        }
    }
    ///No received data in RDR register
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rdrf::_0
    }
    ///Received data in RDR register
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rdrf::_1
    }
}
///Field `RDRF` writer - Receive Data Full Flag
pub type RdrfW<'a, REG> = crate::BitWriter<'a, REG, Rdrf>;
impl<'a, REG> RdrfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No received data in RDR register
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rdrf::_0)
    }
    ///Received data in RDR register
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rdrf::_1)
    }
}
/**Transmit Data Empty Flag

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tdre {
    ///0: Transmit data in TDR register
    _0 = 0,
    ///1: No transmit data in TDR register
    _1 = 1,
}
impl From<Tdre> for bool {
    #[inline(always)]
    fn from(variant: Tdre) -> Self {
        variant as u8 != 0
    }
}
///Field `TDRE` reader - Transmit Data Empty Flag
pub type TdreR = crate::BitReader<Tdre>;
impl TdreR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tdre {
        match self.bits {
            false => Tdre::_0,
            true => Tdre::_1,
        }
    }
    ///Transmit data in TDR register
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tdre::_0
    }
    ///No transmit data in TDR register
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tdre::_1
    }
}
///Field `TDRE` writer - Transmit Data Empty Flag
pub type TdreW<'a, REG> = crate::BitWriter<'a, REG, Tdre>;
impl<'a, REG> TdreW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Transmit data in TDR register
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tdre::_0)
    }
    ///No transmit data in TDR register
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tdre::_1)
    }
}
impl R {
    ///Bit 0 - Multi-Processor Bit Transfer
    #[inline(always)]
    pub fn mpbt(&self) -> MpbtR {
        MpbtR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Multi-Processor
    #[inline(always)]
    pub fn mpb(&self) -> MpbR {
        MpbR::new(((self.bits >> 1) & 1) != 0)
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
    ///Bit 6 - Receive Data Full Flag
    #[inline(always)]
    pub fn rdrf(&self) -> RdrfR {
        RdrfR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Transmit Data Empty Flag
    #[inline(always)]
    pub fn tdre(&self) -> TdreR {
        TdreR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SSR")
            .field("mpbt", &self.mpbt())
            .field("mpb", &self.mpb())
            .field("tend", &self.tend())
            .field("per", &self.per())
            .field("fer", &self.fer())
            .field("orer", &self.orer())
            .field("rdrf", &self.rdrf())
            .field("tdre", &self.tdre())
            .finish()
    }
}
impl W {
    ///Bit 0 - Multi-Processor Bit Transfer
    #[inline(always)]
    pub fn mpbt(&mut self) -> MpbtW<SsrSpec> {
        MpbtW::new(self, 0)
    }
    ///Bit 3 - Parity Error Flag
    #[inline(always)]
    pub fn per(&mut self) -> PerW<SsrSpec> {
        PerW::new(self, 3)
    }
    ///Bit 4 - Framing Error Flag
    #[inline(always)]
    pub fn fer(&mut self) -> FerW<SsrSpec> {
        FerW::new(self, 4)
    }
    ///Bit 5 - Overrun Error Flag
    #[inline(always)]
    pub fn orer(&mut self) -> OrerW<SsrSpec> {
        OrerW::new(self, 5)
    }
    ///Bit 6 - Receive Data Full Flag
    #[inline(always)]
    pub fn rdrf(&mut self) -> RdrfW<SsrSpec> {
        RdrfW::new(self, 6)
    }
    ///Bit 7 - Transmit Data Empty Flag
    #[inline(always)]
    pub fn tdre(&mut self) -> TdreW<SsrSpec> {
        TdreW::new(self, 7)
    }
}
/**Serial Status Register for Non-Smart Card Interface and Non-FIFO Mode (SCMR.SMIF = 0, FCR.FM = 0, and MMR.MANEN = 0)

You can [`read`](crate::Reg::read) this register and get [`ssr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SsrSpec;
impl crate::RegisterSpec for SsrSpec {
    type Ux = u8;
}
///`read()` method returns [`ssr::R`](R) reader structure
impl crate::Readable for SsrSpec {}
///`write(|w| ..)` method takes [`ssr::W`](W) writer structure
impl crate::Writable for SsrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SSR to value 0x84
impl crate::Resettable for SsrSpec {
    const RESET_VALUE: u8 = 0x84;
}
