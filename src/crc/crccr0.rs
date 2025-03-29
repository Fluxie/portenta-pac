///Register `CRCCR0` reader
pub type R = crate::R<Crccr0Spec>;
///Register `CRCCR0` writer
pub type W = crate::W<Crccr0Spec>;
/**CRC Generating Polynomial Switching

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gps {
    ///1: 8-bit CRC-8 (X8 + X2 + X + 1)
    _001 = 1,
    ///2: 16-bit CRC-16 (X16 + X15 + X2 + 1)
    _010 = 2,
    ///3: 16-bit CRC-CCITT (X16 + X12 + X5 + 1)
    _011 = 3,
    ///4: 32-bit CRC-32 (X32 + X26 + X23 + X22 + X16 + X12 + X11 +X10 + X8 + X7 + X5 + X4 + X2 + X + 1)
    _100 = 4,
    ///5: 32-bit CRC-32C (X32 + X28 + X27 + X26 + X25 + X23 + X22 + X20 + X19 + X18 + X14 + X13 + X11 + X10 + X9 + X8 + X6 + 1)
    _101 = 5,
    ///0: No calculation is executed
    Others = 0,
}
impl From<Gps> for u8 {
    #[inline(always)]
    fn from(variant: Gps) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gps {
    type Ux = u8;
}
impl crate::IsEnum for Gps {}
///Field `GPS` reader - CRC Generating Polynomial Switching
pub type GpsR = crate::FieldReader<Gps>;
impl GpsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Gps {
        match self.bits {
            1 => Gps::_001,
            2 => Gps::_010,
            3 => Gps::_011,
            4 => Gps::_100,
            5 => Gps::_101,
            _ => Gps::Others,
        }
    }
    ///8-bit CRC-8 (X8 + X2 + X + 1)
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Gps::_001
    }
    ///16-bit CRC-16 (X16 + X15 + X2 + 1)
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Gps::_010
    }
    ///16-bit CRC-CCITT (X16 + X12 + X5 + 1)
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Gps::_011
    }
    ///32-bit CRC-32 (X32 + X26 + X23 + X22 + X16 + X12 + X11 +X10 + X8 + X7 + X5 + X4 + X2 + X + 1)
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Gps::_100
    }
    ///32-bit CRC-32C (X32 + X28 + X27 + X26 + X25 + X23 + X22 + X20 + X19 + X18 + X14 + X13 + X11 + X10 + X9 + X8 + X6 + 1)
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Gps::_101
    }
    ///No calculation is executed
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Gps::Others)
    }
}
///Field `GPS` writer - CRC Generating Polynomial Switching
pub type GpsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Gps, crate::Safe>;
impl<'a, REG> GpsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///8-bit CRC-8 (X8 + X2 + X + 1)
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Gps::_001)
    }
    ///16-bit CRC-16 (X16 + X15 + X2 + 1)
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Gps::_010)
    }
    ///16-bit CRC-CCITT (X16 + X12 + X5 + 1)
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Gps::_011)
    }
    ///32-bit CRC-32 (X32 + X26 + X23 + X22 + X16 + X12 + X11 +X10 + X8 + X7 + X5 + X4 + X2 + X + 1)
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Gps::_100)
    }
    ///32-bit CRC-32C (X32 + X28 + X27 + X26 + X25 + X23 + X22 + X20 + X19 + X18 + X14 + X13 + X11 + X10 + X9 + X8 + X6 + 1)
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Gps::_101)
    }
    ///No calculation is executed
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Gps::Others)
    }
}
/**CRC Calculation Switching

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lms {
    ///0: Generate CRC code for LSB-first communication
    _0 = 0,
    ///1: Generate CRC code for MSB-first communication
    _1 = 1,
}
impl From<Lms> for bool {
    #[inline(always)]
    fn from(variant: Lms) -> Self {
        variant as u8 != 0
    }
}
///Field `LMS` reader - CRC Calculation Switching
pub type LmsR = crate::BitReader<Lms>;
impl LmsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Lms {
        match self.bits {
            false => Lms::_0,
            true => Lms::_1,
        }
    }
    ///Generate CRC code for LSB-first communication
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lms::_0
    }
    ///Generate CRC code for MSB-first communication
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lms::_1
    }
}
///Field `LMS` writer - CRC Calculation Switching
pub type LmsW<'a, REG> = crate::BitWriter<'a, REG, Lms>;
impl<'a, REG> LmsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Generate CRC code for LSB-first communication
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Lms::_0)
    }
    ///Generate CRC code for MSB-first communication
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Lms::_1)
    }
}
/**CRCDOR/CRCDOR_HA/CRCDOR_BY Register Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dorclr {
    ///0: No effect
    _0 = 0,
    ///1: Clear the CRCDOR/CRCDOR_HA/CRCDOR_BY register
    _1 = 1,
}
impl From<Dorclr> for bool {
    #[inline(always)]
    fn from(variant: Dorclr) -> Self {
        variant as u8 != 0
    }
}
///Field `DORCLR` writer - CRCDOR/CRCDOR_HA/CRCDOR_BY Register Clear
pub type DorclrW<'a, REG> = crate::BitWriter<'a, REG, Dorclr>;
impl<'a, REG> DorclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dorclr::_0)
    }
    ///Clear the CRCDOR/CRCDOR_HA/CRCDOR_BY register
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dorclr::_1)
    }
}
impl R {
    ///Bits 0:2 - CRC Generating Polynomial Switching
    #[inline(always)]
    pub fn gps(&self) -> GpsR {
        GpsR::new(self.bits & 7)
    }
    ///Bit 6 - CRC Calculation Switching
    #[inline(always)]
    pub fn lms(&self) -> LmsR {
        LmsR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRCCR0")
            .field("gps", &self.gps())
            .field("lms", &self.lms())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - CRC Generating Polynomial Switching
    #[inline(always)]
    pub fn gps(&mut self) -> GpsW<Crccr0Spec> {
        GpsW::new(self, 0)
    }
    ///Bit 6 - CRC Calculation Switching
    #[inline(always)]
    pub fn lms(&mut self) -> LmsW<Crccr0Spec> {
        LmsW::new(self, 6)
    }
    ///Bit 7 - CRCDOR/CRCDOR_HA/CRCDOR_BY Register Clear
    #[inline(always)]
    pub fn dorclr(&mut self) -> DorclrW<Crccr0Spec> {
        DorclrW::new(self, 7)
    }
}
/**CRC Control Register 0

You can [`read`](crate::Reg::read) this register and get [`crccr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crccr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Crccr0Spec;
impl crate::RegisterSpec for Crccr0Spec {
    type Ux = u8;
}
///`read()` method returns [`crccr0::R`](R) reader structure
impl crate::Readable for Crccr0Spec {}
///`write(|w| ..)` method takes [`crccr0::W`](W) writer structure
impl crate::Writable for Crccr0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CRCCR0 to value 0
impl crate::Resettable for Crccr0Spec {}
