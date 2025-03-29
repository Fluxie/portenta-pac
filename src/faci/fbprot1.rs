///Register `FBPROT1` reader
pub type R = crate::R<Fbprot1Spec>;
///Register `FBPROT1` writer
pub type W = crate::W<Fbprot1Spec>;
/**Block Protection for Secure Cancel

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpcn1 {
    ///0: Block protection is enabled
    _0 = 0,
    ///1: Block protection is disabled.
    _1 = 1,
}
impl From<Bpcn1> for bool {
    #[inline(always)]
    fn from(variant: Bpcn1) -> Self {
        variant as u8 != 0
    }
}
///Field `BPCN1` reader - Block Protection for Secure Cancel
pub type Bpcn1R = crate::BitReader<Bpcn1>;
impl Bpcn1R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Bpcn1 {
        match self.bits {
            false => Bpcn1::_0,
            true => Bpcn1::_1,
        }
    }
    ///Block protection is enabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bpcn1::_0
    }
    ///Block protection is disabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bpcn1::_1
    }
}
///Field `BPCN1` writer - Block Protection for Secure Cancel
pub type Bpcn1W<'a, REG> = crate::BitWriter<'a, REG, Bpcn1>;
impl<'a, REG> Bpcn1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Block protection is enabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpcn1::_0)
    }
    ///Block protection is disabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpcn1::_1)
    }
}
///Field `KEY` writer - Key Code
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bit 0 - Block Protection for Secure Cancel
    #[inline(always)]
    pub fn bpcn1(&self) -> Bpcn1R {
        Bpcn1R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FBPROT1").field("bpcn1", &self.bpcn1()).finish()
    }
}
impl W {
    ///Bit 0 - Block Protection for Secure Cancel
    #[inline(always)]
    pub fn bpcn1(&mut self) -> Bpcn1W<Fbprot1Spec> {
        Bpcn1W::new(self, 0)
    }
    ///Bits 8:15 - Key Code
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<Fbprot1Spec> {
        KeyW::new(self, 8)
    }
}
/**Flash Block Protection for Secure Register

You can [`read`](crate::Reg::read) this register and get [`fbprot1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fbprot1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Fbprot1Spec;
impl crate::RegisterSpec for Fbprot1Spec {
    type Ux = u16;
}
///`read()` method returns [`fbprot1::R`](R) reader structure
impl crate::Readable for Fbprot1Spec {}
///`write(|w| ..)` method takes [`fbprot1::W`](W) writer structure
impl crate::Writable for Fbprot1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FBPROT1 to value 0
impl crate::Resettable for Fbprot1Spec {}
