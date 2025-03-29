///Register `DPUSR1R` reader
pub type R = crate::R<Dpusr1rSpec>;
///Register `DPUSR1R` writer
pub type W = crate::W<Dpusr1rSpec>;
/**USB DP Interrupt Enable/Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dpinte0 {
    ///0: Disable recovery from Deep Software Standby mode by DP input
    _0 = 0,
    ///1: Enable recovery from Deep Software Standby mode by DP input
    _1 = 1,
}
impl From<Dpinte0> for bool {
    #[inline(always)]
    fn from(variant: Dpinte0) -> Self {
        variant as u8 != 0
    }
}
///Field `DPINTE0` reader - USB DP Interrupt Enable/Clear
pub type Dpinte0R = crate::BitReader<Dpinte0>;
impl Dpinte0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dpinte0 {
        match self.bits {
            false => Dpinte0::_0,
            true => Dpinte0::_1,
        }
    }
    ///Disable recovery from Deep Software Standby mode by DP input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dpinte0::_0
    }
    ///Enable recovery from Deep Software Standby mode by DP input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dpinte0::_1
    }
}
///Field `DPINTE0` writer - USB DP Interrupt Enable/Clear
pub type Dpinte0W<'a, REG> = crate::BitWriter<'a, REG, Dpinte0>;
impl<'a, REG> Dpinte0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable recovery from Deep Software Standby mode by DP input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dpinte0::_0)
    }
    ///Enable recovery from Deep Software Standby mode by DP input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dpinte0::_1)
    }
}
/**USB DM Interrupt Enable/Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dminte0 {
    ///0: Disable recovery from Deep Software Standby mode by DM input
    _0 = 0,
    ///1: Enable recovery from Deep Software Standby mode by DM input
    _1 = 1,
}
impl From<Dminte0> for bool {
    #[inline(always)]
    fn from(variant: Dminte0) -> Self {
        variant as u8 != 0
    }
}
///Field `DMINTE0` reader - USB DM Interrupt Enable/Clear
pub type Dminte0R = crate::BitReader<Dminte0>;
impl Dminte0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dminte0 {
        match self.bits {
            false => Dminte0::_0,
            true => Dminte0::_1,
        }
    }
    ///Disable recovery from Deep Software Standby mode by DM input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dminte0::_0
    }
    ///Enable recovery from Deep Software Standby mode by DM input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dminte0::_1
    }
}
///Field `DMINTE0` writer - USB DM Interrupt Enable/Clear
pub type Dminte0W<'a, REG> = crate::BitWriter<'a, REG, Dminte0>;
impl<'a, REG> Dminte0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable recovery from Deep Software Standby mode by DM input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dminte0::_0)
    }
    ///Enable recovery from Deep Software Standby mode by DM input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dminte0::_1)
    }
}
/**USB OVRCURA Interrupt Enable/Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dovrcrae0 {
    ///0: Disable recovery from Deep Software Standby mode by OVRCURA input
    _0 = 0,
    ///1: Enable recovery from Deep Software Standby mode by OVRCURA input
    _1 = 1,
}
impl From<Dovrcrae0> for bool {
    #[inline(always)]
    fn from(variant: Dovrcrae0) -> Self {
        variant as u8 != 0
    }
}
///Field `DOVRCRAE0` reader - USB OVRCURA Interrupt Enable/Clear
pub type Dovrcrae0R = crate::BitReader<Dovrcrae0>;
impl Dovrcrae0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dovrcrae0 {
        match self.bits {
            false => Dovrcrae0::_0,
            true => Dovrcrae0::_1,
        }
    }
    ///Disable recovery from Deep Software Standby mode by OVRCURA input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dovrcrae0::_0
    }
    ///Enable recovery from Deep Software Standby mode by OVRCURA input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dovrcrae0::_1
    }
}
///Field `DOVRCRAE0` writer - USB OVRCURA Interrupt Enable/Clear
pub type Dovrcrae0W<'a, REG> = crate::BitWriter<'a, REG, Dovrcrae0>;
impl<'a, REG> Dovrcrae0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable recovery from Deep Software Standby mode by OVRCURA input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dovrcrae0::_0)
    }
    ///Enable recovery from Deep Software Standby mode by OVRCURA input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dovrcrae0::_1)
    }
}
/**USB OVRCURB Interrupt Enable/Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dovrcrbe0 {
    ///0: Disable recovery from Deep Software Standby mode by OVRCURB input
    _0 = 0,
    ///1: Enable recovery from Deep Software Standby mode by OVRCURB input
    _1 = 1,
}
impl From<Dovrcrbe0> for bool {
    #[inline(always)]
    fn from(variant: Dovrcrbe0) -> Self {
        variant as u8 != 0
    }
}
///Field `DOVRCRBE0` reader - USB OVRCURB Interrupt Enable/Clear
pub type Dovrcrbe0R = crate::BitReader<Dovrcrbe0>;
impl Dovrcrbe0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dovrcrbe0 {
        match self.bits {
            false => Dovrcrbe0::_0,
            true => Dovrcrbe0::_1,
        }
    }
    ///Disable recovery from Deep Software Standby mode by OVRCURB input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dovrcrbe0::_0
    }
    ///Enable recovery from Deep Software Standby mode by OVRCURB input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dovrcrbe0::_1
    }
}
///Field `DOVRCRBE0` writer - USB OVRCURB Interrupt Enable/Clear
pub type Dovrcrbe0W<'a, REG> = crate::BitWriter<'a, REG, Dovrcrbe0>;
impl<'a, REG> Dovrcrbe0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable recovery from Deep Software Standby mode by OVRCURB input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dovrcrbe0::_0)
    }
    ///Enable recovery from Deep Software Standby mode by OVRCURB input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dovrcrbe0::_1)
    }
}
/**USB VBUS Interrupt Enable/Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dvbse0 {
    ///0: Disable recovery from Deep Software Standby mode by VBUS input
    _0 = 0,
    ///1: Enable recovery from Deep Software Standby mode by VBUS input
    _1 = 1,
}
impl From<Dvbse0> for bool {
    #[inline(always)]
    fn from(variant: Dvbse0) -> Self {
        variant as u8 != 0
    }
}
///Field `DVBSE0` reader - USB VBUS Interrupt Enable/Clear
pub type Dvbse0R = crate::BitReader<Dvbse0>;
impl Dvbse0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dvbse0 {
        match self.bits {
            false => Dvbse0::_0,
            true => Dvbse0::_1,
        }
    }
    ///Disable recovery from Deep Software Standby mode by VBUS input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dvbse0::_0
    }
    ///Enable recovery from Deep Software Standby mode by VBUS input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dvbse0::_1
    }
}
///Field `DVBSE0` writer - USB VBUS Interrupt Enable/Clear
pub type Dvbse0W<'a, REG> = crate::BitWriter<'a, REG, Dvbse0>;
impl<'a, REG> Dvbse0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable recovery from Deep Software Standby mode by VBUS input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dvbse0::_0)
    }
    ///Enable recovery from Deep Software Standby mode by VBUS input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dvbse0::_1)
    }
}
/**USB DP Interrupt Source Recovery

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dpint0 {
    ///0: System has not recovered from Deep Software Standby mode
    _0 = 0,
    ///1: System recovered from Deep Software Standby mode because of DP
    _1 = 1,
}
impl From<Dpint0> for bool {
    #[inline(always)]
    fn from(variant: Dpint0) -> Self {
        variant as u8 != 0
    }
}
///Field `DPINT0` reader - USB DP Interrupt Source Recovery
pub type Dpint0R = crate::BitReader<Dpint0>;
impl Dpint0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dpint0 {
        match self.bits {
            false => Dpint0::_0,
            true => Dpint0::_1,
        }
    }
    ///System has not recovered from Deep Software Standby mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dpint0::_0
    }
    ///System recovered from Deep Software Standby mode because of DP
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dpint0::_1
    }
}
/**USB DM Interrupt Source Recovery

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmint0 {
    ///0: System has not recovered from Deep Software Standby mode
    _0 = 0,
    ///1: System recovered from Deep Software Standby mode because of DM input
    _1 = 1,
}
impl From<Dmint0> for bool {
    #[inline(always)]
    fn from(variant: Dmint0) -> Self {
        variant as u8 != 0
    }
}
///Field `DMINT0` reader - USB DM Interrupt Source Recovery
pub type Dmint0R = crate::BitReader<Dmint0>;
impl Dmint0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dmint0 {
        match self.bits {
            false => Dmint0::_0,
            true => Dmint0::_1,
        }
    }
    ///System has not recovered from Deep Software Standby mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dmint0::_0
    }
    ///System recovered from Deep Software Standby mode because of DM input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dmint0::_1
    }
}
/**USB OVRCURA Interrupt Source Recovery

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dovrcra0 {
    ///0: System has not recovered from Deep Software Standby mode
    _0 = 0,
    ///1: System recovered from Deep Software Standby mode because of OVRCURA input
    _1 = 1,
}
impl From<Dovrcra0> for bool {
    #[inline(always)]
    fn from(variant: Dovrcra0) -> Self {
        variant as u8 != 0
    }
}
///Field `DOVRCRA0` reader - USB OVRCURA Interrupt Source Recovery
pub type Dovrcra0R = crate::BitReader<Dovrcra0>;
impl Dovrcra0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dovrcra0 {
        match self.bits {
            false => Dovrcra0::_0,
            true => Dovrcra0::_1,
        }
    }
    ///System has not recovered from Deep Software Standby mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dovrcra0::_0
    }
    ///System recovered from Deep Software Standby mode because of OVRCURA input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dovrcra0::_1
    }
}
/**USB OVRCURB Interrupt Source Recovery

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dovrcrb0 {
    ///0: System has not recovered from Deep Software Standby mode
    _0 = 0,
    ///1: System recovered from Deep Software Standby mode because of OVRCURB input
    _1 = 1,
}
impl From<Dovrcrb0> for bool {
    #[inline(always)]
    fn from(variant: Dovrcrb0) -> Self {
        variant as u8 != 0
    }
}
///Field `DOVRCRB0` reader - USB OVRCURB Interrupt Source Recovery
pub type Dovrcrb0R = crate::BitReader<Dovrcrb0>;
impl Dovrcrb0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dovrcrb0 {
        match self.bits {
            false => Dovrcrb0::_0,
            true => Dovrcrb0::_1,
        }
    }
    ///System has not recovered from Deep Software Standby mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dovrcrb0::_0
    }
    ///System recovered from Deep Software Standby mode because of OVRCURB input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dovrcrb0::_1
    }
}
/**USB VBUS Interrupt Source Recovery

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dvbint0 {
    ///0: System has not recovered from Deep Software Standby mode
    _0 = 0,
    ///1: System recovered from Deep Software Standby mode because of VBUS input
    _1 = 1,
}
impl From<Dvbint0> for bool {
    #[inline(always)]
    fn from(variant: Dvbint0) -> Self {
        variant as u8 != 0
    }
}
///Field `DVBINT0` reader - USB VBUS Interrupt Source Recovery
pub type Dvbint0R = crate::BitReader<Dvbint0>;
impl Dvbint0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dvbint0 {
        match self.bits {
            false => Dvbint0::_0,
            true => Dvbint0::_1,
        }
    }
    ///System has not recovered from Deep Software Standby mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dvbint0::_0
    }
    ///System recovered from Deep Software Standby mode because of VBUS input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dvbint0::_1
    }
}
impl R {
    ///Bit 0 - USB DP Interrupt Enable/Clear
    #[inline(always)]
    pub fn dpinte0(&self) -> Dpinte0R {
        Dpinte0R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - USB DM Interrupt Enable/Clear
    #[inline(always)]
    pub fn dminte0(&self) -> Dminte0R {
        Dminte0R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - USB OVRCURA Interrupt Enable/Clear
    #[inline(always)]
    pub fn dovrcrae0(&self) -> Dovrcrae0R {
        Dovrcrae0R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - USB OVRCURB Interrupt Enable/Clear
    #[inline(always)]
    pub fn dovrcrbe0(&self) -> Dovrcrbe0R {
        Dovrcrbe0R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - USB VBUS Interrupt Enable/Clear
    #[inline(always)]
    pub fn dvbse0(&self) -> Dvbse0R {
        Dvbse0R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 16 - USB DP Interrupt Source Recovery
    #[inline(always)]
    pub fn dpint0(&self) -> Dpint0R {
        Dpint0R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - USB DM Interrupt Source Recovery
    #[inline(always)]
    pub fn dmint0(&self) -> Dmint0R {
        Dmint0R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 20 - USB OVRCURA Interrupt Source Recovery
    #[inline(always)]
    pub fn dovrcra0(&self) -> Dovrcra0R {
        Dovrcra0R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - USB OVRCURB Interrupt Source Recovery
    #[inline(always)]
    pub fn dovrcrb0(&self) -> Dovrcrb0R {
        Dovrcrb0R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 23 - USB VBUS Interrupt Source Recovery
    #[inline(always)]
    pub fn dvbint0(&self) -> Dvbint0R {
        Dvbint0R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DPUSR1R")
            .field("dpinte0", &self.dpinte0())
            .field("dminte0", &self.dminte0())
            .field("dovrcrae0", &self.dovrcrae0())
            .field("dovrcrbe0", &self.dovrcrbe0())
            .field("dvbse0", &self.dvbse0())
            .field("dpint0", &self.dpint0())
            .field("dmint0", &self.dmint0())
            .field("dovrcra0", &self.dovrcra0())
            .field("dovrcrb0", &self.dovrcrb0())
            .field("dvbint0", &self.dvbint0())
            .finish()
    }
}
impl W {
    ///Bit 0 - USB DP Interrupt Enable/Clear
    #[inline(always)]
    pub fn dpinte0(&mut self) -> Dpinte0W<Dpusr1rSpec> {
        Dpinte0W::new(self, 0)
    }
    ///Bit 1 - USB DM Interrupt Enable/Clear
    #[inline(always)]
    pub fn dminte0(&mut self) -> Dminte0W<Dpusr1rSpec> {
        Dminte0W::new(self, 1)
    }
    ///Bit 4 - USB OVRCURA Interrupt Enable/Clear
    #[inline(always)]
    pub fn dovrcrae0(&mut self) -> Dovrcrae0W<Dpusr1rSpec> {
        Dovrcrae0W::new(self, 4)
    }
    ///Bit 5 - USB OVRCURB Interrupt Enable/Clear
    #[inline(always)]
    pub fn dovrcrbe0(&mut self) -> Dovrcrbe0W<Dpusr1rSpec> {
        Dovrcrbe0W::new(self, 5)
    }
    ///Bit 7 - USB VBUS Interrupt Enable/Clear
    #[inline(always)]
    pub fn dvbse0(&mut self) -> Dvbse0W<Dpusr1rSpec> {
        Dvbse0W::new(self, 7)
    }
}
/**Deep Software Standby USB Suspend/Resume Interrupt Register

You can [`read`](crate::Reg::read) this register and get [`dpusr1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpusr1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Dpusr1rSpec;
impl crate::RegisterSpec for Dpusr1rSpec {
    type Ux = u32;
}
///`read()` method returns [`dpusr1r::R`](R) reader structure
impl crate::Readable for Dpusr1rSpec {}
///`write(|w| ..)` method takes [`dpusr1r::W`](W) writer structure
impl crate::Writable for Dpusr1rSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DPUSR1R to value 0
impl crate::Resettable for Dpusr1rSpec {}
