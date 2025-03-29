///Register `FCPSR` reader
pub type R = crate::R<FcpsrSpec>;
///Register `FCPSR` writer
pub type W = crate::W<FcpsrSpec>;
/**Erasure Suspend Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Esuspmd {
    ///0: Suspension priority mode
    _0 = 0,
    ///1: Erasure priority mode.
    _1 = 1,
}
impl From<Esuspmd> for bool {
    #[inline(always)]
    fn from(variant: Esuspmd) -> Self {
        variant as u8 != 0
    }
}
///Field `ESUSPMD` reader - Erasure Suspend Mode
pub type EsuspmdR = crate::BitReader<Esuspmd>;
impl EsuspmdR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Esuspmd {
        match self.bits {
            false => Esuspmd::_0,
            true => Esuspmd::_1,
        }
    }
    ///Suspension priority mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Esuspmd::_0
    }
    ///Erasure priority mode.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Esuspmd::_1
    }
}
///Field `ESUSPMD` writer - Erasure Suspend Mode
pub type EsuspmdW<'a, REG> = crate::BitWriter<'a, REG, Esuspmd>;
impl<'a, REG> EsuspmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Suspension priority mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Esuspmd::_0)
    }
    ///Erasure priority mode.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Esuspmd::_1)
    }
}
impl R {
    ///Bit 0 - Erasure Suspend Mode
    #[inline(always)]
    pub fn esuspmd(&self) -> EsuspmdR {
        EsuspmdR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCPSR").field("esuspmd", &self.esuspmd()).finish()
    }
}
impl W {
    ///Bit 0 - Erasure Suspend Mode
    #[inline(always)]
    pub fn esuspmd(&mut self) -> EsuspmdW<FcpsrSpec> {
        EsuspmdW::new(self, 0)
    }
}
/**Flash Sequencer Processing Switching Register

You can [`read`](crate::Reg::read) this register and get [`fcpsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcpsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FcpsrSpec;
impl crate::RegisterSpec for FcpsrSpec {
    type Ux = u16;
}
///`read()` method returns [`fcpsr::R`](R) reader structure
impl crate::Readable for FcpsrSpec {}
///`write(|w| ..)` method takes [`fcpsr::W`](W) writer structure
impl crate::Writable for FcpsrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FCPSR to value 0
impl crate::Resettable for FcpsrSpec {}
