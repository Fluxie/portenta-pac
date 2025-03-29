///Register `PLL2CCR` reader
pub type R = crate::R<Pll2ccrSpec>;
///Register `PLL2CCR` writer
pub type W = crate::W<Pll2ccrSpec>;
/**PLL2 Input Frequency Division Ratio Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pl2idiv {
    ///0: ∕ 1 (value after reset)
    _00 = 0,
    ///1: ∕ 2
    _01 = 1,
    ///2: ∕ 3
    _10 = 2,
    ///3: Setting prohibited.
    Others = 3,
}
impl From<Pl2idiv> for u8 {
    #[inline(always)]
    fn from(variant: Pl2idiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pl2idiv {
    type Ux = u8;
}
impl crate::IsEnum for Pl2idiv {}
///Field `PL2IDIV` reader - PLL2 Input Frequency Division Ratio Select
pub type Pl2idivR = crate::FieldReader<Pl2idiv>;
impl Pl2idivR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pl2idiv {
        match self.bits {
            0 => Pl2idiv::_00,
            1 => Pl2idiv::_01,
            2 => Pl2idiv::_10,
            3 => Pl2idiv::Others,
            _ => unreachable!(),
        }
    }
    ///∕ 1 (value after reset)
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Pl2idiv::_00
    }
    ///∕ 2
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Pl2idiv::_01
    }
    ///∕ 3
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Pl2idiv::_10
    }
    ///Setting prohibited.
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        *self == Pl2idiv::Others
    }
}
///Field `PL2IDIV` writer - PLL2 Input Frequency Division Ratio Select
pub type Pl2idivW<'a, REG> = crate::FieldWriter<'a, REG, 2, Pl2idiv, crate::Safe>;
impl<'a, REG> Pl2idivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///∕ 1 (value after reset)
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Pl2idiv::_00)
    }
    ///∕ 2
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Pl2idiv::_01)
    }
    ///∕ 3
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Pl2idiv::_10)
    }
    ///Setting prohibited.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Pl2idiv::Others)
    }
}
/**PLL2 Clock Source Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pl2srcsel {
    ///0: Main clock oscillator
    _0 = 0,
    ///1: HOCO
    _1 = 1,
}
impl From<Pl2srcsel> for bool {
    #[inline(always)]
    fn from(variant: Pl2srcsel) -> Self {
        variant as u8 != 0
    }
}
///Field `PL2SRCSEL` reader - PLL2 Clock Source Select
pub type Pl2srcselR = crate::BitReader<Pl2srcsel>;
impl Pl2srcselR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pl2srcsel {
        match self.bits {
            false => Pl2srcsel::_0,
            true => Pl2srcsel::_1,
        }
    }
    ///Main clock oscillator
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pl2srcsel::_0
    }
    ///HOCO
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pl2srcsel::_1
    }
}
///Field `PL2SRCSEL` writer - PLL2 Clock Source Select
pub type Pl2srcselW<'a, REG> = crate::BitWriter<'a, REG, Pl2srcsel>;
impl<'a, REG> Pl2srcselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Main clock oscillator
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pl2srcsel::_0)
    }
    ///HOCO
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pl2srcsel::_1)
    }
}
///Field `PLL2MUL` reader - PLL2 Frequency Multiplication Factor Select
pub type Pll2mulR = crate::FieldReader;
///Field `PLL2MUL` writer - PLL2 Frequency Multiplication Factor Select
pub type Pll2mulW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bits 0:1 - PLL2 Input Frequency Division Ratio Select
    #[inline(always)]
    pub fn pl2idiv(&self) -> Pl2idivR {
        Pl2idivR::new((self.bits & 3) as u8)
    }
    ///Bit 4 - PLL2 Clock Source Select
    #[inline(always)]
    pub fn pl2srcsel(&self) -> Pl2srcselR {
        Pl2srcselR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 8:13 - PLL2 Frequency Multiplication Factor Select
    #[inline(always)]
    pub fn pll2mul(&self) -> Pll2mulR {
        Pll2mulR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL2CCR")
            .field("pl2idiv", &self.pl2idiv())
            .field("pl2srcsel", &self.pl2srcsel())
            .field("pll2mul", &self.pll2mul())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - PLL2 Input Frequency Division Ratio Select
    #[inline(always)]
    pub fn pl2idiv(&mut self) -> Pl2idivW<Pll2ccrSpec> {
        Pl2idivW::new(self, 0)
    }
    ///Bit 4 - PLL2 Clock Source Select
    #[inline(always)]
    pub fn pl2srcsel(&mut self) -> Pl2srcselW<Pll2ccrSpec> {
        Pl2srcselW::new(self, 4)
    }
    ///Bits 8:13 - PLL2 Frequency Multiplication Factor Select
    #[inline(always)]
    pub fn pll2mul(&mut self) -> Pll2mulW<Pll2ccrSpec> {
        Pll2mulW::new(self, 8)
    }
}
/**PLL2 Clock Control Register

You can [`read`](crate::Reg::read) this register and get [`pll2ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll2ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Pll2ccrSpec;
impl crate::RegisterSpec for Pll2ccrSpec {
    type Ux = u16;
}
///`read()` method returns [`pll2ccr::R`](R) reader structure
impl crate::Readable for Pll2ccrSpec {}
///`write(|w| ..)` method takes [`pll2ccr::W`](W) writer structure
impl crate::Writable for Pll2ccrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PLL2CCR to value 0x1300
impl crate::Resettable for Pll2ccrSpec {
    const RESET_VALUE: u16 = 0x1300;
}
