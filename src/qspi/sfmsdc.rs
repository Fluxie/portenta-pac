///Register `SFMSDC` reader
pub type R = crate::R<SfmsdcSpec>;
///Register `SFMSDC` writer
pub type W = crate::W<SfmsdcSpec>;
/**Number of dummy cycles select for Fast Read instructions

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sfmdn {
    ///0: Default dummy cycles for each instruction: - Fast Read Quad I/O: 6 QSPCLK - Fast Read Quad Output: 8 QSPCLK - Fast Read Dual I/O: 4 QSPCLK - Fast Read Dual Output: 8 QSPCLK - Fast Read: 8 QSPCLK
    _0x0 = 0,
    ///1: 3 QSPCLK
    _0x1 = 1,
    ///2: 4 QSPCLK
    _0x2 = 2,
    ///3: 5 QSPCLK
    _0x3 = 3,
    ///4: 6 QSPCLK
    _0x4 = 4,
    ///5: 7 QSPCLK
    _0x5 = 5,
    ///6: 8 QSPCLK
    _0x6 = 6,
    ///7: 9 QSPCLK
    _0x7 = 7,
    ///8: 10 QSPCLK
    _0x8 = 8,
    ///9: 11 QSPCLK
    _0x9 = 9,
    ///10: 12 QSPCLK
    _0xA = 10,
    ///11: 13 QSPCLK
    _0xB = 11,
    ///12: 14 QSPCLK
    _0xC = 12,
    ///13: 15 QSPCLK
    _0xD = 13,
    ///14: 16 QSPCLK
    _0xE = 14,
    ///15: 17 QSPCLK
    _0xF = 15,
}
impl From<Sfmdn> for u8 {
    #[inline(always)]
    fn from(variant: Sfmdn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sfmdn {
    type Ux = u8;
}
impl crate::IsEnum for Sfmdn {}
///Field `SFMDN` reader - Number of dummy cycles select for Fast Read instructions
pub type SfmdnR = crate::FieldReader<Sfmdn>;
impl SfmdnR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sfmdn {
        match self.bits {
            0 => Sfmdn::_0x0,
            1 => Sfmdn::_0x1,
            2 => Sfmdn::_0x2,
            3 => Sfmdn::_0x3,
            4 => Sfmdn::_0x4,
            5 => Sfmdn::_0x5,
            6 => Sfmdn::_0x6,
            7 => Sfmdn::_0x7,
            8 => Sfmdn::_0x8,
            9 => Sfmdn::_0x9,
            10 => Sfmdn::_0xA,
            11 => Sfmdn::_0xB,
            12 => Sfmdn::_0xC,
            13 => Sfmdn::_0xD,
            14 => Sfmdn::_0xE,
            15 => Sfmdn::_0xF,
            _ => unreachable!(),
        }
    }
    ///Default dummy cycles for each instruction: - Fast Read Quad I/O: 6 QSPCLK - Fast Read Quad Output: 8 QSPCLK - Fast Read Dual I/O: 4 QSPCLK - Fast Read Dual Output: 8 QSPCLK - Fast Read: 8 QSPCLK
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == Sfmdn::_0x0
    }
    ///3 QSPCLK
    #[inline(always)]
    pub fn is_0x1(&self) -> bool {
        *self == Sfmdn::_0x1
    }
    ///4 QSPCLK
    #[inline(always)]
    pub fn is_0x2(&self) -> bool {
        *self == Sfmdn::_0x2
    }
    ///5 QSPCLK
    #[inline(always)]
    pub fn is_0x3(&self) -> bool {
        *self == Sfmdn::_0x3
    }
    ///6 QSPCLK
    #[inline(always)]
    pub fn is_0x4(&self) -> bool {
        *self == Sfmdn::_0x4
    }
    ///7 QSPCLK
    #[inline(always)]
    pub fn is_0x5(&self) -> bool {
        *self == Sfmdn::_0x5
    }
    ///8 QSPCLK
    #[inline(always)]
    pub fn is_0x6(&self) -> bool {
        *self == Sfmdn::_0x6
    }
    ///9 QSPCLK
    #[inline(always)]
    pub fn is_0x7(&self) -> bool {
        *self == Sfmdn::_0x7
    }
    ///10 QSPCLK
    #[inline(always)]
    pub fn is_0x8(&self) -> bool {
        *self == Sfmdn::_0x8
    }
    ///11 QSPCLK
    #[inline(always)]
    pub fn is_0x9(&self) -> bool {
        *self == Sfmdn::_0x9
    }
    ///12 QSPCLK
    #[inline(always)]
    pub fn is_0x_a(&self) -> bool {
        *self == Sfmdn::_0xA
    }
    ///13 QSPCLK
    #[inline(always)]
    pub fn is_0x_b(&self) -> bool {
        *self == Sfmdn::_0xB
    }
    ///14 QSPCLK
    #[inline(always)]
    pub fn is_0x_c(&self) -> bool {
        *self == Sfmdn::_0xC
    }
    ///15 QSPCLK
    #[inline(always)]
    pub fn is_0x_d(&self) -> bool {
        *self == Sfmdn::_0xD
    }
    ///16 QSPCLK
    #[inline(always)]
    pub fn is_0x_e(&self) -> bool {
        *self == Sfmdn::_0xE
    }
    ///17 QSPCLK
    #[inline(always)]
    pub fn is_0x_f(&self) -> bool {
        *self == Sfmdn::_0xF
    }
}
///Field `SFMDN` writer - Number of dummy cycles select for Fast Read instructions
pub type SfmdnW<'a, REG> = crate::FieldWriter<'a, REG, 4, Sfmdn, crate::Safe>;
impl<'a, REG> SfmdnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Default dummy cycles for each instruction: - Fast Read Quad I/O: 6 QSPCLK - Fast Read Quad Output: 8 QSPCLK - Fast Read Dual I/O: 4 QSPCLK - Fast Read Dual Output: 8 QSPCLK - Fast Read: 8 QSPCLK
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmdn::_0x0)
    }
    ///3 QSPCLK
    #[inline(always)]
    pub fn _0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmdn::_0x1)
    }
    ///4 QSPCLK
    #[inline(always)]
    pub fn _0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmdn::_0x2)
    }
    ///5 QSPCLK
    #[inline(always)]
    pub fn _0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmdn::_0x3)
    }
    ///6 QSPCLK
    #[inline(always)]
    pub fn _0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmdn::_0x4)
    }
    ///7 QSPCLK
    #[inline(always)]
    pub fn _0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmdn::_0x5)
    }
    ///8 QSPCLK
    #[inline(always)]
    pub fn _0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmdn::_0x6)
    }
    ///9 QSPCLK
    #[inline(always)]
    pub fn _0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmdn::_0x7)
    }
    ///10 QSPCLK
    #[inline(always)]
    pub fn _0x8(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmdn::_0x8)
    }
    ///11 QSPCLK
    #[inline(always)]
    pub fn _0x9(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmdn::_0x9)
    }
    ///12 QSPCLK
    #[inline(always)]
    pub fn _0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmdn::_0xA)
    }
    ///13 QSPCLK
    #[inline(always)]
    pub fn _0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmdn::_0xB)
    }
    ///14 QSPCLK
    #[inline(always)]
    pub fn _0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmdn::_0xC)
    }
    ///15 QSPCLK
    #[inline(always)]
    pub fn _0x_d(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmdn::_0xD)
    }
    ///16 QSPCLK
    #[inline(always)]
    pub fn _0x_e(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmdn::_0xE)
    }
    ///17 QSPCLK
    #[inline(always)]
    pub fn _0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmdn::_0xF)
    }
}
/**XIP mode status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sfmxst {
    ///0: Normal (non-XIP) mode
    _0 = 0,
    ///1: XIP mode
    _1 = 1,
}
impl From<Sfmxst> for bool {
    #[inline(always)]
    fn from(variant: Sfmxst) -> Self {
        variant as u8 != 0
    }
}
///Field `SFMXST` reader - XIP mode status
pub type SfmxstR = crate::BitReader<Sfmxst>;
impl SfmxstR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sfmxst {
        match self.bits {
            false => Sfmxst::_0,
            true => Sfmxst::_1,
        }
    }
    ///Normal (non-XIP) mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sfmxst::_0
    }
    ///XIP mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sfmxst::_1
    }
}
/**XIP mode permission

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sfmxen {
    ///0: Prohibit XIP mode
    _0 = 0,
    ///1: Permit XIP mode
    _1 = 1,
}
impl From<Sfmxen> for bool {
    #[inline(always)]
    fn from(variant: Sfmxen) -> Self {
        variant as u8 != 0
    }
}
///Field `SFMXEN` reader - XIP mode permission
pub type SfmxenR = crate::BitReader<Sfmxen>;
impl SfmxenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sfmxen {
        match self.bits {
            false => Sfmxen::_0,
            true => Sfmxen::_1,
        }
    }
    ///Prohibit XIP mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sfmxen::_0
    }
    ///Permit XIP mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sfmxen::_1
    }
}
///Field `SFMXEN` writer - XIP mode permission
pub type SfmxenW<'a, REG> = crate::BitWriter<'a, REG, Sfmxen>;
impl<'a, REG> SfmxenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Prohibit XIP mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmxen::_0)
    }
    ///Permit XIP mode
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sfmxen::_1)
    }
}
///Field `SFMXD` reader - Mode data for serial flash (Controls XIP mode.)
pub type SfmxdR = crate::FieldReader;
///Field `SFMXD` writer - Mode data for serial flash (Controls XIP mode.)
pub type SfmxdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:3 - Number of dummy cycles select for Fast Read instructions
    #[inline(always)]
    pub fn sfmdn(&self) -> SfmdnR {
        SfmdnR::new((self.bits & 0x0f) as u8)
    }
    ///Bit 6 - XIP mode status
    #[inline(always)]
    pub fn sfmxst(&self) -> SfmxstR {
        SfmxstR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - XIP mode permission
    #[inline(always)]
    pub fn sfmxen(&self) -> SfmxenR {
        SfmxenR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:15 - Mode data for serial flash (Controls XIP mode.)
    #[inline(always)]
    pub fn sfmxd(&self) -> SfmxdR {
        SfmxdR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SFMSDC")
            .field("sfmdn", &self.sfmdn())
            .field("sfmxst", &self.sfmxst())
            .field("sfmxen", &self.sfmxen())
            .field("sfmxd", &self.sfmxd())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Number of dummy cycles select for Fast Read instructions
    #[inline(always)]
    pub fn sfmdn(&mut self) -> SfmdnW<SfmsdcSpec> {
        SfmdnW::new(self, 0)
    }
    ///Bit 7 - XIP mode permission
    #[inline(always)]
    pub fn sfmxen(&mut self) -> SfmxenW<SfmsdcSpec> {
        SfmxenW::new(self, 7)
    }
    ///Bits 8:15 - Mode data for serial flash (Controls XIP mode.)
    #[inline(always)]
    pub fn sfmxd(&mut self) -> SfmxdW<SfmsdcSpec> {
        SfmxdW::new(self, 8)
    }
}
/**Dummy Cycle Control Register

You can [`read`](crate::Reg::read) this register and get [`sfmsdc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfmsdc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SfmsdcSpec;
impl crate::RegisterSpec for SfmsdcSpec {
    type Ux = u32;
}
///`read()` method returns [`sfmsdc::R`](R) reader structure
impl crate::Readable for SfmsdcSpec {}
///`write(|w| ..)` method takes [`sfmsdc::W`](W) writer structure
impl crate::Writable for SfmsdcSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SFMSDC to value 0xff00
impl crate::Resettable for SfmsdcSpec {
    const RESET_VALUE: u32 = 0xff00;
}
