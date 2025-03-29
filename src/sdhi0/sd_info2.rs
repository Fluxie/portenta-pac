///Register `SD_INFO2` reader
pub type R = crate::R<SdInfo2Spec>;
///Register `SD_INFO2` writer
pub type W = crate::W<SdInfo2Spec>;
/**Command Error Detection Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmde {
    ///0: Command error not detected
    _0 = 0,
    ///1: Command error detected
    _1 = 1,
}
impl From<Cmde> for bool {
    #[inline(always)]
    fn from(variant: Cmde) -> Self {
        variant as u8 != 0
    }
}
///Field `CMDE` reader - Command Error Detection Flag
pub type CmdeR = crate::BitReader<Cmde>;
impl CmdeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmde {
        match self.bits {
            false => Cmde::_0,
            true => Cmde::_1,
        }
    }
    ///Command error not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmde::_0
    }
    ///Command error detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmde::_1
    }
}
///Field `CMDE` writer - Command Error Detection Flag
pub type CmdeW<'a, REG> = crate::BitWriter<'a, REG, Cmde>;
impl<'a, REG> CmdeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Command error not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmde::_0)
    }
    ///Command error detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmde::_1)
    }
}
/**CRC Error Detection Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crce {
    ///0: CRC error not detected
    _0 = 0,
    ///1: CRC error detected
    _1 = 1,
}
impl From<Crce> for bool {
    #[inline(always)]
    fn from(variant: Crce) -> Self {
        variant as u8 != 0
    }
}
///Field `CRCE` reader - CRC Error Detection Flag
pub type CrceR = crate::BitReader<Crce>;
impl CrceR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Crce {
        match self.bits {
            false => Crce::_0,
            true => Crce::_1,
        }
    }
    ///CRC error not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Crce::_0
    }
    ///CRC error detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Crce::_1
    }
}
///Field `CRCE` writer - CRC Error Detection Flag
pub type CrceW<'a, REG> = crate::BitWriter<'a, REG, Crce>;
impl<'a, REG> CrceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CRC error not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Crce::_0)
    }
    ///CRC error detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Crce::_1)
    }
}
/**End Bit Error Detection Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ende {
    ///0: End bit error not detected
    _0 = 0,
    ///1: End bit error detected
    _1 = 1,
}
impl From<Ende> for bool {
    #[inline(always)]
    fn from(variant: Ende) -> Self {
        variant as u8 != 0
    }
}
///Field `ENDE` reader - End Bit Error Detection Flag
pub type EndeR = crate::BitReader<Ende>;
impl EndeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ende {
        match self.bits {
            false => Ende::_0,
            true => Ende::_1,
        }
    }
    ///End bit error not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ende::_0
    }
    ///End bit error detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ende::_1
    }
}
///Field `ENDE` writer - End Bit Error Detection Flag
pub type EndeW<'a, REG> = crate::BitWriter<'a, REG, Ende>;
impl<'a, REG> EndeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///End bit error not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ende::_0)
    }
    ///End bit error detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ende::_1)
    }
}
/**Data Timeout Detection Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dto {
    ///0: Data timeout not detected
    _0 = 0,
    ///1: Data timeout detected
    _1 = 1,
}
impl From<Dto> for bool {
    #[inline(always)]
    fn from(variant: Dto) -> Self {
        variant as u8 != 0
    }
}
///Field `DTO` reader - Data Timeout Detection Flag
pub type DtoR = crate::BitReader<Dto>;
impl DtoR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dto {
        match self.bits {
            false => Dto::_0,
            true => Dto::_1,
        }
    }
    ///Data timeout not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dto::_0
    }
    ///Data timeout detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dto::_1
    }
}
///Field `DTO` writer - Data Timeout Detection Flag
pub type DtoW<'a, REG> = crate::BitWriter<'a, REG, Dto>;
impl<'a, REG> DtoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Data timeout not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dto::_0)
    }
    ///Data timeout detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dto::_1)
    }
}
/**SD_BUF0 Illegal Write Access Detection Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ilw {
    ///0: Illegal write access to the SD_BUF0 register not detected
    _0 = 0,
    ///1: Illegal write access to the SD_BUF0 register detected
    _1 = 1,
}
impl From<Ilw> for bool {
    #[inline(always)]
    fn from(variant: Ilw) -> Self {
        variant as u8 != 0
    }
}
///Field `ILW` reader - SD_BUF0 Illegal Write Access Detection Flag
pub type IlwR = crate::BitReader<Ilw>;
impl IlwR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ilw {
        match self.bits {
            false => Ilw::_0,
            true => Ilw::_1,
        }
    }
    ///Illegal write access to the SD_BUF0 register not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ilw::_0
    }
    ///Illegal write access to the SD_BUF0 register detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ilw::_1
    }
}
///Field `ILW` writer - SD_BUF0 Illegal Write Access Detection Flag
pub type IlwW<'a, REG> = crate::BitWriter<'a, REG, Ilw>;
impl<'a, REG> IlwW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Illegal write access to the SD_BUF0 register not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ilw::_0)
    }
    ///Illegal write access to the SD_BUF0 register detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ilw::_1)
    }
}
/**SD_BUF0 Illegal Read Access Detection Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ilr {
    ///0: Illegal read access to the SD_BUF0 register not detected
    _0 = 0,
    ///1: Illegal read access to the SD_BUF0 register detected
    _1 = 1,
}
impl From<Ilr> for bool {
    #[inline(always)]
    fn from(variant: Ilr) -> Self {
        variant as u8 != 0
    }
}
///Field `ILR` reader - SD_BUF0 Illegal Read Access Detection Flag
pub type IlrR = crate::BitReader<Ilr>;
impl IlrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ilr {
        match self.bits {
            false => Ilr::_0,
            true => Ilr::_1,
        }
    }
    ///Illegal read access to the SD_BUF0 register not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ilr::_0
    }
    ///Illegal read access to the SD_BUF0 register detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ilr::_1
    }
}
///Field `ILR` writer - SD_BUF0 Illegal Read Access Detection Flag
pub type IlrW<'a, REG> = crate::BitWriter<'a, REG, Ilr>;
impl<'a, REG> IlrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Illegal read access to the SD_BUF0 register not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ilr::_0)
    }
    ///Illegal read access to the SD_BUF0 register detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ilr::_1)
    }
}
/**Response Timeout Detection Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rspto {
    ///0: Response timeout not detected
    _0 = 0,
    ///1: Response timeout detected
    _1 = 1,
}
impl From<Rspto> for bool {
    #[inline(always)]
    fn from(variant: Rspto) -> Self {
        variant as u8 != 0
    }
}
///Field `RSPTO` reader - Response Timeout Detection Flag
pub type RsptoR = crate::BitReader<Rspto>;
impl RsptoR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rspto {
        match self.bits {
            false => Rspto::_0,
            true => Rspto::_1,
        }
    }
    ///Response timeout not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rspto::_0
    }
    ///Response timeout detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rspto::_1
    }
}
///Field `RSPTO` writer - Response Timeout Detection Flag
pub type RsptoW<'a, REG> = crate::BitWriter<'a, REG, Rspto>;
impl<'a, REG> RsptoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Response timeout not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rspto::_0)
    }
    ///Response timeout detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rspto::_1)
    }
}
/**SDnDAT0 Pin Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdd0mon {
    ///0: SDnDAT0 pin is low
    _0 = 0,
    ///1: SDnDAT0 pin is high
    _1 = 1,
}
impl From<Sdd0mon> for bool {
    #[inline(always)]
    fn from(variant: Sdd0mon) -> Self {
        variant as u8 != 0
    }
}
///Field `SDD0MON` reader - SDnDAT0 Pin Status Flag
pub type Sdd0monR = crate::BitReader<Sdd0mon>;
impl Sdd0monR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sdd0mon {
        match self.bits {
            false => Sdd0mon::_0,
            true => Sdd0mon::_1,
        }
    }
    ///SDnDAT0 pin is low
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sdd0mon::_0
    }
    ///SDnDAT0 pin is high
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sdd0mon::_1
    }
}
/**SD_BUF0 Read Enable Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bre {
    ///0: Disable read access to the SD_BUF0 register
    _0 = 0,
    ///1: Enable read access to the SD_BUF0 register
    _1 = 1,
}
impl From<Bre> for bool {
    #[inline(always)]
    fn from(variant: Bre) -> Self {
        variant as u8 != 0
    }
}
///Field `BRE` reader - SD_BUF0 Read Enable Flag
pub type BreR = crate::BitReader<Bre>;
impl BreR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Bre {
        match self.bits {
            false => Bre::_0,
            true => Bre::_1,
        }
    }
    ///Disable read access to the SD_BUF0 register
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bre::_0
    }
    ///Enable read access to the SD_BUF0 register
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bre::_1
    }
}
///Field `BRE` writer - SD_BUF0 Read Enable Flag
pub type BreW<'a, REG> = crate::BitWriter<'a, REG, Bre>;
impl<'a, REG> BreW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable read access to the SD_BUF0 register
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bre::_0)
    }
    ///Enable read access to the SD_BUF0 register
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bre::_1)
    }
}
/**SD_BUF0 Write Enable Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bwe {
    ///0: Disable write access to the SD_BUF0 register
    _0 = 0,
    ///1: Enable write access to the SD_BUF0 register
    _1 = 1,
}
impl From<Bwe> for bool {
    #[inline(always)]
    fn from(variant: Bwe) -> Self {
        variant as u8 != 0
    }
}
///Field `BWE` reader - SD_BUF0 Write Enable Flag
pub type BweR = crate::BitReader<Bwe>;
impl BweR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Bwe {
        match self.bits {
            false => Bwe::_0,
            true => Bwe::_1,
        }
    }
    ///Disable write access to the SD_BUF0 register
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bwe::_0
    }
    ///Enable write access to the SD_BUF0 register
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bwe::_1
    }
}
///Field `BWE` writer - SD_BUF0 Write Enable Flag
pub type BweW<'a, REG> = crate::BitWriter<'a, REG, Bwe>;
impl<'a, REG> BweW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable write access to the SD_BUF0 register
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bwe::_0)
    }
    ///Enable write access to the SD_BUF0 register
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bwe::_1)
    }
}
/**SD_CLK_CTRL Write Enable Flag

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SdClkCtrlen {
    ///0: SD/MMC bus (CMD and DAT lines) is busy, so write access to the SD_CLK_CTRL.CLKEN and CLKSEL\[7:0\] bits is disabled
    _0 = 0,
    ///1: SD/MMC bus (CMD and DAT lines) is not busy, so write access to the SD_CLK_CTRL.CLKEN and CLKSEL\[7:0\] bits is enabled
    _1 = 1,
}
impl From<SdClkCtrlen> for bool {
    #[inline(always)]
    fn from(variant: SdClkCtrlen) -> Self {
        variant as u8 != 0
    }
}
///Field `SD_CLK_CTRLEN` reader - SD_CLK_CTRL Write Enable Flag
pub type SdClkCtrlenR = crate::BitReader<SdClkCtrlen>;
impl SdClkCtrlenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SdClkCtrlen {
        match self.bits {
            false => SdClkCtrlen::_0,
            true => SdClkCtrlen::_1,
        }
    }
    ///SD/MMC bus (CMD and DAT lines) is busy, so write access to the SD_CLK_CTRL.CLKEN and CLKSEL\[7:0\] bits is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SdClkCtrlen::_0
    }
    ///SD/MMC bus (CMD and DAT lines) is not busy, so write access to the SD_CLK_CTRL.CLKEN and CLKSEL\[7:0\] bits is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SdClkCtrlen::_1
    }
}
/**Command Sequence Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cbsy {
    ///0: Command sequence complete
    _0 = 0,
    ///1: Command sequence in progress (busy)
    _1 = 1,
}
impl From<Cbsy> for bool {
    #[inline(always)]
    fn from(variant: Cbsy) -> Self {
        variant as u8 != 0
    }
}
///Field `CBSY` reader - Command Sequence Status Flag
pub type CbsyR = crate::BitReader<Cbsy>;
impl CbsyR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cbsy {
        match self.bits {
            false => Cbsy::_0,
            true => Cbsy::_1,
        }
    }
    ///Command sequence complete
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cbsy::_0
    }
    ///Command sequence in progress (busy)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cbsy::_1
    }
}
/**Illegal Access Error Detection Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ila {
    ///0: Illegal access error not detected
    _0 = 0,
    ///1: Illegal access error detected
    _1 = 1,
}
impl From<Ila> for bool {
    #[inline(always)]
    fn from(variant: Ila) -> Self {
        variant as u8 != 0
    }
}
///Field `ILA` reader - Illegal Access Error Detection Flag
pub type IlaR = crate::BitReader<Ila>;
impl IlaR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ila {
        match self.bits {
            false => Ila::_0,
            true => Ila::_1,
        }
    }
    ///Illegal access error not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ila::_0
    }
    ///Illegal access error detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ila::_1
    }
}
///Field `ILA` writer - Illegal Access Error Detection Flag
pub type IlaW<'a, REG> = crate::BitWriter<'a, REG, Ila>;
impl<'a, REG> IlaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Illegal access error not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ila::_0)
    }
    ///Illegal access error detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ila::_1)
    }
}
impl R {
    ///Bit 0 - Command Error Detection Flag
    #[inline(always)]
    pub fn cmde(&self) -> CmdeR {
        CmdeR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CRC Error Detection Flag
    #[inline(always)]
    pub fn crce(&self) -> CrceR {
        CrceR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - End Bit Error Detection Flag
    #[inline(always)]
    pub fn ende(&self) -> EndeR {
        EndeR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Data Timeout Detection Flag
    #[inline(always)]
    pub fn dto(&self) -> DtoR {
        DtoR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - SD_BUF0 Illegal Write Access Detection Flag
    #[inline(always)]
    pub fn ilw(&self) -> IlwR {
        IlwR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - SD_BUF0 Illegal Read Access Detection Flag
    #[inline(always)]
    pub fn ilr(&self) -> IlrR {
        IlrR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Response Timeout Detection Flag
    #[inline(always)]
    pub fn rspto(&self) -> RsptoR {
        RsptoR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - SDnDAT0 Pin Status Flag
    #[inline(always)]
    pub fn sdd0mon(&self) -> Sdd0monR {
        Sdd0monR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - SD_BUF0 Read Enable Flag
    #[inline(always)]
    pub fn bre(&self) -> BreR {
        BreR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - SD_BUF0 Write Enable Flag
    #[inline(always)]
    pub fn bwe(&self) -> BweR {
        BweR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 13 - SD_CLK_CTRL Write Enable Flag
    #[inline(always)]
    pub fn sd_clk_ctrlen(&self) -> SdClkCtrlenR {
        SdClkCtrlenR::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Command Sequence Status Flag
    #[inline(always)]
    pub fn cbsy(&self) -> CbsyR {
        CbsyR::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Illegal Access Error Detection Flag
    #[inline(always)]
    pub fn ila(&self) -> IlaR {
        IlaR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SD_INFO2")
            .field("cmde", &self.cmde())
            .field("crce", &self.crce())
            .field("ende", &self.ende())
            .field("dto", &self.dto())
            .field("ilw", &self.ilw())
            .field("ilr", &self.ilr())
            .field("rspto", &self.rspto())
            .field("sdd0mon", &self.sdd0mon())
            .field("bre", &self.bre())
            .field("bwe", &self.bwe())
            .field("sd_clk_ctrlen", &self.sd_clk_ctrlen())
            .field("cbsy", &self.cbsy())
            .field("ila", &self.ila())
            .finish()
    }
}
impl W {
    ///Bit 0 - Command Error Detection Flag
    #[inline(always)]
    pub fn cmde(&mut self) -> CmdeW<SdInfo2Spec> {
        CmdeW::new(self, 0)
    }
    ///Bit 1 - CRC Error Detection Flag
    #[inline(always)]
    pub fn crce(&mut self) -> CrceW<SdInfo2Spec> {
        CrceW::new(self, 1)
    }
    ///Bit 2 - End Bit Error Detection Flag
    #[inline(always)]
    pub fn ende(&mut self) -> EndeW<SdInfo2Spec> {
        EndeW::new(self, 2)
    }
    ///Bit 3 - Data Timeout Detection Flag
    #[inline(always)]
    pub fn dto(&mut self) -> DtoW<SdInfo2Spec> {
        DtoW::new(self, 3)
    }
    ///Bit 4 - SD_BUF0 Illegal Write Access Detection Flag
    #[inline(always)]
    pub fn ilw(&mut self) -> IlwW<SdInfo2Spec> {
        IlwW::new(self, 4)
    }
    ///Bit 5 - SD_BUF0 Illegal Read Access Detection Flag
    #[inline(always)]
    pub fn ilr(&mut self) -> IlrW<SdInfo2Spec> {
        IlrW::new(self, 5)
    }
    ///Bit 6 - Response Timeout Detection Flag
    #[inline(always)]
    pub fn rspto(&mut self) -> RsptoW<SdInfo2Spec> {
        RsptoW::new(self, 6)
    }
    ///Bit 8 - SD_BUF0 Read Enable Flag
    #[inline(always)]
    pub fn bre(&mut self) -> BreW<SdInfo2Spec> {
        BreW::new(self, 8)
    }
    ///Bit 9 - SD_BUF0 Write Enable Flag
    #[inline(always)]
    pub fn bwe(&mut self) -> BweW<SdInfo2Spec> {
        BweW::new(self, 9)
    }
    ///Bit 15 - Illegal Access Error Detection Flag
    #[inline(always)]
    pub fn ila(&mut self) -> IlaW<SdInfo2Spec> {
        IlaW::new(self, 15)
    }
}
/**SD Card Interrupt Flag Register 2

You can [`read`](crate::Reg::read) this register and get [`sd_info2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_info2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SdInfo2Spec;
impl crate::RegisterSpec for SdInfo2Spec {
    type Ux = u32;
}
///`read()` method returns [`sd_info2::R`](R) reader structure
impl crate::Readable for SdInfo2Spec {}
///`write(|w| ..)` method takes [`sd_info2::W`](W) writer structure
impl crate::Writable for SdInfo2Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SD_INFO2 to value 0x2000
impl crate::Resettable for SdInfo2Spec {
    const RESET_VALUE: u32 = 0x2000;
}
