///Register `EBCKOCR` reader
pub type R = crate::R<EbckocrSpec>;
///Register `EBCKOCR` writer
pub type W = crate::W<EbckocrSpec>;
/**EBCLK Pin Output Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ebckoen {
    ///0: EBCLK pin output is disabled (fixed high)
    _0 = 0,
    ///1: EBCLK pin output is enabled.
    _1 = 1,
}
impl From<Ebckoen> for bool {
    #[inline(always)]
    fn from(variant: Ebckoen) -> Self {
        variant as u8 != 0
    }
}
///Field `EBCKOEN` reader - EBCLK Pin Output Control
pub type EbckoenR = crate::BitReader<Ebckoen>;
impl EbckoenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ebckoen {
        match self.bits {
            false => Ebckoen::_0,
            true => Ebckoen::_1,
        }
    }
    ///EBCLK pin output is disabled (fixed high)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ebckoen::_0
    }
    ///EBCLK pin output is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ebckoen::_1
    }
}
///Field `EBCKOEN` writer - EBCLK Pin Output Control
pub type EbckoenW<'a, REG> = crate::BitWriter<'a, REG, Ebckoen>;
impl<'a, REG> EbckoenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///EBCLK pin output is disabled (fixed high)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ebckoen::_0)
    }
    ///EBCLK pin output is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ebckoen::_1)
    }
}
impl R {
    ///Bit 0 - EBCLK Pin Output Control
    #[inline(always)]
    pub fn ebckoen(&self) -> EbckoenR {
        EbckoenR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EBCKOCR").field("ebckoen", &self.ebckoen()).finish()
    }
}
impl W {
    ///Bit 0 - EBCLK Pin Output Control
    #[inline(always)]
    pub fn ebckoen(&mut self) -> EbckoenW<EbckocrSpec> {
        EbckoenW::new(self, 0)
    }
}
/**External Bus Clock Output Control Register

You can [`read`](crate::Reg::read) this register and get [`ebckocr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ebckocr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct EbckocrSpec;
impl crate::RegisterSpec for EbckocrSpec {
    type Ux = u8;
}
///`read()` method returns [`ebckocr::R`](R) reader structure
impl crate::Readable for EbckocrSpec {}
///`write(|w| ..)` method takes [`ebckocr::W`](W) writer structure
impl crate::Writable for EbckocrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EBCKOCR to value 0
impl crate::Resettable for EbckocrSpec {}
