///Register `FCACHEIV` reader
pub type R = crate::R<FcacheivSpec>;
///Register `FCACHEIV` writer
pub type W = crate::W<FcacheivSpec>;
/**Flash Cache Invalidate

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fcacheiv {
    ///0: Read: Do not invalidate. Write: The setting is ignored.
    _0 = 0,
    ///1: Invalidate FCACHE is invalidated.
    _1 = 1,
}
impl From<Fcacheiv> for bool {
    #[inline(always)]
    fn from(variant: Fcacheiv) -> Self {
        variant as u8 != 0
    }
}
///Field `FCACHEIV` reader - Flash Cache Invalidate
pub type FcacheivR = crate::BitReader<Fcacheiv>;
impl FcacheivR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Fcacheiv {
        match self.bits {
            false => Fcacheiv::_0,
            true => Fcacheiv::_1,
        }
    }
    ///Read: Do not invalidate. Write: The setting is ignored.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Fcacheiv::_0
    }
    ///Invalidate FCACHE is invalidated.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Fcacheiv::_1
    }
}
///Field `FCACHEIV` writer - Flash Cache Invalidate
pub type FcacheivW<'a, REG> = crate::BitWriter<'a, REG, Fcacheiv>;
impl<'a, REG> FcacheivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Read: Do not invalidate. Write: The setting is ignored.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Fcacheiv::_0)
    }
    ///Invalidate FCACHE is invalidated.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Fcacheiv::_1)
    }
}
impl R {
    ///Bit 0 - Flash Cache Invalidate
    #[inline(always)]
    pub fn fcacheiv(&self) -> FcacheivR {
        FcacheivR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCACHEIV").field("fcacheiv", &self.fcacheiv()).finish()
    }
}
impl W {
    ///Bit 0 - Flash Cache Invalidate
    #[inline(always)]
    pub fn fcacheiv(&mut self) -> FcacheivW<FcacheivSpec> {
        FcacheivW::new(self, 0)
    }
}
/**Flash Cache Invalidate Register

You can [`read`](crate::Reg::read) this register and get [`fcacheiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcacheiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FcacheivSpec;
impl crate::RegisterSpec for FcacheivSpec {
    type Ux = u16;
}
///`read()` method returns [`fcacheiv::R`](R) reader structure
impl crate::Readable for FcacheivSpec {}
///`write(|w| ..)` method takes [`fcacheiv::W`](W) writer structure
impl crate::Writable for FcacheivSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FCACHEIV to value 0
impl crate::Resettable for FcacheivSpec {}
