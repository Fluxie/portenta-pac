///Register `DLMMON` reader
pub type R = crate::R<DlmmonSpec>;
/**Device Lifecycle Management State Monitor

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dlmmon {
    ///1: CM
    _0x1 = 1,
    ///2: SSD
    _0x2 = 2,
    ///3: NSECSD
    _0x3 = 3,
    ///4: DPL
    _0x4 = 4,
    ///5: LCK_DBG
    _0x5 = 5,
    ///6: LCK_BOOT
    _0x6 = 6,
    ///7: RMA_REQ
    _0x7 = 7,
    ///8: RMA_ACK
    _0x8 = 8,
    ///0: Reserved
    Others = 0,
}
impl From<Dlmmon> for u8 {
    #[inline(always)]
    fn from(variant: Dlmmon) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dlmmon {
    type Ux = u8;
}
impl crate::IsEnum for Dlmmon {}
///Field `DLMMON` reader - Device Lifecycle Management State Monitor
pub type DlmmonR = crate::FieldReader<Dlmmon>;
impl DlmmonR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dlmmon {
        match self.bits {
            1 => Dlmmon::_0x1,
            2 => Dlmmon::_0x2,
            3 => Dlmmon::_0x3,
            4 => Dlmmon::_0x4,
            5 => Dlmmon::_0x5,
            6 => Dlmmon::_0x6,
            7 => Dlmmon::_0x7,
            8 => Dlmmon::_0x8,
            _ => Dlmmon::Others,
        }
    }
    ///CM
    #[inline(always)]
    pub fn is_0x1(&self) -> bool {
        *self == Dlmmon::_0x1
    }
    ///SSD
    #[inline(always)]
    pub fn is_0x2(&self) -> bool {
        *self == Dlmmon::_0x2
    }
    ///NSECSD
    #[inline(always)]
    pub fn is_0x3(&self) -> bool {
        *self == Dlmmon::_0x3
    }
    ///DPL
    #[inline(always)]
    pub fn is_0x4(&self) -> bool {
        *self == Dlmmon::_0x4
    }
    ///LCK_DBG
    #[inline(always)]
    pub fn is_0x5(&self) -> bool {
        *self == Dlmmon::_0x5
    }
    ///LCK_BOOT
    #[inline(always)]
    pub fn is_0x6(&self) -> bool {
        *self == Dlmmon::_0x6
    }
    ///RMA_REQ
    #[inline(always)]
    pub fn is_0x7(&self) -> bool {
        *self == Dlmmon::_0x7
    }
    ///RMA_ACK
    #[inline(always)]
    pub fn is_0x8(&self) -> bool {
        *self == Dlmmon::_0x8
    }
    ///Reserved
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Dlmmon::Others)
    }
}
impl R {
    ///Bits 0:3 - Device Lifecycle Management State Monitor
    #[inline(always)]
    pub fn dlmmon(&self) -> DlmmonR {
        DlmmonR::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DLMMON").field("dlmmon", &self.dlmmon()).finish()
    }
}
/**Device Lifecycle Management State Monitor Register

You can [`read`](crate::Reg::read) this register and get [`dlmmon::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DlmmonSpec;
impl crate::RegisterSpec for DlmmonSpec {
    type Ux = u32;
}
///`read()` method returns [`dlmmon::R`](R) reader structure
impl crate::Readable for DlmmonSpec {}
///`reset()` method sets DLMMON to value 0
impl crate::Resettable for DlmmonSpec {}
