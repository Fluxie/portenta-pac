///Register `ECCETST` reader
pub type R = crate::R<EccetstSpec>;
///Register `ECCETST` writer
pub type W = crate::W<EccetstSpec>;
/**ECC Bypass Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tstbyp {
    ///0: Disable ECC bypass
    _0 = 0,
    ///1: Enable ECC bypass
    _1 = 1,
}
impl From<Tstbyp> for bool {
    #[inline(always)]
    fn from(variant: Tstbyp) -> Self {
        variant as u8 != 0
    }
}
///Field `TSTBYP` reader - ECC Bypass Select
pub type TstbypR = crate::BitReader<Tstbyp>;
impl TstbypR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tstbyp {
        match self.bits {
            false => Tstbyp::_0,
            true => Tstbyp::_1,
        }
    }
    ///Disable ECC bypass
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tstbyp::_0
    }
    ///Enable ECC bypass
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tstbyp::_1
    }
}
///Field `TSTBYP` writer - ECC Bypass Select
pub type TstbypW<'a, REG> = crate::BitWriter<'a, REG, Tstbyp>;
impl<'a, REG> TstbypW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable ECC bypass
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tstbyp::_0)
    }
    ///Enable ECC bypass
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tstbyp::_1)
    }
}
impl R {
    ///Bit 0 - ECC Bypass Select
    #[inline(always)]
    pub fn tstbyp(&self) -> TstbypR {
        TstbypR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECCETST").field("tstbyp", &self.tstbyp()).finish()
    }
}
impl W {
    ///Bit 0 - ECC Bypass Select
    #[inline(always)]
    pub fn tstbyp(&mut self) -> TstbypW<EccetstSpec> {
        TstbypW::new(self, 0)
    }
}
/**ECC Test Control Register

You can [`read`](crate::Reg::read) this register and get [`eccetst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eccetst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct EccetstSpec;
impl crate::RegisterSpec for EccetstSpec {
    type Ux = u8;
}
///`read()` method returns [`eccetst::R`](R) reader structure
impl crate::Readable for EccetstSpec {}
///`write(|w| ..)` method takes [`eccetst::W`](W) writer structure
impl crate::Writable for EccetstSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ECCETST to value 0
impl crate::Resettable for EccetstSpec {}
