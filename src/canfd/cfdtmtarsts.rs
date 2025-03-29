///Register `CFDTMTARSTS%s` reader
pub type R = crate::R<CfdtmtarstsSpec>;
/**TX Message Buffer Transmission Abort Request Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cfdtmtarsts {
    ///0: Transmission abort not requested for corresponding TX message buffer
    _0 = 0,
    ///1: Transmission abort requested for corresponding TX message buffer
    _1 = 1,
}
impl From<Cfdtmtarsts> for u8 {
    #[inline(always)]
    fn from(variant: Cfdtmtarsts) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cfdtmtarsts {
    type Ux = u8;
}
impl crate::IsEnum for Cfdtmtarsts {}
///Field `CFDTMTARSTS` reader - TX Message Buffer Transmission Abort Request Status
pub type CfdtmtarstsR = crate::FieldReader<Cfdtmtarsts>;
impl CfdtmtarstsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cfdtmtarsts> {
        match self.bits {
            0 => Some(Cfdtmtarsts::_0),
            1 => Some(Cfdtmtarsts::_1),
            _ => None,
        }
    }
    ///Transmission abort not requested for corresponding TX message buffer
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cfdtmtarsts::_0
    }
    ///Transmission abort requested for corresponding TX message buffer
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cfdtmtarsts::_1
    }
}
impl R {
    ///Bits 0:7 - TX Message Buffer Transmission Abort Request Status
    #[inline(always)]
    pub fn cfdtmtarsts(&self) -> CfdtmtarstsR {
        CfdtmtarstsR::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDTMTARSTS").field("cfdtmtarsts", &self.cfdtmtarsts()).finish()
    }
}
/**TX Message Buffer Transmission Abort Request Status Register %s

You can [`read`](crate::Reg::read) this register and get [`cfdtmtarsts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdtmtarstsSpec;
impl crate::RegisterSpec for CfdtmtarstsSpec {
    type Ux = u32;
}
///`read()` method returns [`cfdtmtarsts::R`](R) reader structure
impl crate::Readable for CfdtmtarstsSpec {}
///`reset()` method sets CFDTMTARSTS%s to value 0
impl crate::Resettable for CfdtmtarstsSpec {}
