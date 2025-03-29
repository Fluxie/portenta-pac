///Register `OSTDSR` reader
pub type R = crate::R<OstdsrSpec>;
///Register `OSTDSR` writer
pub type W = crate::W<OstdsrSpec>;
/**Oscillation Stop Detection Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ostdf {
    ///0: Main clock oscillation stop not detected
    _0 = 0,
    ///1: Main clock oscillation stop detected
    _1 = 1,
}
impl From<Ostdf> for bool {
    #[inline(always)]
    fn from(variant: Ostdf) -> Self {
        variant as u8 != 0
    }
}
///Field `OSTDF` reader - Oscillation Stop Detection Flag
pub type OstdfR = crate::BitReader<Ostdf>;
impl OstdfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ostdf {
        match self.bits {
            false => Ostdf::_0,
            true => Ostdf::_1,
        }
    }
    ///Main clock oscillation stop not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ostdf::_0
    }
    ///Main clock oscillation stop detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ostdf::_1
    }
}
///Field `OSTDF` writer - Oscillation Stop Detection Flag
pub type OstdfW<'a, REG> = crate::BitWriter<'a, REG, Ostdf>;
impl<'a, REG> OstdfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Main clock oscillation stop not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ostdf::_0)
    }
    ///Main clock oscillation stop detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ostdf::_1)
    }
}
impl R {
    ///Bit 0 - Oscillation Stop Detection Flag
    #[inline(always)]
    pub fn ostdf(&self) -> OstdfR {
        OstdfR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OSTDSR").field("ostdf", &self.ostdf()).finish()
    }
}
impl W {
    ///Bit 0 - Oscillation Stop Detection Flag
    #[inline(always)]
    pub fn ostdf(&mut self) -> OstdfW<OstdsrSpec> {
        OstdfW::new(self, 0)
    }
}
/**Oscillation Stop Detection Status Register

You can [`read`](crate::Reg::read) this register and get [`ostdsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ostdsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct OstdsrSpec;
impl crate::RegisterSpec for OstdsrSpec {
    type Ux = u8;
}
///`read()` method returns [`ostdsr::R`](R) reader structure
impl crate::Readable for OstdsrSpec {}
///`write(|w| ..)` method takes [`ostdsr::W`](W) writer structure
impl crate::Writable for OstdsrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OSTDSR to value 0
impl crate::Resettable for OstdsrSpec {}
