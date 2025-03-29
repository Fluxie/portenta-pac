///Register `GTSECR` reader
pub type R = crate::R<GtsecrSpec>;
///Register `GTSECR` writer
pub type W = crate::W<GtsecrSpec>;
/**GTCCR Register Buffer Operation Simultaneous Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sbdce {
    ///0: Disable simultaneous enabling GTCCR buffer operations
    _0 = 0,
    ///1: Enable GTCCR register buffer operations simultaneously
    _1 = 1,
}
impl From<Sbdce> for bool {
    #[inline(always)]
    fn from(variant: Sbdce) -> Self {
        variant as u8 != 0
    }
}
///Field `SBDCE` reader - GTCCR Register Buffer Operation Simultaneous Enable
pub type SbdceR = crate::BitReader<Sbdce>;
impl SbdceR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sbdce {
        match self.bits {
            false => Sbdce::_0,
            true => Sbdce::_1,
        }
    }
    ///Disable simultaneous enabling GTCCR buffer operations
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sbdce::_0
    }
    ///Enable GTCCR register buffer operations simultaneously
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sbdce::_1
    }
}
///Field `SBDCE` writer - GTCCR Register Buffer Operation Simultaneous Enable
pub type SbdceW<'a, REG> = crate::BitWriter<'a, REG, Sbdce>;
impl<'a, REG> SbdceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable simultaneous enabling GTCCR buffer operations
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sbdce::_0)
    }
    ///Enable GTCCR register buffer operations simultaneously
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sbdce::_1)
    }
}
/**GTPR Register Buffer Operation Simultaneous Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sbdpe {
    ///0: Disable simultaneous enabling GTPR buffer operations
    _0 = 0,
    ///1: Enable GTPR register buffer operations simultaneously
    _1 = 1,
}
impl From<Sbdpe> for bool {
    #[inline(always)]
    fn from(variant: Sbdpe) -> Self {
        variant as u8 != 0
    }
}
///Field `SBDPE` reader - GTPR Register Buffer Operation Simultaneous Enable
pub type SbdpeR = crate::BitReader<Sbdpe>;
impl SbdpeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sbdpe {
        match self.bits {
            false => Sbdpe::_0,
            true => Sbdpe::_1,
        }
    }
    ///Disable simultaneous enabling GTPR buffer operations
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sbdpe::_0
    }
    ///Enable GTPR register buffer operations simultaneously
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sbdpe::_1
    }
}
///Field `SBDPE` writer - GTPR Register Buffer Operation Simultaneous Enable
pub type SbdpeW<'a, REG> = crate::BitWriter<'a, REG, Sbdpe>;
impl<'a, REG> SbdpeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable simultaneous enabling GTPR buffer operations
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sbdpe::_0)
    }
    ///Enable GTPR register buffer operations simultaneously
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sbdpe::_1)
    }
}
/**GTCCR Register Buffer Operation Simultaneous Disable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sbdcd {
    ///0: Disable simultaneous disabling GTCCR buffer operations
    _0 = 0,
    ///1: Disable GTCCR register buffer operations simultaneously
    _1 = 1,
}
impl From<Sbdcd> for bool {
    #[inline(always)]
    fn from(variant: Sbdcd) -> Self {
        variant as u8 != 0
    }
}
///Field `SBDCD` reader - GTCCR Register Buffer Operation Simultaneous Disable
pub type SbdcdR = crate::BitReader<Sbdcd>;
impl SbdcdR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sbdcd {
        match self.bits {
            false => Sbdcd::_0,
            true => Sbdcd::_1,
        }
    }
    ///Disable simultaneous disabling GTCCR buffer operations
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sbdcd::_0
    }
    ///Disable GTCCR register buffer operations simultaneously
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sbdcd::_1
    }
}
///Field `SBDCD` writer - GTCCR Register Buffer Operation Simultaneous Disable
pub type SbdcdW<'a, REG> = crate::BitWriter<'a, REG, Sbdcd>;
impl<'a, REG> SbdcdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable simultaneous disabling GTCCR buffer operations
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sbdcd::_0)
    }
    ///Disable GTCCR register buffer operations simultaneously
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sbdcd::_1)
    }
}
/**GTPR Register Buffer Operation Simultaneous Disable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sbdpd {
    ///0: Disable simultaneous disabling GTPR buffer operations
    _0 = 0,
    ///1: Disable GTPR register buffer operations simultaneously
    _1 = 1,
}
impl From<Sbdpd> for bool {
    #[inline(always)]
    fn from(variant: Sbdpd) -> Self {
        variant as u8 != 0
    }
}
///Field `SBDPD` reader - GTPR Register Buffer Operation Simultaneous Disable
pub type SbdpdR = crate::BitReader<Sbdpd>;
impl SbdpdR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sbdpd {
        match self.bits {
            false => Sbdpd::_0,
            true => Sbdpd::_1,
        }
    }
    ///Disable simultaneous disabling GTPR buffer operations
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sbdpd::_0
    }
    ///Disable GTPR register buffer operations simultaneously
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sbdpd::_1
    }
}
///Field `SBDPD` writer - GTPR Register Buffer Operation Simultaneous Disable
pub type SbdpdW<'a, REG> = crate::BitWriter<'a, REG, Sbdpd>;
impl<'a, REG> SbdpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable simultaneous disabling GTPR buffer operations
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sbdpd::_0)
    }
    ///Disable GTPR register buffer operations simultaneously
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sbdpd::_1)
    }
}
/**Period Count Function Simultaneous Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spce {
    ///0: Disable simultaneous enabling period count function
    _0 = 0,
    ///1: Enable period count function simultaneously
    _1 = 1,
}
impl From<Spce> for bool {
    #[inline(always)]
    fn from(variant: Spce) -> Self {
        variant as u8 != 0
    }
}
///Field `SPCE` reader - Period Count Function Simultaneous Enable
pub type SpceR = crate::BitReader<Spce>;
impl SpceR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Spce {
        match self.bits {
            false => Spce::_0,
            true => Spce::_1,
        }
    }
    ///Disable simultaneous enabling period count function
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Spce::_0
    }
    ///Enable period count function simultaneously
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Spce::_1
    }
}
///Field `SPCE` writer - Period Count Function Simultaneous Enable
pub type SpceW<'a, REG> = crate::BitWriter<'a, REG, Spce>;
impl<'a, REG> SpceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable simultaneous enabling period count function
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Spce::_0)
    }
    ///Enable period count function simultaneously
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Spce::_1)
    }
}
/**Period Count Function Simultaneous Disable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spcd {
    ///0: Disable simultaneous disabling period count function
    _0 = 0,
    ///1: Disable period count function simultaneously
    _1 = 1,
}
impl From<Spcd> for bool {
    #[inline(always)]
    fn from(variant: Spcd) -> Self {
        variant as u8 != 0
    }
}
///Field `SPCD` reader - Period Count Function Simultaneous Disable
pub type SpcdR = crate::BitReader<Spcd>;
impl SpcdR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Spcd {
        match self.bits {
            false => Spcd::_0,
            true => Spcd::_1,
        }
    }
    ///Disable simultaneous disabling period count function
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Spcd::_0
    }
    ///Disable period count function simultaneously
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Spcd::_1
    }
}
///Field `SPCD` writer - Period Count Function Simultaneous Disable
pub type SpcdW<'a, REG> = crate::BitWriter<'a, REG, Spcd>;
impl<'a, REG> SpcdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable simultaneous disabling period count function
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Spcd::_0)
    }
    ///Disable period count function simultaneously
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Spcd::_1)
    }
}
impl R {
    ///Bit 0 - GTCCR Register Buffer Operation Simultaneous Enable
    #[inline(always)]
    pub fn sbdce(&self) -> SbdceR {
        SbdceR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - GTPR Register Buffer Operation Simultaneous Enable
    #[inline(always)]
    pub fn sbdpe(&self) -> SbdpeR {
        SbdpeR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 8 - GTCCR Register Buffer Operation Simultaneous Disable
    #[inline(always)]
    pub fn sbdcd(&self) -> SbdcdR {
        SbdcdR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - GTPR Register Buffer Operation Simultaneous Disable
    #[inline(always)]
    pub fn sbdpd(&self) -> SbdpdR {
        SbdpdR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 16 - Period Count Function Simultaneous Enable
    #[inline(always)]
    pub fn spce(&self) -> SpceR {
        SpceR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 24 - Period Count Function Simultaneous Disable
    #[inline(always)]
    pub fn spcd(&self) -> SpcdR {
        SpcdR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GTSECR")
            .field("sbdce", &self.sbdce())
            .field("sbdpe", &self.sbdpe())
            .field("sbdcd", &self.sbdcd())
            .field("sbdpd", &self.sbdpd())
            .field("spce", &self.spce())
            .field("spcd", &self.spcd())
            .finish()
    }
}
impl W {
    ///Bit 0 - GTCCR Register Buffer Operation Simultaneous Enable
    #[inline(always)]
    pub fn sbdce(&mut self) -> SbdceW<GtsecrSpec> {
        SbdceW::new(self, 0)
    }
    ///Bit 1 - GTPR Register Buffer Operation Simultaneous Enable
    #[inline(always)]
    pub fn sbdpe(&mut self) -> SbdpeW<GtsecrSpec> {
        SbdpeW::new(self, 1)
    }
    ///Bit 8 - GTCCR Register Buffer Operation Simultaneous Disable
    #[inline(always)]
    pub fn sbdcd(&mut self) -> SbdcdW<GtsecrSpec> {
        SbdcdW::new(self, 8)
    }
    ///Bit 9 - GTPR Register Buffer Operation Simultaneous Disable
    #[inline(always)]
    pub fn sbdpd(&mut self) -> SbdpdW<GtsecrSpec> {
        SbdpdW::new(self, 9)
    }
    ///Bit 16 - Period Count Function Simultaneous Enable
    #[inline(always)]
    pub fn spce(&mut self) -> SpceW<GtsecrSpec> {
        SpceW::new(self, 16)
    }
    ///Bit 24 - Period Count Function Simultaneous Disable
    #[inline(always)]
    pub fn spcd(&mut self) -> SpcdW<GtsecrSpec> {
        SpcdW::new(self, 24)
    }
}
/**General PWM Timer Operation Enable Bit Simultaneous Control Register

You can [`read`](crate::Reg::read) this register and get [`gtsecr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtsecr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GtsecrSpec;
impl crate::RegisterSpec for GtsecrSpec {
    type Ux = u32;
}
///`read()` method returns [`gtsecr::R`](R) reader structure
impl crate::Readable for GtsecrSpec {}
///`write(|w| ..)` method takes [`gtsecr::W`](W) writer structure
impl crate::Writable for GtsecrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTSECR to value 0
impl crate::Resettable for GtsecrSpec {}
