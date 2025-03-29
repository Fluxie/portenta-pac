///Register `VBATTMONR` reader
pub type R = crate::R<VbattmonrSpec>;
/**VBATT Voltage Monitor Bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vbattmon {
    ///0: VBATT ≥ Vbattldet
    _0 = 0,
    ///1: VBATT < Vbattldet
    _1 = 1,
}
impl From<Vbattmon> for bool {
    #[inline(always)]
    fn from(variant: Vbattmon) -> Self {
        variant as u8 != 0
    }
}
///Field `VBATTMON` reader - VBATT Voltage Monitor Bit
pub type VbattmonR = crate::BitReader<Vbattmon>;
impl VbattmonR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Vbattmon {
        match self.bits {
            false => Vbattmon::_0,
            true => Vbattmon::_1,
        }
    }
    ///VBATT ≥ Vbattldet
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Vbattmon::_0
    }
    ///VBATT < Vbattldet
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Vbattmon::_1
    }
}
impl R {
    ///Bit 0 - VBATT Voltage Monitor Bit
    #[inline(always)]
    pub fn vbattmon(&self) -> VbattmonR {
        VbattmonR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VBATTMONR").field("vbattmon", &self.vbattmon()).finish()
    }
}
/**Battery Backup Voltage Monitor Register

You can [`read`](crate::Reg::read) this register and get [`vbattmonr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct VbattmonrSpec;
impl crate::RegisterSpec for VbattmonrSpec {
    type Ux = u8;
}
///`read()` method returns [`vbattmonr::R`](R) reader structure
impl crate::Readable for VbattmonrSpec {}
///`reset()` method sets VBATTMONR to value 0
impl crate::Resettable for VbattmonrSpec {}
