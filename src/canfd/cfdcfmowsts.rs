///Register `CFDCFMOWSTS` reader
pub type R = crate::R<CfdcfmowstsSpec>;
/**Common FIFO Massage Overwrite Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cfxmow {
    ///0: Corresponding FIFO Overwrite flag is not set
    _0 = 0,
    ///1: Corresponding FIFO Overwrite flag is set
    _1 = 1,
}
impl From<Cfxmow> for u8 {
    #[inline(always)]
    fn from(variant: Cfxmow) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cfxmow {
    type Ux = u8;
}
impl crate::IsEnum for Cfxmow {}
///Field `CFXMOW` reader - Common FIFO Massage Overwrite Status
pub type CfxmowR = crate::FieldReader<Cfxmow>;
impl CfxmowR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cfxmow> {
        match self.bits {
            0 => Some(Cfxmow::_0),
            1 => Some(Cfxmow::_1),
            _ => None,
        }
    }
    ///Corresponding FIFO Overwrite flag is not set
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cfxmow::_0
    }
    ///Corresponding FIFO Overwrite flag is set
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cfxmow::_1
    }
}
impl R {
    ///Bits 0:5 - Common FIFO Massage Overwrite Status
    #[inline(always)]
    pub fn cfxmow(&self) -> CfxmowR {
        CfxmowR::new((self.bits & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDCFMOWSTS").field("cfxmow", &self.cfxmow()).finish()
    }
}
/**Common FIFO Message Over Write Status Register

You can [`read`](crate::Reg::read) this register and get [`cfdcfmowsts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdcfmowstsSpec;
impl crate::RegisterSpec for CfdcfmowstsSpec {
    type Ux = u32;
}
///`read()` method returns [`cfdcfmowsts::R`](R) reader structure
impl crate::Readable for CfdcfmowstsSpec {}
///`reset()` method sets CFDCFMOWSTS to value 0
impl crate::Resettable for CfdcfmowstsSpec {}
