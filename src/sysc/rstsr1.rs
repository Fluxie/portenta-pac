///Register `RSTSR1` reader
pub type R = crate::R<Rstsr1Spec>;
///Register `RSTSR1` writer
pub type W = crate::W<Rstsr1Spec>;
/**Independent Watchdog Timer Reset Detect Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iwdtrf {
    ///0: Independent watchdog timer reset not detected
    _0 = 0,
    ///1: Independent watchdog timer reset detected
    _1 = 1,
}
impl From<Iwdtrf> for bool {
    #[inline(always)]
    fn from(variant: Iwdtrf) -> Self {
        variant as u8 != 0
    }
}
///Field `IWDTRF` reader - Independent Watchdog Timer Reset Detect Flag
pub type IwdtrfR = crate::BitReader<Iwdtrf>;
impl IwdtrfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Iwdtrf {
        match self.bits {
            false => Iwdtrf::_0,
            true => Iwdtrf::_1,
        }
    }
    ///Independent watchdog timer reset not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Iwdtrf::_0
    }
    ///Independent watchdog timer reset detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Iwdtrf::_1
    }
}
///Field `IWDTRF` writer - Independent Watchdog Timer Reset Detect Flag
pub type IwdtrfW<'a, REG> = crate::BitWriter<'a, REG, Iwdtrf>;
impl<'a, REG> IwdtrfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Independent watchdog timer reset not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Iwdtrf::_0)
    }
    ///Independent watchdog timer reset detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Iwdtrf::_1)
    }
}
/**Watchdog Timer Reset Detect Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wdtrf {
    ///0: Watchdog timer reset not detected
    _0 = 0,
    ///1: Watchdog timer reset detected
    _1 = 1,
}
impl From<Wdtrf> for bool {
    #[inline(always)]
    fn from(variant: Wdtrf) -> Self {
        variant as u8 != 0
    }
}
///Field `WDTRF` reader - Watchdog Timer Reset Detect Flag
pub type WdtrfR = crate::BitReader<Wdtrf>;
impl WdtrfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Wdtrf {
        match self.bits {
            false => Wdtrf::_0,
            true => Wdtrf::_1,
        }
    }
    ///Watchdog timer reset not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wdtrf::_0
    }
    ///Watchdog timer reset detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wdtrf::_1
    }
}
///Field `WDTRF` writer - Watchdog Timer Reset Detect Flag
pub type WdtrfW<'a, REG> = crate::BitWriter<'a, REG, Wdtrf>;
impl<'a, REG> WdtrfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Watchdog timer reset not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtrf::_0)
    }
    ///Watchdog timer reset detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtrf::_1)
    }
}
/**Software Reset Detect Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swrf {
    ///0: Software reset not detected
    _0 = 0,
    ///1: Software reset detected
    _1 = 1,
}
impl From<Swrf> for bool {
    #[inline(always)]
    fn from(variant: Swrf) -> Self {
        variant as u8 != 0
    }
}
///Field `SWRF` reader - Software Reset Detect Flag
pub type SwrfR = crate::BitReader<Swrf>;
impl SwrfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Swrf {
        match self.bits {
            false => Swrf::_0,
            true => Swrf::_1,
        }
    }
    ///Software reset not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Swrf::_0
    }
    ///Software reset detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Swrf::_1
    }
}
///Field `SWRF` writer - Software Reset Detect Flag
pub type SwrfW<'a, REG> = crate::BitWriter<'a, REG, Swrf>;
impl<'a, REG> SwrfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Software reset not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Swrf::_0)
    }
    ///Software reset detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Swrf::_1)
    }
}
/**SRAM Parity Error Reset Detect Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rperf {
    ///0: SRAM parity error reset not detected
    _0 = 0,
    ///1: SRAM parity error reset detected
    _1 = 1,
}
impl From<Rperf> for bool {
    #[inline(always)]
    fn from(variant: Rperf) -> Self {
        variant as u8 != 0
    }
}
///Field `RPERF` reader - SRAM Parity Error Reset Detect Flag
pub type RperfR = crate::BitReader<Rperf>;
impl RperfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rperf {
        match self.bits {
            false => Rperf::_0,
            true => Rperf::_1,
        }
    }
    ///SRAM parity error reset not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rperf::_0
    }
    ///SRAM parity error reset detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rperf::_1
    }
}
///Field `RPERF` writer - SRAM Parity Error Reset Detect Flag
pub type RperfW<'a, REG> = crate::BitWriter<'a, REG, Rperf>;
impl<'a, REG> RperfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SRAM parity error reset not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rperf::_0)
    }
    ///SRAM parity error reset detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rperf::_1)
    }
}
/**SRAM ECC Error Reset Detect Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reerf {
    ///0: SRAM ECC error reset not detected
    _0 = 0,
    ///1: SRAM ECC error reset detected
    _1 = 1,
}
impl From<Reerf> for bool {
    #[inline(always)]
    fn from(variant: Reerf) -> Self {
        variant as u8 != 0
    }
}
///Field `REERF` reader - SRAM ECC Error Reset Detect Flag
pub type ReerfR = crate::BitReader<Reerf>;
impl ReerfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Reerf {
        match self.bits {
            false => Reerf::_0,
            true => Reerf::_1,
        }
    }
    ///SRAM ECC error reset not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Reerf::_0
    }
    ///SRAM ECC error reset detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Reerf::_1
    }
}
///Field `REERF` writer - SRAM ECC Error Reset Detect Flag
pub type ReerfW<'a, REG> = crate::BitWriter<'a, REG, Reerf>;
impl<'a, REG> ReerfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SRAM ECC error reset not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Reerf::_0)
    }
    ///SRAM ECC error reset detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Reerf::_1)
    }
}
/**Bus Master MPU Error Reset Detect Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Busmrf {
    ///0: Bus master MPU error reset not detected
    _0 = 0,
    ///1: Bus master MPU error reset detected
    _1 = 1,
}
impl From<Busmrf> for bool {
    #[inline(always)]
    fn from(variant: Busmrf) -> Self {
        variant as u8 != 0
    }
}
///Field `BUSMRF` reader - Bus Master MPU Error Reset Detect Flag
pub type BusmrfR = crate::BitReader<Busmrf>;
impl BusmrfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Busmrf {
        match self.bits {
            false => Busmrf::_0,
            true => Busmrf::_1,
        }
    }
    ///Bus master MPU error reset not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Busmrf::_0
    }
    ///Bus master MPU error reset detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Busmrf::_1
    }
}
///Field `BUSMRF` writer - Bus Master MPU Error Reset Detect Flag
pub type BusmrfW<'a, REG> = crate::BitWriter<'a, REG, Busmrf>;
impl<'a, REG> BusmrfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Bus master MPU error reset not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Busmrf::_0)
    }
    ///Bus master MPU error reset detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Busmrf::_1)
    }
}
/**TrustZone Error Reset Detect Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tzerf {
    ///0: TrustZone error reset not detected.
    _0 = 0,
    ///1: TrustZone error reset detected.
    _1 = 1,
}
impl From<Tzerf> for bool {
    #[inline(always)]
    fn from(variant: Tzerf) -> Self {
        variant as u8 != 0
    }
}
///Field `TZERF` reader - TrustZone Error Reset Detect Flag
pub type TzerfR = crate::BitReader<Tzerf>;
impl TzerfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tzerf {
        match self.bits {
            false => Tzerf::_0,
            true => Tzerf::_1,
        }
    }
    ///TrustZone error reset not detected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tzerf::_0
    }
    ///TrustZone error reset detected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tzerf::_1
    }
}
///Field `TZERF` writer - TrustZone Error Reset Detect Flag
pub type TzerfW<'a, REG> = crate::BitWriter<'a, REG, Tzerf>;
impl<'a, REG> TzerfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///TrustZone error reset not detected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tzerf::_0)
    }
    ///TrustZone error reset detected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tzerf::_1)
    }
}
/**Cache Parity Error Reset Detect Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cperf {
    ///0: Cache Parity error reset not detected.
    _0 = 0,
    ///1: Cache Parity error reset detected.
    _1 = 1,
}
impl From<Cperf> for bool {
    #[inline(always)]
    fn from(variant: Cperf) -> Self {
        variant as u8 != 0
    }
}
///Field `CPERF` reader - Cache Parity Error Reset Detect Flag
pub type CperfR = crate::BitReader<Cperf>;
impl CperfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cperf {
        match self.bits {
            false => Cperf::_0,
            true => Cperf::_1,
        }
    }
    ///Cache Parity error reset not detected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cperf::_0
    }
    ///Cache Parity error reset detected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cperf::_1
    }
}
///Field `CPERF` writer - Cache Parity Error Reset Detect Flag
pub type CperfW<'a, REG> = crate::BitWriter<'a, REG, Cperf>;
impl<'a, REG> CperfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cache Parity error reset not detected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cperf::_0)
    }
    ///Cache Parity error reset detected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cperf::_1)
    }
}
impl R {
    ///Bit 0 - Independent Watchdog Timer Reset Detect Flag
    #[inline(always)]
    pub fn iwdtrf(&self) -> IwdtrfR {
        IwdtrfR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Watchdog Timer Reset Detect Flag
    #[inline(always)]
    pub fn wdtrf(&self) -> WdtrfR {
        WdtrfR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Software Reset Detect Flag
    #[inline(always)]
    pub fn swrf(&self) -> SwrfR {
        SwrfR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 8 - SRAM Parity Error Reset Detect Flag
    #[inline(always)]
    pub fn rperf(&self) -> RperfR {
        RperfR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - SRAM ECC Error Reset Detect Flag
    #[inline(always)]
    pub fn reerf(&self) -> ReerfR {
        ReerfR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - Bus Master MPU Error Reset Detect Flag
    #[inline(always)]
    pub fn busmrf(&self) -> BusmrfR {
        BusmrfR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 13 - TrustZone Error Reset Detect Flag
    #[inline(always)]
    pub fn tzerf(&self) -> TzerfR {
        TzerfR::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 15 - Cache Parity Error Reset Detect Flag
    #[inline(always)]
    pub fn cperf(&self) -> CperfR {
        CperfR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSTSR1")
            .field("iwdtrf", &self.iwdtrf())
            .field("wdtrf", &self.wdtrf())
            .field("swrf", &self.swrf())
            .field("rperf", &self.rperf())
            .field("reerf", &self.reerf())
            .field("busmrf", &self.busmrf())
            .field("tzerf", &self.tzerf())
            .field("cperf", &self.cperf())
            .finish()
    }
}
impl W {
    ///Bit 0 - Independent Watchdog Timer Reset Detect Flag
    #[inline(always)]
    pub fn iwdtrf(&mut self) -> IwdtrfW<Rstsr1Spec> {
        IwdtrfW::new(self, 0)
    }
    ///Bit 1 - Watchdog Timer Reset Detect Flag
    #[inline(always)]
    pub fn wdtrf(&mut self) -> WdtrfW<Rstsr1Spec> {
        WdtrfW::new(self, 1)
    }
    ///Bit 2 - Software Reset Detect Flag
    #[inline(always)]
    pub fn swrf(&mut self) -> SwrfW<Rstsr1Spec> {
        SwrfW::new(self, 2)
    }
    ///Bit 8 - SRAM Parity Error Reset Detect Flag
    #[inline(always)]
    pub fn rperf(&mut self) -> RperfW<Rstsr1Spec> {
        RperfW::new(self, 8)
    }
    ///Bit 9 - SRAM ECC Error Reset Detect Flag
    #[inline(always)]
    pub fn reerf(&mut self) -> ReerfW<Rstsr1Spec> {
        ReerfW::new(self, 9)
    }
    ///Bit 11 - Bus Master MPU Error Reset Detect Flag
    #[inline(always)]
    pub fn busmrf(&mut self) -> BusmrfW<Rstsr1Spec> {
        BusmrfW::new(self, 11)
    }
    ///Bit 13 - TrustZone Error Reset Detect Flag
    #[inline(always)]
    pub fn tzerf(&mut self) -> TzerfW<Rstsr1Spec> {
        TzerfW::new(self, 13)
    }
    ///Bit 15 - Cache Parity Error Reset Detect Flag
    #[inline(always)]
    pub fn cperf(&mut self) -> CperfW<Rstsr1Spec> {
        CperfW::new(self, 15)
    }
}
/**Reset Status Register 1

You can [`read`](crate::Reg::read) this register and get [`rstsr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstsr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Rstsr1Spec;
impl crate::RegisterSpec for Rstsr1Spec {
    type Ux = u16;
}
///`read()` method returns [`rstsr1::R`](R) reader structure
impl crate::Readable for Rstsr1Spec {}
///`write(|w| ..)` method takes [`rstsr1::W`](W) writer structure
impl crate::Writable for Rstsr1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RSTSR1 to value 0
impl crate::Resettable for Rstsr1Spec {}
