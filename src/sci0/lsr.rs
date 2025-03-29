///Register `LSR` reader
pub type R = crate::R<LsrSpec>;
/**Overrun Error Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Orer {
    ///0: No overrun error occurred
    _0 = 0,
    ///1: Overrun error occurred
    _1 = 1,
}
impl From<Orer> for bool {
    #[inline(always)]
    fn from(variant: Orer) -> Self {
        variant as u8 != 0
    }
}
///Field `ORER` reader - Overrun Error Flag
pub type OrerR = crate::BitReader<Orer>;
impl OrerR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Orer {
        match self.bits {
            false => Orer::_0,
            true => Orer::_1,
        }
    }
    ///No overrun error occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Orer::_0
    }
    ///Overrun error occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Orer::_1
    }
}
///Field `FNUM` reader - Framing Error Count
pub type FnumR = crate::FieldReader;
///Field `PNUM` reader - Parity Error Count
pub type PnumR = crate::FieldReader;
impl R {
    ///Bit 0 - Overrun Error Flag
    #[inline(always)]
    pub fn orer(&self) -> OrerR {
        OrerR::new((self.bits & 1) != 0)
    }
    ///Bits 2:6 - Framing Error Count
    #[inline(always)]
    pub fn fnum(&self) -> FnumR {
        FnumR::new(((self.bits >> 2) & 0x1f) as u8)
    }
    ///Bits 8:12 - Parity Error Count
    #[inline(always)]
    pub fn pnum(&self) -> PnumR {
        PnumR::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LSR")
            .field("orer", &self.orer())
            .field("fnum", &self.fnum())
            .field("pnum", &self.pnum())
            .finish()
    }
}
/**Line Status Register

You can [`read`](crate::Reg::read) this register and get [`lsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LsrSpec;
impl crate::RegisterSpec for LsrSpec {
    type Ux = u16;
}
///`read()` method returns [`lsr::R`](R) reader structure
impl crate::Readable for LsrSpec {}
///`reset()` method sets LSR to value 0
impl crate::Resettable for LsrSpec {}
