///Register `OSCSF` reader
pub type R = crate::R<OscsfSpec>;
/**HOCO Clock Oscillation Stabilization Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hocosf {
    ///0: The HOCO clock is stopped or is not yet stable
    _0 = 0,
    ///1: The HOCO clock is stable, so is available for use as the system clock
    _1 = 1,
}
impl From<Hocosf> for bool {
    #[inline(always)]
    fn from(variant: Hocosf) -> Self {
        variant as u8 != 0
    }
}
///Field `HOCOSF` reader - HOCO Clock Oscillation Stabilization Flag
pub type HocosfR = crate::BitReader<Hocosf>;
impl HocosfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Hocosf {
        match self.bits {
            false => Hocosf::_0,
            true => Hocosf::_1,
        }
    }
    ///The HOCO clock is stopped or is not yet stable
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Hocosf::_0
    }
    ///The HOCO clock is stable, so is available for use as the system clock
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Hocosf::_1
    }
}
/**Main Clock Oscillation Stabilization Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Moscsf {
    ///0: The main clock oscillator is stopped (MOSTP = 1) or is not yet stable
    _0 = 0,
    ///1: The main clock oscillator is stable, so is available for use as the system clock
    _1 = 1,
}
impl From<Moscsf> for bool {
    #[inline(always)]
    fn from(variant: Moscsf) -> Self {
        variant as u8 != 0
    }
}
///Field `MOSCSF` reader - Main Clock Oscillation Stabilization Flag
pub type MoscsfR = crate::BitReader<Moscsf>;
impl MoscsfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Moscsf {
        match self.bits {
            false => Moscsf::_0,
            true => Moscsf::_1,
        }
    }
    ///The main clock oscillator is stopped (MOSTP = 1) or is not yet stable
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Moscsf::_0
    }
    ///The main clock oscillator is stable, so is available for use as the system clock
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Moscsf::_1
    }
}
/**PLL Clock Oscillation Stabilization Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pllsf {
    ///0: The PLL clock is stopped, or oscillation of the PLL clock is not stable yet
    _0 = 0,
    ///1: The PLL clock is stable, so is available for use as the system clock
    _1 = 1,
}
impl From<Pllsf> for bool {
    #[inline(always)]
    fn from(variant: Pllsf) -> Self {
        variant as u8 != 0
    }
}
///Field `PLLSF` reader - PLL Clock Oscillation Stabilization Flag
pub type PllsfR = crate::BitReader<Pllsf>;
impl PllsfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pllsf {
        match self.bits {
            false => Pllsf::_0,
            true => Pllsf::_1,
        }
    }
    ///The PLL clock is stopped, or oscillation of the PLL clock is not stable yet
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pllsf::_0
    }
    ///The PLL clock is stable, so is available for use as the system clock
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pllsf::_1
    }
}
/**PLL2 Clock Oscillation Stabilization Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pll2sf {
    ///0: The PLL2 clock is stopped, or oscillation of the PLL2 clock is not stable yet
    _0 = 0,
    ///1: The PLL2 clock is stable
    _1 = 1,
}
impl From<Pll2sf> for bool {
    #[inline(always)]
    fn from(variant: Pll2sf) -> Self {
        variant as u8 != 0
    }
}
///Field `PLL2SF` reader - PLL2 Clock Oscillation Stabilization Flag
pub type Pll2sfR = crate::BitReader<Pll2sf>;
impl Pll2sfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pll2sf {
        match self.bits {
            false => Pll2sf::_0,
            true => Pll2sf::_1,
        }
    }
    ///The PLL2 clock is stopped, or oscillation of the PLL2 clock is not stable yet
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pll2sf::_0
    }
    ///The PLL2 clock is stable
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pll2sf::_1
    }
}
impl R {
    ///Bit 0 - HOCO Clock Oscillation Stabilization Flag
    #[inline(always)]
    pub fn hocosf(&self) -> HocosfR {
        HocosfR::new((self.bits & 1) != 0)
    }
    ///Bit 3 - Main Clock Oscillation Stabilization Flag
    #[inline(always)]
    pub fn moscsf(&self) -> MoscsfR {
        MoscsfR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - PLL Clock Oscillation Stabilization Flag
    #[inline(always)]
    pub fn pllsf(&self) -> PllsfR {
        PllsfR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - PLL2 Clock Oscillation Stabilization Flag
    #[inline(always)]
    pub fn pll2sf(&self) -> Pll2sfR {
        Pll2sfR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OSCSF")
            .field("hocosf", &self.hocosf())
            .field("moscsf", &self.moscsf())
            .field("pllsf", &self.pllsf())
            .field("pll2sf", &self.pll2sf())
            .finish()
    }
}
/**Oscillation Stabilization Flag Register

You can [`read`](crate::Reg::read) this register and get [`oscsf::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct OscsfSpec;
impl crate::RegisterSpec for OscsfSpec {
    type Ux = u8;
}
///`read()` method returns [`oscsf::R`](R) reader structure
impl crate::Readable for OscsfSpec {}
///`reset()` method sets OSCSF to value 0
impl crate::Resettable for OscsfSpec {}
