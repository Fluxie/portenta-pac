///Register `SPSCR` reader
pub type R = crate::R<SpscrSpec>;
///Register `SPSCR` writer
pub type W = crate::W<SpscrSpec>;
/**SPI Sequence Length Specification

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Spsln {
    ///0: Sequence Length is 1 (Referenced SPCMDn, n = 0→0→…)
    _000 = 0,
    ///1: Sequence Length is 2 (Referenced SPCMDn, n = 0→1→0→…)
    _001 = 1,
    ///2: Sequence Length is 3 (Referenced SPCMDn, n = 0→1→2→0→…)
    _010 = 2,
    ///3: Sequence Length is 4 (Referenced SPCMDn, n = 0→1→2→3→0→…)
    _011 = 3,
    ///4: Sequence Length is 5 (Referenced SPCMDn, n = 0→1→2→3→4→0→…)
    _100 = 4,
    ///5: Sequence Length is 6 (Referenced SPCMDn, n = 0→1→2→3→4→5→0→…)
    _101 = 5,
    ///6: Sequence Length is 7 (Referenced SPCMDn, n = 0→1→2→3→4→5→6→0→…)
    _110 = 6,
    ///7: Sequence Length is 8 (Referenced SPCMDn, n = 0→1→2→3→4→5→6→7→0→…)
    _111 = 7,
}
impl From<Spsln> for u8 {
    #[inline(always)]
    fn from(variant: Spsln) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Spsln {
    type Ux = u8;
}
impl crate::IsEnum for Spsln {}
///Field `SPSLN` reader - SPI Sequence Length Specification
pub type SpslnR = crate::FieldReader<Spsln>;
impl SpslnR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Spsln {
        match self.bits {
            0 => Spsln::_000,
            1 => Spsln::_001,
            2 => Spsln::_010,
            3 => Spsln::_011,
            4 => Spsln::_100,
            5 => Spsln::_101,
            6 => Spsln::_110,
            7 => Spsln::_111,
            _ => unreachable!(),
        }
    }
    ///Sequence Length is 1 (Referenced SPCMDn, n = 0→0→…)
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Spsln::_000
    }
    ///Sequence Length is 2 (Referenced SPCMDn, n = 0→1→0→…)
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Spsln::_001
    }
    ///Sequence Length is 3 (Referenced SPCMDn, n = 0→1→2→0→…)
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Spsln::_010
    }
    ///Sequence Length is 4 (Referenced SPCMDn, n = 0→1→2→3→0→…)
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Spsln::_011
    }
    ///Sequence Length is 5 (Referenced SPCMDn, n = 0→1→2→3→4→0→…)
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Spsln::_100
    }
    ///Sequence Length is 6 (Referenced SPCMDn, n = 0→1→2→3→4→5→0→…)
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Spsln::_101
    }
    ///Sequence Length is 7 (Referenced SPCMDn, n = 0→1→2→3→4→5→6→0→…)
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Spsln::_110
    }
    ///Sequence Length is 8 (Referenced SPCMDn, n = 0→1→2→3→4→5→6→7→0→…)
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == Spsln::_111
    }
}
///Field `SPSLN` writer - SPI Sequence Length Specification
pub type SpslnW<'a, REG> = crate::FieldWriter<'a, REG, 3, Spsln, crate::Safe>;
impl<'a, REG> SpslnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Sequence Length is 1 (Referenced SPCMDn, n = 0→0→…)
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Spsln::_000)
    }
    ///Sequence Length is 2 (Referenced SPCMDn, n = 0→1→0→…)
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Spsln::_001)
    }
    ///Sequence Length is 3 (Referenced SPCMDn, n = 0→1→2→0→…)
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Spsln::_010)
    }
    ///Sequence Length is 4 (Referenced SPCMDn, n = 0→1→2→3→0→…)
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Spsln::_011)
    }
    ///Sequence Length is 5 (Referenced SPCMDn, n = 0→1→2→3→4→0→…)
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Spsln::_100)
    }
    ///Sequence Length is 6 (Referenced SPCMDn, n = 0→1→2→3→4→5→0→…)
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Spsln::_101)
    }
    ///Sequence Length is 7 (Referenced SPCMDn, n = 0→1→2→3→4→5→6→0→…)
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Spsln::_110)
    }
    ///Sequence Length is 8 (Referenced SPCMDn, n = 0→1→2→3→4→5→6→7→0→…)
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(Spsln::_111)
    }
}
impl R {
    ///Bits 0:2 - SPI Sequence Length Specification
    #[inline(always)]
    pub fn spsln(&self) -> SpslnR {
        SpslnR::new(self.bits & 7)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPSCR").field("spsln", &self.spsln()).finish()
    }
}
impl W {
    ///Bits 0:2 - SPI Sequence Length Specification
    #[inline(always)]
    pub fn spsln(&mut self) -> SpslnW<SpscrSpec> {
        SpslnW::new(self, 0)
    }
}
/**SPI Sequence Control Register

You can [`read`](crate::Reg::read) this register and get [`spscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SpscrSpec;
impl crate::RegisterSpec for SpscrSpec {
    type Ux = u8;
}
///`read()` method returns [`spscr::R`](R) reader structure
impl crate::Readable for SpscrSpec {}
///`write(|w| ..)` method takes [`spscr::W`](W) writer structure
impl crate::Writable for SpscrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SPSCR to value 0
impl crate::Resettable for SpscrSpec {}
