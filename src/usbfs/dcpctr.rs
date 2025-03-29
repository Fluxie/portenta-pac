///Register `DCPCTR` reader
pub type R = crate::R<DcpctrSpec>;
///Register `DCPCTR` writer
pub type W = crate::W<DcpctrSpec>;
/**Response PID

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pid {
    ///0: NAK response
    _00 = 0,
    ///1: BUF response (depends on the buffer state)
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
    ///BUF response (depends on the buffer state)
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
    ///BUF response (depends on the buffer state)
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
/**Control Transfer End Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccpl {
    ///0: Disable control transfer completion
    _0 = 0,
    ///1: Enable control transfer completion
    _1 = 1,
}
impl From<Ccpl> for bool {
    #[inline(always)]
    fn from(variant: Ccpl) -> Self {
        variant as u8 != 0
    }
}
///Field `CCPL` reader - Control Transfer End Enable
pub type CcplR = crate::BitReader<Ccpl>;
impl CcplR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ccpl {
        match self.bits {
            false => Ccpl::_0,
            true => Ccpl::_1,
        }
    }
    ///Disable control transfer completion
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ccpl::_0
    }
    ///Enable control transfer completion
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ccpl::_1
    }
}
///Field `CCPL` writer - Control Transfer End Enable
pub type CcplW<'a, REG> = crate::BitWriter<'a, REG, Ccpl>;
impl<'a, REG> CcplW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable control transfer completion
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ccpl::_0)
    }
    ///Enable control transfer completion
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ccpl::_1)
    }
}
/**Pipe Busy

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pbusy {
    ///0: DCP not used for the USB bus
    _0 = 0,
    ///1: DCP in use for the USB bus
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
    ///DCP not used for the USB bus
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pbusy::_0
    }
    ///DCP in use for the USB bus
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pbusy::_1
    }
}
/**Sequence Toggle Bit Monitor

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sqmon {
    ///0: DATA0
    _0 = 0,
    ///1: ATA1
    _1 = 1,
}
impl From<Sqmon> for bool {
    #[inline(always)]
    fn from(variant: Sqmon) -> Self {
        variant as u8 != 0
    }
}
///Field `SQMON` reader - Sequence Toggle Bit Monitor
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
    ///ATA1
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
/**SUREQ Bit Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sureqclr {
    ///0: Invalid (writing 0 has no effect)
    _0 = 0,
    ///1: Clear SUREQ to 0
    _1 = 1,
}
impl From<Sureqclr> for bool {
    #[inline(always)]
    fn from(variant: Sureqclr) -> Self {
        variant as u8 != 0
    }
}
///Field `SUREQCLR` reader - SUREQ Bit Clear
pub type SureqclrR = crate::BitReader<Sureqclr>;
impl SureqclrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sureqclr {
        match self.bits {
            false => Sureqclr::_0,
            true => Sureqclr::_1,
        }
    }
    ///Invalid (writing 0 has no effect)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sureqclr::_0
    }
    ///Clear SUREQ to 0
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sureqclr::_1
    }
}
///Field `SUREQCLR` writer - SUREQ Bit Clear
pub type SureqclrW<'a, REG> = crate::BitWriter<'a, REG, Sureqclr>;
impl<'a, REG> SureqclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Invalid (writing 0 has no effect)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sureqclr::_0)
    }
    ///Clear SUREQ to 0
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sureqclr::_1)
    }
}
/**Setup Token Transmission

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sureq {
    ///0: Invalid (writing 0 has no effect)
    _0 = 0,
    ///1: Transmit setup packet
    _1 = 1,
}
impl From<Sureq> for bool {
    #[inline(always)]
    fn from(variant: Sureq) -> Self {
        variant as u8 != 0
    }
}
///Field `SUREQ` reader - Setup Token Transmission
pub type SureqR = crate::BitReader<Sureq>;
impl SureqR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sureq {
        match self.bits {
            false => Sureq::_0,
            true => Sureq::_1,
        }
    }
    ///Invalid (writing 0 has no effect)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sureq::_0
    }
    ///Transmit setup packet
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sureq::_1
    }
}
///Field `SUREQ` writer - Setup Token Transmission
pub type SureqW<'a, REG> = crate::BitWriter<'a, REG, Sureq>;
impl<'a, REG> SureqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Invalid (writing 0 has no effect)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sureq::_0)
    }
    ///Transmit setup packet
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sureq::_1)
    }
}
/**Buffer Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bsts {
    ///0: Buffer access disabled
    _0 = 0,
    ///1: Buffer access enabled
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
    ///Buffer access disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bsts::_0
    }
    ///Buffer access enabled
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
    ///Bit 2 - Control Transfer End Enable
    #[inline(always)]
    pub fn ccpl(&self) -> CcplR {
        CcplR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 5 - Pipe Busy
    #[inline(always)]
    pub fn pbusy(&self) -> PbusyR {
        PbusyR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Sequence Toggle Bit Monitor
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
    ///Bit 11 - SUREQ Bit Clear
    #[inline(always)]
    pub fn sureqclr(&self) -> SureqclrR {
        SureqclrR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 14 - Setup Token Transmission
    #[inline(always)]
    pub fn sureq(&self) -> SureqR {
        SureqR::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Buffer Status
    #[inline(always)]
    pub fn bsts(&self) -> BstsR {
        BstsR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCPCTR")
            .field("pid", &self.pid())
            .field("ccpl", &self.ccpl())
            .field("pbusy", &self.pbusy())
            .field("sqmon", &self.sqmon())
            .field("sqset", &self.sqset())
            .field("sqclr", &self.sqclr())
            .field("sureqclr", &self.sureqclr())
            .field("sureq", &self.sureq())
            .field("bsts", &self.bsts())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Response PID
    #[inline(always)]
    pub fn pid(&mut self) -> PidW<DcpctrSpec> {
        PidW::new(self, 0)
    }
    ///Bit 2 - Control Transfer End Enable
    #[inline(always)]
    pub fn ccpl(&mut self) -> CcplW<DcpctrSpec> {
        CcplW::new(self, 2)
    }
    ///Bit 7 - Sequence Toggle Bit Set
    #[inline(always)]
    pub fn sqset(&mut self) -> SqsetW<DcpctrSpec> {
        SqsetW::new(self, 7)
    }
    ///Bit 8 - Sequence Toggle Bit Clear
    #[inline(always)]
    pub fn sqclr(&mut self) -> SqclrW<DcpctrSpec> {
        SqclrW::new(self, 8)
    }
    ///Bit 11 - SUREQ Bit Clear
    #[inline(always)]
    pub fn sureqclr(&mut self) -> SureqclrW<DcpctrSpec> {
        SureqclrW::new(self, 11)
    }
    ///Bit 14 - Setup Token Transmission
    #[inline(always)]
    pub fn sureq(&mut self) -> SureqW<DcpctrSpec> {
        SureqW::new(self, 14)
    }
}
/**DCP Control Register

You can [`read`](crate::Reg::read) this register and get [`dcpctr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcpctr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DcpctrSpec;
impl crate::RegisterSpec for DcpctrSpec {
    type Ux = u16;
}
///`read()` method returns [`dcpctr::R`](R) reader structure
impl crate::Readable for DcpctrSpec {}
///`write(|w| ..)` method takes [`dcpctr::W`](W) writer structure
impl crate::Writable for DcpctrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DCPCTR to value 0x40
impl crate::Resettable for DcpctrSpec {
    const RESET_VALUE: u16 = 0x40;
}
