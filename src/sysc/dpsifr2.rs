///Register `DPSIFR2` reader
pub type R = crate::R<Dpsifr2Spec>;
///Register `DPSIFR2` writer
pub type W = crate::W<Dpsifr2Spec>;
/**LVD1 Deep Software Standby Cancel Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dlvd1if {
    ///0: The cancel request is not generated
    _0 = 0,
    ///1: The cancel request is generated
    _1 = 1,
}
impl From<Dlvd1if> for bool {
    #[inline(always)]
    fn from(variant: Dlvd1if) -> Self {
        variant as u8 != 0
    }
}
///Field `DLVD1IF` reader - LVD1 Deep Software Standby Cancel Flag
pub type Dlvd1ifR = crate::BitReader<Dlvd1if>;
impl Dlvd1ifR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dlvd1if {
        match self.bits {
            false => Dlvd1if::_0,
            true => Dlvd1if::_1,
        }
    }
    ///The cancel request is not generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dlvd1if::_0
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dlvd1if::_1
    }
}
///Field `DLVD1IF` writer - LVD1 Deep Software Standby Cancel Flag
pub type Dlvd1ifW<'a, REG> = crate::BitWriter<'a, REG, Dlvd1if>;
impl<'a, REG> Dlvd1ifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The cancel request is not generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dlvd1if::_0)
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dlvd1if::_1)
    }
}
/**LVD2 Deep Software Standby Cancel Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dlvd2if {
    ///0: The cancel request is not generated
    _0 = 0,
    ///1: The cancel request is generated
    _1 = 1,
}
impl From<Dlvd2if> for bool {
    #[inline(always)]
    fn from(variant: Dlvd2if) -> Self {
        variant as u8 != 0
    }
}
///Field `DLVD2IF` reader - LVD2 Deep Software Standby Cancel Flag
pub type Dlvd2ifR = crate::BitReader<Dlvd2if>;
impl Dlvd2ifR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dlvd2if {
        match self.bits {
            false => Dlvd2if::_0,
            true => Dlvd2if::_1,
        }
    }
    ///The cancel request is not generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dlvd2if::_0
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dlvd2if::_1
    }
}
///Field `DLVD2IF` writer - LVD2 Deep Software Standby Cancel Flag
pub type Dlvd2ifW<'a, REG> = crate::BitWriter<'a, REG, Dlvd2if>;
impl<'a, REG> Dlvd2ifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The cancel request is not generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dlvd2if::_0)
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dlvd2if::_1)
    }
}
/**RTC Interval Interrupt Deep Software Standby Cancel Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Drtciif {
    ///0: The cancel request is not generated
    _0 = 0,
    ///1: The cancel request is generated
    _1 = 1,
}
impl From<Drtciif> for bool {
    #[inline(always)]
    fn from(variant: Drtciif) -> Self {
        variant as u8 != 0
    }
}
///Field `DRTCIIF` reader - RTC Interval Interrupt Deep Software Standby Cancel Flag
pub type DrtciifR = crate::BitReader<Drtciif>;
impl DrtciifR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Drtciif {
        match self.bits {
            false => Drtciif::_0,
            true => Drtciif::_1,
        }
    }
    ///The cancel request is not generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Drtciif::_0
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Drtciif::_1
    }
}
///Field `DRTCIIF` writer - RTC Interval Interrupt Deep Software Standby Cancel Flag
pub type DrtciifW<'a, REG> = crate::BitWriter<'a, REG, Drtciif>;
impl<'a, REG> DrtciifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The cancel request is not generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Drtciif::_0)
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Drtciif::_1)
    }
}
/**RTC Alarm Interrupt Deep Software Standby Cancel Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Drtcaif {
    ///0: The cancel request is not generated
    _0 = 0,
    ///1: The cancel request is generated
    _1 = 1,
}
impl From<Drtcaif> for bool {
    #[inline(always)]
    fn from(variant: Drtcaif) -> Self {
        variant as u8 != 0
    }
}
///Field `DRTCAIF` reader - RTC Alarm Interrupt Deep Software Standby Cancel Flag
pub type DrtcaifR = crate::BitReader<Drtcaif>;
impl DrtcaifR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Drtcaif {
        match self.bits {
            false => Drtcaif::_0,
            true => Drtcaif::_1,
        }
    }
    ///The cancel request is not generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Drtcaif::_0
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Drtcaif::_1
    }
}
///Field `DRTCAIF` writer - RTC Alarm Interrupt Deep Software Standby Cancel Flag
pub type DrtcaifW<'a, REG> = crate::BitWriter<'a, REG, Drtcaif>;
impl<'a, REG> DrtcaifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The cancel request is not generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Drtcaif::_0)
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Drtcaif::_1)
    }
}
/**NMI Pin Deep Software Standby Cancel Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dnmif {
    ///0: The cancel request is not generated
    _0 = 0,
    ///1: The cancel request is generated
    _1 = 1,
}
impl From<Dnmif> for bool {
    #[inline(always)]
    fn from(variant: Dnmif) -> Self {
        variant as u8 != 0
    }
}
///Field `DNMIF` reader - NMI Pin Deep Software Standby Cancel Flag
pub type DnmifR = crate::BitReader<Dnmif>;
impl DnmifR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dnmif {
        match self.bits {
            false => Dnmif::_0,
            true => Dnmif::_1,
        }
    }
    ///The cancel request is not generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dnmif::_0
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dnmif::_1
    }
}
///Field `DNMIF` writer - NMI Pin Deep Software Standby Cancel Flag
pub type DnmifW<'a, REG> = crate::BitWriter<'a, REG, Dnmif>;
impl<'a, REG> DnmifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The cancel request is not generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dnmif::_0)
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dnmif::_1)
    }
}
impl R {
    ///Bit 0 - LVD1 Deep Software Standby Cancel Flag
    #[inline(always)]
    pub fn dlvd1if(&self) -> Dlvd1ifR {
        Dlvd1ifR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LVD2 Deep Software Standby Cancel Flag
    #[inline(always)]
    pub fn dlvd2if(&self) -> Dlvd2ifR {
        Dlvd2ifR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - RTC Interval Interrupt Deep Software Standby Cancel Flag
    #[inline(always)]
    pub fn drtciif(&self) -> DrtciifR {
        DrtciifR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - RTC Alarm Interrupt Deep Software Standby Cancel Flag
    #[inline(always)]
    pub fn drtcaif(&self) -> DrtcaifR {
        DrtcaifR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - NMI Pin Deep Software Standby Cancel Flag
    #[inline(always)]
    pub fn dnmif(&self) -> DnmifR {
        DnmifR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DPSIFR2")
            .field("dlvd1if", &self.dlvd1if())
            .field("dlvd2if", &self.dlvd2if())
            .field("drtciif", &self.drtciif())
            .field("drtcaif", &self.drtcaif())
            .field("dnmif", &self.dnmif())
            .finish()
    }
}
impl W {
    ///Bit 0 - LVD1 Deep Software Standby Cancel Flag
    #[inline(always)]
    pub fn dlvd1if(&mut self) -> Dlvd1ifW<Dpsifr2Spec> {
        Dlvd1ifW::new(self, 0)
    }
    ///Bit 1 - LVD2 Deep Software Standby Cancel Flag
    #[inline(always)]
    pub fn dlvd2if(&mut self) -> Dlvd2ifW<Dpsifr2Spec> {
        Dlvd2ifW::new(self, 1)
    }
    ///Bit 2 - RTC Interval Interrupt Deep Software Standby Cancel Flag
    #[inline(always)]
    pub fn drtciif(&mut self) -> DrtciifW<Dpsifr2Spec> {
        DrtciifW::new(self, 2)
    }
    ///Bit 3 - RTC Alarm Interrupt Deep Software Standby Cancel Flag
    #[inline(always)]
    pub fn drtcaif(&mut self) -> DrtcaifW<Dpsifr2Spec> {
        DrtcaifW::new(self, 3)
    }
    ///Bit 4 - NMI Pin Deep Software Standby Cancel Flag
    #[inline(always)]
    pub fn dnmif(&mut self) -> DnmifW<Dpsifr2Spec> {
        DnmifW::new(self, 4)
    }
}
/**Deep Software Standby Interrupt Flag Register 2

You can [`read`](crate::Reg::read) this register and get [`dpsifr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpsifr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Dpsifr2Spec;
impl crate::RegisterSpec for Dpsifr2Spec {
    type Ux = u8;
}
///`read()` method returns [`dpsifr2::R`](R) reader structure
impl crate::Readable for Dpsifr2Spec {}
///`write(|w| ..)` method takes [`dpsifr2::W`](W) writer structure
impl crate::Writable for Dpsifr2Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DPSIFR2 to value 0
impl crate::Resettable for Dpsifr2Spec {}
