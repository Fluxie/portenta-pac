///Register `BCKCR` reader
pub type R = crate::R<BckcrSpec>;
///Register `BCKCR` writer
pub type W = crate::W<BckcrSpec>;
/**BCLK Pin Output Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bclkdiv {
    ///0: BCLK
    _0 = 0,
    ///1: BCLK ∕ 2.
    _1 = 1,
}
impl From<Bclkdiv> for bool {
    #[inline(always)]
    fn from(variant: Bclkdiv) -> Self {
        variant as u8 != 0
    }
}
///Field `BCLKDIV` reader - BCLK Pin Output Select
pub type BclkdivR = crate::BitReader<Bclkdiv>;
impl BclkdivR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Bclkdiv {
        match self.bits {
            false => Bclkdiv::_0,
            true => Bclkdiv::_1,
        }
    }
    ///BCLK
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bclkdiv::_0
    }
    ///BCLK ∕ 2.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bclkdiv::_1
    }
}
///Field `BCLKDIV` writer - BCLK Pin Output Select
pub type BclkdivW<'a, REG> = crate::BitWriter<'a, REG, Bclkdiv>;
impl<'a, REG> BclkdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///BCLK
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bclkdiv::_0)
    }
    ///BCLK ∕ 2.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bclkdiv::_1)
    }
}
impl R {
    ///Bit 0 - BCLK Pin Output Select
    #[inline(always)]
    pub fn bclkdiv(&self) -> BclkdivR {
        BclkdivR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BCKCR").field("bclkdiv", &self.bclkdiv()).finish()
    }
}
impl W {
    ///Bit 0 - BCLK Pin Output Select
    #[inline(always)]
    pub fn bclkdiv(&mut self) -> BclkdivW<BckcrSpec> {
        BclkdivW::new(self, 0)
    }
}
/**External Bus Clock Control Register

You can [`read`](crate::Reg::read) this register and get [`bckcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bckcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BckcrSpec;
impl crate::RegisterSpec for BckcrSpec {
    type Ux = u8;
}
///`read()` method returns [`bckcr::R`](R) reader structure
impl crate::Readable for BckcrSpec {}
///`write(|w| ..)` method takes [`bckcr::W`](W) writer structure
impl crate::Writable for BckcrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BCKCR to value 0
impl crate::Resettable for BckcrSpec {}
