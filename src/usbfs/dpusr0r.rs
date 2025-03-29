///Register `DPUSR0R` reader
pub type R = crate::R<Dpusr0rSpec>;
///Register `DPUSR0R` writer
pub type W = crate::W<Dpusr0rSpec>;
/**USB Single-ended Receiver Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Srpc0 {
    ///0: Disable input through DP and DM inputs
    _0 = 0,
    ///1: Enable input through DP and DM inputs
    _1 = 1,
}
impl From<Srpc0> for bool {
    #[inline(always)]
    fn from(variant: Srpc0) -> Self {
        variant as u8 != 0
    }
}
///Field `SRPC0` reader - USB Single-ended Receiver Control
pub type Srpc0R = crate::BitReader<Srpc0>;
impl Srpc0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Srpc0 {
        match self.bits {
            false => Srpc0::_0,
            true => Srpc0::_1,
        }
    }
    ///Disable input through DP and DM inputs
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Srpc0::_0
    }
    ///Enable input through DP and DM inputs
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Srpc0::_1
    }
}
///Field `SRPC0` writer - USB Single-ended Receiver Control
pub type Srpc0W<'a, REG> = crate::BitWriter<'a, REG, Srpc0>;
impl<'a, REG> Srpc0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable input through DP and DM inputs
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Srpc0::_0)
    }
    ///Enable input through DP and DM inputs
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Srpc0::_1)
    }
}
/**DP Pull-Up Resistor Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rpue0 {
    ///0: Disable DP pull-up resistor
    _0 = 0,
    ///1: Enable DP pull-up resistor
    _1 = 1,
}
impl From<Rpue0> for bool {
    #[inline(always)]
    fn from(variant: Rpue0) -> Self {
        variant as u8 != 0
    }
}
///Field `RPUE0` reader - DP Pull-Up Resistor Control
pub type Rpue0R = crate::BitReader<Rpue0>;
impl Rpue0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rpue0 {
        match self.bits {
            false => Rpue0::_0,
            true => Rpue0::_1,
        }
    }
    ///Disable DP pull-up resistor
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rpue0::_0
    }
    ///Enable DP pull-up resistor
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rpue0::_1
    }
}
///Field `RPUE0` writer - DP Pull-Up Resistor Control
pub type Rpue0W<'a, REG> = crate::BitWriter<'a, REG, Rpue0>;
impl<'a, REG> Rpue0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable DP pull-up resistor
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rpue0::_0)
    }
    ///Enable DP pull-up resistor
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rpue0::_1)
    }
}
/**D+/D- Pull-Down Resistor Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Drpd0 {
    ///0: Disable DP/DM pull-down resistor
    _0 = 0,
    ///1: Enable DP/DM pull-down resistor
    _1 = 1,
}
impl From<Drpd0> for bool {
    #[inline(always)]
    fn from(variant: Drpd0) -> Self {
        variant as u8 != 0
    }
}
///Field `DRPD0` reader - D+/D- Pull-Down Resistor Control
pub type Drpd0R = crate::BitReader<Drpd0>;
impl Drpd0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Drpd0 {
        match self.bits {
            false => Drpd0::_0,
            true => Drpd0::_1,
        }
    }
    ///Disable DP/DM pull-down resistor
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Drpd0::_0
    }
    ///Enable DP/DM pull-down resistor
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Drpd0::_1
    }
}
///Field `DRPD0` writer - D+/D- Pull-Down Resistor Control
pub type Drpd0W<'a, REG> = crate::BitWriter<'a, REG, Drpd0>;
impl<'a, REG> Drpd0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable DP/DM pull-down resistor
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Drpd0::_0)
    }
    ///Enable DP/DM pull-down resistor
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Drpd0::_1)
    }
}
/**USB Transceiver Output Fix

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fixphy0 {
    ///0: Fix outputs in Normal mode and on return from Deep Software Standby mode
    _0 = 0,
    ///1: Fix outputs on transition to Deep Software Standby mode
    _1 = 1,
}
impl From<Fixphy0> for bool {
    #[inline(always)]
    fn from(variant: Fixphy0) -> Self {
        variant as u8 != 0
    }
}
///Field `FIXPHY0` reader - USB Transceiver Output Fix
pub type Fixphy0R = crate::BitReader<Fixphy0>;
impl Fixphy0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Fixphy0 {
        match self.bits {
            false => Fixphy0::_0,
            true => Fixphy0::_1,
        }
    }
    ///Fix outputs in Normal mode and on return from Deep Software Standby mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Fixphy0::_0
    }
    ///Fix outputs on transition to Deep Software Standby mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Fixphy0::_1
    }
}
///Field `FIXPHY0` writer - USB Transceiver Output Fix
pub type Fixphy0W<'a, REG> = crate::BitWriter<'a, REG, Fixphy0>;
impl<'a, REG> Fixphy0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Fix outputs in Normal mode and on return from Deep Software Standby mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Fixphy0::_0)
    }
    ///Fix outputs on transition to Deep Software Standby mode
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Fixphy0::_1)
    }
}
///Field `DP0` reader - USB D+ Input
pub type Dp0R = crate::BitReader;
///Field `DM0` reader - USB D- Input
pub type Dm0R = crate::BitReader;
///Field `DOVCA0` reader - USB OVRCURA Input
pub type Dovca0R = crate::BitReader;
///Field `DOVCB0` reader - USB OVRCURB Input
pub type Dovcb0R = crate::BitReader;
///Field `DVBSTS0` reader - USB VBUS Input
pub type Dvbsts0R = crate::BitReader;
impl R {
    ///Bit 0 - USB Single-ended Receiver Control
    #[inline(always)]
    pub fn srpc0(&self) -> Srpc0R {
        Srpc0R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DP Pull-Up Resistor Control
    #[inline(always)]
    pub fn rpue0(&self) -> Rpue0R {
        Rpue0R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - D+/D- Pull-Down Resistor Control
    #[inline(always)]
    pub fn drpd0(&self) -> Drpd0R {
        Drpd0R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - USB Transceiver Output Fix
    #[inline(always)]
    pub fn fixphy0(&self) -> Fixphy0R {
        Fixphy0R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 16 - USB D+ Input
    #[inline(always)]
    pub fn dp0(&self) -> Dp0R {
        Dp0R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - USB D- Input
    #[inline(always)]
    pub fn dm0(&self) -> Dm0R {
        Dm0R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 20 - USB OVRCURA Input
    #[inline(always)]
    pub fn dovca0(&self) -> Dovca0R {
        Dovca0R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - USB OVRCURB Input
    #[inline(always)]
    pub fn dovcb0(&self) -> Dovcb0R {
        Dovcb0R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 23 - USB VBUS Input
    #[inline(always)]
    pub fn dvbsts0(&self) -> Dvbsts0R {
        Dvbsts0R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DPUSR0R")
            .field("srpc0", &self.srpc0())
            .field("rpue0", &self.rpue0())
            .field("drpd0", &self.drpd0())
            .field("fixphy0", &self.fixphy0())
            .field("dp0", &self.dp0())
            .field("dm0", &self.dm0())
            .field("dovca0", &self.dovca0())
            .field("dovcb0", &self.dovcb0())
            .field("dvbsts0", &self.dvbsts0())
            .finish()
    }
}
impl W {
    ///Bit 0 - USB Single-ended Receiver Control
    #[inline(always)]
    pub fn srpc0(&mut self) -> Srpc0W<Dpusr0rSpec> {
        Srpc0W::new(self, 0)
    }
    ///Bit 1 - DP Pull-Up Resistor Control
    #[inline(always)]
    pub fn rpue0(&mut self) -> Rpue0W<Dpusr0rSpec> {
        Rpue0W::new(self, 1)
    }
    ///Bit 3 - D+/D- Pull-Down Resistor Control
    #[inline(always)]
    pub fn drpd0(&mut self) -> Drpd0W<Dpusr0rSpec> {
        Drpd0W::new(self, 3)
    }
    ///Bit 4 - USB Transceiver Output Fix
    #[inline(always)]
    pub fn fixphy0(&mut self) -> Fixphy0W<Dpusr0rSpec> {
        Fixphy0W::new(self, 4)
    }
}
/**Deep Software Standby USB Transceiver Control/Pin Monitor Register

You can [`read`](crate::Reg::read) this register and get [`dpusr0r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpusr0r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Dpusr0rSpec;
impl crate::RegisterSpec for Dpusr0rSpec {
    type Ux = u32;
}
///`read()` method returns [`dpusr0r::R`](R) reader structure
impl crate::Readable for Dpusr0rSpec {}
///`write(|w| ..)` method takes [`dpusr0r::W`](W) writer structure
impl crate::Writable for Dpusr0rSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DPUSR0R to value 0
impl crate::Resettable for Dpusr0rSpec {}
