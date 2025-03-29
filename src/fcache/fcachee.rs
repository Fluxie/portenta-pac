///Register `FCACHEE` reader
pub type R = crate::R<FcacheeSpec>;
///Register `FCACHEE` writer
pub type W = crate::W<FcacheeSpec>;
/**Flash Cache Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fcacheen {
    ///0: FCACHE is disabled
    _0 = 0,
    ///1: FCACHE is enabled
    _1 = 1,
}
impl From<Fcacheen> for bool {
    #[inline(always)]
    fn from(variant: Fcacheen) -> Self {
        variant as u8 != 0
    }
}
///Field `FCACHEEN` reader - Flash Cache Enable
pub type FcacheenR = crate::BitReader<Fcacheen>;
impl FcacheenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Fcacheen {
        match self.bits {
            false => Fcacheen::_0,
            true => Fcacheen::_1,
        }
    }
    ///FCACHE is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Fcacheen::_0
    }
    ///FCACHE is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Fcacheen::_1
    }
}
///Field `FCACHEEN` writer - Flash Cache Enable
pub type FcacheenW<'a, REG> = crate::BitWriter<'a, REG, Fcacheen>;
impl<'a, REG> FcacheenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///FCACHE is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Fcacheen::_0)
    }
    ///FCACHE is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Fcacheen::_1)
    }
}
impl R {
    ///Bit 0 - Flash Cache Enable
    #[inline(always)]
    pub fn fcacheen(&self) -> FcacheenR {
        FcacheenR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCACHEE").field("fcacheen", &self.fcacheen()).finish()
    }
}
impl W {
    ///Bit 0 - Flash Cache Enable
    #[inline(always)]
    pub fn fcacheen(&mut self) -> FcacheenW<FcacheeSpec> {
        FcacheenW::new(self, 0)
    }
}
/**Flash Cache Enable Register

You can [`read`](crate::Reg::read) this register and get [`fcachee::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcachee::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FcacheeSpec;
impl crate::RegisterSpec for FcacheeSpec {
    type Ux = u16;
}
///`read()` method returns [`fcachee::R`](R) reader structure
impl crate::Readable for FcacheeSpec {}
///`write(|w| ..)` method takes [`fcachee::W`](W) writer structure
impl crate::Writable for FcacheeSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FCACHEE to value 0
impl crate::Resettable for FcacheeSpec {}
