///Register `ADCMPCR` reader
pub type R = crate::R<AdcmpcrSpec>;
///Register `ADCMPCR` writer
pub type W = crate::W<AdcmpcrSpec>;
/**Window A/B Composite Conditions Setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmpab {
    ///0: Output ADC12i_WCMPM (i = 0, 1) when window A OR window B comparison conditions are met. Otherwise, output ADC12i_WCMPUM (i = 0, 1).
    _00 = 0,
    ///1: Output ADC12i_WCMPM (i = 0, 1) when window A EXOR window B comparison conditions are met. Otherwise, output ADC12i_WCMPUM (i = 0, 1).
    _01 = 1,
    ///2: Output ADC12i_WCMPM (i = 0, 1) when window A AND window B comparison conditions are met. Otherwise, output ADC12i_WCMPUM (i = 0, 1).
    _10 = 2,
    ///3: Setting prohibited.
    _11 = 3,
}
impl From<Cmpab> for u8 {
    #[inline(always)]
    fn from(variant: Cmpab) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmpab {
    type Ux = u8;
}
impl crate::IsEnum for Cmpab {}
///Field `CMPAB` reader - Window A/B Composite Conditions Setting
pub type CmpabR = crate::FieldReader<Cmpab>;
impl CmpabR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmpab {
        match self.bits {
            0 => Cmpab::_00,
            1 => Cmpab::_01,
            2 => Cmpab::_10,
            3 => Cmpab::_11,
            _ => unreachable!(),
        }
    }
    ///Output ADC12i_WCMPM (i = 0, 1) when window A OR window B comparison conditions are met. Otherwise, output ADC12i_WCMPUM (i = 0, 1).
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Cmpab::_00
    }
    ///Output ADC12i_WCMPM (i = 0, 1) when window A EXOR window B comparison conditions are met. Otherwise, output ADC12i_WCMPUM (i = 0, 1).
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Cmpab::_01
    }
    ///Output ADC12i_WCMPM (i = 0, 1) when window A AND window B comparison conditions are met. Otherwise, output ADC12i_WCMPUM (i = 0, 1).
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Cmpab::_10
    }
    ///Setting prohibited.
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Cmpab::_11
    }
}
///Field `CMPAB` writer - Window A/B Composite Conditions Setting
pub type CmpabW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cmpab, crate::Safe>;
impl<'a, REG> CmpabW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Output ADC12i_WCMPM (i = 0, 1) when window A OR window B comparison conditions are met. Otherwise, output ADC12i_WCMPUM (i = 0, 1).
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpab::_00)
    }
    ///Output ADC12i_WCMPM (i = 0, 1) when window A EXOR window B comparison conditions are met. Otherwise, output ADC12i_WCMPUM (i = 0, 1).
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpab::_01)
    }
    ///Output ADC12i_WCMPM (i = 0, 1) when window A AND window B comparison conditions are met. Otherwise, output ADC12i_WCMPUM (i = 0, 1).
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpab::_10)
    }
    ///Setting prohibited.
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpab::_11)
    }
}
/**Compare Window B Operation Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpbe {
    ///0: Disable compare window B operation. Disable ADC12i_WCMPM (i = 0, 1) and ADC12i_WCMPUM (i = 0, 1) outputs.
    _0 = 0,
    ///1: Enable compare window B operation.
    _1 = 1,
}
impl From<Cmpbe> for bool {
    #[inline(always)]
    fn from(variant: Cmpbe) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPBE` reader - Compare Window B Operation Enable
pub type CmpbeR = crate::BitReader<Cmpbe>;
impl CmpbeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmpbe {
        match self.bits {
            false => Cmpbe::_0,
            true => Cmpbe::_1,
        }
    }
    ///Disable compare window B operation. Disable ADC12i_WCMPM (i = 0, 1) and ADC12i_WCMPUM (i = 0, 1) outputs.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpbe::_0
    }
    ///Enable compare window B operation.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpbe::_1
    }
}
///Field `CMPBE` writer - Compare Window B Operation Enable
pub type CmpbeW<'a, REG> = crate::BitWriter<'a, REG, Cmpbe>;
impl<'a, REG> CmpbeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable compare window B operation. Disable ADC12i_WCMPM (i = 0, 1) and ADC12i_WCMPUM (i = 0, 1) outputs.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpbe::_0)
    }
    ///Enable compare window B operation.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpbe::_1)
    }
}
/**Compare Window A Operation Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpae {
    ///0: Disable compare window A operation. Disable ADC12i_WCMPM (i = 0, 1) and ADC12i_WCMPUM (i = 0, 1) outputs.
    _0 = 0,
    ///1: Enable compare window A operation.
    _1 = 1,
}
impl From<Cmpae> for bool {
    #[inline(always)]
    fn from(variant: Cmpae) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPAE` reader - Compare Window A Operation Enable
pub type CmpaeR = crate::BitReader<Cmpae>;
impl CmpaeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmpae {
        match self.bits {
            false => Cmpae::_0,
            true => Cmpae::_1,
        }
    }
    ///Disable compare window A operation. Disable ADC12i_WCMPM (i = 0, 1) and ADC12i_WCMPUM (i = 0, 1) outputs.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpae::_0
    }
    ///Enable compare window A operation.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpae::_1
    }
}
///Field `CMPAE` writer - Compare Window A Operation Enable
pub type CmpaeW<'a, REG> = crate::BitWriter<'a, REG, Cmpae>;
impl<'a, REG> CmpaeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable compare window A operation. Disable ADC12i_WCMPM (i = 0, 1) and ADC12i_WCMPUM (i = 0, 1) outputs.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpae::_0)
    }
    ///Enable compare window A operation.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpae::_1)
    }
}
/**Compare B Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpbie {
    ///0: Disable ADC12i_CMPBI (i = 0, 1) interrupt when comparison conditions (window B) are met.
    _0 = 0,
    ///1: Enable ADC12i_CMPBI (i = 0, 1) interrupt when comparison conditions (window B) are met.
    _1 = 1,
}
impl From<Cmpbie> for bool {
    #[inline(always)]
    fn from(variant: Cmpbie) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPBIE` reader - Compare B Interrupt Enable
pub type CmpbieR = crate::BitReader<Cmpbie>;
impl CmpbieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmpbie {
        match self.bits {
            false => Cmpbie::_0,
            true => Cmpbie::_1,
        }
    }
    ///Disable ADC12i_CMPBI (i = 0, 1) interrupt when comparison conditions (window B) are met.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpbie::_0
    }
    ///Enable ADC12i_CMPBI (i = 0, 1) interrupt when comparison conditions (window B) are met.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpbie::_1
    }
}
///Field `CMPBIE` writer - Compare B Interrupt Enable
pub type CmpbieW<'a, REG> = crate::BitWriter<'a, REG, Cmpbie>;
impl<'a, REG> CmpbieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable ADC12i_CMPBI (i = 0, 1) interrupt when comparison conditions (window B) are met.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpbie::_0)
    }
    ///Enable ADC12i_CMPBI (i = 0, 1) interrupt when comparison conditions (window B) are met.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpbie::_1)
    }
}
/**Window Function Setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wcmpe {
    ///0: Disable window function Window A and window B operate as a comparator to compare the single value on the lower side with the A/D conversion result.
    _0 = 0,
    ///1: Enable window function Window A and window B operate as a comparator to compare the two values on the upper and lower sides with the A/D conversion result.
    _1 = 1,
}
impl From<Wcmpe> for bool {
    #[inline(always)]
    fn from(variant: Wcmpe) -> Self {
        variant as u8 != 0
    }
}
///Field `WCMPE` reader - Window Function Setting
pub type WcmpeR = crate::BitReader<Wcmpe>;
impl WcmpeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Wcmpe {
        match self.bits {
            false => Wcmpe::_0,
            true => Wcmpe::_1,
        }
    }
    ///Disable window function Window A and window B operate as a comparator to compare the single value on the lower side with the A/D conversion result.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wcmpe::_0
    }
    ///Enable window function Window A and window B operate as a comparator to compare the two values on the upper and lower sides with the A/D conversion result.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wcmpe::_1
    }
}
///Field `WCMPE` writer - Window Function Setting
pub type WcmpeW<'a, REG> = crate::BitWriter<'a, REG, Wcmpe>;
impl<'a, REG> WcmpeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable window function Window A and window B operate as a comparator to compare the single value on the lower side with the A/D conversion result.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wcmpe::_0)
    }
    ///Enable window function Window A and window B operate as a comparator to compare the two values on the upper and lower sides with the A/D conversion result.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wcmpe::_1)
    }
}
/**Compare A Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpaie {
    ///0: Disable ADC12i_CMPAI (i = 0, 1) interrupt when comparison conditions (window A) are met.
    _0 = 0,
    ///1: Enable ADC12i_CMPAI (i = 0, 1) interrupt when comparison conditions (window A) are met.
    _1 = 1,
}
impl From<Cmpaie> for bool {
    #[inline(always)]
    fn from(variant: Cmpaie) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPAIE` reader - Compare A Interrupt Enable
pub type CmpaieR = crate::BitReader<Cmpaie>;
impl CmpaieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmpaie {
        match self.bits {
            false => Cmpaie::_0,
            true => Cmpaie::_1,
        }
    }
    ///Disable ADC12i_CMPAI (i = 0, 1) interrupt when comparison conditions (window A) are met.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpaie::_0
    }
    ///Enable ADC12i_CMPAI (i = 0, 1) interrupt when comparison conditions (window A) are met.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpaie::_1
    }
}
///Field `CMPAIE` writer - Compare A Interrupt Enable
pub type CmpaieW<'a, REG> = crate::BitWriter<'a, REG, Cmpaie>;
impl<'a, REG> CmpaieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable ADC12i_CMPAI (i = 0, 1) interrupt when comparison conditions (window A) are met.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpaie::_0)
    }
    ///Enable ADC12i_CMPAI (i = 0, 1) interrupt when comparison conditions (window A) are met.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpaie::_1)
    }
}
impl R {
    ///Bits 0:1 - Window A/B Composite Conditions Setting
    #[inline(always)]
    pub fn cmpab(&self) -> CmpabR {
        CmpabR::new((self.bits & 3) as u8)
    }
    ///Bit 9 - Compare Window B Operation Enable
    #[inline(always)]
    pub fn cmpbe(&self) -> CmpbeR {
        CmpbeR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - Compare Window A Operation Enable
    #[inline(always)]
    pub fn cmpae(&self) -> CmpaeR {
        CmpaeR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 13 - Compare B Interrupt Enable
    #[inline(always)]
    pub fn cmpbie(&self) -> CmpbieR {
        CmpbieR::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Window Function Setting
    #[inline(always)]
    pub fn wcmpe(&self) -> WcmpeR {
        WcmpeR::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Compare A Interrupt Enable
    #[inline(always)]
    pub fn cmpaie(&self) -> CmpaieR {
        CmpaieR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCMPCR")
            .field("cmpab", &self.cmpab())
            .field("cmpbe", &self.cmpbe())
            .field("cmpae", &self.cmpae())
            .field("cmpbie", &self.cmpbie())
            .field("wcmpe", &self.wcmpe())
            .field("cmpaie", &self.cmpaie())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Window A/B Composite Conditions Setting
    #[inline(always)]
    pub fn cmpab(&mut self) -> CmpabW<AdcmpcrSpec> {
        CmpabW::new(self, 0)
    }
    ///Bit 9 - Compare Window B Operation Enable
    #[inline(always)]
    pub fn cmpbe(&mut self) -> CmpbeW<AdcmpcrSpec> {
        CmpbeW::new(self, 9)
    }
    ///Bit 11 - Compare Window A Operation Enable
    #[inline(always)]
    pub fn cmpae(&mut self) -> CmpaeW<AdcmpcrSpec> {
        CmpaeW::new(self, 11)
    }
    ///Bit 13 - Compare B Interrupt Enable
    #[inline(always)]
    pub fn cmpbie(&mut self) -> CmpbieW<AdcmpcrSpec> {
        CmpbieW::new(self, 13)
    }
    ///Bit 14 - Window Function Setting
    #[inline(always)]
    pub fn wcmpe(&mut self) -> WcmpeW<AdcmpcrSpec> {
        WcmpeW::new(self, 14)
    }
    ///Bit 15 - Compare A Interrupt Enable
    #[inline(always)]
    pub fn cmpaie(&mut self) -> CmpaieW<AdcmpcrSpec> {
        CmpaieW::new(self, 15)
    }
}
/**A/D Compare Function Control Register

You can [`read`](crate::Reg::read) this register and get [`adcmpcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AdcmpcrSpec;
impl crate::RegisterSpec for AdcmpcrSpec {
    type Ux = u16;
}
///`read()` method returns [`adcmpcr::R`](R) reader structure
impl crate::Readable for AdcmpcrSpec {}
///`write(|w| ..)` method takes [`adcmpcr::W`](W) writer structure
impl crate::Writable for AdcmpcrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADCMPCR to value 0
impl crate::Resettable for AdcmpcrSpec {}
