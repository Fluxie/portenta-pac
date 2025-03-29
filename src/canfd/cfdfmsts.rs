///Register `CFDFMSTS` reader
pub type R = crate::R<CfdfmstsSpec>;
/**RX FIFO Message Lost Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rfxmlt {
    ///0: Corresponding FIFO Message Lost flag not set
    _0 = 0,
    ///1: Corresponding FIFO Message Lost flag set
    _1 = 1,
}
impl From<Rfxmlt> for u8 {
    #[inline(always)]
    fn from(variant: Rfxmlt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rfxmlt {
    type Ux = u8;
}
impl crate::IsEnum for Rfxmlt {}
///Field `RFXMLT` reader - RX FIFO Message Lost Status
pub type RfxmltR = crate::FieldReader<Rfxmlt>;
impl RfxmltR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rfxmlt> {
        match self.bits {
            0 => Some(Rfxmlt::_0),
            1 => Some(Rfxmlt::_1),
            _ => None,
        }
    }
    ///Corresponding FIFO Message Lost flag not set
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rfxmlt::_0
    }
    ///Corresponding FIFO Message Lost flag set
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rfxmlt::_1
    }
}
/**Common FIFO Message Lost Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cfxmlt {
    ///0: Corresponding FIFO Message Lost flag not set
    _0 = 0,
    ///1: Corresponding FIFO Message Lost flag set
    _1 = 1,
}
impl From<Cfxmlt> for u8 {
    #[inline(always)]
    fn from(variant: Cfxmlt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cfxmlt {
    type Ux = u8;
}
impl crate::IsEnum for Cfxmlt {}
///Field `CFXMLT` reader - Common FIFO Message Lost Status
pub type CfxmltR = crate::FieldReader<Cfxmlt>;
impl CfxmltR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cfxmlt> {
        match self.bits {
            0 => Some(Cfxmlt::_0),
            1 => Some(Cfxmlt::_1),
            _ => None,
        }
    }
    ///Corresponding FIFO Message Lost flag not set
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cfxmlt::_0
    }
    ///Corresponding FIFO Message Lost flag set
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cfxmlt::_1
    }
}
impl R {
    ///Bits 0:7 - RX FIFO Message Lost Status
    #[inline(always)]
    pub fn rfxmlt(&self) -> RfxmltR {
        RfxmltR::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:13 - Common FIFO Message Lost Status
    #[inline(always)]
    pub fn cfxmlt(&self) -> CfxmltR {
        CfxmltR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDFMSTS")
            .field("rfxmlt", &self.rfxmlt())
            .field("cfxmlt", &self.cfxmlt())
            .finish()
    }
}
/**FIFO Message Lost Status Register

You can [`read`](crate::Reg::read) this register and get [`cfdfmsts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdfmstsSpec;
impl crate::RegisterSpec for CfdfmstsSpec {
    type Ux = u32;
}
///`read()` method returns [`cfdfmsts::R`](R) reader structure
impl crate::Readable for CfdfmstsSpec {}
///`reset()` method sets CFDFMSTS to value 0
impl crate::Resettable for CfdfmstsSpec {}
