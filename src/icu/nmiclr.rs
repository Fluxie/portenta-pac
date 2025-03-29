///Register `NMICLR` reader
pub type R = crate::R<NmiclrSpec>;
///Register `NMICLR` writer
pub type W = crate::W<NmiclrSpec>;
/**IWDT Underflow/Refresh Error Interrupt Status Flag Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iwdtclr {
    ///0: No effect
    _0 = 0,
    ///1: Clear the NMISR.IWDTST flag
    _1 = 1,
}
impl From<Iwdtclr> for bool {
    #[inline(always)]
    fn from(variant: Iwdtclr) -> Self {
        variant as u8 != 0
    }
}
///Field `IWDTCLR` reader - IWDT Underflow/Refresh Error Interrupt Status Flag Clear
pub type IwdtclrR = crate::BitReader<Iwdtclr>;
impl IwdtclrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Iwdtclr {
        match self.bits {
            false => Iwdtclr::_0,
            true => Iwdtclr::_1,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Iwdtclr::_0
    }
    ///Clear the NMISR.IWDTST flag
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Iwdtclr::_1
    }
}
///Field `IWDTCLR` writer - IWDT Underflow/Refresh Error Interrupt Status Flag Clear
pub type IwdtclrW<'a, REG> = crate::BitWriter<'a, REG, Iwdtclr>;
impl<'a, REG> IwdtclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Iwdtclr::_0)
    }
    ///Clear the NMISR.IWDTST flag
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Iwdtclr::_1)
    }
}
/**WDT Underflow/Refresh Error Interrupt Status Flag Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wdtclr {
    ///0: No effect
    _0 = 0,
    ///1: Clear the NMISR.WDTST flag
    _1 = 1,
}
impl From<Wdtclr> for bool {
    #[inline(always)]
    fn from(variant: Wdtclr) -> Self {
        variant as u8 != 0
    }
}
///Field `WDTCLR` reader - WDT Underflow/Refresh Error Interrupt Status Flag Clear
pub type WdtclrR = crate::BitReader<Wdtclr>;
impl WdtclrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Wdtclr {
        match self.bits {
            false => Wdtclr::_0,
            true => Wdtclr::_1,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wdtclr::_0
    }
    ///Clear the NMISR.WDTST flag
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wdtclr::_1
    }
}
///Field `WDTCLR` writer - WDT Underflow/Refresh Error Interrupt Status Flag Clear
pub type WdtclrW<'a, REG> = crate::BitWriter<'a, REG, Wdtclr>;
impl<'a, REG> WdtclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtclr::_0)
    }
    ///Clear the NMISR.WDTST flag
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtclr::_1)
    }
}
/**Voltage Monitor 1 Interrupt Status Flag Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lvd1clr {
    ///0: No effect
    _0 = 0,
    ///1: Clear the NMISR.LVD1ST flag
    _1 = 1,
}
impl From<Lvd1clr> for bool {
    #[inline(always)]
    fn from(variant: Lvd1clr) -> Self {
        variant as u8 != 0
    }
}
///Field `LVD1CLR` reader - Voltage Monitor 1 Interrupt Status Flag Clear
pub type Lvd1clrR = crate::BitReader<Lvd1clr>;
impl Lvd1clrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Lvd1clr {
        match self.bits {
            false => Lvd1clr::_0,
            true => Lvd1clr::_1,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lvd1clr::_0
    }
    ///Clear the NMISR.LVD1ST flag
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lvd1clr::_1
    }
}
///Field `LVD1CLR` writer - Voltage Monitor 1 Interrupt Status Flag Clear
pub type Lvd1clrW<'a, REG> = crate::BitWriter<'a, REG, Lvd1clr>;
impl<'a, REG> Lvd1clrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd1clr::_0)
    }
    ///Clear the NMISR.LVD1ST flag
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd1clr::_1)
    }
}
/**Voltage Monitor 2 Interrupt Status Flag Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lvd2clr {
    ///0: No effect
    _0 = 0,
    ///1: Clear the NMISR.LVD2ST flag.
    _1 = 1,
}
impl From<Lvd2clr> for bool {
    #[inline(always)]
    fn from(variant: Lvd2clr) -> Self {
        variant as u8 != 0
    }
}
///Field `LVD2CLR` reader - Voltage Monitor 2 Interrupt Status Flag Clear
pub type Lvd2clrR = crate::BitReader<Lvd2clr>;
impl Lvd2clrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Lvd2clr {
        match self.bits {
            false => Lvd2clr::_0,
            true => Lvd2clr::_1,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lvd2clr::_0
    }
    ///Clear the NMISR.LVD2ST flag.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lvd2clr::_1
    }
}
///Field `LVD2CLR` writer - Voltage Monitor 2 Interrupt Status Flag Clear
pub type Lvd2clrW<'a, REG> = crate::BitWriter<'a, REG, Lvd2clr>;
impl<'a, REG> Lvd2clrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd2clr::_0)
    }
    ///Clear the NMISR.LVD2ST flag.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd2clr::_1)
    }
}
/**Oscillation Stop Detection Interrupt Status Flag Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ostclr {
    ///0: No effect
    _0 = 0,
    ///1: Clear the NMISR.OSTST flag
    _1 = 1,
}
impl From<Ostclr> for bool {
    #[inline(always)]
    fn from(variant: Ostclr) -> Self {
        variant as u8 != 0
    }
}
///Field `OSTCLR` reader - Oscillation Stop Detection Interrupt Status Flag Clear
pub type OstclrR = crate::BitReader<Ostclr>;
impl OstclrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ostclr {
        match self.bits {
            false => Ostclr::_0,
            true => Ostclr::_1,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ostclr::_0
    }
    ///Clear the NMISR.OSTST flag
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ostclr::_1
    }
}
///Field `OSTCLR` writer - Oscillation Stop Detection Interrupt Status Flag Clear
pub type OstclrW<'a, REG> = crate::BitWriter<'a, REG, Ostclr>;
impl<'a, REG> OstclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ostclr::_0)
    }
    ///Clear the NMISR.OSTST flag
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ostclr::_1)
    }
}
/**NMI Pin Interrupt Status Flag Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nmiclr {
    ///0: No effect
    _0 = 0,
    ///1: Clear the NMISR.NMIST flag
    _1 = 1,
}
impl From<Nmiclr> for bool {
    #[inline(always)]
    fn from(variant: Nmiclr) -> Self {
        variant as u8 != 0
    }
}
///Field `NMICLR` reader - NMI Pin Interrupt Status Flag Clear
pub type NmiclrR = crate::BitReader<Nmiclr>;
impl NmiclrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Nmiclr {
        match self.bits {
            false => Nmiclr::_0,
            true => Nmiclr::_1,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Nmiclr::_0
    }
    ///Clear the NMISR.NMIST flag
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Nmiclr::_1
    }
}
///Field `NMICLR` writer - NMI Pin Interrupt Status Flag Clear
pub type NmiclrW<'a, REG> = crate::BitWriter<'a, REG, Nmiclr>;
impl<'a, REG> NmiclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Nmiclr::_0)
    }
    ///Clear the NMISR.NMIST flag
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Nmiclr::_1)
    }
}
/**SRAM Parity Error Interrupt Status Flag Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rpeclr {
    ///0: No effect
    _0 = 0,
    ///1: Clear the NMISR.RPEST flag
    _1 = 1,
}
impl From<Rpeclr> for bool {
    #[inline(always)]
    fn from(variant: Rpeclr) -> Self {
        variant as u8 != 0
    }
}
///Field `RPECLR` reader - SRAM Parity Error Interrupt Status Flag Clear
pub type RpeclrR = crate::BitReader<Rpeclr>;
impl RpeclrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rpeclr {
        match self.bits {
            false => Rpeclr::_0,
            true => Rpeclr::_1,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rpeclr::_0
    }
    ///Clear the NMISR.RPEST flag
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rpeclr::_1
    }
}
///Field `RPECLR` writer - SRAM Parity Error Interrupt Status Flag Clear
pub type RpeclrW<'a, REG> = crate::BitWriter<'a, REG, Rpeclr>;
impl<'a, REG> RpeclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rpeclr::_0)
    }
    ///Clear the NMISR.RPEST flag
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rpeclr::_1)
    }
}
/**SRAM ECC Error Interrupt Status Flag Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reccclr {
    ///0: No effect
    _0 = 0,
    ///1: Clear the NMISR.RECCST flag
    _1 = 1,
}
impl From<Reccclr> for bool {
    #[inline(always)]
    fn from(variant: Reccclr) -> Self {
        variant as u8 != 0
    }
}
///Field `RECCCLR` reader - SRAM ECC Error Interrupt Status Flag Clear
pub type ReccclrR = crate::BitReader<Reccclr>;
impl ReccclrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Reccclr {
        match self.bits {
            false => Reccclr::_0,
            true => Reccclr::_1,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Reccclr::_0
    }
    ///Clear the NMISR.RECCST flag
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Reccclr::_1
    }
}
///Field `RECCCLR` writer - SRAM ECC Error Interrupt Status Flag Clear
pub type ReccclrW<'a, REG> = crate::BitWriter<'a, REG, Reccclr>;
impl<'a, REG> ReccclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Reccclr::_0)
    }
    ///Clear the NMISR.RECCST flag
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Reccclr::_1)
    }
}
/**Bus Master MPU Error Interrupt Status Flag Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Busmclr {
    ///0: No effect
    _0 = 0,
    ///1: Clear the NMISR.BUSMST flag
    _1 = 1,
}
impl From<Busmclr> for bool {
    #[inline(always)]
    fn from(variant: Busmclr) -> Self {
        variant as u8 != 0
    }
}
///Field `BUSMCLR` reader - Bus Master MPU Error Interrupt Status Flag Clear
pub type BusmclrR = crate::BitReader<Busmclr>;
impl BusmclrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Busmclr {
        match self.bits {
            false => Busmclr::_0,
            true => Busmclr::_1,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Busmclr::_0
    }
    ///Clear the NMISR.BUSMST flag
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Busmclr::_1
    }
}
///Field `BUSMCLR` writer - Bus Master MPU Error Interrupt Status Flag Clear
pub type BusmclrW<'a, REG> = crate::BitWriter<'a, REG, Busmclr>;
impl<'a, REG> BusmclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Busmclr::_0)
    }
    ///Clear the NMISR.BUSMST flag
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Busmclr::_1)
    }
}
/**

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tzfclr {
    ///0: No effect
    _0 = 0,
    ///1: Clear the NMISR.TZFCLR flag
    _1 = 1,
}
impl From<Tzfclr> for bool {
    #[inline(always)]
    fn from(variant: Tzfclr) -> Self {
        variant as u8 != 0
    }
}
///Field `TZFCLR` reader -
pub type TzfclrR = crate::BitReader<Tzfclr>;
impl TzfclrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tzfclr {
        match self.bits {
            false => Tzfclr::_0,
            true => Tzfclr::_1,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tzfclr::_0
    }
    ///Clear the NMISR.TZFCLR flag
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tzfclr::_1
    }
}
///Field `TZFCLR` writer -
pub type TzfclrW<'a, REG> = crate::BitWriter<'a, REG, Tzfclr>;
impl<'a, REG> TzfclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tzfclr::_0)
    }
    ///Clear the NMISR.TZFCLR flag
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tzfclr::_1)
    }
}
/**

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpeclr {
    ///0: No effect
    _0 = 0,
    ///1: Clear the NMISR.CPECLR flag
    _1 = 1,
}
impl From<Cpeclr> for bool {
    #[inline(always)]
    fn from(variant: Cpeclr) -> Self {
        variant as u8 != 0
    }
}
///Field `CPECLR` reader -
pub type CpeclrR = crate::BitReader<Cpeclr>;
impl CpeclrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cpeclr {
        match self.bits {
            false => Cpeclr::_0,
            true => Cpeclr::_1,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cpeclr::_0
    }
    ///Clear the NMISR.CPECLR flag
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cpeclr::_1
    }
}
///Field `CPECLR` writer -
pub type CpeclrW<'a, REG> = crate::BitWriter<'a, REG, Cpeclr>;
impl<'a, REG> CpeclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cpeclr::_0)
    }
    ///Clear the NMISR.CPECLR flag
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cpeclr::_1)
    }
}
impl R {
    ///Bit 0 - IWDT Underflow/Refresh Error Interrupt Status Flag Clear
    #[inline(always)]
    pub fn iwdtclr(&self) -> IwdtclrR {
        IwdtclrR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - WDT Underflow/Refresh Error Interrupt Status Flag Clear
    #[inline(always)]
    pub fn wdtclr(&self) -> WdtclrR {
        WdtclrR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Voltage Monitor 1 Interrupt Status Flag Clear
    #[inline(always)]
    pub fn lvd1clr(&self) -> Lvd1clrR {
        Lvd1clrR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Voltage Monitor 2 Interrupt Status Flag Clear
    #[inline(always)]
    pub fn lvd2clr(&self) -> Lvd2clrR {
        Lvd2clrR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 6 - Oscillation Stop Detection Interrupt Status Flag Clear
    #[inline(always)]
    pub fn ostclr(&self) -> OstclrR {
        OstclrR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - NMI Pin Interrupt Status Flag Clear
    #[inline(always)]
    pub fn nmiclr(&self) -> NmiclrR {
        NmiclrR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - SRAM Parity Error Interrupt Status Flag Clear
    #[inline(always)]
    pub fn rpeclr(&self) -> RpeclrR {
        RpeclrR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - SRAM ECC Error Interrupt Status Flag Clear
    #[inline(always)]
    pub fn reccclr(&self) -> ReccclrR {
        ReccclrR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - Bus Master MPU Error Interrupt Status Flag Clear
    #[inline(always)]
    pub fn busmclr(&self) -> BusmclrR {
        BusmclrR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 13
    #[inline(always)]
    pub fn tzfclr(&self) -> TzfclrR {
        TzfclrR::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 15
    #[inline(always)]
    pub fn cpeclr(&self) -> CpeclrR {
        CpeclrR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NMICLR")
            .field("iwdtclr", &self.iwdtclr())
            .field("wdtclr", &self.wdtclr())
            .field("lvd1clr", &self.lvd1clr())
            .field("lvd2clr", &self.lvd2clr())
            .field("ostclr", &self.ostclr())
            .field("nmiclr", &self.nmiclr())
            .field("rpeclr", &self.rpeclr())
            .field("reccclr", &self.reccclr())
            .field("busmclr", &self.busmclr())
            .field("tzfclr", &self.tzfclr())
            .field("cpeclr", &self.cpeclr())
            .finish()
    }
}
impl W {
    ///Bit 0 - IWDT Underflow/Refresh Error Interrupt Status Flag Clear
    #[inline(always)]
    pub fn iwdtclr(&mut self) -> IwdtclrW<NmiclrSpec> {
        IwdtclrW::new(self, 0)
    }
    ///Bit 1 - WDT Underflow/Refresh Error Interrupt Status Flag Clear
    #[inline(always)]
    pub fn wdtclr(&mut self) -> WdtclrW<NmiclrSpec> {
        WdtclrW::new(self, 1)
    }
    ///Bit 2 - Voltage Monitor 1 Interrupt Status Flag Clear
    #[inline(always)]
    pub fn lvd1clr(&mut self) -> Lvd1clrW<NmiclrSpec> {
        Lvd1clrW::new(self, 2)
    }
    ///Bit 3 - Voltage Monitor 2 Interrupt Status Flag Clear
    #[inline(always)]
    pub fn lvd2clr(&mut self) -> Lvd2clrW<NmiclrSpec> {
        Lvd2clrW::new(self, 3)
    }
    ///Bit 6 - Oscillation Stop Detection Interrupt Status Flag Clear
    #[inline(always)]
    pub fn ostclr(&mut self) -> OstclrW<NmiclrSpec> {
        OstclrW::new(self, 6)
    }
    ///Bit 7 - NMI Pin Interrupt Status Flag Clear
    #[inline(always)]
    pub fn nmiclr(&mut self) -> NmiclrW<NmiclrSpec> {
        NmiclrW::new(self, 7)
    }
    ///Bit 8 - SRAM Parity Error Interrupt Status Flag Clear
    #[inline(always)]
    pub fn rpeclr(&mut self) -> RpeclrW<NmiclrSpec> {
        RpeclrW::new(self, 8)
    }
    ///Bit 9 - SRAM ECC Error Interrupt Status Flag Clear
    #[inline(always)]
    pub fn reccclr(&mut self) -> ReccclrW<NmiclrSpec> {
        ReccclrW::new(self, 9)
    }
    ///Bit 11 - Bus Master MPU Error Interrupt Status Flag Clear
    #[inline(always)]
    pub fn busmclr(&mut self) -> BusmclrW<NmiclrSpec> {
        BusmclrW::new(self, 11)
    }
    ///Bit 13
    #[inline(always)]
    pub fn tzfclr(&mut self) -> TzfclrW<NmiclrSpec> {
        TzfclrW::new(self, 13)
    }
    ///Bit 15
    #[inline(always)]
    pub fn cpeclr(&mut self) -> CpeclrW<NmiclrSpec> {
        CpeclrW::new(self, 15)
    }
}
/**Non-Maskable Interrupt Status Clear Register

You can [`read`](crate::Reg::read) this register and get [`nmiclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmiclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct NmiclrSpec;
impl crate::RegisterSpec for NmiclrSpec {
    type Ux = u16;
}
///`read()` method returns [`nmiclr::R`](R) reader structure
impl crate::Readable for NmiclrSpec {}
///`write(|w| ..)` method takes [`nmiclr::W`](W) writer structure
impl crate::Writable for NmiclrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets NMICLR to value 0
impl crate::Resettable for NmiclrSpec {}
