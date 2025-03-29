///Register `CTSUERRS` reader
pub type R = crate::R<CtsuerrsSpec>;
///Register `CTSUERRS` writer
pub type W = crate::W<CtsuerrsSpec>;
/**Calibration Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctsuspmd {
    ///0: Capacitance measurement mode
    _00 = 0,
    ///2: Calibration mode
    _10 = 2,
    ///1: Seting prohibited
    Others = 1,
}
impl From<Ctsuspmd> for u8 {
    #[inline(always)]
    fn from(variant: Ctsuspmd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctsuspmd {
    type Ux = u8;
}
impl crate::IsEnum for Ctsuspmd {}
///Field `CTSUSPMD` reader - Calibration Mode
pub type CtsuspmdR = crate::FieldReader<Ctsuspmd>;
impl CtsuspmdR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ctsuspmd {
        match self.bits {
            0 => Ctsuspmd::_00,
            2 => Ctsuspmd::_10,
            _ => Ctsuspmd::Others,
        }
    }
    ///Capacitance measurement mode
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Ctsuspmd::_00
    }
    ///Calibration mode
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Ctsuspmd::_10
    }
    ///Seting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Ctsuspmd::Others)
    }
}
///Field `CTSUSPMD` writer - Calibration Mode
pub type CtsuspmdW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ctsuspmd, crate::Safe>;
impl<'a, REG> CtsuspmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Capacitance measurement mode
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsuspmd::_00)
    }
    ///Calibration mode
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsuspmd::_10)
    }
    ///Seting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsuspmd::Others)
    }
}
/**TS Pin Fixed Output

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctsutsod {
    ///0: Capacitance measurement mode
    _0 = 0,
    ///1: TS pins are forced to be high or low
    _1 = 1,
}
impl From<Ctsutsod> for bool {
    #[inline(always)]
    fn from(variant: Ctsutsod) -> Self {
        variant as u8 != 0
    }
}
///Field `CTSUTSOD` reader - TS Pin Fixed Output
pub type CtsutsodR = crate::BitReader<Ctsutsod>;
impl CtsutsodR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ctsutsod {
        match self.bits {
            false => Ctsutsod::_0,
            true => Ctsutsod::_1,
        }
    }
    ///Capacitance measurement mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ctsutsod::_0
    }
    ///TS pins are forced to be high or low
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ctsutsod::_1
    }
}
///Field `CTSUTSOD` writer - TS Pin Fixed Output
pub type CtsutsodW<'a, REG> = crate::BitWriter<'a, REG, Ctsutsod>;
impl<'a, REG> CtsutsodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Capacitance measurement mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsutsod::_0)
    }
    ///TS pins are forced to be high or low
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsutsod::_1)
    }
}
/**Calibration Setting 1

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctsudrv {
    ///0: Capacitance measurement mode
    _0 = 0,
    ///1: Calibration setting 1
    _1 = 1,
}
impl From<Ctsudrv> for bool {
    #[inline(always)]
    fn from(variant: Ctsudrv) -> Self {
        variant as u8 != 0
    }
}
///Field `CTSUDRV` reader - Calibration Setting 1
pub type CtsudrvR = crate::BitReader<Ctsudrv>;
impl CtsudrvR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ctsudrv {
        match self.bits {
            false => Ctsudrv::_0,
            true => Ctsudrv::_1,
        }
    }
    ///Capacitance measurement mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ctsudrv::_0
    }
    ///Calibration setting 1
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ctsudrv::_1
    }
}
///Field `CTSUDRV` writer - Calibration Setting 1
pub type CtsudrvW<'a, REG> = crate::BitWriter<'a, REG, Ctsudrv>;
impl<'a, REG> CtsudrvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Capacitance measurement mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsudrv::_0)
    }
    ///Calibration setting 1
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsudrv::_1)
    }
}
/**Calibration Setting 3

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctsuclksel1 {
    ///0: Capacitance measurement mode
    _0 = 0,
    ///1: Calibration setting 3
    _1 = 1,
}
impl From<Ctsuclksel1> for bool {
    #[inline(always)]
    fn from(variant: Ctsuclksel1) -> Self {
        variant as u8 != 0
    }
}
///Field `CTSUCLKSEL1` reader - Calibration Setting 3
pub type Ctsuclksel1R = crate::BitReader<Ctsuclksel1>;
impl Ctsuclksel1R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ctsuclksel1 {
        match self.bits {
            false => Ctsuclksel1::_0,
            true => Ctsuclksel1::_1,
        }
    }
    ///Capacitance measurement mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ctsuclksel1::_0
    }
    ///Calibration setting 3
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ctsuclksel1::_1
    }
}
///Field `CTSUCLKSEL1` writer - Calibration Setting 3
pub type Ctsuclksel1W<'a, REG> = crate::BitWriter<'a, REG, Ctsuclksel1>;
impl<'a, REG> Ctsuclksel1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Capacitance measurement mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsuclksel1::_0)
    }
    ///Calibration setting 3
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsuclksel1::_1)
    }
}
/**Calibration Setting 2

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctsutsoc {
    ///0: Capacitance measurement mode
    _0 = 0,
    ///1: Calibration setting 2
    _1 = 1,
}
impl From<Ctsutsoc> for bool {
    #[inline(always)]
    fn from(variant: Ctsutsoc) -> Self {
        variant as u8 != 0
    }
}
///Field `CTSUTSOC` reader - Calibration Setting 2
pub type CtsutsocR = crate::BitReader<Ctsutsoc>;
impl CtsutsocR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ctsutsoc {
        match self.bits {
            false => Ctsutsoc::_0,
            true => Ctsutsoc::_1,
        }
    }
    ///Capacitance measurement mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ctsutsoc::_0
    }
    ///Calibration setting 2
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ctsutsoc::_1
    }
}
///Field `CTSUTSOC` writer - Calibration Setting 2
pub type CtsutsocW<'a, REG> = crate::BitWriter<'a, REG, Ctsutsoc>;
impl<'a, REG> CtsutsocW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Capacitance measurement mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsutsoc::_0)
    }
    ///Calibration setting 2
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsutsoc::_1)
    }
}
/**TSCAP Voltage Error Monitor

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctsuicomp {
    ///0: Normal TSCAP voltage
    _0 = 0,
    ///1: Abnormal TSCAP voltage
    _1 = 1,
}
impl From<Ctsuicomp> for bool {
    #[inline(always)]
    fn from(variant: Ctsuicomp) -> Self {
        variant as u8 != 0
    }
}
///Field `CTSUICOMP` reader - TSCAP Voltage Error Monitor
pub type CtsuicompR = crate::BitReader<Ctsuicomp>;
impl CtsuicompR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ctsuicomp {
        match self.bits {
            false => Ctsuicomp::_0,
            true => Ctsuicomp::_1,
        }
    }
    ///Normal TSCAP voltage
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ctsuicomp::_0
    }
    ///Abnormal TSCAP voltage
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ctsuicomp::_1
    }
}
impl R {
    ///Bits 0:1 - Calibration Mode
    #[inline(always)]
    pub fn ctsuspmd(&self) -> CtsuspmdR {
        CtsuspmdR::new((self.bits & 3) as u8)
    }
    ///Bit 2 - TS Pin Fixed Output
    #[inline(always)]
    pub fn ctsutsod(&self) -> CtsutsodR {
        CtsutsodR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Calibration Setting 1
    #[inline(always)]
    pub fn ctsudrv(&self) -> CtsudrvR {
        CtsudrvR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 6 - Calibration Setting 3
    #[inline(always)]
    pub fn ctsuclksel1(&self) -> Ctsuclksel1R {
        Ctsuclksel1R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Calibration Setting 2
    #[inline(always)]
    pub fn ctsutsoc(&self) -> CtsutsocR {
        CtsutsocR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 15 - TSCAP Voltage Error Monitor
    #[inline(always)]
    pub fn ctsuicomp(&self) -> CtsuicompR {
        CtsuicompR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTSUERRS")
            .field("ctsuspmd", &self.ctsuspmd())
            .field("ctsutsod", &self.ctsutsod())
            .field("ctsudrv", &self.ctsudrv())
            .field("ctsuclksel1", &self.ctsuclksel1())
            .field("ctsutsoc", &self.ctsutsoc())
            .field("ctsuicomp", &self.ctsuicomp())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Calibration Mode
    #[inline(always)]
    pub fn ctsuspmd(&mut self) -> CtsuspmdW<CtsuerrsSpec> {
        CtsuspmdW::new(self, 0)
    }
    ///Bit 2 - TS Pin Fixed Output
    #[inline(always)]
    pub fn ctsutsod(&mut self) -> CtsutsodW<CtsuerrsSpec> {
        CtsutsodW::new(self, 2)
    }
    ///Bit 3 - Calibration Setting 1
    #[inline(always)]
    pub fn ctsudrv(&mut self) -> CtsudrvW<CtsuerrsSpec> {
        CtsudrvW::new(self, 3)
    }
    ///Bit 6 - Calibration Setting 3
    #[inline(always)]
    pub fn ctsuclksel1(&mut self) -> Ctsuclksel1W<CtsuerrsSpec> {
        Ctsuclksel1W::new(self, 6)
    }
    ///Bit 7 - Calibration Setting 2
    #[inline(always)]
    pub fn ctsutsoc(&mut self) -> CtsutsocW<CtsuerrsSpec> {
        CtsutsocW::new(self, 7)
    }
}
/**CTSU Error Status Register

You can [`read`](crate::Reg::read) this register and get [`ctsuerrs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsuerrs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CtsuerrsSpec;
impl crate::RegisterSpec for CtsuerrsSpec {
    type Ux = u16;
}
///`read()` method returns [`ctsuerrs::R`](R) reader structure
impl crate::Readable for CtsuerrsSpec {}
///`write(|w| ..)` method takes [`ctsuerrs::W`](W) writer structure
impl crate::Writable for CtsuerrsSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CTSUERRS to value 0
impl crate::Resettable for CtsuerrsSpec {}
