///Register `PIPE%sCTR` reader
pub type R = crate::R<PipectrSpec>;
///Register `PIPE%sCTR` writer
pub type W = crate::W<PipectrSpec>;
/**Response PID

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pid {
    ///0: NAK response
    _00 = 0,
    ///1: BUF response (depends buffer state)
    _01 = 1,
    ///2: STALL response
    _10 = 2,
    ///3: STALL response
    _11 = 3,
}
impl From<Pid> for u8 {
    #[inline(always)]
    fn from(variant: Pid) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pid {
    type Ux = u8;
}
impl crate::IsEnum for Pid {}
///Field `PID` reader - Response PID
pub type PidR = crate::FieldReader<Pid>;
impl PidR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pid {
        match self.bits {
            0 => Pid::_00,
            1 => Pid::_01,
            2 => Pid::_10,
            3 => Pid::_11,
            _ => unreachable!(),
        }
    }
    ///NAK response
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Pid::_00
    }
    ///BUF response (depends buffer state)
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Pid::_01
    }
    ///STALL response
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Pid::_10
    }
    ///STALL response
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Pid::_11
    }
}
///Field `PID` writer - Response PID
pub type PidW<'a, REG> = crate::FieldWriter<'a, REG, 2, Pid, crate::Safe>;
impl<'a, REG> PidW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///NAK response
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Pid::_00)
    }
    ///BUF response (depends buffer state)
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Pid::_01)
    }
    ///STALL response
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Pid::_10)
    }
    ///STALL response
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Pid::_11)
    }
}
/**Pipe Busy

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pbusy {
    ///0: Pipe n not in use for the transaction
    _0 = 0,
    ///1: Pipe n in use for the transaction
    _1 = 1,
}
impl From<Pbusy> for bool {
    #[inline(always)]
    fn from(variant: Pbusy) -> Self {
        variant as u8 != 0
    }
}
///Field `PBUSY` reader - Pipe Busy
pub type PbusyR = crate::BitReader<Pbusy>;
impl PbusyR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pbusy {
        match self.bits {
            false => Pbusy::_0,
            true => Pbusy::_1,
        }
    }
    ///Pipe n not in use for the transaction
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pbusy::_0
    }
    ///Pipe n in use for the transaction
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pbusy::_1
    }
}
/**Sequence Toggle Bit Confirmation

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sqmon {
    ///0: DATA0
    _0 = 0,
    ///1: DATA1
    _1 = 1,
}
impl From<Sqmon> for bool {
    #[inline(always)]
    fn from(variant: Sqmon) -> Self {
        variant as u8 != 0
    }
}
///Field `SQMON` reader - Sequence Toggle Bit Confirmation
pub type SqmonR = crate::BitReader<Sqmon>;
impl SqmonR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sqmon {
        match self.bits {
            false => Sqmon::_0,
            true => Sqmon::_1,
        }
    }
    ///DATA0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sqmon::_0
    }
    ///DATA1
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sqmon::_1
    }
}
/**Sequence Toggle Bit Set

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sqset {
    ///0: Invalid (writing 0 has no effect)
    _0 = 0,
    ///1: Set the expected value for the next transaction to DATA1
    _1 = 1,
}
impl From<Sqset> for bool {
    #[inline(always)]
    fn from(variant: Sqset) -> Self {
        variant as u8 != 0
    }
}
///Field `SQSET` reader - Sequence Toggle Bit Set
pub type SqsetR = crate::BitReader<Sqset>;
impl SqsetR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sqset {
        match self.bits {
            false => Sqset::_0,
            true => Sqset::_1,
        }
    }
    ///Invalid (writing 0 has no effect)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sqset::_0
    }
    ///Set the expected value for the next transaction to DATA1
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sqset::_1
    }
}
///Field `SQSET` writer - Sequence Toggle Bit Set
pub type SqsetW<'a, REG> = crate::BitWriter<'a, REG, Sqset>;
impl<'a, REG> SqsetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Invalid (writing 0 has no effect)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sqset::_0)
    }
    ///Set the expected value for the next transaction to DATA1
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sqset::_1)
    }
}
/**Sequence Toggle Bit Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sqclr {
    ///0: Invalid (writing 0 has no effect)
    _0 = 0,
    ///1: Clear the expected value for the next transaction to DATA0
    _1 = 1,
}
impl From<Sqclr> for bool {
    #[inline(always)]
    fn from(variant: Sqclr) -> Self {
        variant as u8 != 0
    }
}
///Field `SQCLR` reader - Sequence Toggle Bit Clear
pub type SqclrR = crate::BitReader<Sqclr>;
impl SqclrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sqclr {
        match self.bits {
            false => Sqclr::_0,
            true => Sqclr::_1,
        }
    }
    ///Invalid (writing 0 has no effect)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sqclr::_0
    }
    ///Clear the expected value for the next transaction to DATA0
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sqclr::_1
    }
}
///Field `SQCLR` writer - Sequence Toggle Bit Clear
pub type SqclrW<'a, REG> = crate::BitWriter<'a, REG, Sqclr>;
impl<'a, REG> SqclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Invalid (writing 0 has no effect)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sqclr::_0)
    }
    ///Clear the expected value for the next transaction to DATA0
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sqclr::_1)
    }
}
/**Auto Buffer Clear Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aclrm {
    ///0: Disable
    _0 = 0,
    ///1: Enable (initialize all buffers)
    _1 = 1,
}
impl From<Aclrm> for bool {
    #[inline(always)]
    fn from(variant: Aclrm) -> Self {
        variant as u8 != 0
    }
}
///Field `ACLRM` reader - Auto Buffer Clear Mode
pub type AclrmR = crate::BitReader<Aclrm>;
impl AclrmR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Aclrm {
        match self.bits {
            false => Aclrm::_0,
            true => Aclrm::_1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Aclrm::_0
    }
    ///Enable (initialize all buffers)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Aclrm::_1
    }
}
///Field `ACLRM` writer - Auto Buffer Clear Mode
pub type AclrmW<'a, REG> = crate::BitWriter<'a, REG, Aclrm>;
impl<'a, REG> AclrmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Aclrm::_0)
    }
    ///Enable (initialize all buffers)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Aclrm::_1)
    }
}
/**Auto Response Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Atrepm {
    ///0: Disable auto response mode
    _0 = 0,
    ///1: Enable auto response mode
    _1 = 1,
}
impl From<Atrepm> for bool {
    #[inline(always)]
    fn from(variant: Atrepm) -> Self {
        variant as u8 != 0
    }
}
///Field `ATREPM` reader - Auto Response Mode
pub type AtrepmR = crate::BitReader<Atrepm>;
impl AtrepmR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Atrepm {
        match self.bits {
            false => Atrepm::_0,
            true => Atrepm::_1,
        }
    }
    ///Disable auto response mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Atrepm::_0
    }
    ///Enable auto response mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Atrepm::_1
    }
}
///Field `ATREPM` writer - Auto Response Mode
pub type AtrepmW<'a, REG> = crate::BitWriter<'a, REG, Atrepm>;
impl<'a, REG> AtrepmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable auto response mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Atrepm::_0)
    }
    ///Enable auto response mode
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Atrepm::_1)
    }
}
/**Transmit Buffer Monitor

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inbufm {
    ///0: No data to be transmitted is in the FIFO buffer
    _0 = 0,
    ///1: Data to be transmitted is in the FIFO buffer
    _1 = 1,
}
impl From<Inbufm> for bool {
    #[inline(always)]
    fn from(variant: Inbufm) -> Self {
        variant as u8 != 0
    }
}
///Field `INBUFM` reader - Transmit Buffer Monitor
pub type InbufmR = crate::BitReader<Inbufm>;
impl InbufmR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Inbufm {
        match self.bits {
            false => Inbufm::_0,
            true => Inbufm::_1,
        }
    }
    ///No data to be transmitted is in the FIFO buffer
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Inbufm::_0
    }
    ///Data to be transmitted is in the FIFO buffer
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Inbufm::_1
    }
}
/**Buffer Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bsts {
    ///0: Buffer access by the CPU disabled
    _0 = 0,
    ///1: Buffer access by the CPU enabled
    _1 = 1,
}
impl From<Bsts> for bool {
    #[inline(always)]
    fn from(variant: Bsts) -> Self {
        variant as u8 != 0
    }
}
///Field `BSTS` reader - Buffer Status
pub type BstsR = crate::BitReader<Bsts>;
impl BstsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Bsts {
        match self.bits {
            false => Bsts::_0,
            true => Bsts::_1,
        }
    }
    ///Buffer access by the CPU disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bsts::_0
    }
    ///Buffer access by the CPU enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bsts::_1
    }
}
impl R {
    ///Bits 0:1 - Response PID
    #[inline(always)]
    pub fn pid(&self) -> PidR {
        PidR::new((self.bits & 3) as u8)
    }
    ///Bit 5 - Pipe Busy
    #[inline(always)]
    pub fn pbusy(&self) -> PbusyR {
        PbusyR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Sequence Toggle Bit Confirmation
    #[inline(always)]
    pub fn sqmon(&self) -> SqmonR {
        SqmonR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Sequence Toggle Bit Set
    #[inline(always)]
    pub fn sqset(&self) -> SqsetR {
        SqsetR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Sequence Toggle Bit Clear
    #[inline(always)]
    pub fn sqclr(&self) -> SqclrR {
        SqclrR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Auto Buffer Clear Mode
    #[inline(always)]
    pub fn aclrm(&self) -> AclrmR {
        AclrmR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Auto Response Mode
    #[inline(always)]
    pub fn atrepm(&self) -> AtrepmR {
        AtrepmR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 14 - Transmit Buffer Monitor
    #[inline(always)]
    pub fn inbufm(&self) -> InbufmR {
        InbufmR::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Buffer Status
    #[inline(always)]
    pub fn bsts(&self) -> BstsR {
        BstsR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIPECTR")
            .field("pid", &self.pid())
            .field("pbusy", &self.pbusy())
            .field("sqmon", &self.sqmon())
            .field("sqset", &self.sqset())
            .field("sqclr", &self.sqclr())
            .field("aclrm", &self.aclrm())
            .field("atrepm", &self.atrepm())
            .field("inbufm", &self.inbufm())
            .field("bsts", &self.bsts())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Response PID
    #[inline(always)]
    pub fn pid(&mut self) -> PidW<PipectrSpec> {
        PidW::new(self, 0)
    }
    ///Bit 7 - Sequence Toggle Bit Set
    #[inline(always)]
    pub fn sqset(&mut self) -> SqsetW<PipectrSpec> {
        SqsetW::new(self, 7)
    }
    ///Bit 8 - Sequence Toggle Bit Clear
    #[inline(always)]
    pub fn sqclr(&mut self) -> SqclrW<PipectrSpec> {
        SqclrW::new(self, 8)
    }
    ///Bit 9 - Auto Buffer Clear Mode
    #[inline(always)]
    pub fn aclrm(&mut self) -> AclrmW<PipectrSpec> {
        AclrmW::new(self, 9)
    }
    ///Bit 10 - Auto Response Mode
    #[inline(always)]
    pub fn atrepm(&mut self) -> AtrepmW<PipectrSpec> {
        AtrepmW::new(self, 10)
    }
}
/**PIPE%s Control Registers

You can [`read`](crate::Reg::read) this register and get [`pipectr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pipectr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PipectrSpec;
impl crate::RegisterSpec for PipectrSpec {
    type Ux = u16;
}
///`read()` method returns [`pipectr::R`](R) reader structure
impl crate::Readable for PipectrSpec {}
///`write(|w| ..)` method takes [`pipectr::W`](W) writer structure
impl crate::Writable for PipectrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PIPE%sCTR to value 0
impl crate::Resettable for PipectrSpec {}
