///Register `INTENB0` reader
pub type R = crate::R<Intenb0Spec>;
///Register `INTENB0` writer
pub type W = crate::W<Intenb0Spec>;
/**Buffer Ready Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Brdye {
    ///0: Disable interrupt request
    _0 = 0,
    ///1: Enable interrupt request
    _1 = 1,
}
impl From<Brdye> for bool {
    #[inline(always)]
    fn from(variant: Brdye) -> Self {
        variant as u8 != 0
    }
}
///Field `BRDYE` reader - Buffer Ready Interrupt Request Enable
pub type BrdyeR = crate::BitReader<Brdye>;
impl BrdyeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Brdye {
        match self.bits {
            false => Brdye::_0,
            true => Brdye::_1,
        }
    }
    ///Disable interrupt request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Brdye::_0
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Brdye::_1
    }
}
///Field `BRDYE` writer - Buffer Ready Interrupt Request Enable
pub type BrdyeW<'a, REG> = crate::BitWriter<'a, REG, Brdye>;
impl<'a, REG> BrdyeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable interrupt request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Brdye::_0)
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Brdye::_1)
    }
}
/**Buffer Not Ready Response Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nrdye {
    ///0: Disable interrupt request
    _0 = 0,
    ///1: Enable interrupt request
    _1 = 1,
}
impl From<Nrdye> for bool {
    #[inline(always)]
    fn from(variant: Nrdye) -> Self {
        variant as u8 != 0
    }
}
///Field `NRDYE` reader - Buffer Not Ready Response Interrupt Request Enable
pub type NrdyeR = crate::BitReader<Nrdye>;
impl NrdyeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Nrdye {
        match self.bits {
            false => Nrdye::_0,
            true => Nrdye::_1,
        }
    }
    ///Disable interrupt request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Nrdye::_0
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Nrdye::_1
    }
}
///Field `NRDYE` writer - Buffer Not Ready Response Interrupt Request Enable
pub type NrdyeW<'a, REG> = crate::BitWriter<'a, REG, Nrdye>;
impl<'a, REG> NrdyeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable interrupt request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Nrdye::_0)
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Nrdye::_1)
    }
}
/**Buffer Empty Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bempe {
    ///0: Disable interrupt request
    _0 = 0,
    ///1: Enable interrupt request
    _1 = 1,
}
impl From<Bempe> for bool {
    #[inline(always)]
    fn from(variant: Bempe) -> Self {
        variant as u8 != 0
    }
}
///Field `BEMPE` reader - Buffer Empty Interrupt Request Enable
pub type BempeR = crate::BitReader<Bempe>;
impl BempeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Bempe {
        match self.bits {
            false => Bempe::_0,
            true => Bempe::_1,
        }
    }
    ///Disable interrupt request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bempe::_0
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bempe::_1
    }
}
///Field `BEMPE` writer - Buffer Empty Interrupt Request Enable
pub type BempeW<'a, REG> = crate::BitWriter<'a, REG, Bempe>;
impl<'a, REG> BempeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable interrupt request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bempe::_0)
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bempe::_1)
    }
}
/**Control Transfer Stage Transition Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctre {
    ///0: Disable interrupt request
    _0 = 0,
    ///1: Enable interrupt request
    _1 = 1,
}
impl From<Ctre> for bool {
    #[inline(always)]
    fn from(variant: Ctre) -> Self {
        variant as u8 != 0
    }
}
///Field `CTRE` reader - Control Transfer Stage Transition Interrupt Request Enable
pub type CtreR = crate::BitReader<Ctre>;
impl CtreR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ctre {
        match self.bits {
            false => Ctre::_0,
            true => Ctre::_1,
        }
    }
    ///Disable interrupt request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ctre::_0
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ctre::_1
    }
}
///Field `CTRE` writer - Control Transfer Stage Transition Interrupt Request Enable
pub type CtreW<'a, REG> = crate::BitWriter<'a, REG, Ctre>;
impl<'a, REG> CtreW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable interrupt request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ctre::_0)
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ctre::_1)
    }
}
/**Device State Transition Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dvse {
    ///0: Disable interrupt request
    _0 = 0,
    ///1: Enable interrupt request
    _1 = 1,
}
impl From<Dvse> for bool {
    #[inline(always)]
    fn from(variant: Dvse) -> Self {
        variant as u8 != 0
    }
}
///Field `DVSE` reader - Device State Transition Interrupt Request Enable
pub type DvseR = crate::BitReader<Dvse>;
impl DvseR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dvse {
        match self.bits {
            false => Dvse::_0,
            true => Dvse::_1,
        }
    }
    ///Disable interrupt request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dvse::_0
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dvse::_1
    }
}
///Field `DVSE` writer - Device State Transition Interrupt Request Enable
pub type DvseW<'a, REG> = crate::BitWriter<'a, REG, Dvse>;
impl<'a, REG> DvseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable interrupt request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dvse::_0)
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dvse::_1)
    }
}
/**Frame Number Update Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sofe {
    ///0: Disable interrupt request
    _0 = 0,
    ///1: Enable interrupt request
    _1 = 1,
}
impl From<Sofe> for bool {
    #[inline(always)]
    fn from(variant: Sofe) -> Self {
        variant as u8 != 0
    }
}
///Field `SOFE` reader - Frame Number Update Interrupt Request Enable
pub type SofeR = crate::BitReader<Sofe>;
impl SofeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sofe {
        match self.bits {
            false => Sofe::_0,
            true => Sofe::_1,
        }
    }
    ///Disable interrupt request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sofe::_0
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sofe::_1
    }
}
///Field `SOFE` writer - Frame Number Update Interrupt Request Enable
pub type SofeW<'a, REG> = crate::BitWriter<'a, REG, Sofe>;
impl<'a, REG> SofeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable interrupt request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sofe::_0)
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sofe::_1)
    }
}
/**Resume Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rsme {
    ///0: Disable interrupt request
    _0 = 0,
    ///1: Enable interrupt request
    _1 = 1,
}
impl From<Rsme> for bool {
    #[inline(always)]
    fn from(variant: Rsme) -> Self {
        variant as u8 != 0
    }
}
///Field `RSME` reader - Resume Interrupt Request Enable
pub type RsmeR = crate::BitReader<Rsme>;
impl RsmeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rsme {
        match self.bits {
            false => Rsme::_0,
            true => Rsme::_1,
        }
    }
    ///Disable interrupt request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rsme::_0
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rsme::_1
    }
}
///Field `RSME` writer - Resume Interrupt Request Enable
pub type RsmeW<'a, REG> = crate::BitWriter<'a, REG, Rsme>;
impl<'a, REG> RsmeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable interrupt request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rsme::_0)
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rsme::_1)
    }
}
/**VBUS Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vbse {
    ///0: Disable interrupt request
    _0 = 0,
    ///1: Enable interrupt request
    _1 = 1,
}
impl From<Vbse> for bool {
    #[inline(always)]
    fn from(variant: Vbse) -> Self {
        variant as u8 != 0
    }
}
///Field `VBSE` reader - VBUS Interrupt Request Enable
pub type VbseR = crate::BitReader<Vbse>;
impl VbseR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Vbse {
        match self.bits {
            false => Vbse::_0,
            true => Vbse::_1,
        }
    }
    ///Disable interrupt request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Vbse::_0
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Vbse::_1
    }
}
///Field `VBSE` writer - VBUS Interrupt Request Enable
pub type VbseW<'a, REG> = crate::BitWriter<'a, REG, Vbse>;
impl<'a, REG> VbseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable interrupt request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Vbse::_0)
    }
    ///Enable interrupt request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Vbse::_1)
    }
}
impl R {
    ///Bit 8 - Buffer Ready Interrupt Request Enable
    #[inline(always)]
    pub fn brdye(&self) -> BrdyeR {
        BrdyeR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Buffer Not Ready Response Interrupt Request Enable
    #[inline(always)]
    pub fn nrdye(&self) -> NrdyeR {
        NrdyeR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Buffer Empty Interrupt Request Enable
    #[inline(always)]
    pub fn bempe(&self) -> BempeR {
        BempeR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Control Transfer Stage Transition Interrupt Request Enable
    #[inline(always)]
    pub fn ctre(&self) -> CtreR {
        CtreR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Device State Transition Interrupt Request Enable
    #[inline(always)]
    pub fn dvse(&self) -> DvseR {
        DvseR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Frame Number Update Interrupt Request Enable
    #[inline(always)]
    pub fn sofe(&self) -> SofeR {
        SofeR::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Resume Interrupt Request Enable
    #[inline(always)]
    pub fn rsme(&self) -> RsmeR {
        RsmeR::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - VBUS Interrupt Request Enable
    #[inline(always)]
    pub fn vbse(&self) -> VbseR {
        VbseR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTENB0")
            .field("brdye", &self.brdye())
            .field("nrdye", &self.nrdye())
            .field("bempe", &self.bempe())
            .field("ctre", &self.ctre())
            .field("dvse", &self.dvse())
            .field("sofe", &self.sofe())
            .field("rsme", &self.rsme())
            .field("vbse", &self.vbse())
            .finish()
    }
}
impl W {
    ///Bit 8 - Buffer Ready Interrupt Request Enable
    #[inline(always)]
    pub fn brdye(&mut self) -> BrdyeW<Intenb0Spec> {
        BrdyeW::new(self, 8)
    }
    ///Bit 9 - Buffer Not Ready Response Interrupt Request Enable
    #[inline(always)]
    pub fn nrdye(&mut self) -> NrdyeW<Intenb0Spec> {
        NrdyeW::new(self, 9)
    }
    ///Bit 10 - Buffer Empty Interrupt Request Enable
    #[inline(always)]
    pub fn bempe(&mut self) -> BempeW<Intenb0Spec> {
        BempeW::new(self, 10)
    }
    ///Bit 11 - Control Transfer Stage Transition Interrupt Request Enable
    #[inline(always)]
    pub fn ctre(&mut self) -> CtreW<Intenb0Spec> {
        CtreW::new(self, 11)
    }
    ///Bit 12 - Device State Transition Interrupt Request Enable
    #[inline(always)]
    pub fn dvse(&mut self) -> DvseW<Intenb0Spec> {
        DvseW::new(self, 12)
    }
    ///Bit 13 - Frame Number Update Interrupt Request Enable
    #[inline(always)]
    pub fn sofe(&mut self) -> SofeW<Intenb0Spec> {
        SofeW::new(self, 13)
    }
    ///Bit 14 - Resume Interrupt Request Enable
    #[inline(always)]
    pub fn rsme(&mut self) -> RsmeW<Intenb0Spec> {
        RsmeW::new(self, 14)
    }
    ///Bit 15 - VBUS Interrupt Request Enable
    #[inline(always)]
    pub fn vbse(&mut self) -> VbseW<Intenb0Spec> {
        VbseW::new(self, 15)
    }
}
/**Interrupt Enable Register 0

You can [`read`](crate::Reg::read) this register and get [`intenb0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenb0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Intenb0Spec;
impl crate::RegisterSpec for Intenb0Spec {
    type Ux = u16;
}
///`read()` method returns [`intenb0::R`](R) reader structure
impl crate::Readable for Intenb0Spec {}
///`write(|w| ..)` method takes [`intenb0::W`](W) writer structure
impl crate::Writable for Intenb0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets INTENB0 to value 0
impl crate::Resettable for Intenb0Spec {}
