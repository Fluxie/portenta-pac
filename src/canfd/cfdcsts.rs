///Register `CFDC%sSTS` reader
pub type R = crate::R<CfdcstsSpec>;
///Register `CFDC%sSTS` writer
pub type W = crate::W<CfdcstsSpec>;
/**Channel Reset Status

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crststs {
    ///0: Channel not in Reset mode
    _0 = 0,
    ///1: Channel in Reset mode
    _1 = 1,
}
impl From<Crststs> for bool {
    #[inline(always)]
    fn from(variant: Crststs) -> Self {
        variant as u8 != 0
    }
}
///Field `CRSTSTS` reader - Channel Reset Status
pub type CrststsR = crate::BitReader<Crststs>;
impl CrststsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Crststs {
        match self.bits {
            false => Crststs::_0,
            true => Crststs::_1,
        }
    }
    ///Channel not in Reset mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Crststs::_0
    }
    ///Channel in Reset mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Crststs::_1
    }
}
/**Channel Halt Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chltsts {
    ///0: Channel not in Halt mode
    _0 = 0,
    ///1: Channel in Halt mode
    _1 = 1,
}
impl From<Chltsts> for bool {
    #[inline(always)]
    fn from(variant: Chltsts) -> Self {
        variant as u8 != 0
    }
}
///Field `CHLTSTS` reader - Channel Halt Status
pub type ChltstsR = crate::BitReader<Chltsts>;
impl ChltstsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Chltsts {
        match self.bits {
            false => Chltsts::_0,
            true => Chltsts::_1,
        }
    }
    ///Channel not in Halt mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Chltsts::_0
    }
    ///Channel in Halt mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Chltsts::_1
    }
}
/**Channel Sleep Status

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cslpsts {
    ///0: Channel not in Sleep mode
    _0 = 0,
    ///1: Channel in Sleep mode
    _1 = 1,
}
impl From<Cslpsts> for bool {
    #[inline(always)]
    fn from(variant: Cslpsts) -> Self {
        variant as u8 != 0
    }
}
///Field `CSLPSTS` reader - Channel Sleep Status
pub type CslpstsR = crate::BitReader<Cslpsts>;
impl CslpstsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cslpsts {
        match self.bits {
            false => Cslpsts::_0,
            true => Cslpsts::_1,
        }
    }
    ///Channel not in Sleep mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cslpsts::_0
    }
    ///Channel in Sleep mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cslpsts::_1
    }
}
/**Channel Error Passive Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Epsts {
    ///0: Channel not in error passive state
    _0 = 0,
    ///1: Channel in error passive state
    _1 = 1,
}
impl From<Epsts> for bool {
    #[inline(always)]
    fn from(variant: Epsts) -> Self {
        variant as u8 != 0
    }
}
///Field `EPSTS` reader - Channel Error Passive Status
pub type EpstsR = crate::BitReader<Epsts>;
impl EpstsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Epsts {
        match self.bits {
            false => Epsts::_0,
            true => Epsts::_1,
        }
    }
    ///Channel not in error passive state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Epsts::_0
    }
    ///Channel in error passive state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Epsts::_1
    }
}
/**Channel Bus-Off Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bosts {
    ///0: Channel not in bus-off state
    _0 = 0,
    ///1: Channel in bus-off state
    _1 = 1,
}
impl From<Bosts> for bool {
    #[inline(always)]
    fn from(variant: Bosts) -> Self {
        variant as u8 != 0
    }
}
///Field `BOSTS` reader - Channel Bus-Off Status
pub type BostsR = crate::BitReader<Bosts>;
impl BostsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Bosts {
        match self.bits {
            false => Bosts::_0,
            true => Bosts::_1,
        }
    }
    ///Channel not in bus-off state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bosts::_0
    }
    ///Channel in bus-off state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bosts::_1
    }
}
/**Channel Transmit Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trmsts {
    ///0: Channel is not transmitting
    _0 = 0,
    ///1: Channel is transmitting
    _1 = 1,
}
impl From<Trmsts> for bool {
    #[inline(always)]
    fn from(variant: Trmsts) -> Self {
        variant as u8 != 0
    }
}
///Field `TRMSTS` reader - Channel Transmit Status
pub type TrmstsR = crate::BitReader<Trmsts>;
impl TrmstsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Trmsts {
        match self.bits {
            false => Trmsts::_0,
            true => Trmsts::_1,
        }
    }
    ///Channel is not transmitting
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Trmsts::_0
    }
    ///Channel is transmitting
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Trmsts::_1
    }
}
/**Channel Receive Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Recsts {
    ///0: Channel is not receiving
    _0 = 0,
    ///1: Channel is receiving
    _1 = 1,
}
impl From<Recsts> for bool {
    #[inline(always)]
    fn from(variant: Recsts) -> Self {
        variant as u8 != 0
    }
}
///Field `RECSTS` reader - Channel Receive Status
pub type RecstsR = crate::BitReader<Recsts>;
impl RecstsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Recsts {
        match self.bits {
            false => Recsts::_0,
            true => Recsts::_1,
        }
    }
    ///Channel is not receiving
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Recsts::_0
    }
    ///Channel is receiving
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Recsts::_1
    }
}
/**Channel Communication Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Comsts {
    ///0: Channel is not ready for communication
    _0 = 0,
    ///1: Channel is ready for communication
    _1 = 1,
}
impl From<Comsts> for bool {
    #[inline(always)]
    fn from(variant: Comsts) -> Self {
        variant as u8 != 0
    }
}
///Field `COMSTS` reader - Channel Communication Status
pub type ComstsR = crate::BitReader<Comsts>;
impl ComstsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Comsts {
        match self.bits {
            false => Comsts::_0,
            true => Comsts::_1,
        }
    }
    ///Channel is not ready for communication
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Comsts::_0
    }
    ///Channel is ready for communication
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Comsts::_1
    }
}
/**Error State Indication Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Esif {
    ///0: No CANFD message has been received when the ESI flag was set
    _0 = 0,
    ///1: At least one CANFD message was received when the ESI flag was set
    _1 = 1,
}
impl From<Esif> for bool {
    #[inline(always)]
    fn from(variant: Esif) -> Self {
        variant as u8 != 0
    }
}
///Field `ESIF` reader - Error State Indication Flag
pub type EsifR = crate::BitReader<Esif>;
impl EsifR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Esif {
        match self.bits {
            false => Esif::_0,
            true => Esif::_1,
        }
    }
    ///No CANFD message has been received when the ESI flag was set
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Esif::_0
    }
    ///At least one CANFD message was received when the ESI flag was set
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Esif::_1
    }
}
///Field `ESIF` writer - Error State Indication Flag
pub type EsifW<'a, REG> = crate::BitWriter<'a, REG, Esif>;
impl<'a, REG> EsifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No CANFD message has been received when the ESI flag was set
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Esif::_0)
    }
    ///At least one CANFD message was received when the ESI flag was set
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Esif::_1)
    }
}
///Field `REC` reader - Reception Error Count
pub type RecR = crate::FieldReader;
///Field `TEC` reader - Transmission Error Count
pub type TecR = crate::FieldReader;
impl R {
    ///Bit 0 - Channel Reset Status
    #[inline(always)]
    pub fn crststs(&self) -> CrststsR {
        CrststsR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Channel Halt Status
    #[inline(always)]
    pub fn chltsts(&self) -> ChltstsR {
        ChltstsR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Channel Sleep Status
    #[inline(always)]
    pub fn cslpsts(&self) -> CslpstsR {
        CslpstsR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Channel Error Passive Status
    #[inline(always)]
    pub fn epsts(&self) -> EpstsR {
        EpstsR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Channel Bus-Off Status
    #[inline(always)]
    pub fn bosts(&self) -> BostsR {
        BostsR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Channel Transmit Status
    #[inline(always)]
    pub fn trmsts(&self) -> TrmstsR {
        TrmstsR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Channel Receive Status
    #[inline(always)]
    pub fn recsts(&self) -> RecstsR {
        RecstsR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Channel Communication Status
    #[inline(always)]
    pub fn comsts(&self) -> ComstsR {
        ComstsR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Error State Indication Flag
    #[inline(always)]
    pub fn esif(&self) -> EsifR {
        EsifR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 16:23 - Reception Error Count
    #[inline(always)]
    pub fn rec(&self) -> RecR {
        RecR::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Transmission Error Count
    #[inline(always)]
    pub fn tec(&self) -> TecR {
        TecR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDCSTS")
            .field("crststs", &self.crststs())
            .field("chltsts", &self.chltsts())
            .field("cslpsts", &self.cslpsts())
            .field("epsts", &self.epsts())
            .field("bosts", &self.bosts())
            .field("trmsts", &self.trmsts())
            .field("recsts", &self.recsts())
            .field("comsts", &self.comsts())
            .field("esif", &self.esif())
            .field("rec", &self.rec())
            .field("tec", &self.tec())
            .finish()
    }
}
impl W {
    ///Bit 8 - Error State Indication Flag
    #[inline(always)]
    pub fn esif(&mut self) -> EsifW<CfdcstsSpec> {
        EsifW::new(self, 8)
    }
}
/**Channel %s Status Registers

You can [`read`](crate::Reg::read) this register and get [`cfdcsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdcstsSpec;
impl crate::RegisterSpec for CfdcstsSpec {
    type Ux = u32;
}
///`read()` method returns [`cfdcsts::R`](R) reader structure
impl crate::Readable for CfdcstsSpec {}
///`write(|w| ..)` method takes [`cfdcsts::W`](W) writer structure
impl crate::Writable for CfdcstsSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDC%sSTS to value 0x05
impl crate::Resettable for CfdcstsSpec {
    const RESET_VALUE: u32 = 0x05;
}
