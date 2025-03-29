///Register `DMACDTCERRSTAT` reader
pub type R = crate::R<DmacdtcerrstatSpec>;
/**Master TrustZone Filter Error Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mterrstat {
    ///0: No error occurred
    _0 = 0,
    ///1: Error occurred
    _1 = 1,
}
impl From<Mterrstat> for bool {
    #[inline(always)]
    fn from(variant: Mterrstat) -> Self {
        variant as u8 != 0
    }
}
///Field `MTERRSTAT` reader - Master TrustZone Filter Error Status
pub type MterrstatR = crate::BitReader<Mterrstat>;
impl MterrstatR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mterrstat {
        match self.bits {
            false => Mterrstat::_0,
            true => Mterrstat::_1,
        }
    }
    ///No error occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mterrstat::_0
    }
    ///Error occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mterrstat::_1
    }
}
impl R {
    ///Bit 0 - Master TrustZone Filter Error Status
    #[inline(always)]
    pub fn mterrstat(&self) -> MterrstatR {
        MterrstatR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACDTCERRSTAT").field("mterrstat", &self.mterrstat()).finish()
    }
}
/**DMAC/DTC Error Status Register

You can [`read`](crate::Reg::read) this register and get [`dmacdtcerrstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DmacdtcerrstatSpec;
impl crate::RegisterSpec for DmacdtcerrstatSpec {
    type Ux = u8;
}
///`read()` method returns [`dmacdtcerrstat::R`](R) reader structure
impl crate::Readable for DmacdtcerrstatSpec {}
///`reset()` method sets DMACDTCERRSTAT to value 0
impl crate::Resettable for DmacdtcerrstatSpec {}
