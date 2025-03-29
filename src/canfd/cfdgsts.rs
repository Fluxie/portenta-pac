///Register `CFDGSTS` reader
pub type R = crate::R<CfdgstsSpec>;
/**Global Reset Status

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Grststs {
    ///0: Not in Reset mode
    _0 = 0,
    ///1: In Reset mode
    _1 = 1,
}
impl From<Grststs> for bool {
    #[inline(always)]
    fn from(variant: Grststs) -> Self {
        variant as u8 != 0
    }
}
///Field `GRSTSTS` reader - Global Reset Status
pub type GrststsR = crate::BitReader<Grststs>;
impl GrststsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Grststs {
        match self.bits {
            false => Grststs::_0,
            true => Grststs::_1,
        }
    }
    ///Not in Reset mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Grststs::_0
    }
    ///In Reset mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Grststs::_1
    }
}
/**Global Halt Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ghltsts {
    ///0: Not in Halt mode
    _0 = 0,
    ///1: In Halt mode
    _1 = 1,
}
impl From<Ghltsts> for bool {
    #[inline(always)]
    fn from(variant: Ghltsts) -> Self {
        variant as u8 != 0
    }
}
///Field `GHLTSTS` reader - Global Halt Status
pub type GhltstsR = crate::BitReader<Ghltsts>;
impl GhltstsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ghltsts {
        match self.bits {
            false => Ghltsts::_0,
            true => Ghltsts::_1,
        }
    }
    ///Not in Halt mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ghltsts::_0
    }
    ///In Halt mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ghltsts::_1
    }
}
/**Global Sleep Status

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gslpsts {
    ///0: Not in Sleep mode
    _0 = 0,
    ///1: In Sleep mode
    _1 = 1,
}
impl From<Gslpsts> for bool {
    #[inline(always)]
    fn from(variant: Gslpsts) -> Self {
        variant as u8 != 0
    }
}
///Field `GSLPSTS` reader - Global Sleep Status
pub type GslpstsR = crate::BitReader<Gslpsts>;
impl GslpstsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Gslpsts {
        match self.bits {
            false => Gslpsts::_0,
            true => Gslpsts::_1,
        }
    }
    ///Not in Sleep mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Gslpsts::_0
    }
    ///In Sleep mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Gslpsts::_1
    }
}
/**Global RAM Initialization

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Graminit {
    ///0: RAM initialization is complete
    _0 = 0,
    ///1: RAM initialization is ongoing
    _1 = 1,
}
impl From<Graminit> for bool {
    #[inline(always)]
    fn from(variant: Graminit) -> Self {
        variant as u8 != 0
    }
}
///Field `GRAMINIT` reader - Global RAM Initialization
pub type GraminitR = crate::BitReader<Graminit>;
impl GraminitR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Graminit {
        match self.bits {
            false => Graminit::_0,
            true => Graminit::_1,
        }
    }
    ///RAM initialization is complete
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Graminit::_0
    }
    ///RAM initialization is ongoing
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Graminit::_1
    }
}
impl R {
    ///Bit 0 - Global Reset Status
    #[inline(always)]
    pub fn grststs(&self) -> GrststsR {
        GrststsR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Global Halt Status
    #[inline(always)]
    pub fn ghltsts(&self) -> GhltstsR {
        GhltstsR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Global Sleep Status
    #[inline(always)]
    pub fn gslpsts(&self) -> GslpstsR {
        GslpstsR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Global RAM Initialization
    #[inline(always)]
    pub fn graminit(&self) -> GraminitR {
        GraminitR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDGSTS")
            .field("grststs", &self.grststs())
            .field("ghltsts", &self.ghltsts())
            .field("gslpsts", &self.gslpsts())
            .field("graminit", &self.graminit())
            .finish()
    }
}
/**Global Status Register

You can [`read`](crate::Reg::read) this register and get [`cfdgsts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdgstsSpec;
impl crate::RegisterSpec for CfdgstsSpec {
    type Ux = u32;
}
///`read()` method returns [`cfdgsts::R`](R) reader structure
impl crate::Readable for CfdgstsSpec {}
///`reset()` method sets CFDGSTS to value 0x0d
impl crate::Resettable for CfdgstsSpec {
    const RESET_VALUE: u32 = 0x0d;
}
