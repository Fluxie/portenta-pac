///Register `SCACTL` reader
pub type R = crate::R<ScactlSpec>;
///Register `SCACTL` writer
pub type W = crate::W<ScactlSpec>;
/**S-Cache Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ens {
    ///0: Disable S-cache
    _0 = 0,
    ///1: Enable S-cache
    _1 = 1,
}
impl From<Ens> for bool {
    #[inline(always)]
    fn from(variant: Ens) -> Self {
        variant as u8 != 0
    }
}
///Field `ENS` reader - S-Cache Enable
pub type EnsR = crate::BitReader<Ens>;
impl EnsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ens {
        match self.bits {
            false => Ens::_0,
            true => Ens::_1,
        }
    }
    ///Disable S-cache
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ens::_0
    }
    ///Enable S-cache
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ens::_1
    }
}
///Field `ENS` writer - S-Cache Enable
pub type EnsW<'a, REG> = crate::BitWriter<'a, REG, Ens>;
impl<'a, REG> EnsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable S-cache
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ens::_0)
    }
    ///Enable S-cache
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ens::_1)
    }
}
impl R {
    ///Bit 0 - S-Cache Enable
    #[inline(always)]
    pub fn ens(&self) -> EnsR {
        EnsR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCACTL").field("ens", &self.ens()).finish()
    }
}
impl W {
    ///Bit 0 - S-Cache Enable
    #[inline(always)]
    pub fn ens(&mut self) -> EnsW<ScactlSpec> {
        EnsW::new(self, 0)
    }
}
/**S-Cache Control Register

You can [`read`](crate::Reg::read) this register and get [`scactl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scactl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ScactlSpec;
impl crate::RegisterSpec for ScactlSpec {
    type Ux = u32;
}
///`read()` method returns [`scactl::R`](R) reader structure
impl crate::Readable for ScactlSpec {}
///`write(|w| ..)` method takes [`scactl::W`](W) writer structure
impl crate::Writable for ScactlSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SCACTL to value 0
impl crate::Resettable for ScactlSpec {}
