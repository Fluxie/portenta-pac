///Register `FLLCR1` reader
pub type R = crate::R<Fllcr1Spec>;
///Register `FLLCR1` writer
pub type W = crate::W<Fllcr1Spec>;
/**FLL Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fllen {
    ///0: FLL function is disabled
    _0 = 0,
    ///1: FLL function is enabled.
    _1 = 1,
}
impl From<Fllen> for bool {
    #[inline(always)]
    fn from(variant: Fllen) -> Self {
        variant as u8 != 0
    }
}
///Field `FLLEN` reader - FLL Enable
pub type FllenR = crate::BitReader<Fllen>;
impl FllenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Fllen {
        match self.bits {
            false => Fllen::_0,
            true => Fllen::_1,
        }
    }
    ///FLL function is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Fllen::_0
    }
    ///FLL function is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Fllen::_1
    }
}
///Field `FLLEN` writer - FLL Enable
pub type FllenW<'a, REG> = crate::BitWriter<'a, REG, Fllen>;
impl<'a, REG> FllenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///FLL function is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Fllen::_0)
    }
    ///FLL function is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Fllen::_1)
    }
}
impl R {
    ///Bit 0 - FLL Enable
    #[inline(always)]
    pub fn fllen(&self) -> FllenR {
        FllenR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLLCR1").field("fllen", &self.fllen()).finish()
    }
}
impl W {
    ///Bit 0 - FLL Enable
    #[inline(always)]
    pub fn fllen(&mut self) -> FllenW<Fllcr1Spec> {
        FllenW::new(self, 0)
    }
}
/**FLL Control Register1

You can [`read`](crate::Reg::read) this register and get [`fllcr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fllcr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Fllcr1Spec;
impl crate::RegisterSpec for Fllcr1Spec {
    type Ux = u8;
}
///`read()` method returns [`fllcr1::R`](R) reader structure
impl crate::Readable for Fllcr1Spec {}
///`write(|w| ..)` method takes [`fllcr1::W`](W) writer structure
impl crate::Writable for Fllcr1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FLLCR1 to value 0
impl crate::Resettable for Fllcr1Spec {}
