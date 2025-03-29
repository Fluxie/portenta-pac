///Register `PHYSET` reader
pub type R = crate::R<PhysetSpec>;
///Register `PHYSET` writer
pub type W = crate::W<PhysetSpec>;
/**Power-Down Control

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dirpd {
    ///0: Do not enter low power mode
    _0 = 0,
    ///1: Enter low power mode
    _1 = 1,
}
impl From<Dirpd> for bool {
    #[inline(always)]
    fn from(variant: Dirpd) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRPD` reader - Power-Down Control
pub type DirpdR = crate::BitReader<Dirpd>;
impl DirpdR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dirpd {
        match self.bits {
            false => Dirpd::_0,
            true => Dirpd::_1,
        }
    }
    ///Do not enter low power mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dirpd::_0
    }
    ///Enter low power mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dirpd::_1
    }
}
///Field `DIRPD` writer - Power-Down Control
pub type DirpdW<'a, REG> = crate::BitWriter<'a, REG, Dirpd>;
impl<'a, REG> DirpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not enter low power mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dirpd::_0)
    }
    ///Enter low power mode
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dirpd::_1)
    }
}
/**PLL Reset Control

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pllreset {
    ///0: Disable PLL reset control for UTMI_PHY
    _0 = 0,
    ///1: Enable PLL reset control for UTMI_PHY
    _1 = 1,
}
impl From<Pllreset> for bool {
    #[inline(always)]
    fn from(variant: Pllreset) -> Self {
        variant as u8 != 0
    }
}
///Field `PLLRESET` reader - PLL Reset Control
pub type PllresetR = crate::BitReader<Pllreset>;
impl PllresetR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pllreset {
        match self.bits {
            false => Pllreset::_0,
            true => Pllreset::_1,
        }
    }
    ///Disable PLL reset control for UTMI_PHY
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pllreset::_0
    }
    ///Enable PLL reset control for UTMI_PHY
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pllreset::_1
    }
}
///Field `PLLRESET` writer - PLL Reset Control
pub type PllresetW<'a, REG> = crate::BitWriter<'a, REG, Pllreset>;
impl<'a, REG> PllresetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable PLL reset control for UTMI_PHY
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pllreset::_0)
    }
    ///Enable PLL reset control for UTMI_PHY
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pllreset::_1)
    }
}
/**Charging Downstream Port Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cdpen {
    ///0: Disable downstream port charging
    _0 = 0,
    ///1: Enable downstream port charging
    _1 = 1,
}
impl From<Cdpen> for bool {
    #[inline(always)]
    fn from(variant: Cdpen) -> Self {
        variant as u8 != 0
    }
}
///Field `CDPEN` reader - Charging Downstream Port Enable
pub type CdpenR = crate::BitReader<Cdpen>;
impl CdpenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cdpen {
        match self.bits {
            false => Cdpen::_0,
            true => Cdpen::_1,
        }
    }
    ///Disable downstream port charging
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cdpen::_0
    }
    ///Enable downstream port charging
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cdpen::_1
    }
}
///Field `CDPEN` writer - Charging Downstream Port Enable
pub type CdpenW<'a, REG> = crate::BitWriter<'a, REG, Cdpen>;
impl<'a, REG> CdpenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable downstream port charging
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cdpen::_0)
    }
    ///Enable downstream port charging
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cdpen::_1)
    }
}
/**Input System Clock Frequency

Value on reset: 3*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clksel {
    ///0: 12 MHz
    _00 = 0,
    ///1: Setting prohibited
    _01 = 1,
    ///2: 20 MHz
    _10 = 2,
    ///3: 24 MHz
    _11 = 3,
}
impl From<Clksel> for u8 {
    #[inline(always)]
    fn from(variant: Clksel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clksel {
    type Ux = u8;
}
impl crate::IsEnum for Clksel {}
///Field `CLKSEL` reader - Input System Clock Frequency
pub type ClkselR = crate::FieldReader<Clksel>;
impl ClkselR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Clksel {
        match self.bits {
            0 => Clksel::_00,
            1 => Clksel::_01,
            2 => Clksel::_10,
            3 => Clksel::_11,
            _ => unreachable!(),
        }
    }
    ///12 MHz
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Clksel::_00
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Clksel::_01
    }
    ///20 MHz
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Clksel::_10
    }
    ///24 MHz
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Clksel::_11
    }
}
///Field `CLKSEL` writer - Input System Clock Frequency
pub type ClkselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Clksel, crate::Safe>;
impl<'a, REG> ClkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///12 MHz
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::_00)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::_01)
    }
    ///20 MHz
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::_10)
    }
    ///24 MHz
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::_11)
    }
}
/**Terminating Resistance Adjustment Cycle

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Repsel {
    ///0: No cycle is set
    _00 = 0,
    ///1: Adjust terminating resistance at 16-second intervals
    _01 = 1,
    ///2: Adjust terminating resistance at 64-second intervals
    _10 = 2,
    ///3: Adjust terminating resistance at 128-second intervals
    _11 = 3,
}
impl From<Repsel> for u8 {
    #[inline(always)]
    fn from(variant: Repsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Repsel {
    type Ux = u8;
}
impl crate::IsEnum for Repsel {}
///Field `REPSEL` reader - Terminating Resistance Adjustment Cycle
pub type RepselR = crate::FieldReader<Repsel>;
impl RepselR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Repsel {
        match self.bits {
            0 => Repsel::_00,
            1 => Repsel::_01,
            2 => Repsel::_10,
            3 => Repsel::_11,
            _ => unreachable!(),
        }
    }
    ///No cycle is set
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Repsel::_00
    }
    ///Adjust terminating resistance at 16-second intervals
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Repsel::_01
    }
    ///Adjust terminating resistance at 64-second intervals
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Repsel::_10
    }
    ///Adjust terminating resistance at 128-second intervals
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Repsel::_11
    }
}
///Field `REPSEL` writer - Terminating Resistance Adjustment Cycle
pub type RepselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Repsel, crate::Safe>;
impl<'a, REG> RepselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No cycle is set
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Repsel::_00)
    }
    ///Adjust terminating resistance at 16-second intervals
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Repsel::_01)
    }
    ///Adjust terminating resistance at 64-second intervals
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Repsel::_10)
    }
    ///Adjust terminating resistance at 128-second intervals
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Repsel::_11)
    }
}
/**Forcibly Start Terminating Resistance Adjustment

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Repstart {
    ///0: Force terminating resistance adjustment to start
    _0 = 0,
    ///1: Do not force terminating resistance adjustment to start
    _1 = 1,
}
impl From<Repstart> for bool {
    #[inline(always)]
    fn from(variant: Repstart) -> Self {
        variant as u8 != 0
    }
}
///Field `REPSTART` reader - Forcibly Start Terminating Resistance Adjustment
pub type RepstartR = crate::BitReader<Repstart>;
impl RepstartR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Repstart {
        match self.bits {
            false => Repstart::_0,
            true => Repstart::_1,
        }
    }
    ///Force terminating resistance adjustment to start
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Repstart::_0
    }
    ///Do not force terminating resistance adjustment to start
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Repstart::_1
    }
}
///Field `REPSTART` writer - Forcibly Start Terminating Resistance Adjustment
pub type RepstartW<'a, REG> = crate::BitWriter<'a, REG, Repstart>;
impl<'a, REG> RepstartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Force terminating resistance adjustment to start
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Repstart::_0)
    }
    ///Do not force terminating resistance adjustment to start
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Repstart::_1)
    }
}
/**CL-only mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hseb {
    ///0: Disable CL-only mode
    _0 = 0,
    ///1: Enable CL-only mode
    _1 = 1,
}
impl From<Hseb> for bool {
    #[inline(always)]
    fn from(variant: Hseb) -> Self {
        variant as u8 != 0
    }
}
///Field `HSEB` reader - CL-only mode
pub type HsebR = crate::BitReader<Hseb>;
impl HsebR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Hseb {
        match self.bits {
            false => Hseb::_0,
            true => Hseb::_1,
        }
    }
    ///Disable CL-only mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Hseb::_0
    }
    ///Enable CL-only mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Hseb::_1
    }
}
///Field `HSEB` writer - CL-only mode
pub type HsebW<'a, REG> = crate::BitWriter<'a, REG, Hseb>;
impl<'a, REG> HsebW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable CL-only mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Hseb::_0)
    }
    ///Enable CL-only mode
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Hseb::_1)
    }
}
impl R {
    ///Bit 0 - Power-Down Control
    #[inline(always)]
    pub fn dirpd(&self) -> DirpdR {
        DirpdR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - PLL Reset Control
    #[inline(always)]
    pub fn pllreset(&self) -> PllresetR {
        PllresetR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - Charging Downstream Port Enable
    #[inline(always)]
    pub fn cdpen(&self) -> CdpenR {
        CdpenR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:5 - Input System Clock Frequency
    #[inline(always)]
    pub fn clksel(&self) -> ClkselR {
        ClkselR::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 8:9 - Terminating Resistance Adjustment Cycle
    #[inline(always)]
    pub fn repsel(&self) -> RepselR {
        RepselR::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 11 - Forcibly Start Terminating Resistance Adjustment
    #[inline(always)]
    pub fn repstart(&self) -> RepstartR {
        RepstartR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 15 - CL-only mode
    #[inline(always)]
    pub fn hseb(&self) -> HsebR {
        HsebR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PHYSET")
            .field("dirpd", &self.dirpd())
            .field("pllreset", &self.pllreset())
            .field("cdpen", &self.cdpen())
            .field("clksel", &self.clksel())
            .field("repsel", &self.repsel())
            .field("repstart", &self.repstart())
            .field("hseb", &self.hseb())
            .finish()
    }
}
impl W {
    ///Bit 0 - Power-Down Control
    #[inline(always)]
    pub fn dirpd(&mut self) -> DirpdW<PhysetSpec> {
        DirpdW::new(self, 0)
    }
    ///Bit 1 - PLL Reset Control
    #[inline(always)]
    pub fn pllreset(&mut self) -> PllresetW<PhysetSpec> {
        PllresetW::new(self, 1)
    }
    ///Bit 3 - Charging Downstream Port Enable
    #[inline(always)]
    pub fn cdpen(&mut self) -> CdpenW<PhysetSpec> {
        CdpenW::new(self, 3)
    }
    ///Bits 4:5 - Input System Clock Frequency
    #[inline(always)]
    pub fn clksel(&mut self) -> ClkselW<PhysetSpec> {
        ClkselW::new(self, 4)
    }
    ///Bits 8:9 - Terminating Resistance Adjustment Cycle
    #[inline(always)]
    pub fn repsel(&mut self) -> RepselW<PhysetSpec> {
        RepselW::new(self, 8)
    }
    ///Bit 11 - Forcibly Start Terminating Resistance Adjustment
    #[inline(always)]
    pub fn repstart(&mut self) -> RepstartW<PhysetSpec> {
        RepstartW::new(self, 11)
    }
    ///Bit 15 - CL-only mode
    #[inline(always)]
    pub fn hseb(&mut self) -> HsebW<PhysetSpec> {
        HsebW::new(self, 15)
    }
}
/**PHY Setting Register

You can [`read`](crate::Reg::read) this register and get [`physet::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`physet::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PhysetSpec;
impl crate::RegisterSpec for PhysetSpec {
    type Ux = u16;
}
///`read()` method returns [`physet::R`](R) reader structure
impl crate::Readable for PhysetSpec {}
///`write(|w| ..)` method takes [`physet::W`](W) writer structure
impl crate::Writable for PhysetSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PHYSET to value 0x33
impl crate::Resettable for PhysetSpec {
    const RESET_VALUE: u16 = 0x33;
}
