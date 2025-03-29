///Register `ADCMPLER` reader
pub type R = crate::R<AdcmplerSpec>;
///Register `ADCMPLER` writer
pub type W = crate::W<AdcmplerSpec>;
/**Compare Window A Temperature Sensor Output Comparison Condition Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpltsa {
    ///0: When window function is disabled (ADCMPCR.WCMPE = 0) : ADCMPDR0 value > A/D-converted value Compare Window A Temperature Sensor Output Comparison Condition Select When window function is enabled (ADCMPCR.WCMPE = 1) : Compare Window A Temperature Sensor Output Comparison Condition A/D-converted value < ADCMPDR0 value, or A/D-converted value > ADCMPDR1 value
    _0 = 0,
    ///1: When window function is disabled (ADCMPCR.WCMPE = 0) : ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1) : ADCMPDR0 value < A/D-converted value < ADCMPDR1 value
    _1 = 1,
}
impl From<Cmpltsa> for bool {
    #[inline(always)]
    fn from(variant: Cmpltsa) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPLTSA` reader - Compare Window A Temperature Sensor Output Comparison Condition Select
pub type CmpltsaR = crate::BitReader<Cmpltsa>;
impl CmpltsaR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmpltsa {
        match self.bits {
            false => Cmpltsa::_0,
            true => Cmpltsa::_1,
        }
    }
    ///When window function is disabled (ADCMPCR.WCMPE = 0) : ADCMPDR0 value > A/D-converted value Compare Window A Temperature Sensor Output Comparison Condition Select When window function is enabled (ADCMPCR.WCMPE = 1) : Compare Window A Temperature Sensor Output Comparison Condition A/D-converted value < ADCMPDR0 value, or A/D-converted value > ADCMPDR1 value
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpltsa::_0
    }
    ///When window function is disabled (ADCMPCR.WCMPE = 0) : ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1) : ADCMPDR0 value < A/D-converted value < ADCMPDR1 value
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpltsa::_1
    }
}
///Field `CMPLTSA` writer - Compare Window A Temperature Sensor Output Comparison Condition Select
pub type CmpltsaW<'a, REG> = crate::BitWriter<'a, REG, Cmpltsa>;
impl<'a, REG> CmpltsaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///When window function is disabled (ADCMPCR.WCMPE = 0) : ADCMPDR0 value > A/D-converted value Compare Window A Temperature Sensor Output Comparison Condition Select When window function is enabled (ADCMPCR.WCMPE = 1) : Compare Window A Temperature Sensor Output Comparison Condition A/D-converted value < ADCMPDR0 value, or A/D-converted value > ADCMPDR1 value
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpltsa::_0)
    }
    ///When window function is disabled (ADCMPCR.WCMPE = 0) : ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1) : ADCMPDR0 value < A/D-converted value < ADCMPDR1 value
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpltsa::_1)
    }
}
/**Compare Window A Internal Reference Voltage Comparison Condition Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmploca {
    ///0: When window function is disabled (ADCMPCR.WCMPE = 0) : ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or A/D-converted value > ADCMPDR1 value
    _0 = 0,
    ///1: When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value
    _1 = 1,
}
impl From<Cmploca> for bool {
    #[inline(always)]
    fn from(variant: Cmploca) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPLOCA` reader - Compare Window A Internal Reference Voltage Comparison Condition Select
pub type CmplocaR = crate::BitReader<Cmploca>;
impl CmplocaR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmploca {
        match self.bits {
            false => Cmploca::_0,
            true => Cmploca::_1,
        }
    }
    ///When window function is disabled (ADCMPCR.WCMPE = 0) : ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or A/D-converted value > ADCMPDR1 value
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmploca::_0
    }
    ///When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmploca::_1
    }
}
///Field `CMPLOCA` writer - Compare Window A Internal Reference Voltage Comparison Condition Select
pub type CmplocaW<'a, REG> = crate::BitWriter<'a, REG, Cmploca>;
impl<'a, REG> CmplocaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///When window function is disabled (ADCMPCR.WCMPE = 0) : ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or A/D-converted value > ADCMPDR1 value
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmploca::_0)
    }
    ///When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmploca::_1)
    }
}
impl R {
    ///Bit 0 - Compare Window A Temperature Sensor Output Comparison Condition Select
    #[inline(always)]
    pub fn cmpltsa(&self) -> CmpltsaR {
        CmpltsaR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Compare Window A Internal Reference Voltage Comparison Condition Select
    #[inline(always)]
    pub fn cmploca(&self) -> CmplocaR {
        CmplocaR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCMPLER")
            .field("cmpltsa", &self.cmpltsa())
            .field("cmploca", &self.cmploca())
            .finish()
    }
}
impl W {
    ///Bit 0 - Compare Window A Temperature Sensor Output Comparison Condition Select
    #[inline(always)]
    pub fn cmpltsa(&mut self) -> CmpltsaW<AdcmplerSpec> {
        CmpltsaW::new(self, 0)
    }
    ///Bit 1 - Compare Window A Internal Reference Voltage Comparison Condition Select
    #[inline(always)]
    pub fn cmploca(&mut self) -> CmplocaW<AdcmplerSpec> {
        CmplocaW::new(self, 1)
    }
}
/**A/D Compare Function Window A Extended Input Comparison Condition Setting Register

You can [`read`](crate::Reg::read) this register and get [`adcmpler::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpler::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AdcmplerSpec;
impl crate::RegisterSpec for AdcmplerSpec {
    type Ux = u8;
}
///`read()` method returns [`adcmpler::R`](R) reader structure
impl crate::Readable for AdcmplerSpec {}
///`write(|w| ..)` method takes [`adcmpler::W`](W) writer structure
impl crate::Writable for AdcmplerSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADCMPLER to value 0
impl crate::Resettable for AdcmplerSpec {}
