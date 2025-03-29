///Register `ADRD` reader
pub type R = crate::R<AdrdSpec>;
///Field `AD` reader - Converted Value 11 to 0
pub type AdR = crate::FieldReader<u16>;
/**Self-Diagnosis Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Diagst {
    ///0: Self-diagnosis not executed after power-on.
    _00 = 0,
    ///1: Self-diagnosis was executed using the 0 V voltage.
    _01 = 1,
    ///2: Self-diagnosis was executed using the reference voltage × 1/2.
    _10 = 2,
    ///3: Self-diagnosis was executed using the reference voltage .
    _11 = 3,
}
impl From<Diagst> for u8 {
    #[inline(always)]
    fn from(variant: Diagst) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Diagst {
    type Ux = u8;
}
impl crate::IsEnum for Diagst {}
///Field `DIAGST` reader - Self-Diagnosis Status
pub type DiagstR = crate::FieldReader<Diagst>;
impl DiagstR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Diagst {
        match self.bits {
            0 => Diagst::_00,
            1 => Diagst::_01,
            2 => Diagst::_10,
            3 => Diagst::_11,
            _ => unreachable!(),
        }
    }
    ///Self-diagnosis not executed after power-on.
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Diagst::_00
    }
    ///Self-diagnosis was executed using the 0 V voltage.
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Diagst::_01
    }
    ///Self-diagnosis was executed using the reference voltage × 1/2.
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Diagst::_10
    }
    ///Self-diagnosis was executed using the reference voltage .
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Diagst::_11
    }
}
impl R {
    ///Bits 0:11 - Converted Value 11 to 0
    #[inline(always)]
    pub fn ad(&self) -> AdR {
        AdR::new(self.bits & 0x0fff)
    }
    ///Bits 14:15 - Self-Diagnosis Status
    #[inline(always)]
    pub fn diagst(&self) -> DiagstR {
        DiagstR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADRD")
            .field("ad", &self.ad())
            .field("diagst", &self.diagst())
            .finish()
    }
}
/**A/D Self-Diagnosis Data Register

You can [`read`](crate::Reg::read) this register and get [`adrd::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AdrdSpec;
impl crate::RegisterSpec for AdrdSpec {
    type Ux = u16;
}
///`read()` method returns [`adrd::R`](R) reader structure
impl crate::Readable for AdrdSpec {}
///`reset()` method sets ADRD to value 0
impl crate::Resettable for AdrdSpec {}
