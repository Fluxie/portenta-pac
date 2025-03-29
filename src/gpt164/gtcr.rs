///Register `GTCR` reader
pub type R = crate::R<GtcrSpec>;
///Register `GTCR` writer
pub type W = crate::W<GtcrSpec>;
/**Count Start

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cst {
    ///0: Count operation is stopped
    _0 = 0,
    ///1: Count operation is performed
    _1 = 1,
}
impl From<Cst> for bool {
    #[inline(always)]
    fn from(variant: Cst) -> Self {
        variant as u8 != 0
    }
}
///Field `CST` reader - Count Start
pub type CstR = crate::BitReader<Cst>;
impl CstR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cst {
        match self.bits {
            false => Cst::_0,
            true => Cst::_1,
        }
    }
    ///Count operation is stopped
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cst::_0
    }
    ///Count operation is performed
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cst::_1
    }
}
///Field `CST` writer - Count Start
pub type CstW<'a, REG> = crate::BitWriter<'a, REG, Cst>;
impl<'a, REG> CstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Count operation is stopped
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cst::_0)
    }
    ///Count operation is performed
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cst::_1)
    }
}
/**Mode Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Md {
    ///0: Saw-wave PWM mode (single buffer or double buffer possible)
    _000 = 0,
    ///1: Saw-wave one-shot pulse mode (fixed buffer operation)
    _001 = 1,
    ///2: Setting prohibited
    _010 = 2,
    ///3: Setting prohibited
    _011 = 3,
    ///4: Triangle-wave PWM mode 1 (32-bit transfer at trough) (single buffer or double buffer is possible)
    _100 = 4,
    ///5: Triangle-wave PWM mode 2 (32-bit transfer at crest and trough) (single buffer or double buffer is possible)
    _101 = 5,
    ///6: Triangle-wave PWM mode 3 (64-bit transfer at trough) (fixed buffer operation)
    _110 = 6,
    ///7: Setting prohibited
    _111 = 7,
}
impl From<Md> for u8 {
    #[inline(always)]
    fn from(variant: Md) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Md {
    type Ux = u8;
}
impl crate::IsEnum for Md {}
///Field `MD` reader - Mode Select
pub type MdR = crate::FieldReader<Md>;
impl MdR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Md {
        match self.bits {
            0 => Md::_000,
            1 => Md::_001,
            2 => Md::_010,
            3 => Md::_011,
            4 => Md::_100,
            5 => Md::_101,
            6 => Md::_110,
            7 => Md::_111,
            _ => unreachable!(),
        }
    }
    ///Saw-wave PWM mode (single buffer or double buffer possible)
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Md::_000
    }
    ///Saw-wave one-shot pulse mode (fixed buffer operation)
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Md::_001
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Md::_010
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Md::_011
    }
    ///Triangle-wave PWM mode 1 (32-bit transfer at trough) (single buffer or double buffer is possible)
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Md::_100
    }
    ///Triangle-wave PWM mode 2 (32-bit transfer at crest and trough) (single buffer or double buffer is possible)
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Md::_101
    }
    ///Triangle-wave PWM mode 3 (64-bit transfer at trough) (fixed buffer operation)
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Md::_110
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == Md::_111
    }
}
///Field `MD` writer - Mode Select
pub type MdW<'a, REG> = crate::FieldWriter<'a, REG, 3, Md, crate::Safe>;
impl<'a, REG> MdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Saw-wave PWM mode (single buffer or double buffer possible)
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Md::_000)
    }
    ///Saw-wave one-shot pulse mode (fixed buffer operation)
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Md::_001)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Md::_010)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Md::_011)
    }
    ///Triangle-wave PWM mode 1 (32-bit transfer at trough) (single buffer or double buffer is possible)
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Md::_100)
    }
    ///Triangle-wave PWM mode 2 (32-bit transfer at crest and trough) (single buffer or double buffer is possible)
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Md::_101)
    }
    ///Triangle-wave PWM mode 3 (64-bit transfer at trough) (fixed buffer operation)
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Md::_110)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(Md::_111)
    }
}
/**Timer Prescaler Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tpcs {
    ///0: PCLKD/1
    _0x0 = 0,
    ///1: PCLKD/2
    _0x1 = 1,
    ///2: PCLKD/4
    _0x2 = 2,
    ///3: PCLKD/8
    _0x3 = 3,
    ///4: PCLKD/16
    _0x4 = 4,
    ///5: PCLKD/32
    _0x5 = 5,
    ///6: PCLKD/64
    _0x6 = 6,
    ///7: Setting prohibited
    _0x7 = 7,
    ///8: PCLKD/256
    _0x8 = 8,
    ///9: Setting prohibited
    _0x9 = 9,
    ///10: PCLKD/1024
    _0xA = 10,
    ///11: Setting prohibited
    _0xB = 11,
    ///12: GTETRGA (Via the POEG)
    _0xC = 12,
    ///13: GTETRGB (Via the POEG)
    _0xD = 13,
    ///14: GTETRGC (Via the POEG)
    _0xE = 14,
    ///15: GTETRGD (Via the POEG)
    _0xF = 15,
}
impl From<Tpcs> for u8 {
    #[inline(always)]
    fn from(variant: Tpcs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tpcs {
    type Ux = u8;
}
impl crate::IsEnum for Tpcs {}
///Field `TPCS` reader - Timer Prescaler Select
pub type TpcsR = crate::FieldReader<Tpcs>;
impl TpcsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tpcs {
        match self.bits {
            0 => Tpcs::_0x0,
            1 => Tpcs::_0x1,
            2 => Tpcs::_0x2,
            3 => Tpcs::_0x3,
            4 => Tpcs::_0x4,
            5 => Tpcs::_0x5,
            6 => Tpcs::_0x6,
            7 => Tpcs::_0x7,
            8 => Tpcs::_0x8,
            9 => Tpcs::_0x9,
            10 => Tpcs::_0xA,
            11 => Tpcs::_0xB,
            12 => Tpcs::_0xC,
            13 => Tpcs::_0xD,
            14 => Tpcs::_0xE,
            15 => Tpcs::_0xF,
            _ => unreachable!(),
        }
    }
    ///PCLKD/1
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == Tpcs::_0x0
    }
    ///PCLKD/2
    #[inline(always)]
    pub fn is_0x1(&self) -> bool {
        *self == Tpcs::_0x1
    }
    ///PCLKD/4
    #[inline(always)]
    pub fn is_0x2(&self) -> bool {
        *self == Tpcs::_0x2
    }
    ///PCLKD/8
    #[inline(always)]
    pub fn is_0x3(&self) -> bool {
        *self == Tpcs::_0x3
    }
    ///PCLKD/16
    #[inline(always)]
    pub fn is_0x4(&self) -> bool {
        *self == Tpcs::_0x4
    }
    ///PCLKD/32
    #[inline(always)]
    pub fn is_0x5(&self) -> bool {
        *self == Tpcs::_0x5
    }
    ///PCLKD/64
    #[inline(always)]
    pub fn is_0x6(&self) -> bool {
        *self == Tpcs::_0x6
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_0x7(&self) -> bool {
        *self == Tpcs::_0x7
    }
    ///PCLKD/256
    #[inline(always)]
    pub fn is_0x8(&self) -> bool {
        *self == Tpcs::_0x8
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_0x9(&self) -> bool {
        *self == Tpcs::_0x9
    }
    ///PCLKD/1024
    #[inline(always)]
    pub fn is_0x_a(&self) -> bool {
        *self == Tpcs::_0xA
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_0x_b(&self) -> bool {
        *self == Tpcs::_0xB
    }
    ///GTETRGA (Via the POEG)
    #[inline(always)]
    pub fn is_0x_c(&self) -> bool {
        *self == Tpcs::_0xC
    }
    ///GTETRGB (Via the POEG)
    #[inline(always)]
    pub fn is_0x_d(&self) -> bool {
        *self == Tpcs::_0xD
    }
    ///GTETRGC (Via the POEG)
    #[inline(always)]
    pub fn is_0x_e(&self) -> bool {
        *self == Tpcs::_0xE
    }
    ///GTETRGD (Via the POEG)
    #[inline(always)]
    pub fn is_0x_f(&self) -> bool {
        *self == Tpcs::_0xF
    }
}
///Field `TPCS` writer - Timer Prescaler Select
pub type TpcsW<'a, REG> = crate::FieldWriter<'a, REG, 4, Tpcs, crate::Safe>;
impl<'a, REG> TpcsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PCLKD/1
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tpcs::_0x0)
    }
    ///PCLKD/2
    #[inline(always)]
    pub fn _0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tpcs::_0x1)
    }
    ///PCLKD/4
    #[inline(always)]
    pub fn _0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Tpcs::_0x2)
    }
    ///PCLKD/8
    #[inline(always)]
    pub fn _0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Tpcs::_0x3)
    }
    ///PCLKD/16
    #[inline(always)]
    pub fn _0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Tpcs::_0x4)
    }
    ///PCLKD/32
    #[inline(always)]
    pub fn _0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Tpcs::_0x5)
    }
    ///PCLKD/64
    #[inline(always)]
    pub fn _0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Tpcs::_0x6)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Tpcs::_0x7)
    }
    ///PCLKD/256
    #[inline(always)]
    pub fn _0x8(self) -> &'a mut crate::W<REG> {
        self.variant(Tpcs::_0x8)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _0x9(self) -> &'a mut crate::W<REG> {
        self.variant(Tpcs::_0x9)
    }
    ///PCLKD/1024
    #[inline(always)]
    pub fn _0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(Tpcs::_0xA)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(Tpcs::_0xB)
    }
    ///GTETRGA (Via the POEG)
    #[inline(always)]
    pub fn _0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(Tpcs::_0xC)
    }
    ///GTETRGB (Via the POEG)
    #[inline(always)]
    pub fn _0x_d(self) -> &'a mut crate::W<REG> {
        self.variant(Tpcs::_0xD)
    }
    ///GTETRGC (Via the POEG)
    #[inline(always)]
    pub fn _0x_e(self) -> &'a mut crate::W<REG> {
        self.variant(Tpcs::_0xE)
    }
    ///GTETRGD (Via the POEG)
    #[inline(always)]
    pub fn _0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(Tpcs::_0xF)
    }
}
impl R {
    ///Bit 0 - Count Start
    #[inline(always)]
    pub fn cst(&self) -> CstR {
        CstR::new((self.bits & 1) != 0)
    }
    ///Bits 16:18 - Mode Select
    #[inline(always)]
    pub fn md(&self) -> MdR {
        MdR::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bits 23:26 - Timer Prescaler Select
    #[inline(always)]
    pub fn tpcs(&self) -> TpcsR {
        TpcsR::new(((self.bits >> 23) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GTCR")
            .field("cst", &self.cst())
            .field("md", &self.md())
            .field("tpcs", &self.tpcs())
            .finish()
    }
}
impl W {
    ///Bit 0 - Count Start
    #[inline(always)]
    pub fn cst(&mut self) -> CstW<GtcrSpec> {
        CstW::new(self, 0)
    }
    ///Bits 16:18 - Mode Select
    #[inline(always)]
    pub fn md(&mut self) -> MdW<GtcrSpec> {
        MdW::new(self, 16)
    }
    ///Bits 23:26 - Timer Prescaler Select
    #[inline(always)]
    pub fn tpcs(&mut self) -> TpcsW<GtcrSpec> {
        TpcsW::new(self, 23)
    }
}
/**General PWM Timer Control Register

You can [`read`](crate::Reg::read) this register and get [`gtcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GtcrSpec;
impl crate::RegisterSpec for GtcrSpec {
    type Ux = u32;
}
///`read()` method returns [`gtcr::R`](R) reader structure
impl crate::Readable for GtcrSpec {}
///`write(|w| ..)` method takes [`gtcr::W`](W) writer structure
impl crate::Writable for GtcrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTCR to value 0
impl crate::Resettable for GtcrSpec {}
