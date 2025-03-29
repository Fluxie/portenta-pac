///Register `CFDTHLACC0%s` reader
pub type R = crate::R<Cfdthlacc0Spec>;
/**Buffer Type

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bt {
    ///1: Flat TX message buffer
    _001 = 1,
    ///2: TX FIFO message buffer number and gateway FIFO message number
    _010 = 2,
    ///4: TX Queue message buffer number
    _100 = 4,
}
impl From<Bt> for u8 {
    #[inline(always)]
    fn from(variant: Bt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bt {
    type Ux = u8;
}
impl crate::IsEnum for Bt {}
///Field `BT` reader - Buffer Type
pub type BtR = crate::FieldReader<Bt>;
impl BtR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<Bt> {
        match self.bits {
            1 => Some(Bt::_001),
            2 => Some(Bt::_010),
            4 => Some(Bt::_100),
            _ => None,
        }
    }
    ///Flat TX message buffer
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Bt::_001
    }
    ///TX FIFO message buffer number and gateway FIFO message number
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Bt::_010
    }
    ///TX Queue message buffer number
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Bt::_100
    }
}
///Field `BN` reader - Buffer Number
pub type BnR = crate::FieldReader;
/**Transmit Gateway Buffer Indication

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tgw {
    ///0: No transmission from gateway
    _0 = 0,
    ///1: Transmission from gateway
    _1 = 1,
}
impl From<Tgw> for bool {
    #[inline(always)]
    fn from(variant: Tgw) -> Self {
        variant as u8 != 0
    }
}
///Field `TGW` reader - Transmit Gateway Buffer Indication
pub type TgwR = crate::BitReader<Tgw>;
impl TgwR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tgw {
        match self.bits {
            false => Tgw::_0,
            true => Tgw::_1,
        }
    }
    ///No transmission from gateway
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tgw::_0
    }
    ///Transmission from gateway
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tgw::_1
    }
}
///Field `TMTS` reader - Transmit Timestamp
pub type TmtsR = crate::FieldReader<u16>;
impl R {
    ///Bits 0:2 - Buffer Type
    #[inline(always)]
    pub fn bt(&self) -> BtR {
        BtR::new((self.bits & 7) as u8)
    }
    ///Bits 3:9 - Buffer Number
    #[inline(always)]
    pub fn bn(&self) -> BnR {
        BnR::new(((self.bits >> 3) & 0x7f) as u8)
    }
    ///Bit 15 - Transmit Gateway Buffer Indication
    #[inline(always)]
    pub fn tgw(&self) -> TgwR {
        TgwR::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:31 - Transmit Timestamp
    #[inline(always)]
    pub fn tmts(&self) -> TmtsR {
        TmtsR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDTHLACC0")
            .field("bt", &self.bt())
            .field("bn", &self.bn())
            .field("tgw", &self.tgw())
            .field("tmts", &self.tmts())
            .finish()
    }
}
/**TX History List Access Registers 0

You can [`read`](crate::Reg::read) this register and get [`cfdthlacc0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Cfdthlacc0Spec;
impl crate::RegisterSpec for Cfdthlacc0Spec {
    type Ux = u32;
}
///`read()` method returns [`cfdthlacc0::R`](R) reader structure
impl crate::Readable for Cfdthlacc0Spec {}
///`reset()` method sets CFDTHLACC0%s to value 0
impl crate::Resettable for Cfdthlacc0Spec {}
