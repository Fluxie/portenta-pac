///Register `SFMSSC` reader
pub type R = crate::R<SfmsscSpec>;
///Register `SFMSSC` writer
pub type W = crate::W<SfmsscSpec>;
/**Minimum High-level Width Select for QSSL Signal

Value on reset: 7*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sfmsw {
    ///0: 1 QSPCLK
    _0x0 = 0,
    ///1: 2 QSPCLK
    _0x1 = 1,
    ///2: 3 QSPCLK
    _0x2 = 2,
    ///3: 4 QSPCLK
    _0x3 = 3,
    ///4: 5 QSPCLK
    _0x4 = 4,
    ///5: 6 QSPCLK
    _0x5 = 5,
    ///6: 7 QSPCLK
    _0x6 = 6,
    ///7: 8 QSPCLK
    _0x7 = 7,
    ///8: 9 QSPCLK
    _0x8 = 8,
    ///9: 10 QSPCLK
    _0x9 = 9,
    ///10: 11 QSPCLK
    _0xA = 10,
    ///11: 12 QSPCLK
    _0xB = 11,
    ///12: 13 QSPCLK
    _0xC = 12,
    ///13: 14 QSPCLK
    _0xD = 13,
    ///14: 15 QSPCLK
    _0xE = 14,
    ///15: 16 QSPCLK
    _0xF = 15,
}
impl From<Sfmsw> for u8 {
    #[inline(always)]
    fn from(variant: Sfmsw) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sfmsw {
    type Ux = u8;
}
impl crate::IsEnum for Sfmsw {}
///Field `SFMSW` reader - Minimum High-level Width Select for QSSL Signal
pub type SfmswR = crate::FieldReader<Sfmsw>;
impl SfmswR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sfmsw {
        match self.bits {
            0 => Sfmsw::_0x0,
            1 => Sfmsw::_0x1,
            2 => Sfmsw::_0x2,
            3 => Sfmsw::_0x3,
            4 => Sfmsw::_0x4,
            5 => Sfmsw::_0x5,
            6 => Sfmsw::_0x6,
            7 => Sfmsw::_0x7,
            8 => Sfmsw::_0x8,
            9 => Sfmsw::_0x9,
            10 => Sfmsw::_0xA,
            11 => Sfmsw::_0xB,
            12 => Sfmsw::_0xC,
            13 => Sfmsw::_0xD,
            14 => Sfmsw::_0xE,
            15 => Sfmsw::_0xF,
            _ => unreachable!(),
        }
    }
    ///1 QSPCLK
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == Sfmsw::_0x0
    }
    ///2 QSPCLK
    #[inline(always)]
    pub fn is_0x1(&self) -> bool {
        *self == Sfmsw::_0x1
    }
    ///3 QSPCLK
    #[inline(always)]
    pub fn is_0x2(&self) -> bool {
        *self == Sfmsw::_0x2
    }
    ///4 QSPCLK
    #[inline(always)]
    pub fn is_0x3(&self) -> bool {
        *self == Sfmsw::_0x3
    }
    ///5 QSPCLK
    #[inline(always)]
    pub fn is_0x4(&self) -> bool {
        *self == Sfmsw::_0x4
    }
    ///6 QSPCLK
    #[inline(always)]
    pub fn is_0x5(&self) -> bool {
        *self == Sfmsw::_0x5
    }
    ///7 QSPCLK
    #[inline(always)]
    pub fn is_0x6(&self) -> bool {
        *self == Sfmsw::_0x6
    }
    ///8 QSPCLK
    #[inline(always)]
    pub fn is_0x7(&self) -> bool {
        *self == Sfmsw::_0x7
    }
    ///9 QSPCLK
    #[inline(always)]
    pub fn is_0x8(&self) -> bool {
        *self == Sfmsw::_0x8
    }
    ///10 QSPCLK
    #[inline(always)]
    pub fn is_0x9(&self) -> bool {
        *self == Sfmsw::_0x9
    }
    ///11 QSPCLK
    #[inline(always)]
    pub fn is_0x_a(&self) -> bool {
        *self == Sfmsw::_0xA
    }
    ///12 QSPCLK
    #[inline(always)]
    pub fn is_0x_b(&self) -> bool {
        *self == Sfmsw::_0xB
    }
    ///13 QSPCLK
    #[inline(always)]
    pub fn is_0x_c(&self) -> bool {
        *self == Sfmsw::_0xC
    }
    ///14 QSPCLK
    #[inline(always)]
    pub fn is_0x_d(&self) -> bool {
        *self == Sfmsw::_0xD
    }
    ///15 QSPCLK
    #[inline(always)]
    pub fn is_0x_e(&self) -> bool {
        *self == Sfmsw::_0xE
    }
    ///16 QSPCLK
    #[inline(always)]
    pub fn is_0x_f(&self) -> bool {
        *self == Sfmsw::_0xF
    }
}
///Field `SFMSW` writer - Minimum High-level Width Select for QSSL Signal
pub type SfmswW<'a, REG> = crate::FieldWriter<'a, REG, 4, Sfmsw, crate::Safe>;
impl<'a, REG> SfmswW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///1 QSPCLK
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmsw::_0x0)
    }
    ///2 QSPCLK
    #[inline(always)]
    pub fn _0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmsw::_0x1)
    }
    ///3 QSPCLK
    #[inline(always)]
    pub fn _0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmsw::_0x2)
    }
    ///4 QSPCLK
    #[inline(always)]
    pub fn _0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmsw::_0x3)
    }
    ///5 QSPCLK
    #[inline(always)]
    pub fn _0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmsw::_0x4)
    }
    ///6 QSPCLK
    #[inline(always)]
    pub fn _0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmsw::_0x5)
    }
    ///7 QSPCLK
    #[inline(always)]
    pub fn _0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmsw::_0x6)
    }
    ///8 QSPCLK
    #[inline(always)]
    pub fn _0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmsw::_0x7)
    }
    ///9 QSPCLK
    #[inline(always)]
    pub fn _0x8(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmsw::_0x8)
    }
    ///10 QSPCLK
    #[inline(always)]
    pub fn _0x9(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmsw::_0x9)
    }
    ///11 QSPCLK
    #[inline(always)]
    pub fn _0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmsw::_0xA)
    }
    ///12 QSPCLK
    #[inline(always)]
    pub fn _0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmsw::_0xB)
    }
    ///13 QSPCLK
    #[inline(always)]
    pub fn _0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmsw::_0xC)
    }
    ///14 QSPCLK
    #[inline(always)]
    pub fn _0x_d(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmsw::_0xD)
    }
    ///15 QSPCLK
    #[inline(always)]
    pub fn _0x_e(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmsw::_0xE)
    }
    ///16 QSPCLK
    #[inline(always)]
    pub fn _0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmsw::_0xF)
    }
}
/**QSSL Signal Hold Time

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sfmshd {
    ///0: QSSL outputs high after 0.5 QSPCLK cycles from the last rising edge of QSPCLK.
    _0 = 0,
    ///1: QSSL outputs high after 1.5 QSPCLK cycles from the last rising edge of QSPCLK.
    _1 = 1,
}
impl From<Sfmshd> for bool {
    #[inline(always)]
    fn from(variant: Sfmshd) -> Self {
        variant as u8 != 0
    }
}
///Field `SFMSHD` reader - QSSL Signal Hold Time
pub type SfmshdR = crate::BitReader<Sfmshd>;
impl SfmshdR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sfmshd {
        match self.bits {
            false => Sfmshd::_0,
            true => Sfmshd::_1,
        }
    }
    ///QSSL outputs high after 0.5 QSPCLK cycles from the last rising edge of QSPCLK.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sfmshd::_0
    }
    ///QSSL outputs high after 1.5 QSPCLK cycles from the last rising edge of QSPCLK.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sfmshd::_1
    }
}
///Field `SFMSHD` writer - QSSL Signal Hold Time
pub type SfmshdW<'a, REG> = crate::BitWriter<'a, REG, Sfmshd>;
impl<'a, REG> SfmshdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///QSSL outputs high after 0.5 QSPCLK cycles from the last rising edge of QSPCLK.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmshd::_0)
    }
    ///QSSL outputs high after 1.5 QSPCLK cycles from the last rising edge of QSPCLK.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmshd::_1)
    }
}
/**QSSL Signal Setup Time

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sfmsld {
    ///0: QSSL outputs low before 0.5 QSPCLK cycles from the first rising edge of QSPCLK.
    _0 = 0,
    ///1: QSSL outputs low before 1.5 QSPCLK cycles from the first rising edge of QSPCLK.
    _1 = 1,
}
impl From<Sfmsld> for bool {
    #[inline(always)]
    fn from(variant: Sfmsld) -> Self {
        variant as u8 != 0
    }
}
///Field `SFMSLD` reader - QSSL Signal Setup Time
pub type SfmsldR = crate::BitReader<Sfmsld>;
impl SfmsldR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sfmsld {
        match self.bits {
            false => Sfmsld::_0,
            true => Sfmsld::_1,
        }
    }
    ///QSSL outputs low before 0.5 QSPCLK cycles from the first rising edge of QSPCLK.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sfmsld::_0
    }
    ///QSSL outputs low before 1.5 QSPCLK cycles from the first rising edge of QSPCLK.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sfmsld::_1
    }
}
///Field `SFMSLD` writer - QSSL Signal Setup Time
pub type SfmsldW<'a, REG> = crate::BitWriter<'a, REG, Sfmsld>;
impl<'a, REG> SfmsldW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///QSSL outputs low before 0.5 QSPCLK cycles from the first rising edge of QSPCLK.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmsld::_0)
    }
    ///QSSL outputs low before 1.5 QSPCLK cycles from the first rising edge of QSPCLK.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmsld::_1)
    }
}
impl R {
    ///Bits 0:3 - Minimum High-level Width Select for QSSL Signal
    #[inline(always)]
    pub fn sfmsw(&self) -> SfmswR {
        SfmswR::new((self.bits & 0x0f) as u8)
    }
    ///Bit 4 - QSSL Signal Hold Time
    #[inline(always)]
    pub fn sfmshd(&self) -> SfmshdR {
        SfmshdR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - QSSL Signal Setup Time
    #[inline(always)]
    pub fn sfmsld(&self) -> SfmsldR {
        SfmsldR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SFMSSC")
            .field("sfmsw", &self.sfmsw())
            .field("sfmshd", &self.sfmshd())
            .field("sfmsld", &self.sfmsld())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Minimum High-level Width Select for QSSL Signal
    #[inline(always)]
    pub fn sfmsw(&mut self) -> SfmswW<SfmsscSpec> {
        SfmswW::new(self, 0)
    }
    ///Bit 4 - QSSL Signal Hold Time
    #[inline(always)]
    pub fn sfmshd(&mut self) -> SfmshdW<SfmsscSpec> {
        SfmshdW::new(self, 4)
    }
    ///Bit 5 - QSSL Signal Setup Time
    #[inline(always)]
    pub fn sfmsld(&mut self) -> SfmsldW<SfmsscSpec> {
        SfmsldW::new(self, 5)
    }
}
/**Chip Selection Control Register

You can [`read`](crate::Reg::read) this register and get [`sfmssc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfmssc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SfmsscSpec;
impl crate::RegisterSpec for SfmsscSpec {
    type Ux = u32;
}
///`read()` method returns [`sfmssc::R`](R) reader structure
impl crate::Readable for SfmsscSpec {}
///`write(|w| ..)` method takes [`sfmssc::W`](W) writer structure
impl crate::Writable for SfmsscSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SFMSSC to value 0x37
impl crate::Resettable for SfmsscSpec {
    const RESET_VALUE: u32 = 0x37;
}
