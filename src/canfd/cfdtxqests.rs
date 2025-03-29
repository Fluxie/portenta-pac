///Register `CFDTXQESTS` reader
pub type R = crate::R<CfdtxqestsSpec>;
/**TXQ Empty Status

Value on reset: 255*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TxqxEmp {
    ///0: TXQ not empty
    _0 = 0,
    ///1: TXQ empty
    _1 = 1,
}
impl From<TxqxEmp> for u8 {
    #[inline(always)]
    fn from(variant: TxqxEmp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TxqxEmp {
    type Ux = u8;
}
impl crate::IsEnum for TxqxEmp {}
///Field `TXQxEMP` reader - TXQ Empty Status
pub type TxqxEmpR = crate::FieldReader<TxqxEmp>;
impl TxqxEmpR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<TxqxEmp> {
        match self.bits {
            0 => Some(TxqxEmp::_0),
            1 => Some(TxqxEmp::_1),
            _ => None,
        }
    }
    ///TXQ not empty
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TxqxEmp::_0
    }
    ///TXQ empty
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TxqxEmp::_1
    }
}
impl R {
    ///Bits 0:7 - TXQ Empty Status
    #[inline(always)]
    pub fn txqx_emp(&self) -> TxqxEmpR {
        TxqxEmpR::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDTXQESTS").field("txqx_emp", &self.txqx_emp()).finish()
    }
}
/**TX Queue Empty Status Register

You can [`read`](crate::Reg::read) this register and get [`cfdtxqests::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdtxqestsSpec;
impl crate::RegisterSpec for CfdtxqestsSpec {
    type Ux = u32;
}
///`read()` method returns [`cfdtxqests::R`](R) reader structure
impl crate::Readable for CfdtxqestsSpec {}
///`reset()` method sets CFDTXQESTS to value 0xff
impl crate::Resettable for CfdtxqestsSpec {
    const RESET_VALUE: u32 = 0xff;
}
