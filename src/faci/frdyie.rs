///Register `FRDYIE` reader
pub type R = crate::R<FrdyieSpec>;
///Register `FRDYIE` writer
pub type W = crate::W<FrdyieSpec>;
/**Flash Ready Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Frdyie {
    ///0: Generation of an FRDY interrupt request is disabled
    _0 = 0,
    ///1: Generation of an FRDY interrupt request is enabled.
    _1 = 1,
}
impl From<Frdyie> for bool {
    #[inline(always)]
    fn from(variant: Frdyie) -> Self {
        variant as u8 != 0
    }
}
///Field `FRDYIE` reader - Flash Ready Interrupt Enable
pub type FrdyieR = crate::BitReader<Frdyie>;
impl FrdyieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Frdyie {
        match self.bits {
            false => Frdyie::_0,
            true => Frdyie::_1,
        }
    }
    ///Generation of an FRDY interrupt request is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Frdyie::_0
    }
    ///Generation of an FRDY interrupt request is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Frdyie::_1
    }
}
///Field `FRDYIE` writer - Flash Ready Interrupt Enable
pub type FrdyieW<'a, REG> = crate::BitWriter<'a, REG, Frdyie>;
impl<'a, REG> FrdyieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Generation of an FRDY interrupt request is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Frdyie::_0)
    }
    ///Generation of an FRDY interrupt request is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Frdyie::_1)
    }
}
impl R {
    ///Bit 0 - Flash Ready Interrupt Enable
    #[inline(always)]
    pub fn frdyie(&self) -> FrdyieR {
        FrdyieR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FRDYIE").field("frdyie", &self.frdyie()).finish()
    }
}
impl W {
    ///Bit 0 - Flash Ready Interrupt Enable
    #[inline(always)]
    pub fn frdyie(&mut self) -> FrdyieW<FrdyieSpec> {
        FrdyieW::new(self, 0)
    }
}
/**Flash Ready Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`frdyie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frdyie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FrdyieSpec;
impl crate::RegisterSpec for FrdyieSpec {
    type Ux = u8;
}
///`read()` method returns [`frdyie::R`](R) reader structure
impl crate::Readable for FrdyieSpec {}
///`write(|w| ..)` method takes [`frdyie::W`](W) writer structure
impl crate::Writable for FrdyieSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FRDYIE to value 0
impl crate::Resettable for FrdyieSpec {}
