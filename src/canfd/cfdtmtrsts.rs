///Register `CFDTMTRSTS%s` reader
pub type R = crate::R<CfdtmtrstsSpec>;
/**TX Message Buffer Transmission Request Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cfdtmtrsts {
    ///0: Transmission not requested for corresponding TX message buffer
    _0 = 0,
    ///1: Transmission requested for corresponding TX message buffer
    _1 = 1,
}
impl From<Cfdtmtrsts> for u8 {
    #[inline(always)]
    fn from(variant: Cfdtmtrsts) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cfdtmtrsts {
    type Ux = u8;
}
impl crate::IsEnum for Cfdtmtrsts {}
///Field `CFDTMTRSTS` reader - TX Message Buffer Transmission Request Status
pub type CfdtmtrstsR = crate::FieldReader<Cfdtmtrsts>;
impl CfdtmtrstsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cfdtmtrsts> {
        match self.bits {
            0 => Some(Cfdtmtrsts::_0),
            1 => Some(Cfdtmtrsts::_1),
            _ => None,
        }
    }
    ///Transmission not requested for corresponding TX message buffer
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cfdtmtrsts::_0
    }
    ///Transmission requested for corresponding TX message buffer
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cfdtmtrsts::_1
    }
}
impl R {
    ///Bits 0:7 - TX Message Buffer Transmission Request Status
    #[inline(always)]
    pub fn cfdtmtrsts(&self) -> CfdtmtrstsR {
        CfdtmtrstsR::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDTMTRSTS").field("cfdtmtrsts", &self.cfdtmtrsts()).finish()
    }
}
/**TX Message Buffer Transmission Request Status Register %s

You can [`read`](crate::Reg::read) this register and get [`cfdtmtrsts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdtmtrstsSpec;
impl crate::RegisterSpec for CfdtmtrstsSpec {
    type Ux = u32;
}
///`read()` method returns [`cfdtmtrsts::R`](R) reader structure
impl crate::Readable for CfdtmtrstsSpec {}
///`reset()` method sets CFDTMTRSTS%s to value 0
impl crate::Resettable for CfdtmtrstsSpec {}
