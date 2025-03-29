///Register `CFDFESTS` reader
pub type R = crate::R<CfdfestsSpec>;
/**RX FIFO Empty Status

Value on reset: 255*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rfxemp {
    ///0: Corresponding FIFO not empty
    _0 = 0,
    ///1: Corresponding FIFO empty
    _1 = 1,
}
impl From<Rfxemp> for u8 {
    #[inline(always)]
    fn from(variant: Rfxemp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rfxemp {
    type Ux = u8;
}
impl crate::IsEnum for Rfxemp {}
///Field `RFXEMP` reader - RX FIFO Empty Status
pub type RfxempR = crate::FieldReader<Rfxemp>;
impl RfxempR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rfxemp> {
        match self.bits {
            0 => Some(Rfxemp::_0),
            1 => Some(Rfxemp::_1),
            _ => None,
        }
    }
    ///Corresponding FIFO not empty
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rfxemp::_0
    }
    ///Corresponding FIFO empty
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rfxemp::_1
    }
}
/**Common FIFO Empty Status

Value on reset: 63*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cfxemp {
    ///0: Corresponding FIFO not empty
    _0 = 0,
    ///1: Corresponding FIFO empty
    _1 = 1,
}
impl From<Cfxemp> for u8 {
    #[inline(always)]
    fn from(variant: Cfxemp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cfxemp {
    type Ux = u8;
}
impl crate::IsEnum for Cfxemp {}
///Field `CFXEMP` reader - Common FIFO Empty Status
pub type CfxempR = crate::FieldReader<Cfxemp>;
impl CfxempR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cfxemp> {
        match self.bits {
            0 => Some(Cfxemp::_0),
            1 => Some(Cfxemp::_1),
            _ => None,
        }
    }
    ///Corresponding FIFO not empty
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cfxemp::_0
    }
    ///Corresponding FIFO empty
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cfxemp::_1
    }
}
impl R {
    ///Bits 0:7 - RX FIFO Empty Status
    #[inline(always)]
    pub fn rfxemp(&self) -> RfxempR {
        RfxempR::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:13 - Common FIFO Empty Status
    #[inline(always)]
    pub fn cfxemp(&self) -> CfxempR {
        CfxempR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDFESTS")
            .field("rfxemp", &self.rfxemp())
            .field("cfxemp", &self.cfxemp())
            .finish()
    }
}
/**FIFO Empty Status Register

You can [`read`](crate::Reg::read) this register and get [`cfdfests::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdfestsSpec;
impl crate::RegisterSpec for CfdfestsSpec {
    type Ux = u32;
}
///`read()` method returns [`cfdfests::R`](R) reader structure
impl crate::Readable for CfdfestsSpec {}
///`reset()` method sets CFDFESTS to value 0x3fff
impl crate::Resettable for CfdfestsSpec {
    const RESET_VALUE: u32 = 0x3fff;
}
