///Register `AGTCMSR` reader
pub type R = crate::R<AgtcmsrSpec>;
///Register `AGTCMSR` writer
pub type W = crate::W<AgtcmsrSpec>;
/**AGT Compare Match A Register Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcmea {
    ///0: AGT Compare match A register disabled
    _0 = 0,
    ///1: AGT Compare match A register enabled
    _1 = 1,
}
impl From<Tcmea> for bool {
    #[inline(always)]
    fn from(variant: Tcmea) -> Self {
        variant as u8 != 0
    }
}
///Field `TCMEA` reader - AGT Compare Match A Register Enable
pub type TcmeaR = crate::BitReader<Tcmea>;
impl TcmeaR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tcmea {
        match self.bits {
            false => Tcmea::_0,
            true => Tcmea::_1,
        }
    }
    ///AGT Compare match A register disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tcmea::_0
    }
    ///AGT Compare match A register enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tcmea::_1
    }
}
///Field `TCMEA` writer - AGT Compare Match A Register Enable
pub type TcmeaW<'a, REG> = crate::BitWriter<'a, REG, Tcmea>;
impl<'a, REG> TcmeaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AGT Compare match A register disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tcmea::_0)
    }
    ///AGT Compare match A register enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tcmea::_1)
    }
}
/**AGTOAn Pin Output Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Toea {
    ///0: AGTOAn pin output disabled
    _0 = 0,
    ///1: AGTOAn pin output enabled
    _1 = 1,
}
impl From<Toea> for bool {
    #[inline(always)]
    fn from(variant: Toea) -> Self {
        variant as u8 != 0
    }
}
///Field `TOEA` reader - AGTOAn Pin Output Enable
pub type ToeaR = crate::BitReader<Toea>;
impl ToeaR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Toea {
        match self.bits {
            false => Toea::_0,
            true => Toea::_1,
        }
    }
    ///AGTOAn pin output disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Toea::_0
    }
    ///AGTOAn pin output enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Toea::_1
    }
}
///Field `TOEA` writer - AGTOAn Pin Output Enable
pub type ToeaW<'a, REG> = crate::BitWriter<'a, REG, Toea>;
impl<'a, REG> ToeaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AGTOAn pin output disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Toea::_0)
    }
    ///AGTOAn pin output enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Toea::_1)
    }
}
/**AGTOAn Pin Polarity Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Topola {
    ///0: AGTOAn pin output is started on low. i.e. normal output
    _0 = 0,
    ///1: AGTOAn pin output is started on high. i.e. inverted output
    _1 = 1,
}
impl From<Topola> for bool {
    #[inline(always)]
    fn from(variant: Topola) -> Self {
        variant as u8 != 0
    }
}
///Field `TOPOLA` reader - AGTOAn Pin Polarity Select
pub type TopolaR = crate::BitReader<Topola>;
impl TopolaR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Topola {
        match self.bits {
            false => Topola::_0,
            true => Topola::_1,
        }
    }
    ///AGTOAn pin output is started on low. i.e. normal output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Topola::_0
    }
    ///AGTOAn pin output is started on high. i.e. inverted output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Topola::_1
    }
}
///Field `TOPOLA` writer - AGTOAn Pin Polarity Select
pub type TopolaW<'a, REG> = crate::BitWriter<'a, REG, Topola>;
impl<'a, REG> TopolaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AGTOAn pin output is started on low. i.e. normal output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Topola::_0)
    }
    ///AGTOAn pin output is started on high. i.e. inverted output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Topola::_1)
    }
}
/**AGT Compare Match B Register Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcmeb {
    ///0: Compare match B register disabled
    _0 = 0,
    ///1: Compare match B register enabled
    _1 = 1,
}
impl From<Tcmeb> for bool {
    #[inline(always)]
    fn from(variant: Tcmeb) -> Self {
        variant as u8 != 0
    }
}
///Field `TCMEB` reader - AGT Compare Match B Register Enable
pub type TcmebR = crate::BitReader<Tcmeb>;
impl TcmebR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tcmeb {
        match self.bits {
            false => Tcmeb::_0,
            true => Tcmeb::_1,
        }
    }
    ///Compare match B register disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tcmeb::_0
    }
    ///Compare match B register enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tcmeb::_1
    }
}
///Field `TCMEB` writer - AGT Compare Match B Register Enable
pub type TcmebW<'a, REG> = crate::BitWriter<'a, REG, Tcmeb>;
impl<'a, REG> TcmebW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Compare match B register disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tcmeb::_0)
    }
    ///Compare match B register enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tcmeb::_1)
    }
}
/**AGTOBn Pin Output Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Toeb {
    ///0: AGTOBn pin output disabled
    _0 = 0,
    ///1: AGTOBn pin output enabled
    _1 = 1,
}
impl From<Toeb> for bool {
    #[inline(always)]
    fn from(variant: Toeb) -> Self {
        variant as u8 != 0
    }
}
///Field `TOEB` reader - AGTOBn Pin Output Enable
pub type ToebR = crate::BitReader<Toeb>;
impl ToebR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Toeb {
        match self.bits {
            false => Toeb::_0,
            true => Toeb::_1,
        }
    }
    ///AGTOBn pin output disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Toeb::_0
    }
    ///AGTOBn pin output enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Toeb::_1
    }
}
///Field `TOEB` writer - AGTOBn Pin Output Enable
pub type ToebW<'a, REG> = crate::BitWriter<'a, REG, Toeb>;
impl<'a, REG> ToebW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AGTOBn pin output disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Toeb::_0)
    }
    ///AGTOBn pin output enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Toeb::_1)
    }
}
/**AGTOBn Pin Polarity Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Topolb {
    ///0: AGTOBn pin output is started on low. i.e. normal output
    _0 = 0,
    ///1: AGTOBn pin output is started on high. i.e. inverted output
    _1 = 1,
}
impl From<Topolb> for bool {
    #[inline(always)]
    fn from(variant: Topolb) -> Self {
        variant as u8 != 0
    }
}
///Field `TOPOLB` reader - AGTOBn Pin Polarity Select
pub type TopolbR = crate::BitReader<Topolb>;
impl TopolbR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Topolb {
        match self.bits {
            false => Topolb::_0,
            true => Topolb::_1,
        }
    }
    ///AGTOBn pin output is started on low. i.e. normal output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Topolb::_0
    }
    ///AGTOBn pin output is started on high. i.e. inverted output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Topolb::_1
    }
}
///Field `TOPOLB` writer - AGTOBn Pin Polarity Select
pub type TopolbW<'a, REG> = crate::BitWriter<'a, REG, Topolb>;
impl<'a, REG> TopolbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AGTOBn pin output is started on low. i.e. normal output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Topolb::_0)
    }
    ///AGTOBn pin output is started on high. i.e. inverted output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Topolb::_1)
    }
}
impl R {
    ///Bit 0 - AGT Compare Match A Register Enable
    #[inline(always)]
    pub fn tcmea(&self) -> TcmeaR {
        TcmeaR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - AGTOAn Pin Output Enable
    #[inline(always)]
    pub fn toea(&self) -> ToeaR {
        ToeaR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - AGTOAn Pin Polarity Select
    #[inline(always)]
    pub fn topola(&self) -> TopolaR {
        TopolaR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - AGT Compare Match B Register Enable
    #[inline(always)]
    pub fn tcmeb(&self) -> TcmebR {
        TcmebR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - AGTOBn Pin Output Enable
    #[inline(always)]
    pub fn toeb(&self) -> ToebR {
        ToebR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - AGTOBn Pin Polarity Select
    #[inline(always)]
    pub fn topolb(&self) -> TopolbR {
        TopolbR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AGTCMSR")
            .field("tcmea", &self.tcmea())
            .field("toea", &self.toea())
            .field("topola", &self.topola())
            .field("tcmeb", &self.tcmeb())
            .field("toeb", &self.toeb())
            .field("topolb", &self.topolb())
            .finish()
    }
}
impl W {
    ///Bit 0 - AGT Compare Match A Register Enable
    #[inline(always)]
    pub fn tcmea(&mut self) -> TcmeaW<AgtcmsrSpec> {
        TcmeaW::new(self, 0)
    }
    ///Bit 1 - AGTOAn Pin Output Enable
    #[inline(always)]
    pub fn toea(&mut self) -> ToeaW<AgtcmsrSpec> {
        ToeaW::new(self, 1)
    }
    ///Bit 2 - AGTOAn Pin Polarity Select
    #[inline(always)]
    pub fn topola(&mut self) -> TopolaW<AgtcmsrSpec> {
        TopolaW::new(self, 2)
    }
    ///Bit 4 - AGT Compare Match B Register Enable
    #[inline(always)]
    pub fn tcmeb(&mut self) -> TcmebW<AgtcmsrSpec> {
        TcmebW::new(self, 4)
    }
    ///Bit 5 - AGTOBn Pin Output Enable
    #[inline(always)]
    pub fn toeb(&mut self) -> ToebW<AgtcmsrSpec> {
        ToebW::new(self, 5)
    }
    ///Bit 6 - AGTOBn Pin Polarity Select
    #[inline(always)]
    pub fn topolb(&mut self) -> TopolbW<AgtcmsrSpec> {
        TopolbW::new(self, 6)
    }
}
/**AGT Compare Match Function Select Register

You can [`read`](crate::Reg::read) this register and get [`agtcmsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agtcmsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AgtcmsrSpec;
impl crate::RegisterSpec for AgtcmsrSpec {
    type Ux = u8;
}
///`read()` method returns [`agtcmsr::R`](R) reader structure
impl crate::Readable for AgtcmsrSpec {}
///`write(|w| ..)` method takes [`agtcmsr::W`](W) writer structure
impl crate::Writable for AgtcmsrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AGTCMSR to value 0
impl crate::Resettable for AgtcmsrSpec {}
