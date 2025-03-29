///Register `CFDFFSTS` reader
pub type R = crate::R<CfdffstsSpec>;
/**RX FIFO Full Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rfxfll {
    ///0: Corresponding FIFO not full
    _0 = 0,
    ///1: Corresponding FIFO full
    _1 = 1,
}
impl From<Rfxfll> for u8 {
    #[inline(always)]
    fn from(variant: Rfxfll) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rfxfll {
    type Ux = u8;
}
impl crate::IsEnum for Rfxfll {}
///Field `RFXFLL` reader - RX FIFO Full Status
pub type RfxfllR = crate::FieldReader<Rfxfll>;
impl RfxfllR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rfxfll> {
        match self.bits {
            0 => Some(Rfxfll::_0),
            1 => Some(Rfxfll::_1),
            _ => None,
        }
    }
    ///Corresponding FIFO not full
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rfxfll::_0
    }
    ///Corresponding FIFO full
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rfxfll::_1
    }
}
/**Common FIFO Full Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cfxfll {
    ///0: Corresponding FIFO not full
    _0 = 0,
    ///1: Corresponding FIFO full
    _1 = 1,
}
impl From<Cfxfll> for u8 {
    #[inline(always)]
    fn from(variant: Cfxfll) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cfxfll {
    type Ux = u8;
}
impl crate::IsEnum for Cfxfll {}
///Field `CFXFLL` reader - Common FIFO Full Status
pub type CfxfllR = crate::FieldReader<Cfxfll>;
impl CfxfllR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cfxfll> {
        match self.bits {
            0 => Some(Cfxfll::_0),
            1 => Some(Cfxfll::_1),
            _ => None,
        }
    }
    ///Corresponding FIFO not full
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cfxfll::_0
    }
    ///Corresponding FIFO full
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cfxfll::_1
    }
}
impl R {
    ///Bits 0:7 - RX FIFO Full Status
    #[inline(always)]
    pub fn rfxfll(&self) -> RfxfllR {
        RfxfllR::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:13 - Common FIFO Full Status
    #[inline(always)]
    pub fn cfxfll(&self) -> CfxfllR {
        CfxfllR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDFFSTS")
            .field("rfxfll", &self.rfxfll())
            .field("cfxfll", &self.cfxfll())
            .finish()
    }
}
/**FIFO Full Status Register

You can [`read`](crate::Reg::read) this register and get [`cfdffsts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdffstsSpec;
impl crate::RegisterSpec for CfdffstsSpec {
    type Ux = u32;
}
///`read()` method returns [`cfdffsts::R`](R) reader structure
impl crate::Readable for CfdffstsSpec {}
///`reset()` method sets CFDFFSTS to value 0
impl crate::Resettable for CfdffstsSpec {}
