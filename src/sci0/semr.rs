///Register `SEMR` reader
pub type R = crate::R<SemrSpec>;
///Register `SEMR` writer
pub type W = crate::W<SemrSpec>;
/**Asynchronous Mode Clock Source Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acs0 {
    ///0: External clock input
    _0 = 0,
    ///1: Logical AND of compare matches output from the internal GPT. These bit for the other SCI channels than SCIn (n = 1, 2) are reserved.
    _1 = 1,
}
impl From<Acs0> for bool {
    #[inline(always)]
    fn from(variant: Acs0) -> Self {
        variant as u8 != 0
    }
}
///Field `ACS0` reader - Asynchronous Mode Clock Source Select
pub type Acs0R = crate::BitReader<Acs0>;
impl Acs0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Acs0 {
        match self.bits {
            false => Acs0::_0,
            true => Acs0::_1,
        }
    }
    ///External clock input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Acs0::_0
    }
    ///Logical AND of compare matches output from the internal GPT. These bit for the other SCI channels than SCIn (n = 1, 2) are reserved.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Acs0::_1
    }
}
///Field `ACS0` writer - Asynchronous Mode Clock Source Select
pub type Acs0W<'a, REG> = crate::BitWriter<'a, REG, Acs0>;
impl<'a, REG> Acs0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///External clock input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Acs0::_0)
    }
    ///Logical AND of compare matches output from the internal GPT. These bit for the other SCI channels than SCIn (n = 1, 2) are reserved.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Acs0::_1)
    }
}
/**Preamble function Disable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Padis {
    ///0: Preamble output function is enabled
    _0 = 0,
    ///1: Preamble output function is disabled These bits for the other SCI channels than SCIn (n = 0, 3 to 9) are reserved.
    _1 = 1,
}
impl From<Padis> for bool {
    #[inline(always)]
    fn from(variant: Padis) -> Self {
        variant as u8 != 0
    }
}
///Field `PADIS` reader - Preamble function Disable
pub type PadisR = crate::BitReader<Padis>;
impl PadisR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Padis {
        match self.bits {
            false => Padis::_0,
            true => Padis::_1,
        }
    }
    ///Preamble output function is enabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Padis::_0
    }
    ///Preamble output function is disabled These bits for the other SCI channels than SCIn (n = 0, 3 to 9) are reserved.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Padis::_1
    }
}
///Field `PADIS` writer - Preamble function Disable
pub type PadisW<'a, REG> = crate::BitWriter<'a, REG, Padis>;
impl<'a, REG> PadisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Preamble output function is enabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Padis::_0)
    }
    ///Preamble output function is disabled These bits for the other SCI channels than SCIn (n = 0, 3 to 9) are reserved.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Padis::_1)
    }
}
/**Bit Rate Modulation Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Brme {
    ///0: Disable bit rate modulation function
    _0 = 0,
    ///1: Enable bit rate modulation function
    _1 = 1,
}
impl From<Brme> for bool {
    #[inline(always)]
    fn from(variant: Brme) -> Self {
        variant as u8 != 0
    }
}
///Field `BRME` reader - Bit Rate Modulation Enable
pub type BrmeR = crate::BitReader<Brme>;
impl BrmeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Brme {
        match self.bits {
            false => Brme::_0,
            true => Brme::_1,
        }
    }
    ///Disable bit rate modulation function
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Brme::_0
    }
    ///Enable bit rate modulation function
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Brme::_1
    }
}
///Field `BRME` writer - Bit Rate Modulation Enable
pub type BrmeW<'a, REG> = crate::BitWriter<'a, REG, Brme>;
impl<'a, REG> BrmeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable bit rate modulation function
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Brme::_0)
    }
    ///Enable bit rate modulation function
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Brme::_1)
    }
}
/**Asynchronous Mode Extended Base Clock Select 1

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Abcse {
    ///0: Clock cycles for 1-bit period determined by combination of the BGDM and ABCS bits in the SEMR register
    _0 = 0,
    ///1: Baud rate is 6 base clock cycles for 1-bit period These bits for the other SCI channels than SCIn (n = 0, 3 to 9) are reserved.
    _1 = 1,
}
impl From<Abcse> for bool {
    #[inline(always)]
    fn from(variant: Abcse) -> Self {
        variant as u8 != 0
    }
}
///Field `ABCSE` reader - Asynchronous Mode Extended Base Clock Select 1
pub type AbcseR = crate::BitReader<Abcse>;
impl AbcseR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Abcse {
        match self.bits {
            false => Abcse::_0,
            true => Abcse::_1,
        }
    }
    ///Clock cycles for 1-bit period determined by combination of the BGDM and ABCS bits in the SEMR register
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Abcse::_0
    }
    ///Baud rate is 6 base clock cycles for 1-bit period These bits for the other SCI channels than SCIn (n = 0, 3 to 9) are reserved.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Abcse::_1
    }
}
///Field `ABCSE` writer - Asynchronous Mode Extended Base Clock Select 1
pub type AbcseW<'a, REG> = crate::BitWriter<'a, REG, Abcse>;
impl<'a, REG> AbcseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clock cycles for 1-bit period determined by combination of the BGDM and ABCS bits in the SEMR register
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Abcse::_0)
    }
    ///Baud rate is 6 base clock cycles for 1-bit period These bits for the other SCI channels than SCIn (n = 0, 3 to 9) are reserved.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Abcse::_1)
    }
}
/**Asynchronous Mode Base Clock Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Abcs {
    ///0: Select 16 base clock cycles for 1-bit period
    _0 = 0,
    ///1: Select 8 base clock cycles for 1-bit period
    _1 = 1,
}
impl From<Abcs> for bool {
    #[inline(always)]
    fn from(variant: Abcs) -> Self {
        variant as u8 != 0
    }
}
///Field `ABCS` reader - Asynchronous Mode Base Clock Select
pub type AbcsR = crate::BitReader<Abcs>;
impl AbcsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Abcs {
        match self.bits {
            false => Abcs::_0,
            true => Abcs::_1,
        }
    }
    ///Select 16 base clock cycles for 1-bit period
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Abcs::_0
    }
    ///Select 8 base clock cycles for 1-bit period
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Abcs::_1
    }
}
///Field `ABCS` writer - Asynchronous Mode Base Clock Select
pub type AbcsW<'a, REG> = crate::BitWriter<'a, REG, Abcs>;
impl<'a, REG> AbcsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Select 16 base clock cycles for 1-bit period
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Abcs::_0)
    }
    ///Select 8 base clock cycles for 1-bit period
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Abcs::_1)
    }
}
/**Digital Noise Filter Function Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nfen {
    ///0: In asynchronous mode: Disable noise cancellation function for RXDn input signal In simple I2C mode: Disable noise cancellation function for SCLn and SDAn input signals
    _0 = 0,
    ///1: In asynchronous mode: Enable noise cancellation function for RXDn input signal In simple I2C mode: Enable noise cancellation function for SCLn and SDAn input signals
    _1 = 1,
}
impl From<Nfen> for bool {
    #[inline(always)]
    fn from(variant: Nfen) -> Self {
        variant as u8 != 0
    }
}
///Field `NFEN` reader - Digital Noise Filter Function Enable
pub type NfenR = crate::BitReader<Nfen>;
impl NfenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Nfen {
        match self.bits {
            false => Nfen::_0,
            true => Nfen::_1,
        }
    }
    ///In asynchronous mode: Disable noise cancellation function for RXDn input signal In simple I2C mode: Disable noise cancellation function for SCLn and SDAn input signals
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Nfen::_0
    }
    ///In asynchronous mode: Enable noise cancellation function for RXDn input signal In simple I2C mode: Enable noise cancellation function for SCLn and SDAn input signals
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Nfen::_1
    }
}
///Field `NFEN` writer - Digital Noise Filter Function Enable
pub type NfenW<'a, REG> = crate::BitWriter<'a, REG, Nfen>;
impl<'a, REG> NfenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///In asynchronous mode: Disable noise cancellation function for RXDn input signal In simple I2C mode: Disable noise cancellation function for SCLn and SDAn input signals
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Nfen::_0)
    }
    ///In asynchronous mode: Enable noise cancellation function for RXDn input signal In simple I2C mode: Enable noise cancellation function for SCLn and SDAn input signals
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Nfen::_1)
    }
}
/**Baud Rate Generator Double-Speed Mode Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bgdm {
    ///0: Output clock from baud rate generator with normal frequency
    _0 = 0,
    ///1: Output clock from baud rate generator with doubled frequency
    _1 = 1,
}
impl From<Bgdm> for bool {
    #[inline(always)]
    fn from(variant: Bgdm) -> Self {
        variant as u8 != 0
    }
}
///Field `BGDM` reader - Baud Rate Generator Double-Speed Mode Select
pub type BgdmR = crate::BitReader<Bgdm>;
impl BgdmR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Bgdm {
        match self.bits {
            false => Bgdm::_0,
            true => Bgdm::_1,
        }
    }
    ///Output clock from baud rate generator with normal frequency
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bgdm::_0
    }
    ///Output clock from baud rate generator with doubled frequency
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bgdm::_1
    }
}
///Field `BGDM` writer - Baud Rate Generator Double-Speed Mode Select
pub type BgdmW<'a, REG> = crate::BitWriter<'a, REG, Bgdm>;
impl<'a, REG> BgdmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Output clock from baud rate generator with normal frequency
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bgdm::_0)
    }
    ///Output clock from baud rate generator with doubled frequency
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bgdm::_1)
    }
}
/**Asynchronous Start Bit Edge Detection Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxdesel {
    ///0: Detect low level on RXDn pin as start bit
    _0 = 0,
    ///1: Detect falling edge of RXDn pin as start bit
    _1 = 1,
}
impl From<Rxdesel> for bool {
    #[inline(always)]
    fn from(variant: Rxdesel) -> Self {
        variant as u8 != 0
    }
}
///Field `RXDESEL` reader - Asynchronous Start Bit Edge Detection Select
pub type RxdeselR = crate::BitReader<Rxdesel>;
impl RxdeselR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rxdesel {
        match self.bits {
            false => Rxdesel::_0,
            true => Rxdesel::_1,
        }
    }
    ///Detect low level on RXDn pin as start bit
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rxdesel::_0
    }
    ///Detect falling edge of RXDn pin as start bit
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rxdesel::_1
    }
}
///Field `RXDESEL` writer - Asynchronous Start Bit Edge Detection Select
pub type RxdeselW<'a, REG> = crate::BitWriter<'a, REG, Rxdesel>;
impl<'a, REG> RxdeselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Detect low level on RXDn pin as start bit
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rxdesel::_0)
    }
    ///Detect falling edge of RXDn pin as start bit
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxdesel::_1)
    }
}
impl R {
    ///Bit 0 - Asynchronous Mode Clock Source Select
    #[inline(always)]
    pub fn acs0(&self) -> Acs0R {
        Acs0R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Preamble function Disable
    #[inline(always)]
    pub fn padis(&self) -> PadisR {
        PadisR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Bit Rate Modulation Enable
    #[inline(always)]
    pub fn brme(&self) -> BrmeR {
        BrmeR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Asynchronous Mode Extended Base Clock Select 1
    #[inline(always)]
    pub fn abcse(&self) -> AbcseR {
        AbcseR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Asynchronous Mode Base Clock Select
    #[inline(always)]
    pub fn abcs(&self) -> AbcsR {
        AbcsR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Digital Noise Filter Function Enable
    #[inline(always)]
    pub fn nfen(&self) -> NfenR {
        NfenR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Baud Rate Generator Double-Speed Mode Select
    #[inline(always)]
    pub fn bgdm(&self) -> BgdmR {
        BgdmR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Asynchronous Start Bit Edge Detection Select
    #[inline(always)]
    pub fn rxdesel(&self) -> RxdeselR {
        RxdeselR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEMR")
            .field("acs0", &self.acs0())
            .field("padis", &self.padis())
            .field("brme", &self.brme())
            .field("abcse", &self.abcse())
            .field("abcs", &self.abcs())
            .field("nfen", &self.nfen())
            .field("bgdm", &self.bgdm())
            .field("rxdesel", &self.rxdesel())
            .finish()
    }
}
impl W {
    ///Bit 0 - Asynchronous Mode Clock Source Select
    #[inline(always)]
    pub fn acs0(&mut self) -> Acs0W<SemrSpec> {
        Acs0W::new(self, 0)
    }
    ///Bit 1 - Preamble function Disable
    #[inline(always)]
    pub fn padis(&mut self) -> PadisW<SemrSpec> {
        PadisW::new(self, 1)
    }
    ///Bit 2 - Bit Rate Modulation Enable
    #[inline(always)]
    pub fn brme(&mut self) -> BrmeW<SemrSpec> {
        BrmeW::new(self, 2)
    }
    ///Bit 3 - Asynchronous Mode Extended Base Clock Select 1
    #[inline(always)]
    pub fn abcse(&mut self) -> AbcseW<SemrSpec> {
        AbcseW::new(self, 3)
    }
    ///Bit 4 - Asynchronous Mode Base Clock Select
    #[inline(always)]
    pub fn abcs(&mut self) -> AbcsW<SemrSpec> {
        AbcsW::new(self, 4)
    }
    ///Bit 5 - Digital Noise Filter Function Enable
    #[inline(always)]
    pub fn nfen(&mut self) -> NfenW<SemrSpec> {
        NfenW::new(self, 5)
    }
    ///Bit 6 - Baud Rate Generator Double-Speed Mode Select
    #[inline(always)]
    pub fn bgdm(&mut self) -> BgdmW<SemrSpec> {
        BgdmW::new(self, 6)
    }
    ///Bit 7 - Asynchronous Start Bit Edge Detection Select
    #[inline(always)]
    pub fn rxdesel(&mut self) -> RxdeselW<SemrSpec> {
        RxdeselW::new(self, 7)
    }
}
/**Serial Extended Mode Register

You can [`read`](crate::Reg::read) this register and get [`semr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`semr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SemrSpec;
impl crate::RegisterSpec for SemrSpec {
    type Ux = u8;
}
///`read()` method returns [`semr::R`](R) reader structure
impl crate::Readable for SemrSpec {}
///`write(|w| ..)` method takes [`semr::W`](W) writer structure
impl crate::Writable for SemrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SEMR to value 0
impl crate::Resettable for SemrSpec {}
