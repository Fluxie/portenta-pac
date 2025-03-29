///Register `DMAST` reader
pub type R = crate::R<DmastSpec>;
///Register `DMAST` writer
pub type W = crate::W<DmastSpec>;
/**DMAC Operation Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmst {
    ///0: DMAC activation is disabled.
    _0 = 0,
    ///1: DMAC activation is enabled.
    _1 = 1,
}
impl From<Dmst> for bool {
    #[inline(always)]
    fn from(variant: Dmst) -> Self {
        variant as u8 != 0
    }
}
///Field `DMST` reader - DMAC Operation Enable
pub type DmstR = crate::BitReader<Dmst>;
impl DmstR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dmst {
        match self.bits {
            false => Dmst::_0,
            true => Dmst::_1,
        }
    }
    ///DMAC activation is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dmst::_0
    }
    ///DMAC activation is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dmst::_1
    }
}
///Field `DMST` writer - DMAC Operation Enable
pub type DmstW<'a, REG> = crate::BitWriter<'a, REG, Dmst>;
impl<'a, REG> DmstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DMAC activation is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dmst::_0)
    }
    ///DMAC activation is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dmst::_1)
    }
}
impl R {
    ///Bit 0 - DMAC Operation Enable
    #[inline(always)]
    pub fn dmst(&self) -> DmstR {
        DmstR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAST").field("dmst", &self.dmst()).finish()
    }
}
impl W {
    ///Bit 0 - DMAC Operation Enable
    #[inline(always)]
    pub fn dmst(&mut self) -> DmstW<DmastSpec> {
        DmstW::new(self, 0)
    }
}
/**DMA Module Activation Register

You can [`read`](crate::Reg::read) this register and get [`dmast::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmast::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DmastSpec;
impl crate::RegisterSpec for DmastSpec {
    type Ux = u8;
}
///`read()` method returns [`dmast::R`](R) reader structure
impl crate::Readable for DmastSpec {}
///`write(|w| ..)` method takes [`dmast::W`](W) writer structure
impl crate::Writable for DmastSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMAST to value 0
impl crate::Resettable for DmastSpec {}
