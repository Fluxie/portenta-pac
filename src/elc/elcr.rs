///Register `ELCR` reader
pub type R = crate::R<ElcrSpec>;
///Register `ELCR` writer
pub type W = crate::W<ElcrSpec>;
/**All Event Link Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Elcon {
    ///0: ELC function is disabled.
    _0 = 0,
    ///1: ELC function is enabled.
    _1 = 1,
}
impl From<Elcon> for bool {
    #[inline(always)]
    fn from(variant: Elcon) -> Self {
        variant as u8 != 0
    }
}
///Field `ELCON` reader - All Event Link Enable
pub type ElconR = crate::BitReader<Elcon>;
impl ElconR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Elcon {
        match self.bits {
            false => Elcon::_0,
            true => Elcon::_1,
        }
    }
    ///ELC function is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Elcon::_0
    }
    ///ELC function is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Elcon::_1
    }
}
///Field `ELCON` writer - All Event Link Enable
pub type ElconW<'a, REG> = crate::BitWriter<'a, REG, Elcon>;
impl<'a, REG> ElconW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ELC function is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Elcon::_0)
    }
    ///ELC function is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Elcon::_1)
    }
}
impl R {
    ///Bit 7 - All Event Link Enable
    #[inline(always)]
    pub fn elcon(&self) -> ElconR {
        ElconR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ELCR").field("elcon", &self.elcon()).finish()
    }
}
impl W {
    ///Bit 7 - All Event Link Enable
    #[inline(always)]
    pub fn elcon(&mut self) -> ElconW<ElcrSpec> {
        ElconW::new(self, 7)
    }
}
/**Event Link Controller Register

You can [`read`](crate::Reg::read) this register and get [`elcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`elcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ElcrSpec;
impl crate::RegisterSpec for ElcrSpec {
    type Ux = u8;
}
///`read()` method returns [`elcr::R`](R) reader structure
impl crate::Readable for ElcrSpec {}
///`write(|w| ..)` method takes [`elcr::W`](W) writer structure
impl crate::Writable for ElcrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ELCR to value 0
impl crate::Resettable for ElcrSpec {}
