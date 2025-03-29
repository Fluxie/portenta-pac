///Register `RTCCR%s` reader
pub type R = crate::R<RtccrSpec>;
///Register `RTCCR%s` writer
pub type W = crate::W<RtccrSpec>;
/**Time Capture Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tcct {
    ///0: Do not detect events
    _00 = 0,
    ///1: Detect rising edge
    _01 = 1,
    ///2: Detect falling edge
    _10 = 2,
    ///3: Detect both edges
    _11 = 3,
}
impl From<Tcct> for u8 {
    #[inline(always)]
    fn from(variant: Tcct) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tcct {
    type Ux = u8;
}
impl crate::IsEnum for Tcct {}
///Field `TCCT` reader - Time Capture Control
pub type TcctR = crate::FieldReader<Tcct>;
impl TcctR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tcct {
        match self.bits {
            0 => Tcct::_00,
            1 => Tcct::_01,
            2 => Tcct::_10,
            3 => Tcct::_11,
            _ => unreachable!(),
        }
    }
    ///Do not detect events
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Tcct::_00
    }
    ///Detect rising edge
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Tcct::_01
    }
    ///Detect falling edge
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Tcct::_10
    }
    ///Detect both edges
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Tcct::_11
    }
}
///Field `TCCT` writer - Time Capture Control
pub type TcctW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tcct, crate::Safe>;
impl<'a, REG> TcctW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Do not detect events
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Tcct::_00)
    }
    ///Detect rising edge
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Tcct::_01)
    }
    ///Detect falling edge
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Tcct::_10)
    }
    ///Detect both edges
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Tcct::_11)
    }
}
/**Time Capture Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcst {
    ///0: No event detected
    _0 = 0,
    ///1: Event detected
    _1 = 1,
}
impl From<Tcst> for bool {
    #[inline(always)]
    fn from(variant: Tcst) -> Self {
        variant as u8 != 0
    }
}
///Field `TCST` reader - Time Capture Status
pub type TcstR = crate::BitReader<Tcst>;
impl TcstR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tcst {
        match self.bits {
            false => Tcst::_0,
            true => Tcst::_1,
        }
    }
    ///No event detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tcst::_0
    }
    ///Event detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tcst::_1
    }
}
///Field `TCST` writer - Time Capture Status
pub type TcstW<'a, REG> = crate::BitWriter<'a, REG, Tcst>;
impl<'a, REG> TcstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No event detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tcst::_0)
    }
    ///Event detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tcst::_1)
    }
}
/**Time Capture Noise Filter Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tcnf {
    ///0: Turn noise filter off
    _00 = 0,
    ///1: Setting prohibited
    _01 = 1,
    ///2: Turn noise filter on (count source)
    _10 = 2,
    ///3: Turn noise filter on (count source by divided by 32)
    _11 = 3,
}
impl From<Tcnf> for u8 {
    #[inline(always)]
    fn from(variant: Tcnf) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tcnf {
    type Ux = u8;
}
impl crate::IsEnum for Tcnf {}
///Field `TCNF` reader - Time Capture Noise Filter Control
pub type TcnfR = crate::FieldReader<Tcnf>;
impl TcnfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tcnf {
        match self.bits {
            0 => Tcnf::_00,
            1 => Tcnf::_01,
            2 => Tcnf::_10,
            3 => Tcnf::_11,
            _ => unreachable!(),
        }
    }
    ///Turn noise filter off
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Tcnf::_00
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Tcnf::_01
    }
    ///Turn noise filter on (count source)
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Tcnf::_10
    }
    ///Turn noise filter on (count source by divided by 32)
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Tcnf::_11
    }
}
///Field `TCNF` writer - Time Capture Noise Filter Control
pub type TcnfW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tcnf, crate::Safe>;
impl<'a, REG> TcnfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Turn noise filter off
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Tcnf::_00)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Tcnf::_01)
    }
    ///Turn noise filter on (count source)
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Tcnf::_10)
    }
    ///Turn noise filter on (count source by divided by 32)
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Tcnf::_11)
    }
}
/**Time Capture Event Input Pin Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcen {
    ///0: Disable the RTCICn pin as the time capture event input pin
    _0 = 0,
    ///1: Enable the RTCICn pin as the time capture event input pin
    _1 = 1,
}
impl From<Tcen> for bool {
    #[inline(always)]
    fn from(variant: Tcen) -> Self {
        variant as u8 != 0
    }
}
///Field `TCEN` reader - Time Capture Event Input Pin Enable
pub type TcenR = crate::BitReader<Tcen>;
impl TcenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tcen {
        match self.bits {
            false => Tcen::_0,
            true => Tcen::_1,
        }
    }
    ///Disable the RTCICn pin as the time capture event input pin
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tcen::_0
    }
    ///Enable the RTCICn pin as the time capture event input pin
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tcen::_1
    }
}
///Field `TCEN` writer - Time Capture Event Input Pin Enable
pub type TcenW<'a, REG> = crate::BitWriter<'a, REG, Tcen>;
impl<'a, REG> TcenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable the RTCICn pin as the time capture event input pin
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tcen::_0)
    }
    ///Enable the RTCICn pin as the time capture event input pin
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tcen::_1)
    }
}
impl R {
    ///Bits 0:1 - Time Capture Control
    #[inline(always)]
    pub fn tcct(&self) -> TcctR {
        TcctR::new(self.bits & 3)
    }
    ///Bit 2 - Time Capture Status
    #[inline(always)]
    pub fn tcst(&self) -> TcstR {
        TcstR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 4:5 - Time Capture Noise Filter Control
    #[inline(always)]
    pub fn tcnf(&self) -> TcnfR {
        TcnfR::new((self.bits >> 4) & 3)
    }
    ///Bit 7 - Time Capture Event Input Pin Enable
    #[inline(always)]
    pub fn tcen(&self) -> TcenR {
        TcenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTCCR")
            .field("tcct", &self.tcct())
            .field("tcst", &self.tcst())
            .field("tcnf", &self.tcnf())
            .field("tcen", &self.tcen())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Time Capture Control
    #[inline(always)]
    pub fn tcct(&mut self) -> TcctW<RtccrSpec> {
        TcctW::new(self, 0)
    }
    ///Bit 2 - Time Capture Status
    #[inline(always)]
    pub fn tcst(&mut self) -> TcstW<RtccrSpec> {
        TcstW::new(self, 2)
    }
    ///Bits 4:5 - Time Capture Noise Filter Control
    #[inline(always)]
    pub fn tcnf(&mut self) -> TcnfW<RtccrSpec> {
        TcnfW::new(self, 4)
    }
    ///Bit 7 - Time Capture Event Input Pin Enable
    #[inline(always)]
    pub fn tcen(&mut self) -> TcenW<RtccrSpec> {
        TcenW::new(self, 7)
    }
}
/**Time Capture Control Register %s

You can [`read`](crate::Reg::read) this register and get [`rtccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RtccrSpec;
impl crate::RegisterSpec for RtccrSpec {
    type Ux = u8;
}
///`read()` method returns [`rtccr::R`](R) reader structure
impl crate::Readable for RtccrSpec {}
///`write(|w| ..)` method takes [`rtccr::W`](W) writer structure
impl crate::Writable for RtccrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RTCCR%s to value 0
impl crate::Resettable for RtccrSpec {}
