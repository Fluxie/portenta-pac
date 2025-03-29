///Register `CFDCFOFRISTS` reader
pub type R = crate::R<CfdcfofristsSpec>;
/**Common FIFO One Frame RX Interrupt Flag Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cfxofrxif {
    ///0: Corresponding Common FIFO One Frame RX Interrupt flag is not set
    _0 = 0,
    ///1: Corresponding Common FIFO One Frame RX Interrupt flag is set
    _1 = 1,
}
impl From<Cfxofrxif> for u8 {
    #[inline(always)]
    fn from(variant: Cfxofrxif) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cfxofrxif {
    type Ux = u8;
}
impl crate::IsEnum for Cfxofrxif {}
///Field `CFXOFRXIF` reader - Common FIFO One Frame RX Interrupt Flag Status
pub type CfxofrxifR = crate::FieldReader<Cfxofrxif>;
impl CfxofrxifR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cfxofrxif> {
        match self.bits {
            0 => Some(Cfxofrxif::_0),
            1 => Some(Cfxofrxif::_1),
            _ => None,
        }
    }
    ///Corresponding Common FIFO One Frame RX Interrupt flag is not set
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cfxofrxif::_0
    }
    ///Corresponding Common FIFO One Frame RX Interrupt flag is set
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cfxofrxif::_1
    }
}
impl R {
    ///Bits 0:5 - Common FIFO One Frame RX Interrupt Flag Status
    #[inline(always)]
    pub fn cfxofrxif(&self) -> CfxofrxifR {
        CfxofrxifR::new((self.bits & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDCFOFRISTS").field("cfxofrxif", &self.cfxofrxif()).finish()
    }
}
/**Common FIFO One Frame RX Interrupt Flag Status Register

You can [`read`](crate::Reg::read) this register and get [`cfdcfofrists::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdcfofristsSpec;
impl crate::RegisterSpec for CfdcfofristsSpec {
    type Ux = u32;
}
///`read()` method returns [`cfdcfofrists::R`](R) reader structure
impl crate::Readable for CfdcfofristsSpec {}
///`reset()` method sets CFDCFOFRISTS to value 0
impl crate::Resettable for CfdcfofristsSpec {}
