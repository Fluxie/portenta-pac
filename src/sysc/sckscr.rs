///Register `SCKSCR` reader
pub type R = crate::R<SckscrSpec>;
///Register `SCKSCR` writer
pub type W = crate::W<SckscrSpec>;
/**Clock Source Select

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cksel {
    ///0: HOCO
    _000 = 0,
    ///1: MOCO
    _001 = 1,
    ///2: LOCO
    _010 = 2,
    ///3: Main clock oscillator (MOSC)
    _011 = 3,
    ///4: Sub-clock oscillator (SOSC)
    _100 = 4,
    ///5: PLL
    _101 = 5,
    ///6: Setting prohibited
    _110 = 6,
    ///7: Setting prohibited
    _111 = 7,
}
impl From<Cksel> for u8 {
    #[inline(always)]
    fn from(variant: Cksel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cksel {
    type Ux = u8;
}
impl crate::IsEnum for Cksel {}
///Field `CKSEL` reader - Clock Source Select
pub type CkselR = crate::FieldReader<Cksel>;
impl CkselR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cksel {
        match self.bits {
            0 => Cksel::_000,
            1 => Cksel::_001,
            2 => Cksel::_010,
            3 => Cksel::_011,
            4 => Cksel::_100,
            5 => Cksel::_101,
            6 => Cksel::_110,
            7 => Cksel::_111,
            _ => unreachable!(),
        }
    }
    ///HOCO
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Cksel::_000
    }
    ///MOCO
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Cksel::_001
    }
    ///LOCO
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Cksel::_010
    }
    ///Main clock oscillator (MOSC)
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Cksel::_011
    }
    ///Sub-clock oscillator (SOSC)
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Cksel::_100
    }
    ///PLL
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Cksel::_101
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Cksel::_110
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == Cksel::_111
    }
}
///Field `CKSEL` writer - Clock Source Select
pub type CkselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Cksel, crate::Safe>;
impl<'a, REG> CkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///HOCO
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::_000)
    }
    ///MOCO
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::_001)
    }
    ///LOCO
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::_010)
    }
    ///Main clock oscillator (MOSC)
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::_011)
    }
    ///Sub-clock oscillator (SOSC)
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::_100)
    }
    ///PLL
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::_101)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::_110)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::_111)
    }
}
impl R {
    ///Bits 0:2 - Clock Source Select
    #[inline(always)]
    pub fn cksel(&self) -> CkselR {
        CkselR::new(self.bits & 7)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCKSCR").field("cksel", &self.cksel()).finish()
    }
}
impl W {
    ///Bits 0:2 - Clock Source Select
    #[inline(always)]
    pub fn cksel(&mut self) -> CkselW<SckscrSpec> {
        CkselW::new(self, 0)
    }
}
/**System Clock Source Control Register

You can [`read`](crate::Reg::read) this register and get [`sckscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sckscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SckscrSpec;
impl crate::RegisterSpec for SckscrSpec {
    type Ux = u8;
}
///`read()` method returns [`sckscr::R`](R) reader structure
impl crate::Readable for SckscrSpec {}
///`write(|w| ..)` method takes [`sckscr::W`](W) writer structure
impl crate::Writable for SckscrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SCKSCR to value 0x01
impl crate::Resettable for SckscrSpec {
    const RESET_VALUE: u8 = 0x01;
}
