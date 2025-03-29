///Register `DTCSTS` reader
pub type R = crate::R<DtcstsSpec>;
///Field `VECN` reader - DTC-Activating Vector Number Monitoring
pub type VecnR = crate::FieldReader;
/**DTC Active Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Act {
    ///0: DTC transfer operation is not in progress.
    _0 = 0,
    ///1: DTC transfer operation is in progress.
    _1 = 1,
}
impl From<Act> for bool {
    #[inline(always)]
    fn from(variant: Act) -> Self {
        variant as u8 != 0
    }
}
///Field `ACT` reader - DTC Active Flag
pub type ActR = crate::BitReader<Act>;
impl ActR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Act {
        match self.bits {
            false => Act::_0,
            true => Act::_1,
        }
    }
    ///DTC transfer operation is not in progress.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Act::_0
    }
    ///DTC transfer operation is in progress.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Act::_1
    }
}
impl R {
    ///Bits 0:7 - DTC-Activating Vector Number Monitoring
    #[inline(always)]
    pub fn vecn(&self) -> VecnR {
        VecnR::new((self.bits & 0xff) as u8)
    }
    ///Bit 15 - DTC Active Flag
    #[inline(always)]
    pub fn act(&self) -> ActR {
        ActR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DTCSTS")
            .field("vecn", &self.vecn())
            .field("act", &self.act())
            .finish()
    }
}
/**DTC Status Register

You can [`read`](crate::Reg::read) this register and get [`dtcsts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DtcstsSpec;
impl crate::RegisterSpec for DtcstsSpec {
    type Ux = u16;
}
///`read()` method returns [`dtcsts::R`](R) reader structure
impl crate::Readable for DtcstsSpec {}
///`reset()` method sets DTCSTS to value 0
impl crate::Resettable for DtcstsSpec {}
