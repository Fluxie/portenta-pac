///Register `FBCSTAT` reader
pub type R = crate::R<FbcstatSpec>;
///Register `FBCSTAT` writer
pub type W = crate::W<FbcstatSpec>;
/**Blank Check Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bcst {
    ///0: The target area is in the non-programmed state, that is, the area has been erased but has not yet been reprogrammed
    _0 = 0,
    ///1: The target area has been programmed with 0s or 1s.
    _1 = 1,
}
impl From<Bcst> for bool {
    #[inline(always)]
    fn from(variant: Bcst) -> Self {
        variant as u8 != 0
    }
}
///Field `BCST` reader - Blank Check Status Flag
pub type BcstR = crate::BitReader<Bcst>;
impl BcstR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Bcst {
        match self.bits {
            false => Bcst::_0,
            true => Bcst::_1,
        }
    }
    ///The target area is in the non-programmed state, that is, the area has been erased but has not yet been reprogrammed
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bcst::_0
    }
    ///The target area has been programmed with 0s or 1s.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bcst::_1
    }
}
impl R {
    ///Bit 0 - Blank Check Status Flag
    #[inline(always)]
    pub fn bcst(&self) -> BcstR {
        BcstR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FBCSTAT").field("bcst", &self.bcst()).finish()
    }
}
impl W {}
/**Blank Check Status Register

You can [`read`](crate::Reg::read) this register and get [`fbcstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fbcstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FbcstatSpec;
impl crate::RegisterSpec for FbcstatSpec {
    type Ux = u8;
}
///`read()` method returns [`fbcstat::R`](R) reader structure
impl crate::Readable for FbcstatSpec {}
///`write(|w| ..)` method takes [`fbcstat::W`](W) writer structure
impl crate::Writable for FbcstatSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FBCSTAT to value 0
impl crate::Resettable for FbcstatSpec {}
