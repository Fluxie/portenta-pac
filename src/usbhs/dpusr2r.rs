///Register `DPUSR2R` reader
pub type R = crate::R<Dpusr2rSpec>;
///Register `DPUSR2R` writer
pub type W = crate::W<Dpusr2rSpec>;
/**Indication of Return from DP Interrupt Source

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dpint {
    ///0: System has not recovered from Deep Software Standby mode
    _0 = 0,
    ///1: System recovered from Deep Software Standby mode
    _1 = 1,
}
impl From<Dpint> for bool {
    #[inline(always)]
    fn from(variant: Dpint) -> Self {
        variant as u8 != 0
    }
}
///Field `DPINT` reader - Indication of Return from DP Interrupt Source
pub type DpintR = crate::BitReader<Dpint>;
impl DpintR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dpint {
        match self.bits {
            false => Dpint::_0,
            true => Dpint::_1,
        }
    }
    ///System has not recovered from Deep Software Standby mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dpint::_0
    }
    ///System recovered from Deep Software Standby mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dpint::_1
    }
}
/**Indication of Return from DM Interrupt Source

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmint {
    ///0: System has not recovered from Deep Software Standby mode
    _0 = 0,
    ///1: System recovered from Deep Software Standby mode
    _1 = 1,
}
impl From<Dmint> for bool {
    #[inline(always)]
    fn from(variant: Dmint) -> Self {
        variant as u8 != 0
    }
}
///Field `DMINT` reader - Indication of Return from DM Interrupt Source
pub type DmintR = crate::BitReader<Dmint>;
impl DmintR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dmint {
        match self.bits {
            false => Dmint::_0,
            true => Dmint::_1,
        }
    }
    ///System has not recovered from Deep Software Standby mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dmint::_0
    }
    ///System recovered from Deep Software Standby mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dmint::_1
    }
}
///Field `DPVAL` reader - DP Input
pub type DpvalR = crate::BitReader;
///Field `DMVAL` reader - DM Input
pub type DmvalR = crate::BitReader;
/**DP Interrupt Enable Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dpinte {
    ///0: Disable recovery from Deep Software Standby mode
    _0 = 0,
    ///1: Enable recovery from Deep Software Standby mode
    _1 = 1,
}
impl From<Dpinte> for bool {
    #[inline(always)]
    fn from(variant: Dpinte) -> Self {
        variant as u8 != 0
    }
}
///Field `DPINTE` reader - DP Interrupt Enable Clear
pub type DpinteR = crate::BitReader<Dpinte>;
impl DpinteR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dpinte {
        match self.bits {
            false => Dpinte::_0,
            true => Dpinte::_1,
        }
    }
    ///Disable recovery from Deep Software Standby mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dpinte::_0
    }
    ///Enable recovery from Deep Software Standby mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dpinte::_1
    }
}
///Field `DPINTE` writer - DP Interrupt Enable Clear
pub type DpinteW<'a, REG> = crate::BitWriter<'a, REG, Dpinte>;
impl<'a, REG> DpinteW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable recovery from Deep Software Standby mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dpinte::_0)
    }
    ///Enable recovery from Deep Software Standby mode
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dpinte::_1)
    }
}
/**DM Interrupt Enable Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dminte {
    ///0: Disable recovery from Deep Software Standby mode
    _0 = 0,
    ///1: Enable recovery from Deep Software Standby mode
    _1 = 1,
}
impl From<Dminte> for bool {
    #[inline(always)]
    fn from(variant: Dminte) -> Self {
        variant as u8 != 0
    }
}
///Field `DMINTE` reader - DM Interrupt Enable Clear
pub type DminteR = crate::BitReader<Dminte>;
impl DminteR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dminte {
        match self.bits {
            false => Dminte::_0,
            true => Dminte::_1,
        }
    }
    ///Disable recovery from Deep Software Standby mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dminte::_0
    }
    ///Enable recovery from Deep Software Standby mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dminte::_1
    }
}
///Field `DMINTE` writer - DM Interrupt Enable Clear
pub type DminteW<'a, REG> = crate::BitWriter<'a, REG, Dminte>;
impl<'a, REG> DminteW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable recovery from Deep Software Standby mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dminte::_0)
    }
    ///Enable recovery from Deep Software Standby mode
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dminte::_1)
    }
}
impl R {
    ///Bit 0 - Indication of Return from DP Interrupt Source
    #[inline(always)]
    pub fn dpint(&self) -> DpintR {
        DpintR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Indication of Return from DM Interrupt Source
    #[inline(always)]
    pub fn dmint(&self) -> DmintR {
        DmintR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - DP Input
    #[inline(always)]
    pub fn dpval(&self) -> DpvalR {
        DpvalR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - DM Input
    #[inline(always)]
    pub fn dmval(&self) -> DmvalR {
        DmvalR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - DP Interrupt Enable Clear
    #[inline(always)]
    pub fn dpinte(&self) -> DpinteR {
        DpinteR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - DM Interrupt Enable Clear
    #[inline(always)]
    pub fn dminte(&self) -> DminteR {
        DminteR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DPUSR2R")
            .field("dpint", &self.dpint())
            .field("dmint", &self.dmint())
            .field("dpval", &self.dpval())
            .field("dmval", &self.dmval())
            .field("dpinte", &self.dpinte())
            .field("dminte", &self.dminte())
            .finish()
    }
}
impl W {
    ///Bit 8 - DP Interrupt Enable Clear
    #[inline(always)]
    pub fn dpinte(&mut self) -> DpinteW<Dpusr2rSpec> {
        DpinteW::new(self, 8)
    }
    ///Bit 9 - DM Interrupt Enable Clear
    #[inline(always)]
    pub fn dminte(&mut self) -> DminteW<Dpusr2rSpec> {
        DminteW::new(self, 9)
    }
}
/**Deep Software Standby USB Suspend/Resume Interrupt Register

You can [`read`](crate::Reg::read) this register and get [`dpusr2r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpusr2r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Dpusr2rSpec;
impl crate::RegisterSpec for Dpusr2rSpec {
    type Ux = u16;
}
///`read()` method returns [`dpusr2r::R`](R) reader structure
impl crate::Readable for Dpusr2rSpec {}
///`write(|w| ..)` method takes [`dpusr2r::W`](W) writer structure
impl crate::Writable for Dpusr2rSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DPUSR2R to value 0
impl crate::Resettable for Dpusr2rSpec {}
