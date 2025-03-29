///Register `SOMCR` reader
pub type R = crate::R<SomcrSpec>;
///Register `SOMCR` writer
pub type W = crate::W<SomcrSpec>;
/**Sub-Clock Oscillator Drive Capability Switching

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sodrv {
    ///0: Standard
    _0 = 0,
    ///1: Low
    _1 = 1,
}
impl From<Sodrv> for bool {
    #[inline(always)]
    fn from(variant: Sodrv) -> Self {
        variant as u8 != 0
    }
}
///Field `SODRV` reader - Sub-Clock Oscillator Drive Capability Switching
pub type SodrvR = crate::BitReader<Sodrv>;
impl SodrvR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sodrv {
        match self.bits {
            false => Sodrv::_0,
            true => Sodrv::_1,
        }
    }
    ///Standard
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sodrv::_0
    }
    ///Low
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sodrv::_1
    }
}
///Field `SODRV` writer - Sub-Clock Oscillator Drive Capability Switching
pub type SodrvW<'a, REG> = crate::BitWriter<'a, REG, Sodrv>;
impl<'a, REG> SodrvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Standard
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sodrv::_0)
    }
    ///Low
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sodrv::_1)
    }
}
impl R {
    ///Bit 1 - Sub-Clock Oscillator Drive Capability Switching
    #[inline(always)]
    pub fn sodrv(&self) -> SodrvR {
        SodrvR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SOMCR").field("sodrv", &self.sodrv()).finish()
    }
}
impl W {
    ///Bit 1 - Sub-Clock Oscillator Drive Capability Switching
    #[inline(always)]
    pub fn sodrv(&mut self) -> SodrvW<SomcrSpec> {
        SodrvW::new(self, 1)
    }
}
/**Sub-Clock Oscillator Mode Control Register

You can [`read`](crate::Reg::read) this register and get [`somcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`somcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SomcrSpec;
impl crate::RegisterSpec for SomcrSpec {
    type Ux = u8;
}
///`read()` method returns [`somcr::R`](R) reader structure
impl crate::Readable for SomcrSpec {}
///`write(|w| ..)` method takes [`somcr::W`](W) writer structure
impl crate::Writable for SomcrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SOMCR to value 0
impl crate::Resettable for SomcrSpec {}
