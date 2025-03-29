///Register `SNZCR` reader
pub type R = crate::R<SnzcrSpec>;
///Register `SNZCR` writer
pub type W = crate::W<SnzcrSpec>;
/**RXD0 Snooze Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxdreqen {
    ///0: Ignore RXD0 falling edge in Software Standby mode
    _0 = 0,
    ///1: Detect RXD0 falling edge in Software Standby mode
    _1 = 1,
}
impl From<Rxdreqen> for bool {
    #[inline(always)]
    fn from(variant: Rxdreqen) -> Self {
        variant as u8 != 0
    }
}
///Field `RXDREQEN` reader - RXD0 Snooze Request Enable
pub type RxdreqenR = crate::BitReader<Rxdreqen>;
impl RxdreqenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rxdreqen {
        match self.bits {
            false => Rxdreqen::_0,
            true => Rxdreqen::_1,
        }
    }
    ///Ignore RXD0 falling edge in Software Standby mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rxdreqen::_0
    }
    ///Detect RXD0 falling edge in Software Standby mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rxdreqen::_1
    }
}
///Field `RXDREQEN` writer - RXD0 Snooze Request Enable
pub type RxdreqenW<'a, REG> = crate::BitWriter<'a, REG, Rxdreqen>;
impl<'a, REG> RxdreqenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Ignore RXD0 falling edge in Software Standby mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rxdreqen::_0)
    }
    ///Detect RXD0 falling edge in Software Standby mode
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxdreqen::_1)
    }
}
/**DTC Enable in Snooze mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snzdtcen {
    ///0: Disable DTC operation
    _0 = 0,
    ///1: Enable DTC operation
    _1 = 1,
}
impl From<Snzdtcen> for bool {
    #[inline(always)]
    fn from(variant: Snzdtcen) -> Self {
        variant as u8 != 0
    }
}
///Field `SNZDTCEN` reader - DTC Enable in Snooze mode
pub type SnzdtcenR = crate::BitReader<Snzdtcen>;
impl SnzdtcenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Snzdtcen {
        match self.bits {
            false => Snzdtcen::_0,
            true => Snzdtcen::_1,
        }
    }
    ///Disable DTC operation
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Snzdtcen::_0
    }
    ///Enable DTC operation
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Snzdtcen::_1
    }
}
///Field `SNZDTCEN` writer - DTC Enable in Snooze mode
pub type SnzdtcenW<'a, REG> = crate::BitWriter<'a, REG, Snzdtcen>;
impl<'a, REG> SnzdtcenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable DTC operation
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Snzdtcen::_0)
    }
    ///Enable DTC operation
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Snzdtcen::_1)
    }
}
/**Snooze mode Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snze {
    ///0: Disable Snooze mode
    _0 = 0,
    ///1: Enable Snooze mode
    _1 = 1,
}
impl From<Snze> for bool {
    #[inline(always)]
    fn from(variant: Snze) -> Self {
        variant as u8 != 0
    }
}
///Field `SNZE` reader - Snooze mode Enable
pub type SnzeR = crate::BitReader<Snze>;
impl SnzeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Snze {
        match self.bits {
            false => Snze::_0,
            true => Snze::_1,
        }
    }
    ///Disable Snooze mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Snze::_0
    }
    ///Enable Snooze mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Snze::_1
    }
}
///Field `SNZE` writer - Snooze mode Enable
pub type SnzeW<'a, REG> = crate::BitWriter<'a, REG, Snze>;
impl<'a, REG> SnzeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable Snooze mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Snze::_0)
    }
    ///Enable Snooze mode
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Snze::_1)
    }
}
impl R {
    ///Bit 0 - RXD0 Snooze Request Enable
    #[inline(always)]
    pub fn rxdreqen(&self) -> RxdreqenR {
        RxdreqenR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DTC Enable in Snooze mode
    #[inline(always)]
    pub fn snzdtcen(&self) -> SnzdtcenR {
        SnzdtcenR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 7 - Snooze mode Enable
    #[inline(always)]
    pub fn snze(&self) -> SnzeR {
        SnzeR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SNZCR")
            .field("rxdreqen", &self.rxdreqen())
            .field("snzdtcen", &self.snzdtcen())
            .field("snze", &self.snze())
            .finish()
    }
}
impl W {
    ///Bit 0 - RXD0 Snooze Request Enable
    #[inline(always)]
    pub fn rxdreqen(&mut self) -> RxdreqenW<SnzcrSpec> {
        RxdreqenW::new(self, 0)
    }
    ///Bit 1 - DTC Enable in Snooze mode
    #[inline(always)]
    pub fn snzdtcen(&mut self) -> SnzdtcenW<SnzcrSpec> {
        SnzdtcenW::new(self, 1)
    }
    ///Bit 7 - Snooze mode Enable
    #[inline(always)]
    pub fn snze(&mut self) -> SnzeW<SnzcrSpec> {
        SnzeW::new(self, 7)
    }
}
/**Snooze Control Register

You can [`read`](crate::Reg::read) this register and get [`snzcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snzcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SnzcrSpec;
impl crate::RegisterSpec for SnzcrSpec {
    type Ux = u8;
}
///`read()` method returns [`snzcr::R`](R) reader structure
impl crate::Readable for SnzcrSpec {}
///`write(|w| ..)` method takes [`snzcr::W`](W) writer structure
impl crate::Writable for SnzcrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SNZCR to value 0
impl crate::Resettable for SnzcrSpec {}
