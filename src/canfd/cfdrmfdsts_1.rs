///Register `CFDRMFDSTS%s_1` reader
pub type R = crate::R<Cfdrmfdsts1Spec>;
/**Error State Indicator bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rmesi {
    ///0: CAN-FD frame received from error active node
    _0 = 0,
    ///1: CAN-FD frame received from error passive node
    _1 = 1,
}
impl From<Rmesi> for bool {
    #[inline(always)]
    fn from(variant: Rmesi) -> Self {
        variant as u8 != 0
    }
}
///Field `RMESI` reader - Error State Indicator bit
pub type RmesiR = crate::BitReader<Rmesi>;
impl RmesiR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rmesi {
        match self.bits {
            false => Rmesi::_0,
            true => Rmesi::_1,
        }
    }
    ///CAN-FD frame received from error active node
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rmesi::_0
    }
    ///CAN-FD frame received from error passive node
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rmesi::_1
    }
}
/**Bit Rate Switch bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rmbrs {
    ///0: CAN-FD frame received with no bit rate switch
    _0 = 0,
    ///1: CAN-FD frame received with bit rate switch
    _1 = 1,
}
impl From<Rmbrs> for bool {
    #[inline(always)]
    fn from(variant: Rmbrs) -> Self {
        variant as u8 != 0
    }
}
///Field `RMBRS` reader - Bit Rate Switch bit
pub type RmbrsR = crate::BitReader<Rmbrs>;
impl RmbrsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rmbrs {
        match self.bits {
            false => Rmbrs::_0,
            true => Rmbrs::_1,
        }
    }
    ///CAN-FD frame received with no bit rate switch
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rmbrs::_0
    }
    ///CAN-FD frame received with bit rate switch
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rmbrs::_1
    }
}
/**CAN FD Format bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rmfdf {
    ///0: Non CAN-FD frame received
    _0 = 0,
    ///1: CAN-FD frame received
    _1 = 1,
}
impl From<Rmfdf> for bool {
    #[inline(always)]
    fn from(variant: Rmfdf) -> Self {
        variant as u8 != 0
    }
}
///Field `RMFDF` reader - CAN FD Format bit
pub type RmfdfR = crate::BitReader<Rmfdf>;
impl RmfdfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rmfdf {
        match self.bits {
            false => Rmfdf::_0,
            true => Rmfdf::_1,
        }
    }
    ///Non CAN-FD frame received
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rmfdf::_0
    }
    ///CAN-FD frame received
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rmfdf::_1
    }
}
///Field `RMIFL` reader - RX Message Buffer Information Label Field
pub type RmiflR = crate::FieldReader;
///Field `RMPTR` reader - RX Message Buffer Pointer Field
pub type RmptrR = crate::FieldReader<u16>;
impl R {
    ///Bit 0 - Error State Indicator bit
    #[inline(always)]
    pub fn rmesi(&self) -> RmesiR {
        RmesiR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Bit Rate Switch bit
    #[inline(always)]
    pub fn rmbrs(&self) -> RmbrsR {
        RmbrsR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CAN FD Format bit
    #[inline(always)]
    pub fn rmfdf(&self) -> RmfdfR {
        RmfdfR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 8:9 - RX Message Buffer Information Label Field
    #[inline(always)]
    pub fn rmifl(&self) -> RmiflR {
        RmiflR::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 16:31 - RX Message Buffer Pointer Field
    #[inline(always)]
    pub fn rmptr(&self) -> RmptrR {
        RmptrR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDRMFDSTS_1")
            .field("rmesi", &self.rmesi())
            .field("rmbrs", &self.rmbrs())
            .field("rmfdf", &self.rmfdf())
            .field("rmifl", &self.rmifl())
            .field("rmptr", &self.rmptr())
            .finish()
    }
}
/**RX Message Buffer CAN-FD Status Register %s Channel 1

You can [`read`](crate::Reg::read) this register and get [`cfdrmfdsts_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Cfdrmfdsts1Spec;
impl crate::RegisterSpec for Cfdrmfdsts1Spec {
    type Ux = u32;
}
///`read()` method returns [`cfdrmfdsts_1::R`](R) reader structure
impl crate::Readable for Cfdrmfdsts1Spec {}
///`reset()` method sets CFDRMFDSTS%s_1 to value 0
impl crate::Resettable for Cfdrmfdsts1Spec {}
