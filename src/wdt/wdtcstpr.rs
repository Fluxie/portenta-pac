///Register `WDTCSTPR` reader
pub type R = crate::R<WdtcstprSpec>;
///Register `WDTCSTPR` writer
pub type W = crate::W<WdtcstprSpec>;
/**Sleep-Mode Count Stop Control Register

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slcstp {
    ///0: Disable count stop
    _0 = 0,
    ///1: Stop count on transition to Sleep mode
    _1 = 1,
}
impl From<Slcstp> for bool {
    #[inline(always)]
    fn from(variant: Slcstp) -> Self {
        variant as u8 != 0
    }
}
///Field `SLCSTP` reader - Sleep-Mode Count Stop Control Register
pub type SlcstpR = crate::BitReader<Slcstp>;
impl SlcstpR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Slcstp {
        match self.bits {
            false => Slcstp::_0,
            true => Slcstp::_1,
        }
    }
    ///Disable count stop
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Slcstp::_0
    }
    ///Stop count on transition to Sleep mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Slcstp::_1
    }
}
///Field `SLCSTP` writer - Sleep-Mode Count Stop Control Register
pub type SlcstpW<'a, REG> = crate::BitWriter<'a, REG, Slcstp>;
impl<'a, REG> SlcstpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable count stop
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Slcstp::_0)
    }
    ///Stop count on transition to Sleep mode
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Slcstp::_1)
    }
}
impl R {
    ///Bit 7 - Sleep-Mode Count Stop Control Register
    #[inline(always)]
    pub fn slcstp(&self) -> SlcstpR {
        SlcstpR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDTCSTPR").field("slcstp", &self.slcstp()).finish()
    }
}
impl W {
    ///Bit 7 - Sleep-Mode Count Stop Control Register
    #[inline(always)]
    pub fn slcstp(&mut self) -> SlcstpW<WdtcstprSpec> {
        SlcstpW::new(self, 7)
    }
}
/**WDT Count Stop Control Register

You can [`read`](crate::Reg::read) this register and get [`wdtcstpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtcstpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct WdtcstprSpec;
impl crate::RegisterSpec for WdtcstprSpec {
    type Ux = u8;
}
///`read()` method returns [`wdtcstpr::R`](R) reader structure
impl crate::Readable for WdtcstprSpec {}
///`write(|w| ..)` method takes [`wdtcstpr::W`](W) writer structure
impl crate::Writable for WdtcstprSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WDTCSTPR to value 0x80
impl crate::Resettable for WdtcstprSpec {
    const RESET_VALUE: u8 = 0x80;
}
