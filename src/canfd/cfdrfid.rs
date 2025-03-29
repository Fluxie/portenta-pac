///Register `CFDRFID%s` reader
pub type R = crate::R<CfdrfidSpec>;
///Field `RFID` reader - RX FIFO Buffer ID Field
pub type RfidR = crate::FieldReader<u32>;
/**RX FIFO Buffer RTR bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfrtr {
    ///0: Data frame
    _0 = 0,
    ///1: Remote frame
    _1 = 1,
}
impl From<Rfrtr> for bool {
    #[inline(always)]
    fn from(variant: Rfrtr) -> Self {
        variant as u8 != 0
    }
}
///Field `RFRTR` reader - RX FIFO Buffer RTR bit
pub type RfrtrR = crate::BitReader<Rfrtr>;
impl RfrtrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rfrtr {
        match self.bits {
            false => Rfrtr::_0,
            true => Rfrtr::_1,
        }
    }
    ///Data frame
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rfrtr::_0
    }
    ///Remote frame
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rfrtr::_1
    }
}
/**RX FIFO Buffer IDE bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfide {
    ///0: STD-ID has been received
    _0 = 0,
    ///1: EXT-ID has been received
    _1 = 1,
}
impl From<Rfide> for bool {
    #[inline(always)]
    fn from(variant: Rfide) -> Self {
        variant as u8 != 0
    }
}
///Field `RFIDE` reader - RX FIFO Buffer IDE bit
pub type RfideR = crate::BitReader<Rfide>;
impl RfideR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rfide {
        match self.bits {
            false => Rfide::_0,
            true => Rfide::_1,
        }
    }
    ///STD-ID has been received
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rfide::_0
    }
    ///EXT-ID has been received
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rfide::_1
    }
}
impl R {
    ///Bits 0:28 - RX FIFO Buffer ID Field
    #[inline(always)]
    pub fn rfid(&self) -> RfidR {
        RfidR::new(self.bits & 0x1fff_ffff)
    }
    ///Bit 30 - RX FIFO Buffer RTR bit
    #[inline(always)]
    pub fn rfrtr(&self) -> RfrtrR {
        RfrtrR::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - RX FIFO Buffer IDE bit
    #[inline(always)]
    pub fn rfide(&self) -> RfideR {
        RfideR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDRFID")
            .field("rfid", &self.rfid())
            .field("rfrtr", &self.rfrtr())
            .field("rfide", &self.rfide())
            .finish()
    }
}
/**RX FIFO Access ID Register %s

You can [`read`](crate::Reg::read) this register and get [`cfdrfid::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdrfidSpec;
impl crate::RegisterSpec for CfdrfidSpec {
    type Ux = u32;
}
///`read()` method returns [`cfdrfid::R`](R) reader structure
impl crate::Readable for CfdrfidSpec {}
///`reset()` method sets CFDRFID%s to value 0
impl crate::Resettable for CfdrfidSpec {}
