///Register `WUPEN0` reader
pub type R = crate::R<Wupen0Spec>;
///Register `WUPEN0` writer
pub type W = crate::W<Wupen0Spec>;
/**IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 15)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Irqwupen {
    ///0: Software Standby/Snooze Mode returns by IRQn interrupt is disabled
    _0 = 0,
    ///1: Software Standby/Snooze Mode returns by IRQn interrupt is enabled
    _1 = 1,
}
impl From<Irqwupen> for u16 {
    #[inline(always)]
    fn from(variant: Irqwupen) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Irqwupen {
    type Ux = u16;
}
impl crate::IsEnum for Irqwupen {}
///Field `IRQWUPEN` reader - IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 15)
pub type IrqwupenR = crate::FieldReader<Irqwupen>;
impl IrqwupenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<Irqwupen> {
        match self.bits {
            0 => Some(Irqwupen::_0),
            1 => Some(Irqwupen::_1),
            _ => None,
        }
    }
    ///Software Standby/Snooze Mode returns by IRQn interrupt is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Irqwupen::_0
    }
    ///Software Standby/Snooze Mode returns by IRQn interrupt is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Irqwupen::_1
    }
}
///Field `IRQWUPEN` writer - IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 15)
pub type IrqwupenW<'a, REG> = crate::FieldWriter<'a, REG, 16, Irqwupen>;
impl<'a, REG> IrqwupenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///Software Standby/Snooze Mode returns by IRQn interrupt is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Irqwupen::_0)
    }
    ///Software Standby/Snooze Mode returns by IRQn interrupt is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Irqwupen::_1)
    }
}
/**IWDT Interrupt Software Standby/Snooze Mode Returns Enable bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iwdtwupen {
    ///0: Software Standby/Snooze Mode returns by IWDT interrupt is disabled
    _0 = 0,
    ///1: Software Standby/Snooze Mode returns by IWDT interrupt is enabled
    _1 = 1,
}
impl From<Iwdtwupen> for bool {
    #[inline(always)]
    fn from(variant: Iwdtwupen) -> Self {
        variant as u8 != 0
    }
}
///Field `IWDTWUPEN` reader - IWDT Interrupt Software Standby/Snooze Mode Returns Enable bit
pub type IwdtwupenR = crate::BitReader<Iwdtwupen>;
impl IwdtwupenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Iwdtwupen {
        match self.bits {
            false => Iwdtwupen::_0,
            true => Iwdtwupen::_1,
        }
    }
    ///Software Standby/Snooze Mode returns by IWDT interrupt is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Iwdtwupen::_0
    }
    ///Software Standby/Snooze Mode returns by IWDT interrupt is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Iwdtwupen::_1
    }
}
///Field `IWDTWUPEN` writer - IWDT Interrupt Software Standby/Snooze Mode Returns Enable bit
pub type IwdtwupenW<'a, REG> = crate::BitWriter<'a, REG, Iwdtwupen>;
impl<'a, REG> IwdtwupenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Software Standby/Snooze Mode returns by IWDT interrupt is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Iwdtwupen::_0)
    }
    ///Software Standby/Snooze Mode returns by IWDT interrupt is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Iwdtwupen::_1)
    }
}
/**LVD1 Interrupt Software Standby/Snooze Mode Returns Enable bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lvd1wupen {
    ///0: Software Standby/Snooze Mode returns by LVD1 interrupt is disabled
    _0 = 0,
    ///1: Software Standby/Snooze Mode returns by LVD1 interrupt is enabled
    _1 = 1,
}
impl From<Lvd1wupen> for bool {
    #[inline(always)]
    fn from(variant: Lvd1wupen) -> Self {
        variant as u8 != 0
    }
}
///Field `LVD1WUPEN` reader - LVD1 Interrupt Software Standby/Snooze Mode Returns Enable bit
pub type Lvd1wupenR = crate::BitReader<Lvd1wupen>;
impl Lvd1wupenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Lvd1wupen {
        match self.bits {
            false => Lvd1wupen::_0,
            true => Lvd1wupen::_1,
        }
    }
    ///Software Standby/Snooze Mode returns by LVD1 interrupt is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lvd1wupen::_0
    }
    ///Software Standby/Snooze Mode returns by LVD1 interrupt is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lvd1wupen::_1
    }
}
///Field `LVD1WUPEN` writer - LVD1 Interrupt Software Standby/Snooze Mode Returns Enable bit
pub type Lvd1wupenW<'a, REG> = crate::BitWriter<'a, REG, Lvd1wupen>;
impl<'a, REG> Lvd1wupenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Software Standby/Snooze Mode returns by LVD1 interrupt is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd1wupen::_0)
    }
    ///Software Standby/Snooze Mode returns by LVD1 interrupt is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd1wupen::_1)
    }
}
/**LVD2 Interrupt Software Standby/Snooze Mode Returns Enable bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lvd2wupen {
    ///0: Software Standby/Snooze Mode returns by LVD2 interrupt is disabled
    _0 = 0,
    ///1: Software Standby/Snooze Mode returns by LVD2 interrupt is enabled
    _1 = 1,
}
impl From<Lvd2wupen> for bool {
    #[inline(always)]
    fn from(variant: Lvd2wupen) -> Self {
        variant as u8 != 0
    }
}
///Field `LVD2WUPEN` reader - LVD2 Interrupt Software Standby/Snooze Mode Returns Enable bit
pub type Lvd2wupenR = crate::BitReader<Lvd2wupen>;
impl Lvd2wupenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Lvd2wupen {
        match self.bits {
            false => Lvd2wupen::_0,
            true => Lvd2wupen::_1,
        }
    }
    ///Software Standby/Snooze Mode returns by LVD2 interrupt is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lvd2wupen::_0
    }
    ///Software Standby/Snooze Mode returns by LVD2 interrupt is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lvd2wupen::_1
    }
}
///Field `LVD2WUPEN` writer - LVD2 Interrupt Software Standby/Snooze Mode Returns Enable bit
pub type Lvd2wupenW<'a, REG> = crate::BitWriter<'a, REG, Lvd2wupen>;
impl<'a, REG> Lvd2wupenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Software Standby/Snooze Mode returns by LVD2 interrupt is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd2wupen::_0)
    }
    ///Software Standby/Snooze Mode returns by LVD2 interrupt is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd2wupen::_1)
    }
}
/**RTC Alarm Interrupt Software Standby/Snooze Mode Returns Enable bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtcalmwupen {
    ///0: Software Standby/Snooze Mode returns by RTC alarm interrupt is disabled
    _0 = 0,
    ///1: Software Standby/Snooze Mode returns by RTC alarm interrupt is enabled
    _1 = 1,
}
impl From<Rtcalmwupen> for bool {
    #[inline(always)]
    fn from(variant: Rtcalmwupen) -> Self {
        variant as u8 != 0
    }
}
///Field `RTCALMWUPEN` reader - RTC Alarm Interrupt Software Standby/Snooze Mode Returns Enable bit
pub type RtcalmwupenR = crate::BitReader<Rtcalmwupen>;
impl RtcalmwupenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rtcalmwupen {
        match self.bits {
            false => Rtcalmwupen::_0,
            true => Rtcalmwupen::_1,
        }
    }
    ///Software Standby/Snooze Mode returns by RTC alarm interrupt is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rtcalmwupen::_0
    }
    ///Software Standby/Snooze Mode returns by RTC alarm interrupt is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rtcalmwupen::_1
    }
}
///Field `RTCALMWUPEN` writer - RTC Alarm Interrupt Software Standby/Snooze Mode Returns Enable bit
pub type RtcalmwupenW<'a, REG> = crate::BitWriter<'a, REG, Rtcalmwupen>;
impl<'a, REG> RtcalmwupenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Software Standby/Snooze Mode returns by RTC alarm interrupt is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcalmwupen::_0)
    }
    ///Software Standby/Snooze Mode returns by RTC alarm interrupt is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcalmwupen::_1)
    }
}
/**RTC Period Interrupt Software Standby/Snooze Mode Returns Enable bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtcprdwupen {
    ///0: Software Standby/Snooze Mode returns by RTC period interrupt is disabled
    _0 = 0,
    ///1: Software Standby/Snooze Mode returns by RTC period interrupt is enabled
    _1 = 1,
}
impl From<Rtcprdwupen> for bool {
    #[inline(always)]
    fn from(variant: Rtcprdwupen) -> Self {
        variant as u8 != 0
    }
}
///Field `RTCPRDWUPEN` reader - RTC Period Interrupt Software Standby/Snooze Mode Returns Enable bit
pub type RtcprdwupenR = crate::BitReader<Rtcprdwupen>;
impl RtcprdwupenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rtcprdwupen {
        match self.bits {
            false => Rtcprdwupen::_0,
            true => Rtcprdwupen::_1,
        }
    }
    ///Software Standby/Snooze Mode returns by RTC period interrupt is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rtcprdwupen::_0
    }
    ///Software Standby/Snooze Mode returns by RTC period interrupt is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rtcprdwupen::_1
    }
}
///Field `RTCPRDWUPEN` writer - RTC Period Interrupt Software Standby/Snooze Mode Returns Enable bit
pub type RtcprdwupenW<'a, REG> = crate::BitWriter<'a, REG, Rtcprdwupen>;
impl<'a, REG> RtcprdwupenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Software Standby/Snooze Mode returns by RTC period interrupt is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcprdwupen::_0)
    }
    ///Software Standby/Snooze Mode returns by RTC period interrupt is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcprdwupen::_1)
    }
}
/**USBHS Interrupt Software Standby/Snooze Mode Returns Enable bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbhswupen {
    ///0: Software Standby/Snooze Mode returns by USBHS interrupt is disabled
    _0 = 0,
    ///1: Software Standby/Snooze Mode returns by USBHS interrupt is enabled
    _1 = 1,
}
impl From<Usbhswupen> for bool {
    #[inline(always)]
    fn from(variant: Usbhswupen) -> Self {
        variant as u8 != 0
    }
}
///Field `USBHSWUPEN` reader - USBHS Interrupt Software Standby/Snooze Mode Returns Enable bit
pub type UsbhswupenR = crate::BitReader<Usbhswupen>;
impl UsbhswupenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Usbhswupen {
        match self.bits {
            false => Usbhswupen::_0,
            true => Usbhswupen::_1,
        }
    }
    ///Software Standby/Snooze Mode returns by USBHS interrupt is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Usbhswupen::_0
    }
    ///Software Standby/Snooze Mode returns by USBHS interrupt is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Usbhswupen::_1
    }
}
///Field `USBHSWUPEN` writer - USBHS Interrupt Software Standby/Snooze Mode Returns Enable bit
pub type UsbhswupenW<'a, REG> = crate::BitWriter<'a, REG, Usbhswupen>;
impl<'a, REG> UsbhswupenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Software Standby/Snooze Mode returns by USBHS interrupt is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Usbhswupen::_0)
    }
    ///Software Standby/Snooze Mode returns by USBHS interrupt is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Usbhswupen::_1)
    }
}
/**USBFS0 Interrupt Software Standby/Snooze Mode Returns Enable bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbfs0wupen {
    ///0: Software Standby/Snooze Mode returns by USBFS0 interrupt is disabled
    _0 = 0,
    ///1: Software Standby/Snooze Mode returns by USBFS0 interrupt is enabled
    _1 = 1,
}
impl From<Usbfs0wupen> for bool {
    #[inline(always)]
    fn from(variant: Usbfs0wupen) -> Self {
        variant as u8 != 0
    }
}
///Field `USBFS0WUPEN` reader - USBFS0 Interrupt Software Standby/Snooze Mode Returns Enable bit
pub type Usbfs0wupenR = crate::BitReader<Usbfs0wupen>;
impl Usbfs0wupenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Usbfs0wupen {
        match self.bits {
            false => Usbfs0wupen::_0,
            true => Usbfs0wupen::_1,
        }
    }
    ///Software Standby/Snooze Mode returns by USBFS0 interrupt is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Usbfs0wupen::_0
    }
    ///Software Standby/Snooze Mode returns by USBFS0 interrupt is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Usbfs0wupen::_1
    }
}
///Field `USBFS0WUPEN` writer - USBFS0 Interrupt Software Standby/Snooze Mode Returns Enable bit
pub type Usbfs0wupenW<'a, REG> = crate::BitWriter<'a, REG, Usbfs0wupen>;
impl<'a, REG> Usbfs0wupenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Software Standby/Snooze Mode returns by USBFS0 interrupt is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Usbfs0wupen::_0)
    }
    ///Software Standby/Snooze Mode returns by USBFS0 interrupt is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Usbfs0wupen::_1)
    }
}
/**AGT1 Underflow Interrupt Software Standby/Snooze Mode Returns Enable bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Agt1udwupen {
    ///0: Software Standby/Snooze Mode returns by AGT1 underflow interrupt is disabled
    _0 = 0,
    ///1: Software Standby/Snooze Mode returns by AGT1 underflow interrupt is enabled
    _1 = 1,
}
impl From<Agt1udwupen> for bool {
    #[inline(always)]
    fn from(variant: Agt1udwupen) -> Self {
        variant as u8 != 0
    }
}
///Field `AGT1UDWUPEN` reader - AGT1 Underflow Interrupt Software Standby/Snooze Mode Returns Enable bit
pub type Agt1udwupenR = crate::BitReader<Agt1udwupen>;
impl Agt1udwupenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Agt1udwupen {
        match self.bits {
            false => Agt1udwupen::_0,
            true => Agt1udwupen::_1,
        }
    }
    ///Software Standby/Snooze Mode returns by AGT1 underflow interrupt is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Agt1udwupen::_0
    }
    ///Software Standby/Snooze Mode returns by AGT1 underflow interrupt is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Agt1udwupen::_1
    }
}
///Field `AGT1UDWUPEN` writer - AGT1 Underflow Interrupt Software Standby/Snooze Mode Returns Enable bit
pub type Agt1udwupenW<'a, REG> = crate::BitWriter<'a, REG, Agt1udwupen>;
impl<'a, REG> Agt1udwupenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Software Standby/Snooze Mode returns by AGT1 underflow interrupt is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Agt1udwupen::_0)
    }
    ///Software Standby/Snooze Mode returns by AGT1 underflow interrupt is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Agt1udwupen::_1)
    }
}
/**AGT1 Compare Match A Interrupt Software Standby/Snooze Mode Returns Enable bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Agt1cawupen {
    ///0: Software Standby/Snooze Mode returns by AGT1 compare match A interrupt is disabled
    _0 = 0,
    ///1: Software Standby/Snooze Mode returns by AGT1 compare match A interrupt is enabled
    _1 = 1,
}
impl From<Agt1cawupen> for bool {
    #[inline(always)]
    fn from(variant: Agt1cawupen) -> Self {
        variant as u8 != 0
    }
}
///Field `AGT1CAWUPEN` reader - AGT1 Compare Match A Interrupt Software Standby/Snooze Mode Returns Enable bit
pub type Agt1cawupenR = crate::BitReader<Agt1cawupen>;
impl Agt1cawupenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Agt1cawupen {
        match self.bits {
            false => Agt1cawupen::_0,
            true => Agt1cawupen::_1,
        }
    }
    ///Software Standby/Snooze Mode returns by AGT1 compare match A interrupt is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Agt1cawupen::_0
    }
    ///Software Standby/Snooze Mode returns by AGT1 compare match A interrupt is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Agt1cawupen::_1
    }
}
///Field `AGT1CAWUPEN` writer - AGT1 Compare Match A Interrupt Software Standby/Snooze Mode Returns Enable bit
pub type Agt1cawupenW<'a, REG> = crate::BitWriter<'a, REG, Agt1cawupen>;
impl<'a, REG> Agt1cawupenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Software Standby/Snooze Mode returns by AGT1 compare match A interrupt is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Agt1cawupen::_0)
    }
    ///Software Standby/Snooze Mode returns by AGT1 compare match A interrupt is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Agt1cawupen::_1)
    }
}
/**AGT1 Compare Match B Interrupt Software Standby/Snooze Mode Returns Enable bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Agt1cbwupen {
    ///0: Software Standby/Snooze Mode returns by AGT1 compare match B interrupt is disabled
    _0 = 0,
    ///1: Software Standby/Snooze Mode returns by AGT1 compare match B interrupt is enabled
    _1 = 1,
}
impl From<Agt1cbwupen> for bool {
    #[inline(always)]
    fn from(variant: Agt1cbwupen) -> Self {
        variant as u8 != 0
    }
}
///Field `AGT1CBWUPEN` reader - AGT1 Compare Match B Interrupt Software Standby/Snooze Mode Returns Enable bit
pub type Agt1cbwupenR = crate::BitReader<Agt1cbwupen>;
impl Agt1cbwupenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Agt1cbwupen {
        match self.bits {
            false => Agt1cbwupen::_0,
            true => Agt1cbwupen::_1,
        }
    }
    ///Software Standby/Snooze Mode returns by AGT1 compare match B interrupt is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Agt1cbwupen::_0
    }
    ///Software Standby/Snooze Mode returns by AGT1 compare match B interrupt is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Agt1cbwupen::_1
    }
}
///Field `AGT1CBWUPEN` writer - AGT1 Compare Match B Interrupt Software Standby/Snooze Mode Returns Enable bit
pub type Agt1cbwupenW<'a, REG> = crate::BitWriter<'a, REG, Agt1cbwupen>;
impl<'a, REG> Agt1cbwupenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Software Standby/Snooze Mode returns by AGT1 compare match B interrupt is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Agt1cbwupen::_0)
    }
    ///Software Standby/Snooze Mode returns by AGT1 compare match B interrupt is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Agt1cbwupen::_1)
    }
}
/**IIC0 Address Match Interrupt Software Standby/Snooze Mode Returns Enable bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iic0wupen {
    ///0: Software Standby/Snooze Mode returns by IIC0 address match interrupt is disabled
    _0 = 0,
    ///1: Software Standby/Snooze Mode returns by IIC0 address match interrupt is enabled
    _1 = 1,
}
impl From<Iic0wupen> for bool {
    #[inline(always)]
    fn from(variant: Iic0wupen) -> Self {
        variant as u8 != 0
    }
}
///Field `IIC0WUPEN` reader - IIC0 Address Match Interrupt Software Standby/Snooze Mode Returns Enable bit
pub type Iic0wupenR = crate::BitReader<Iic0wupen>;
impl Iic0wupenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Iic0wupen {
        match self.bits {
            false => Iic0wupen::_0,
            true => Iic0wupen::_1,
        }
    }
    ///Software Standby/Snooze Mode returns by IIC0 address match interrupt is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Iic0wupen::_0
    }
    ///Software Standby/Snooze Mode returns by IIC0 address match interrupt is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Iic0wupen::_1
    }
}
///Field `IIC0WUPEN` writer - IIC0 Address Match Interrupt Software Standby/Snooze Mode Returns Enable bit
pub type Iic0wupenW<'a, REG> = crate::BitWriter<'a, REG, Iic0wupen>;
impl<'a, REG> Iic0wupenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Software Standby/Snooze Mode returns by IIC0 address match interrupt is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Iic0wupen::_0)
    }
    ///Software Standby/Snooze Mode returns by IIC0 address match interrupt is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Iic0wupen::_1)
    }
}
impl R {
    ///Bits 0:15 - IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 15)
    #[inline(always)]
    pub fn irqwupen(&self) -> IrqwupenR {
        IrqwupenR::new((self.bits & 0xffff) as u16)
    }
    ///Bit 16 - IWDT Interrupt Software Standby/Snooze Mode Returns Enable bit
    #[inline(always)]
    pub fn iwdtwupen(&self) -> IwdtwupenR {
        IwdtwupenR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - LVD1 Interrupt Software Standby/Snooze Mode Returns Enable bit
    #[inline(always)]
    pub fn lvd1wupen(&self) -> Lvd1wupenR {
        Lvd1wupenR::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - LVD2 Interrupt Software Standby/Snooze Mode Returns Enable bit
    #[inline(always)]
    pub fn lvd2wupen(&self) -> Lvd2wupenR {
        Lvd2wupenR::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 24 - RTC Alarm Interrupt Software Standby/Snooze Mode Returns Enable bit
    #[inline(always)]
    pub fn rtcalmwupen(&self) -> RtcalmwupenR {
        RtcalmwupenR::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - RTC Period Interrupt Software Standby/Snooze Mode Returns Enable bit
    #[inline(always)]
    pub fn rtcprdwupen(&self) -> RtcprdwupenR {
        RtcprdwupenR::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - USBHS Interrupt Software Standby/Snooze Mode Returns Enable bit
    #[inline(always)]
    pub fn usbhswupen(&self) -> UsbhswupenR {
        UsbhswupenR::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - USBFS0 Interrupt Software Standby/Snooze Mode Returns Enable bit
    #[inline(always)]
    pub fn usbfs0wupen(&self) -> Usbfs0wupenR {
        Usbfs0wupenR::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - AGT1 Underflow Interrupt Software Standby/Snooze Mode Returns Enable bit
    #[inline(always)]
    pub fn agt1udwupen(&self) -> Agt1udwupenR {
        Agt1udwupenR::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - AGT1 Compare Match A Interrupt Software Standby/Snooze Mode Returns Enable bit
    #[inline(always)]
    pub fn agt1cawupen(&self) -> Agt1cawupenR {
        Agt1cawupenR::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - AGT1 Compare Match B Interrupt Software Standby/Snooze Mode Returns Enable bit
    #[inline(always)]
    pub fn agt1cbwupen(&self) -> Agt1cbwupenR {
        Agt1cbwupenR::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - IIC0 Address Match Interrupt Software Standby/Snooze Mode Returns Enable bit
    #[inline(always)]
    pub fn iic0wupen(&self) -> Iic0wupenR {
        Iic0wupenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WUPEN0")
            .field("irqwupen", &self.irqwupen())
            .field("iwdtwupen", &self.iwdtwupen())
            .field("lvd1wupen", &self.lvd1wupen())
            .field("lvd2wupen", &self.lvd2wupen())
            .field("rtcalmwupen", &self.rtcalmwupen())
            .field("rtcprdwupen", &self.rtcprdwupen())
            .field("usbhswupen", &self.usbhswupen())
            .field("usbfs0wupen", &self.usbfs0wupen())
            .field("agt1udwupen", &self.agt1udwupen())
            .field("agt1cawupen", &self.agt1cawupen())
            .field("agt1cbwupen", &self.agt1cbwupen())
            .field("iic0wupen", &self.iic0wupen())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 15)
    #[inline(always)]
    pub fn irqwupen(&mut self) -> IrqwupenW<Wupen0Spec> {
        IrqwupenW::new(self, 0)
    }
    ///Bit 16 - IWDT Interrupt Software Standby/Snooze Mode Returns Enable bit
    #[inline(always)]
    pub fn iwdtwupen(&mut self) -> IwdtwupenW<Wupen0Spec> {
        IwdtwupenW::new(self, 16)
    }
    ///Bit 18 - LVD1 Interrupt Software Standby/Snooze Mode Returns Enable bit
    #[inline(always)]
    pub fn lvd1wupen(&mut self) -> Lvd1wupenW<Wupen0Spec> {
        Lvd1wupenW::new(self, 18)
    }
    ///Bit 19 - LVD2 Interrupt Software Standby/Snooze Mode Returns Enable bit
    #[inline(always)]
    pub fn lvd2wupen(&mut self) -> Lvd2wupenW<Wupen0Spec> {
        Lvd2wupenW::new(self, 19)
    }
    ///Bit 24 - RTC Alarm Interrupt Software Standby/Snooze Mode Returns Enable bit
    #[inline(always)]
    pub fn rtcalmwupen(&mut self) -> RtcalmwupenW<Wupen0Spec> {
        RtcalmwupenW::new(self, 24)
    }
    ///Bit 25 - RTC Period Interrupt Software Standby/Snooze Mode Returns Enable bit
    #[inline(always)]
    pub fn rtcprdwupen(&mut self) -> RtcprdwupenW<Wupen0Spec> {
        RtcprdwupenW::new(self, 25)
    }
    ///Bit 26 - USBHS Interrupt Software Standby/Snooze Mode Returns Enable bit
    #[inline(always)]
    pub fn usbhswupen(&mut self) -> UsbhswupenW<Wupen0Spec> {
        UsbhswupenW::new(self, 26)
    }
    ///Bit 27 - USBFS0 Interrupt Software Standby/Snooze Mode Returns Enable bit
    #[inline(always)]
    pub fn usbfs0wupen(&mut self) -> Usbfs0wupenW<Wupen0Spec> {
        Usbfs0wupenW::new(self, 27)
    }
    ///Bit 28 - AGT1 Underflow Interrupt Software Standby/Snooze Mode Returns Enable bit
    #[inline(always)]
    pub fn agt1udwupen(&mut self) -> Agt1udwupenW<Wupen0Spec> {
        Agt1udwupenW::new(self, 28)
    }
    ///Bit 29 - AGT1 Compare Match A Interrupt Software Standby/Snooze Mode Returns Enable bit
    #[inline(always)]
    pub fn agt1cawupen(&mut self) -> Agt1cawupenW<Wupen0Spec> {
        Agt1cawupenW::new(self, 29)
    }
    ///Bit 30 - AGT1 Compare Match B Interrupt Software Standby/Snooze Mode Returns Enable bit
    #[inline(always)]
    pub fn agt1cbwupen(&mut self) -> Agt1cbwupenW<Wupen0Spec> {
        Agt1cbwupenW::new(self, 30)
    }
    ///Bit 31 - IIC0 Address Match Interrupt Software Standby/Snooze Mode Returns Enable bit
    #[inline(always)]
    pub fn iic0wupen(&mut self) -> Iic0wupenW<Wupen0Spec> {
        Iic0wupenW::new(self, 31)
    }
}
/**Wake Up Interrupt Enable Register 0

You can [`read`](crate::Reg::read) this register and get [`wupen0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wupen0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Wupen0Spec;
impl crate::RegisterSpec for Wupen0Spec {
    type Ux = u32;
}
///`read()` method returns [`wupen0::R`](R) reader structure
impl crate::Readable for Wupen0Spec {}
///`write(|w| ..)` method takes [`wupen0::W`](W) writer structure
impl crate::Writable for Wupen0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WUPEN0 to value 0
impl crate::Resettable for Wupen0Spec {}
