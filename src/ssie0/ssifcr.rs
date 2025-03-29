///Register `SSIFCR` reader
pub type R = crate::R<SsifcrSpec>;
///Register `SSIFCR` writer
pub type W = crate::W<SsifcrSpec>;
/**Receive FIFO Data Register Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfrst {
    ///0: Clears a receive data FIFO reset condition
    _0 = 0,
    ///1: Sets a receive data FIFO reset condition
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
    ///Clears a receive data FIFO reset condition
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rfrst::_0
    }
    ///Sets a receive data FIFO reset condition
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
    ///Clears a receive data FIFO reset condition
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rfrst::_0)
    }
    ///Sets a receive data FIFO reset condition
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rfrst::_1)
    }
}
/**Transmit FIFO Data Register Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tfrst {
    ///0: Clears a transmit data FIFO reset condition
    _0 = 0,
    ///1: Sets a transmit data FIFO reset condition
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
    ///Clears a transmit data FIFO reset condition
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tfrst::_0
    }
    ///Sets a transmit data FIFO reset condition
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
    ///Clears a transmit data FIFO reset condition
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tfrst::_0)
    }
    ///Sets a transmit data FIFO reset condition
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tfrst::_1)
    }
}
/**Receive Data Full Interrupt Output Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rie {
    ///0: Disables receive data full interrupts
    _0 = 0,
    ///1: Enables receive data full interrupts
    _1 = 1,
}
impl From<Rie> for bool {
    #[inline(always)]
    fn from(variant: Rie) -> Self {
        variant as u8 != 0
    }
}
///Field `RIE` reader - Receive Data Full Interrupt Output Enable
pub type RieR = crate::BitReader<Rie>;
impl RieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rie {
        match self.bits {
            false => Rie::_0,
            true => Rie::_1,
        }
    }
    ///Disables receive data full interrupts
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rie::_0
    }
    ///Enables receive data full interrupts
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rie::_1
    }
}
///Field `RIE` writer - Receive Data Full Interrupt Output Enable
pub type RieW<'a, REG> = crate::BitWriter<'a, REG, Rie>;
impl<'a, REG> RieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables receive data full interrupts
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rie::_0)
    }
    ///Enables receive data full interrupts
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rie::_1)
    }
}
/**Transmit Data Empty Interrupt Output Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tie {
    ///0: Disables transmit data empty interrupts
    _0 = 0,
    ///1: Enables transmit data empty interrupts
    _1 = 1,
}
impl From<Tie> for bool {
    #[inline(always)]
    fn from(variant: Tie) -> Self {
        variant as u8 != 0
    }
}
///Field `TIE` reader - Transmit Data Empty Interrupt Output Enable
pub type TieR = crate::BitReader<Tie>;
impl TieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tie {
        match self.bits {
            false => Tie::_0,
            true => Tie::_1,
        }
    }
    ///Disables transmit data empty interrupts
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tie::_0
    }
    ///Enables transmit data empty interrupts
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tie::_1
    }
}
///Field `TIE` writer - Transmit Data Empty Interrupt Output Enable
pub type TieW<'a, REG> = crate::BitWriter<'a, REG, Tie>;
impl<'a, REG> TieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables transmit data empty interrupts
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tie::_0)
    }
    ///Enables transmit data empty interrupts
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tie::_1)
    }
}
/**Byte Swap Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bsw {
    ///0: Disables byte swap
    _0 = 0,
    ///1: Enables byte swap
    _1 = 1,
}
impl From<Bsw> for bool {
    #[inline(always)]
    fn from(variant: Bsw) -> Self {
        variant as u8 != 0
    }
}
///Field `BSW` reader - Byte Swap Enable
pub type BswR = crate::BitReader<Bsw>;
impl BswR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Bsw {
        match self.bits {
            false => Bsw::_0,
            true => Bsw::_1,
        }
    }
    ///Disables byte swap
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bsw::_0
    }
    ///Enables byte swap
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bsw::_1
    }
}
///Field `BSW` writer - Byte Swap Enable
pub type BswW<'a, REG> = crate::BitWriter<'a, REG, Bsw>;
impl<'a, REG> BswW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables byte swap
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bsw::_0)
    }
    ///Enables byte swap
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bsw::_1)
    }
}
/**Software Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ssirst {
    ///0: Clears a software reset condition
    _0 = 0,
    ///1: Sets a software reset condition
    _1 = 1,
}
impl From<Ssirst> for bool {
    #[inline(always)]
    fn from(variant: Ssirst) -> Self {
        variant as u8 != 0
    }
}
///Field `SSIRST` reader - Software Reset
pub type SsirstR = crate::BitReader<Ssirst>;
impl SsirstR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ssirst {
        match self.bits {
            false => Ssirst::_0,
            true => Ssirst::_1,
        }
    }
    ///Clears a software reset condition
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ssirst::_0
    }
    ///Sets a software reset condition
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ssirst::_1
    }
}
///Field `SSIRST` writer - Software Reset
pub type SsirstW<'a, REG> = crate::BitWriter<'a, REG, Ssirst>;
impl<'a, REG> SsirstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears a software reset condition
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ssirst::_0)
    }
    ///Sets a software reset condition
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ssirst::_1)
    }
}
/**AUDIO_MCK Enable in Master-mode Communication

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aucke {
    ///0: Disables supply of AUDIO_MCK
    _0 = 0,
    ///1: Enables supply of AUDIO_MCK
    _1 = 1,
}
impl From<Aucke> for bool {
    #[inline(always)]
    fn from(variant: Aucke) -> Self {
        variant as u8 != 0
    }
}
///Field `AUCKE` reader - AUDIO_MCK Enable in Master-mode Communication
pub type AuckeR = crate::BitReader<Aucke>;
impl AuckeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Aucke {
        match self.bits {
            false => Aucke::_0,
            true => Aucke::_1,
        }
    }
    ///Disables supply of AUDIO_MCK
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Aucke::_0
    }
    ///Enables supply of AUDIO_MCK
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Aucke::_1
    }
}
///Field `AUCKE` writer - AUDIO_MCK Enable in Master-mode Communication
pub type AuckeW<'a, REG> = crate::BitWriter<'a, REG, Aucke>;
impl<'a, REG> AuckeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables supply of AUDIO_MCK
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Aucke::_0)
    }
    ///Enables supply of AUDIO_MCK
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Aucke::_1)
    }
}
impl R {
    ///Bit 0 - Receive FIFO Data Register Reset
    #[inline(always)]
    pub fn rfrst(&self) -> RfrstR {
        RfrstR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Transmit FIFO Data Register Reset
    #[inline(always)]
    pub fn tfrst(&self) -> TfrstR {
        TfrstR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Receive Data Full Interrupt Output Enable
    #[inline(always)]
    pub fn rie(&self) -> RieR {
        RieR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Transmit Data Empty Interrupt Output Enable
    #[inline(always)]
    pub fn tie(&self) -> TieR {
        TieR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 11 - Byte Swap Enable
    #[inline(always)]
    pub fn bsw(&self) -> BswR {
        BswR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 16 - Software Reset
    #[inline(always)]
    pub fn ssirst(&self) -> SsirstR {
        SsirstR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 31 - AUDIO_MCK Enable in Master-mode Communication
    #[inline(always)]
    pub fn aucke(&self) -> AuckeR {
        AuckeR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SSIFCR")
            .field("rfrst", &self.rfrst())
            .field("tfrst", &self.tfrst())
            .field("rie", &self.rie())
            .field("tie", &self.tie())
            .field("bsw", &self.bsw())
            .field("ssirst", &self.ssirst())
            .field("aucke", &self.aucke())
            .finish()
    }
}
impl W {
    ///Bit 0 - Receive FIFO Data Register Reset
    #[inline(always)]
    pub fn rfrst(&mut self) -> RfrstW<SsifcrSpec> {
        RfrstW::new(self, 0)
    }
    ///Bit 1 - Transmit FIFO Data Register Reset
    #[inline(always)]
    pub fn tfrst(&mut self) -> TfrstW<SsifcrSpec> {
        TfrstW::new(self, 1)
    }
    ///Bit 2 - Receive Data Full Interrupt Output Enable
    #[inline(always)]
    pub fn rie(&mut self) -> RieW<SsifcrSpec> {
        RieW::new(self, 2)
    }
    ///Bit 3 - Transmit Data Empty Interrupt Output Enable
    #[inline(always)]
    pub fn tie(&mut self) -> TieW<SsifcrSpec> {
        TieW::new(self, 3)
    }
    ///Bit 11 - Byte Swap Enable
    #[inline(always)]
    pub fn bsw(&mut self) -> BswW<SsifcrSpec> {
        BswW::new(self, 11)
    }
    ///Bit 16 - Software Reset
    #[inline(always)]
    pub fn ssirst(&mut self) -> SsirstW<SsifcrSpec> {
        SsirstW::new(self, 16)
    }
    ///Bit 31 - AUDIO_MCK Enable in Master-mode Communication
    #[inline(always)]
    pub fn aucke(&mut self) -> AuckeW<SsifcrSpec> {
        AuckeW::new(self, 31)
    }
}
/**FIFO Control Register

You can [`read`](crate::Reg::read) this register and get [`ssifcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssifcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SsifcrSpec;
impl crate::RegisterSpec for SsifcrSpec {
    type Ux = u32;
}
///`read()` method returns [`ssifcr::R`](R) reader structure
impl crate::Readable for SsifcrSpec {}
///`write(|w| ..)` method takes [`ssifcr::W`](W) writer structure
impl crate::Writable for SsifcrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SSIFCR to value 0
impl crate::Resettable for SsifcrSpec {}
