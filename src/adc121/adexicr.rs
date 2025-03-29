///Register `ADEXICR` reader
pub type R = crate::R<AdexicrSpec>;
///Register `ADEXICR` writer
pub type W = crate::W<AdexicrSpec>;
/**Temperature Sensor Output A/D-Converted Value Addition/Average Mode Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tssad {
    ///0: Do not select addition/average mode for temperature sensor output.
    _0 = 0,
    ///1: Select addition/average mode for temperature sensor output.
    _1 = 1,
}
impl From<Tssad> for bool {
    #[inline(always)]
    fn from(variant: Tssad) -> Self {
        variant as u8 != 0
    }
}
///Field `TSSAD` reader - Temperature Sensor Output A/D-Converted Value Addition/Average Mode Select
pub type TssadR = crate::BitReader<Tssad>;
impl TssadR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tssad {
        match self.bits {
            false => Tssad::_0,
            true => Tssad::_1,
        }
    }
    ///Do not select addition/average mode for temperature sensor output.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tssad::_0
    }
    ///Select addition/average mode for temperature sensor output.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tssad::_1
    }
}
///Field `TSSAD` writer - Temperature Sensor Output A/D-Converted Value Addition/Average Mode Select
pub type TssadW<'a, REG> = crate::BitWriter<'a, REG, Tssad>;
impl<'a, REG> TssadW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select addition/average mode for temperature sensor output.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tssad::_0)
    }
    ///Select addition/average mode for temperature sensor output.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tssad::_1)
    }
}
/**Internal Reference Voltage A/D-Converted Value Addition/Average Mode Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ocsad {
    ///0: Do not select addition/average mode for internal reference voltage.
    _0 = 0,
    ///1: Select addition/average mode for internal reference voltage.
    _1 = 1,
}
impl From<Ocsad> for bool {
    #[inline(always)]
    fn from(variant: Ocsad) -> Self {
        variant as u8 != 0
    }
}
///Field `OCSAD` reader - Internal Reference Voltage A/D-Converted Value Addition/Average Mode Select
pub type OcsadR = crate::BitReader<Ocsad>;
impl OcsadR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ocsad {
        match self.bits {
            false => Ocsad::_0,
            true => Ocsad::_1,
        }
    }
    ///Do not select addition/average mode for internal reference voltage.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ocsad::_0
    }
    ///Select addition/average mode for internal reference voltage.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ocsad::_1
    }
}
///Field `OCSAD` writer - Internal Reference Voltage A/D-Converted Value Addition/Average Mode Select
pub type OcsadW<'a, REG> = crate::BitWriter<'a, REG, Ocsad>;
impl<'a, REG> OcsadW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select addition/average mode for internal reference voltage.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ocsad::_0)
    }
    ///Select addition/average mode for internal reference voltage.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ocsad::_1)
    }
}
/**Temperature Sensor Output A/D Conversion Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tssa {
    ///0: Disable A/D conversion of temperature sensor output
    _0 = 0,
    ///1: Enable A/D conversion of temperature sensor output
    _1 = 1,
}
impl From<Tssa> for bool {
    #[inline(always)]
    fn from(variant: Tssa) -> Self {
        variant as u8 != 0
    }
}
///Field `TSSA` reader - Temperature Sensor Output A/D Conversion Select
pub type TssaR = crate::BitReader<Tssa>;
impl TssaR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tssa {
        match self.bits {
            false => Tssa::_0,
            true => Tssa::_1,
        }
    }
    ///Disable A/D conversion of temperature sensor output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tssa::_0
    }
    ///Enable A/D conversion of temperature sensor output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tssa::_1
    }
}
///Field `TSSA` writer - Temperature Sensor Output A/D Conversion Select
pub type TssaW<'a, REG> = crate::BitWriter<'a, REG, Tssa>;
impl<'a, REG> TssaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable A/D conversion of temperature sensor output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tssa::_0)
    }
    ///Enable A/D conversion of temperature sensor output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tssa::_1)
    }
}
/**Internal Reference Voltage A/D Conversion Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ocsa {
    ///0: Disable A/D conversion of internal reference voltage
    _0 = 0,
    ///1: Enable A/D conversion of internal reference voltage
    _1 = 1,
}
impl From<Ocsa> for bool {
    #[inline(always)]
    fn from(variant: Ocsa) -> Self {
        variant as u8 != 0
    }
}
///Field `OCSA` reader - Internal Reference Voltage A/D Conversion Select
pub type OcsaR = crate::BitReader<Ocsa>;
impl OcsaR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ocsa {
        match self.bits {
            false => Ocsa::_0,
            true => Ocsa::_1,
        }
    }
    ///Disable A/D conversion of internal reference voltage
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ocsa::_0
    }
    ///Enable A/D conversion of internal reference voltage
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ocsa::_1
    }
}
///Field `OCSA` writer - Internal Reference Voltage A/D Conversion Select
pub type OcsaW<'a, REG> = crate::BitWriter<'a, REG, Ocsa>;
impl<'a, REG> OcsaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable A/D conversion of internal reference voltage
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ocsa::_0)
    }
    ///Enable A/D conversion of internal reference voltage
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ocsa::_1)
    }
}
/**Temperature Sensor Output A/D Conversion Select for Group B

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tssb {
    ///0: Disable A/D conversion of temperature sensor output
    _0 = 0,
    ///1: Enable A/D conversion of temperature sensor output
    _1 = 1,
}
impl From<Tssb> for bool {
    #[inline(always)]
    fn from(variant: Tssb) -> Self {
        variant as u8 != 0
    }
}
///Field `TSSB` reader - Temperature Sensor Output A/D Conversion Select for Group B
pub type TssbR = crate::BitReader<Tssb>;
impl TssbR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tssb {
        match self.bits {
            false => Tssb::_0,
            true => Tssb::_1,
        }
    }
    ///Disable A/D conversion of temperature sensor output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tssb::_0
    }
    ///Enable A/D conversion of temperature sensor output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tssb::_1
    }
}
///Field `TSSB` writer - Temperature Sensor Output A/D Conversion Select for Group B
pub type TssbW<'a, REG> = crate::BitWriter<'a, REG, Tssb>;
impl<'a, REG> TssbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable A/D conversion of temperature sensor output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tssb::_0)
    }
    ///Enable A/D conversion of temperature sensor output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tssb::_1)
    }
}
/**Internal Reference Voltage A/D Conversion Select for Group B

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ocsb {
    ///0: Disable A/D conversion of internal reference voltage
    _0 = 0,
    ///1: Enable A/D conversion of internal reference voltage
    _1 = 1,
}
impl From<Ocsb> for bool {
    #[inline(always)]
    fn from(variant: Ocsb) -> Self {
        variant as u8 != 0
    }
}
///Field `OCSB` reader - Internal Reference Voltage A/D Conversion Select for Group B
pub type OcsbR = crate::BitReader<Ocsb>;
impl OcsbR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ocsb {
        match self.bits {
            false => Ocsb::_0,
            true => Ocsb::_1,
        }
    }
    ///Disable A/D conversion of internal reference voltage
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ocsb::_0
    }
    ///Enable A/D conversion of internal reference voltage
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ocsb::_1
    }
}
///Field `OCSB` writer - Internal Reference Voltage A/D Conversion Select for Group B
pub type OcsbW<'a, REG> = crate::BitWriter<'a, REG, Ocsb>;
impl<'a, REG> OcsbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable A/D conversion of internal reference voltage
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ocsb::_0)
    }
    ///Enable A/D conversion of internal reference voltage
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ocsb::_1)
    }
}
impl R {
    ///Bit 0 - Temperature Sensor Output A/D-Converted Value Addition/Average Mode Select
    #[inline(always)]
    pub fn tssad(&self) -> TssadR {
        TssadR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Internal Reference Voltage A/D-Converted Value Addition/Average Mode Select
    #[inline(always)]
    pub fn ocsad(&self) -> OcsadR {
        OcsadR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 8 - Temperature Sensor Output A/D Conversion Select
    #[inline(always)]
    pub fn tssa(&self) -> TssaR {
        TssaR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Internal Reference Voltage A/D Conversion Select
    #[inline(always)]
    pub fn ocsa(&self) -> OcsaR {
        OcsaR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Temperature Sensor Output A/D Conversion Select for Group B
    #[inline(always)]
    pub fn tssb(&self) -> TssbR {
        TssbR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Internal Reference Voltage A/D Conversion Select for Group B
    #[inline(always)]
    pub fn ocsb(&self) -> OcsbR {
        OcsbR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADEXICR")
            .field("tssad", &self.tssad())
            .field("ocsad", &self.ocsad())
            .field("tssa", &self.tssa())
            .field("ocsa", &self.ocsa())
            .field("tssb", &self.tssb())
            .field("ocsb", &self.ocsb())
            .finish()
    }
}
impl W {
    ///Bit 0 - Temperature Sensor Output A/D-Converted Value Addition/Average Mode Select
    #[inline(always)]
    pub fn tssad(&mut self) -> TssadW<AdexicrSpec> {
        TssadW::new(self, 0)
    }
    ///Bit 1 - Internal Reference Voltage A/D-Converted Value Addition/Average Mode Select
    #[inline(always)]
    pub fn ocsad(&mut self) -> OcsadW<AdexicrSpec> {
        OcsadW::new(self, 1)
    }
    ///Bit 8 - Temperature Sensor Output A/D Conversion Select
    #[inline(always)]
    pub fn tssa(&mut self) -> TssaW<AdexicrSpec> {
        TssaW::new(self, 8)
    }
    ///Bit 9 - Internal Reference Voltage A/D Conversion Select
    #[inline(always)]
    pub fn ocsa(&mut self) -> OcsaW<AdexicrSpec> {
        OcsaW::new(self, 9)
    }
    ///Bit 10 - Temperature Sensor Output A/D Conversion Select for Group B
    #[inline(always)]
    pub fn tssb(&mut self) -> TssbW<AdexicrSpec> {
        TssbW::new(self, 10)
    }
    ///Bit 11 - Internal Reference Voltage A/D Conversion Select for Group B
    #[inline(always)]
    pub fn ocsb(&mut self) -> OcsbW<AdexicrSpec> {
        OcsbW::new(self, 11)
    }
}
/**A/D Conversion Extended Input Control Registers

You can [`read`](crate::Reg::read) this register and get [`adexicr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adexicr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AdexicrSpec;
impl crate::RegisterSpec for AdexicrSpec {
    type Ux = u16;
}
///`read()` method returns [`adexicr::R`](R) reader structure
impl crate::Readable for AdexicrSpec {}
///`write(|w| ..)` method takes [`adexicr::W`](W) writer structure
impl crate::Writable for AdexicrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADEXICR to value 0
impl crate::Resettable for AdexicrSpec {}
