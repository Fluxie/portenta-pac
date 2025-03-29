///Register `CFDCFSTS%s` reader
pub type R = crate::R<CfdcfstsSpec>;
///Register `CFDCFSTS%s` writer
pub type W = crate::W<CfdcfstsSpec>;
/**Common FIFO Empty

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cfemp {
    ///0: FIFO not empty
    _0 = 0,
    ///1: FIFO empty
    _1 = 1,
}
impl From<Cfemp> for bool {
    #[inline(always)]
    fn from(variant: Cfemp) -> Self {
        variant as u8 != 0
    }
}
///Field `CFEMP` reader - Common FIFO Empty
pub type CfempR = crate::BitReader<Cfemp>;
impl CfempR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cfemp {
        match self.bits {
            false => Cfemp::_0,
            true => Cfemp::_1,
        }
    }
    ///FIFO not empty
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cfemp::_0
    }
    ///FIFO empty
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cfemp::_1
    }
}
/**Common FIFO Full

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cffll {
    ///0: FIFO not full
    _0 = 0,
    ///1: FIFO full
    _1 = 1,
}
impl From<Cffll> for bool {
    #[inline(always)]
    fn from(variant: Cffll) -> Self {
        variant as u8 != 0
    }
}
///Field `CFFLL` reader - Common FIFO Full
pub type CffllR = crate::BitReader<Cffll>;
impl CffllR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cffll {
        match self.bits {
            false => Cffll::_0,
            true => Cffll::_1,
        }
    }
    ///FIFO not full
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cffll::_0
    }
    ///FIFO full
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cffll::_1
    }
}
/**Common FIFO Message Lost

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cfmlt {
    ///0: Number of message lost in FIFO
    _0 = 0,
    ///1: FIFO message lost
    _1 = 1,
}
impl From<Cfmlt> for bool {
    #[inline(always)]
    fn from(variant: Cfmlt) -> Self {
        variant as u8 != 0
    }
}
///Field `CFMLT` reader - Common FIFO Message Lost
pub type CfmltR = crate::BitReader<Cfmlt>;
impl CfmltR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cfmlt {
        match self.bits {
            false => Cfmlt::_0,
            true => Cfmlt::_1,
        }
    }
    ///Number of message lost in FIFO
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cfmlt::_0
    }
    ///FIFO message lost
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cfmlt::_1
    }
}
///Field `CFMLT` writer - Common FIFO Message Lost
pub type CfmltW<'a, REG> = crate::BitWriter<'a, REG, Cfmlt>;
impl<'a, REG> CfmltW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Number of message lost in FIFO
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cfmlt::_0)
    }
    ///FIFO message lost
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cfmlt::_1)
    }
}
/**Common RX FIFO Interrupt Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cfrxif {
    ///0: FIFO interrupt condition not satisfied after frame reception
    _0 = 0,
    ///1: FIFO interrupt condition satisfied after frame reception
    _1 = 1,
}
impl From<Cfrxif> for bool {
    #[inline(always)]
    fn from(variant: Cfrxif) -> Self {
        variant as u8 != 0
    }
}
///Field `CFRXIF` reader - Common RX FIFO Interrupt Flag
pub type CfrxifR = crate::BitReader<Cfrxif>;
impl CfrxifR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cfrxif {
        match self.bits {
            false => Cfrxif::_0,
            true => Cfrxif::_1,
        }
    }
    ///FIFO interrupt condition not satisfied after frame reception
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cfrxif::_0
    }
    ///FIFO interrupt condition satisfied after frame reception
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cfrxif::_1
    }
}
///Field `CFRXIF` writer - Common RX FIFO Interrupt Flag
pub type CfrxifW<'a, REG> = crate::BitWriter<'a, REG, Cfrxif>;
impl<'a, REG> CfrxifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///FIFO interrupt condition not satisfied after frame reception
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cfrxif::_0)
    }
    ///FIFO interrupt condition satisfied after frame reception
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cfrxif::_1)
    }
}
/**Common TX FIFO Interrupt Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cftxif {
    ///0: FIFO interrupt condition not satisfied after frame transmission
    _0 = 0,
    ///1: FIFO Interrupt condition satisfied after frame transmission
    _1 = 1,
}
impl From<Cftxif> for bool {
    #[inline(always)]
    fn from(variant: Cftxif) -> Self {
        variant as u8 != 0
    }
}
///Field `CFTXIF` reader - Common TX FIFO Interrupt Flag
pub type CftxifR = crate::BitReader<Cftxif>;
impl CftxifR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cftxif {
        match self.bits {
            false => Cftxif::_0,
            true => Cftxif::_1,
        }
    }
    ///FIFO interrupt condition not satisfied after frame transmission
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cftxif::_0
    }
    ///FIFO Interrupt condition satisfied after frame transmission
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cftxif::_1
    }
}
///Field `CFTXIF` writer - Common TX FIFO Interrupt Flag
pub type CftxifW<'a, REG> = crate::BitWriter<'a, REG, Cftxif>;
impl<'a, REG> CftxifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///FIFO interrupt condition not satisfied after frame transmission
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cftxif::_0)
    }
    ///FIFO Interrupt condition satisfied after frame transmission
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cftxif::_1)
    }
}
///Field `CFMC` reader - Common FIFO Message Count
pub type CfmcR = crate::FieldReader;
/**Common FIFO Full Interrupt Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cffif {
    ///0: Interrupt condition not satisfied for FIFO full interrupt
    _0 = 0,
    ///1: Interrupt condition satisfied for FIFO full interrupt
    _1 = 1,
}
impl From<Cffif> for bool {
    #[inline(always)]
    fn from(variant: Cffif) -> Self {
        variant as u8 != 0
    }
}
///Field `CFFIF` reader - Common FIFO Full Interrupt Flag
pub type CffifR = crate::BitReader<Cffif>;
impl CffifR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cffif {
        match self.bits {
            false => Cffif::_0,
            true => Cffif::_1,
        }
    }
    ///Interrupt condition not satisfied for FIFO full interrupt
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cffif::_0
    }
    ///Interrupt condition satisfied for FIFO full interrupt
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cffif::_1
    }
}
///Field `CFFIF` writer - Common FIFO Full Interrupt Flag
pub type CffifW<'a, REG> = crate::BitWriter<'a, REG, Cffif>;
impl<'a, REG> CffifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt condition not satisfied for FIFO full interrupt
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cffif::_0)
    }
    ///Interrupt condition satisfied for FIFO full interrupt
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cffif::_1)
    }
}
///Field `CFOFRXIF` reader - Common FIFO One Frame Reception Interrupt Flag
pub type CfofrxifR = crate::BitReader;
///Field `CFOFRXIF` writer - Common FIFO One Frame Reception Interrupt Flag
pub type CfofrxifW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CFOFTXIF` reader - Common FIFO One Frame Transmission Interrupt Flag
pub type CfoftxifR = crate::BitReader;
///Field `CFOFTXIF` writer - Common FIFO One Frame Transmission Interrupt Flag
pub type CfoftxifW<'a, REG> = crate::BitWriter<'a, REG>;
/**Common FIFO Message Overwrite

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cfmow {
    ///0: No message overwrite occurred in FIFO
    _0 = 0,
    ///1: Message overwrite occurred in FIFO
    _1 = 1,
}
impl From<Cfmow> for bool {
    #[inline(always)]
    fn from(variant: Cfmow) -> Self {
        variant as u8 != 0
    }
}
///Field `CFMOW` reader - Common FIFO Message Overwrite
pub type CfmowR = crate::BitReader<Cfmow>;
impl CfmowR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cfmow {
        match self.bits {
            false => Cfmow::_0,
            true => Cfmow::_1,
        }
    }
    ///No message overwrite occurred in FIFO
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cfmow::_0
    }
    ///Message overwrite occurred in FIFO
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cfmow::_1
    }
}
///Field `CFMOW` writer - Common FIFO Message Overwrite
pub type CfmowW<'a, REG> = crate::BitWriter<'a, REG, Cfmow>;
impl<'a, REG> CfmowW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No message overwrite occurred in FIFO
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cfmow::_0)
    }
    ///Message overwrite occurred in FIFO
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cfmow::_1)
    }
}
impl R {
    ///Bit 0 - Common FIFO Empty
    #[inline(always)]
    pub fn cfemp(&self) -> CfempR {
        CfempR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Common FIFO Full
    #[inline(always)]
    pub fn cffll(&self) -> CffllR {
        CffllR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Common FIFO Message Lost
    #[inline(always)]
    pub fn cfmlt(&self) -> CfmltR {
        CfmltR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Common RX FIFO Interrupt Flag
    #[inline(always)]
    pub fn cfrxif(&self) -> CfrxifR {
        CfrxifR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Common TX FIFO Interrupt Flag
    #[inline(always)]
    pub fn cftxif(&self) -> CftxifR {
        CftxifR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 8:15 - Common FIFO Message Count
    #[inline(always)]
    pub fn cfmc(&self) -> CfmcR {
        CfmcR::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bit 16 - Common FIFO Full Interrupt Flag
    #[inline(always)]
    pub fn cffif(&self) -> CffifR {
        CffifR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Common FIFO One Frame Reception Interrupt Flag
    #[inline(always)]
    pub fn cfofrxif(&self) -> CfofrxifR {
        CfofrxifR::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Common FIFO One Frame Transmission Interrupt Flag
    #[inline(always)]
    pub fn cfoftxif(&self) -> CfoftxifR {
        CfoftxifR::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 24 - Common FIFO Message Overwrite
    #[inline(always)]
    pub fn cfmow(&self) -> CfmowR {
        CfmowR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDCFSTS")
            .field("cfemp", &self.cfemp())
            .field("cffll", &self.cffll())
            .field("cfmlt", &self.cfmlt())
            .field("cfrxif", &self.cfrxif())
            .field("cftxif", &self.cftxif())
            .field("cfmc", &self.cfmc())
            .field("cffif", &self.cffif())
            .field("cfofrxif", &self.cfofrxif())
            .field("cfoftxif", &self.cfoftxif())
            .field("cfmow", &self.cfmow())
            .finish()
    }
}
impl W {
    ///Bit 2 - Common FIFO Message Lost
    #[inline(always)]
    pub fn cfmlt(&mut self) -> CfmltW<CfdcfstsSpec> {
        CfmltW::new(self, 2)
    }
    ///Bit 3 - Common RX FIFO Interrupt Flag
    #[inline(always)]
    pub fn cfrxif(&mut self) -> CfrxifW<CfdcfstsSpec> {
        CfrxifW::new(self, 3)
    }
    ///Bit 4 - Common TX FIFO Interrupt Flag
    #[inline(always)]
    pub fn cftxif(&mut self) -> CftxifW<CfdcfstsSpec> {
        CftxifW::new(self, 4)
    }
    ///Bit 16 - Common FIFO Full Interrupt Flag
    #[inline(always)]
    pub fn cffif(&mut self) -> CffifW<CfdcfstsSpec> {
        CffifW::new(self, 16)
    }
    ///Bit 17 - Common FIFO One Frame Reception Interrupt Flag
    #[inline(always)]
    pub fn cfofrxif(&mut self) -> CfofrxifW<CfdcfstsSpec> {
        CfofrxifW::new(self, 17)
    }
    ///Bit 18 - Common FIFO One Frame Transmission Interrupt Flag
    #[inline(always)]
    pub fn cfoftxif(&mut self) -> CfoftxifW<CfdcfstsSpec> {
        CfoftxifW::new(self, 18)
    }
    ///Bit 24 - Common FIFO Message Overwrite
    #[inline(always)]
    pub fn cfmow(&mut self) -> CfmowW<CfdcfstsSpec> {
        CfmowW::new(self, 24)
    }
}
/**Common FIFO Status Registers %s

You can [`read`](crate::Reg::read) this register and get [`cfdcfsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcfsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdcfstsSpec;
impl crate::RegisterSpec for CfdcfstsSpec {
    type Ux = u32;
}
///`read()` method returns [`cfdcfsts::R`](R) reader structure
impl crate::Readable for CfdcfstsSpec {}
///`write(|w| ..)` method takes [`cfdcfsts::W`](W) writer structure
impl crate::Writable for CfdcfstsSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDCFSTS%s to value 0x01
impl crate::Resettable for CfdcfstsSpec {
    const RESET_VALUE: u32 = 0x01;
}
