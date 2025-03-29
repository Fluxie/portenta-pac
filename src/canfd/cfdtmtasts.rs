///Register `CFDTMTASTS%s` reader
pub type R = crate::R<CfdtmtastsSpec>;
/**TX Message Buffer Transmission Abort Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cfdtmtasts {
    ///0: Transmission not aborted for corresponding TX message buffer
    _0 = 0,
    ///1: Transmission aborted for corresponding TX message buffer
    _1 = 1,
}
impl From<Cfdtmtasts> for u8 {
    #[inline(always)]
    fn from(variant: Cfdtmtasts) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cfdtmtasts {
    type Ux = u8;
}
impl crate::IsEnum for Cfdtmtasts {}
///Field `CFDTMTASTS` reader - TX Message Buffer Transmission Abort Status
pub type CfdtmtastsR = crate::FieldReader<Cfdtmtasts>;
impl CfdtmtastsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cfdtmtasts> {
        match self.bits {
            0 => Some(Cfdtmtasts::_0),
            1 => Some(Cfdtmtasts::_1),
            _ => None,
        }
    }
    ///Transmission not aborted for corresponding TX message buffer
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cfdtmtasts::_0
    }
    ///Transmission aborted for corresponding TX message buffer
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cfdtmtasts::_1
    }
}
impl R {
    ///Bits 0:7 - TX Message Buffer Transmission Abort Status
    #[inline(always)]
    pub fn cfdtmtasts(&self) -> CfdtmtastsR {
        CfdtmtastsR::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDTMTASTS").field("cfdtmtasts", &self.cfdtmtasts()).finish()
    }
}
/**TX Message Buffer Transmission Abort Status Register %s

You can [`read`](crate::Reg::read) this register and get [`cfdtmtasts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdtmtastsSpec;
impl crate::RegisterSpec for CfdtmtastsSpec {
    type Ux = u32;
}
///`read()` method returns [`cfdtmtasts::R`](R) reader structure
impl crate::Readable for CfdtmtastsSpec {}
///`reset()` method sets CFDTMTASTS%s to value 0
impl crate::Resettable for CfdtmtastsSpec {}
