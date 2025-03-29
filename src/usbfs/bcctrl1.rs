///Register `BCCTRL1` reader
pub type R = crate::R<Bcctrl1Spec>;
///Register `BCCTRL1` writer
pub type W = crate::W<Bcctrl1Spec>;
/**D- Line Pull-down Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rpdme {
    ///0: Disable D- Line Pull-down
    _0 = 0,
    ///1: Enable D- Line Pull-down
    _1 = 1,
}
impl From<Rpdme> for bool {
    #[inline(always)]
    fn from(variant: Rpdme) -> Self {
        variant as u8 != 0
    }
}
///Field `RPDME` reader - D- Line Pull-down Control
pub type RpdmeR = crate::BitReader<Rpdme>;
impl RpdmeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rpdme {
        match self.bits {
            false => Rpdme::_0,
            true => Rpdme::_1,
        }
    }
    ///Disable D- Line Pull-down
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rpdme::_0
    }
    ///Enable D- Line Pull-down
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rpdme::_1
    }
}
///Field `RPDME` writer - D- Line Pull-down Control
pub type RpdmeW<'a, REG> = crate::BitWriter<'a, REG, Rpdme>;
impl<'a, REG> RpdmeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable D- Line Pull-down
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rpdme::_0)
    }
    ///Enable D- Line Pull-down
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rpdme::_1)
    }
}
/**D+ Line IDPSRC Output Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Idpsrce {
    ///0: Stopped
    _0 = 0,
    ///1: 10 µA output
    _1 = 1,
}
impl From<Idpsrce> for bool {
    #[inline(always)]
    fn from(variant: Idpsrce) -> Self {
        variant as u8 != 0
    }
}
///Field `IDPSRCE` reader - D+ Line IDPSRC Output Control
pub type IdpsrceR = crate::BitReader<Idpsrce>;
impl IdpsrceR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Idpsrce {
        match self.bits {
            false => Idpsrce::_0,
            true => Idpsrce::_1,
        }
    }
    ///Stopped
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Idpsrce::_0
    }
    ///10 µA output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Idpsrce::_1
    }
}
///Field `IDPSRCE` writer - D+ Line IDPSRC Output Control
pub type IdpsrceW<'a, REG> = crate::BitWriter<'a, REG, Idpsrce>;
impl<'a, REG> IdpsrceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Stopped
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Idpsrce::_0)
    }
    ///10 µA output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Idpsrce::_1)
    }
}
/**D- Line VDMSRC (0.6 V) Output Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vdmsrce {
    ///0: Stopped
    _0 = 0,
    ///1: 0.6 V output
    _1 = 1,
}
impl From<Vdmsrce> for bool {
    #[inline(always)]
    fn from(variant: Vdmsrce) -> Self {
        variant as u8 != 0
    }
}
///Field `VDMSRCE` reader - D- Line VDMSRC (0.6 V) Output Control
pub type VdmsrceR = crate::BitReader<Vdmsrce>;
impl VdmsrceR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Vdmsrce {
        match self.bits {
            false => Vdmsrce::_0,
            true => Vdmsrce::_1,
        }
    }
    ///Stopped
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Vdmsrce::_0
    }
    ///0.6 V output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Vdmsrce::_1
    }
}
///Field `VDMSRCE` writer - D- Line VDMSRC (0.6 V) Output Control
pub type VdmsrceW<'a, REG> = crate::BitWriter<'a, REG, Vdmsrce>;
impl<'a, REG> VdmsrceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Stopped
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Vdmsrce::_0)
    }
    ///0.6 V output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Vdmsrce::_1)
    }
}
/**D+ Line VDPSRC (0.6 V) Output Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vdpsrce {
    ///0: Stopped
    _0 = 0,
    ///1: 0.6 V output
    _1 = 1,
}
impl From<Vdpsrce> for bool {
    #[inline(always)]
    fn from(variant: Vdpsrce) -> Self {
        variant as u8 != 0
    }
}
///Field `VDPSRCE` reader - D+ Line VDPSRC (0.6 V) Output Control
pub type VdpsrceR = crate::BitReader<Vdpsrce>;
impl VdpsrceR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Vdpsrce {
        match self.bits {
            false => Vdpsrce::_0,
            true => Vdpsrce::_1,
        }
    }
    ///Stopped
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Vdpsrce::_0
    }
    ///0.6 V output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Vdpsrce::_1
    }
}
///Field `VDPSRCE` writer - D+ Line VDPSRC (0.6 V) Output Control
pub type VdpsrceW<'a, REG> = crate::BitWriter<'a, REG, Vdpsrce>;
impl<'a, REG> VdpsrceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Stopped
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Vdpsrce::_0)
    }
    ///0.6 V output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Vdpsrce::_1)
    }
}
/**D+ Line 0.6 V Input Detection Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pddete {
    ///0: Disable detection
    _0 = 0,
    ///1: Enable detection
    _1 = 1,
}
impl From<Pddete> for bool {
    #[inline(always)]
    fn from(variant: Pddete) -> Self {
        variant as u8 != 0
    }
}
///Field `PDDETE` reader - D+ Line 0.6 V Input Detection Control
pub type PddeteR = crate::BitReader<Pddete>;
impl PddeteR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pddete {
        match self.bits {
            false => Pddete::_0,
            true => Pddete::_1,
        }
    }
    ///Disable detection
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pddete::_0
    }
    ///Enable detection
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pddete::_1
    }
}
///Field `PDDETE` writer - D+ Line 0.6 V Input Detection Control
pub type PddeteW<'a, REG> = crate::BitWriter<'a, REG, Pddete>;
impl<'a, REG> PddeteW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable detection
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pddete::_0)
    }
    ///Enable detection
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pddete::_1)
    }
}
/**D- Line 0.6 V Input Detection Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chgdete {
    ///0: Disable detection
    _0 = 0,
    ///1: Enable detection
    _1 = 1,
}
impl From<Chgdete> for bool {
    #[inline(always)]
    fn from(variant: Chgdete) -> Self {
        variant as u8 != 0
    }
}
///Field `CHGDETE` reader - D- Line 0.6 V Input Detection Control
pub type ChgdeteR = crate::BitReader<Chgdete>;
impl ChgdeteR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Chgdete {
        match self.bits {
            false => Chgdete::_0,
            true => Chgdete::_1,
        }
    }
    ///Disable detection
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Chgdete::_0
    }
    ///Enable detection
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Chgdete::_1
    }
}
///Field `CHGDETE` writer - D- Line 0.6 V Input Detection Control
pub type ChgdeteW<'a, REG> = crate::BitWriter<'a, REG, Chgdete>;
impl<'a, REG> ChgdeteW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable detection
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Chgdete::_0)
    }
    ///Enable detection
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Chgdete::_1)
    }
}
/**D+ Line 0.6 V Input Detection Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pddetsts {
    ///0: Not detected
    _0 = 0,
    ///1: Detected
    _1 = 1,
}
impl From<Pddetsts> for bool {
    #[inline(always)]
    fn from(variant: Pddetsts) -> Self {
        variant as u8 != 0
    }
}
///Field `PDDETSTS` reader - D+ Line 0.6 V Input Detection Status Flag
pub type PddetstsR = crate::BitReader<Pddetsts>;
impl PddetstsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pddetsts {
        match self.bits {
            false => Pddetsts::_0,
            true => Pddetsts::_1,
        }
    }
    ///Not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pddetsts::_0
    }
    ///Detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pddetsts::_1
    }
}
/**D- Line 0.6 V Input Detection Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chgdetsts {
    ///0: Not detected
    _0 = 0,
    ///1: Detected
    _1 = 1,
}
impl From<Chgdetsts> for bool {
    #[inline(always)]
    fn from(variant: Chgdetsts) -> Self {
        variant as u8 != 0
    }
}
///Field `CHGDETSTS` reader - D- Line 0.6 V Input Detection Status Flag
pub type ChgdetstsR = crate::BitReader<Chgdetsts>;
impl ChgdetstsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Chgdetsts {
        match self.bits {
            false => Chgdetsts::_0,
            true => Chgdetsts::_1,
        }
    }
    ///Not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Chgdetsts::_0
    }
    ///Detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Chgdetsts::_1
    }
}
impl R {
    ///Bit 0 - D- Line Pull-down Control
    #[inline(always)]
    pub fn rpdme(&self) -> RpdmeR {
        RpdmeR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - D+ Line IDPSRC Output Control
    #[inline(always)]
    pub fn idpsrce(&self) -> IdpsrceR {
        IdpsrceR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - D- Line VDMSRC (0.6 V) Output Control
    #[inline(always)]
    pub fn vdmsrce(&self) -> VdmsrceR {
        VdmsrceR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - D+ Line VDPSRC (0.6 V) Output Control
    #[inline(always)]
    pub fn vdpsrce(&self) -> VdpsrceR {
        VdpsrceR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - D+ Line 0.6 V Input Detection Control
    #[inline(always)]
    pub fn pddete(&self) -> PddeteR {
        PddeteR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - D- Line 0.6 V Input Detection Control
    #[inline(always)]
    pub fn chgdete(&self) -> ChgdeteR {
        ChgdeteR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - D+ Line 0.6 V Input Detection Status Flag
    #[inline(always)]
    pub fn pddetsts(&self) -> PddetstsR {
        PddetstsR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - D- Line 0.6 V Input Detection Status Flag
    #[inline(always)]
    pub fn chgdetsts(&self) -> ChgdetstsR {
        ChgdetstsR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BCCTRL1")
            .field("rpdme", &self.rpdme())
            .field("idpsrce", &self.idpsrce())
            .field("vdmsrce", &self.vdmsrce())
            .field("vdpsrce", &self.vdpsrce())
            .field("pddete", &self.pddete())
            .field("chgdete", &self.chgdete())
            .field("pddetsts", &self.pddetsts())
            .field("chgdetsts", &self.chgdetsts())
            .finish()
    }
}
impl W {
    ///Bit 0 - D- Line Pull-down Control
    #[inline(always)]
    pub fn rpdme(&mut self) -> RpdmeW<Bcctrl1Spec> {
        RpdmeW::new(self, 0)
    }
    ///Bit 1 - D+ Line IDPSRC Output Control
    #[inline(always)]
    pub fn idpsrce(&mut self) -> IdpsrceW<Bcctrl1Spec> {
        IdpsrceW::new(self, 1)
    }
    ///Bit 2 - D- Line VDMSRC (0.6 V) Output Control
    #[inline(always)]
    pub fn vdmsrce(&mut self) -> VdmsrceW<Bcctrl1Spec> {
        VdmsrceW::new(self, 2)
    }
    ///Bit 3 - D+ Line VDPSRC (0.6 V) Output Control
    #[inline(always)]
    pub fn vdpsrce(&mut self) -> VdpsrceW<Bcctrl1Spec> {
        VdpsrceW::new(self, 3)
    }
    ///Bit 4 - D+ Line 0.6 V Input Detection Control
    #[inline(always)]
    pub fn pddete(&mut self) -> PddeteW<Bcctrl1Spec> {
        PddeteW::new(self, 4)
    }
    ///Bit 5 - D- Line 0.6 V Input Detection Control
    #[inline(always)]
    pub fn chgdete(&mut self) -> ChgdeteW<Bcctrl1Spec> {
        ChgdeteW::new(self, 5)
    }
}
/**Battery Charging Control Register 1

You can [`read`](crate::Reg::read) this register and get [`bcctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Bcctrl1Spec;
impl crate::RegisterSpec for Bcctrl1Spec {
    type Ux = u32;
}
///`read()` method returns [`bcctrl1::R`](R) reader structure
impl crate::Readable for Bcctrl1Spec {}
///`write(|w| ..)` method takes [`bcctrl1::W`](W) writer structure
impl crate::Writable for Bcctrl1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BCCTRL1 to value 0
impl crate::Resettable for Bcctrl1Spec {}
