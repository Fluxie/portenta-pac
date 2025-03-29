///Register `NMISR` reader
pub type R = crate::R<NmisrSpec>;
/**IWDT Underflow/Refresh Error Interrupt Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iwdtst {
    ///0: Interrupt not requested
    _0 = 0,
    ///1: Interrupt requested
    _1 = 1,
}
impl From<Iwdtst> for bool {
    #[inline(always)]
    fn from(variant: Iwdtst) -> Self {
        variant as u8 != 0
    }
}
///Field `IWDTST` reader - IWDT Underflow/Refresh Error Interrupt Status Flag
pub type IwdtstR = crate::BitReader<Iwdtst>;
impl IwdtstR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Iwdtst {
        match self.bits {
            false => Iwdtst::_0,
            true => Iwdtst::_1,
        }
    }
    ///Interrupt not requested
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Iwdtst::_0
    }
    ///Interrupt requested
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Iwdtst::_1
    }
}
/**WDT Underflow/Refresh Error Interrupt Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wdtst {
    ///0: Interrupt not requested
    _0 = 0,
    ///1: Interrupt requested
    _1 = 1,
}
impl From<Wdtst> for bool {
    #[inline(always)]
    fn from(variant: Wdtst) -> Self {
        variant as u8 != 0
    }
}
///Field `WDTST` reader - WDT Underflow/Refresh Error Interrupt Status Flag
pub type WdtstR = crate::BitReader<Wdtst>;
impl WdtstR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Wdtst {
        match self.bits {
            false => Wdtst::_0,
            true => Wdtst::_1,
        }
    }
    ///Interrupt not requested
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wdtst::_0
    }
    ///Interrupt requested
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wdtst::_1
    }
}
/**Voltage Monitor 1 Interrupt Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lvd1st {
    ///0: Interrupt not requested
    _0 = 0,
    ///1: Interrupt requested
    _1 = 1,
}
impl From<Lvd1st> for bool {
    #[inline(always)]
    fn from(variant: Lvd1st) -> Self {
        variant as u8 != 0
    }
}
///Field `LVD1ST` reader - Voltage Monitor 1 Interrupt Status Flag
pub type Lvd1stR = crate::BitReader<Lvd1st>;
impl Lvd1stR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Lvd1st {
        match self.bits {
            false => Lvd1st::_0,
            true => Lvd1st::_1,
        }
    }
    ///Interrupt not requested
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lvd1st::_0
    }
    ///Interrupt requested
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lvd1st::_1
    }
}
/**Voltage Monitor 2 Interrupt Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lvd2st {
    ///0: Interrupt not requested
    _0 = 0,
    ///1: Interrupt requested
    _1 = 1,
}
impl From<Lvd2st> for bool {
    #[inline(always)]
    fn from(variant: Lvd2st) -> Self {
        variant as u8 != 0
    }
}
///Field `LVD2ST` reader - Voltage Monitor 2 Interrupt Status Flag
pub type Lvd2stR = crate::BitReader<Lvd2st>;
impl Lvd2stR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Lvd2st {
        match self.bits {
            false => Lvd2st::_0,
            true => Lvd2st::_1,
        }
    }
    ///Interrupt not requested
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lvd2st::_0
    }
    ///Interrupt requested
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lvd2st::_1
    }
}
/**Main Clock Oscillation Stop Detection Interrupt Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ostst {
    ///0: Interrupt not requested for main clock oscillation stop
    _0 = 0,
    ///1: Interrupt requested for main clock oscillation stop
    _1 = 1,
}
impl From<Ostst> for bool {
    #[inline(always)]
    fn from(variant: Ostst) -> Self {
        variant as u8 != 0
    }
}
///Field `OSTST` reader - Main Clock Oscillation Stop Detection Interrupt Status Flag
pub type OststR = crate::BitReader<Ostst>;
impl OststR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ostst {
        match self.bits {
            false => Ostst::_0,
            true => Ostst::_1,
        }
    }
    ///Interrupt not requested for main clock oscillation stop
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ostst::_0
    }
    ///Interrupt requested for main clock oscillation stop
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ostst::_1
    }
}
/**NMI Pin Interrupt Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nmist {
    ///0: Interrupt not requested
    _0 = 0,
    ///1: Interrupt requested
    _1 = 1,
}
impl From<Nmist> for bool {
    #[inline(always)]
    fn from(variant: Nmist) -> Self {
        variant as u8 != 0
    }
}
///Field `NMIST` reader - NMI Pin Interrupt Status Flag
pub type NmistR = crate::BitReader<Nmist>;
impl NmistR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Nmist {
        match self.bits {
            false => Nmist::_0,
            true => Nmist::_1,
        }
    }
    ///Interrupt not requested
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Nmist::_0
    }
    ///Interrupt requested
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Nmist::_1
    }
}
/**SRAM Parity Error Interrupt Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rpest {
    ///0: Interrupt not requested
    _0 = 0,
    ///1: Interrupt requested
    _1 = 1,
}
impl From<Rpest> for bool {
    #[inline(always)]
    fn from(variant: Rpest) -> Self {
        variant as u8 != 0
    }
}
///Field `RPEST` reader - SRAM Parity Error Interrupt Status Flag
pub type RpestR = crate::BitReader<Rpest>;
impl RpestR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rpest {
        match self.bits {
            false => Rpest::_0,
            true => Rpest::_1,
        }
    }
    ///Interrupt not requested
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rpest::_0
    }
    ///Interrupt requested
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rpest::_1
    }
}
/**SRAM ECC Error Interrupt Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reccst {
    ///0: Interrupt not requested
    _0 = 0,
    ///1: Interrupt requested
    _1 = 1,
}
impl From<Reccst> for bool {
    #[inline(always)]
    fn from(variant: Reccst) -> Self {
        variant as u8 != 0
    }
}
///Field `RECCST` reader - SRAM ECC Error Interrupt Status Flag
pub type ReccstR = crate::BitReader<Reccst>;
impl ReccstR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Reccst {
        match self.bits {
            false => Reccst::_0,
            true => Reccst::_1,
        }
    }
    ///Interrupt not requested
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Reccst::_0
    }
    ///Interrupt requested
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Reccst::_1
    }
}
/**Bus Master MPU Error Interrupt Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Busmst {
    ///0: Interrupt not requested
    _0 = 0,
    ///1: Interrupt requested
    _1 = 1,
}
impl From<Busmst> for bool {
    #[inline(always)]
    fn from(variant: Busmst) -> Self {
        variant as u8 != 0
    }
}
///Field `BUSMST` reader - Bus Master MPU Error Interrupt Status Flag
pub type BusmstR = crate::BitReader<Busmst>;
impl BusmstR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Busmst {
        match self.bits {
            false => Busmst::_0,
            true => Busmst::_1,
        }
    }
    ///Interrupt not requested
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Busmst::_0
    }
    ///Interrupt requested
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Busmst::_1
    }
}
/**

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tzfst {
    ///0: Interrupt not requested
    _0 = 0,
    ///1: Interrupt requested
    _1 = 1,
}
impl From<Tzfst> for bool {
    #[inline(always)]
    fn from(variant: Tzfst) -> Self {
        variant as u8 != 0
    }
}
///Field `TZFST` reader -
pub type TzfstR = crate::BitReader<Tzfst>;
impl TzfstR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tzfst {
        match self.bits {
            false => Tzfst::_0,
            true => Tzfst::_1,
        }
    }
    ///Interrupt not requested
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tzfst::_0
    }
    ///Interrupt requested
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tzfst::_1
    }
}
/**

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpest {
    ///0: Interrupt not requested
    _0 = 0,
    ///1: Interrupt requested
    _1 = 1,
}
impl From<Cpest> for bool {
    #[inline(always)]
    fn from(variant: Cpest) -> Self {
        variant as u8 != 0
    }
}
///Field `CPEST` reader -
pub type CpestR = crate::BitReader<Cpest>;
impl CpestR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cpest {
        match self.bits {
            false => Cpest::_0,
            true => Cpest::_1,
        }
    }
    ///Interrupt not requested
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cpest::_0
    }
    ///Interrupt requested
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cpest::_1
    }
}
impl R {
    ///Bit 0 - IWDT Underflow/Refresh Error Interrupt Status Flag
    #[inline(always)]
    pub fn iwdtst(&self) -> IwdtstR {
        IwdtstR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - WDT Underflow/Refresh Error Interrupt Status Flag
    #[inline(always)]
    pub fn wdtst(&self) -> WdtstR {
        WdtstR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Voltage Monitor 1 Interrupt Status Flag
    #[inline(always)]
    pub fn lvd1st(&self) -> Lvd1stR {
        Lvd1stR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Voltage Monitor 2 Interrupt Status Flag
    #[inline(always)]
    pub fn lvd2st(&self) -> Lvd2stR {
        Lvd2stR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 6 - Main Clock Oscillation Stop Detection Interrupt Status Flag
    #[inline(always)]
    pub fn ostst(&self) -> OststR {
        OststR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - NMI Pin Interrupt Status Flag
    #[inline(always)]
    pub fn nmist(&self) -> NmistR {
        NmistR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - SRAM Parity Error Interrupt Status Flag
    #[inline(always)]
    pub fn rpest(&self) -> RpestR {
        RpestR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - SRAM ECC Error Interrupt Status Flag
    #[inline(always)]
    pub fn reccst(&self) -> ReccstR {
        ReccstR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - Bus Master MPU Error Interrupt Status Flag
    #[inline(always)]
    pub fn busmst(&self) -> BusmstR {
        BusmstR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 13
    #[inline(always)]
    pub fn tzfst(&self) -> TzfstR {
        TzfstR::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 15
    #[inline(always)]
    pub fn cpest(&self) -> CpestR {
        CpestR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NMISR")
            .field("iwdtst", &self.iwdtst())
            .field("wdtst", &self.wdtst())
            .field("lvd1st", &self.lvd1st())
            .field("lvd2st", &self.lvd2st())
            .field("ostst", &self.ostst())
            .field("nmist", &self.nmist())
            .field("rpest", &self.rpest())
            .field("reccst", &self.reccst())
            .field("busmst", &self.busmst())
            .field("tzfst", &self.tzfst())
            .field("cpest", &self.cpest())
            .finish()
    }
}
/**Non-Maskable Interrupt Status Register

You can [`read`](crate::Reg::read) this register and get [`nmisr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct NmisrSpec;
impl crate::RegisterSpec for NmisrSpec {
    type Ux = u16;
}
///`read()` method returns [`nmisr::R`](R) reader structure
impl crate::Readable for NmisrSpec {}
///`reset()` method sets NMISR to value 0
impl crate::Resettable for NmisrSpec {}
