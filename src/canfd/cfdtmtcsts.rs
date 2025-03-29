///Register `CFDTMTCSTS%s` reader
pub type R = crate::R<CfdtmtcstsSpec>;
/**TX Message Buffer Transmission Completion Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cfdtmtcsts {
    ///0: Transmission not complete for corresponding TX message buffer
    _0 = 0,
    ///1: Transmission completed for corresponding TX message buffer
    _1 = 1,
}
impl From<Cfdtmtcsts> for u8 {
    #[inline(always)]
    fn from(variant: Cfdtmtcsts) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cfdtmtcsts {
    type Ux = u8;
}
impl crate::IsEnum for Cfdtmtcsts {}
///Field `CFDTMTCSTS` reader - TX Message Buffer Transmission Completion Status
pub type CfdtmtcstsR = crate::FieldReader<Cfdtmtcsts>;
impl CfdtmtcstsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cfdtmtcsts> {
        match self.bits {
            0 => Some(Cfdtmtcsts::_0),
            1 => Some(Cfdtmtcsts::_1),
            _ => None,
        }
    }
    ///Transmission not complete for corresponding TX message buffer
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cfdtmtcsts::_0
    }
    ///Transmission completed for corresponding TX message buffer
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cfdtmtcsts::_1
    }
}
impl R {
    ///Bits 0:7 - TX Message Buffer Transmission Completion Status
    #[inline(always)]
    pub fn cfdtmtcsts(&self) -> CfdtmtcstsR {
        CfdtmtcstsR::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDTMTCSTS").field("cfdtmtcsts", &self.cfdtmtcsts()).finish()
    }
}
/**TX Message Buffer Transmission Completion Status Register %s

You can [`read`](crate::Reg::read) this register and get [`cfdtmtcsts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdtmtcstsSpec;
impl crate::RegisterSpec for CfdtmtcstsSpec {
    type Ux = u32;
}
///`read()` method returns [`cfdtmtcsts::R`](R) reader structure
impl crate::Readable for CfdtmtcstsSpec {}
///`reset()` method sets CFDTMTCSTS%s to value 0
impl crate::Resettable for CfdtmtcstsSpec {}
