///Register `SYSSTS0` reader
pub type R = crate::R<Syssts0Spec>;
///Field `LNST` reader - USB Data Line Status Monitor
pub type LnstR = crate::FieldReader;
/**External ID0 Input Pin Monitor

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Idmon {
    ///0: USB_ID pin is low
    _0 = 0,
    ///1: USB_ID pin is high
    _1 = 1,
}
impl From<Idmon> for bool {
    #[inline(always)]
    fn from(variant: Idmon) -> Self {
        variant as u8 != 0
    }
}
///Field `IDMON` reader - External ID0 Input Pin Monitor
pub type IdmonR = crate::BitReader<Idmon>;
impl IdmonR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Idmon {
        match self.bits {
            false => Idmon::_0,
            true => Idmon::_1,
        }
    }
    ///USB_ID pin is low
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Idmon::_0
    }
    ///USB_ID pin is high
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Idmon::_1
    }
}
/**Active Monitor When the Host Controller Is Selected

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sofea {
    ///0: SOF output stopped
    _0 = 0,
    ///1: SOF output operating
    _1 = 1,
}
impl From<Sofea> for bool {
    #[inline(always)]
    fn from(variant: Sofea) -> Self {
        variant as u8 != 0
    }
}
///Field `SOFEA` reader - Active Monitor When the Host Controller Is Selected
pub type SofeaR = crate::BitReader<Sofea>;
impl SofeaR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sofea {
        match self.bits {
            false => Sofea::_0,
            true => Sofea::_1,
        }
    }
    ///SOF output stopped
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sofea::_0
    }
    ///SOF output operating
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sofea::_1
    }
}
/**USB Host Sequencer Status Monitor

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Htact {
    ///0: Host sequencer completely stopped
    _0 = 0,
    ///1: Host sequencer not completely stopped
    _1 = 1,
}
impl From<Htact> for bool {
    #[inline(always)]
    fn from(variant: Htact) -> Self {
        variant as u8 != 0
    }
}
///Field `HTACT` reader - USB Host Sequencer Status Monitor
pub type HtactR = crate::BitReader<Htact>;
impl HtactR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Htact {
        match self.bits {
            false => Htact::_0,
            true => Htact::_1,
        }
    }
    ///Host sequencer completely stopped
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Htact::_0
    }
    ///Host sequencer not completely stopped
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Htact::_1
    }
}
///Field `OVCMON` reader - External USB_OVRCURA/ USB_OVRCURB Input Pin Monitor
pub type OvcmonR = crate::FieldReader;
impl R {
    ///Bits 0:1 - USB Data Line Status Monitor
    #[inline(always)]
    pub fn lnst(&self) -> LnstR {
        LnstR::new((self.bits & 3) as u8)
    }
    ///Bit 2 - External ID0 Input Pin Monitor
    #[inline(always)]
    pub fn idmon(&self) -> IdmonR {
        IdmonR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 5 - Active Monitor When the Host Controller Is Selected
    #[inline(always)]
    pub fn sofea(&self) -> SofeaR {
        SofeaR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - USB Host Sequencer Status Monitor
    #[inline(always)]
    pub fn htact(&self) -> HtactR {
        HtactR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 14:15 - External USB_OVRCURA/ USB_OVRCURB Input Pin Monitor
    #[inline(always)]
    pub fn ovcmon(&self) -> OvcmonR {
        OvcmonR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSSTS0")
            .field("lnst", &self.lnst())
            .field("idmon", &self.idmon())
            .field("sofea", &self.sofea())
            .field("htact", &self.htact())
            .field("ovcmon", &self.ovcmon())
            .finish()
    }
}
/**System Configuration Status Register 0

You can [`read`](crate::Reg::read) this register and get [`syssts0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Syssts0Spec;
impl crate::RegisterSpec for Syssts0Spec {
    type Ux = u16;
}
///`read()` method returns [`syssts0::R`](R) reader structure
impl crate::Readable for Syssts0Spec {}
///`reset()` method sets SYSSTS0 to value 0
impl crate::Resettable for Syssts0Spec {}
