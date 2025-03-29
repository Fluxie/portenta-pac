///Register `FBCCNT` reader
pub type R = crate::R<FbccntSpec>;
///Register `FBCCNT` writer
pub type W = crate::W<FbccntSpec>;
/**Blank Check Direction

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bcdir {
    ///0: Blank checking is executed from the lower addresses to the higher addresses (incremental mode)
    _0 = 0,
    ///1: Blank checking is executed from the higher addresses to the lower addresses (decremental mode).
    _1 = 1,
}
impl From<Bcdir> for bool {
    #[inline(always)]
    fn from(variant: Bcdir) -> Self {
        variant as u8 != 0
    }
}
///Field `BCDIR` reader - Blank Check Direction
pub type BcdirR = crate::BitReader<Bcdir>;
impl BcdirR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Bcdir {
        match self.bits {
            false => Bcdir::_0,
            true => Bcdir::_1,
        }
    }
    ///Blank checking is executed from the lower addresses to the higher addresses (incremental mode)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bcdir::_0
    }
    ///Blank checking is executed from the higher addresses to the lower addresses (decremental mode).
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bcdir::_1
    }
}
///Field `BCDIR` writer - Blank Check Direction
pub type BcdirW<'a, REG> = crate::BitWriter<'a, REG, Bcdir>;
impl<'a, REG> BcdirW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Blank checking is executed from the lower addresses to the higher addresses (incremental mode)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bcdir::_0)
    }
    ///Blank checking is executed from the higher addresses to the lower addresses (decremental mode).
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bcdir::_1)
    }
}
impl R {
    ///Bit 0 - Blank Check Direction
    #[inline(always)]
    pub fn bcdir(&self) -> BcdirR {
        BcdirR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FBCCNT").field("bcdir", &self.bcdir()).finish()
    }
}
impl W {
    ///Bit 0 - Blank Check Direction
    #[inline(always)]
    pub fn bcdir(&mut self) -> BcdirW<FbccntSpec> {
        BcdirW::new(self, 0)
    }
}
/**Blank Check Control Register

You can [`read`](crate::Reg::read) this register and get [`fbccnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fbccnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FbccntSpec;
impl crate::RegisterSpec for FbccntSpec {
    type Ux = u8;
}
///`read()` method returns [`fbccnt::R`](R) reader structure
impl crate::Readable for FbccntSpec {}
///`write(|w| ..)` method takes [`fbccnt::W`](W) writer structure
impl crate::Writable for FbccntSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FBCCNT to value 0
impl crate::Resettable for FbccntSpec {}
