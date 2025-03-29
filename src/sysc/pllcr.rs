///Register `PLLCR` reader
pub type R = crate::R<PllcrSpec>;
///Register `PLLCR` writer
pub type W = crate::W<PllcrSpec>;
/**PLL Stop Control

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pllstp {
    ///0: PLL is operating
    _0 = 0,
    ///1: PLL is stopped.
    _1 = 1,
}
impl From<Pllstp> for bool {
    #[inline(always)]
    fn from(variant: Pllstp) -> Self {
        variant as u8 != 0
    }
}
///Field `PLLSTP` reader - PLL Stop Control
pub type PllstpR = crate::BitReader<Pllstp>;
impl PllstpR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pllstp {
        match self.bits {
            false => Pllstp::_0,
            true => Pllstp::_1,
        }
    }
    ///PLL is operating
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pllstp::_0
    }
    ///PLL is stopped.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pllstp::_1
    }
}
///Field `PLLSTP` writer - PLL Stop Control
pub type PllstpW<'a, REG> = crate::BitWriter<'a, REG, Pllstp>;
impl<'a, REG> PllstpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PLL is operating
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pllstp::_0)
    }
    ///PLL is stopped.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pllstp::_1)
    }
}
impl R {
    ///Bit 0 - PLL Stop Control
    #[inline(always)]
    pub fn pllstp(&self) -> PllstpR {
        PllstpR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLLCR").field("pllstp", &self.pllstp()).finish()
    }
}
impl W {
    ///Bit 0 - PLL Stop Control
    #[inline(always)]
    pub fn pllstp(&mut self) -> PllstpW<PllcrSpec> {
        PllstpW::new(self, 0)
    }
}
/**PLL Control Register

You can [`read`](crate::Reg::read) this register and get [`pllcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PllcrSpec;
impl crate::RegisterSpec for PllcrSpec {
    type Ux = u8;
}
///`read()` method returns [`pllcr::R`](R) reader structure
impl crate::Readable for PllcrSpec {}
///`write(|w| ..)` method takes [`pllcr::W`](W) writer structure
impl crate::Writable for PllcrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PLLCR to value 0x01
impl crate::Resettable for PllcrSpec {
    const RESET_VALUE: u8 = 0x01;
}
