///Register `FRDRH` reader
pub type R = crate::R<FrdrhSpec>;
/**Multi-Processor Bit Flag

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
///Field `MPB` reader - Multi-Processor Bit Flag
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
/**Receive Data Ready Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dr {
    ///0: Receiving is in progress, or no received data remains in the FRDRH and FRDRL registers after successfully completed reception
    _0 = 0,
    ///1: Next receive data is not received for a period after successfully completed reception
    _1 = 1,
}
impl From<Dr> for bool {
    #[inline(always)]
    fn from(variant: Dr) -> Self {
        variant as u8 != 0
    }
}
///Field `DR` reader - Receive Data Ready Flag
pub type DrR = crate::BitReader<Dr>;
impl DrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dr {
        match self.bits {
            false => Dr::_0,
            true => Dr::_1,
        }
    }
    ///Receiving is in progress, or no received data remains in the FRDRH and FRDRL registers after successfully completed reception
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dr::_0
    }
    ///Next receive data is not received for a period after successfully completed reception
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dr::_1
    }
}
/**Parity Error Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Per {
    ///0: No parity error occurred in the first data of FRDRH and FRDRL
    _0 = 0,
    ///1: Parity error occurred in the first data of FRDRH and FRDRL
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
    ///No parity error occurred in the first data of FRDRH and FRDRL
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Per::_0
    }
    ///Parity error occurred in the first data of FRDRH and FRDRL
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Per::_1
    }
}
/**Framing Error Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fer {
    ///0: No framing error occurred in the first data of FRDRH and FRDRL
    _0 = 0,
    ///1: Framing error occurred in the first data of FRDRH and FRDRL
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
    ///No framing error occurred in the first data of FRDRH and FRDRL
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Fer::_0
    }
    ///Framing error occurred in the first data of FRDRH and FRDRL
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Fer::_1
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
/**Receive FIFO Data Full Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdf {
    ///0: The amount of receive data written in FRDRH and FRDRL is less than the specified receive triggering number
    _0 = 0,
    ///1: The amount of receive data written in FRDRH and FRDRL is equal to or greater than the specified receive triggering number
    _1 = 1,
}
impl From<Rdf> for bool {
    #[inline(always)]
    fn from(variant: Rdf) -> Self {
        variant as u8 != 0
    }
}
///Field `RDF` reader - Receive FIFO Data Full Flag
pub type RdfR = crate::BitReader<Rdf>;
impl RdfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rdf {
        match self.bits {
            false => Rdf::_0,
            true => Rdf::_1,
        }
    }
    ///The amount of receive data written in FRDRH and FRDRL is less than the specified receive triggering number
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rdf::_0
    }
    ///The amount of receive data written in FRDRH and FRDRL is equal to or greater than the specified receive triggering number
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rdf::_1
    }
}
impl R {
    ///Bit 1 - Multi-Processor Bit Flag
    #[inline(always)]
    pub fn mpb(&self) -> MpbR {
        MpbR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Receive Data Ready Flag
    #[inline(always)]
    pub fn dr(&self) -> DrR {
        DrR::new(((self.bits >> 2) & 1) != 0)
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
    ///Bit 6 - Receive FIFO Data Full Flag
    #[inline(always)]
    pub fn rdf(&self) -> RdfR {
        RdfR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FRDRH")
            .field("mpb", &self.mpb())
            .field("dr", &self.dr())
            .field("per", &self.per())
            .field("fer", &self.fer())
            .field("orer", &self.orer())
            .field("rdf", &self.rdf())
            .finish()
    }
}
/**Receive FIFO Data Register

You can [`read`](crate::Reg::read) this register and get [`frdrh::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FrdrhSpec;
impl crate::RegisterSpec for FrdrhSpec {
    type Ux = u8;
}
///`read()` method returns [`frdrh::R`](R) reader structure
impl crate::Readable for FrdrhSpec {}
///`reset()` method sets FRDRH to value 0
impl crate::Resettable for FrdrhSpec {}
