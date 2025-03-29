///Register `STR` reader
pub type R = crate::R<StrSpec>;
///Field `BFDF` reader - Break Field Low Width Detection Flag
pub type BfdfR = crate::BitReader;
///Field `CF0MF` reader - Control Field 0 Match Flag
pub type Cf0mfR = crate::BitReader;
///Field `CF1MF` reader - Control Field 1 Match Flag
pub type Cf1mfR = crate::BitReader;
///Field `PIBDF` reader - Priority Interrupt Bit Detection Flag
pub type PibdfR = crate::BitReader;
///Field `BCDF` reader - Bus Collision Detected Flag
pub type BcdfR = crate::BitReader;
///Field `AEDF` reader - Valid Edge Detection Flag
pub type AedfR = crate::BitReader;
impl R {
    ///Bit 0 - Break Field Low Width Detection Flag
    #[inline(always)]
    pub fn bfdf(&self) -> BfdfR {
        BfdfR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Control Field 0 Match Flag
    #[inline(always)]
    pub fn cf0mf(&self) -> Cf0mfR {
        Cf0mfR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Control Field 1 Match Flag
    #[inline(always)]
    pub fn cf1mf(&self) -> Cf1mfR {
        Cf1mfR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Priority Interrupt Bit Detection Flag
    #[inline(always)]
    pub fn pibdf(&self) -> PibdfR {
        PibdfR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Bus Collision Detected Flag
    #[inline(always)]
    pub fn bcdf(&self) -> BcdfR {
        BcdfR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Valid Edge Detection Flag
    #[inline(always)]
    pub fn aedf(&self) -> AedfR {
        AedfR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STR")
            .field("bfdf", &self.bfdf())
            .field("cf0mf", &self.cf0mf())
            .field("cf1mf", &self.cf1mf())
            .field("pibdf", &self.pibdf())
            .field("bcdf", &self.bcdf())
            .field("aedf", &self.aedf())
            .finish()
    }
}
/**Status Register

You can [`read`](crate::Reg::read) this register and get [`str::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct StrSpec;
impl crate::RegisterSpec for StrSpec {
    type Ux = u8;
}
///`read()` method returns [`str::R`](R) reader structure
impl crate::Readable for StrSpec {}
///`reset()` method sets STR to value 0
impl crate::Resettable for StrSpec {}
