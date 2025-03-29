///Register `ADADC` reader
pub type R = crate::R<AdadcSpec>;
///Register `ADADC` writer
pub type W = crate::W<AdadcSpec>;
/**Addition/Average Count Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adc {
    ///0: 1-time conversion (no addition, same as normal conversion)
    _000 = 0,
    ///1: 2-time conversion (1 addition)
    _001 = 1,
    ///2: 3-time conversion (2 additions)
    _010 = 2,
    ///3: 4-time conversion (3 additions)
    _011 = 3,
    ///5: 16-time conversion (15 additions)
    _101 = 5,
    ///4: Setting prohibited
    Others = 4,
}
impl From<Adc> for u8 {
    #[inline(always)]
    fn from(variant: Adc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adc {
    type Ux = u8;
}
impl crate::IsEnum for Adc {}
///Field `ADC` reader - Addition/Average Count Select
pub type AdcR = crate::FieldReader<Adc>;
impl AdcR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Adc {
        match self.bits {
            0 => Adc::_000,
            1 => Adc::_001,
            2 => Adc::_010,
            3 => Adc::_011,
            5 => Adc::_101,
            _ => Adc::Others,
        }
    }
    ///1-time conversion (no addition, same as normal conversion)
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Adc::_000
    }
    ///2-time conversion (1 addition)
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Adc::_001
    }
    ///3-time conversion (2 additions)
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Adc::_010
    }
    ///4-time conversion (3 additions)
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Adc::_011
    }
    ///16-time conversion (15 additions)
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Adc::_101
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Adc::Others)
    }
}
///Field `ADC` writer - Addition/Average Count Select
pub type AdcW<'a, REG> = crate::FieldWriter<'a, REG, 3, Adc, crate::Safe>;
impl<'a, REG> AdcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///1-time conversion (no addition, same as normal conversion)
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Adc::_000)
    }
    ///2-time conversion (1 addition)
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Adc::_001)
    }
    ///3-time conversion (2 additions)
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Adc::_010)
    }
    ///4-time conversion (3 additions)
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Adc::_011)
    }
    ///16-time conversion (15 additions)
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Adc::_101)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Adc::Others)
    }
}
/**Average Mode Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Avee {
    ///0: Enable addition mode
    _0 = 0,
    ///1: Enable average mode
    _1 = 1,
}
impl From<Avee> for bool {
    #[inline(always)]
    fn from(variant: Avee) -> Self {
        variant as u8 != 0
    }
}
///Field `AVEE` reader - Average Mode Select
pub type AveeR = crate::BitReader<Avee>;
impl AveeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Avee {
        match self.bits {
            false => Avee::_0,
            true => Avee::_1,
        }
    }
    ///Enable addition mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Avee::_0
    }
    ///Enable average mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Avee::_1
    }
}
///Field `AVEE` writer - Average Mode Select
pub type AveeW<'a, REG> = crate::BitWriter<'a, REG, Avee>;
impl<'a, REG> AveeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Enable addition mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Avee::_0)
    }
    ///Enable average mode
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Avee::_1)
    }
}
impl R {
    ///Bits 0:2 - Addition/Average Count Select
    #[inline(always)]
    pub fn adc(&self) -> AdcR {
        AdcR::new(self.bits & 7)
    }
    ///Bit 7 - Average Mode Select
    #[inline(always)]
    pub fn avee(&self) -> AveeR {
        AveeR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADADC")
            .field("adc", &self.adc())
            .field("avee", &self.avee())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Addition/Average Count Select
    #[inline(always)]
    pub fn adc(&mut self) -> AdcW<AdadcSpec> {
        AdcW::new(self, 0)
    }
    ///Bit 7 - Average Mode Select
    #[inline(always)]
    pub fn avee(&mut self) -> AveeW<AdadcSpec> {
        AveeW::new(self, 7)
    }
}
/**A/D-Converted Value Addition/Average Count Select Register

You can [`read`](crate::Reg::read) this register and get [`adadc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adadc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AdadcSpec;
impl crate::RegisterSpec for AdadcSpec {
    type Ux = u8;
}
///`read()` method returns [`adadc::R`](R) reader structure
impl crate::Readable for AdadcSpec {}
///`write(|w| ..)` method takes [`adadc::W`](W) writer structure
impl crate::Writable for AdadcSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADADC to value 0
impl crate::Resettable for AdadcSpec {}
