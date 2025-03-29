///Register `CFDFFFSTS` reader
pub type R = crate::R<CfdfffstsSpec>;
/**RX FIFO FDC Level Full Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rfxffll {
    ///0: Corresponding FIFO full interrupt not set
    _0 = 0,
    ///1: Corresponding FIFO full interrupt is set
    _1 = 1,
}
impl From<Rfxffll> for u8 {
    #[inline(always)]
    fn from(variant: Rfxffll) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rfxffll {
    type Ux = u8;
}
impl crate::IsEnum for Rfxffll {}
///Field `RFXFFLL` reader - RX FIFO FDC Level Full Status
pub type RfxffllR = crate::FieldReader<Rfxffll>;
impl RfxffllR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rfxffll> {
        match self.bits {
            0 => Some(Rfxffll::_0),
            1 => Some(Rfxffll::_1),
            _ => None,
        }
    }
    ///Corresponding FIFO full interrupt not set
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rfxffll::_0
    }
    ///Corresponding FIFO full interrupt is set
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rfxffll::_1
    }
}
/**COMMON FIFO FDC Level Full Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cfxffll {
    ///0: Corresponding FIFO full interrupt not set
    _0 = 0,
    ///1: Corresponding FIFO full interrupt is set
    _1 = 1,
}
impl From<Cfxffll> for u8 {
    #[inline(always)]
    fn from(variant: Cfxffll) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cfxffll {
    type Ux = u8;
}
impl crate::IsEnum for Cfxffll {}
///Field `CFXFFLL` reader - COMMON FIFO FDC Level Full Status
pub type CfxffllR = crate::FieldReader<Cfxffll>;
impl CfxffllR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cfxffll> {
        match self.bits {
            0 => Some(Cfxffll::_0),
            1 => Some(Cfxffll::_1),
            _ => None,
        }
    }
    ///Corresponding FIFO full interrupt not set
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cfxffll::_0
    }
    ///Corresponding FIFO full interrupt is set
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cfxffll::_1
    }
}
impl R {
    ///Bits 0:7 - RX FIFO FDC Level Full Status
    #[inline(always)]
    pub fn rfxffll(&self) -> RfxffllR {
        RfxffllR::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:13 - COMMON FIFO FDC Level Full Status
    #[inline(always)]
    pub fn cfxffll(&self) -> CfxffllR {
        CfxffllR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDFFFSTS")
            .field("rfxffll", &self.rfxffll())
            .field("cfxffll", &self.cfxffll())
            .finish()
    }
}
/**FIFO FDC Full Status Register

You can [`read`](crate::Reg::read) this register and get [`cfdfffsts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdfffstsSpec;
impl crate::RegisterSpec for CfdfffstsSpec {
    type Ux = u32;
}
///`read()` method returns [`cfdfffsts::R`](R) reader structure
impl crate::Readable for CfdfffstsSpec {}
///`reset()` method sets CFDFFFSTS to value 0
impl crate::Resettable for CfdfffstsSpec {}
