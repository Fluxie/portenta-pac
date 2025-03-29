///Register `RSTSR0` reader
pub type R = crate::R<Rstsr0Spec>;
///Register `RSTSR0` writer
pub type W = crate::W<Rstsr0Spec>;
/**Power-On Reset Detect Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Porf {
    ///0: Power-on reset not detected
    _0 = 0,
    ///1: Power-on reset detected
    _1 = 1,
}
impl From<Porf> for bool {
    #[inline(always)]
    fn from(variant: Porf) -> Self {
        variant as u8 != 0
    }
}
///Field `PORF` reader - Power-On Reset Detect Flag
pub type PorfR = crate::BitReader<Porf>;
impl PorfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Porf {
        match self.bits {
            false => Porf::_0,
            true => Porf::_1,
        }
    }
    ///Power-on reset not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Porf::_0
    }
    ///Power-on reset detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Porf::_1
    }
}
///Field `PORF` writer - Power-On Reset Detect Flag
pub type PorfW<'a, REG> = crate::BitWriter<'a, REG, Porf>;
impl<'a, REG> PorfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Power-on reset not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Porf::_0)
    }
    ///Power-on reset detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Porf::_1)
    }
}
/**Voltage Monitor 0 Reset Detect Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lvd0rf {
    ///0: Voltage monitor 0 reset not detected
    _0 = 0,
    ///1: Voltage monitor 0 reset detected
    _1 = 1,
}
impl From<Lvd0rf> for bool {
    #[inline(always)]
    fn from(variant: Lvd0rf) -> Self {
        variant as u8 != 0
    }
}
///Field `LVD0RF` reader - Voltage Monitor 0 Reset Detect Flag
pub type Lvd0rfR = crate::BitReader<Lvd0rf>;
impl Lvd0rfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Lvd0rf {
        match self.bits {
            false => Lvd0rf::_0,
            true => Lvd0rf::_1,
        }
    }
    ///Voltage monitor 0 reset not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lvd0rf::_0
    }
    ///Voltage monitor 0 reset detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lvd0rf::_1
    }
}
///Field `LVD0RF` writer - Voltage Monitor 0 Reset Detect Flag
pub type Lvd0rfW<'a, REG> = crate::BitWriter<'a, REG, Lvd0rf>;
impl<'a, REG> Lvd0rfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Voltage monitor 0 reset not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd0rf::_0)
    }
    ///Voltage monitor 0 reset detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd0rf::_1)
    }
}
/**Voltage Monitor 1 Reset Detect Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lvd1rf {
    ///0: Voltage monitor 1 reset not detected
    _0 = 0,
    ///1: Voltage monitor 1 reset detected
    _1 = 1,
}
impl From<Lvd1rf> for bool {
    #[inline(always)]
    fn from(variant: Lvd1rf) -> Self {
        variant as u8 != 0
    }
}
///Field `LVD1RF` reader - Voltage Monitor 1 Reset Detect Flag
pub type Lvd1rfR = crate::BitReader<Lvd1rf>;
impl Lvd1rfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Lvd1rf {
        match self.bits {
            false => Lvd1rf::_0,
            true => Lvd1rf::_1,
        }
    }
    ///Voltage monitor 1 reset not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lvd1rf::_0
    }
    ///Voltage monitor 1 reset detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lvd1rf::_1
    }
}
///Field `LVD1RF` writer - Voltage Monitor 1 Reset Detect Flag
pub type Lvd1rfW<'a, REG> = crate::BitWriter<'a, REG, Lvd1rf>;
impl<'a, REG> Lvd1rfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Voltage monitor 1 reset not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd1rf::_0)
    }
    ///Voltage monitor 1 reset detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd1rf::_1)
    }
}
/**Voltage Monitor 2 Reset Detect Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lvd2rf {
    ///0: Voltage monitor 2 reset not detected
    _0 = 0,
    ///1: Voltage monitor 2 reset detected
    _1 = 1,
}
impl From<Lvd2rf> for bool {
    #[inline(always)]
    fn from(variant: Lvd2rf) -> Self {
        variant as u8 != 0
    }
}
///Field `LVD2RF` reader - Voltage Monitor 2 Reset Detect Flag
pub type Lvd2rfR = crate::BitReader<Lvd2rf>;
impl Lvd2rfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Lvd2rf {
        match self.bits {
            false => Lvd2rf::_0,
            true => Lvd2rf::_1,
        }
    }
    ///Voltage monitor 2 reset not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lvd2rf::_0
    }
    ///Voltage monitor 2 reset detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lvd2rf::_1
    }
}
///Field `LVD2RF` writer - Voltage Monitor 2 Reset Detect Flag
pub type Lvd2rfW<'a, REG> = crate::BitWriter<'a, REG, Lvd2rf>;
impl<'a, REG> Lvd2rfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Voltage monitor 2 reset not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd2rf::_0)
    }
    ///Voltage monitor 2 reset detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd2rf::_1)
    }
}
/**Deep Software Standby Reset Detect Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dpsrstf {
    ///0: Deep software standby mode cancellation not requested by an interrupt.
    _0 = 0,
    ///1: Deep software standby mode cancellation requested by an interrupt.
    _1 = 1,
}
impl From<Dpsrstf> for bool {
    #[inline(always)]
    fn from(variant: Dpsrstf) -> Self {
        variant as u8 != 0
    }
}
///Field `DPSRSTF` reader - Deep Software Standby Reset Detect Flag
pub type DpsrstfR = crate::BitReader<Dpsrstf>;
impl DpsrstfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dpsrstf {
        match self.bits {
            false => Dpsrstf::_0,
            true => Dpsrstf::_1,
        }
    }
    ///Deep software standby mode cancellation not requested by an interrupt.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dpsrstf::_0
    }
    ///Deep software standby mode cancellation requested by an interrupt.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dpsrstf::_1
    }
}
///Field `DPSRSTF` writer - Deep Software Standby Reset Detect Flag
pub type DpsrstfW<'a, REG> = crate::BitWriter<'a, REG, Dpsrstf>;
impl<'a, REG> DpsrstfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Deep software standby mode cancellation not requested by an interrupt.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dpsrstf::_0)
    }
    ///Deep software standby mode cancellation requested by an interrupt.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dpsrstf::_1)
    }
}
impl R {
    ///Bit 0 - Power-On Reset Detect Flag
    #[inline(always)]
    pub fn porf(&self) -> PorfR {
        PorfR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Voltage Monitor 0 Reset Detect Flag
    #[inline(always)]
    pub fn lvd0rf(&self) -> Lvd0rfR {
        Lvd0rfR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Voltage Monitor 1 Reset Detect Flag
    #[inline(always)]
    pub fn lvd1rf(&self) -> Lvd1rfR {
        Lvd1rfR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Voltage Monitor 2 Reset Detect Flag
    #[inline(always)]
    pub fn lvd2rf(&self) -> Lvd2rfR {
        Lvd2rfR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 7 - Deep Software Standby Reset Detect Flag
    #[inline(always)]
    pub fn dpsrstf(&self) -> DpsrstfR {
        DpsrstfR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSTSR0")
            .field("porf", &self.porf())
            .field("lvd0rf", &self.lvd0rf())
            .field("lvd1rf", &self.lvd1rf())
            .field("lvd2rf", &self.lvd2rf())
            .field("dpsrstf", &self.dpsrstf())
            .finish()
    }
}
impl W {
    ///Bit 0 - Power-On Reset Detect Flag
    #[inline(always)]
    pub fn porf(&mut self) -> PorfW<Rstsr0Spec> {
        PorfW::new(self, 0)
    }
    ///Bit 1 - Voltage Monitor 0 Reset Detect Flag
    #[inline(always)]
    pub fn lvd0rf(&mut self) -> Lvd0rfW<Rstsr0Spec> {
        Lvd0rfW::new(self, 1)
    }
    ///Bit 2 - Voltage Monitor 1 Reset Detect Flag
    #[inline(always)]
    pub fn lvd1rf(&mut self) -> Lvd1rfW<Rstsr0Spec> {
        Lvd1rfW::new(self, 2)
    }
    ///Bit 3 - Voltage Monitor 2 Reset Detect Flag
    #[inline(always)]
    pub fn lvd2rf(&mut self) -> Lvd2rfW<Rstsr0Spec> {
        Lvd2rfW::new(self, 3)
    }
    ///Bit 7 - Deep Software Standby Reset Detect Flag
    #[inline(always)]
    pub fn dpsrstf(&mut self) -> DpsrstfW<Rstsr0Spec> {
        DpsrstfW::new(self, 7)
    }
}
/**Reset Status Register 0

You can [`read`](crate::Reg::read) this register and get [`rstsr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstsr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Rstsr0Spec;
impl crate::RegisterSpec for Rstsr0Spec {
    type Ux = u8;
}
///`read()` method returns [`rstsr0::R`](R) reader structure
impl crate::Readable for Rstsr0Spec {}
///`write(|w| ..)` method takes [`rstsr0::W`](W) writer structure
impl crate::Writable for Rstsr0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RSTSR0 to value 0
impl crate::Resettable for Rstsr0Spec {}
