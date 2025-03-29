///Register `RCR1` reader
pub type R = crate::R<Rcr1Spec>;
///Register `RCR1` writer
pub type W = crate::W<Rcr1Spec>;
/**Alarm Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aie {
    ///0: Disable alarm interrupt requests
    _0 = 0,
    ///1: Enable alarm interrupt requests
    _1 = 1,
}
impl From<Aie> for bool {
    #[inline(always)]
    fn from(variant: Aie) -> Self {
        variant as u8 != 0
    }
}
///Field `AIE` reader - Alarm Interrupt Enable
pub type AieR = crate::BitReader<Aie>;
impl AieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Aie {
        match self.bits {
            false => Aie::_0,
            true => Aie::_1,
        }
    }
    ///Disable alarm interrupt requests
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Aie::_0
    }
    ///Enable alarm interrupt requests
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Aie::_1
    }
}
///Field `AIE` writer - Alarm Interrupt Enable
pub type AieW<'a, REG> = crate::BitWriter<'a, REG, Aie>;
impl<'a, REG> AieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable alarm interrupt requests
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Aie::_0)
    }
    ///Enable alarm interrupt requests
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Aie::_1)
    }
}
/**Carry Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cie {
    ///0: Disable carry interrupt requests
    _0 = 0,
    ///1: Enable carry interrupt requests
    _1 = 1,
}
impl From<Cie> for bool {
    #[inline(always)]
    fn from(variant: Cie) -> Self {
        variant as u8 != 0
    }
}
///Field `CIE` reader - Carry Interrupt Enable
pub type CieR = crate::BitReader<Cie>;
impl CieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cie {
        match self.bits {
            false => Cie::_0,
            true => Cie::_1,
        }
    }
    ///Disable carry interrupt requests
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cie::_0
    }
    ///Enable carry interrupt requests
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cie::_1
    }
}
///Field `CIE` writer - Carry Interrupt Enable
pub type CieW<'a, REG> = crate::BitWriter<'a, REG, Cie>;
impl<'a, REG> CieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable carry interrupt requests
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cie::_0)
    }
    ///Enable carry interrupt requests
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cie::_1)
    }
}
/**Periodic Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pie {
    ///0: Disable periodic interrupt requests
    _0 = 0,
    ///1: Enable periodic interrupt requests
    _1 = 1,
}
impl From<Pie> for bool {
    #[inline(always)]
    fn from(variant: Pie) -> Self {
        variant as u8 != 0
    }
}
///Field `PIE` reader - Periodic Interrupt Enable
pub type PieR = crate::BitReader<Pie>;
impl PieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pie {
        match self.bits {
            false => Pie::_0,
            true => Pie::_1,
        }
    }
    ///Disable periodic interrupt requests
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pie::_0
    }
    ///Enable periodic interrupt requests
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pie::_1
    }
}
///Field `PIE` writer - Periodic Interrupt Enable
pub type PieW<'a, REG> = crate::BitWriter<'a, REG, Pie>;
impl<'a, REG> PieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable periodic interrupt requests
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pie::_0)
    }
    ///Enable periodic interrupt requests
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pie::_1)
    }
}
/**RTCOUT Output Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtcos {
    ///0: Outputs 1 Hz on RTCOUT
    _0 = 0,
    ///1: Outputs 64 Hz RTCOUT
    _1 = 1,
}
impl From<Rtcos> for bool {
    #[inline(always)]
    fn from(variant: Rtcos) -> Self {
        variant as u8 != 0
    }
}
///Field `RTCOS` reader - RTCOUT Output Select
pub type RtcosR = crate::BitReader<Rtcos>;
impl RtcosR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rtcos {
        match self.bits {
            false => Rtcos::_0,
            true => Rtcos::_1,
        }
    }
    ///Outputs 1 Hz on RTCOUT
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rtcos::_0
    }
    ///Outputs 64 Hz RTCOUT
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rtcos::_1
    }
}
///Field `RTCOS` writer - RTCOUT Output Select
pub type RtcosW<'a, REG> = crate::BitWriter<'a, REG, Rtcos>;
impl<'a, REG> RtcosW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Outputs 1 Hz on RTCOUT
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcos::_0)
    }
    ///Outputs 64 Hz RTCOUT
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcos::_1)
    }
}
/**Periodic Interrupt Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pes {
    ///6: Generate periodic interrupt every 1/256 second
    _0x6 = 6,
    ///7: Generate periodic interrupt every 1/128 second
    _0x7 = 7,
    ///8: Generate periodic interrupt every 1/64 second
    _0x8 = 8,
    ///9: Generate periodic interrupt every 1/32 second
    _0x9 = 9,
    ///10: Generate periodic interrupt every 1/16 second
    _0xA = 10,
    ///11: Generate periodic interrupt every 1/8 second
    _0xB = 11,
    ///12: Generate periodic interrupt every 1/4 second
    _0xC = 12,
    ///13: Generate periodic interrupt every 1/2 second
    _0xD = 13,
    ///14: Generate periodic interrupt every 1 second
    _0xE = 14,
    ///15: Generate periodic interrupt every 2 seconds
    _0xF = 15,
    ///0: Do not generate periodic interrupts
    Others = 0,
}
impl From<Pes> for u8 {
    #[inline(always)]
    fn from(variant: Pes) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pes {
    type Ux = u8;
}
impl crate::IsEnum for Pes {}
///Field `PES` reader - Periodic Interrupt Select
pub type PesR = crate::FieldReader<Pes>;
impl PesR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pes {
        match self.bits {
            6 => Pes::_0x6,
            7 => Pes::_0x7,
            8 => Pes::_0x8,
            9 => Pes::_0x9,
            10 => Pes::_0xA,
            11 => Pes::_0xB,
            12 => Pes::_0xC,
            13 => Pes::_0xD,
            14 => Pes::_0xE,
            15 => Pes::_0xF,
            _ => Pes::Others,
        }
    }
    ///Generate periodic interrupt every 1/256 second
    #[inline(always)]
    pub fn is_0x6(&self) -> bool {
        *self == Pes::_0x6
    }
    ///Generate periodic interrupt every 1/128 second
    #[inline(always)]
    pub fn is_0x7(&self) -> bool {
        *self == Pes::_0x7
    }
    ///Generate periodic interrupt every 1/64 second
    #[inline(always)]
    pub fn is_0x8(&self) -> bool {
        *self == Pes::_0x8
    }
    ///Generate periodic interrupt every 1/32 second
    #[inline(always)]
    pub fn is_0x9(&self) -> bool {
        *self == Pes::_0x9
    }
    ///Generate periodic interrupt every 1/16 second
    #[inline(always)]
    pub fn is_0x_a(&self) -> bool {
        *self == Pes::_0xA
    }
    ///Generate periodic interrupt every 1/8 second
    #[inline(always)]
    pub fn is_0x_b(&self) -> bool {
        *self == Pes::_0xB
    }
    ///Generate periodic interrupt every 1/4 second
    #[inline(always)]
    pub fn is_0x_c(&self) -> bool {
        *self == Pes::_0xC
    }
    ///Generate periodic interrupt every 1/2 second
    #[inline(always)]
    pub fn is_0x_d(&self) -> bool {
        *self == Pes::_0xD
    }
    ///Generate periodic interrupt every 1 second
    #[inline(always)]
    pub fn is_0x_e(&self) -> bool {
        *self == Pes::_0xE
    }
    ///Generate periodic interrupt every 2 seconds
    #[inline(always)]
    pub fn is_0x_f(&self) -> bool {
        *self == Pes::_0xF
    }
    ///Do not generate periodic interrupts
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Pes::Others)
    }
}
///Field `PES` writer - Periodic Interrupt Select
pub type PesW<'a, REG> = crate::FieldWriter<'a, REG, 4, Pes, crate::Safe>;
impl<'a, REG> PesW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Generate periodic interrupt every 1/256 second
    #[inline(always)]
    pub fn _0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Pes::_0x6)
    }
    ///Generate periodic interrupt every 1/128 second
    #[inline(always)]
    pub fn _0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Pes::_0x7)
    }
    ///Generate periodic interrupt every 1/64 second
    #[inline(always)]
    pub fn _0x8(self) -> &'a mut crate::W<REG> {
        self.variant(Pes::_0x8)
    }
    ///Generate periodic interrupt every 1/32 second
    #[inline(always)]
    pub fn _0x9(self) -> &'a mut crate::W<REG> {
        self.variant(Pes::_0x9)
    }
    ///Generate periodic interrupt every 1/16 second
    #[inline(always)]
    pub fn _0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(Pes::_0xA)
    }
    ///Generate periodic interrupt every 1/8 second
    #[inline(always)]
    pub fn _0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(Pes::_0xB)
    }
    ///Generate periodic interrupt every 1/4 second
    #[inline(always)]
    pub fn _0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(Pes::_0xC)
    }
    ///Generate periodic interrupt every 1/2 second
    #[inline(always)]
    pub fn _0x_d(self) -> &'a mut crate::W<REG> {
        self.variant(Pes::_0xD)
    }
    ///Generate periodic interrupt every 1 second
    #[inline(always)]
    pub fn _0x_e(self) -> &'a mut crate::W<REG> {
        self.variant(Pes::_0xE)
    }
    ///Generate periodic interrupt every 2 seconds
    #[inline(always)]
    pub fn _0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(Pes::_0xF)
    }
    ///Do not generate periodic interrupts
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Pes::Others)
    }
}
impl R {
    ///Bit 0 - Alarm Interrupt Enable
    #[inline(always)]
    pub fn aie(&self) -> AieR {
        AieR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Carry Interrupt Enable
    #[inline(always)]
    pub fn cie(&self) -> CieR {
        CieR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Periodic Interrupt Enable
    #[inline(always)]
    pub fn pie(&self) -> PieR {
        PieR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - RTCOUT Output Select
    #[inline(always)]
    pub fn rtcos(&self) -> RtcosR {
        RtcosR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:7 - Periodic Interrupt Select
    #[inline(always)]
    pub fn pes(&self) -> PesR {
        PesR::new((self.bits >> 4) & 0x0f)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCR1")
            .field("aie", &self.aie())
            .field("cie", &self.cie())
            .field("pie", &self.pie())
            .field("rtcos", &self.rtcos())
            .field("pes", &self.pes())
            .finish()
    }
}
impl W {
    ///Bit 0 - Alarm Interrupt Enable
    #[inline(always)]
    pub fn aie(&mut self) -> AieW<Rcr1Spec> {
        AieW::new(self, 0)
    }
    ///Bit 1 - Carry Interrupt Enable
    #[inline(always)]
    pub fn cie(&mut self) -> CieW<Rcr1Spec> {
        CieW::new(self, 1)
    }
    ///Bit 2 - Periodic Interrupt Enable
    #[inline(always)]
    pub fn pie(&mut self) -> PieW<Rcr1Spec> {
        PieW::new(self, 2)
    }
    ///Bit 3 - RTCOUT Output Select
    #[inline(always)]
    pub fn rtcos(&mut self) -> RtcosW<Rcr1Spec> {
        RtcosW::new(self, 3)
    }
    ///Bits 4:7 - Periodic Interrupt Select
    #[inline(always)]
    pub fn pes(&mut self) -> PesW<Rcr1Spec> {
        PesW::new(self, 4)
    }
}
/**RTC Control Register 1

You can [`read`](crate::Reg::read) this register and get [`rcr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Rcr1Spec;
impl crate::RegisterSpec for Rcr1Spec {
    type Ux = u8;
}
///`read()` method returns [`rcr1::R`](R) reader structure
impl crate::Readable for Rcr1Spec {}
///`write(|w| ..)` method takes [`rcr1::W`](W) writer structure
impl crate::Writable for Rcr1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RCR1 to value 0
impl crate::Resettable for Rcr1Spec {}
