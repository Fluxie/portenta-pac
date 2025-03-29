///Register `SSR_SMCI` reader
pub type R = crate::R<SsrSmciSpec>;
///Register `SSR_SMCI` writer
pub type W = crate::W<SsrSmciSpec>;
///Field `MPBT` reader - Multi-Processor Bit Transfer
pub type MpbtR = crate::BitReader;
///Field `MPBT` writer - Multi-Processor Bit Transfer
pub type MpbtW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MPB` reader - Multi-Processor
pub type MpbR = crate::BitReader;
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
/**Error Signal Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ers {
    ///0: No low error signal response
    _0 = 0,
    ///1: Low error signal response occurred
    _1 = 1,
}
impl From<Ers> for bool {
    #[inline(always)]
    fn from(variant: Ers) -> Self {
        variant as u8 != 0
    }
}
///Field `ERS` reader - Error Signal Status Flag
pub type ErsR = crate::BitReader<Ers>;
impl ErsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ers {
        match self.bits {
            false => Ers::_0,
            true => Ers::_1,
        }
    }
    ///No low error signal response
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ers::_0
    }
    ///Low error signal response occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ers::_1
    }
}
///Field `ERS` writer - Error Signal Status Flag
pub type ErsW<'a, REG> = crate::BitWriter<'a, REG, Ers>;
impl<'a, REG> ErsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No low error signal response
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ers::_0)
    }
    ///Low error signal response occurred
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ers::_1)
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
    ///Bit 4 - Error Signal Status Flag
    #[inline(always)]
    pub fn ers(&self) -> ErsR {
        ErsR::new(((self.bits >> 4) & 1) != 0)
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
        f.debug_struct("SSR_SMCI")
            .field("mpbt", &self.mpbt())
            .field("mpb", &self.mpb())
            .field("tend", &self.tend())
            .field("per", &self.per())
            .field("ers", &self.ers())
            .field("orer", &self.orer())
            .field("rdrf", &self.rdrf())
            .field("tdre", &self.tdre())
            .finish()
    }
}
impl W {
    ///Bit 0 - Multi-Processor Bit Transfer
    #[inline(always)]
    pub fn mpbt(&mut self) -> MpbtW<SsrSmciSpec> {
        MpbtW::new(self, 0)
    }
    ///Bit 3 - Parity Error Flag
    #[inline(always)]
    pub fn per(&mut self) -> PerW<SsrSmciSpec> {
        PerW::new(self, 3)
    }
    ///Bit 4 - Error Signal Status Flag
    #[inline(always)]
    pub fn ers(&mut self) -> ErsW<SsrSmciSpec> {
        ErsW::new(self, 4)
    }
    ///Bit 5 - Overrun Error Flag
    #[inline(always)]
    pub fn orer(&mut self) -> OrerW<SsrSmciSpec> {
        OrerW::new(self, 5)
    }
    ///Bit 6 - Receive Data Full Flag
    #[inline(always)]
    pub fn rdrf(&mut self) -> RdrfW<SsrSmciSpec> {
        RdrfW::new(self, 6)
    }
    ///Bit 7 - Transmit Data Empty Flag
    #[inline(always)]
    pub fn tdre(&mut self) -> TdreW<SsrSmciSpec> {
        TdreW::new(self, 7)
    }
}
/**Serial Status Register for Smart Card Interface Mode (SCMR.SMIF = 1, and MMR.MANEN = 0)

You can [`read`](crate::Reg::read) this register and get [`ssr_smci::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssr_smci::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SsrSmciSpec;
impl crate::RegisterSpec for SsrSmciSpec {
    type Ux = u8;
}
///`read()` method returns [`ssr_smci::R`](R) reader structure
impl crate::Readable for SsrSmciSpec {}
///`write(|w| ..)` method takes [`ssr_smci::W`](W) writer structure
impl crate::Writable for SsrSmciSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SSR_SMCI to value 0x84
impl crate::Resettable for SsrSmciSpec {
    const RESET_VALUE: u8 = 0x84;
}
