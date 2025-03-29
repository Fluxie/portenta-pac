///Register `DPSIER3` reader
pub type R = crate::R<Dpsier3Spec>;
///Register `DPSIER3` writer
pub type W = crate::W<Dpsier3Spec>;
/**USBFS0 Suspend/Resume Deep Software Standby Cancel Signal Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dusbfs0ie {
    ///0: Cancelling Deep Software Standby mode is disabled
    _0 = 0,
    ///1: Cancelling Deep Software Standby mode is enabled
    _1 = 1,
}
impl From<Dusbfs0ie> for bool {
    #[inline(always)]
    fn from(variant: Dusbfs0ie) -> Self {
        variant as u8 != 0
    }
}
///Field `DUSBFS0IE` reader - USBFS0 Suspend/Resume Deep Software Standby Cancel Signal Enable
pub type Dusbfs0ieR = crate::BitReader<Dusbfs0ie>;
impl Dusbfs0ieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dusbfs0ie {
        match self.bits {
            false => Dusbfs0ie::_0,
            true => Dusbfs0ie::_1,
        }
    }
    ///Cancelling Deep Software Standby mode is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dusbfs0ie::_0
    }
    ///Cancelling Deep Software Standby mode is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dusbfs0ie::_1
    }
}
///Field `DUSBFS0IE` writer - USBFS0 Suspend/Resume Deep Software Standby Cancel Signal Enable
pub type Dusbfs0ieW<'a, REG> = crate::BitWriter<'a, REG, Dusbfs0ie>;
impl<'a, REG> Dusbfs0ieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancelling Deep Software Standby mode is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dusbfs0ie::_0)
    }
    ///Cancelling Deep Software Standby mode is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dusbfs0ie::_1)
    }
}
/**USBHS Suspend/Resume Deep Software Standby Cancel Signal Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dusbhsie {
    ///0: Cancelling Deep Software Standby mode is disabled
    _0 = 0,
    ///1: Cancelling Deep Software Standby mode is enabled
    _1 = 1,
}
impl From<Dusbhsie> for bool {
    #[inline(always)]
    fn from(variant: Dusbhsie) -> Self {
        variant as u8 != 0
    }
}
///Field `DUSBHSIE` reader - USBHS Suspend/Resume Deep Software Standby Cancel Signal Enable
pub type DusbhsieR = crate::BitReader<Dusbhsie>;
impl DusbhsieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dusbhsie {
        match self.bits {
            false => Dusbhsie::_0,
            true => Dusbhsie::_1,
        }
    }
    ///Cancelling Deep Software Standby mode is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dusbhsie::_0
    }
    ///Cancelling Deep Software Standby mode is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dusbhsie::_1
    }
}
///Field `DUSBHSIE` writer - USBHS Suspend/Resume Deep Software Standby Cancel Signal Enable
pub type DusbhsieW<'a, REG> = crate::BitWriter<'a, REG, Dusbhsie>;
impl<'a, REG> DusbhsieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancelling Deep Software Standby mode is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dusbhsie::_0)
    }
    ///Cancelling Deep Software Standby mode is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dusbhsie::_1)
    }
}
/**AGT1 Underflow Deep Software Standby Cancel Signal Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dagt1ie {
    ///0: Cancelling Deep Software Standby mode is disabled
    _0 = 0,
    ///1: Cancelling Deep Software Standby mode is enabled
    _1 = 1,
}
impl From<Dagt1ie> for bool {
    #[inline(always)]
    fn from(variant: Dagt1ie) -> Self {
        variant as u8 != 0
    }
}
///Field `DAGT1IE` reader - AGT1 Underflow Deep Software Standby Cancel Signal Enable
pub type Dagt1ieR = crate::BitReader<Dagt1ie>;
impl Dagt1ieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dagt1ie {
        match self.bits {
            false => Dagt1ie::_0,
            true => Dagt1ie::_1,
        }
    }
    ///Cancelling Deep Software Standby mode is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dagt1ie::_0
    }
    ///Cancelling Deep Software Standby mode is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dagt1ie::_1
    }
}
///Field `DAGT1IE` writer - AGT1 Underflow Deep Software Standby Cancel Signal Enable
pub type Dagt1ieW<'a, REG> = crate::BitWriter<'a, REG, Dagt1ie>;
impl<'a, REG> Dagt1ieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancelling Deep Software Standby mode is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dagt1ie::_0)
    }
    ///Cancelling Deep Software Standby mode is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dagt1ie::_1)
    }
}
/**AGT3 Underflow Deep Software Standby Cancel Signal Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dagt3ie {
    ///0: Cancelling Deep Software Standby mode is disabled
    _0 = 0,
    ///1: Cancelling Deep Software Standby mode is enabled
    _1 = 1,
}
impl From<Dagt3ie> for bool {
    #[inline(always)]
    fn from(variant: Dagt3ie) -> Self {
        variant as u8 != 0
    }
}
///Field `DAGT3IE` reader - AGT3 Underflow Deep Software Standby Cancel Signal Enable
pub type Dagt3ieR = crate::BitReader<Dagt3ie>;
impl Dagt3ieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dagt3ie {
        match self.bits {
            false => Dagt3ie::_0,
            true => Dagt3ie::_1,
        }
    }
    ///Cancelling Deep Software Standby mode is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dagt3ie::_0
    }
    ///Cancelling Deep Software Standby mode is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dagt3ie::_1
    }
}
///Field `DAGT3IE` writer - AGT3 Underflow Deep Software Standby Cancel Signal Enable
pub type Dagt3ieW<'a, REG> = crate::BitWriter<'a, REG, Dagt3ie>;
impl<'a, REG> Dagt3ieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancelling Deep Software Standby mode is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dagt3ie::_0)
    }
    ///Cancelling Deep Software Standby mode is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dagt3ie::_1)
    }
}
impl R {
    ///Bit 0 - USBFS0 Suspend/Resume Deep Software Standby Cancel Signal Enable
    #[inline(always)]
    pub fn dusbfs0ie(&self) -> Dusbfs0ieR {
        Dusbfs0ieR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - USBHS Suspend/Resume Deep Software Standby Cancel Signal Enable
    #[inline(always)]
    pub fn dusbhsie(&self) -> DusbhsieR {
        DusbhsieR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - AGT1 Underflow Deep Software Standby Cancel Signal Enable
    #[inline(always)]
    pub fn dagt1ie(&self) -> Dagt1ieR {
        Dagt1ieR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - AGT3 Underflow Deep Software Standby Cancel Signal Enable
    #[inline(always)]
    pub fn dagt3ie(&self) -> Dagt3ieR {
        Dagt3ieR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DPSIER3")
            .field("dusbfs0ie", &self.dusbfs0ie())
            .field("dusbhsie", &self.dusbhsie())
            .field("dagt1ie", &self.dagt1ie())
            .field("dagt3ie", &self.dagt3ie())
            .finish()
    }
}
impl W {
    ///Bit 0 - USBFS0 Suspend/Resume Deep Software Standby Cancel Signal Enable
    #[inline(always)]
    pub fn dusbfs0ie(&mut self) -> Dusbfs0ieW<Dpsier3Spec> {
        Dusbfs0ieW::new(self, 0)
    }
    ///Bit 1 - USBHS Suspend/Resume Deep Software Standby Cancel Signal Enable
    #[inline(always)]
    pub fn dusbhsie(&mut self) -> DusbhsieW<Dpsier3Spec> {
        DusbhsieW::new(self, 1)
    }
    ///Bit 2 - AGT1 Underflow Deep Software Standby Cancel Signal Enable
    #[inline(always)]
    pub fn dagt1ie(&mut self) -> Dagt1ieW<Dpsier3Spec> {
        Dagt1ieW::new(self, 2)
    }
    ///Bit 3 - AGT3 Underflow Deep Software Standby Cancel Signal Enable
    #[inline(always)]
    pub fn dagt3ie(&mut self) -> Dagt3ieW<Dpsier3Spec> {
        Dagt3ieW::new(self, 3)
    }
}
/**Deep Software Standby Interrupt Enable Register 3

You can [`read`](crate::Reg::read) this register and get [`dpsier3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpsier3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Dpsier3Spec;
impl crate::RegisterSpec for Dpsier3Spec {
    type Ux = u8;
}
///`read()` method returns [`dpsier3::R`](R) reader structure
impl crate::Readable for Dpsier3Spec {}
///`write(|w| ..)` method takes [`dpsier3::W`](W) writer structure
impl crate::Writable for Dpsier3Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DPSIER3 to value 0
impl crate::Resettable for Dpsier3Spec {}
