///Register `ADCMPBNSR` reader
pub type R = crate::R<AdcmpbnsrSpec>;
///Register `ADCMPBNSR` writer
pub type W = crate::W<AdcmpbnsrSpec>;
///Field `CMPCHB` reader - Compare Window B Channel Select
pub type CmpchbR = crate::FieldReader;
///Field `CMPCHB` writer - Compare Window B Channel Select
pub type CmpchbW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
/**Compare Window B Comparison Condition Setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmplb {
    ///0: When window function is disabled (ADCMPCR.WCMPE = 0): ADWINLLB value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADWINLLB value, or ADWINULB value < A/D-converted value
    _0 = 0,
    ///1: When window function is disabled (ADCMPCR.WCMPE = 0): ADWINLLB value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADWINLLB value < A/D-converted value < ADWINULB value
    _1 = 1,
}
impl From<Cmplb> for bool {
    #[inline(always)]
    fn from(variant: Cmplb) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPLB` reader - Compare Window B Comparison Condition Setting
pub type CmplbR = crate::BitReader<Cmplb>;
impl CmplbR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmplb {
        match self.bits {
            false => Cmplb::_0,
            true => Cmplb::_1,
        }
    }
    ///When window function is disabled (ADCMPCR.WCMPE = 0): ADWINLLB value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADWINLLB value, or ADWINULB value < A/D-converted value
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmplb::_0
    }
    ///When window function is disabled (ADCMPCR.WCMPE = 0): ADWINLLB value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADWINLLB value < A/D-converted value < ADWINULB value
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmplb::_1
    }
}
///Field `CMPLB` writer - Compare Window B Comparison Condition Setting
pub type CmplbW<'a, REG> = crate::BitWriter<'a, REG, Cmplb>;
impl<'a, REG> CmplbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///When window function is disabled (ADCMPCR.WCMPE = 0): ADWINLLB value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADWINLLB value, or ADWINULB value < A/D-converted value
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplb::_0)
    }
    ///When window function is disabled (ADCMPCR.WCMPE = 0): ADWINLLB value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADWINLLB value < A/D-converted value < ADWINULB value
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmplb::_1)
    }
}
impl R {
    ///Bits 0:5 - Compare Window B Channel Select
    #[inline(always)]
    pub fn cmpchb(&self) -> CmpchbR {
        CmpchbR::new(self.bits & 0x3f)
    }
    ///Bit 7 - Compare Window B Comparison Condition Setting
    #[inline(always)]
    pub fn cmplb(&self) -> CmplbR {
        CmplbR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCMPBNSR")
            .field("cmpchb", &self.cmpchb())
            .field("cmplb", &self.cmplb())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - Compare Window B Channel Select
    #[inline(always)]
    pub fn cmpchb(&mut self) -> CmpchbW<AdcmpbnsrSpec> {
        CmpchbW::new(self, 0)
    }
    ///Bit 7 - Compare Window B Comparison Condition Setting
    #[inline(always)]
    pub fn cmplb(&mut self) -> CmplbW<AdcmpbnsrSpec> {
        CmplbW::new(self, 7)
    }
}
/**A/D Compare Function Window B Channel Select Register

You can [`read`](crate::Reg::read) this register and get [`adcmpbnsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpbnsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AdcmpbnsrSpec;
impl crate::RegisterSpec for AdcmpbnsrSpec {
    type Ux = u8;
}
///`read()` method returns [`adcmpbnsr::R`](R) reader structure
impl crate::Readable for AdcmpbnsrSpec {}
///`write(|w| ..)` method takes [`adcmpbnsr::W`](W) writer structure
impl crate::Writable for AdcmpbnsrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADCMPBNSR to value 0
impl crate::Resettable for AdcmpbnsrSpec {}
