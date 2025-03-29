///Register `DPSIER2` reader
pub type R = crate::R<Dpsier2Spec>;
///Register `DPSIER2` writer
pub type W = crate::W<Dpsier2Spec>;
/**LVD1 Deep Software Standby Cancel Signal Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dlvd1ie {
    ///0: Cancelling Deep Software Standby mode is disabled
    _0 = 0,
    ///1: Cancelling Deep Software Standby mode is enabled
    _1 = 1,
}
impl From<Dlvd1ie> for bool {
    #[inline(always)]
    fn from(variant: Dlvd1ie) -> Self {
        variant as u8 != 0
    }
}
///Field `DLVD1IE` reader - LVD1 Deep Software Standby Cancel Signal Enable
pub type Dlvd1ieR = crate::BitReader<Dlvd1ie>;
impl Dlvd1ieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dlvd1ie {
        match self.bits {
            false => Dlvd1ie::_0,
            true => Dlvd1ie::_1,
        }
    }
    ///Cancelling Deep Software Standby mode is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dlvd1ie::_0
    }
    ///Cancelling Deep Software Standby mode is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dlvd1ie::_1
    }
}
///Field `DLVD1IE` writer - LVD1 Deep Software Standby Cancel Signal Enable
pub type Dlvd1ieW<'a, REG> = crate::BitWriter<'a, REG, Dlvd1ie>;
impl<'a, REG> Dlvd1ieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancelling Deep Software Standby mode is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dlvd1ie::_0)
    }
    ///Cancelling Deep Software Standby mode is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dlvd1ie::_1)
    }
}
/**LVD2 Deep Software Standby Cancel Signal Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dlvd2ie {
    ///0: Cancelling Deep Software Standby mode is disabled
    _0 = 0,
    ///1: Cancelling Deep Software Standby mode is enabled
    _1 = 1,
}
impl From<Dlvd2ie> for bool {
    #[inline(always)]
    fn from(variant: Dlvd2ie) -> Self {
        variant as u8 != 0
    }
}
///Field `DLVD2IE` reader - LVD2 Deep Software Standby Cancel Signal Enable
pub type Dlvd2ieR = crate::BitReader<Dlvd2ie>;
impl Dlvd2ieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dlvd2ie {
        match self.bits {
            false => Dlvd2ie::_0,
            true => Dlvd2ie::_1,
        }
    }
    ///Cancelling Deep Software Standby mode is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dlvd2ie::_0
    }
    ///Cancelling Deep Software Standby mode is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dlvd2ie::_1
    }
}
///Field `DLVD2IE` writer - LVD2 Deep Software Standby Cancel Signal Enable
pub type Dlvd2ieW<'a, REG> = crate::BitWriter<'a, REG, Dlvd2ie>;
impl<'a, REG> Dlvd2ieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancelling Deep Software Standby mode is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dlvd2ie::_0)
    }
    ///Cancelling Deep Software Standby mode is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dlvd2ie::_1)
    }
}
/**RTC Interval interrupt Deep Software Standby Cancel Signal Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Drtciie {
    ///0: Cancelling Deep Software Standby mode is disabled
    _0 = 0,
    ///1: Cancelling Deep Software Standby mode is enabled
    _1 = 1,
}
impl From<Drtciie> for bool {
    #[inline(always)]
    fn from(variant: Drtciie) -> Self {
        variant as u8 != 0
    }
}
///Field `DRTCIIE` reader - RTC Interval interrupt Deep Software Standby Cancel Signal Enable
pub type DrtciieR = crate::BitReader<Drtciie>;
impl DrtciieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Drtciie {
        match self.bits {
            false => Drtciie::_0,
            true => Drtciie::_1,
        }
    }
    ///Cancelling Deep Software Standby mode is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Drtciie::_0
    }
    ///Cancelling Deep Software Standby mode is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Drtciie::_1
    }
}
///Field `DRTCIIE` writer - RTC Interval interrupt Deep Software Standby Cancel Signal Enable
pub type DrtciieW<'a, REG> = crate::BitWriter<'a, REG, Drtciie>;
impl<'a, REG> DrtciieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancelling Deep Software Standby mode is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Drtciie::_0)
    }
    ///Cancelling Deep Software Standby mode is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Drtciie::_1)
    }
}
/**RTC Alarm interrupt Deep Software Standby Cancel Signal Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Drtcaie {
    ///0: Cancelling Deep Software Standby mode is disabled
    _0 = 0,
    ///1: Cancelling Deep Software Standby mode is enabled
    _1 = 1,
}
impl From<Drtcaie> for bool {
    #[inline(always)]
    fn from(variant: Drtcaie) -> Self {
        variant as u8 != 0
    }
}
///Field `DRTCAIE` reader - RTC Alarm interrupt Deep Software Standby Cancel Signal Enable
pub type DrtcaieR = crate::BitReader<Drtcaie>;
impl DrtcaieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Drtcaie {
        match self.bits {
            false => Drtcaie::_0,
            true => Drtcaie::_1,
        }
    }
    ///Cancelling Deep Software Standby mode is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Drtcaie::_0
    }
    ///Cancelling Deep Software Standby mode is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Drtcaie::_1
    }
}
///Field `DRTCAIE` writer - RTC Alarm interrupt Deep Software Standby Cancel Signal Enable
pub type DrtcaieW<'a, REG> = crate::BitWriter<'a, REG, Drtcaie>;
impl<'a, REG> DrtcaieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancelling Deep Software Standby mode is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Drtcaie::_0)
    }
    ///Cancelling Deep Software Standby mode is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Drtcaie::_1)
    }
}
/**NMI Pin Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dnmie {
    ///0: Cancelling Deep Software Standby mode is disabled
    _0 = 0,
    ///1: Cancelling Deep Software Standby mode is enabled
    _1 = 1,
}
impl From<Dnmie> for bool {
    #[inline(always)]
    fn from(variant: Dnmie) -> Self {
        variant as u8 != 0
    }
}
///Field `DNMIE` reader - NMI Pin Enable
pub type DnmieR = crate::BitReader<Dnmie>;
impl DnmieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dnmie {
        match self.bits {
            false => Dnmie::_0,
            true => Dnmie::_1,
        }
    }
    ///Cancelling Deep Software Standby mode is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dnmie::_0
    }
    ///Cancelling Deep Software Standby mode is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dnmie::_1
    }
}
///Field `DNMIE` writer - NMI Pin Enable
pub type DnmieW<'a, REG> = crate::BitWriter<'a, REG, Dnmie>;
impl<'a, REG> DnmieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancelling Deep Software Standby mode is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dnmie::_0)
    }
    ///Cancelling Deep Software Standby mode is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dnmie::_1)
    }
}
impl R {
    ///Bit 0 - LVD1 Deep Software Standby Cancel Signal Enable
    #[inline(always)]
    pub fn dlvd1ie(&self) -> Dlvd1ieR {
        Dlvd1ieR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LVD2 Deep Software Standby Cancel Signal Enable
    #[inline(always)]
    pub fn dlvd2ie(&self) -> Dlvd2ieR {
        Dlvd2ieR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - RTC Interval interrupt Deep Software Standby Cancel Signal Enable
    #[inline(always)]
    pub fn drtciie(&self) -> DrtciieR {
        DrtciieR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - RTC Alarm interrupt Deep Software Standby Cancel Signal Enable
    #[inline(always)]
    pub fn drtcaie(&self) -> DrtcaieR {
        DrtcaieR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - NMI Pin Enable
    #[inline(always)]
    pub fn dnmie(&self) -> DnmieR {
        DnmieR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DPSIER2")
            .field("dlvd1ie", &self.dlvd1ie())
            .field("dlvd2ie", &self.dlvd2ie())
            .field("drtciie", &self.drtciie())
            .field("drtcaie", &self.drtcaie())
            .field("dnmie", &self.dnmie())
            .finish()
    }
}
impl W {
    ///Bit 0 - LVD1 Deep Software Standby Cancel Signal Enable
    #[inline(always)]
    pub fn dlvd1ie(&mut self) -> Dlvd1ieW<Dpsier2Spec> {
        Dlvd1ieW::new(self, 0)
    }
    ///Bit 1 - LVD2 Deep Software Standby Cancel Signal Enable
    #[inline(always)]
    pub fn dlvd2ie(&mut self) -> Dlvd2ieW<Dpsier2Spec> {
        Dlvd2ieW::new(self, 1)
    }
    ///Bit 2 - RTC Interval interrupt Deep Software Standby Cancel Signal Enable
    #[inline(always)]
    pub fn drtciie(&mut self) -> DrtciieW<Dpsier2Spec> {
        DrtciieW::new(self, 2)
    }
    ///Bit 3 - RTC Alarm interrupt Deep Software Standby Cancel Signal Enable
    #[inline(always)]
    pub fn drtcaie(&mut self) -> DrtcaieW<Dpsier2Spec> {
        DrtcaieW::new(self, 3)
    }
    ///Bit 4 - NMI Pin Enable
    #[inline(always)]
    pub fn dnmie(&mut self) -> DnmieW<Dpsier2Spec> {
        DnmieW::new(self, 4)
    }
}
/**Deep Software Standby Interrupt Enable Register 2

You can [`read`](crate::Reg::read) this register and get [`dpsier2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpsier2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Dpsier2Spec;
impl crate::RegisterSpec for Dpsier2Spec {
    type Ux = u8;
}
///`read()` method returns [`dpsier2::R`](R) reader structure
impl crate::Readable for Dpsier2Spec {}
///`write(|w| ..)` method takes [`dpsier2::W`](W) writer structure
impl crate::Writable for Dpsier2Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DPSIER2 to value 0
impl crate::Resettable for Dpsier2Spec {}
