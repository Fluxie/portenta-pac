///Register `DPSIFR3` reader
pub type R = crate::R<Dpsifr3Spec>;
///Register `DPSIFR3` writer
pub type W = crate::W<Dpsifr3Spec>;
/**USBFS0 Suspend/Resume Deep Software Standby Cancel Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dusbfs0if {
    ///0: The cancel request is not generated.
    _0 = 0,
    ///1: The cancel request is generated.
    _1 = 1,
}
impl From<Dusbfs0if> for bool {
    #[inline(always)]
    fn from(variant: Dusbfs0if) -> Self {
        variant as u8 != 0
    }
}
///Field `DUSBFS0IF` reader - USBFS0 Suspend/Resume Deep Software Standby Cancel Flag
pub type Dusbfs0ifR = crate::BitReader<Dusbfs0if>;
impl Dusbfs0ifR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dusbfs0if {
        match self.bits {
            false => Dusbfs0if::_0,
            true => Dusbfs0if::_1,
        }
    }
    ///The cancel request is not generated.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dusbfs0if::_0
    }
    ///The cancel request is generated.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dusbfs0if::_1
    }
}
///Field `DUSBFS0IF` writer - USBFS0 Suspend/Resume Deep Software Standby Cancel Flag
pub type Dusbfs0ifW<'a, REG> = crate::BitWriter<'a, REG, Dusbfs0if>;
impl<'a, REG> Dusbfs0ifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The cancel request is not generated.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dusbfs0if::_0)
    }
    ///The cancel request is generated.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dusbfs0if::_1)
    }
}
/**USBHS Suspend/Resume Deep Software Standby Cancel Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dusbhsif {
    ///0: The cancel request is not generated
    _0 = 0,
    ///1: The cancel request is generated
    _1 = 1,
}
impl From<Dusbhsif> for bool {
    #[inline(always)]
    fn from(variant: Dusbhsif) -> Self {
        variant as u8 != 0
    }
}
///Field `DUSBHSIF` reader - USBHS Suspend/Resume Deep Software Standby Cancel Flag
pub type DusbhsifR = crate::BitReader<Dusbhsif>;
impl DusbhsifR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dusbhsif {
        match self.bits {
            false => Dusbhsif::_0,
            true => Dusbhsif::_1,
        }
    }
    ///The cancel request is not generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dusbhsif::_0
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dusbhsif::_1
    }
}
///Field `DUSBHSIF` writer - USBHS Suspend/Resume Deep Software Standby Cancel Flag
pub type DusbhsifW<'a, REG> = crate::BitWriter<'a, REG, Dusbhsif>;
impl<'a, REG> DusbhsifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The cancel request is not generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dusbhsif::_0)
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dusbhsif::_1)
    }
}
/**AGT1 Underflow Deep Software Standby Cancel Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dagt1if {
    ///0: The cancel request is not generated.
    _0 = 0,
    ///1: The cancel request is generated.
    _1 = 1,
}
impl From<Dagt1if> for bool {
    #[inline(always)]
    fn from(variant: Dagt1if) -> Self {
        variant as u8 != 0
    }
}
///Field `DAGT1IF` reader - AGT1 Underflow Deep Software Standby Cancel Flag
pub type Dagt1ifR = crate::BitReader<Dagt1if>;
impl Dagt1ifR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dagt1if {
        match self.bits {
            false => Dagt1if::_0,
            true => Dagt1if::_1,
        }
    }
    ///The cancel request is not generated.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dagt1if::_0
    }
    ///The cancel request is generated.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dagt1if::_1
    }
}
///Field `DAGT1IF` writer - AGT1 Underflow Deep Software Standby Cancel Flag
pub type Dagt1ifW<'a, REG> = crate::BitWriter<'a, REG, Dagt1if>;
impl<'a, REG> Dagt1ifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The cancel request is not generated.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dagt1if::_0)
    }
    ///The cancel request is generated.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dagt1if::_1)
    }
}
/**AGT3 Underflow Deep Software Standby Cancel Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dagt3if {
    ///0: The cancel request is not generated.
    _0 = 0,
    ///1: The cancel request is generated.
    _1 = 1,
}
impl From<Dagt3if> for bool {
    #[inline(always)]
    fn from(variant: Dagt3if) -> Self {
        variant as u8 != 0
    }
}
///Field `DAGT3IF` reader - AGT3 Underflow Deep Software Standby Cancel Flag
pub type Dagt3ifR = crate::BitReader<Dagt3if>;
impl Dagt3ifR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dagt3if {
        match self.bits {
            false => Dagt3if::_0,
            true => Dagt3if::_1,
        }
    }
    ///The cancel request is not generated.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dagt3if::_0
    }
    ///The cancel request is generated.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dagt3if::_1
    }
}
///Field `DAGT3IF` writer - AGT3 Underflow Deep Software Standby Cancel Flag
pub type Dagt3ifW<'a, REG> = crate::BitWriter<'a, REG, Dagt3if>;
impl<'a, REG> Dagt3ifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The cancel request is not generated.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dagt3if::_0)
    }
    ///The cancel request is generated.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dagt3if::_1)
    }
}
impl R {
    ///Bit 0 - USBFS0 Suspend/Resume Deep Software Standby Cancel Flag
    #[inline(always)]
    pub fn dusbfs0if(&self) -> Dusbfs0ifR {
        Dusbfs0ifR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - USBHS Suspend/Resume Deep Software Standby Cancel Flag
    #[inline(always)]
    pub fn dusbhsif(&self) -> DusbhsifR {
        DusbhsifR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - AGT1 Underflow Deep Software Standby Cancel Flag
    #[inline(always)]
    pub fn dagt1if(&self) -> Dagt1ifR {
        Dagt1ifR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - AGT3 Underflow Deep Software Standby Cancel Flag
    #[inline(always)]
    pub fn dagt3if(&self) -> Dagt3ifR {
        Dagt3ifR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DPSIFR3")
            .field("dusbfs0if", &self.dusbfs0if())
            .field("dusbhsif", &self.dusbhsif())
            .field("dagt1if", &self.dagt1if())
            .field("dagt3if", &self.dagt3if())
            .finish()
    }
}
impl W {
    ///Bit 0 - USBFS0 Suspend/Resume Deep Software Standby Cancel Flag
    #[inline(always)]
    pub fn dusbfs0if(&mut self) -> Dusbfs0ifW<Dpsifr3Spec> {
        Dusbfs0ifW::new(self, 0)
    }
    ///Bit 1 - USBHS Suspend/Resume Deep Software Standby Cancel Flag
    #[inline(always)]
    pub fn dusbhsif(&mut self) -> DusbhsifW<Dpsifr3Spec> {
        DusbhsifW::new(self, 1)
    }
    ///Bit 2 - AGT1 Underflow Deep Software Standby Cancel Flag
    #[inline(always)]
    pub fn dagt1if(&mut self) -> Dagt1ifW<Dpsifr3Spec> {
        Dagt1ifW::new(self, 2)
    }
    ///Bit 3 - AGT3 Underflow Deep Software Standby Cancel Flag
    #[inline(always)]
    pub fn dagt3if(&mut self) -> Dagt3ifW<Dpsifr3Spec> {
        Dagt3ifW::new(self, 3)
    }
}
/**Deep Software Standby Interrupt Flag Register 3

You can [`read`](crate::Reg::read) this register and get [`dpsifr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpsifr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Dpsifr3Spec;
impl crate::RegisterSpec for Dpsifr3Spec {
    type Ux = u8;
}
///`read()` method returns [`dpsifr3::R`](R) reader structure
impl crate::Readable for Dpsifr3Spec {}
///`write(|w| ..)` method takes [`dpsifr3::W`](W) writer structure
impl crate::Writable for Dpsifr3Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DPSIFR3 to value 0
impl crate::Resettable for Dpsifr3Spec {}
