///Register `SFMSKC` reader
pub type R = crate::R<SfmskcSpec>;
///Register `SFMSKC` writer
pub type W = crate::W<SfmskcSpec>;
/**Serial interface reference cycle select. (Pay attention to irregularities.)

Value on reset: 8*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sfmdv {
    ///0: 2 PCLKA
    _0x00 = 0,
    ///1: 3 PCLKA (divided by an odd number)
    _0x01 = 1,
    ///2: 4 PCLKA
    _0x02 = 2,
    ///3: 5 PCLKA (divided by an odd number)
    _0x03 = 3,
    ///4: 6 PCLKA
    _0x04 = 4,
    ///5: 7 PCLKA (divided by an odd number)
    _0x05 = 5,
    ///6: 8 PCLKA
    _0x06 = 6,
    ///7: 9 PCLKA (divided by an odd number)
    _0x07 = 7,
    ///8: 10 PCLKA
    _0x08 = 8,
    ///9: 11 PCLKA (divided by an odd number)
    _0x09 = 9,
    ///10: 12 PCLKA
    _0x0a = 10,
    ///11: 13 PCLKA (divided by an odd number)
    _0x0b = 11,
    ///12: 14 PCLKA
    _0x0c = 12,
    ///13: 15 PCLKA (divided by an odd number)
    _0x0d = 13,
    ///14: 16 PCLKA
    _0x0e = 14,
    ///15: 17 PCLKA (divided by an odd number)
    _0x0f = 15,
    ///16: 18 PCLKA
    _0x10 = 16,
    ///17: 20 PCLKA
    _0x11 = 17,
    ///18: 22 PCLKA
    _0x12 = 18,
    ///19: 24 PCLKA
    _0x13 = 19,
    ///20: 26 PCLKA
    _0x14 = 20,
    ///21: 28 PCLKA
    _0x15 = 21,
    ///22: 30 PCLKA
    _0x16 = 22,
    ///23: 32 PCLKA
    _0x17 = 23,
    ///24: 34 PCLKA
    _0x18 = 24,
    ///25: 36 PCLKA
    _0x19 = 25,
    ///26: 38 PCLKA
    _0x1a = 26,
    ///27: 40 PCLKA
    _0x1b = 27,
    ///28: 42 PCLKA
    _0x1c = 28,
    ///29: 44 PCLKA
    _0x1d = 29,
    ///30: 46 PCLKA
    _0x1e = 30,
    ///31: 48 PCLKA
    _0x1f = 31,
}
impl From<Sfmdv> for u8 {
    #[inline(always)]
    fn from(variant: Sfmdv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sfmdv {
    type Ux = u8;
}
impl crate::IsEnum for Sfmdv {}
///Field `SFMDV` reader - Serial interface reference cycle select. (Pay attention to irregularities.)
pub type SfmdvR = crate::FieldReader<Sfmdv>;
impl SfmdvR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sfmdv {
        match self.bits {
            0 => Sfmdv::_0x00,
            1 => Sfmdv::_0x01,
            2 => Sfmdv::_0x02,
            3 => Sfmdv::_0x03,
            4 => Sfmdv::_0x04,
            5 => Sfmdv::_0x05,
            6 => Sfmdv::_0x06,
            7 => Sfmdv::_0x07,
            8 => Sfmdv::_0x08,
            9 => Sfmdv::_0x09,
            10 => Sfmdv::_0x0a,
            11 => Sfmdv::_0x0b,
            12 => Sfmdv::_0x0c,
            13 => Sfmdv::_0x0d,
            14 => Sfmdv::_0x0e,
            15 => Sfmdv::_0x0f,
            16 => Sfmdv::_0x10,
            17 => Sfmdv::_0x11,
            18 => Sfmdv::_0x12,
            19 => Sfmdv::_0x13,
            20 => Sfmdv::_0x14,
            21 => Sfmdv::_0x15,
            22 => Sfmdv::_0x16,
            23 => Sfmdv::_0x17,
            24 => Sfmdv::_0x18,
            25 => Sfmdv::_0x19,
            26 => Sfmdv::_0x1a,
            27 => Sfmdv::_0x1b,
            28 => Sfmdv::_0x1c,
            29 => Sfmdv::_0x1d,
            30 => Sfmdv::_0x1e,
            31 => Sfmdv::_0x1f,
            _ => unreachable!(),
        }
    }
    ///2 PCLKA
    #[inline(always)]
    pub fn is_0x00(&self) -> bool {
        *self == Sfmdv::_0x00
    }
    ///3 PCLKA (divided by an odd number)
    #[inline(always)]
    pub fn is_0x01(&self) -> bool {
        *self == Sfmdv::_0x01
    }
    ///4 PCLKA
    #[inline(always)]
    pub fn is_0x02(&self) -> bool {
        *self == Sfmdv::_0x02
    }
    ///5 PCLKA (divided by an odd number)
    #[inline(always)]
    pub fn is_0x03(&self) -> bool {
        *self == Sfmdv::_0x03
    }
    ///6 PCLKA
    #[inline(always)]
    pub fn is_0x04(&self) -> bool {
        *self == Sfmdv::_0x04
    }
    ///7 PCLKA (divided by an odd number)
    #[inline(always)]
    pub fn is_0x05(&self) -> bool {
        *self == Sfmdv::_0x05
    }
    ///8 PCLKA
    #[inline(always)]
    pub fn is_0x06(&self) -> bool {
        *self == Sfmdv::_0x06
    }
    ///9 PCLKA (divided by an odd number)
    #[inline(always)]
    pub fn is_0x07(&self) -> bool {
        *self == Sfmdv::_0x07
    }
    ///10 PCLKA
    #[inline(always)]
    pub fn is_0x08(&self) -> bool {
        *self == Sfmdv::_0x08
    }
    ///11 PCLKA (divided by an odd number)
    #[inline(always)]
    pub fn is_0x09(&self) -> bool {
        *self == Sfmdv::_0x09
    }
    ///12 PCLKA
    #[inline(always)]
    pub fn is_0x0a(&self) -> bool {
        *self == Sfmdv::_0x0a
    }
    ///13 PCLKA (divided by an odd number)
    #[inline(always)]
    pub fn is_0x0b(&self) -> bool {
        *self == Sfmdv::_0x0b
    }
    ///14 PCLKA
    #[inline(always)]
    pub fn is_0x0c(&self) -> bool {
        *self == Sfmdv::_0x0c
    }
    ///15 PCLKA (divided by an odd number)
    #[inline(always)]
    pub fn is_0x0d(&self) -> bool {
        *self == Sfmdv::_0x0d
    }
    ///16 PCLKA
    #[inline(always)]
    pub fn is_0x0e(&self) -> bool {
        *self == Sfmdv::_0x0e
    }
    ///17 PCLKA (divided by an odd number)
    #[inline(always)]
    pub fn is_0x0f(&self) -> bool {
        *self == Sfmdv::_0x0f
    }
    ///18 PCLKA
    #[inline(always)]
    pub fn is_0x10(&self) -> bool {
        *self == Sfmdv::_0x10
    }
    ///20 PCLKA
    #[inline(always)]
    pub fn is_0x11(&self) -> bool {
        *self == Sfmdv::_0x11
    }
    ///22 PCLKA
    #[inline(always)]
    pub fn is_0x12(&self) -> bool {
        *self == Sfmdv::_0x12
    }
    ///24 PCLKA
    #[inline(always)]
    pub fn is_0x13(&self) -> bool {
        *self == Sfmdv::_0x13
    }
    ///26 PCLKA
    #[inline(always)]
    pub fn is_0x14(&self) -> bool {
        *self == Sfmdv::_0x14
    }
    ///28 PCLKA
    #[inline(always)]
    pub fn is_0x15(&self) -> bool {
        *self == Sfmdv::_0x15
    }
    ///30 PCLKA
    #[inline(always)]
    pub fn is_0x16(&self) -> bool {
        *self == Sfmdv::_0x16
    }
    ///32 PCLKA
    #[inline(always)]
    pub fn is_0x17(&self) -> bool {
        *self == Sfmdv::_0x17
    }
    ///34 PCLKA
    #[inline(always)]
    pub fn is_0x18(&self) -> bool {
        *self == Sfmdv::_0x18
    }
    ///36 PCLKA
    #[inline(always)]
    pub fn is_0x19(&self) -> bool {
        *self == Sfmdv::_0x19
    }
    ///38 PCLKA
    #[inline(always)]
    pub fn is_0x1a(&self) -> bool {
        *self == Sfmdv::_0x1a
    }
    ///40 PCLKA
    #[inline(always)]
    pub fn is_0x1b(&self) -> bool {
        *self == Sfmdv::_0x1b
    }
    ///42 PCLKA
    #[inline(always)]
    pub fn is_0x1c(&self) -> bool {
        *self == Sfmdv::_0x1c
    }
    ///44 PCLKA
    #[inline(always)]
    pub fn is_0x1d(&self) -> bool {
        *self == Sfmdv::_0x1d
    }
    ///46 PCLKA
    #[inline(always)]
    pub fn is_0x1e(&self) -> bool {
        *self == Sfmdv::_0x1e
    }
    ///48 PCLKA
    #[inline(always)]
    pub fn is_0x1f(&self) -> bool {
        *self == Sfmdv::_0x1f
    }
}
///Field `SFMDV` writer - Serial interface reference cycle select. (Pay attention to irregularities.)
pub type SfmdvW<'a, REG> = crate::FieldWriter<'a, REG, 5, Sfmdv, crate::Safe>;
impl<'a, REG> SfmdvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///2 PCLKA
    #[inline(always)]
    pub fn _0x00(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmdv::_0x00)
    }
    ///3 PCLKA (divided by an odd number)
    #[inline(always)]
    pub fn _0x01(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmdv::_0x01)
    }
    ///4 PCLKA
    #[inline(always)]
    pub fn _0x02(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmdv::_0x02)
    }
    ///5 PCLKA (divided by an odd number)
    #[inline(always)]
    pub fn _0x03(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmdv::_0x03)
    }
    ///6 PCLKA
    #[inline(always)]
    pub fn _0x04(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmdv::_0x04)
    }
    ///7 PCLKA (divided by an odd number)
    #[inline(always)]
    pub fn _0x05(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmdv::_0x05)
    }
    ///8 PCLKA
    #[inline(always)]
    pub fn _0x06(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmdv::_0x06)
    }
    ///9 PCLKA (divided by an odd number)
    #[inline(always)]
    pub fn _0x07(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmdv::_0x07)
    }
    ///10 PCLKA
    #[inline(always)]
    pub fn _0x08(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmdv::_0x08)
    }
    ///11 PCLKA (divided by an odd number)
    #[inline(always)]
    pub fn _0x09(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmdv::_0x09)
    }
    ///12 PCLKA
    #[inline(always)]
    pub fn _0x0a(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmdv::_0x0a)
    }
    ///13 PCLKA (divided by an odd number)
    #[inline(always)]
    pub fn _0x0b(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmdv::_0x0b)
    }
    ///14 PCLKA
    #[inline(always)]
    pub fn _0x0c(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmdv::_0x0c)
    }
    ///15 PCLKA (divided by an odd number)
    #[inline(always)]
    pub fn _0x0d(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmdv::_0x0d)
    }
    ///16 PCLKA
    #[inline(always)]
    pub fn _0x0e(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmdv::_0x0e)
    }
    ///17 PCLKA (divided by an odd number)
    #[inline(always)]
    pub fn _0x0f(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmdv::_0x0f)
    }
    ///18 PCLKA
    #[inline(always)]
    pub fn _0x10(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmdv::_0x10)
    }
    ///20 PCLKA
    #[inline(always)]
    pub fn _0x11(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmdv::_0x11)
    }
    ///22 PCLKA
    #[inline(always)]
    pub fn _0x12(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmdv::_0x12)
    }
    ///24 PCLKA
    #[inline(always)]
    pub fn _0x13(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmdv::_0x13)
    }
    ///26 PCLKA
    #[inline(always)]
    pub fn _0x14(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmdv::_0x14)
    }
    ///28 PCLKA
    #[inline(always)]
    pub fn _0x15(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmdv::_0x15)
    }
    ///30 PCLKA
    #[inline(always)]
    pub fn _0x16(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmdv::_0x16)
    }
    ///32 PCLKA
    #[inline(always)]
    pub fn _0x17(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmdv::_0x17)
    }
    ///34 PCLKA
    #[inline(always)]
    pub fn _0x18(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmdv::_0x18)
    }
    ///36 PCLKA
    #[inline(always)]
    pub fn _0x19(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmdv::_0x19)
    }
    ///38 PCLKA
    #[inline(always)]
    pub fn _0x1a(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmdv::_0x1a)
    }
    ///40 PCLKA
    #[inline(always)]
    pub fn _0x1b(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmdv::_0x1b)
    }
    ///42 PCLKA
    #[inline(always)]
    pub fn _0x1c(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmdv::_0x1c)
    }
    ///44 PCLKA
    #[inline(always)]
    pub fn _0x1d(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmdv::_0x1d)
    }
    ///46 PCLKA
    #[inline(always)]
    pub fn _0x1e(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmdv::_0x1e)
    }
    ///48 PCLKA
    #[inline(always)]
    pub fn _0x1f(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmdv::_0x1f)
    }
}
/**Duty ratio correction function select for the QSPCLK signal when devided by an odd number

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sfmdty {
    ///0: Make no correction
    _0 = 0,
    ///1: Make correction
    _1 = 1,
}
impl From<Sfmdty> for bool {
    #[inline(always)]
    fn from(variant: Sfmdty) -> Self {
        variant as u8 != 0
    }
}
///Field `SFMDTY` reader - Duty ratio correction function select for the QSPCLK signal when devided by an odd number
pub type SfmdtyR = crate::BitReader<Sfmdty>;
impl SfmdtyR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sfmdty {
        match self.bits {
            false => Sfmdty::_0,
            true => Sfmdty::_1,
        }
    }
    ///Make no correction
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sfmdty::_0
    }
    ///Make correction
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sfmdty::_1
    }
}
///Field `SFMDTY` writer - Duty ratio correction function select for the QSPCLK signal when devided by an odd number
pub type SfmdtyW<'a, REG> = crate::BitWriter<'a, REG, Sfmdty>;
impl<'a, REG> SfmdtyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Make no correction
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmdty::_0)
    }
    ///Make correction
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmdty::_1)
    }
}
impl R {
    ///Bits 0:4 - Serial interface reference cycle select. (Pay attention to irregularities.)
    #[inline(always)]
    pub fn sfmdv(&self) -> SfmdvR {
        SfmdvR::new((self.bits & 0x1f) as u8)
    }
    ///Bit 5 - Duty ratio correction function select for the QSPCLK signal when devided by an odd number
    #[inline(always)]
    pub fn sfmdty(&self) -> SfmdtyR {
        SfmdtyR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SFMSKC")
            .field("sfmdv", &self.sfmdv())
            .field("sfmdty", &self.sfmdty())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - Serial interface reference cycle select. (Pay attention to irregularities.)
    #[inline(always)]
    pub fn sfmdv(&mut self) -> SfmdvW<SfmskcSpec> {
        SfmdvW::new(self, 0)
    }
    ///Bit 5 - Duty ratio correction function select for the QSPCLK signal when devided by an odd number
    #[inline(always)]
    pub fn sfmdty(&mut self) -> SfmdtyW<SfmskcSpec> {
        SfmdtyW::new(self, 5)
    }
}
/**Clock Control Register

You can [`read`](crate::Reg::read) this register and get [`sfmskc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfmskc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SfmskcSpec;
impl crate::RegisterSpec for SfmskcSpec {
    type Ux = u32;
}
///`read()` method returns [`sfmskc::R`](R) reader structure
impl crate::Readable for SfmskcSpec {}
///`write(|w| ..)` method takes [`sfmskc::W`](W) writer structure
impl crate::Writable for SfmskcSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SFMSKC to value 0x08
impl crate::Resettable for SfmskcSpec {
    const RESET_VALUE: u32 = 0x08;
}
