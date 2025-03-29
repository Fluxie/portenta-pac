///Register `ICIER` reader
pub type R = crate::R<IcierSpec>;
///Register `ICIER` writer
pub type W = crate::W<IcierSpec>;
/**Timeout Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tmoie {
    ///0: Disable timeout interrupt (TMOI) request
    _0 = 0,
    ///1: Enable timeout interrupt (TMOI) request
    _1 = 1,
}
impl From<Tmoie> for bool {
    #[inline(always)]
    fn from(variant: Tmoie) -> Self {
        variant as u8 != 0
    }
}
///Field `TMOIE` reader - Timeout Interrupt Request Enable
pub type TmoieR = crate::BitReader<Tmoie>;
impl TmoieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tmoie {
        match self.bits {
            false => Tmoie::_0,
            true => Tmoie::_1,
        }
    }
    ///Disable timeout interrupt (TMOI) request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tmoie::_0
    }
    ///Enable timeout interrupt (TMOI) request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tmoie::_1
    }
}
///Field `TMOIE` writer - Timeout Interrupt Request Enable
pub type TmoieW<'a, REG> = crate::BitWriter<'a, REG, Tmoie>;
impl<'a, REG> TmoieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable timeout interrupt (TMOI) request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tmoie::_0)
    }
    ///Enable timeout interrupt (TMOI) request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tmoie::_1)
    }
}
/**Arbitration-Lost Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alie {
    ///0: Disable arbitration-lost interrupt (ALI) request
    _0 = 0,
    ///1: Enable arbitration-lost interrupt (ALI) request
    _1 = 1,
}
impl From<Alie> for bool {
    #[inline(always)]
    fn from(variant: Alie) -> Self {
        variant as u8 != 0
    }
}
///Field `ALIE` reader - Arbitration-Lost Interrupt Request Enable
pub type AlieR = crate::BitReader<Alie>;
impl AlieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Alie {
        match self.bits {
            false => Alie::_0,
            true => Alie::_1,
        }
    }
    ///Disable arbitration-lost interrupt (ALI) request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Alie::_0
    }
    ///Enable arbitration-lost interrupt (ALI) request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Alie::_1
    }
}
///Field `ALIE` writer - Arbitration-Lost Interrupt Request Enable
pub type AlieW<'a, REG> = crate::BitWriter<'a, REG, Alie>;
impl<'a, REG> AlieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable arbitration-lost interrupt (ALI) request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Alie::_0)
    }
    ///Enable arbitration-lost interrupt (ALI) request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Alie::_1)
    }
}
/**Start Condition Detection Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stie {
    ///0: Disable start condition detection interrupt (STI) request
    _0 = 0,
    ///1: Enable start condition detection interrupt (STI) request
    _1 = 1,
}
impl From<Stie> for bool {
    #[inline(always)]
    fn from(variant: Stie) -> Self {
        variant as u8 != 0
    }
}
///Field `STIE` reader - Start Condition Detection Interrupt Request Enable
pub type StieR = crate::BitReader<Stie>;
impl StieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Stie {
        match self.bits {
            false => Stie::_0,
            true => Stie::_1,
        }
    }
    ///Disable start condition detection interrupt (STI) request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Stie::_0
    }
    ///Enable start condition detection interrupt (STI) request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Stie::_1
    }
}
///Field `STIE` writer - Start Condition Detection Interrupt Request Enable
pub type StieW<'a, REG> = crate::BitWriter<'a, REG, Stie>;
impl<'a, REG> StieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable start condition detection interrupt (STI) request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Stie::_0)
    }
    ///Enable start condition detection interrupt (STI) request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Stie::_1)
    }
}
/**Stop Condition Detection Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spie {
    ///0: Disable stop condition detection interrupt (SPI) request
    _0 = 0,
    ///1: Enable stop condition detection interrupt (SPI) request
    _1 = 1,
}
impl From<Spie> for bool {
    #[inline(always)]
    fn from(variant: Spie) -> Self {
        variant as u8 != 0
    }
}
///Field `SPIE` reader - Stop Condition Detection Interrupt Request Enable
pub type SpieR = crate::BitReader<Spie>;
impl SpieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Spie {
        match self.bits {
            false => Spie::_0,
            true => Spie::_1,
        }
    }
    ///Disable stop condition detection interrupt (SPI) request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Spie::_0
    }
    ///Enable stop condition detection interrupt (SPI) request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Spie::_1
    }
}
///Field `SPIE` writer - Stop Condition Detection Interrupt Request Enable
pub type SpieW<'a, REG> = crate::BitWriter<'a, REG, Spie>;
impl<'a, REG> SpieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable stop condition detection interrupt (SPI) request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Spie::_0)
    }
    ///Enable stop condition detection interrupt (SPI) request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Spie::_1)
    }
}
/**NACK Reception Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nakie {
    ///0: Disable NACK reception interrupt (NAKI) request
    _0 = 0,
    ///1: Enable NACK reception interrupt (NAKI) request
    _1 = 1,
}
impl From<Nakie> for bool {
    #[inline(always)]
    fn from(variant: Nakie) -> Self {
        variant as u8 != 0
    }
}
///Field `NAKIE` reader - NACK Reception Interrupt Request Enable
pub type NakieR = crate::BitReader<Nakie>;
impl NakieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Nakie {
        match self.bits {
            false => Nakie::_0,
            true => Nakie::_1,
        }
    }
    ///Disable NACK reception interrupt (NAKI) request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Nakie::_0
    }
    ///Enable NACK reception interrupt (NAKI) request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Nakie::_1
    }
}
///Field `NAKIE` writer - NACK Reception Interrupt Request Enable
pub type NakieW<'a, REG> = crate::BitWriter<'a, REG, Nakie>;
impl<'a, REG> NakieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable NACK reception interrupt (NAKI) request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Nakie::_0)
    }
    ///Enable NACK reception interrupt (NAKI) request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Nakie::_1)
    }
}
/**Receive Data Full Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rie {
    ///0: Disable receive data full interrupt (IICn_RXI) request
    _0 = 0,
    ///1: Enable receive data full interrupt (IICn_RXI) request
    _1 = 1,
}
impl From<Rie> for bool {
    #[inline(always)]
    fn from(variant: Rie) -> Self {
        variant as u8 != 0
    }
}
///Field `RIE` reader - Receive Data Full Interrupt Request Enable
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
    ///Disable receive data full interrupt (IICn_RXI) request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rie::_0
    }
    ///Enable receive data full interrupt (IICn_RXI) request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rie::_1
    }
}
///Field `RIE` writer - Receive Data Full Interrupt Request Enable
pub type RieW<'a, REG> = crate::BitWriter<'a, REG, Rie>;
impl<'a, REG> RieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable receive data full interrupt (IICn_RXI) request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rie::_0)
    }
    ///Enable receive data full interrupt (IICn_RXI) request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rie::_1)
    }
}
/**Transmit End Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Teie {
    ///0: Disable transmit end interrupt (IICn_TEI) request
    _0 = 0,
    ///1: Enable transmit end interrupt (IICn_TEI) request
    _1 = 1,
}
impl From<Teie> for bool {
    #[inline(always)]
    fn from(variant: Teie) -> Self {
        variant as u8 != 0
    }
}
///Field `TEIE` reader - Transmit End Interrupt Request Enable
pub type TeieR = crate::BitReader<Teie>;
impl TeieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Teie {
        match self.bits {
            false => Teie::_0,
            true => Teie::_1,
        }
    }
    ///Disable transmit end interrupt (IICn_TEI) request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Teie::_0
    }
    ///Enable transmit end interrupt (IICn_TEI) request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Teie::_1
    }
}
///Field `TEIE` writer - Transmit End Interrupt Request Enable
pub type TeieW<'a, REG> = crate::BitWriter<'a, REG, Teie>;
impl<'a, REG> TeieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable transmit end interrupt (IICn_TEI) request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Teie::_0)
    }
    ///Enable transmit end interrupt (IICn_TEI) request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Teie::_1)
    }
}
/**Transmit Data Empty Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tie {
    ///0: Disable transmit data empty interrupt (IICn_TXI) request
    _0 = 0,
    ///1: Enable transmit data empty interrupt (IICn_TXI) request
    _1 = 1,
}
impl From<Tie> for bool {
    #[inline(always)]
    fn from(variant: Tie) -> Self {
        variant as u8 != 0
    }
}
///Field `TIE` reader - Transmit Data Empty Interrupt Request Enable
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
    ///Disable transmit data empty interrupt (IICn_TXI) request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tie::_0
    }
    ///Enable transmit data empty interrupt (IICn_TXI) request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tie::_1
    }
}
///Field `TIE` writer - Transmit Data Empty Interrupt Request Enable
pub type TieW<'a, REG> = crate::BitWriter<'a, REG, Tie>;
impl<'a, REG> TieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable transmit data empty interrupt (IICn_TXI) request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tie::_0)
    }
    ///Enable transmit data empty interrupt (IICn_TXI) request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tie::_1)
    }
}
impl R {
    ///Bit 0 - Timeout Interrupt Request Enable
    #[inline(always)]
    pub fn tmoie(&self) -> TmoieR {
        TmoieR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Arbitration-Lost Interrupt Request Enable
    #[inline(always)]
    pub fn alie(&self) -> AlieR {
        AlieR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Start Condition Detection Interrupt Request Enable
    #[inline(always)]
    pub fn stie(&self) -> StieR {
        StieR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Stop Condition Detection Interrupt Request Enable
    #[inline(always)]
    pub fn spie(&self) -> SpieR {
        SpieR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - NACK Reception Interrupt Request Enable
    #[inline(always)]
    pub fn nakie(&self) -> NakieR {
        NakieR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Receive Data Full Interrupt Request Enable
    #[inline(always)]
    pub fn rie(&self) -> RieR {
        RieR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Transmit End Interrupt Request Enable
    #[inline(always)]
    pub fn teie(&self) -> TeieR {
        TeieR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Transmit Data Empty Interrupt Request Enable
    #[inline(always)]
    pub fn tie(&self) -> TieR {
        TieR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICIER")
            .field("tmoie", &self.tmoie())
            .field("alie", &self.alie())
            .field("stie", &self.stie())
            .field("spie", &self.spie())
            .field("nakie", &self.nakie())
            .field("rie", &self.rie())
            .field("teie", &self.teie())
            .field("tie", &self.tie())
            .finish()
    }
}
impl W {
    ///Bit 0 - Timeout Interrupt Request Enable
    #[inline(always)]
    pub fn tmoie(&mut self) -> TmoieW<IcierSpec> {
        TmoieW::new(self, 0)
    }
    ///Bit 1 - Arbitration-Lost Interrupt Request Enable
    #[inline(always)]
    pub fn alie(&mut self) -> AlieW<IcierSpec> {
        AlieW::new(self, 1)
    }
    ///Bit 2 - Start Condition Detection Interrupt Request Enable
    #[inline(always)]
    pub fn stie(&mut self) -> StieW<IcierSpec> {
        StieW::new(self, 2)
    }
    ///Bit 3 - Stop Condition Detection Interrupt Request Enable
    #[inline(always)]
    pub fn spie(&mut self) -> SpieW<IcierSpec> {
        SpieW::new(self, 3)
    }
    ///Bit 4 - NACK Reception Interrupt Request Enable
    #[inline(always)]
    pub fn nakie(&mut self) -> NakieW<IcierSpec> {
        NakieW::new(self, 4)
    }
    ///Bit 5 - Receive Data Full Interrupt Request Enable
    #[inline(always)]
    pub fn rie(&mut self) -> RieW<IcierSpec> {
        RieW::new(self, 5)
    }
    ///Bit 6 - Transmit End Interrupt Request Enable
    #[inline(always)]
    pub fn teie(&mut self) -> TeieW<IcierSpec> {
        TeieW::new(self, 6)
    }
    ///Bit 7 - Transmit Data Empty Interrupt Request Enable
    #[inline(always)]
    pub fn tie(&mut self) -> TieW<IcierSpec> {
        TieW::new(self, 7)
    }
}
/**I2C Bus Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`icier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IcierSpec;
impl crate::RegisterSpec for IcierSpec {
    type Ux = u8;
}
///`read()` method returns [`icier::R`](R) reader structure
impl crate::Readable for IcierSpec {}
///`write(|w| ..)` method takes [`icier::W`](W) writer structure
impl crate::Writable for IcierSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICIER to value 0
impl crate::Resettable for IcierSpec {}
