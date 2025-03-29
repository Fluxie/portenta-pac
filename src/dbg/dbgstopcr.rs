///Register `DBGSTOPCR` reader
pub type R = crate::R<DbgstopcrSpec>;
///Register `DBGSTOPCR` writer
pub type W = crate::W<DbgstopcrSpec>;
/**Mask bit for IWDT reset/interrupt in the OCD run mode

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgstopIwdt {
    ///0: Enable IWDT reset/interrupt
    _0 = 0,
    ///1: Mask IWDT reset/interrupt and stop IWDT counter
    _1 = 1,
}
impl From<DbgstopIwdt> for bool {
    #[inline(always)]
    fn from(variant: DbgstopIwdt) -> Self {
        variant as u8 != 0
    }
}
///Field `DBGSTOP_IWDT` reader - Mask bit for IWDT reset/interrupt in the OCD run mode
pub type DbgstopIwdtR = crate::BitReader<DbgstopIwdt>;
impl DbgstopIwdtR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DbgstopIwdt {
        match self.bits {
            false => DbgstopIwdt::_0,
            true => DbgstopIwdt::_1,
        }
    }
    ///Enable IWDT reset/interrupt
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DbgstopIwdt::_0
    }
    ///Mask IWDT reset/interrupt and stop IWDT counter
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DbgstopIwdt::_1
    }
}
///Field `DBGSTOP_IWDT` writer - Mask bit for IWDT reset/interrupt in the OCD run mode
pub type DbgstopIwdtW<'a, REG> = crate::BitWriter<'a, REG, DbgstopIwdt>;
impl<'a, REG> DbgstopIwdtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Enable IWDT reset/interrupt
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DbgstopIwdt::_0)
    }
    ///Mask IWDT reset/interrupt and stop IWDT counter
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DbgstopIwdt::_1)
    }
}
/**Mask bit for WDT reset/interrupt in the OCD run mode

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgstopWdt {
    ///0: Enable WDT reset/interrupt
    _0 = 0,
    ///1: Mask WDT reset/interrupt and stop WDT counter
    _1 = 1,
}
impl From<DbgstopWdt> for bool {
    #[inline(always)]
    fn from(variant: DbgstopWdt) -> Self {
        variant as u8 != 0
    }
}
///Field `DBGSTOP_WDT` reader - Mask bit for WDT reset/interrupt in the OCD run mode
pub type DbgstopWdtR = crate::BitReader<DbgstopWdt>;
impl DbgstopWdtR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DbgstopWdt {
        match self.bits {
            false => DbgstopWdt::_0,
            true => DbgstopWdt::_1,
        }
    }
    ///Enable WDT reset/interrupt
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DbgstopWdt::_0
    }
    ///Mask WDT reset/interrupt and stop WDT counter
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DbgstopWdt::_1
    }
}
///Field `DBGSTOP_WDT` writer - Mask bit for WDT reset/interrupt in the OCD run mode
pub type DbgstopWdtW<'a, REG> = crate::BitWriter<'a, REG, DbgstopWdt>;
impl<'a, REG> DbgstopWdtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Enable WDT reset/interrupt
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DbgstopWdt::_0)
    }
    ///Mask WDT reset/interrupt and stop WDT counter
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DbgstopWdt::_1)
    }
}
/**Mask bit for LVD0 reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgstopLvd0 {
    ///0: Enable LVD0 reset
    _0 = 0,
    ///1: Mask LVD0 reset
    _1 = 1,
}
impl From<DbgstopLvd0> for bool {
    #[inline(always)]
    fn from(variant: DbgstopLvd0) -> Self {
        variant as u8 != 0
    }
}
///Field `DBGSTOP_LVD0` reader - Mask bit for LVD0 reset
pub type DbgstopLvd0R = crate::BitReader<DbgstopLvd0>;
impl DbgstopLvd0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DbgstopLvd0 {
        match self.bits {
            false => DbgstopLvd0::_0,
            true => DbgstopLvd0::_1,
        }
    }
    ///Enable LVD0 reset
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DbgstopLvd0::_0
    }
    ///Mask LVD0 reset
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DbgstopLvd0::_1
    }
}
///Field `DBGSTOP_LVD0` writer - Mask bit for LVD0 reset
pub type DbgstopLvd0W<'a, REG> = crate::BitWriter<'a, REG, DbgstopLvd0>;
impl<'a, REG> DbgstopLvd0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Enable LVD0 reset
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DbgstopLvd0::_0)
    }
    ///Mask LVD0 reset
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DbgstopLvd0::_1)
    }
}
/**Mask bit for LVD1 reset/interrupt

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgstopLvd1 {
    ///0: Enable LVD1 reset/interrupt
    _0 = 0,
    ///1: Mask LVD1 reset/interrupt
    _1 = 1,
}
impl From<DbgstopLvd1> for bool {
    #[inline(always)]
    fn from(variant: DbgstopLvd1) -> Self {
        variant as u8 != 0
    }
}
///Field `DBGSTOP_LVD1` reader - Mask bit for LVD1 reset/interrupt
pub type DbgstopLvd1R = crate::BitReader<DbgstopLvd1>;
impl DbgstopLvd1R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DbgstopLvd1 {
        match self.bits {
            false => DbgstopLvd1::_0,
            true => DbgstopLvd1::_1,
        }
    }
    ///Enable LVD1 reset/interrupt
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DbgstopLvd1::_0
    }
    ///Mask LVD1 reset/interrupt
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DbgstopLvd1::_1
    }
}
///Field `DBGSTOP_LVD1` writer - Mask bit for LVD1 reset/interrupt
pub type DbgstopLvd1W<'a, REG> = crate::BitWriter<'a, REG, DbgstopLvd1>;
impl<'a, REG> DbgstopLvd1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Enable LVD1 reset/interrupt
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DbgstopLvd1::_0)
    }
    ///Mask LVD1 reset/interrupt
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DbgstopLvd1::_1)
    }
}
/**Mask bit for LVD2 reset/interrupt

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgstopLvd2 {
    ///0: Enable LVD2 reset/interrupt
    _0 = 0,
    ///1: Mask LVD2 reset/interrupt
    _1 = 1,
}
impl From<DbgstopLvd2> for bool {
    #[inline(always)]
    fn from(variant: DbgstopLvd2) -> Self {
        variant as u8 != 0
    }
}
///Field `DBGSTOP_LVD2` reader - Mask bit for LVD2 reset/interrupt
pub type DbgstopLvd2R = crate::BitReader<DbgstopLvd2>;
impl DbgstopLvd2R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DbgstopLvd2 {
        match self.bits {
            false => DbgstopLvd2::_0,
            true => DbgstopLvd2::_1,
        }
    }
    ///Enable LVD2 reset/interrupt
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DbgstopLvd2::_0
    }
    ///Mask LVD2 reset/interrupt
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DbgstopLvd2::_1
    }
}
///Field `DBGSTOP_LVD2` writer - Mask bit for LVD2 reset/interrupt
pub type DbgstopLvd2W<'a, REG> = crate::BitWriter<'a, REG, DbgstopLvd2>;
impl<'a, REG> DbgstopLvd2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Enable LVD2 reset/interrupt
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DbgstopLvd2::_0)
    }
    ///Mask LVD2 reset/interrupt
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DbgstopLvd2::_1)
    }
}
/**Mask bit for SRAM parity error reset/interrupt

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgstopRper {
    ///0: Enable SRAM parity error reset/interrupt
    _0 = 0,
    ///1: Mask SRAM parity error reset/interrupt
    _1 = 1,
}
impl From<DbgstopRper> for bool {
    #[inline(always)]
    fn from(variant: DbgstopRper) -> Self {
        variant as u8 != 0
    }
}
///Field `DBGSTOP_RPER` reader - Mask bit for SRAM parity error reset/interrupt
pub type DbgstopRperR = crate::BitReader<DbgstopRper>;
impl DbgstopRperR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DbgstopRper {
        match self.bits {
            false => DbgstopRper::_0,
            true => DbgstopRper::_1,
        }
    }
    ///Enable SRAM parity error reset/interrupt
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DbgstopRper::_0
    }
    ///Mask SRAM parity error reset/interrupt
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DbgstopRper::_1
    }
}
///Field `DBGSTOP_RPER` writer - Mask bit for SRAM parity error reset/interrupt
pub type DbgstopRperW<'a, REG> = crate::BitWriter<'a, REG, DbgstopRper>;
impl<'a, REG> DbgstopRperW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Enable SRAM parity error reset/interrupt
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DbgstopRper::_0)
    }
    ///Mask SRAM parity error reset/interrupt
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DbgstopRper::_1)
    }
}
/**Mask bit for SRAM ECC error reset/interrupt

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgstopReccr {
    ///0: Enable SRAM ECC error reset/interrupt
    _0 = 0,
    ///1: Mask SRAM ECC error reset/interrupt
    _1 = 1,
}
impl From<DbgstopReccr> for bool {
    #[inline(always)]
    fn from(variant: DbgstopReccr) -> Self {
        variant as u8 != 0
    }
}
///Field `DBGSTOP_RECCR` reader - Mask bit for SRAM ECC error reset/interrupt
pub type DbgstopReccrR = crate::BitReader<DbgstopReccr>;
impl DbgstopReccrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DbgstopReccr {
        match self.bits {
            false => DbgstopReccr::_0,
            true => DbgstopReccr::_1,
        }
    }
    ///Enable SRAM ECC error reset/interrupt
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DbgstopReccr::_0
    }
    ///Mask SRAM ECC error reset/interrupt
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DbgstopReccr::_1
    }
}
///Field `DBGSTOP_RECCR` writer - Mask bit for SRAM ECC error reset/interrupt
pub type DbgstopReccrW<'a, REG> = crate::BitWriter<'a, REG, DbgstopReccr>;
impl<'a, REG> DbgstopReccrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Enable SRAM ECC error reset/interrupt
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DbgstopReccr::_0)
    }
    ///Mask SRAM ECC error reset/interrupt
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DbgstopReccr::_1)
    }
}
/**Mask bit for Cache SRAM parity error reset/interrupt

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgstopCper {
    ///0: Enable Cache SRAM parity error reset/interrupt
    _0 = 0,
    ///1: Mask Cache SRAM parity error reset/interrupt
    _1 = 1,
}
impl From<DbgstopCper> for bool {
    #[inline(always)]
    fn from(variant: DbgstopCper) -> Self {
        variant as u8 != 0
    }
}
///Field `DBGSTOP_CPER` reader - Mask bit for Cache SRAM parity error reset/interrupt
pub type DbgstopCperR = crate::BitReader<DbgstopCper>;
impl DbgstopCperR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DbgstopCper {
        match self.bits {
            false => DbgstopCper::_0,
            true => DbgstopCper::_1,
        }
    }
    ///Enable Cache SRAM parity error reset/interrupt
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DbgstopCper::_0
    }
    ///Mask Cache SRAM parity error reset/interrupt
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DbgstopCper::_1
    }
}
///Field `DBGSTOP_CPER` writer - Mask bit for Cache SRAM parity error reset/interrupt
pub type DbgstopCperW<'a, REG> = crate::BitWriter<'a, REG, DbgstopCper>;
impl<'a, REG> DbgstopCperW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Enable Cache SRAM parity error reset/interrupt
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DbgstopCper::_0)
    }
    ///Mask Cache SRAM parity error reset/interrupt
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DbgstopCper::_1)
    }
}
impl R {
    ///Bit 0 - Mask bit for IWDT reset/interrupt in the OCD run mode
    #[inline(always)]
    pub fn dbgstop_iwdt(&self) -> DbgstopIwdtR {
        DbgstopIwdtR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Mask bit for WDT reset/interrupt in the OCD run mode
    #[inline(always)]
    pub fn dbgstop_wdt(&self) -> DbgstopWdtR {
        DbgstopWdtR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 16 - Mask bit for LVD0 reset
    #[inline(always)]
    pub fn dbgstop_lvd0(&self) -> DbgstopLvd0R {
        DbgstopLvd0R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Mask bit for LVD1 reset/interrupt
    #[inline(always)]
    pub fn dbgstop_lvd1(&self) -> DbgstopLvd1R {
        DbgstopLvd1R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Mask bit for LVD2 reset/interrupt
    #[inline(always)]
    pub fn dbgstop_lvd2(&self) -> DbgstopLvd2R {
        DbgstopLvd2R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 24 - Mask bit for SRAM parity error reset/interrupt
    #[inline(always)]
    pub fn dbgstop_rper(&self) -> DbgstopRperR {
        DbgstopRperR::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Mask bit for SRAM ECC error reset/interrupt
    #[inline(always)]
    pub fn dbgstop_reccr(&self) -> DbgstopReccrR {
        DbgstopReccrR::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 31 - Mask bit for Cache SRAM parity error reset/interrupt
    #[inline(always)]
    pub fn dbgstop_cper(&self) -> DbgstopCperR {
        DbgstopCperR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBGSTOPCR")
            .field("dbgstop_iwdt", &self.dbgstop_iwdt())
            .field("dbgstop_wdt", &self.dbgstop_wdt())
            .field("dbgstop_lvd0", &self.dbgstop_lvd0())
            .field("dbgstop_lvd1", &self.dbgstop_lvd1())
            .field("dbgstop_lvd2", &self.dbgstop_lvd2())
            .field("dbgstop_rper", &self.dbgstop_rper())
            .field("dbgstop_reccr", &self.dbgstop_reccr())
            .field("dbgstop_cper", &self.dbgstop_cper())
            .finish()
    }
}
impl W {
    ///Bit 0 - Mask bit for IWDT reset/interrupt in the OCD run mode
    #[inline(always)]
    pub fn dbgstop_iwdt(&mut self) -> DbgstopIwdtW<DbgstopcrSpec> {
        DbgstopIwdtW::new(self, 0)
    }
    ///Bit 1 - Mask bit for WDT reset/interrupt in the OCD run mode
    #[inline(always)]
    pub fn dbgstop_wdt(&mut self) -> DbgstopWdtW<DbgstopcrSpec> {
        DbgstopWdtW::new(self, 1)
    }
    ///Bit 16 - Mask bit for LVD0 reset
    #[inline(always)]
    pub fn dbgstop_lvd0(&mut self) -> DbgstopLvd0W<DbgstopcrSpec> {
        DbgstopLvd0W::new(self, 16)
    }
    ///Bit 17 - Mask bit for LVD1 reset/interrupt
    #[inline(always)]
    pub fn dbgstop_lvd1(&mut self) -> DbgstopLvd1W<DbgstopcrSpec> {
        DbgstopLvd1W::new(self, 17)
    }
    ///Bit 18 - Mask bit for LVD2 reset/interrupt
    #[inline(always)]
    pub fn dbgstop_lvd2(&mut self) -> DbgstopLvd2W<DbgstopcrSpec> {
        DbgstopLvd2W::new(self, 18)
    }
    ///Bit 24 - Mask bit for SRAM parity error reset/interrupt
    #[inline(always)]
    pub fn dbgstop_rper(&mut self) -> DbgstopRperW<DbgstopcrSpec> {
        DbgstopRperW::new(self, 24)
    }
    ///Bit 25 - Mask bit for SRAM ECC error reset/interrupt
    #[inline(always)]
    pub fn dbgstop_reccr(&mut self) -> DbgstopReccrW<DbgstopcrSpec> {
        DbgstopReccrW::new(self, 25)
    }
    ///Bit 31 - Mask bit for Cache SRAM parity error reset/interrupt
    #[inline(always)]
    pub fn dbgstop_cper(&mut self) -> DbgstopCperW<DbgstopcrSpec> {
        DbgstopCperW::new(self, 31)
    }
}
/**Debug Stop Control Register

You can [`read`](crate::Reg::read) this register and get [`dbgstopcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgstopcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DbgstopcrSpec;
impl crate::RegisterSpec for DbgstopcrSpec {
    type Ux = u32;
}
///`read()` method returns [`dbgstopcr::R`](R) reader structure
impl crate::Readable for DbgstopcrSpec {}
///`write(|w| ..)` method takes [`dbgstopcr::W`](W) writer structure
impl crate::Writable for DbgstopcrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DBGSTOPCR to value 0x03
impl crate::Resettable for DbgstopcrSpec {
    const RESET_VALUE: u32 = 0x03;
}
