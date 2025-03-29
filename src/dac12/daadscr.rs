///Register `DAADSCR` reader
pub type R = crate::R<DaadscrSpec>;
///Register `DAADSCR` writer
pub type W = crate::W<DaadscrSpec>;
/**D/A A/D Synchronous Conversion

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Daadst {
    ///0: Do not synchronize DAC12 with ADC12 (unit 1) operation (disable interference reduction between D/A and A/D conversion).
    _0 = 0,
    ///1: Synchronize DAC12 with ADC12 (unit 1) operation (enable interference reduction between D/A and A/D conversion).
    _1 = 1,
}
impl From<Daadst> for bool {
    #[inline(always)]
    fn from(variant: Daadst) -> Self {
        variant as u8 != 0
    }
}
///Field `DAADST` reader - D/A A/D Synchronous Conversion
pub type DaadstR = crate::BitReader<Daadst>;
impl DaadstR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Daadst {
        match self.bits {
            false => Daadst::_0,
            true => Daadst::_1,
        }
    }
    ///Do not synchronize DAC12 with ADC12 (unit 1) operation (disable interference reduction between D/A and A/D conversion).
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Daadst::_0
    }
    ///Synchronize DAC12 with ADC12 (unit 1) operation (enable interference reduction between D/A and A/D conversion).
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Daadst::_1
    }
}
///Field `DAADST` writer - D/A A/D Synchronous Conversion
pub type DaadstW<'a, REG> = crate::BitWriter<'a, REG, Daadst>;
impl<'a, REG> DaadstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not synchronize DAC12 with ADC12 (unit 1) operation (disable interference reduction between D/A and A/D conversion).
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Daadst::_0)
    }
    ///Synchronize DAC12 with ADC12 (unit 1) operation (enable interference reduction between D/A and A/D conversion).
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Daadst::_1)
    }
}
impl R {
    ///Bit 7 - D/A A/D Synchronous Conversion
    #[inline(always)]
    pub fn daadst(&self) -> DaadstR {
        DaadstR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAADSCR").field("daadst", &self.daadst()).finish()
    }
}
impl W {
    ///Bit 7 - D/A A/D Synchronous Conversion
    #[inline(always)]
    pub fn daadst(&mut self) -> DaadstW<DaadscrSpec> {
        DaadstW::new(self, 7)
    }
}
/**D/A A/D Synchronous Start Control Register

You can [`read`](crate::Reg::read) this register and get [`daadscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daadscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DaadscrSpec;
impl crate::RegisterSpec for DaadscrSpec {
    type Ux = u8;
}
///`read()` method returns [`daadscr::R`](R) reader structure
impl crate::Readable for DaadscrSpec {}
///`write(|w| ..)` method takes [`daadscr::W`](W) writer structure
impl crate::Writable for DaadscrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DAADSCR to value 0
impl crate::Resettable for DaadscrSpec {}
