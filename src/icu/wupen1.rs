///Register `WUPEN1` reader
pub type R = crate::R<Wupen1Spec>;
///Register `WUPEN1` writer
pub type W = crate::W<Wupen1Spec>;
/**AGT3 Underflow Interrupt Software Standby Return Enable bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Agt3udwupen {
    ///0: Software standby returns by AGT3 underflow interrupt is disabled
    _0 = 0,
    ///1: Software standby returns by AGT3 underflow interrupt is enabled
    _1 = 1,
}
impl From<Agt3udwupen> for bool {
    #[inline(always)]
    fn from(variant: Agt3udwupen) -> Self {
        variant as u8 != 0
    }
}
///Field `AGT3UDWUPEN` reader - AGT3 Underflow Interrupt Software Standby Return Enable bit
pub type Agt3udwupenR = crate::BitReader<Agt3udwupen>;
impl Agt3udwupenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Agt3udwupen {
        match self.bits {
            false => Agt3udwupen::_0,
            true => Agt3udwupen::_1,
        }
    }
    ///Software standby returns by AGT3 underflow interrupt is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Agt3udwupen::_0
    }
    ///Software standby returns by AGT3 underflow interrupt is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Agt3udwupen::_1
    }
}
///Field `AGT3UDWUPEN` writer - AGT3 Underflow Interrupt Software Standby Return Enable bit
pub type Agt3udwupenW<'a, REG> = crate::BitWriter<'a, REG, Agt3udwupen>;
impl<'a, REG> Agt3udwupenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Software standby returns by AGT3 underflow interrupt is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Agt3udwupen::_0)
    }
    ///Software standby returns by AGT3 underflow interrupt is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Agt3udwupen::_1)
    }
}
/**AGT3 Compare Match A Interrupt Software Standby Return Enable bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Agt3cawupen {
    ///0: Software standby returns by AGT3 compare match A interrupt is disabled
    _0 = 0,
    ///1: Software standby returns by AGT3 compare match A interrupt is enabled
    _1 = 1,
}
impl From<Agt3cawupen> for bool {
    #[inline(always)]
    fn from(variant: Agt3cawupen) -> Self {
        variant as u8 != 0
    }
}
///Field `AGT3CAWUPEN` reader - AGT3 Compare Match A Interrupt Software Standby Return Enable bit
pub type Agt3cawupenR = crate::BitReader<Agt3cawupen>;
impl Agt3cawupenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Agt3cawupen {
        match self.bits {
            false => Agt3cawupen::_0,
            true => Agt3cawupen::_1,
        }
    }
    ///Software standby returns by AGT3 compare match A interrupt is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Agt3cawupen::_0
    }
    ///Software standby returns by AGT3 compare match A interrupt is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Agt3cawupen::_1
    }
}
///Field `AGT3CAWUPEN` writer - AGT3 Compare Match A Interrupt Software Standby Return Enable bit
pub type Agt3cawupenW<'a, REG> = crate::BitWriter<'a, REG, Agt3cawupen>;
impl<'a, REG> Agt3cawupenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Software standby returns by AGT3 compare match A interrupt is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Agt3cawupen::_0)
    }
    ///Software standby returns by AGT3 compare match A interrupt is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Agt3cawupen::_1)
    }
}
/**AGT3 Compare Match B Interrupt Software Standby Return Enable bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Agt3cbwupen {
    ///0: Software standby returns by AGT3 compare match B interrupt is disabled
    _0 = 0,
    ///1: Software standby returns by AGT3 compare match B interrupt is enabled
    _1 = 1,
}
impl From<Agt3cbwupen> for bool {
    #[inline(always)]
    fn from(variant: Agt3cbwupen) -> Self {
        variant as u8 != 0
    }
}
///Field `AGT3CBWUPEN` reader - AGT3 Compare Match B Interrupt Software Standby Return Enable bit
pub type Agt3cbwupenR = crate::BitReader<Agt3cbwupen>;
impl Agt3cbwupenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Agt3cbwupen {
        match self.bits {
            false => Agt3cbwupen::_0,
            true => Agt3cbwupen::_1,
        }
    }
    ///Software standby returns by AGT3 compare match B interrupt is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Agt3cbwupen::_0
    }
    ///Software standby returns by AGT3 compare match B interrupt is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Agt3cbwupen::_1
    }
}
///Field `AGT3CBWUPEN` writer - AGT3 Compare Match B Interrupt Software Standby Return Enable bit
pub type Agt3cbwupenW<'a, REG> = crate::BitWriter<'a, REG, Agt3cbwupen>;
impl<'a, REG> Agt3cbwupenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Software standby returns by AGT3 compare match B interrupt is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Agt3cbwupen::_0)
    }
    ///Software standby returns by AGT3 compare match B interrupt is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Agt3cbwupen::_1)
    }
}
impl R {
    ///Bit 0 - AGT3 Underflow Interrupt Software Standby Return Enable bit
    #[inline(always)]
    pub fn agt3udwupen(&self) -> Agt3udwupenR {
        Agt3udwupenR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - AGT3 Compare Match A Interrupt Software Standby Return Enable bit
    #[inline(always)]
    pub fn agt3cawupen(&self) -> Agt3cawupenR {
        Agt3cawupenR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - AGT3 Compare Match B Interrupt Software Standby Return Enable bit
    #[inline(always)]
    pub fn agt3cbwupen(&self) -> Agt3cbwupenR {
        Agt3cbwupenR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WUPEN1")
            .field("agt3udwupen", &self.agt3udwupen())
            .field("agt3cawupen", &self.agt3cawupen())
            .field("agt3cbwupen", &self.agt3cbwupen())
            .finish()
    }
}
impl W {
    ///Bit 0 - AGT3 Underflow Interrupt Software Standby Return Enable bit
    #[inline(always)]
    pub fn agt3udwupen(&mut self) -> Agt3udwupenW<Wupen1Spec> {
        Agt3udwupenW::new(self, 0)
    }
    ///Bit 1 - AGT3 Compare Match A Interrupt Software Standby Return Enable bit
    #[inline(always)]
    pub fn agt3cawupen(&mut self) -> Agt3cawupenW<Wupen1Spec> {
        Agt3cawupenW::new(self, 1)
    }
    ///Bit 2 - AGT3 Compare Match B Interrupt Software Standby Return Enable bit
    #[inline(always)]
    pub fn agt3cbwupen(&mut self) -> Agt3cbwupenW<Wupen1Spec> {
        Agt3cbwupenW::new(self, 2)
    }
}
/**Wake Up interrupt enable register 1

You can [`read`](crate::Reg::read) this register and get [`wupen1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wupen1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Wupen1Spec;
impl crate::RegisterSpec for Wupen1Spec {
    type Ux = u32;
}
///`read()` method returns [`wupen1::R`](R) reader structure
impl crate::Readable for Wupen1Spec {}
///`write(|w| ..)` method takes [`wupen1::W`](W) writer structure
impl crate::Writable for Wupen1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WUPEN1 to value 0
impl crate::Resettable for Wupen1Spec {}
