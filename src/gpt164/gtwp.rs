///Register `GTWP` reader
pub type R = crate::R<GtwpSpec>;
///Register `GTWP` writer
pub type W = crate::W<GtwpSpec>;
/**Register Write Disable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp {
    ///0: Write to the register enabled
    _0 = 0,
    ///1: Write to the register disabled
    _1 = 1,
}
impl From<Wp> for bool {
    #[inline(always)]
    fn from(variant: Wp) -> Self {
        variant as u8 != 0
    }
}
///Field `WP` reader - Register Write Disable
pub type WpR = crate::BitReader<Wp>;
impl WpR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Wp {
        match self.bits {
            false => Wp::_0,
            true => Wp::_1,
        }
    }
    ///Write to the register enabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wp::_0
    }
    ///Write to the register disabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wp::_1
    }
}
///Field `WP` writer - Register Write Disable
pub type WpW<'a, REG> = crate::BitWriter<'a, REG, Wp>;
impl<'a, REG> WpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Write to the register enabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wp::_0)
    }
    ///Write to the register disabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wp::_1)
    }
}
/**GTSTR.CSTRT Bit Write Disable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Strwp {
    ///0: Write to the bit is enabled
    _0 = 0,
    ///1: Write to the bit is disabled
    _1 = 1,
}
impl From<Strwp> for bool {
    #[inline(always)]
    fn from(variant: Strwp) -> Self {
        variant as u8 != 0
    }
}
///Field `STRWP` reader - GTSTR.CSTRT Bit Write Disable
pub type StrwpR = crate::BitReader<Strwp>;
impl StrwpR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Strwp {
        match self.bits {
            false => Strwp::_0,
            true => Strwp::_1,
        }
    }
    ///Write to the bit is enabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Strwp::_0
    }
    ///Write to the bit is disabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Strwp::_1
    }
}
///Field `STRWP` writer - GTSTR.CSTRT Bit Write Disable
pub type StrwpW<'a, REG> = crate::BitWriter<'a, REG, Strwp>;
impl<'a, REG> StrwpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Write to the bit is enabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Strwp::_0)
    }
    ///Write to the bit is disabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Strwp::_1)
    }
}
/**GTSTP.CSTOP Bit Write Disable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stpwp {
    ///0: Write to the bit is enabled
    _0 = 0,
    ///1: Write to the bit is disabled
    _1 = 1,
}
impl From<Stpwp> for bool {
    #[inline(always)]
    fn from(variant: Stpwp) -> Self {
        variant as u8 != 0
    }
}
///Field `STPWP` reader - GTSTP.CSTOP Bit Write Disable
pub type StpwpR = crate::BitReader<Stpwp>;
impl StpwpR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Stpwp {
        match self.bits {
            false => Stpwp::_0,
            true => Stpwp::_1,
        }
    }
    ///Write to the bit is enabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Stpwp::_0
    }
    ///Write to the bit is disabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Stpwp::_1
    }
}
///Field `STPWP` writer - GTSTP.CSTOP Bit Write Disable
pub type StpwpW<'a, REG> = crate::BitWriter<'a, REG, Stpwp>;
impl<'a, REG> StpwpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Write to the bit is enabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Stpwp::_0)
    }
    ///Write to the bit is disabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Stpwp::_1)
    }
}
/**GTCLR.CCLR Bit Write Disable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clrwp {
    ///0: Write to the bit is enabled
    _0 = 0,
    ///1: Write to the bit is disabled
    _1 = 1,
}
impl From<Clrwp> for bool {
    #[inline(always)]
    fn from(variant: Clrwp) -> Self {
        variant as u8 != 0
    }
}
///Field `CLRWP` reader - GTCLR.CCLR Bit Write Disable
pub type ClrwpR = crate::BitReader<Clrwp>;
impl ClrwpR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Clrwp {
        match self.bits {
            false => Clrwp::_0,
            true => Clrwp::_1,
        }
    }
    ///Write to the bit is enabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Clrwp::_0
    }
    ///Write to the bit is disabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Clrwp::_1
    }
}
///Field `CLRWP` writer - GTCLR.CCLR Bit Write Disable
pub type ClrwpW<'a, REG> = crate::BitWriter<'a, REG, Clrwp>;
impl<'a, REG> ClrwpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Write to the bit is enabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Clrwp::_0)
    }
    ///Write to the bit is disabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Clrwp::_1)
    }
}
/**Common Register Write Disabled

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmnwp {
    ///0: Write to the register is enabled
    _0 = 0,
    ///1: Write to the register is disabled
    _1 = 1,
}
impl From<Cmnwp> for bool {
    #[inline(always)]
    fn from(variant: Cmnwp) -> Self {
        variant as u8 != 0
    }
}
///Field `CMNWP` reader - Common Register Write Disabled
pub type CmnwpR = crate::BitReader<Cmnwp>;
impl CmnwpR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmnwp {
        match self.bits {
            false => Cmnwp::_0,
            true => Cmnwp::_1,
        }
    }
    ///Write to the register is enabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmnwp::_0
    }
    ///Write to the register is disabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmnwp::_1
    }
}
///Field `CMNWP` writer - Common Register Write Disabled
pub type CmnwpW<'a, REG> = crate::BitWriter<'a, REG, Cmnwp>;
impl<'a, REG> CmnwpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Write to the register is enabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmnwp::_0)
    }
    ///Write to the register is disabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmnwp::_1)
    }
}
///Field `PRKEY` writer - GTWP Key Code
pub type PrkeyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bit 0 - Register Write Disable
    #[inline(always)]
    pub fn wp(&self) -> WpR {
        WpR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - GTSTR.CSTRT Bit Write Disable
    #[inline(always)]
    pub fn strwp(&self) -> StrwpR {
        StrwpR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - GTSTP.CSTOP Bit Write Disable
    #[inline(always)]
    pub fn stpwp(&self) -> StpwpR {
        StpwpR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - GTCLR.CCLR Bit Write Disable
    #[inline(always)]
    pub fn clrwp(&self) -> ClrwpR {
        ClrwpR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Common Register Write Disabled
    #[inline(always)]
    pub fn cmnwp(&self) -> CmnwpR {
        CmnwpR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GTWP")
            .field("wp", &self.wp())
            .field("strwp", &self.strwp())
            .field("stpwp", &self.stpwp())
            .field("clrwp", &self.clrwp())
            .field("cmnwp", &self.cmnwp())
            .finish()
    }
}
impl W {
    ///Bit 0 - Register Write Disable
    #[inline(always)]
    pub fn wp(&mut self) -> WpW<GtwpSpec> {
        WpW::new(self, 0)
    }
    ///Bit 1 - GTSTR.CSTRT Bit Write Disable
    #[inline(always)]
    pub fn strwp(&mut self) -> StrwpW<GtwpSpec> {
        StrwpW::new(self, 1)
    }
    ///Bit 2 - GTSTP.CSTOP Bit Write Disable
    #[inline(always)]
    pub fn stpwp(&mut self) -> StpwpW<GtwpSpec> {
        StpwpW::new(self, 2)
    }
    ///Bit 3 - GTCLR.CCLR Bit Write Disable
    #[inline(always)]
    pub fn clrwp(&mut self) -> ClrwpW<GtwpSpec> {
        ClrwpW::new(self, 3)
    }
    ///Bit 4 - Common Register Write Disabled
    #[inline(always)]
    pub fn cmnwp(&mut self) -> CmnwpW<GtwpSpec> {
        CmnwpW::new(self, 4)
    }
    ///Bits 8:15 - GTWP Key Code
    #[inline(always)]
    pub fn prkey(&mut self) -> PrkeyW<GtwpSpec> {
        PrkeyW::new(self, 8)
    }
}
/**General PWM Timer Write-Protection Register

You can [`read`](crate::Reg::read) this register and get [`gtwp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtwp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GtwpSpec;
impl crate::RegisterSpec for GtwpSpec {
    type Ux = u32;
}
///`read()` method returns [`gtwp::R`](R) reader structure
impl crate::Readable for GtwpSpec {}
///`write(|w| ..)` method takes [`gtwp::W`](W) writer structure
impl crate::Writable for GtwpSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTWP to value 0
impl crate::Resettable for GtwpSpec {}
