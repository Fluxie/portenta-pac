///Register `RCR4` reader
pub type R = crate::R<Rcr4Spec>;
///Register `RCR4` writer
pub type W = crate::W<Rcr4Spec>;
/**Count Source Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rcksel {
    ///0: Sub-clock oscillator is selected
    _0 = 0,
    ///1: LOCO is selected
    _1 = 1,
}
impl From<Rcksel> for bool {
    #[inline(always)]
    fn from(variant: Rcksel) -> Self {
        variant as u8 != 0
    }
}
///Field `RCKSEL` reader - Count Source Select
pub type RckselR = crate::BitReader<Rcksel>;
impl RckselR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rcksel {
        match self.bits {
            false => Rcksel::_0,
            true => Rcksel::_1,
        }
    }
    ///Sub-clock oscillator is selected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rcksel::_0
    }
    ///LOCO is selected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rcksel::_1
    }
}
///Field `RCKSEL` writer - Count Source Select
pub type RckselW<'a, REG> = crate::BitWriter<'a, REG, Rcksel>;
impl<'a, REG> RckselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Sub-clock oscillator is selected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rcksel::_0)
    }
    ///LOCO is selected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rcksel::_1)
    }
}
impl R {
    ///Bit 0 - Count Source Select
    #[inline(always)]
    pub fn rcksel(&self) -> RckselR {
        RckselR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCR4").field("rcksel", &self.rcksel()).finish()
    }
}
impl W {
    ///Bit 0 - Count Source Select
    #[inline(always)]
    pub fn rcksel(&mut self) -> RckselW<Rcr4Spec> {
        RckselW::new(self, 0)
    }
}
/**RTC Control Register 4

You can [`read`](crate::Reg::read) this register and get [`rcr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Rcr4Spec;
impl crate::RegisterSpec for Rcr4Spec {
    type Ux = u8;
}
///`read()` method returns [`rcr4::R`](R) reader structure
impl crate::Readable for Rcr4Spec {}
///`write(|w| ..)` method takes [`rcr4::W`](W) writer structure
impl crate::Writable for Rcr4Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RCR4 to value 0
impl crate::Resettable for Rcr4Spec {}
