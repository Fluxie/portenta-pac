///Register `CCACTL` reader
pub type R = crate::R<CcactlSpec>;
///Register `CCACTL` writer
pub type W = crate::W<CcactlSpec>;
/**C-Cache Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enc {
    ///0: Disable C-cache
    _0 = 0,
    ///1: Enable C-cache
    _1 = 1,
}
impl From<Enc> for bool {
    #[inline(always)]
    fn from(variant: Enc) -> Self {
        variant as u8 != 0
    }
}
///Field `ENC` reader - C-Cache Enable
pub type EncR = crate::BitReader<Enc>;
impl EncR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Enc {
        match self.bits {
            false => Enc::_0,
            true => Enc::_1,
        }
    }
    ///Disable C-cache
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Enc::_0
    }
    ///Enable C-cache
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Enc::_1
    }
}
///Field `ENC` writer - C-Cache Enable
pub type EncW<'a, REG> = crate::BitWriter<'a, REG, Enc>;
impl<'a, REG> EncW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable C-cache
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Enc::_0)
    }
    ///Enable C-cache
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Enc::_1)
    }
}
impl R {
    ///Bit 0 - C-Cache Enable
    #[inline(always)]
    pub fn enc(&self) -> EncR {
        EncR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCACTL").field("enc", &self.enc()).finish()
    }
}
impl W {
    ///Bit 0 - C-Cache Enable
    #[inline(always)]
    pub fn enc(&mut self) -> EncW<CcactlSpec> {
        EncW::new(self, 0)
    }
}
/**C-Cache Control Register

You can [`read`](crate::Reg::read) this register and get [`ccactl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccactl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CcactlSpec;
impl crate::RegisterSpec for CcactlSpec {
    type Ux = u32;
}
///`read()` method returns [`ccactl::R`](R) reader structure
impl crate::Readable for CcactlSpec {}
///`write(|w| ..)` method takes [`ccactl::W`](W) writer structure
impl crate::Writable for CcactlSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCACTL to value 0
impl crate::Resettable for CcactlSpec {}
