///Register `SPCR` reader
pub type R = crate::R<SpcrSpec>;
///Register `SPCR` writer
pub type W = crate::W<SpcrSpec>;
/**SPI Mode Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spms {
    ///0: Select SPI operation (4-wire method)
    _0 = 0,
    ///1: Select clock synchronous operation (3-wire method)
    _1 = 1,
}
impl From<Spms> for bool {
    #[inline(always)]
    fn from(variant: Spms) -> Self {
        variant as u8 != 0
    }
}
///Field `SPMS` reader - SPI Mode Select
pub type SpmsR = crate::BitReader<Spms>;
impl SpmsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Spms {
        match self.bits {
            false => Spms::_0,
            true => Spms::_1,
        }
    }
    ///Select SPI operation (4-wire method)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Spms::_0
    }
    ///Select clock synchronous operation (3-wire method)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Spms::_1
    }
}
///Field `SPMS` writer - SPI Mode Select
pub type SpmsW<'a, REG> = crate::BitWriter<'a, REG, Spms>;
impl<'a, REG> SpmsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Select SPI operation (4-wire method)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Spms::_0)
    }
    ///Select clock synchronous operation (3-wire method)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Spms::_1)
    }
}
/**Communications Operating Mode Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txmd {
    ///0: Select full-duplex synchronous serial communications
    _0 = 0,
    ///1: Select serial communications with transmit-only
    _1 = 1,
}
impl From<Txmd> for bool {
    #[inline(always)]
    fn from(variant: Txmd) -> Self {
        variant as u8 != 0
    }
}
///Field `TXMD` reader - Communications Operating Mode Select
pub type TxmdR = crate::BitReader<Txmd>;
impl TxmdR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Txmd {
        match self.bits {
            false => Txmd::_0,
            true => Txmd::_1,
        }
    }
    ///Select full-duplex synchronous serial communications
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Txmd::_0
    }
    ///Select serial communications with transmit-only
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Txmd::_1
    }
}
///Field `TXMD` writer - Communications Operating Mode Select
pub type TxmdW<'a, REG> = crate::BitWriter<'a, REG, Txmd>;
impl<'a, REG> TxmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Select full-duplex synchronous serial communications
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Txmd::_0)
    }
    ///Select serial communications with transmit-only
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Txmd::_1)
    }
}
/**Mode Fault Error Detection Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Modfen {
    ///0: Disable detection of mode fault errors
    _0 = 0,
    ///1: Enable detection of mode fault errors
    _1 = 1,
}
impl From<Modfen> for bool {
    #[inline(always)]
    fn from(variant: Modfen) -> Self {
        variant as u8 != 0
    }
}
///Field `MODFEN` reader - Mode Fault Error Detection Enable
pub type ModfenR = crate::BitReader<Modfen>;
impl ModfenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Modfen {
        match self.bits {
            false => Modfen::_0,
            true => Modfen::_1,
        }
    }
    ///Disable detection of mode fault errors
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Modfen::_0
    }
    ///Enable detection of mode fault errors
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Modfen::_1
    }
}
///Field `MODFEN` writer - Mode Fault Error Detection Enable
pub type ModfenW<'a, REG> = crate::BitWriter<'a, REG, Modfen>;
impl<'a, REG> ModfenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable detection of mode fault errors
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Modfen::_0)
    }
    ///Enable detection of mode fault errors
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Modfen::_1)
    }
}
/**SPI Master/Slave Mode Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstr {
    ///0: Select slave mode
    _0 = 0,
    ///1: Select master mode
    _1 = 1,
}
impl From<Mstr> for bool {
    #[inline(always)]
    fn from(variant: Mstr) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTR` reader - SPI Master/Slave Mode Select
pub type MstrR = crate::BitReader<Mstr>;
impl MstrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mstr {
        match self.bits {
            false => Mstr::_0,
            true => Mstr::_1,
        }
    }
    ///Select slave mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstr::_0
    }
    ///Select master mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstr::_1
    }
}
///Field `MSTR` writer - SPI Master/Slave Mode Select
pub type MstrW<'a, REG> = crate::BitWriter<'a, REG, Mstr>;
impl<'a, REG> MstrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Select slave mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstr::_0)
    }
    ///Select master mode
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstr::_1)
    }
}
/**SPI Error Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Speie {
    ///0: Disable SPI error interrupt requests
    _0 = 0,
    ///1: Enable SPI error interrupt requests
    _1 = 1,
}
impl From<Speie> for bool {
    #[inline(always)]
    fn from(variant: Speie) -> Self {
        variant as u8 != 0
    }
}
///Field `SPEIE` reader - SPI Error Interrupt Enable
pub type SpeieR = crate::BitReader<Speie>;
impl SpeieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Speie {
        match self.bits {
            false => Speie::_0,
            true => Speie::_1,
        }
    }
    ///Disable SPI error interrupt requests
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Speie::_0
    }
    ///Enable SPI error interrupt requests
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Speie::_1
    }
}
///Field `SPEIE` writer - SPI Error Interrupt Enable
pub type SpeieW<'a, REG> = crate::BitWriter<'a, REG, Speie>;
impl<'a, REG> SpeieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable SPI error interrupt requests
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Speie::_0)
    }
    ///Enable SPI error interrupt requests
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Speie::_1)
    }
}
/**Transmit Buffer Empty Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sptie {
    ///0: Disable transmit buffer empty interrupt requests
    _0 = 0,
    ///1: Enable transmit buffer empty interrupt requests
    _1 = 1,
}
impl From<Sptie> for bool {
    #[inline(always)]
    fn from(variant: Sptie) -> Self {
        variant as u8 != 0
    }
}
///Field `SPTIE` reader - Transmit Buffer Empty Interrupt Enable
pub type SptieR = crate::BitReader<Sptie>;
impl SptieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sptie {
        match self.bits {
            false => Sptie::_0,
            true => Sptie::_1,
        }
    }
    ///Disable transmit buffer empty interrupt requests
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sptie::_0
    }
    ///Enable transmit buffer empty interrupt requests
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sptie::_1
    }
}
///Field `SPTIE` writer - Transmit Buffer Empty Interrupt Enable
pub type SptieW<'a, REG> = crate::BitWriter<'a, REG, Sptie>;
impl<'a, REG> SptieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable transmit buffer empty interrupt requests
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sptie::_0)
    }
    ///Enable transmit buffer empty interrupt requests
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sptie::_1)
    }
}
/**SPI Function Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spe {
    ///0: Disable SPI function
    _0 = 0,
    ///1: Enable SPI function
    _1 = 1,
}
impl From<Spe> for bool {
    #[inline(always)]
    fn from(variant: Spe) -> Self {
        variant as u8 != 0
    }
}
///Field `SPE` reader - SPI Function Enable
pub type SpeR = crate::BitReader<Spe>;
impl SpeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Spe {
        match self.bits {
            false => Spe::_0,
            true => Spe::_1,
        }
    }
    ///Disable SPI function
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Spe::_0
    }
    ///Enable SPI function
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Spe::_1
    }
}
///Field `SPE` writer - SPI Function Enable
pub type SpeW<'a, REG> = crate::BitWriter<'a, REG, Spe>;
impl<'a, REG> SpeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable SPI function
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Spe::_0)
    }
    ///Enable SPI function
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Spe::_1)
    }
}
/**SPI Receive Buffer Full Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sprie {
    ///0: Disable SPI receive buffer full interrupt requests
    _0 = 0,
    ///1: Enable SPI receive buffer full interrupt requests
    _1 = 1,
}
impl From<Sprie> for bool {
    #[inline(always)]
    fn from(variant: Sprie) -> Self {
        variant as u8 != 0
    }
}
///Field `SPRIE` reader - SPI Receive Buffer Full Interrupt Enable
pub type SprieR = crate::BitReader<Sprie>;
impl SprieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sprie {
        match self.bits {
            false => Sprie::_0,
            true => Sprie::_1,
        }
    }
    ///Disable SPI receive buffer full interrupt requests
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sprie::_0
    }
    ///Enable SPI receive buffer full interrupt requests
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sprie::_1
    }
}
///Field `SPRIE` writer - SPI Receive Buffer Full Interrupt Enable
pub type SprieW<'a, REG> = crate::BitWriter<'a, REG, Sprie>;
impl<'a, REG> SprieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable SPI receive buffer full interrupt requests
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sprie::_0)
    }
    ///Enable SPI receive buffer full interrupt requests
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sprie::_1)
    }
}
impl R {
    ///Bit 0 - SPI Mode Select
    #[inline(always)]
    pub fn spms(&self) -> SpmsR {
        SpmsR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Communications Operating Mode Select
    #[inline(always)]
    pub fn txmd(&self) -> TxmdR {
        TxmdR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Mode Fault Error Detection Enable
    #[inline(always)]
    pub fn modfen(&self) -> ModfenR {
        ModfenR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - SPI Master/Slave Mode Select
    #[inline(always)]
    pub fn mstr(&self) -> MstrR {
        MstrR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - SPI Error Interrupt Enable
    #[inline(always)]
    pub fn speie(&self) -> SpeieR {
        SpeieR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Transmit Buffer Empty Interrupt Enable
    #[inline(always)]
    pub fn sptie(&self) -> SptieR {
        SptieR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - SPI Function Enable
    #[inline(always)]
    pub fn spe(&self) -> SpeR {
        SpeR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - SPI Receive Buffer Full Interrupt Enable
    #[inline(always)]
    pub fn sprie(&self) -> SprieR {
        SprieR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPCR")
            .field("spms", &self.spms())
            .field("txmd", &self.txmd())
            .field("modfen", &self.modfen())
            .field("mstr", &self.mstr())
            .field("speie", &self.speie())
            .field("sptie", &self.sptie())
            .field("spe", &self.spe())
            .field("sprie", &self.sprie())
            .finish()
    }
}
impl W {
    ///Bit 0 - SPI Mode Select
    #[inline(always)]
    pub fn spms(&mut self) -> SpmsW<SpcrSpec> {
        SpmsW::new(self, 0)
    }
    ///Bit 1 - Communications Operating Mode Select
    #[inline(always)]
    pub fn txmd(&mut self) -> TxmdW<SpcrSpec> {
        TxmdW::new(self, 1)
    }
    ///Bit 2 - Mode Fault Error Detection Enable
    #[inline(always)]
    pub fn modfen(&mut self) -> ModfenW<SpcrSpec> {
        ModfenW::new(self, 2)
    }
    ///Bit 3 - SPI Master/Slave Mode Select
    #[inline(always)]
    pub fn mstr(&mut self) -> MstrW<SpcrSpec> {
        MstrW::new(self, 3)
    }
    ///Bit 4 - SPI Error Interrupt Enable
    #[inline(always)]
    pub fn speie(&mut self) -> SpeieW<SpcrSpec> {
        SpeieW::new(self, 4)
    }
    ///Bit 5 - Transmit Buffer Empty Interrupt Enable
    #[inline(always)]
    pub fn sptie(&mut self) -> SptieW<SpcrSpec> {
        SptieW::new(self, 5)
    }
    ///Bit 6 - SPI Function Enable
    #[inline(always)]
    pub fn spe(&mut self) -> SpeW<SpcrSpec> {
        SpeW::new(self, 6)
    }
    ///Bit 7 - SPI Receive Buffer Full Interrupt Enable
    #[inline(always)]
    pub fn sprie(&mut self) -> SprieW<SpcrSpec> {
        SprieW::new(self, 7)
    }
}
/**SPI Control Register

You can [`read`](crate::Reg::read) this register and get [`spcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SpcrSpec;
impl crate::RegisterSpec for SpcrSpec {
    type Ux = u8;
}
///`read()` method returns [`spcr::R`](R) reader structure
impl crate::Readable for SpcrSpec {}
///`write(|w| ..)` method takes [`spcr::W`](W) writer structure
impl crate::Writable for SpcrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SPCR to value 0
impl crate::Resettable for SpcrSpec {}
