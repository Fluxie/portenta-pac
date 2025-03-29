///Register `RSTSR2` reader
pub type R = crate::R<Rstsr2Spec>;
///Register `RSTSR2` writer
pub type W = crate::W<Rstsr2Spec>;
/**Cold/Warm Start Determination Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cwsf {
    ///0: Cold start
    _0 = 0,
    ///1: Warm start
    _1 = 1,
}
impl From<Cwsf> for bool {
    #[inline(always)]
    fn from(variant: Cwsf) -> Self {
        variant as u8 != 0
    }
}
///Field `CWSF` reader - Cold/Warm Start Determination Flag
pub type CwsfR = crate::BitReader<Cwsf>;
impl CwsfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cwsf {
        match self.bits {
            false => Cwsf::_0,
            true => Cwsf::_1,
        }
    }
    ///Cold start
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cwsf::_0
    }
    ///Warm start
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cwsf::_1
    }
}
///Field `CWSF` writer - Cold/Warm Start Determination Flag
pub type CwsfW<'a, REG> = crate::BitWriter<'a, REG, Cwsf>;
impl<'a, REG> CwsfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cold start
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cwsf::_0)
    }
    ///Warm start
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cwsf::_1)
    }
}
impl R {
    ///Bit 0 - Cold/Warm Start Determination Flag
    #[inline(always)]
    pub fn cwsf(&self) -> CwsfR {
        CwsfR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSTSR2").field("cwsf", &self.cwsf()).finish()
    }
}
impl W {
    ///Bit 0 - Cold/Warm Start Determination Flag
    #[inline(always)]
    pub fn cwsf(&mut self) -> CwsfW<Rstsr2Spec> {
        CwsfW::new(self, 0)
    }
}
/**Reset Status Register 2

You can [`read`](crate::Reg::read) this register and get [`rstsr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstsr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Rstsr2Spec;
impl crate::RegisterSpec for Rstsr2Spec {
    type Ux = u8;
}
///`read()` method returns [`rstsr2::R`](R) reader structure
impl crate::Readable for Rstsr2Spec {}
///`write(|w| ..)` method takes [`rstsr2::W`](W) writer structure
impl crate::Writable for Rstsr2Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RSTSR2 to value 0
impl crate::Resettable for Rstsr2Spec {}
