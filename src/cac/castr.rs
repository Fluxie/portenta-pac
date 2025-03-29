///Register `CASTR` reader
pub type R = crate::R<CastrSpec>;
/**Frequency Error Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ferrf {
    ///0: Clock frequency is within the allowable range
    _0 = 0,
    ///1: Clock frequency has deviated beyond the allowable range (frequency error).
    _1 = 1,
}
impl From<Ferrf> for bool {
    #[inline(always)]
    fn from(variant: Ferrf) -> Self {
        variant as u8 != 0
    }
}
///Field `FERRF` reader - Frequency Error Flag
pub type FerrfR = crate::BitReader<Ferrf>;
impl FerrfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ferrf {
        match self.bits {
            false => Ferrf::_0,
            true => Ferrf::_1,
        }
    }
    ///Clock frequency is within the allowable range
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ferrf::_0
    }
    ///Clock frequency has deviated beyond the allowable range (frequency error).
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ferrf::_1
    }
}
/**Measurement End Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mendf {
    ///0: Measurement is in progress
    _0 = 0,
    ///1: Measurement ended
    _1 = 1,
}
impl From<Mendf> for bool {
    #[inline(always)]
    fn from(variant: Mendf) -> Self {
        variant as u8 != 0
    }
}
///Field `MENDF` reader - Measurement End Flag
pub type MendfR = crate::BitReader<Mendf>;
impl MendfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mendf {
        match self.bits {
            false => Mendf::_0,
            true => Mendf::_1,
        }
    }
    ///Measurement is in progress
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mendf::_0
    }
    ///Measurement ended
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mendf::_1
    }
}
/**Overflow Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ovff {
    ///0: Counter has not overflowed
    _0 = 0,
    ///1: Counter overflowed
    _1 = 1,
}
impl From<Ovff> for bool {
    #[inline(always)]
    fn from(variant: Ovff) -> Self {
        variant as u8 != 0
    }
}
///Field `OVFF` reader - Overflow Flag
pub type OvffR = crate::BitReader<Ovff>;
impl OvffR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ovff {
        match self.bits {
            false => Ovff::_0,
            true => Ovff::_1,
        }
    }
    ///Counter has not overflowed
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ovff::_0
    }
    ///Counter overflowed
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ovff::_1
    }
}
impl R {
    ///Bit 0 - Frequency Error Flag
    #[inline(always)]
    pub fn ferrf(&self) -> FerrfR {
        FerrfR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Measurement End Flag
    #[inline(always)]
    pub fn mendf(&self) -> MendfR {
        MendfR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Overflow Flag
    #[inline(always)]
    pub fn ovff(&self) -> OvffR {
        OvffR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CASTR")
            .field("ferrf", &self.ferrf())
            .field("mendf", &self.mendf())
            .field("ovff", &self.ovff())
            .finish()
    }
}
/**CAC Status Register

You can [`read`](crate::Reg::read) this register and get [`castr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CastrSpec;
impl crate::RegisterSpec for CastrSpec {
    type Ux = u8;
}
///`read()` method returns [`castr::R`](R) reader structure
impl crate::Readable for CastrSpec {}
///`reset()` method sets CASTR to value 0
impl crate::Resettable for CastrSpec {}
