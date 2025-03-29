///Register `SSR_MANC` reader
pub type R = crate::R<SsrMancSpec>;
///Register `SSR_MANC` writer
pub type W = crate::W<SsrMancSpec>;
/**Manchester Error Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mer {
    ///0: No Manchester error occurred
    _0 = 0,
    ///1: Manchester error has occurred
    _1 = 1,
}
impl From<Mer> for bool {
    #[inline(always)]
    fn from(variant: Mer) -> Self {
        variant as u8 != 0
    }
}
///Field `MER` reader - Manchester Error Flag
pub type MerR = crate::BitReader<Mer>;
impl MerR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mer {
        match self.bits {
            false => Mer::_0,
            true => Mer::_1,
        }
    }
    ///No Manchester error occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mer::_0
    }
    ///Manchester error has occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mer::_1
    }
}
///Field `MER` writer - Manchester Error Flag
pub type MerW<'a, REG> = crate::BitWriter<'a, REG, Mer>;
impl<'a, REG> MerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No Manchester error occurred
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mer::_0)
    }
    ///Manchester error has occurred
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mer::_1)
    }
}
/**Multi-Processor

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mpb {
    ///0: Data transmission cycles
    _0 = 0,
    ///1: ID transmission cycles
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
    ///Data transmission cycles
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mpb::_0
    }
    ///ID transmission cycles
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
    ///1: Character transfer has been completed.
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
    ///Character transfer has been completed.
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
    ///1: A parity error has occurred
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
    ///A parity error has occurred
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
    ///A parity error has occurred
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
    ///1: A framing error has occurred
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
    ///A framing error has occurred
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
    ///A framing error has occurred
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
    ///1: An overrun error has occurred
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
    ///An overrun error has occurred
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
    ///An overrun error has occurred
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Orer::_1)
    }
}
/**Receive Data Full Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdrf {
    ///0: No received data is in RDR register
    _0 = 0,
    ///1: Received data is in RDR register
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
    ///No received data is in RDR register
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rdrf::_0
    }
    ///Received data is in RDR register
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
    ///No received data is in RDR register
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rdrf::_0)
    }
    ///Received data is in RDR register
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rdrf::_1)
    }
}
/**Transmit Data Empty Flag

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tdre {
    ///0: Transmit data is in TDR register
    _0 = 0,
    ///1: No transmit data is in TDR register
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
    ///Transmit data is in TDR register
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tdre::_0
    }
    ///No transmit data is in TDR register
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
    ///Transmit data is in TDR register
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tdre::_0)
    }
    ///No transmit data is in TDR register
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tdre::_1)
    }
}
impl R {
    ///Bit 0 - Manchester Error Flag
    #[inline(always)]
    pub fn mer(&self) -> MerR {
        MerR::new((self.bits & 1) != 0)
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
        f.debug_struct("SSR_MANC")
            .field("mer", &self.mer())
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
    ///Bit 0 - Manchester Error Flag
    #[inline(always)]
    pub fn mer(&mut self) -> MerW<SsrMancSpec> {
        MerW::new(self, 0)
    }
    ///Bit 3 - Parity Error Flag
    #[inline(always)]
    pub fn per(&mut self) -> PerW<SsrMancSpec> {
        PerW::new(self, 3)
    }
    ///Bit 4 - Framing Error Flag
    #[inline(always)]
    pub fn fer(&mut self) -> FerW<SsrMancSpec> {
        FerW::new(self, 4)
    }
    ///Bit 5 - Overrun Error Flag
    #[inline(always)]
    pub fn orer(&mut self) -> OrerW<SsrMancSpec> {
        OrerW::new(self, 5)
    }
    ///Bit 6 - Receive Data Full Flag
    #[inline(always)]
    pub fn rdrf(&mut self) -> RdrfW<SsrMancSpec> {
        RdrfW::new(self, 6)
    }
    ///Bit 7 - Transmit Data Empty Flag
    #[inline(always)]
    pub fn tdre(&mut self) -> TdreW<SsrMancSpec> {
        TdreW::new(self, 7)
    }
}
/**Serial Status Register for Manchester Mode (SCMR.SMIF = 0, and MMR.MANEN = 1)

You can [`read`](crate::Reg::read) this register and get [`ssr_manc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssr_manc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SsrMancSpec;
impl crate::RegisterSpec for SsrMancSpec {
    type Ux = u8;
}
///`read()` method returns [`ssr_manc::R`](R) reader structure
impl crate::Readable for SsrMancSpec {}
///`write(|w| ..)` method takes [`ssr_manc::W`](W) writer structure
impl crate::Writable for SsrMancSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SSR_MANC to value 0x84
impl crate::Resettable for SsrMancSpec {
    const RESET_VALUE: u8 = 0x84;
}
