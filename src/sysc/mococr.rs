///Register `MOCOCR` reader
pub type R = crate::R<MococrSpec>;
///Register `MOCOCR` writer
pub type W = crate::W<MococrSpec>;
/**MOCO Stop

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mcstp {
    ///0: MOCO clock is operating
    _0 = 0,
    ///1: MOCO clock is stopped
    _1 = 1,
}
impl From<Mcstp> for bool {
    #[inline(always)]
    fn from(variant: Mcstp) -> Self {
        variant as u8 != 0
    }
}
///Field `MCSTP` reader - MOCO Stop
pub type McstpR = crate::BitReader<Mcstp>;
impl McstpR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mcstp {
        match self.bits {
            false => Mcstp::_0,
            true => Mcstp::_1,
        }
    }
    ///MOCO clock is operating
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mcstp::_0
    }
    ///MOCO clock is stopped
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mcstp::_1
    }
}
///Field `MCSTP` writer - MOCO Stop
pub type McstpW<'a, REG> = crate::BitWriter<'a, REG, Mcstp>;
impl<'a, REG> McstpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///MOCO clock is operating
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mcstp::_0)
    }
    ///MOCO clock is stopped
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mcstp::_1)
    }
}
impl R {
    ///Bit 0 - MOCO Stop
    #[inline(always)]
    pub fn mcstp(&self) -> McstpR {
        McstpR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MOCOCR").field("mcstp", &self.mcstp()).finish()
    }
}
impl W {
    ///Bit 0 - MOCO Stop
    #[inline(always)]
    pub fn mcstp(&mut self) -> McstpW<MococrSpec> {
        McstpW::new(self, 0)
    }
}
/**Middle-Speed On-Chip Oscillator Control Register

You can [`read`](crate::Reg::read) this register and get [`mococr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mococr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MococrSpec;
impl crate::RegisterSpec for MococrSpec {
    type Ux = u8;
}
///`read()` method returns [`mococr::R`](R) reader structure
impl crate::Readable for MococrSpec {}
///`write(|w| ..)` method takes [`mococr::W`](W) writer structure
impl crate::Writable for MococrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MOCOCR to value 0
impl crate::Resettable for MococrSpec {}
