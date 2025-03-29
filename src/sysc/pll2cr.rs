///Register `PLL2CR` reader
pub type R = crate::R<Pll2crSpec>;
///Register `PLL2CR` writer
pub type W = crate::W<Pll2crSpec>;
/**PLL2 Stop Control

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pll2stp {
    ///0: PLL2 is operating
    _0 = 0,
    ///1: PLL2 is stopped.
    _1 = 1,
}
impl From<Pll2stp> for bool {
    #[inline(always)]
    fn from(variant: Pll2stp) -> Self {
        variant as u8 != 0
    }
}
///Field `PLL2STP` reader - PLL2 Stop Control
pub type Pll2stpR = crate::BitReader<Pll2stp>;
impl Pll2stpR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pll2stp {
        match self.bits {
            false => Pll2stp::_0,
            true => Pll2stp::_1,
        }
    }
    ///PLL2 is operating
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pll2stp::_0
    }
    ///PLL2 is stopped.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pll2stp::_1
    }
}
///Field `PLL2STP` writer - PLL2 Stop Control
pub type Pll2stpW<'a, REG> = crate::BitWriter<'a, REG, Pll2stp>;
impl<'a, REG> Pll2stpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PLL2 is operating
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pll2stp::_0)
    }
    ///PLL2 is stopped.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pll2stp::_1)
    }
}
impl R {
    ///Bit 0 - PLL2 Stop Control
    #[inline(always)]
    pub fn pll2stp(&self) -> Pll2stpR {
        Pll2stpR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL2CR").field("pll2stp", &self.pll2stp()).finish()
    }
}
impl W {
    ///Bit 0 - PLL2 Stop Control
    #[inline(always)]
    pub fn pll2stp(&mut self) -> Pll2stpW<Pll2crSpec> {
        Pll2stpW::new(self, 0)
    }
}
/**PLL2 Control Register

You can [`read`](crate::Reg::read) this register and get [`pll2cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll2cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Pll2crSpec;
impl crate::RegisterSpec for Pll2crSpec {
    type Ux = u8;
}
///`read()` method returns [`pll2cr::R`](R) reader structure
impl crate::Readable for Pll2crSpec {}
///`write(|w| ..)` method takes [`pll2cr::W`](W) writer structure
impl crate::Writable for Pll2crSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PLL2CR to value 0x01
impl crate::Resettable for Pll2crSpec {
    const RESET_VALUE: u8 = 0x01;
}
