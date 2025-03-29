///Register `OCTACKDIVCR` reader
pub type R = crate::R<OctackdivcrSpec>;
///Register `OCTACKDIVCR` writer
pub type W = crate::W<OctackdivcrSpec>;
/**Octal-SPI Clock (OCTACLK) Division Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Octackdiv {
    ///0: ∕ 1 (value after reset)
    _000 = 0,
    ///1: ∕ 2
    _001 = 1,
    ///2: ∕ 4
    _010 = 2,
    ///3: ∕ 6
    _011 = 3,
    ///4: ∕ 8
    _100 = 4,
    ///5: Setting prohibited.
    Others = 5,
}
impl From<Octackdiv> for u8 {
    #[inline(always)]
    fn from(variant: Octackdiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Octackdiv {
    type Ux = u8;
}
impl crate::IsEnum for Octackdiv {}
///Field `OCTACKDIV` reader - Octal-SPI Clock (OCTACLK) Division Select
pub type OctackdivR = crate::FieldReader<Octackdiv>;
impl OctackdivR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Octackdiv {
        match self.bits {
            0 => Octackdiv::_000,
            1 => Octackdiv::_001,
            2 => Octackdiv::_010,
            3 => Octackdiv::_011,
            4 => Octackdiv::_100,
            _ => Octackdiv::Others,
        }
    }
    ///∕ 1 (value after reset)
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Octackdiv::_000
    }
    ///∕ 2
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Octackdiv::_001
    }
    ///∕ 4
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Octackdiv::_010
    }
    ///∕ 6
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Octackdiv::_011
    }
    ///∕ 8
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Octackdiv::_100
    }
    ///Setting prohibited.
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Octackdiv::Others)
    }
}
///Field `OCTACKDIV` writer - Octal-SPI Clock (OCTACLK) Division Select
pub type OctackdivW<'a, REG> = crate::FieldWriter<'a, REG, 3, Octackdiv, crate::Safe>;
impl<'a, REG> OctackdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///∕ 1 (value after reset)
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Octackdiv::_000)
    }
    ///∕ 2
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Octackdiv::_001)
    }
    ///∕ 4
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Octackdiv::_010)
    }
    ///∕ 6
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Octackdiv::_011)
    }
    ///∕ 8
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Octackdiv::_100)
    }
    ///Setting prohibited.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Octackdiv::Others)
    }
}
impl R {
    ///Bits 0:2 - Octal-SPI Clock (OCTACLK) Division Select
    #[inline(always)]
    pub fn octackdiv(&self) -> OctackdivR {
        OctackdivR::new(self.bits & 7)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OCTACKDIVCR").field("octackdiv", &self.octackdiv()).finish()
    }
}
impl W {
    ///Bits 0:2 - Octal-SPI Clock (OCTACLK) Division Select
    #[inline(always)]
    pub fn octackdiv(&mut self) -> OctackdivW<OctackdivcrSpec> {
        OctackdivW::new(self, 0)
    }
}
/**Octal-SPI Clock Division Control Register

You can [`read`](crate::Reg::read) this register and get [`octackdivcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`octackdivcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct OctackdivcrSpec;
impl crate::RegisterSpec for OctackdivcrSpec {
    type Ux = u8;
}
///`read()` method returns [`octackdivcr::R`](R) reader structure
impl crate::Readable for OctackdivcrSpec {}
///`write(|w| ..)` method takes [`octackdivcr::W`](W) writer structure
impl crate::Writable for OctackdivcrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OCTACKDIVCR to value 0
impl crate::Resettable for OctackdivcrSpec {}
