///Register `SPND` reader
pub type R = crate::R<SpndSpec>;
///Register `SPND` writer
pub type W = crate::W<SpndSpec>;
/**SPI Next-Access Delay Setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Spndl {
    ///0: 1 RSPCK + 2 PCLKA
    _000 = 0,
    ///1: 2 RSPCK + 2 PCLKA
    _001 = 1,
    ///2: 3 RSPCK + 2 PCLKA
    _010 = 2,
    ///3: 4 RSPCK + 2 PCLKA
    _011 = 3,
    ///4: 5 RSPCK + 2 PCLKA
    _100 = 4,
    ///5: 6 RSPCK + 2 PCLKA
    _101 = 5,
    ///6: 7 RSPCK + 2 PCLKA
    _110 = 6,
    ///7: 8 RSPCK + 2 PCLKA
    _111 = 7,
}
impl From<Spndl> for u8 {
    #[inline(always)]
    fn from(variant: Spndl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Spndl {
    type Ux = u8;
}
impl crate::IsEnum for Spndl {}
///Field `SPNDL` reader - SPI Next-Access Delay Setting
pub type SpndlR = crate::FieldReader<Spndl>;
impl SpndlR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Spndl {
        match self.bits {
            0 => Spndl::_000,
            1 => Spndl::_001,
            2 => Spndl::_010,
            3 => Spndl::_011,
            4 => Spndl::_100,
            5 => Spndl::_101,
            6 => Spndl::_110,
            7 => Spndl::_111,
            _ => unreachable!(),
        }
    }
    ///1 RSPCK + 2 PCLKA
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Spndl::_000
    }
    ///2 RSPCK + 2 PCLKA
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Spndl::_001
    }
    ///3 RSPCK + 2 PCLKA
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Spndl::_010
    }
    ///4 RSPCK + 2 PCLKA
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Spndl::_011
    }
    ///5 RSPCK + 2 PCLKA
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Spndl::_100
    }
    ///6 RSPCK + 2 PCLKA
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Spndl::_101
    }
    ///7 RSPCK + 2 PCLKA
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Spndl::_110
    }
    ///8 RSPCK + 2 PCLKA
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == Spndl::_111
    }
}
///Field `SPNDL` writer - SPI Next-Access Delay Setting
pub type SpndlW<'a, REG> = crate::FieldWriter<'a, REG, 3, Spndl, crate::Safe>;
impl<'a, REG> SpndlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///1 RSPCK + 2 PCLKA
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Spndl::_000)
    }
    ///2 RSPCK + 2 PCLKA
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Spndl::_001)
    }
    ///3 RSPCK + 2 PCLKA
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Spndl::_010)
    }
    ///4 RSPCK + 2 PCLKA
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Spndl::_011)
    }
    ///5 RSPCK + 2 PCLKA
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Spndl::_100)
    }
    ///6 RSPCK + 2 PCLKA
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Spndl::_101)
    }
    ///7 RSPCK + 2 PCLKA
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Spndl::_110)
    }
    ///8 RSPCK + 2 PCLKA
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(Spndl::_111)
    }
}
impl R {
    ///Bits 0:2 - SPI Next-Access Delay Setting
    #[inline(always)]
    pub fn spndl(&self) -> SpndlR {
        SpndlR::new(self.bits & 7)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPND").field("spndl", &self.spndl()).finish()
    }
}
impl W {
    ///Bits 0:2 - SPI Next-Access Delay Setting
    #[inline(always)]
    pub fn spndl(&mut self) -> SpndlW<SpndSpec> {
        SpndlW::new(self, 0)
    }
}
/**SPI Next-Access Delay Register

You can [`read`](crate::Reg::read) this register and get [`spnd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spnd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SpndSpec;
impl crate::RegisterSpec for SpndSpec {
    type Ux = u8;
}
///`read()` method returns [`spnd::R`](R) reader structure
impl crate::Readable for SpndSpec {}
///`write(|w| ..)` method takes [`spnd::W`](W) writer structure
impl crate::Writable for SpndSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SPND to value 0
impl crate::Resettable for SpndSpec {}
