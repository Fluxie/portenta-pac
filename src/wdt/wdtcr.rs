///Register `WDTCR` reader
pub type R = crate::R<WdtcrSpec>;
///Register `WDTCR` writer
pub type W = crate::W<WdtcrSpec>;
/**Timeout Period Select

Value on reset: 3*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tops {
    ///0: 1024 cycles (0x03FF)
    _00 = 0,
    ///1: 4096 cycles (0x0FFF)
    _01 = 1,
    ///2: 8192 cycles (0x1FFF)
    _10 = 2,
    ///3: 16384 cycles (0x3FFF)
    _11 = 3,
}
impl From<Tops> for u8 {
    #[inline(always)]
    fn from(variant: Tops) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tops {
    type Ux = u8;
}
impl crate::IsEnum for Tops {}
///Field `TOPS` reader - Timeout Period Select
pub type TopsR = crate::FieldReader<Tops>;
impl TopsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tops {
        match self.bits {
            0 => Tops::_00,
            1 => Tops::_01,
            2 => Tops::_10,
            3 => Tops::_11,
            _ => unreachable!(),
        }
    }
    ///1024 cycles (0x03FF)
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Tops::_00
    }
    ///4096 cycles (0x0FFF)
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Tops::_01
    }
    ///8192 cycles (0x1FFF)
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Tops::_10
    }
    ///16384 cycles (0x3FFF)
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Tops::_11
    }
}
///Field `TOPS` writer - Timeout Period Select
pub type TopsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tops, crate::Safe>;
impl<'a, REG> TopsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///1024 cycles (0x03FF)
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Tops::_00)
    }
    ///4096 cycles (0x0FFF)
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Tops::_01)
    }
    ///8192 cycles (0x1FFF)
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Tops::_10)
    }
    ///16384 cycles (0x3FFF)
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Tops::_11)
    }
}
/**Clock Division Ratio Select

Value on reset: 15*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cks {
    ///1: PCLKB/4
    _0x1 = 1,
    ///4: PCLKB/64
    _0x4 = 4,
    ///15: PCLKB/128
    _0xF = 15,
    ///6: PCLKB/512
    _0x6 = 6,
    ///7: PCLKB/2048
    _0x7 = 7,
    ///8: PCLKB/8192
    _0x8 = 8,
    ///0: Setting prohibited
    Others = 0,
}
impl From<Cks> for u8 {
    #[inline(always)]
    fn from(variant: Cks) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cks {
    type Ux = u8;
}
impl crate::IsEnum for Cks {}
///Field `CKS` reader - Clock Division Ratio Select
pub type CksR = crate::FieldReader<Cks>;
impl CksR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cks {
        match self.bits {
            1 => Cks::_0x1,
            4 => Cks::_0x4,
            15 => Cks::_0xF,
            6 => Cks::_0x6,
            7 => Cks::_0x7,
            8 => Cks::_0x8,
            _ => Cks::Others,
        }
    }
    ///PCLKB/4
    #[inline(always)]
    pub fn is_0x1(&self) -> bool {
        *self == Cks::_0x1
    }
    ///PCLKB/64
    #[inline(always)]
    pub fn is_0x4(&self) -> bool {
        *self == Cks::_0x4
    }
    ///PCLKB/128
    #[inline(always)]
    pub fn is_0x_f(&self) -> bool {
        *self == Cks::_0xF
    }
    ///PCLKB/512
    #[inline(always)]
    pub fn is_0x6(&self) -> bool {
        *self == Cks::_0x6
    }
    ///PCLKB/2048
    #[inline(always)]
    pub fn is_0x7(&self) -> bool {
        *self == Cks::_0x7
    }
    ///PCLKB/8192
    #[inline(always)]
    pub fn is_0x8(&self) -> bool {
        *self == Cks::_0x8
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Cks::Others)
    }
}
///Field `CKS` writer - Clock Division Ratio Select
pub type CksW<'a, REG> = crate::FieldWriter<'a, REG, 4, Cks, crate::Safe>;
impl<'a, REG> CksW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PCLKB/4
    #[inline(always)]
    pub fn _0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Cks::_0x1)
    }
    ///PCLKB/64
    #[inline(always)]
    pub fn _0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Cks::_0x4)
    }
    ///PCLKB/128
    #[inline(always)]
    pub fn _0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(Cks::_0xF)
    }
    ///PCLKB/512
    #[inline(always)]
    pub fn _0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Cks::_0x6)
    }
    ///PCLKB/2048
    #[inline(always)]
    pub fn _0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Cks::_0x7)
    }
    ///PCLKB/8192
    #[inline(always)]
    pub fn _0x8(self) -> &'a mut crate::W<REG> {
        self.variant(Cks::_0x8)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Cks::Others)
    }
}
/**Window End Position Select

Value on reset: 3*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rpes {
    ///0: 75%
    _00 = 0,
    ///1: 50%
    _01 = 1,
    ///2: 25%
    _10 = 2,
    ///3: 0% (do not specify window end position).
    _11 = 3,
}
impl From<Rpes> for u8 {
    #[inline(always)]
    fn from(variant: Rpes) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rpes {
    type Ux = u8;
}
impl crate::IsEnum for Rpes {}
///Field `RPES` reader - Window End Position Select
pub type RpesR = crate::FieldReader<Rpes>;
impl RpesR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rpes {
        match self.bits {
            0 => Rpes::_00,
            1 => Rpes::_01,
            2 => Rpes::_10,
            3 => Rpes::_11,
            _ => unreachable!(),
        }
    }
    ///75%
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Rpes::_00
    }
    ///50%
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Rpes::_01
    }
    ///25%
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Rpes::_10
    }
    ///0% (do not specify window end position).
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Rpes::_11
    }
}
///Field `RPES` writer - Window End Position Select
pub type RpesW<'a, REG> = crate::FieldWriter<'a, REG, 2, Rpes, crate::Safe>;
impl<'a, REG> RpesW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///75%
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Rpes::_00)
    }
    ///50%
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Rpes::_01)
    }
    ///25%
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Rpes::_10)
    }
    ///0% (do not specify window end position).
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Rpes::_11)
    }
}
/**Window Start Position Select

Value on reset: 3*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rpss {
    ///0: 25%
    _00 = 0,
    ///1: 50%
    _01 = 1,
    ///2: 75%
    _10 = 2,
    ///3: 100% (do not specify window start position).
    _11 = 3,
}
impl From<Rpss> for u8 {
    #[inline(always)]
    fn from(variant: Rpss) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rpss {
    type Ux = u8;
}
impl crate::IsEnum for Rpss {}
///Field `RPSS` reader - Window Start Position Select
pub type RpssR = crate::FieldReader<Rpss>;
impl RpssR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rpss {
        match self.bits {
            0 => Rpss::_00,
            1 => Rpss::_01,
            2 => Rpss::_10,
            3 => Rpss::_11,
            _ => unreachable!(),
        }
    }
    ///25%
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Rpss::_00
    }
    ///50%
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Rpss::_01
    }
    ///75%
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Rpss::_10
    }
    ///100% (do not specify window start position).
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Rpss::_11
    }
}
///Field `RPSS` writer - Window Start Position Select
pub type RpssW<'a, REG> = crate::FieldWriter<'a, REG, 2, Rpss, crate::Safe>;
impl<'a, REG> RpssW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///25%
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Rpss::_00)
    }
    ///50%
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Rpss::_01)
    }
    ///75%
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Rpss::_10)
    }
    ///100% (do not specify window start position).
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Rpss::_11)
    }
}
impl R {
    ///Bits 0:1 - Timeout Period Select
    #[inline(always)]
    pub fn tops(&self) -> TopsR {
        TopsR::new((self.bits & 3) as u8)
    }
    ///Bits 4:7 - Clock Division Ratio Select
    #[inline(always)]
    pub fn cks(&self) -> CksR {
        CksR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:9 - Window End Position Select
    #[inline(always)]
    pub fn rpes(&self) -> RpesR {
        RpesR::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 12:13 - Window Start Position Select
    #[inline(always)]
    pub fn rpss(&self) -> RpssR {
        RpssR::new(((self.bits >> 12) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDTCR")
            .field("tops", &self.tops())
            .field("cks", &self.cks())
            .field("rpes", &self.rpes())
            .field("rpss", &self.rpss())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Timeout Period Select
    #[inline(always)]
    pub fn tops(&mut self) -> TopsW<WdtcrSpec> {
        TopsW::new(self, 0)
    }
    ///Bits 4:7 - Clock Division Ratio Select
    #[inline(always)]
    pub fn cks(&mut self) -> CksW<WdtcrSpec> {
        CksW::new(self, 4)
    }
    ///Bits 8:9 - Window End Position Select
    #[inline(always)]
    pub fn rpes(&mut self) -> RpesW<WdtcrSpec> {
        RpesW::new(self, 8)
    }
    ///Bits 12:13 - Window Start Position Select
    #[inline(always)]
    pub fn rpss(&mut self) -> RpssW<WdtcrSpec> {
        RpssW::new(self, 12)
    }
}
/**WDT Control Register

You can [`read`](crate::Reg::read) this register and get [`wdtcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct WdtcrSpec;
impl crate::RegisterSpec for WdtcrSpec {
    type Ux = u16;
}
///`read()` method returns [`wdtcr::R`](R) reader structure
impl crate::Readable for WdtcrSpec {}
///`write(|w| ..)` method takes [`wdtcr::W`](W) writer structure
impl crate::Writable for WdtcrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WDTCR to value 0x33f3
impl crate::Resettable for WdtcrSpec {
    const RESET_VALUE: u16 = 0x33f3;
}
