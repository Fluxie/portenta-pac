///Register `SD_ERR_STS1` reader
pub type R = crate::R<SdErrSts1Spec>;
/**Command Error Flag 0

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmde0 {
    ///0: No error exists in command index field value of a command response
    _0 = 0,
    ///1: Error exists in command index field value of a command response
    _1 = 1,
}
impl From<Cmde0> for bool {
    #[inline(always)]
    fn from(variant: Cmde0) -> Self {
        variant as u8 != 0
    }
}
///Field `CMDE0` reader - Command Error Flag 0
pub type Cmde0R = crate::BitReader<Cmde0>;
impl Cmde0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmde0 {
        match self.bits {
            false => Cmde0::_0,
            true => Cmde0::_1,
        }
    }
    ///No error exists in command index field value of a command response
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmde0::_0
    }
    ///Error exists in command index field value of a command response
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmde0::_1
    }
}
/**Command Error Flag 1

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmde1 {
    ///0: No error exists in command index field value of a command response
    _0 = 0,
    ///1: Error exists in command index field value of a command response (with SD_CMD.CMDIDX\[5:0\] setting, an error that occurs with CMD12 issue is indicated in the CMDE0 flag)
    _1 = 1,
}
impl From<Cmde1> for bool {
    #[inline(always)]
    fn from(variant: Cmde1) -> Self {
        variant as u8 != 0
    }
}
///Field `CMDE1` reader - Command Error Flag 1
pub type Cmde1R = crate::BitReader<Cmde1>;
impl Cmde1R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmde1 {
        match self.bits {
            false => Cmde1::_0,
            true => Cmde1::_1,
        }
    }
    ///No error exists in command index field value of a command response
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmde1::_0
    }
    ///Error exists in command index field value of a command response (with SD_CMD.CMDIDX\[5:0\] setting, an error that occurs with CMD12 issue is indicated in the CMDE0 flag)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmde1::_1
    }
}
/**Response Length Error Flag 0

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rsplene0 {
    ///0: No error exists in command response length
    _0 = 0,
    ///1: Error exists in command response length
    _1 = 1,
}
impl From<Rsplene0> for bool {
    #[inline(always)]
    fn from(variant: Rsplene0) -> Self {
        variant as u8 != 0
    }
}
///Field `RSPLENE0` reader - Response Length Error Flag 0
pub type Rsplene0R = crate::BitReader<Rsplene0>;
impl Rsplene0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rsplene0 {
        match self.bits {
            false => Rsplene0::_0,
            true => Rsplene0::_1,
        }
    }
    ///No error exists in command response length
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rsplene0::_0
    }
    ///Error exists in command response length
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rsplene0::_1
    }
}
/**Response Length Error Flag 1

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rsplene1 {
    ///0: No error exists in command response length
    _0 = 0,
    ///1: Error exists in command response length (with SD_CMD.CMDIDX\[5:0\] setting, an error that occurs with CMD12 issue is indicated in the RSPLENE0 flag)
    _1 = 1,
}
impl From<Rsplene1> for bool {
    #[inline(always)]
    fn from(variant: Rsplene1) -> Self {
        variant as u8 != 0
    }
}
///Field `RSPLENE1` reader - Response Length Error Flag 1
pub type Rsplene1R = crate::BitReader<Rsplene1>;
impl Rsplene1R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rsplene1 {
        match self.bits {
            false => Rsplene1::_0,
            true => Rsplene1::_1,
        }
    }
    ///No error exists in command response length
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rsplene1::_0
    }
    ///Error exists in command response length (with SD_CMD.CMDIDX\[5:0\] setting, an error that occurs with CMD12 issue is indicated in the RSPLENE0 flag)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rsplene1::_1
    }
}
/**Read Data Length Error Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdlene {
    ///0: No read data length error occurred
    _0 = 0,
    ///1: Read data length error occurred
    _1 = 1,
}
impl From<Rdlene> for bool {
    #[inline(always)]
    fn from(variant: Rdlene) -> Self {
        variant as u8 != 0
    }
}
///Field `RDLENE` reader - Read Data Length Error Flag
pub type RdleneR = crate::BitReader<Rdlene>;
impl RdleneR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rdlene {
        match self.bits {
            false => Rdlene::_0,
            true => Rdlene::_1,
        }
    }
    ///No read data length error occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rdlene::_0
    }
    ///Read data length error occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rdlene::_1
    }
}
/**CRC Status Token Length Error Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crclene {
    ///0: No CRC status token length error occurred
    _0 = 0,
    ///1: CRC status token length error occurred
    _1 = 1,
}
impl From<Crclene> for bool {
    #[inline(always)]
    fn from(variant: Crclene) -> Self {
        variant as u8 != 0
    }
}
///Field `CRCLENE` reader - CRC Status Token Length Error Flag
pub type CrcleneR = crate::BitReader<Crclene>;
impl CrcleneR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Crclene {
        match self.bits {
            false => Crclene::_0,
            true => Crclene::_1,
        }
    }
    ///No CRC status token length error occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Crclene::_0
    }
    ///CRC status token length error occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Crclene::_1
    }
}
/**Response CRC Error Flag 0

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rspcrce0 {
    ///0: No CRC error detected in command response
    _0 = 0,
    ///1: CRC error detected in command response
    _1 = 1,
}
impl From<Rspcrce0> for bool {
    #[inline(always)]
    fn from(variant: Rspcrce0) -> Self {
        variant as u8 != 0
    }
}
///Field `RSPCRCE0` reader - Response CRC Error Flag 0
pub type Rspcrce0R = crate::BitReader<Rspcrce0>;
impl Rspcrce0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rspcrce0 {
        match self.bits {
            false => Rspcrce0::_0,
            true => Rspcrce0::_1,
        }
    }
    ///No CRC error detected in command response
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rspcrce0::_0
    }
    ///CRC error detected in command response
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rspcrce0::_1
    }
}
/**Response CRC Error Flag 1

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rspcrce1 {
    ///0: No CRC error detected in command response (with SD_CMD.CMDIDX\[5:0\] setting, an error that occurs with CMD12 issue is indicated in the RSPCRCE0 flag)
    _0 = 0,
    ///1: CRC error detected in command response
    _1 = 1,
}
impl From<Rspcrce1> for bool {
    #[inline(always)]
    fn from(variant: Rspcrce1) -> Self {
        variant as u8 != 0
    }
}
///Field `RSPCRCE1` reader - Response CRC Error Flag 1
pub type Rspcrce1R = crate::BitReader<Rspcrce1>;
impl Rspcrce1R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rspcrce1 {
        match self.bits {
            false => Rspcrce1::_0,
            true => Rspcrce1::_1,
        }
    }
    ///No CRC error detected in command response (with SD_CMD.CMDIDX\[5:0\] setting, an error that occurs with CMD12 issue is indicated in the RSPCRCE0 flag)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rspcrce1::_0
    }
    ///CRC error detected in command response
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rspcrce1::_1
    }
}
/**Read Data CRC Error Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdcrce {
    ///0: No CRC error detected in read data
    _0 = 0,
    ///1: CRC error detected in read data
    _1 = 1,
}
impl From<Rdcrce> for bool {
    #[inline(always)]
    fn from(variant: Rdcrce) -> Self {
        variant as u8 != 0
    }
}
///Field `RDCRCE` reader - Read Data CRC Error Flag
pub type RdcrceR = crate::BitReader<Rdcrce>;
impl RdcrceR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rdcrce {
        match self.bits {
            false => Rdcrce::_0,
            true => Rdcrce::_1,
        }
    }
    ///No CRC error detected in read data
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rdcrce::_0
    }
    ///CRC error detected in read data
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rdcrce::_1
    }
}
/**CRC Status Token Error Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crctke {
    ///0: No error detected in CRC status token
    _0 = 0,
    ///1: Error detected in CRC status token
    _1 = 1,
}
impl From<Crctke> for bool {
    #[inline(always)]
    fn from(variant: Crctke) -> Self {
        variant as u8 != 0
    }
}
///Field `CRCTKE` reader - CRC Status Token Error Flag
pub type CrctkeR = crate::BitReader<Crctke>;
impl CrctkeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Crctke {
        match self.bits {
            false => Crctke::_0,
            true => Crctke::_1,
        }
    }
    ///No error detected in CRC status token
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Crctke::_0
    }
    ///Error detected in CRC status token
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Crctke::_1
    }
}
///Field `CRCTK` reader - CRC Status Token
pub type CrctkR = crate::FieldReader;
impl R {
    ///Bit 0 - Command Error Flag 0
    #[inline(always)]
    pub fn cmde0(&self) -> Cmde0R {
        Cmde0R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Command Error Flag 1
    #[inline(always)]
    pub fn cmde1(&self) -> Cmde1R {
        Cmde1R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Response Length Error Flag 0
    #[inline(always)]
    pub fn rsplene0(&self) -> Rsplene0R {
        Rsplene0R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Response Length Error Flag 1
    #[inline(always)]
    pub fn rsplene1(&self) -> Rsplene1R {
        Rsplene1R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Read Data Length Error Flag
    #[inline(always)]
    pub fn rdlene(&self) -> RdleneR {
        RdleneR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - CRC Status Token Length Error Flag
    #[inline(always)]
    pub fn crclene(&self) -> CrcleneR {
        CrcleneR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - Response CRC Error Flag 0
    #[inline(always)]
    pub fn rspcrce0(&self) -> Rspcrce0R {
        Rspcrce0R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Response CRC Error Flag 1
    #[inline(always)]
    pub fn rspcrce1(&self) -> Rspcrce1R {
        Rspcrce1R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Read Data CRC Error Flag
    #[inline(always)]
    pub fn rdcrce(&self) -> RdcrceR {
        RdcrceR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - CRC Status Token Error Flag
    #[inline(always)]
    pub fn crctke(&self) -> CrctkeR {
        CrctkeR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:14 - CRC Status Token
    #[inline(always)]
    pub fn crctk(&self) -> CrctkR {
        CrctkR::new(((self.bits >> 12) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SD_ERR_STS1")
            .field("cmde0", &self.cmde0())
            .field("cmde1", &self.cmde1())
            .field("rsplene0", &self.rsplene0())
            .field("rsplene1", &self.rsplene1())
            .field("rdlene", &self.rdlene())
            .field("crclene", &self.crclene())
            .field("rspcrce0", &self.rspcrce0())
            .field("rspcrce1", &self.rspcrce1())
            .field("rdcrce", &self.rdcrce())
            .field("crctke", &self.crctke())
            .field("crctk", &self.crctk())
            .finish()
    }
}
/**SD Error Status Register 1

You can [`read`](crate::Reg::read) this register and get [`sd_err_sts1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SdErrSts1Spec;
impl crate::RegisterSpec for SdErrSts1Spec {
    type Ux = u32;
}
///`read()` method returns [`sd_err_sts1::R`](R) reader structure
impl crate::Readable for SdErrSts1Spec {}
///`reset()` method sets SD_ERR_STS1 to value 0x2000
impl crate::Resettable for SdErrSts1Spec {
    const RESET_VALUE: u32 = 0x2000;
}
