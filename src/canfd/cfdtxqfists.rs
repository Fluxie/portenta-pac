///Register `CFDTXQFISTS` reader
pub type R = crate::R<CfdtxqfistsSpec>;
///Register `CFDTXQFISTS` writer
pub type W = crate::W<CfdtxqfistsSpec>;
/**TXQ Full Interrupt Status for Channel 0

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Txq0full {
    ///0: TXQ full interrupt is not set
    _0 = 0,
    ///1: TXQ full interrupt is set
    _1 = 1,
}
impl From<Txq0full> for u8 {
    #[inline(always)]
    fn from(variant: Txq0full) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Txq0full {
    type Ux = u8;
}
impl crate::IsEnum for Txq0full {}
///Field `TXQ0FULL` reader - TXQ Full Interrupt Status for Channel 0
pub type Txq0fullR = crate::FieldReader<Txq0full>;
impl Txq0fullR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<Txq0full> {
        match self.bits {
            0 => Some(Txq0full::_0),
            1 => Some(Txq0full::_1),
            _ => None,
        }
    }
    ///TXQ full interrupt is not set
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Txq0full::_0
    }
    ///TXQ full interrupt is set
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Txq0full::_1
    }
}
/**TXQ Full Interrupt Status for Channel 1

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Txq1full {
    ///0: TXQ full interrupt is not set
    _0 = 0,
    ///1: TXQ full interrupt is set
    _1 = 1,
}
impl From<Txq1full> for u8 {
    #[inline(always)]
    fn from(variant: Txq1full) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Txq1full {
    type Ux = u8;
}
impl crate::IsEnum for Txq1full {}
///Field `TXQ1FULL` reader - TXQ Full Interrupt Status for Channel 1
pub type Txq1fullR = crate::FieldReader<Txq1full>;
impl Txq1fullR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<Txq1full> {
        match self.bits {
            0 => Some(Txq1full::_0),
            1 => Some(Txq1full::_1),
            _ => None,
        }
    }
    ///TXQ full interrupt is not set
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Txq1full::_0
    }
    ///TXQ full interrupt is set
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Txq1full::_1
    }
}
impl R {
    ///Bits 0:2 - TXQ Full Interrupt Status for Channel 0
    #[inline(always)]
    pub fn txq0full(&self) -> Txq0fullR {
        Txq0fullR::new((self.bits & 7) as u8)
    }
    ///Bits 4:6 - TXQ Full Interrupt Status for Channel 1
    #[inline(always)]
    pub fn txq1full(&self) -> Txq1fullR {
        Txq1fullR::new(((self.bits >> 4) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDTXQFISTS")
            .field("txq0full", &self.txq0full())
            .field("txq1full", &self.txq1full())
            .finish()
    }
}
impl W {}
/**TX Queue Full Interrupt Status Register

You can [`read`](crate::Reg::read) this register and get [`cfdtxqfists::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtxqfists::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdtxqfistsSpec;
impl crate::RegisterSpec for CfdtxqfistsSpec {
    type Ux = u32;
}
///`read()` method returns [`cfdtxqfists::R`](R) reader structure
impl crate::Readable for CfdtxqfistsSpec {}
///`write(|w| ..)` method takes [`cfdtxqfists::W`](W) writer structure
impl crate::Writable for CfdtxqfistsSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDTXQFISTS to value 0
impl crate::Resettable for CfdtxqfistsSpec {}
