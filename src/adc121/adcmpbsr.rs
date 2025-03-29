///Register `ADCMPBSR` reader
pub type R = crate::R<AdcmpbsrSpec>;
///Register `ADCMPBSR` writer
pub type W = crate::W<AdcmpbsrSpec>;
/**Compare Window B Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpstb {
    ///0: Comparison conditions are not met.
    _0 = 0,
    ///1: Comparison conditions are met.
    _1 = 1,
}
impl From<Cmpstb> for bool {
    #[inline(always)]
    fn from(variant: Cmpstb) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPSTB` reader - Compare Window B Flag
pub type CmpstbR = crate::BitReader<Cmpstb>;
impl CmpstbR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmpstb {
        match self.bits {
            false => Cmpstb::_0,
            true => Cmpstb::_1,
        }
    }
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpstb::_0
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpstb::_1
    }
}
///Field `CMPSTB` writer - Compare Window B Flag
pub type CmpstbW<'a, REG> = crate::BitWriter<'a, REG, Cmpstb>;
impl<'a, REG> CmpstbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstb::_0)
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstb::_1)
    }
}
impl R {
    ///Bit 0 - Compare Window B Flag
    #[inline(always)]
    pub fn cmpstb(&self) -> CmpstbR {
        CmpstbR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCMPBSR").field("cmpstb", &self.cmpstb()).finish()
    }
}
impl W {
    ///Bit 0 - Compare Window B Flag
    #[inline(always)]
    pub fn cmpstb(&mut self) -> CmpstbW<AdcmpbsrSpec> {
        CmpstbW::new(self, 0)
    }
}
/**A/D Compare Function Window B Status Register

You can [`read`](crate::Reg::read) this register and get [`adcmpbsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpbsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AdcmpbsrSpec;
impl crate::RegisterSpec for AdcmpbsrSpec {
    type Ux = u8;
}
///`read()` method returns [`adcmpbsr::R`](R) reader structure
impl crate::Readable for AdcmpbsrSpec {}
///`write(|w| ..)` method takes [`adcmpbsr::W`](W) writer structure
impl crate::Writable for AdcmpbsrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADCMPBSR to value 0
impl crate::Resettable for AdcmpbsrSpec {}
