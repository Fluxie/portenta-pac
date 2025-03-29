///Register `DPUSRCR` reader
pub type R = crate::R<DpusrcrSpec>;
///Register `DPUSRCR` writer
pub type W = crate::W<DpusrcrSpec>;
/**USB Transceiver Control Fix

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fixphy {
    ///0: Normal mode
    _0 = 0,
    ///1: Invoke/recover from Deep Software Standby mode
    _1 = 1,
}
impl From<Fixphy> for bool {
    #[inline(always)]
    fn from(variant: Fixphy) -> Self {
        variant as u8 != 0
    }
}
///Field `FIXPHY` reader - USB Transceiver Control Fix
pub type FixphyR = crate::BitReader<Fixphy>;
impl FixphyR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Fixphy {
        match self.bits {
            false => Fixphy::_0,
            true => Fixphy::_1,
        }
    }
    ///Normal mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Fixphy::_0
    }
    ///Invoke/recover from Deep Software Standby mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Fixphy::_1
    }
}
///Field `FIXPHY` writer - USB Transceiver Control Fix
pub type FixphyW<'a, REG> = crate::BitWriter<'a, REG, Fixphy>;
impl<'a, REG> FixphyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Normal mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Fixphy::_0)
    }
    ///Invoke/recover from Deep Software Standby mode
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Fixphy::_1)
    }
}
/**USB Transceiver Control Fix for PLL

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fixphypd {
    ///0: Normal mode
    _0 = 0,
    ///1: Invoke/recover from Deep Software Standby mode
    _1 = 1,
}
impl From<Fixphypd> for bool {
    #[inline(always)]
    fn from(variant: Fixphypd) -> Self {
        variant as u8 != 0
    }
}
///Field `FIXPHYPD` reader - USB Transceiver Control Fix for PLL
pub type FixphypdR = crate::BitReader<Fixphypd>;
impl FixphypdR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Fixphypd {
        match self.bits {
            false => Fixphypd::_0,
            true => Fixphypd::_1,
        }
    }
    ///Normal mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Fixphypd::_0
    }
    ///Invoke/recover from Deep Software Standby mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Fixphypd::_1
    }
}
///Field `FIXPHYPD` writer - USB Transceiver Control Fix for PLL
pub type FixphypdW<'a, REG> = crate::BitWriter<'a, REG, Fixphypd>;
impl<'a, REG> FixphypdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Normal mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Fixphypd::_0)
    }
    ///Invoke/recover from Deep Software Standby mode
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Fixphypd::_1)
    }
}
impl R {
    ///Bit 0 - USB Transceiver Control Fix
    #[inline(always)]
    pub fn fixphy(&self) -> FixphyR {
        FixphyR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - USB Transceiver Control Fix for PLL
    #[inline(always)]
    pub fn fixphypd(&self) -> FixphypdR {
        FixphypdR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DPUSRCR")
            .field("fixphy", &self.fixphy())
            .field("fixphypd", &self.fixphypd())
            .finish()
    }
}
impl W {
    ///Bit 0 - USB Transceiver Control Fix
    #[inline(always)]
    pub fn fixphy(&mut self) -> FixphyW<DpusrcrSpec> {
        FixphyW::new(self, 0)
    }
    ///Bit 1 - USB Transceiver Control Fix for PLL
    #[inline(always)]
    pub fn fixphypd(&mut self) -> FixphypdW<DpusrcrSpec> {
        FixphypdW::new(self, 1)
    }
}
/**Deep Software Standby USB Suspend/Resume Command Register

You can [`read`](crate::Reg::read) this register and get [`dpusrcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpusrcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DpusrcrSpec;
impl crate::RegisterSpec for DpusrcrSpec {
    type Ux = u16;
}
///`read()` method returns [`dpusrcr::R`](R) reader structure
impl crate::Readable for DpusrcrSpec {}
///`write(|w| ..)` method takes [`dpusrcr::W`](W) writer structure
impl crate::Writable for DpusrcrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DPUSRCR to value 0
impl crate::Resettable for DpusrcrSpec {}
