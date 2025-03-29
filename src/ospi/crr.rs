///Register `CRR` reader
pub type R = crate::R<CrrSpec>;
///Field `RD0` reader - Read data 0
pub type Rd0R = crate::FieldReader;
///Field `RD1` reader - Read data 1
pub type Rd1R = crate::FieldReader;
///Field `RD2` reader - Read data 2
pub type Rd2R = crate::FieldReader;
///Field `RD3` reader - Read data 3
pub type Rd3R = crate::FieldReader;
impl R {
    ///Bits 0:7 - Read data 0
    #[inline(always)]
    pub fn rd0(&self) -> Rd0R {
        Rd0R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Read data 1
    #[inline(always)]
    pub fn rd1(&self) -> Rd1R {
        Rd1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Read data 2
    #[inline(always)]
    pub fn rd2(&self) -> Rd2R {
        Rd2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Read data 3
    #[inline(always)]
    pub fn rd3(&self) -> Rd3R {
        Rd3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRR")
            .field("rd0", &self.rd0())
            .field("rd1", &self.rd1())
            .field("rd2", &self.rd2())
            .field("rd3", &self.rd3())
            .finish()
    }
}
/**Configure Read Register

You can [`read`](crate::Reg::read) this register and get [`crr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CrrSpec;
impl crate::RegisterSpec for CrrSpec {
    type Ux = u32;
}
///`read()` method returns [`crr::R`](R) reader structure
impl crate::Readable for CrrSpec {}
///`reset()` method sets CRR to value 0
impl crate::Resettable for CrrSpec {}
