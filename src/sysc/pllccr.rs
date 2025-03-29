///Register `PLLCCR` reader
pub type R = crate::R<PllccrSpec>;
///Register `PLLCCR` writer
pub type W = crate::W<PllccrSpec>;
/**PLL Input Frequency Division Ratio Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Plidiv {
    ///0: /1
    _00 = 0,
    ///1: /2
    _01 = 1,
    ///2: /3
    _10 = 2,
    ///3: Setting prohibited.
    Others = 3,
}
impl From<Plidiv> for u8 {
    #[inline(always)]
    fn from(variant: Plidiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Plidiv {
    type Ux = u8;
}
impl crate::IsEnum for Plidiv {}
///Field `PLIDIV` reader - PLL Input Frequency Division Ratio Select
pub type PlidivR = crate::FieldReader<Plidiv>;
impl PlidivR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Plidiv {
        match self.bits {
            0 => Plidiv::_00,
            1 => Plidiv::_01,
            2 => Plidiv::_10,
            3 => Plidiv::Others,
            _ => unreachable!(),
        }
    }
    #[doc = "/1"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Plidiv::_00
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Plidiv::_01
    }
    #[doc = "/3"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Plidiv::_10
    }
    ///Setting prohibited.
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        *self == Plidiv::Others
    }
}
///Field `PLIDIV` writer - PLL Input Frequency Division Ratio Select
pub type PlidivW<'a, REG> = crate::FieldWriter<'a, REG, 2, Plidiv, crate::Safe>;
impl<'a, REG> PlidivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "/1"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Plidiv::_00)
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Plidiv::_01)
    }
    #[doc = "/3"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Plidiv::_10)
    }
    ///Setting prohibited.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Plidiv::Others)
    }
}
/**PLL Clock Source Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Plsrcsel {
    ///0: Main clock oscillator
    _0 = 0,
    ///1: HOCO
    _1 = 1,
}
impl From<Plsrcsel> for bool {
    #[inline(always)]
    fn from(variant: Plsrcsel) -> Self {
        variant as u8 != 0
    }
}
///Field `PLSRCSEL` reader - PLL Clock Source Select
pub type PlsrcselR = crate::BitReader<Plsrcsel>;
impl PlsrcselR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Plsrcsel {
        match self.bits {
            false => Plsrcsel::_0,
            true => Plsrcsel::_1,
        }
    }
    ///Main clock oscillator
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Plsrcsel::_0
    }
    ///HOCO
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Plsrcsel::_1
    }
}
///Field `PLSRCSEL` writer - PLL Clock Source Select
pub type PlsrcselW<'a, REG> = crate::BitWriter<'a, REG, Plsrcsel>;
impl<'a, REG> PlsrcselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Main clock oscillator
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Plsrcsel::_0)
    }
    ///HOCO
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Plsrcsel::_1)
    }
}
///Field `PLLMUL` reader - PLL Frequency Multiplication Factor Select
pub type PllmulR = crate::FieldReader;
///Field `PLLMUL` writer - PLL Frequency Multiplication Factor Select
pub type PllmulW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bits 0:1 - PLL Input Frequency Division Ratio Select
    #[inline(always)]
    pub fn plidiv(&self) -> PlidivR {
        PlidivR::new((self.bits & 3) as u8)
    }
    ///Bit 4 - PLL Clock Source Select
    #[inline(always)]
    pub fn plsrcsel(&self) -> PlsrcselR {
        PlsrcselR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 8:13 - PLL Frequency Multiplication Factor Select
    #[inline(always)]
    pub fn pllmul(&self) -> PllmulR {
        PllmulR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLLCCR")
            .field("plidiv", &self.plidiv())
            .field("plsrcsel", &self.plsrcsel())
            .field("pllmul", &self.pllmul())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - PLL Input Frequency Division Ratio Select
    #[inline(always)]
    pub fn plidiv(&mut self) -> PlidivW<PllccrSpec> {
        PlidivW::new(self, 0)
    }
    ///Bit 4 - PLL Clock Source Select
    #[inline(always)]
    pub fn plsrcsel(&mut self) -> PlsrcselW<PllccrSpec> {
        PlsrcselW::new(self, 4)
    }
    ///Bits 8:13 - PLL Frequency Multiplication Factor Select
    #[inline(always)]
    pub fn pllmul(&mut self) -> PllmulW<PllccrSpec> {
        PllmulW::new(self, 8)
    }
}
/**PLL Clock Control Register

You can [`read`](crate::Reg::read) this register and get [`pllccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PllccrSpec;
impl crate::RegisterSpec for PllccrSpec {
    type Ux = u16;
}
///`read()` method returns [`pllccr::R`](R) reader structure
impl crate::Readable for PllccrSpec {}
///`write(|w| ..)` method takes [`pllccr::W`](W) writer structure
impl crate::Writable for PllccrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PLLCCR to value 0x1300
impl crate::Resettable for PllccrSpec {
    const RESET_VALUE: u16 = 0x1300;
}
