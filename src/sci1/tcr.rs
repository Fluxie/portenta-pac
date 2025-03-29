///Register `TCR` reader
pub type R = crate::R<TcrSpec>;
///Register `TCR` writer
pub type W = crate::W<TcrSpec>;
/**Timer Count Start

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcst {
    ///0: Stops the timer counting
    _0 = 0,
    ///1: Starts the timer counting
    _1 = 1,
}
impl From<Tcst> for bool {
    #[inline(always)]
    fn from(variant: Tcst) -> Self {
        variant as u8 != 0
    }
}
///Field `TCST` reader - Timer Count Start
pub type TcstR = crate::BitReader<Tcst>;
impl TcstR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tcst {
        match self.bits {
            false => Tcst::_0,
            true => Tcst::_1,
        }
    }
    ///Stops the timer counting
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tcst::_0
    }
    ///Starts the timer counting
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tcst::_1
    }
}
///Field `TCST` writer - Timer Count Start
pub type TcstW<'a, REG> = crate::BitWriter<'a, REG, Tcst>;
impl<'a, REG> TcstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Stops the timer counting
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tcst::_0)
    }
    ///Starts the timer counting
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tcst::_1)
    }
}
impl R {
    ///Bit 0 - Timer Count Start
    #[inline(always)]
    pub fn tcst(&self) -> TcstR {
        TcstR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TCR").field("tcst", &self.tcst()).finish()
    }
}
impl W {
    ///Bit 0 - Timer Count Start
    #[inline(always)]
    pub fn tcst(&mut self) -> TcstW<TcrSpec> {
        TcstW::new(self, 0)
    }
}
/**Timer Control Register

You can [`read`](crate::Reg::read) this register and get [`tcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TcrSpec;
impl crate::RegisterSpec for TcrSpec {
    type Ux = u8;
}
///`read()` method returns [`tcr::R`](R) reader structure
impl crate::Readable for TcrSpec {}
///`write(|w| ..)` method takes [`tcr::W`](W) writer structure
impl crate::Writable for TcrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TCR to value 0
impl crate::Resettable for TcrSpec {}
