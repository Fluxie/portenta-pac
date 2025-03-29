///Register `LPSTS` reader
pub type R = crate::R<LpstsSpec>;
///Register `LPSTS` writer
pub type W = crate::W<LpstsSpec>;
/**UTMI SuspendM Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Suspendm {
    ///0: UTMI suspension mode
    _0 = 0,
    ///1: UTMI normal mode
    _1 = 1,
}
impl From<Suspendm> for bool {
    #[inline(always)]
    fn from(variant: Suspendm) -> Self {
        variant as u8 != 0
    }
}
///Field `SUSPENDM` reader - UTMI SuspendM Control
pub type SuspendmR = crate::BitReader<Suspendm>;
impl SuspendmR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Suspendm {
        match self.bits {
            false => Suspendm::_0,
            true => Suspendm::_1,
        }
    }
    ///UTMI suspension mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Suspendm::_0
    }
    ///UTMI normal mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Suspendm::_1
    }
}
///Field `SUSPENDM` writer - UTMI SuspendM Control
pub type SuspendmW<'a, REG> = crate::BitWriter<'a, REG, Suspendm>;
impl<'a, REG> SuspendmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///UTMI suspension mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Suspendm::_0)
    }
    ///UTMI normal mode
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Suspendm::_1)
    }
}
impl R {
    ///Bit 14 - UTMI SuspendM Control
    #[inline(always)]
    pub fn suspendm(&self) -> SuspendmR {
        SuspendmR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPSTS").field("suspendm", &self.suspendm()).finish()
    }
}
impl W {
    ///Bit 14 - UTMI SuspendM Control
    #[inline(always)]
    pub fn suspendm(&mut self) -> SuspendmW<LpstsSpec> {
        SuspendmW::new(self, 14)
    }
}
/**Low Power Status Register

You can [`read`](crate::Reg::read) this register and get [`lpsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LpstsSpec;
impl crate::RegisterSpec for LpstsSpec {
    type Ux = u16;
}
///`read()` method returns [`lpsts::R`](R) reader structure
impl crate::Readable for LpstsSpec {}
///`write(|w| ..)` method takes [`lpsts::W`](W) writer structure
impl crate::Writable for LpstsSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LPSTS to value 0
impl crate::Resettable for LpstsSpec {}
