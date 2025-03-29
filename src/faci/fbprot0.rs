///Register `FBPROT0` reader
pub type R = crate::R<Fbprot0Spec>;
///Register `FBPROT0` writer
pub type W = crate::W<Fbprot0Spec>;
/**Block Protection for Non-secure Cancel

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpcn0 {
    ///0: Block protection is enabled
    _0 = 0,
    ///1: Block protection is disabled.
    _1 = 1,
}
impl From<Bpcn0> for bool {
    #[inline(always)]
    fn from(variant: Bpcn0) -> Self {
        variant as u8 != 0
    }
}
///Field `BPCN0` reader - Block Protection for Non-secure Cancel
pub type Bpcn0R = crate::BitReader<Bpcn0>;
impl Bpcn0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Bpcn0 {
        match self.bits {
            false => Bpcn0::_0,
            true => Bpcn0::_1,
        }
    }
    ///Block protection is enabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bpcn0::_0
    }
    ///Block protection is disabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bpcn0::_1
    }
}
///Field `BPCN0` writer - Block Protection for Non-secure Cancel
pub type Bpcn0W<'a, REG> = crate::BitWriter<'a, REG, Bpcn0>;
impl<'a, REG> Bpcn0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Block protection is enabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpcn0::_0)
    }
    ///Block protection is disabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpcn0::_1)
    }
}
///Field `KEY` writer - Key Code
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bit 0 - Block Protection for Non-secure Cancel
    #[inline(always)]
    pub fn bpcn0(&self) -> Bpcn0R {
        Bpcn0R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FBPROT0").field("bpcn0", &self.bpcn0()).finish()
    }
}
impl W {
    ///Bit 0 - Block Protection for Non-secure Cancel
    #[inline(always)]
    pub fn bpcn0(&mut self) -> Bpcn0W<Fbprot0Spec> {
        Bpcn0W::new(self, 0)
    }
    ///Bits 8:15 - Key Code
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<Fbprot0Spec> {
        KeyW::new(self, 8)
    }
}
/**Flash Block Protection Register

You can [`read`](crate::Reg::read) this register and get [`fbprot0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fbprot0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Fbprot0Spec;
impl crate::RegisterSpec for Fbprot0Spec {
    type Ux = u16;
}
///`read()` method returns [`fbprot0::R`](R) reader structure
impl crate::Readable for Fbprot0Spec {}
///`write(|w| ..)` method takes [`fbprot0::W`](W) writer structure
impl crate::Writable for Fbprot0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FBPROT0 to value 0
impl crate::Resettable for Fbprot0Spec {}
