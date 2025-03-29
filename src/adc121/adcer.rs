///Register `ADCER` reader
pub type R = crate::R<AdcerSpec>;
///Register `ADCER` writer
pub type W = crate::W<AdcerSpec>;
/**

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adprc {
    ///0: 12-bit accuracy
    _00 = 0,
    ///1: 10-bit accuracy
    _01 = 1,
    ///2: 8-bit accuracy
    _10 = 2,
    ///3: Setting prohibited
    _11 = 3,
}
impl From<Adprc> for u8 {
    #[inline(always)]
    fn from(variant: Adprc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adprc {
    type Ux = u8;
}
impl crate::IsEnum for Adprc {}
///Field `ADPRC` reader -
pub type AdprcR = crate::FieldReader<Adprc>;
impl AdprcR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Adprc {
        match self.bits {
            0 => Adprc::_00,
            1 => Adprc::_01,
            2 => Adprc::_10,
            3 => Adprc::_11,
            _ => unreachable!(),
        }
    }
    ///12-bit accuracy
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Adprc::_00
    }
    ///10-bit accuracy
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Adprc::_01
    }
    ///8-bit accuracy
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Adprc::_10
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Adprc::_11
    }
}
///Field `ADPRC` writer -
pub type AdprcW<'a, REG> = crate::FieldWriter<'a, REG, 2, Adprc, crate::Safe>;
impl<'a, REG> AdprcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///12-bit accuracy
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Adprc::_00)
    }
    ///10-bit accuracy
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Adprc::_01)
    }
    ///8-bit accuracy
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Adprc::_10)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Adprc::_11)
    }
}
/**A/D Data Register Automatic Clearing Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ace {
    ///0: Disable automatic clearing
    _0 = 0,
    ///1: Enable automatic clearing
    _1 = 1,
}
impl From<Ace> for bool {
    #[inline(always)]
    fn from(variant: Ace) -> Self {
        variant as u8 != 0
    }
}
///Field `ACE` reader - A/D Data Register Automatic Clearing Enable
pub type AceR = crate::BitReader<Ace>;
impl AceR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ace {
        match self.bits {
            false => Ace::_0,
            true => Ace::_1,
        }
    }
    ///Disable automatic clearing
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ace::_0
    }
    ///Enable automatic clearing
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ace::_1
    }
}
///Field `ACE` writer - A/D Data Register Automatic Clearing Enable
pub type AceW<'a, REG> = crate::BitWriter<'a, REG, Ace>;
impl<'a, REG> AceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable automatic clearing
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ace::_0)
    }
    ///Enable automatic clearing
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ace::_1)
    }
}
/**Self-Diagnosis Conversion Voltage Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Diagval {
    ///0: Setting prohibited when self-diagnosis is enabled
    _00 = 0,
    ///1: 0 volts
    _01 = 1,
    ///2: Reference voltage × 1/2
    _10 = 2,
    ///3: Reference voltage
    _11 = 3,
}
impl From<Diagval> for u8 {
    #[inline(always)]
    fn from(variant: Diagval) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Diagval {
    type Ux = u8;
}
impl crate::IsEnum for Diagval {}
///Field `DIAGVAL` reader - Self-Diagnosis Conversion Voltage Select
pub type DiagvalR = crate::FieldReader<Diagval>;
impl DiagvalR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Diagval {
        match self.bits {
            0 => Diagval::_00,
            1 => Diagval::_01,
            2 => Diagval::_10,
            3 => Diagval::_11,
            _ => unreachable!(),
        }
    }
    ///Setting prohibited when self-diagnosis is enabled
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Diagval::_00
    }
    ///0 volts
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Diagval::_01
    }
    ///Reference voltage × 1/2
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Diagval::_10
    }
    ///Reference voltage
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Diagval::_11
    }
}
///Field `DIAGVAL` writer - Self-Diagnosis Conversion Voltage Select
pub type DiagvalW<'a, REG> = crate::FieldWriter<'a, REG, 2, Diagval, crate::Safe>;
impl<'a, REG> DiagvalW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Setting prohibited when self-diagnosis is enabled
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Diagval::_00)
    }
    ///0 volts
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Diagval::_01)
    }
    ///Reference voltage × 1/2
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Diagval::_10)
    }
    ///Reference voltage
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Diagval::_11)
    }
}
/**Self-Diagnosis Mode Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Diagld {
    ///0: Select rotation mode for self-diagnosis voltage
    _0 = 0,
    ///1: Select mixed mode for self-diagnosis voltage
    _1 = 1,
}
impl From<Diagld> for bool {
    #[inline(always)]
    fn from(variant: Diagld) -> Self {
        variant as u8 != 0
    }
}
///Field `DIAGLD` reader - Self-Diagnosis Mode Select
pub type DiagldR = crate::BitReader<Diagld>;
impl DiagldR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Diagld {
        match self.bits {
            false => Diagld::_0,
            true => Diagld::_1,
        }
    }
    ///Select rotation mode for self-diagnosis voltage
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Diagld::_0
    }
    ///Select mixed mode for self-diagnosis voltage
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Diagld::_1
    }
}
///Field `DIAGLD` writer - Self-Diagnosis Mode Select
pub type DiagldW<'a, REG> = crate::BitWriter<'a, REG, Diagld>;
impl<'a, REG> DiagldW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Select rotation mode for self-diagnosis voltage
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Diagld::_0)
    }
    ///Select mixed mode for self-diagnosis voltage
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Diagld::_1)
    }
}
/**Self-Diagnosis Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Diagm {
    ///0: Disable ADC12 self-diagnosis
    _0 = 0,
    ///1: Enable ADC12 self-diagnosis
    _1 = 1,
}
impl From<Diagm> for bool {
    #[inline(always)]
    fn from(variant: Diagm) -> Self {
        variant as u8 != 0
    }
}
///Field `DIAGM` reader - Self-Diagnosis Enable
pub type DiagmR = crate::BitReader<Diagm>;
impl DiagmR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Diagm {
        match self.bits {
            false => Diagm::_0,
            true => Diagm::_1,
        }
    }
    ///Disable ADC12 self-diagnosis
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Diagm::_0
    }
    ///Enable ADC12 self-diagnosis
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Diagm::_1
    }
}
///Field `DIAGM` writer - Self-Diagnosis Enable
pub type DiagmW<'a, REG> = crate::BitWriter<'a, REG, Diagm>;
impl<'a, REG> DiagmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable ADC12 self-diagnosis
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Diagm::_0)
    }
    ///Enable ADC12 self-diagnosis
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Diagm::_1)
    }
}
/**A/D Data Register Format Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adrfmt {
    ///0: Select right-justified for the A/D data register format
    _0 = 0,
    ///1: Select left-justified for the A/D data register format
    _1 = 1,
}
impl From<Adrfmt> for bool {
    #[inline(always)]
    fn from(variant: Adrfmt) -> Self {
        variant as u8 != 0
    }
}
///Field `ADRFMT` reader - A/D Data Register Format Select
pub type AdrfmtR = crate::BitReader<Adrfmt>;
impl AdrfmtR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Adrfmt {
        match self.bits {
            false => Adrfmt::_0,
            true => Adrfmt::_1,
        }
    }
    ///Select right-justified for the A/D data register format
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Adrfmt::_0
    }
    ///Select left-justified for the A/D data register format
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Adrfmt::_1
    }
}
///Field `ADRFMT` writer - A/D Data Register Format Select
pub type AdrfmtW<'a, REG> = crate::BitWriter<'a, REG, Adrfmt>;
impl<'a, REG> AdrfmtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Select right-justified for the A/D data register format
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Adrfmt::_0)
    }
    ///Select left-justified for the A/D data register format
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Adrfmt::_1)
    }
}
impl R {
    ///Bits 1:2
    #[inline(always)]
    pub fn adprc(&self) -> AdprcR {
        AdprcR::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bit 5 - A/D Data Register Automatic Clearing Enable
    #[inline(always)]
    pub fn ace(&self) -> AceR {
        AceR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 8:9 - Self-Diagnosis Conversion Voltage Select
    #[inline(always)]
    pub fn diagval(&self) -> DiagvalR {
        DiagvalR::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 10 - Self-Diagnosis Mode Select
    #[inline(always)]
    pub fn diagld(&self) -> DiagldR {
        DiagldR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Self-Diagnosis Enable
    #[inline(always)]
    pub fn diagm(&self) -> DiagmR {
        DiagmR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 15 - A/D Data Register Format Select
    #[inline(always)]
    pub fn adrfmt(&self) -> AdrfmtR {
        AdrfmtR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCER")
            .field("adprc", &self.adprc())
            .field("ace", &self.ace())
            .field("diagval", &self.diagval())
            .field("diagld", &self.diagld())
            .field("diagm", &self.diagm())
            .field("adrfmt", &self.adrfmt())
            .finish()
    }
}
impl W {
    ///Bits 1:2
    #[inline(always)]
    pub fn adprc(&mut self) -> AdprcW<AdcerSpec> {
        AdprcW::new(self, 1)
    }
    ///Bit 5 - A/D Data Register Automatic Clearing Enable
    #[inline(always)]
    pub fn ace(&mut self) -> AceW<AdcerSpec> {
        AceW::new(self, 5)
    }
    ///Bits 8:9 - Self-Diagnosis Conversion Voltage Select
    #[inline(always)]
    pub fn diagval(&mut self) -> DiagvalW<AdcerSpec> {
        DiagvalW::new(self, 8)
    }
    ///Bit 10 - Self-Diagnosis Mode Select
    #[inline(always)]
    pub fn diagld(&mut self) -> DiagldW<AdcerSpec> {
        DiagldW::new(self, 10)
    }
    ///Bit 11 - Self-Diagnosis Enable
    #[inline(always)]
    pub fn diagm(&mut self) -> DiagmW<AdcerSpec> {
        DiagmW::new(self, 11)
    }
    ///Bit 15 - A/D Data Register Format Select
    #[inline(always)]
    pub fn adrfmt(&mut self) -> AdrfmtW<AdcerSpec> {
        AdrfmtW::new(self, 15)
    }
}
/**A/D Control Extended Register

You can [`read`](crate::Reg::read) this register and get [`adcer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AdcerSpec;
impl crate::RegisterSpec for AdcerSpec {
    type Ux = u16;
}
///`read()` method returns [`adcer::R`](R) reader structure
impl crate::Readable for AdcerSpec {}
///`write(|w| ..)` method takes [`adcer::W`](W) writer structure
impl crate::Writable for AdcerSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADCER to value 0
impl crate::Resettable for AdcerSpec {}
