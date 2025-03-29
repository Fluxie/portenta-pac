///Register `CFDRFFDSTS%s` reader
pub type R = crate::R<CfdrffdstsSpec>;
/**Error State Indicator bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfesi {
    ///0: CAN-FD frame received from error active node
    _0 = 0,
    ///1: CAN-FD frame received from error passive node
    _1 = 1,
}
impl From<Rfesi> for bool {
    #[inline(always)]
    fn from(variant: Rfesi) -> Self {
        variant as u8 != 0
    }
}
///Field `RFESI` reader - Error State Indicator bit
pub type RfesiR = crate::BitReader<Rfesi>;
impl RfesiR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rfesi {
        match self.bits {
            false => Rfesi::_0,
            true => Rfesi::_1,
        }
    }
    ///CAN-FD frame received from error active node
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rfesi::_0
    }
    ///CAN-FD frame received from error passive node
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rfesi::_1
    }
}
/**Bit Rate Switch bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfbrs {
    ///0: CAN-FD frame received with no bit rate switch
    _0 = 0,
    ///1: CAN-FD frame received with bit rate switch
    _1 = 1,
}
impl From<Rfbrs> for bool {
    #[inline(always)]
    fn from(variant: Rfbrs) -> Self {
        variant as u8 != 0
    }
}
///Field `RFBRS` reader - Bit Rate Switch bit
pub type RfbrsR = crate::BitReader<Rfbrs>;
impl RfbrsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rfbrs {
        match self.bits {
            false => Rfbrs::_0,
            true => Rfbrs::_1,
        }
    }
    ///CAN-FD frame received with no bit rate switch
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rfbrs::_0
    }
    ///CAN-FD frame received with bit rate switch
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rfbrs::_1
    }
}
/**CAN FD Format bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rffdf {
    ///0: Non CAN-FD frame received
    _0 = 0,
    ///1: CAN-FD frame received
    _1 = 1,
}
impl From<Rffdf> for bool {
    #[inline(always)]
    fn from(variant: Rffdf) -> Self {
        variant as u8 != 0
    }
}
///Field `RFFDF` reader - CAN FD Format bit
pub type RffdfR = crate::BitReader<Rffdf>;
impl RffdfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rffdf {
        match self.bits {
            false => Rffdf::_0,
            true => Rffdf::_1,
        }
    }
    ///Non CAN-FD frame received
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rffdf::_0
    }
    ///CAN-FD frame received
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rffdf::_1
    }
}
///Field `RFIFL` reader - RX FIFO Buffer Information Label Field
pub type RfiflR = crate::FieldReader;
///Field `CFDRFPTR` reader - RX FIFO Buffer Pointer Field
pub type CfdrfptrR = crate::FieldReader<u16>;
impl R {
    ///Bit 0 - Error State Indicator bit
    #[inline(always)]
    pub fn rfesi(&self) -> RfesiR {
        RfesiR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Bit Rate Switch bit
    #[inline(always)]
    pub fn rfbrs(&self) -> RfbrsR {
        RfbrsR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CAN FD Format bit
    #[inline(always)]
    pub fn rffdf(&self) -> RffdfR {
        RffdfR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 8:9 - RX FIFO Buffer Information Label Field
    #[inline(always)]
    pub fn rfifl(&self) -> RfiflR {
        RfiflR::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 16:31 - RX FIFO Buffer Pointer Field
    #[inline(always)]
    pub fn cfdrfptr(&self) -> CfdrfptrR {
        CfdrfptrR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDRFFDSTS")
            .field("rfesi", &self.rfesi())
            .field("rfbrs", &self.rfbrs())
            .field("rffdf", &self.rffdf())
            .field("rfifl", &self.rfifl())
            .field("cfdrfptr", &self.cfdrfptr())
            .finish()
    }
}
/**RX FIFO Access CAN-FD Status Register %s

You can [`read`](crate::Reg::read) this register and get [`cfdrffdsts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdrffdstsSpec;
impl crate::RegisterSpec for CfdrffdstsSpec {
    type Ux = u32;
}
///`read()` method returns [`cfdrffdsts::R`](R) reader structure
impl crate::Readable for CfdrffdstsSpec {}
///`reset()` method sets CFDRFFDSTS%s to value 0
impl crate::Resettable for CfdrffdstsSpec {}
