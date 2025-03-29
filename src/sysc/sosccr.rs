///Register `SOSCCR` reader
pub type R = crate::R<SosccrSpec>;
///Register `SOSCCR` writer
pub type W = crate::W<SosccrSpec>;
/**Sub Clock Oscillator Stop

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sostp {
    ///0: Operate the sub-clock oscillator
    _0 = 0,
    ///1: Stop the sub-clock oscillator
    _1 = 1,
}
impl From<Sostp> for bool {
    #[inline(always)]
    fn from(variant: Sostp) -> Self {
        variant as u8 != 0
    }
}
///Field `SOSTP` reader - Sub Clock Oscillator Stop
pub type SostpR = crate::BitReader<Sostp>;
impl SostpR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sostp {
        match self.bits {
            false => Sostp::_0,
            true => Sostp::_1,
        }
    }
    ///Operate the sub-clock oscillator
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sostp::_0
    }
    ///Stop the sub-clock oscillator
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sostp::_1
    }
}
///Field `SOSTP` writer - Sub Clock Oscillator Stop
pub type SostpW<'a, REG> = crate::BitWriter<'a, REG, Sostp>;
impl<'a, REG> SostpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Operate the sub-clock oscillator
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sostp::_0)
    }
    ///Stop the sub-clock oscillator
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sostp::_1)
    }
}
impl R {
    ///Bit 0 - Sub Clock Oscillator Stop
    #[inline(always)]
    pub fn sostp(&self) -> SostpR {
        SostpR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SOSCCR").field("sostp", &self.sostp()).finish()
    }
}
impl W {
    ///Bit 0 - Sub Clock Oscillator Stop
    #[inline(always)]
    pub fn sostp(&mut self) -> SostpW<SosccrSpec> {
        SostpW::new(self, 0)
    }
}
/**Sub-Clock Oscillator Control Register

You can [`read`](crate::Reg::read) this register and get [`sosccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sosccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SosccrSpec;
impl crate::RegisterSpec for SosccrSpec {
    type Ux = u8;
}
///`read()` method returns [`sosccr::R`](R) reader structure
impl crate::Readable for SosccrSpec {}
///`write(|w| ..)` method takes [`sosccr::W`](W) writer structure
impl crate::Writable for SosccrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SOSCCR to value 0
impl crate::Resettable for SosccrSpec {}
