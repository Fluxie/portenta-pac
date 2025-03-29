///Register `ESMER` reader
pub type R = crate::R<EsmerSpec>;
///Register `ESMER` writer
pub type W = crate::W<EsmerSpec>;
/**Extended Serial Mode Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Esme {
    ///0: The extended serial mode is disabled.
    _0 = 0,
    ///1: The extended serial mode is enabled.
    _1 = 1,
}
impl From<Esme> for bool {
    #[inline(always)]
    fn from(variant: Esme) -> Self {
        variant as u8 != 0
    }
}
///Field `ESME` reader - Extended Serial Mode Enable
pub type EsmeR = crate::BitReader<Esme>;
impl EsmeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Esme {
        match self.bits {
            false => Esme::_0,
            true => Esme::_1,
        }
    }
    ///The extended serial mode is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Esme::_0
    }
    ///The extended serial mode is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Esme::_1
    }
}
///Field `ESME` writer - Extended Serial Mode Enable
pub type EsmeW<'a, REG> = crate::BitWriter<'a, REG, Esme>;
impl<'a, REG> EsmeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The extended serial mode is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Esme::_0)
    }
    ///The extended serial mode is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Esme::_1)
    }
}
impl R {
    ///Bit 0 - Extended Serial Mode Enable
    #[inline(always)]
    pub fn esme(&self) -> EsmeR {
        EsmeR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ESMER").field("esme", &self.esme()).finish()
    }
}
impl W {
    ///Bit 0 - Extended Serial Mode Enable
    #[inline(always)]
    pub fn esme(&mut self) -> EsmeW<EsmerSpec> {
        EsmeW::new(self, 0)
    }
}
/**Extended Serial Module Enable Register

You can [`read`](crate::Reg::read) this register and get [`esmer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esmer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct EsmerSpec;
impl crate::RegisterSpec for EsmerSpec {
    type Ux = u8;
}
///`read()` method returns [`esmer::R`](R) reader structure
impl crate::Readable for EsmerSpec {}
///`write(|w| ..)` method takes [`esmer::W`](W) writer structure
impl crate::Writable for EsmerSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ESMER to value 0
impl crate::Resettable for EsmerSpec {}
