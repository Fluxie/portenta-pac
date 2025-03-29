///Register `RCR2` reader
pub type R = crate::R<Rcr2Spec>;
///Register `RCR2` writer
pub type W = crate::W<Rcr2Spec>;
/**Start

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Start {
    ///0: Stop prescaler and time counter
    _0 = 0,
    ///1: Operate prescaler and time counter normally
    _1 = 1,
}
impl From<Start> for bool {
    #[inline(always)]
    fn from(variant: Start) -> Self {
        variant as u8 != 0
    }
}
///Field `START` reader - Start
pub type StartR = crate::BitReader<Start>;
impl StartR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Start {
        match self.bits {
            false => Start::_0,
            true => Start::_1,
        }
    }
    ///Stop prescaler and time counter
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Start::_0
    }
    ///Operate prescaler and time counter normally
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Start::_1
    }
}
///Field `START` writer - Start
pub type StartW<'a, REG> = crate::BitWriter<'a, REG, Start>;
impl<'a, REG> StartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Stop prescaler and time counter
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Start::_0)
    }
    ///Operate prescaler and time counter normally
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Start::_1)
    }
}
/**RTC Software Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reset {
    ///0: In writing: Invalid (writing 0 has no effect). In reading: Normal time operation in progress, or an RTC software reset has completed.
    _0 = 0,
    ///1: In writing: Initialize the prescaler and target registers for RTC software reset. In reading: RTC software reset in progress.
    _1 = 1,
}
impl From<Reset> for bool {
    #[inline(always)]
    fn from(variant: Reset) -> Self {
        variant as u8 != 0
    }
}
///Field `RESET` reader - RTC Software Reset
pub type ResetR = crate::BitReader<Reset>;
impl ResetR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Reset {
        match self.bits {
            false => Reset::_0,
            true => Reset::_1,
        }
    }
    ///In writing: Invalid (writing 0 has no effect). In reading: Normal time operation in progress, or an RTC software reset has completed.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Reset::_0
    }
    ///In writing: Initialize the prescaler and target registers for RTC software reset. In reading: RTC software reset in progress.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Reset::_1
    }
}
///Field `RESET` writer - RTC Software Reset
pub type ResetW<'a, REG> = crate::BitWriter<'a, REG, Reset>;
impl<'a, REG> ResetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///In writing: Invalid (writing 0 has no effect). In reading: Normal time operation in progress, or an RTC software reset has completed.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Reset::_0)
    }
    ///In writing: Initialize the prescaler and target registers for RTC software reset. In reading: RTC software reset in progress.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Reset::_1)
    }
}
/**30-Second Adjustment

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adj30 {
    ///0: In writing: Invalid (writing 0 has no effect). In reading: Normal time operation in progress, or 30-second adjustment has completed.
    _0 = 0,
    ///1: In writing: Execute 30-second adjustment. In reading: 30-second adjustment in progress.
    _1 = 1,
}
impl From<Adj30> for bool {
    #[inline(always)]
    fn from(variant: Adj30) -> Self {
        variant as u8 != 0
    }
}
///Field `ADJ30` reader - 30-Second Adjustment
pub type Adj30R = crate::BitReader<Adj30>;
impl Adj30R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Adj30 {
        match self.bits {
            false => Adj30::_0,
            true => Adj30::_1,
        }
    }
    ///In writing: Invalid (writing 0 has no effect). In reading: Normal time operation in progress, or 30-second adjustment has completed.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Adj30::_0
    }
    ///In writing: Execute 30-second adjustment. In reading: 30-second adjustment in progress.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Adj30::_1
    }
}
///Field `ADJ30` writer - 30-Second Adjustment
pub type Adj30W<'a, REG> = crate::BitWriter<'a, REG, Adj30>;
impl<'a, REG> Adj30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///In writing: Invalid (writing 0 has no effect). In reading: Normal time operation in progress, or 30-second adjustment has completed.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Adj30::_0)
    }
    ///In writing: Execute 30-second adjustment. In reading: 30-second adjustment in progress.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Adj30::_1)
    }
}
/**RTCOUT Output Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtcoe {
    ///0: Disable RTCOUT output
    _0 = 0,
    ///1: Enable RTCOUT output
    _1 = 1,
}
impl From<Rtcoe> for bool {
    #[inline(always)]
    fn from(variant: Rtcoe) -> Self {
        variant as u8 != 0
    }
}
///Field `RTCOE` reader - RTCOUT Output Enable
pub type RtcoeR = crate::BitReader<Rtcoe>;
impl RtcoeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rtcoe {
        match self.bits {
            false => Rtcoe::_0,
            true => Rtcoe::_1,
        }
    }
    ///Disable RTCOUT output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rtcoe::_0
    }
    ///Enable RTCOUT output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rtcoe::_1
    }
}
///Field `RTCOE` writer - RTCOUT Output Enable
pub type RtcoeW<'a, REG> = crate::BitWriter<'a, REG, Rtcoe>;
impl<'a, REG> RtcoeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable RTCOUT output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcoe::_0)
    }
    ///Enable RTCOUT output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcoe::_1)
    }
}
/**Automatic Adjustment Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aadje {
    ///0: Disable automatic adjustment
    _0 = 0,
    ///1: Enable automatic adjustment
    _1 = 1,
}
impl From<Aadje> for bool {
    #[inline(always)]
    fn from(variant: Aadje) -> Self {
        variant as u8 != 0
    }
}
///Field `AADJE` reader - Automatic Adjustment Enable
pub type AadjeR = crate::BitReader<Aadje>;
impl AadjeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Aadje {
        match self.bits {
            false => Aadje::_0,
            true => Aadje::_1,
        }
    }
    ///Disable automatic adjustment
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Aadje::_0
    }
    ///Enable automatic adjustment
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Aadje::_1
    }
}
///Field `AADJE` writer - Automatic Adjustment Enable
pub type AadjeW<'a, REG> = crate::BitWriter<'a, REG, Aadje>;
impl<'a, REG> AadjeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable automatic adjustment
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Aadje::_0)
    }
    ///Enable automatic adjustment
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Aadje::_1)
    }
}
/**Automatic Adjustment Period Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aadjp {
    ///0: The RADJ.ADJ\[5:0\] setting from the count value of the prescaler every minute.
    _0 = 0,
    ///1: The RADJ.ADJ\[5:0\] setting value is adjusted from the coun tvalue of the prescaler every 10 seconds.
    _1 = 1,
}
impl From<Aadjp> for bool {
    #[inline(always)]
    fn from(variant: Aadjp) -> Self {
        variant as u8 != 0
    }
}
///Field `AADJP` reader - Automatic Adjustment Period Select
pub type AadjpR = crate::BitReader<Aadjp>;
impl AadjpR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Aadjp {
        match self.bits {
            false => Aadjp::_0,
            true => Aadjp::_1,
        }
    }
    ///The RADJ.ADJ\[5:0\] setting from the count value of the prescaler every minute.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Aadjp::_0
    }
    ///The RADJ.ADJ\[5:0\] setting value is adjusted from the coun tvalue of the prescaler every 10 seconds.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Aadjp::_1
    }
}
///Field `AADJP` writer - Automatic Adjustment Period Select
pub type AadjpW<'a, REG> = crate::BitWriter<'a, REG, Aadjp>;
impl<'a, REG> AadjpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The RADJ.ADJ\[5:0\] setting from the count value of the prescaler every minute.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Aadjp::_0)
    }
    ///The RADJ.ADJ\[5:0\] setting value is adjusted from the coun tvalue of the prescaler every 10 seconds.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Aadjp::_1)
    }
}
/**Hours Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hr24 {
    ///0: Operate RTC in 12-hour mode
    _0 = 0,
    ///1: Operate RTC in 24-hour mode
    _1 = 1,
}
impl From<Hr24> for bool {
    #[inline(always)]
    fn from(variant: Hr24) -> Self {
        variant as u8 != 0
    }
}
///Field `HR24` reader - Hours Mode
pub type Hr24R = crate::BitReader<Hr24>;
impl Hr24R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Hr24 {
        match self.bits {
            false => Hr24::_0,
            true => Hr24::_1,
        }
    }
    ///Operate RTC in 12-hour mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Hr24::_0
    }
    ///Operate RTC in 24-hour mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Hr24::_1
    }
}
///Field `HR24` writer - Hours Mode
pub type Hr24W<'a, REG> = crate::BitWriter<'a, REG, Hr24>;
impl<'a, REG> Hr24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Operate RTC in 12-hour mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Hr24::_0)
    }
    ///Operate RTC in 24-hour mode
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Hr24::_1)
    }
}
/**Count Mode Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cntmd {
    ///0: Calendar count mode
    _0 = 0,
    ///1: Binary count mode
    _1 = 1,
}
impl From<Cntmd> for bool {
    #[inline(always)]
    fn from(variant: Cntmd) -> Self {
        variant as u8 != 0
    }
}
///Field `CNTMD` reader - Count Mode Select
pub type CntmdR = crate::BitReader<Cntmd>;
impl CntmdR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cntmd {
        match self.bits {
            false => Cntmd::_0,
            true => Cntmd::_1,
        }
    }
    ///Calendar count mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cntmd::_0
    }
    ///Binary count mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cntmd::_1
    }
}
///Field `CNTMD` writer - Count Mode Select
pub type CntmdW<'a, REG> = crate::BitWriter<'a, REG, Cntmd>;
impl<'a, REG> CntmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Calendar count mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cntmd::_0)
    }
    ///Binary count mode
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cntmd::_1)
    }
}
impl R {
    ///Bit 0 - Start
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - RTC Software Reset
    #[inline(always)]
    pub fn reset(&self) -> ResetR {
        ResetR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - 30-Second Adjustment
    #[inline(always)]
    pub fn adj30(&self) -> Adj30R {
        Adj30R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - RTCOUT Output Enable
    #[inline(always)]
    pub fn rtcoe(&self) -> RtcoeR {
        RtcoeR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Automatic Adjustment Enable
    #[inline(always)]
    pub fn aadje(&self) -> AadjeR {
        AadjeR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Automatic Adjustment Period Select
    #[inline(always)]
    pub fn aadjp(&self) -> AadjpR {
        AadjpR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Hours Mode
    #[inline(always)]
    pub fn hr24(&self) -> Hr24R {
        Hr24R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Count Mode Select
    #[inline(always)]
    pub fn cntmd(&self) -> CntmdR {
        CntmdR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCR2")
            .field("start", &self.start())
            .field("reset", &self.reset())
            .field("adj30", &self.adj30())
            .field("rtcoe", &self.rtcoe())
            .field("aadje", &self.aadje())
            .field("aadjp", &self.aadjp())
            .field("hr24", &self.hr24())
            .field("cntmd", &self.cntmd())
            .finish()
    }
}
impl W {
    ///Bit 0 - Start
    #[inline(always)]
    pub fn start(&mut self) -> StartW<Rcr2Spec> {
        StartW::new(self, 0)
    }
    ///Bit 1 - RTC Software Reset
    #[inline(always)]
    pub fn reset(&mut self) -> ResetW<Rcr2Spec> {
        ResetW::new(self, 1)
    }
    ///Bit 2 - 30-Second Adjustment
    #[inline(always)]
    pub fn adj30(&mut self) -> Adj30W<Rcr2Spec> {
        Adj30W::new(self, 2)
    }
    ///Bit 3 - RTCOUT Output Enable
    #[inline(always)]
    pub fn rtcoe(&mut self) -> RtcoeW<Rcr2Spec> {
        RtcoeW::new(self, 3)
    }
    ///Bit 4 - Automatic Adjustment Enable
    #[inline(always)]
    pub fn aadje(&mut self) -> AadjeW<Rcr2Spec> {
        AadjeW::new(self, 4)
    }
    ///Bit 5 - Automatic Adjustment Period Select
    #[inline(always)]
    pub fn aadjp(&mut self) -> AadjpW<Rcr2Spec> {
        AadjpW::new(self, 5)
    }
    ///Bit 6 - Hours Mode
    #[inline(always)]
    pub fn hr24(&mut self) -> Hr24W<Rcr2Spec> {
        Hr24W::new(self, 6)
    }
    ///Bit 7 - Count Mode Select
    #[inline(always)]
    pub fn cntmd(&mut self) -> CntmdW<Rcr2Spec> {
        CntmdW::new(self, 7)
    }
}
/**RTC Control Register 2 (in Calendar Count Mode)

You can [`read`](crate::Reg::read) this register and get [`rcr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Rcr2Spec;
impl crate::RegisterSpec for Rcr2Spec {
    type Ux = u8;
}
///`read()` method returns [`rcr2::R`](R) reader structure
impl crate::Readable for Rcr2Spec {}
///`write(|w| ..)` method takes [`rcr2::W`](W) writer structure
impl crate::Writable for Rcr2Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RCR2 to value 0
impl crate::Resettable for Rcr2Spec {}
