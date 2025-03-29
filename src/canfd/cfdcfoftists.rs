///Register `CFDCFOFTISTS` reader
pub type R = crate::R<CfdcfoftistsSpec>;
/**Common FIFO One Frame TX Interrupt Flag Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cfxoftxif {
    ///0: Corresponding Common FIFO One Frame TX Interrupt flag is not set
    _0 = 0,
    ///1: Corresponding Common FIFO One Frame TX Interrupt flag is set
    _1 = 1,
}
impl From<Cfxoftxif> for u8 {
    #[inline(always)]
    fn from(variant: Cfxoftxif) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cfxoftxif {
    type Ux = u8;
}
impl crate::IsEnum for Cfxoftxif {}
///Field `CFXOFTXIF` reader - Common FIFO One Frame TX Interrupt Flag Status
pub type CfxoftxifR = crate::FieldReader<Cfxoftxif>;
impl CfxoftxifR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cfxoftxif> {
        match self.bits {
            0 => Some(Cfxoftxif::_0),
            1 => Some(Cfxoftxif::_1),
            _ => None,
        }
    }
    ///Corresponding Common FIFO One Frame TX Interrupt flag is not set
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cfxoftxif::_0
    }
    ///Corresponding Common FIFO One Frame TX Interrupt flag is set
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cfxoftxif::_1
    }
}
impl R {
    ///Bits 0:5 - Common FIFO One Frame TX Interrupt Flag Status
    #[inline(always)]
    pub fn cfxoftxif(&self) -> CfxoftxifR {
        CfxoftxifR::new((self.bits & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDCFOFTISTS").field("cfxoftxif", &self.cfxoftxif()).finish()
    }
}
/**Common FIFO One Frame TX Interrupt Flag Status Register

You can [`read`](crate::Reg::read) this register and get [`cfdcfoftists::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdcfoftistsSpec;
impl crate::RegisterSpec for CfdcfoftistsSpec {
    type Ux = u32;
}
///`read()` method returns [`cfdcfoftists::R`](R) reader structure
impl crate::Readable for CfdcfoftistsSpec {}
///`reset()` method sets CFDCFOFTISTS to value 0
impl crate::Resettable for CfdcfoftistsSpec {}
