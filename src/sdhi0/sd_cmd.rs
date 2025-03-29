///Register `SD_CMD` reader
pub type R = crate::R<SdCmdSpec>;
///Register `SD_CMD` writer
pub type W = crate::W<SdCmdSpec>;
/**Command Index Field Value Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmdidx {
    ///6: CMD6
    Cmd6 = 6,
    ///13: ACMD13
    Acmd13 = 13,
    ///18: CMD18
    Cmd18 = 18,
}
impl From<Cmdidx> for u8 {
    #[inline(always)]
    fn from(variant: Cmdidx) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmdidx {
    type Ux = u8;
}
impl crate::IsEnum for Cmdidx {}
///Field `CMDIDX` reader - Command Index Field Value Select
pub type CmdidxR = crate::FieldReader<Cmdidx>;
impl CmdidxR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cmdidx> {
        match self.bits {
            6 => Some(Cmdidx::Cmd6),
            13 => Some(Cmdidx::Acmd13),
            18 => Some(Cmdidx::Cmd18),
            _ => None,
        }
    }
    ///CMD6
    #[inline(always)]
    pub fn is_cmd6(&self) -> bool {
        *self == Cmdidx::Cmd6
    }
    ///ACMD13
    #[inline(always)]
    pub fn is_acmd13(&self) -> bool {
        *self == Cmdidx::Acmd13
    }
    ///CMD18
    #[inline(always)]
    pub fn is_cmd18(&self) -> bool {
        *self == Cmdidx::Cmd18
    }
}
///Field `CMDIDX` writer - Command Index Field Value Select
pub type CmdidxW<'a, REG> = crate::FieldWriter<'a, REG, 6, Cmdidx>;
impl<'a, REG> CmdidxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///CMD6
    #[inline(always)]
    pub fn cmd6(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdidx::Cmd6)
    }
    ///ACMD13
    #[inline(always)]
    pub fn acmd13(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdidx::Acmd13)
    }
    ///CMD18
    #[inline(always)]
    pub fn cmd18(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdidx::Cmd18)
    }
}
/**Command Type Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Acmd {
    ///0: CMD
    _00 = 0,
    ///1: ACMD
    _01 = 1,
    ///2: Setting prohibited
    Others = 2,
}
impl From<Acmd> for u8 {
    #[inline(always)]
    fn from(variant: Acmd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Acmd {
    type Ux = u8;
}
impl crate::IsEnum for Acmd {}
///Field `ACMD` reader - Command Type Select
pub type AcmdR = crate::FieldReader<Acmd>;
impl AcmdR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Acmd {
        match self.bits {
            0 => Acmd::_00,
            1 => Acmd::_01,
            _ => Acmd::Others,
        }
    }
    ///CMD
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Acmd::_00
    }
    ///ACMD
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Acmd::_01
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Acmd::Others)
    }
}
///Field `ACMD` writer - Command Type Select
pub type AcmdW<'a, REG> = crate::FieldWriter<'a, REG, 2, Acmd, crate::Safe>;
impl<'a, REG> AcmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///CMD
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Acmd::_00)
    }
    ///ACMD
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Acmd::_01)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Acmd::Others)
    }
}
/**Response Type Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rsptp {
    ///0: Normal mode Depending on the command, the response type and transfer method are selected in the ACMD\[1:0\] and CMDIDX\[5:0\] bits. At this time, the values for bits 15 to 11 in this register are invalid.
    _000 = 0,
    ///3: Extended mode and no response
    _011 = 3,
    ///4: Extended mode and R1, R5, R6, or R7 response
    _100 = 4,
    ///5: Extended mode and R1b response
    _101 = 5,
    ///6: Extended mode and R2 response
    _110 = 6,
    ///7: Extended mode and R3 or R4 response
    _111 = 7,
    ///1: Setting prohibited
    Others = 1,
}
impl From<Rsptp> for u8 {
    #[inline(always)]
    fn from(variant: Rsptp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rsptp {
    type Ux = u8;
}
impl crate::IsEnum for Rsptp {}
///Field `RSPTP` reader - Response Type Select
pub type RsptpR = crate::FieldReader<Rsptp>;
impl RsptpR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rsptp {
        match self.bits {
            0 => Rsptp::_000,
            3 => Rsptp::_011,
            4 => Rsptp::_100,
            5 => Rsptp::_101,
            6 => Rsptp::_110,
            7 => Rsptp::_111,
            _ => Rsptp::Others,
        }
    }
    ///Normal mode Depending on the command, the response type and transfer method are selected in the ACMD\[1:0\] and CMDIDX\[5:0\] bits. At this time, the values for bits 15 to 11 in this register are invalid.
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Rsptp::_000
    }
    ///Extended mode and no response
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Rsptp::_011
    }
    ///Extended mode and R1, R5, R6, or R7 response
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Rsptp::_100
    }
    ///Extended mode and R1b response
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Rsptp::_101
    }
    ///Extended mode and R2 response
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Rsptp::_110
    }
    ///Extended mode and R3 or R4 response
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == Rsptp::_111
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Rsptp::Others)
    }
}
///Field `RSPTP` writer - Response Type Select
pub type RsptpW<'a, REG> = crate::FieldWriter<'a, REG, 3, Rsptp, crate::Safe>;
impl<'a, REG> RsptpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Normal mode Depending on the command, the response type and transfer method are selected in the ACMD\[1:0\] and CMDIDX\[5:0\] bits. At this time, the values for bits 15 to 11 in this register are invalid.
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Rsptp::_000)
    }
    ///Extended mode and no response
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Rsptp::_011)
    }
    ///Extended mode and R1, R5, R6, or R7 response
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Rsptp::_100)
    }
    ///Extended mode and R1b response
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Rsptp::_101)
    }
    ///Extended mode and R2 response
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Rsptp::_110)
    }
    ///Extended mode and R3 or R4 response
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(Rsptp::_111)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Rsptp::Others)
    }
}
/**Data Transfer Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmdtp {
    ///0: Do not include data transfer (bc, bcr, or ac) in command
    _0 = 0,
    ///1: Include data transfer (adtc) in command
    _1 = 1,
}
impl From<Cmdtp> for bool {
    #[inline(always)]
    fn from(variant: Cmdtp) -> Self {
        variant as u8 != 0
    }
}
///Field `CMDTP` reader - Data Transfer Select
pub type CmdtpR = crate::BitReader<Cmdtp>;
impl CmdtpR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmdtp {
        match self.bits {
            false => Cmdtp::_0,
            true => Cmdtp::_1,
        }
    }
    ///Do not include data transfer (bc, bcr, or ac) in command
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmdtp::_0
    }
    ///Include data transfer (adtc) in command
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmdtp::_1
    }
}
///Field `CMDTP` writer - Data Transfer Select
pub type CmdtpW<'a, REG> = crate::BitWriter<'a, REG, Cmdtp>;
impl<'a, REG> CmdtpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not include data transfer (bc, bcr, or ac) in command
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdtp::_0)
    }
    ///Include data transfer (adtc) in command
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdtp::_1)
    }
}
/**Data Transfer Direction Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmdrw {
    ///0: Write (SD/MMC Host Interface → SD card/MMC)
    _0 = 0,
    ///1: Read (SD/MMC Host Interface ← SD card/MMC)
    _1 = 1,
}
impl From<Cmdrw> for bool {
    #[inline(always)]
    fn from(variant: Cmdrw) -> Self {
        variant as u8 != 0
    }
}
///Field `CMDRW` reader - Data Transfer Direction Select
pub type CmdrwR = crate::BitReader<Cmdrw>;
impl CmdrwR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmdrw {
        match self.bits {
            false => Cmdrw::_0,
            true => Cmdrw::_1,
        }
    }
    ///Write (SD/MMC Host Interface → SD card/MMC)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmdrw::_0
    }
    ///Read (SD/MMC Host Interface ← SD card/MMC)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmdrw::_1
    }
}
///Field `CMDRW` writer - Data Transfer Direction Select
pub type CmdrwW<'a, REG> = crate::BitWriter<'a, REG, Cmdrw>;
impl<'a, REG> CmdrwW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Write (SD/MMC Host Interface → SD card/MMC)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdrw::_0)
    }
    ///Read (SD/MMC Host Interface ← SD card/MMC)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdrw::_1)
    }
}
/**Block Transfer Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trstp {
    ///0: Single block transfer
    _0 = 0,
    ///1: Multiple blocks transfer
    _1 = 1,
}
impl From<Trstp> for bool {
    #[inline(always)]
    fn from(variant: Trstp) -> Self {
        variant as u8 != 0
    }
}
///Field `TRSTP` reader - Block Transfer Select
pub type TrstpR = crate::BitReader<Trstp>;
impl TrstpR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Trstp {
        match self.bits {
            false => Trstp::_0,
            true => Trstp::_1,
        }
    }
    ///Single block transfer
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Trstp::_0
    }
    ///Multiple blocks transfer
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Trstp::_1
    }
}
///Field `TRSTP` writer - Block Transfer Select
pub type TrstpW<'a, REG> = crate::BitWriter<'a, REG, Trstp>;
impl<'a, REG> TrstpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Single block transfer
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Trstp::_0)
    }
    ///Multiple blocks transfer
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Trstp::_1)
    }
}
/**CMD12 Automatic Issue Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmd12at {
    ///0: Automatically issue CMD12 during multiblock transfer
    _00 = 0,
    ///1: Do not automatically issue CMD12 during multiblock transfer
    _01 = 1,
    ///2: Setting prohibited
    Others = 2,
}
impl From<Cmd12at> for u8 {
    #[inline(always)]
    fn from(variant: Cmd12at) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmd12at {
    type Ux = u8;
}
impl crate::IsEnum for Cmd12at {}
///Field `CMD12AT` reader - CMD12 Automatic Issue Select
pub type Cmd12atR = crate::FieldReader<Cmd12at>;
impl Cmd12atR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmd12at {
        match self.bits {
            0 => Cmd12at::_00,
            1 => Cmd12at::_01,
            _ => Cmd12at::Others,
        }
    }
    ///Automatically issue CMD12 during multiblock transfer
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Cmd12at::_00
    }
    ///Do not automatically issue CMD12 during multiblock transfer
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Cmd12at::_01
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Cmd12at::Others)
    }
}
///Field `CMD12AT` writer - CMD12 Automatic Issue Select
pub type Cmd12atW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cmd12at, crate::Safe>;
impl<'a, REG> Cmd12atW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Automatically issue CMD12 during multiblock transfer
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd12at::_00)
    }
    ///Do not automatically issue CMD12 during multiblock transfer
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd12at::_01)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd12at::Others)
    }
}
impl R {
    ///Bits 0:5 - Command Index Field Value Select
    #[inline(always)]
    pub fn cmdidx(&self) -> CmdidxR {
        CmdidxR::new((self.bits & 0x3f) as u8)
    }
    ///Bits 6:7 - Command Type Select
    #[inline(always)]
    pub fn acmd(&self) -> AcmdR {
        AcmdR::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:10 - Response Type Select
    #[inline(always)]
    pub fn rsptp(&self) -> RsptpR {
        RsptpR::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bit 11 - Data Transfer Select
    #[inline(always)]
    pub fn cmdtp(&self) -> CmdtpR {
        CmdtpR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Data Transfer Direction Select
    #[inline(always)]
    pub fn cmdrw(&self) -> CmdrwR {
        CmdrwR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Block Transfer Select
    #[inline(always)]
    pub fn trstp(&self) -> TrstpR {
        TrstpR::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bits 14:15 - CMD12 Automatic Issue Select
    #[inline(always)]
    pub fn cmd12at(&self) -> Cmd12atR {
        Cmd12atR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SD_CMD")
            .field("cmdidx", &self.cmdidx())
            .field("acmd", &self.acmd())
            .field("rsptp", &self.rsptp())
            .field("cmdtp", &self.cmdtp())
            .field("cmdrw", &self.cmdrw())
            .field("trstp", &self.trstp())
            .field("cmd12at", &self.cmd12at())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - Command Index Field Value Select
    #[inline(always)]
    pub fn cmdidx(&mut self) -> CmdidxW<SdCmdSpec> {
        CmdidxW::new(self, 0)
    }
    ///Bits 6:7 - Command Type Select
    #[inline(always)]
    pub fn acmd(&mut self) -> AcmdW<SdCmdSpec> {
        AcmdW::new(self, 6)
    }
    ///Bits 8:10 - Response Type Select
    #[inline(always)]
    pub fn rsptp(&mut self) -> RsptpW<SdCmdSpec> {
        RsptpW::new(self, 8)
    }
    ///Bit 11 - Data Transfer Select
    #[inline(always)]
    pub fn cmdtp(&mut self) -> CmdtpW<SdCmdSpec> {
        CmdtpW::new(self, 11)
    }
    ///Bit 12 - Data Transfer Direction Select
    #[inline(always)]
    pub fn cmdrw(&mut self) -> CmdrwW<SdCmdSpec> {
        CmdrwW::new(self, 12)
    }
    ///Bit 13 - Block Transfer Select
    #[inline(always)]
    pub fn trstp(&mut self) -> TrstpW<SdCmdSpec> {
        TrstpW::new(self, 13)
    }
    ///Bits 14:15 - CMD12 Automatic Issue Select
    #[inline(always)]
    pub fn cmd12at(&mut self) -> Cmd12atW<SdCmdSpec> {
        Cmd12atW::new(self, 14)
    }
}
/**Command Type Register

You can [`read`](crate::Reg::read) this register and get [`sd_cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SdCmdSpec;
impl crate::RegisterSpec for SdCmdSpec {
    type Ux = u32;
}
///`read()` method returns [`sd_cmd::R`](R) reader structure
impl crate::Readable for SdCmdSpec {}
///`write(|w| ..)` method takes [`sd_cmd::W`](W) writer structure
impl crate::Writable for SdCmdSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SD_CMD to value 0
impl crate::Resettable for SdCmdSpec {}
