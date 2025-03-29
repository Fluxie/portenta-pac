///Register `MMR` reader
pub type R = crate::R<MmrSpec>;
///Register `MMR` writer
pub type W = crate::W<MmrSpec>;
/**Polarity of Received Manchester Code

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rmpol {
    ///0: Logic 0 is coded as a zero-to-one transition in Manchester code Logic 1 is coded as a one-to-zero transition in Manchester code
    _0 = 0,
    ///1: Logic 0 is coded as a one-to-zero transition in Manchester code Logic 1 is coded as a zero-to-one transition in Manchester code
    _1 = 1,
}
impl From<Rmpol> for bool {
    #[inline(always)]
    fn from(variant: Rmpol) -> Self {
        variant as u8 != 0
    }
}
///Field `RMPOL` reader - Polarity of Received Manchester Code
pub type RmpolR = crate::BitReader<Rmpol>;
impl RmpolR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rmpol {
        match self.bits {
            false => Rmpol::_0,
            true => Rmpol::_1,
        }
    }
    ///Logic 0 is coded as a zero-to-one transition in Manchester code Logic 1 is coded as a one-to-zero transition in Manchester code
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rmpol::_0
    }
    ///Logic 0 is coded as a one-to-zero transition in Manchester code Logic 1 is coded as a zero-to-one transition in Manchester code
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rmpol::_1
    }
}
///Field `RMPOL` writer - Polarity of Received Manchester Code
pub type RmpolW<'a, REG> = crate::BitWriter<'a, REG, Rmpol>;
impl<'a, REG> RmpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Logic 0 is coded as a zero-to-one transition in Manchester code Logic 1 is coded as a one-to-zero transition in Manchester code
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rmpol::_0)
    }
    ///Logic 0 is coded as a one-to-zero transition in Manchester code Logic 1 is coded as a zero-to-one transition in Manchester code
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rmpol::_1)
    }
}
/**Polarity of Transmit Manchester Code

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tmpol {
    ///0: Logic 0 is coded as a zero-to-one transition in Manchester code Logic 1 is coded as a one-to-zero transition in Manchester code
    _0 = 0,
    ///1: Logic 0 is coded as a one-to-zero transition in Manchester code Logic 1 is coded as a zero-to-one transition in Manchester code
    _1 = 1,
}
impl From<Tmpol> for bool {
    #[inline(always)]
    fn from(variant: Tmpol) -> Self {
        variant as u8 != 0
    }
}
///Field `TMPOL` reader - Polarity of Transmit Manchester Code
pub type TmpolR = crate::BitReader<Tmpol>;
impl TmpolR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tmpol {
        match self.bits {
            false => Tmpol::_0,
            true => Tmpol::_1,
        }
    }
    ///Logic 0 is coded as a zero-to-one transition in Manchester code Logic 1 is coded as a one-to-zero transition in Manchester code
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tmpol::_0
    }
    ///Logic 0 is coded as a one-to-zero transition in Manchester code Logic 1 is coded as a zero-to-one transition in Manchester code
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tmpol::_1
    }
}
///Field `TMPOL` writer - Polarity of Transmit Manchester Code
pub type TmpolW<'a, REG> = crate::BitWriter<'a, REG, Tmpol>;
impl<'a, REG> TmpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Logic 0 is coded as a zero-to-one transition in Manchester code Logic 1 is coded as a one-to-zero transition in Manchester code
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tmpol::_0)
    }
    ///Logic 0 is coded as a one-to-zero transition in Manchester code Logic 1 is coded as a zero-to-one transition in Manchester code
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tmpol::_1)
    }
}
/**Manchester Edge Retiming Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Erten {
    ///0: Disables the receive retiming function
    _0 = 0,
    ///1: Enables the receive retiming function
    _1 = 1,
}
impl From<Erten> for bool {
    #[inline(always)]
    fn from(variant: Erten) -> Self {
        variant as u8 != 0
    }
}
///Field `ERTEN` reader - Manchester Edge Retiming Enable
pub type ErtenR = crate::BitReader<Erten>;
impl ErtenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Erten {
        match self.bits {
            false => Erten::_0,
            true => Erten::_1,
        }
    }
    ///Disables the receive retiming function
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Erten::_0
    }
    ///Enables the receive retiming function
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Erten::_1
    }
}
///Field `ERTEN` writer - Manchester Edge Retiming Enable
pub type ErtenW<'a, REG> = crate::BitWriter<'a, REG, Erten>;
impl<'a, REG> ErtenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables the receive retiming function
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Erten::_0)
    }
    ///Enables the receive retiming function
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Erten::_1)
    }
}
/**SYNC value Setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Synval {
    ///0: The start bit is added as a zero-to-one transition.
    _0 = 0,
    ///1: The start bit is added as a one-to-zero transition.
    _1 = 1,
}
impl From<Synval> for bool {
    #[inline(always)]
    fn from(variant: Synval) -> Self {
        variant as u8 != 0
    }
}
///Field `SYNVAL` reader - SYNC value Setting
pub type SynvalR = crate::BitReader<Synval>;
impl SynvalR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Synval {
        match self.bits {
            false => Synval::_0,
            true => Synval::_1,
        }
    }
    ///The start bit is added as a zero-to-one transition.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Synval::_0
    }
    ///The start bit is added as a one-to-zero transition.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Synval::_1
    }
}
///Field `SYNVAL` writer - SYNC value Setting
pub type SynvalW<'a, REG> = crate::BitWriter<'a, REG, Synval>;
impl<'a, REG> SynvalW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The start bit is added as a zero-to-one transition.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Synval::_0)
    }
    ///The start bit is added as a one-to-zero transition.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Synval::_1)
    }
}
/**SYNC Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Synsel {
    ///0: The start bit pattern is set with the SYNVAL bit
    _0 = 0,
    ///1: The start bit pattern is set with the TSYNC bit.
    _1 = 1,
}
impl From<Synsel> for bool {
    #[inline(always)]
    fn from(variant: Synsel) -> Self {
        variant as u8 != 0
    }
}
///Field `SYNSEL` reader - SYNC Select
pub type SynselR = crate::BitReader<Synsel>;
impl SynselR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Synsel {
        match self.bits {
            false => Synsel::_0,
            true => Synsel::_1,
        }
    }
    ///The start bit pattern is set with the SYNVAL bit
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Synsel::_0
    }
    ///The start bit pattern is set with the TSYNC bit.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Synsel::_1
    }
}
///Field `SYNSEL` writer - SYNC Select
pub type SynselW<'a, REG> = crate::BitWriter<'a, REG, Synsel>;
impl<'a, REG> SynselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The start bit pattern is set with the SYNVAL bit
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Synsel::_0)
    }
    ///The start bit pattern is set with the TSYNC bit.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Synsel::_1)
    }
}
/**Start Bit Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sbsel {
    ///0: The start bit area consists of one bit.
    _0 = 0,
    ///1: The start bit area consists of three bits (COMMAND SYNC or DATA SYNC)
    _1 = 1,
}
impl From<Sbsel> for bool {
    #[inline(always)]
    fn from(variant: Sbsel) -> Self {
        variant as u8 != 0
    }
}
///Field `SBSEL` reader - Start Bit Select
pub type SbselR = crate::BitReader<Sbsel>;
impl SbselR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sbsel {
        match self.bits {
            false => Sbsel::_0,
            true => Sbsel::_1,
        }
    }
    ///The start bit area consists of one bit.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sbsel::_0
    }
    ///The start bit area consists of three bits (COMMAND SYNC or DATA SYNC)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sbsel::_1
    }
}
///Field `SBSEL` writer - Start Bit Select
pub type SbselW<'a, REG> = crate::BitWriter<'a, REG, Sbsel>;
impl<'a, REG> SbselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The start bit area consists of one bit.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sbsel::_0)
    }
    ///The start bit area consists of three bits (COMMAND SYNC or DATA SYNC)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sbsel::_1)
    }
}
/**Manchester Mode Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Manen {
    ///0: Disables the Manchester mode
    _0 = 0,
    ///1: Enables the Manchester mode
    _1 = 1,
}
impl From<Manen> for bool {
    #[inline(always)]
    fn from(variant: Manen) -> Self {
        variant as u8 != 0
    }
}
///Field `MANEN` reader - Manchester Mode Enable
pub type ManenR = crate::BitReader<Manen>;
impl ManenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Manen {
        match self.bits {
            false => Manen::_0,
            true => Manen::_1,
        }
    }
    ///Disables the Manchester mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Manen::_0
    }
    ///Enables the Manchester mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Manen::_1
    }
}
///Field `MANEN` writer - Manchester Mode Enable
pub type ManenW<'a, REG> = crate::BitWriter<'a, REG, Manen>;
impl<'a, REG> ManenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables the Manchester mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Manen::_0)
    }
    ///Enables the Manchester mode
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Manen::_1)
    }
}
impl R {
    ///Bit 0 - Polarity of Received Manchester Code
    #[inline(always)]
    pub fn rmpol(&self) -> RmpolR {
        RmpolR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Polarity of Transmit Manchester Code
    #[inline(always)]
    pub fn tmpol(&self) -> TmpolR {
        TmpolR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Manchester Edge Retiming Enable
    #[inline(always)]
    pub fn erten(&self) -> ErtenR {
        ErtenR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - SYNC value Setting
    #[inline(always)]
    pub fn synval(&self) -> SynvalR {
        SynvalR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - SYNC Select
    #[inline(always)]
    pub fn synsel(&self) -> SynselR {
        SynselR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Start Bit Select
    #[inline(always)]
    pub fn sbsel(&self) -> SbselR {
        SbselR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Manchester Mode Enable
    #[inline(always)]
    pub fn manen(&self) -> ManenR {
        ManenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMR")
            .field("rmpol", &self.rmpol())
            .field("tmpol", &self.tmpol())
            .field("erten", &self.erten())
            .field("synval", &self.synval())
            .field("synsel", &self.synsel())
            .field("sbsel", &self.sbsel())
            .field("manen", &self.manen())
            .finish()
    }
}
impl W {
    ///Bit 0 - Polarity of Received Manchester Code
    #[inline(always)]
    pub fn rmpol(&mut self) -> RmpolW<MmrSpec> {
        RmpolW::new(self, 0)
    }
    ///Bit 1 - Polarity of Transmit Manchester Code
    #[inline(always)]
    pub fn tmpol(&mut self) -> TmpolW<MmrSpec> {
        TmpolW::new(self, 1)
    }
    ///Bit 2 - Manchester Edge Retiming Enable
    #[inline(always)]
    pub fn erten(&mut self) -> ErtenW<MmrSpec> {
        ErtenW::new(self, 2)
    }
    ///Bit 4 - SYNC value Setting
    #[inline(always)]
    pub fn synval(&mut self) -> SynvalW<MmrSpec> {
        SynvalW::new(self, 4)
    }
    ///Bit 5 - SYNC Select
    #[inline(always)]
    pub fn synsel(&mut self) -> SynselW<MmrSpec> {
        SynselW::new(self, 5)
    }
    ///Bit 6 - Start Bit Select
    #[inline(always)]
    pub fn sbsel(&mut self) -> SbselW<MmrSpec> {
        SbselW::new(self, 6)
    }
    ///Bit 7 - Manchester Mode Enable
    #[inline(always)]
    pub fn manen(&mut self) -> ManenW<MmrSpec> {
        ManenW::new(self, 7)
    }
}
/**Manchester Mode Register

You can [`read`](crate::Reg::read) this register and get [`mmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MmrSpec;
impl crate::RegisterSpec for MmrSpec {
    type Ux = u8;
}
///`read()` method returns [`mmr::R`](R) reader structure
impl crate::Readable for MmrSpec {}
///`write(|w| ..)` method takes [`mmr::W`](W) writer structure
impl crate::Writable for MmrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MMR to value 0
impl crate::Resettable for MmrSpec {}
