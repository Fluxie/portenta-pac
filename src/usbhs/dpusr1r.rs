///Register `DPUSR1R` reader
pub type R = crate::R<Dpusr1rSpec>;
///Register `DPUSR1R` writer
pub type W = crate::W<Dpusr1rSpec>;
/**OVRCURA Interrupt Enable Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dovcahe {
    ///0: Disable recovery from Deep Software Standby mode
    _0 = 0,
    ///1: Enable recovery from Deep Software Standby mode
    _1 = 1,
}
impl From<Dovcahe> for bool {
    #[inline(always)]
    fn from(variant: Dovcahe) -> Self {
        variant as u8 != 0
    }
}
///Field `DOVCAHE` reader - OVRCURA Interrupt Enable Clear
pub type DovcaheR = crate::BitReader<Dovcahe>;
impl DovcaheR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dovcahe {
        match self.bits {
            false => Dovcahe::_0,
            true => Dovcahe::_1,
        }
    }
    ///Disable recovery from Deep Software Standby mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dovcahe::_0
    }
    ///Enable recovery from Deep Software Standby mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dovcahe::_1
    }
}
///Field `DOVCAHE` writer - OVRCURA Interrupt Enable Clear
pub type DovcaheW<'a, REG> = crate::BitWriter<'a, REG, Dovcahe>;
impl<'a, REG> DovcaheW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable recovery from Deep Software Standby mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dovcahe::_0)
    }
    ///Enable recovery from Deep Software Standby mode
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dovcahe::_1)
    }
}
/**OVRCURB Interrupt Enable Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dovcbhe {
    ///0: Disable recovery from Deep Software Standby mode
    _0 = 0,
    ///1: Enable recovery from Deep Software Standby mode
    _1 = 1,
}
impl From<Dovcbhe> for bool {
    #[inline(always)]
    fn from(variant: Dovcbhe) -> Self {
        variant as u8 != 0
    }
}
///Field `DOVCBHE` reader - OVRCURB Interrupt Enable Clear
pub type DovcbheR = crate::BitReader<Dovcbhe>;
impl DovcbheR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dovcbhe {
        match self.bits {
            false => Dovcbhe::_0,
            true => Dovcbhe::_1,
        }
    }
    ///Disable recovery from Deep Software Standby mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dovcbhe::_0
    }
    ///Enable recovery from Deep Software Standby mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dovcbhe::_1
    }
}
///Field `DOVCBHE` writer - OVRCURB Interrupt Enable Clear
pub type DovcbheW<'a, REG> = crate::BitWriter<'a, REG, Dovcbhe>;
impl<'a, REG> DovcbheW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable recovery from Deep Software Standby mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dovcbhe::_0)
    }
    ///Enable recovery from Deep Software Standby mode
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dovcbhe::_1)
    }
}
/**VBUS Interrupt Enable/Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dvbstshe {
    ///0: Disable recovery from Deep Software Standby mode
    _0 = 0,
    ///1: Enable recovery from Deep Software Standby mode
    _1 = 1,
}
impl From<Dvbstshe> for bool {
    #[inline(always)]
    fn from(variant: Dvbstshe) -> Self {
        variant as u8 != 0
    }
}
///Field `DVBSTSHE` reader - VBUS Interrupt Enable/Clear
pub type DvbstsheR = crate::BitReader<Dvbstshe>;
impl DvbstsheR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dvbstshe {
        match self.bits {
            false => Dvbstshe::_0,
            true => Dvbstshe::_1,
        }
    }
    ///Disable recovery from Deep Software Standby mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dvbstshe::_0
    }
    ///Enable recovery from Deep Software Standby mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dvbstshe::_1
    }
}
///Field `DVBSTSHE` writer - VBUS Interrupt Enable/Clear
pub type DvbstsheW<'a, REG> = crate::BitWriter<'a, REG, Dvbstshe>;
impl<'a, REG> DvbstsheW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable recovery from Deep Software Standby mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dvbstshe::_0)
    }
    ///Enable recovery from Deep Software Standby mode
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dvbstshe::_1)
    }
}
/**OVRCURA Interrupt Source Return Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dovcah {
    ///0: System has not recovered from Deep Software Standby mode
    _0 = 0,
    ///1: System recovered from Deep Software Standby mode
    _1 = 1,
}
impl From<Dovcah> for bool {
    #[inline(always)]
    fn from(variant: Dovcah) -> Self {
        variant as u8 != 0
    }
}
///Field `DOVCAH` reader - OVRCURA Interrupt Source Return Status Flag
pub type DovcahR = crate::BitReader<Dovcah>;
impl DovcahR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dovcah {
        match self.bits {
            false => Dovcah::_0,
            true => Dovcah::_1,
        }
    }
    ///System has not recovered from Deep Software Standby mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dovcah::_0
    }
    ///System recovered from Deep Software Standby mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dovcah::_1
    }
}
/**OVRCURB Interrupt Source Return Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dovcbh {
    ///0: System has not recovered from Deep Software Standby mode
    _0 = 0,
    ///1: System recovered from Deep Software Standby mode
    _1 = 1,
}
impl From<Dovcbh> for bool {
    #[inline(always)]
    fn from(variant: Dovcbh) -> Self {
        variant as u8 != 0
    }
}
///Field `DOVCBH` reader - OVRCURB Interrupt Source Return Status Flag
pub type DovcbhR = crate::BitReader<Dovcbh>;
impl DovcbhR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dovcbh {
        match self.bits {
            false => Dovcbh::_0,
            true => Dovcbh::_1,
        }
    }
    ///System has not recovered from Deep Software Standby mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dovcbh::_0
    }
    ///System recovered from Deep Software Standby mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dovcbh::_1
    }
}
/**VBUS Interrupt Source Return Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dvbstsh {
    ///0: System has not recovered from Deep Software Standby mode
    _0 = 0,
    ///1: System recovered from Deep Software Standby mode
    _1 = 1,
}
impl From<Dvbstsh> for bool {
    #[inline(always)]
    fn from(variant: Dvbstsh) -> Self {
        variant as u8 != 0
    }
}
///Field `DVBSTSH` reader - VBUS Interrupt Source Return Status Flag
pub type DvbstshR = crate::BitReader<Dvbstsh>;
impl DvbstshR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dvbstsh {
        match self.bits {
            false => Dvbstsh::_0,
            true => Dvbstsh::_1,
        }
    }
    ///System has not recovered from Deep Software Standby mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dvbstsh::_0
    }
    ///System recovered from Deep Software Standby mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dvbstsh::_1
    }
}
impl R {
    ///Bit 4 - OVRCURA Interrupt Enable Clear
    #[inline(always)]
    pub fn dovcahe(&self) -> DovcaheR {
        DovcaheR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - OVRCURB Interrupt Enable Clear
    #[inline(always)]
    pub fn dovcbhe(&self) -> DovcbheR {
        DovcbheR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - VBUS Interrupt Enable/Clear
    #[inline(always)]
    pub fn dvbstshe(&self) -> DvbstsheR {
        DvbstsheR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 20 - OVRCURA Interrupt Source Return Status Flag
    #[inline(always)]
    pub fn dovcah(&self) -> DovcahR {
        DovcahR::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - OVRCURB Interrupt Source Return Status Flag
    #[inline(always)]
    pub fn dovcbh(&self) -> DovcbhR {
        DovcbhR::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 23 - VBUS Interrupt Source Return Status Flag
    #[inline(always)]
    pub fn dvbstsh(&self) -> DvbstshR {
        DvbstshR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DPUSR1R")
            .field("dovcahe", &self.dovcahe())
            .field("dovcbhe", &self.dovcbhe())
            .field("dvbstshe", &self.dvbstshe())
            .field("dovcah", &self.dovcah())
            .field("dovcbh", &self.dovcbh())
            .field("dvbstsh", &self.dvbstsh())
            .finish()
    }
}
impl W {
    ///Bit 4 - OVRCURA Interrupt Enable Clear
    #[inline(always)]
    pub fn dovcahe(&mut self) -> DovcaheW<Dpusr1rSpec> {
        DovcaheW::new(self, 4)
    }
    ///Bit 5 - OVRCURB Interrupt Enable Clear
    #[inline(always)]
    pub fn dovcbhe(&mut self) -> DovcbheW<Dpusr1rSpec> {
        DovcbheW::new(self, 5)
    }
    ///Bit 7 - VBUS Interrupt Enable/Clear
    #[inline(always)]
    pub fn dvbstshe(&mut self) -> DvbstsheW<Dpusr1rSpec> {
        DvbstsheW::new(self, 7)
    }
}
/**Deep Software Standby USB Suspend/Resume Interrupt Register

You can [`read`](crate::Reg::read) this register and get [`dpusr1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpusr1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Dpusr1rSpec;
impl crate::RegisterSpec for Dpusr1rSpec {
    type Ux = u32;
}
///`read()` method returns [`dpusr1r::R`](R) reader structure
impl crate::Readable for Dpusr1rSpec {}
///`write(|w| ..)` method takes [`dpusr1r::W`](W) writer structure
impl crate::Writable for Dpusr1rSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DPUSR1R to value 0
impl crate::Resettable for Dpusr1rSpec {}
