///Register `R64CNT` reader
pub type R = crate::R<R64cntSpec>;
///Field `F64HZ` reader - 64-Hz Flag
pub type F64hzR = crate::BitReader;
///Field `F32HZ` reader - 32-Hz Flag
pub type F32hzR = crate::BitReader;
///Field `F16HZ` reader - 16-Hz Flag
pub type F16hzR = crate::BitReader;
///Field `F8HZ` reader - 8-Hz Flag
pub type F8hzR = crate::BitReader;
///Field `F4HZ` reader - 4-Hz Flag
pub type F4hzR = crate::BitReader;
///Field `F2HZ` reader - 2-Hz Flag
pub type F2hzR = crate::BitReader;
///Field `F1HZ` reader - 1-Hz Flag
pub type F1hzR = crate::BitReader;
impl R {
    ///Bit 0 - 64-Hz Flag
    #[inline(always)]
    pub fn f64hz(&self) -> F64hzR {
        F64hzR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - 32-Hz Flag
    #[inline(always)]
    pub fn f32hz(&self) -> F32hzR {
        F32hzR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - 16-Hz Flag
    #[inline(always)]
    pub fn f16hz(&self) -> F16hzR {
        F16hzR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - 8-Hz Flag
    #[inline(always)]
    pub fn f8hz(&self) -> F8hzR {
        F8hzR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - 4-Hz Flag
    #[inline(always)]
    pub fn f4hz(&self) -> F4hzR {
        F4hzR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - 2-Hz Flag
    #[inline(always)]
    pub fn f2hz(&self) -> F2hzR {
        F2hzR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - 1-Hz Flag
    #[inline(always)]
    pub fn f1hz(&self) -> F1hzR {
        F1hzR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("R64CNT")
            .field("f64hz", &self.f64hz())
            .field("f32hz", &self.f32hz())
            .field("f16hz", &self.f16hz())
            .field("f8hz", &self.f8hz())
            .field("f4hz", &self.f4hz())
            .field("f2hz", &self.f2hz())
            .field("f1hz", &self.f1hz())
            .finish()
    }
}
/**64-Hz Counter

You can [`read`](crate::Reg::read) this register and get [`r64cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct R64cntSpec;
impl crate::RegisterSpec for R64cntSpec {
    type Ux = u8;
}
///`read()` method returns [`r64cnt::R`](R) reader structure
impl crate::Readable for R64cntSpec {}
///`reset()` method sets R64CNT to value 0
impl crate::Resettable for R64cntSpec {}
