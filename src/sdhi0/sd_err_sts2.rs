///Register `SD_ERR_STS2` reader
pub type R = crate::R<SdErrSts2Spec>;
/**Response Timeout Flag 0

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rspto0 {
    ///0: After command was issued, response was received in less than 640 cycles of the SD/MMC clock
    _0 = 0,
    ///1: After command was issued, response was not received in 640 or more cycles of the SD/MMC clock
    _1 = 1,
}
impl From<Rspto0> for bool {
    #[inline(always)]
    fn from(variant: Rspto0) -> Self {
        variant as u8 != 0
    }
}
///Field `RSPTO0` reader - Response Timeout Flag 0
pub type Rspto0R = crate::BitReader<Rspto0>;
impl Rspto0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rspto0 {
        match self.bits {
            false => Rspto0::_0,
            true => Rspto0::_1,
        }
    }
    ///After command was issued, response was received in less than 640 cycles of the SD/MMC clock
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rspto0::_0
    }
    ///After command was issued, response was not received in 640 or more cycles of the SD/MMC clock
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rspto0::_1
    }
}
/**Response Timeout Flag 1

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rspto1 {
    ///0: After command was issued, response was received in less than 640 cycles of the SD/MMC clock
    _0 = 0,
    ///1: After command was issued, response was not received after 640 or more cycles of the SD/MMC clock (with SD_CMD.CMDIDX\[5:0\] setting, an error that occurs with CMD12 issue is indicated in the RSPTO0 flag)
    _1 = 1,
}
impl From<Rspto1> for bool {
    #[inline(always)]
    fn from(variant: Rspto1) -> Self {
        variant as u8 != 0
    }
}
///Field `RSPTO1` reader - Response Timeout Flag 1
pub type Rspto1R = crate::BitReader<Rspto1>;
impl Rspto1R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rspto1 {
        match self.bits {
            false => Rspto1::_0,
            true => Rspto1::_1,
        }
    }
    ///After command was issued, response was received in less than 640 cycles of the SD/MMC clock
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rspto1::_0
    }
    ///After command was issued, response was not received after 640 or more cycles of the SD/MMC clock (with SD_CMD.CMDIDX\[5:0\] setting, an error that occurs with CMD12 issue is indicated in the RSPTO0 flag)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rspto1::_1
    }
}
/**Busy Timeout Flag 0

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bsyto0 {
    ///0: After R1b response was received, SD/MMC was released from the busy state during the specified period
    _0 = 0,
    ///1: After R1b response was received, SD/MMC was in the busy state after the specified period elapsed
    _1 = 1,
}
impl From<Bsyto0> for bool {
    #[inline(always)]
    fn from(variant: Bsyto0) -> Self {
        variant as u8 != 0
    }
}
///Field `BSYTO0` reader - Busy Timeout Flag 0
pub type Bsyto0R = crate::BitReader<Bsyto0>;
impl Bsyto0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Bsyto0 {
        match self.bits {
            false => Bsyto0::_0,
            true => Bsyto0::_1,
        }
    }
    ///After R1b response was received, SD/MMC was released from the busy state during the specified period
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bsyto0::_0
    }
    ///After R1b response was received, SD/MMC was in the busy state after the specified period elapsed
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bsyto0::_1
    }
}
/**Busy Timeout Flag 1

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bsyto1 {
    ///0: After CMD12 was automatically issued, SD/MMC was released from the busy state during the specified period
    _0 = 0,
    ///1: After CMD12 was automatically issued, SD/MMC was in the busy state after the specified period elapsed (with SD_CMD.CMDIDX\[5:0\] setting, an error that occurs with CMD12 issue is indicated in the BSYTO0 flag)
    _1 = 1,
}
impl From<Bsyto1> for bool {
    #[inline(always)]
    fn from(variant: Bsyto1) -> Self {
        variant as u8 != 0
    }
}
///Field `BSYTO1` reader - Busy Timeout Flag 1
pub type Bsyto1R = crate::BitReader<Bsyto1>;
impl Bsyto1R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Bsyto1 {
        match self.bits {
            false => Bsyto1::_0,
            true => Bsyto1::_1,
        }
    }
    ///After CMD12 was automatically issued, SD/MMC was released from the busy state during the specified period
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bsyto1::_0
    }
    ///After CMD12 was automatically issued, SD/MMC was in the busy state after the specified period elapsed (with SD_CMD.CMDIDX\[5:0\] setting, an error that occurs with CMD12 issue is indicated in the BSYTO0 flag)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bsyto1::_1
    }
}
///Field `RDTO` reader - Read Data Timeout Flag
pub type RdtoR = crate::BitReader;
/**CRC Status Token Timeout Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crcto {
    ///0: After CRC data was written to the SD card/MMC, a CRC status token was received during the specified period
    _0 = 0,
    ///1: After CRC data was written to the SD card/MMC, a CRC status token was not received after the specified period elapsed
    _1 = 1,
}
impl From<Crcto> for bool {
    #[inline(always)]
    fn from(variant: Crcto) -> Self {
        variant as u8 != 0
    }
}
///Field `CRCTO` reader - CRC Status Token Timeout Flag
pub type CrctoR = crate::BitReader<Crcto>;
impl CrctoR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Crcto {
        match self.bits {
            false => Crcto::_0,
            true => Crcto::_1,
        }
    }
    ///After CRC data was written to the SD card/MMC, a CRC status token was received during the specified period
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Crcto::_0
    }
    ///After CRC data was written to the SD card/MMC, a CRC status token was not received after the specified period elapsed
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Crcto::_1
    }
}
/**CRC Status Token Busy Timeout Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crcbsyto {
    ///0: After a CRC status token was received, the SD/MMC was released from the busy state during the specified period
    _0 = 0,
    ///1: After a CRC status token was received, the SD/MMC was in the busy state after the specified period elapsed
    _1 = 1,
}
impl From<Crcbsyto> for bool {
    #[inline(always)]
    fn from(variant: Crcbsyto) -> Self {
        variant as u8 != 0
    }
}
///Field `CRCBSYTO` reader - CRC Status Token Busy Timeout Flag
pub type CrcbsytoR = crate::BitReader<Crcbsyto>;
impl CrcbsytoR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Crcbsyto {
        match self.bits {
            false => Crcbsyto::_0,
            true => Crcbsyto::_1,
        }
    }
    ///After a CRC status token was received, the SD/MMC was released from the busy state during the specified period
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Crcbsyto::_0
    }
    ///After a CRC status token was received, the SD/MMC was in the busy state after the specified period elapsed
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Crcbsyto::_1
    }
}
impl R {
    ///Bit 0 - Response Timeout Flag 0
    #[inline(always)]
    pub fn rspto0(&self) -> Rspto0R {
        Rspto0R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Response Timeout Flag 1
    #[inline(always)]
    pub fn rspto1(&self) -> Rspto1R {
        Rspto1R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Busy Timeout Flag 0
    #[inline(always)]
    pub fn bsyto0(&self) -> Bsyto0R {
        Bsyto0R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Busy Timeout Flag 1
    #[inline(always)]
    pub fn bsyto1(&self) -> Bsyto1R {
        Bsyto1R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Read Data Timeout Flag
    #[inline(always)]
    pub fn rdto(&self) -> RdtoR {
        RdtoR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - CRC Status Token Timeout Flag
    #[inline(always)]
    pub fn crcto(&self) -> CrctoR {
        CrctoR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - CRC Status Token Busy Timeout Flag
    #[inline(always)]
    pub fn crcbsyto(&self) -> CrcbsytoR {
        CrcbsytoR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SD_ERR_STS2")
            .field("rspto0", &self.rspto0())
            .field("rspto1", &self.rspto1())
            .field("bsyto0", &self.bsyto0())
            .field("bsyto1", &self.bsyto1())
            .field("rdto", &self.rdto())
            .field("crcto", &self.crcto())
            .field("crcbsyto", &self.crcbsyto())
            .finish()
    }
}
/**SD Error Status Register 2

You can [`read`](crate::Reg::read) this register and get [`sd_err_sts2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SdErrSts2Spec;
impl crate::RegisterSpec for SdErrSts2Spec {
    type Ux = u32;
}
///`read()` method returns [`sd_err_sts2::R`](R) reader structure
impl crate::Readable for SdErrSts2Spec {}
///`reset()` method sets SD_ERR_STS2 to value 0
impl crate::Resettable for SdErrSts2Spec {}
