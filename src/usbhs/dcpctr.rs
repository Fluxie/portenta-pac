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
    Nak = 0,
    ///1: BUF response
    Buf = 1,
    ///2: STALL response
    Stall2 = 2,
    ///3: STALL response
    Stall3 = 3,
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
            0 => Pid::Nak,
            1 => Pid::Buf,
            2 => Pid::Stall2,
            3 => Pid::Stall3,
            _ => unreachable!(),
        }
    }
    ///NAK response
    #[inline(always)]
    pub fn is_nak(&self) -> bool {
        *self == Pid::Nak
    }
    ///BUF response
    #[inline(always)]
    pub fn is_buf(&self) -> bool {
        *self == Pid::Buf
    }
    ///STALL response
    #[inline(always)]
    pub fn is_stall2(&self) -> bool {
        *self == Pid::Stall2
    }
    ///STALL response
    #[inline(always)]
    pub fn is_stall3(&self) -> bool {
        *self == Pid::Stall3
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
    pub fn nak(self) -> &'a mut crate::W<REG> {
        self.variant(Pid::Nak)
    }
    ///BUF response
    #[inline(always)]
    pub fn buf(self) -> &'a mut crate::W<REG> {
        self.variant(Pid::Buf)
    }
    ///STALL response
    #[inline(always)]
    pub fn stall2(self) -> &'a mut crate::W<REG> {
        self.variant(Pid::Stall2)
    }
    ///STALL response
    #[inline(always)]
    pub fn stall3(self) -> &'a mut crate::W<REG> {
        self.variant(Pid::Stall3)
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
/**PING Token Issue Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pinge {
    ///0: Disable PING token
    _0 = 0,
    ///1: Enable normal PING operation
    _1 = 1,
}
impl From<Pinge> for bool {
    #[inline(always)]
    fn from(variant: Pinge) -> Self {
        variant as u8 != 0
    }
}
///Field `PINGE` reader - PING Token Issue Enable
pub type PingeR = crate::BitReader<Pinge>;
impl PingeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pinge {
        match self.bits {
            false => Pinge::_0,
            true => Pinge::_1,
        }
    }
    ///Disable PING token
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pinge::_0
    }
    ///Enable normal PING operation
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pinge::_1
    }
}
///Field `PINGE` writer - PING Token Issue Enable
pub type PingeW<'a, REG> = crate::BitWriter<'a, REG, Pinge>;
impl<'a, REG> PingeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable PING token
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pinge::_0)
    }
    ///Enable normal PING operation
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pinge::_1)
    }
}
/**Pipe Busy Flag

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
///Field `PBUSY` reader - Pipe Busy Flag
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
/**Sequence Toggle Bit Monitor Flag

Value on reset: 1*/
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
///Field `SQMON` reader - Sequence Toggle Bit Monitor Flag
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
/**CSSTS Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cssts {
    ///0: Start-split (SSPLIT) transaction, or processing for devices that are not using split transactions, in progress
    _0 = 0,
    ///1: Complete-split (CSPLIT) transaction in progress
    _1 = 1,
}
impl From<Cssts> for bool {
    #[inline(always)]
    fn from(variant: Cssts) -> Self {
        variant as u8 != 0
    }
}
///Field `CSSTS` reader - CSSTS Status Flag
pub type CsstsR = crate::BitReader<Cssts>;
impl CsstsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cssts {
        match self.bits {
            false => Cssts::_0,
            true => Cssts::_1,
        }
    }
    ///Start-split (SSPLIT) transaction, or processing for devices that are not using split transactions, in progress
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cssts::_0
    }
    ///Complete-split (CSPLIT) transaction in progress
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cssts::_1
    }
}
/**CSSTS Status Flag Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Csclr {
    ///0: (writing 0 has no effect)
    _0 = 0,
    ///1: Clear CSSTS to 0
    _1 = 1,
}
impl From<Csclr> for bool {
    #[inline(always)]
    fn from(variant: Csclr) -> Self {
        variant as u8 != 0
    }
}
///Field `CSCLR` writer - CSSTS Status Flag Clear
pub type CsclrW<'a, REG> = crate::BitWriter<'a, REG, Csclr>;
impl<'a, REG> CsclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///(writing 0 has no effect)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Csclr::_0)
    }
    ///Clear CSSTS to 0
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Csclr::_1)
    }
}
/**SETUP Token Transmission

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
///Field `SUREQ` reader - SETUP Token Transmission
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
///Field `SUREQ` writer - SETUP Token Transmission
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
/**Buffer Status Flag

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
///Field `BSTS` reader - Buffer Status Flag
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
    ///Bit 4 - PING Token Issue Enable
    #[inline(always)]
    pub fn pinge(&self) -> PingeR {
        PingeR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Pipe Busy Flag
    #[inline(always)]
    pub fn pbusy(&self) -> PbusyR {
        PbusyR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Sequence Toggle Bit Monitor Flag
    #[inline(always)]
    pub fn sqmon(&self) -> SqmonR {
        SqmonR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 12 - CSSTS Status Flag
    #[inline(always)]
    pub fn cssts(&self) -> CsstsR {
        CsstsR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - SETUP Token Transmission
    #[inline(always)]
    pub fn sureq(&self) -> SureqR {
        SureqR::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Buffer Status Flag
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
            .field("pinge", &self.pinge())
            .field("pbusy", &self.pbusy())
            .field("sqmon", &self.sqmon())
            .field("cssts", &self.cssts())
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
    ///Bit 4 - PING Token Issue Enable
    #[inline(always)]
    pub fn pinge(&mut self) -> PingeW<DcpctrSpec> {
        PingeW::new(self, 4)
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
    ///Bit 13 - CSSTS Status Flag Clear
    #[inline(always)]
    pub fn csclr(&mut self) -> CsclrW<DcpctrSpec> {
        CsclrW::new(self, 13)
    }
    ///Bit 14 - SETUP Token Transmission
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
