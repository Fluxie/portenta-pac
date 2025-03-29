///Register `CFDC%sBLCT` reader
pub type R = crate::R<CfdcblctSpec>;
///Register `CFDC%sBLCT` writer
pub type W = crate::W<CfdcblctSpec>;
/**Bus Load Counter Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Blce {
    ///0: Bus load counter disable
    _0 = 0,
    ///1: Bus load counter enable
    _1 = 1,
}
impl From<Blce> for bool {
    #[inline(always)]
    fn from(variant: Blce) -> Self {
        variant as u8 != 0
    }
}
///Field `BLCE` reader - Bus Load Counter Enable
pub type BlceR = crate::BitReader<Blce>;
impl BlceR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Blce {
        match self.bits {
            false => Blce::_0,
            true => Blce::_1,
        }
    }
    ///Bus load counter disable
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Blce::_0
    }
    ///Bus load counter enable
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Blce::_1
    }
}
///Field `BLCE` writer - Bus Load Counter Enable
pub type BlceW<'a, REG> = crate::BitWriter<'a, REG, Blce>;
impl<'a, REG> BlceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Bus load counter disable
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Blce::_0)
    }
    ///Bus load counter enable
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Blce::_1)
    }
}
///Field `BLCLD` writer - BUS Load Counter Load
pub type BlcldW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Bus Load Counter Enable
    #[inline(always)]
    pub fn blce(&self) -> BlceR {
        BlceR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDCBLCT").field("blce", &self.blce()).finish()
    }
}
impl W {
    ///Bit 0 - Bus Load Counter Enable
    #[inline(always)]
    pub fn blce(&mut self) -> BlceW<CfdcblctSpec> {
        BlceW::new(self, 0)
    }
    ///Bit 8 - BUS Load Counter Load
    #[inline(always)]
    pub fn blcld(&mut self) -> BlcldW<CfdcblctSpec> {
        BlcldW::new(self, 8)
    }
}
/**Channel %s Bus Load Control Register

You can [`read`](crate::Reg::read) this register and get [`cfdcblct::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcblct::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdcblctSpec;
impl crate::RegisterSpec for CfdcblctSpec {
    type Ux = u32;
}
///`read()` method returns [`cfdcblct::R`](R) reader structure
impl crate::Readable for CfdcblctSpec {}
///`write(|w| ..)` method takes [`cfdcblct::W`](W) writer structure
impl crate::Writable for CfdcblctSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDC%sBLCT to value 0
impl crate::Resettable for CfdcblctSpec {}
