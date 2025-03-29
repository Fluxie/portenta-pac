///Register `MMPUACEDMAC%s` reader
pub type R = crate::R<MmpuacedmacSpec>;
///Register `MMPUACEDMAC%s` writer
pub type W = crate::W<MmpuacedmacSpec>;
/**Region enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable {
    ///0: EDMAC Region n unit is disabled
    _0 = 0,
    ///1: EDMAC Region n unit is enabled
    _1 = 1,
}
impl From<Enable> for bool {
    #[inline(always)]
    fn from(variant: Enable) -> Self {
        variant as u8 != 0
    }
}
///Field `ENABLE` reader - Region enable
pub type EnableR = crate::BitReader<Enable>;
impl EnableR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Enable {
        match self.bits {
            false => Enable::_0,
            true => Enable::_1,
        }
    }
    ///EDMAC Region n unit is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Enable::_0
    }
    ///EDMAC Region n unit is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Enable::_1
    }
}
///Field `ENABLE` writer - Region enable
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG, Enable>;
impl<'a, REG> EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///EDMAC Region n unit is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::_0)
    }
    ///EDMAC Region n unit is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::_1)
    }
}
/**Read protection

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rp {
    ///0: Read permission
    _0 = 0,
    ///1: Read protection
    _1 = 1,
}
impl From<Rp> for bool {
    #[inline(always)]
    fn from(variant: Rp) -> Self {
        variant as u8 != 0
    }
}
///Field `RP` reader - Read protection
pub type RpR = crate::BitReader<Rp>;
impl RpR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rp {
        match self.bits {
            false => Rp::_0,
            true => Rp::_1,
        }
    }
    ///Read permission
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rp::_0
    }
    ///Read protection
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rp::_1
    }
}
///Field `RP` writer - Read protection
pub type RpW<'a, REG> = crate::BitWriter<'a, REG, Rp>;
impl<'a, REG> RpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Read permission
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rp::_0)
    }
    ///Read protection
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rp::_1)
    }
}
/**Write protection

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp {
    ///0: Write permission
    _0 = 0,
    ///1: Write protection
    _1 = 1,
}
impl From<Wp> for bool {
    #[inline(always)]
    fn from(variant: Wp) -> Self {
        variant as u8 != 0
    }
}
///Field `WP` reader - Write protection
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
    ///Write permission
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wp::_0
    }
    ///Write protection
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wp::_1
    }
}
///Field `WP` writer - Write protection
pub type WpW<'a, REG> = crate::BitWriter<'a, REG, Wp>;
impl<'a, REG> WpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Write permission
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wp::_0)
    }
    ///Write protection
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wp::_1)
    }
}
impl R {
    ///Bit 0 - Region enable
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Read protection
    #[inline(always)]
    pub fn rp(&self) -> RpR {
        RpR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Write protection
    #[inline(always)]
    pub fn wp(&self) -> WpR {
        WpR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMPUACEDMAC")
            .field("enable", &self.enable())
            .field("rp", &self.rp())
            .field("wp", &self.wp())
            .finish()
    }
}
impl W {
    ///Bit 0 - Region enable
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<MmpuacedmacSpec> {
        EnableW::new(self, 0)
    }
    ///Bit 1 - Read protection
    #[inline(always)]
    pub fn rp(&mut self) -> RpW<MmpuacedmacSpec> {
        RpW::new(self, 1)
    }
    ///Bit 2 - Write protection
    #[inline(always)]
    pub fn wp(&mut self) -> WpW<MmpuacedmacSpec> {
        WpW::new(self, 2)
    }
}
/**MMPU Access Control Register for EDMAC

You can [`read`](crate::Reg::read) this register and get [`mmpuacedmac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmpuacedmac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MmpuacedmacSpec;
impl crate::RegisterSpec for MmpuacedmacSpec {
    type Ux = u16;
}
///`read()` method returns [`mmpuacedmac::R`](R) reader structure
impl crate::Readable for MmpuacedmacSpec {}
///`write(|w| ..)` method takes [`mmpuacedmac::W`](W) writer structure
impl crate::Writable for MmpuacedmacSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MMPUACEDMAC%s to value 0
impl crate::Resettable for MmpuacedmacSpec {}
