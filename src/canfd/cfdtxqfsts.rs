///Register `CFDTXQFSTS` reader
pub type R = crate::R<CfdtxqfstsSpec>;
/**TXQ Full Status Flag for Channel 0

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Txq0fsf {
    ///0: TXQ Full flag is not set
    _0 = 0,
    ///1: TXQ Full flag is set
    _1 = 1,
}
impl From<Txq0fsf> for u8 {
    #[inline(always)]
    fn from(variant: Txq0fsf) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Txq0fsf {
    type Ux = u8;
}
impl crate::IsEnum for Txq0fsf {}
///Field `TXQ0FSF` reader - TXQ Full Status Flag for Channel 0
pub type Txq0fsfR = crate::FieldReader<Txq0fsf>;
impl Txq0fsfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<Txq0fsf> {
        match self.bits {
            0 => Some(Txq0fsf::_0),
            1 => Some(Txq0fsf::_1),
            _ => None,
        }
    }
    ///TXQ Full flag is not set
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Txq0fsf::_0
    }
    ///TXQ Full flag is set
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Txq0fsf::_1
    }
}
/**TXQ Full Status Flag for Channel 1

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Txq1fsf {
    ///0: TXQ Full flag is not set
    _0 = 0,
    ///1: TXQ Full flag is set
    _1 = 1,
}
impl From<Txq1fsf> for u8 {
    #[inline(always)]
    fn from(variant: Txq1fsf) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Txq1fsf {
    type Ux = u8;
}
impl crate::IsEnum for Txq1fsf {}
///Field `TXQ1FSF` reader - TXQ Full Status Flag for Channel 1
pub type Txq1fsfR = crate::FieldReader<Txq1fsf>;
impl Txq1fsfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<Txq1fsf> {
        match self.bits {
            0 => Some(Txq1fsf::_0),
            1 => Some(Txq1fsf::_1),
            _ => None,
        }
    }
    ///TXQ Full flag is not set
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Txq1fsf::_0
    }
    ///TXQ Full flag is set
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Txq1fsf::_1
    }
}
impl R {
    ///Bits 0:3 - TXQ Full Status Flag for Channel 0
    #[inline(always)]
    pub fn txq0fsf(&self) -> Txq0fsfR {
        Txq0fsfR::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - TXQ Full Status Flag for Channel 1
    #[inline(always)]
    pub fn txq1fsf(&self) -> Txq1fsfR {
        Txq1fsfR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDTXQFSTS")
            .field("txq0fsf", &self.txq0fsf())
            .field("txq1fsf", &self.txq1fsf())
            .finish()
    }
}
/**TX Queue Full Status Register

You can [`read`](crate::Reg::read) this register and get [`cfdtxqfsts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdtxqfstsSpec;
impl crate::RegisterSpec for CfdtxqfstsSpec {
    type Ux = u32;
}
///`read()` method returns [`cfdtxqfsts::R`](R) reader structure
impl crate::Readable for CfdtxqfstsSpec {}
///`reset()` method sets CFDTXQFSTS to value 0
impl crate::Resettable for CfdtxqfstsSpec {}
