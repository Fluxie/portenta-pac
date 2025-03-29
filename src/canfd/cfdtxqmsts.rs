///Register `CFDTXQMSTS` reader
pub type R = crate::R<CfdtxqmstsSpec>;
///Register `CFDTXQMSTS` writer
pub type W = crate::W<CfdtxqmstsSpec>;
/**TXQ Message Lost Status for Channel 0

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Txq0ml {
    ///0: TXQ message lost flag is not set
    _0 = 0,
    ///1: TXQ message lost flag is set
    _1 = 1,
}
impl From<Txq0ml> for u8 {
    #[inline(always)]
    fn from(variant: Txq0ml) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Txq0ml {
    type Ux = u8;
}
impl crate::IsEnum for Txq0ml {}
///Field `TXQ0ML` reader - TXQ Message Lost Status for Channel 0
pub type Txq0mlR = crate::FieldReader<Txq0ml>;
impl Txq0mlR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<Txq0ml> {
        match self.bits {
            0 => Some(Txq0ml::_0),
            1 => Some(Txq0ml::_1),
            _ => None,
        }
    }
    ///TXQ message lost flag is not set
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Txq0ml::_0
    }
    ///TXQ message lost flag is set
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Txq0ml::_1
    }
}
/**TXQ Message Lost Status for Channel 1

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Txq1ml {
    ///0: TXQ message lost flag is not set
    _0 = 0,
    ///1: TXQ message lost flag is set
    _1 = 1,
}
impl From<Txq1ml> for u8 {
    #[inline(always)]
    fn from(variant: Txq1ml) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Txq1ml {
    type Ux = u8;
}
impl crate::IsEnum for Txq1ml {}
///Field `TXQ1ML` reader - TXQ Message Lost Status for Channel 1
pub type Txq1mlR = crate::FieldReader<Txq1ml>;
impl Txq1mlR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<Txq1ml> {
        match self.bits {
            0 => Some(Txq1ml::_0),
            1 => Some(Txq1ml::_1),
            _ => None,
        }
    }
    ///TXQ message lost flag is not set
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Txq1ml::_0
    }
    ///TXQ message lost flag is set
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Txq1ml::_1
    }
}
impl R {
    ///Bits 0:2 - TXQ Message Lost Status for Channel 0
    #[inline(always)]
    pub fn txq0ml(&self) -> Txq0mlR {
        Txq0mlR::new((self.bits & 7) as u8)
    }
    ///Bits 4:6 - TXQ Message Lost Status for Channel 1
    #[inline(always)]
    pub fn txq1ml(&self) -> Txq1mlR {
        Txq1mlR::new(((self.bits >> 4) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDTXQMSTS")
            .field("txq0ml", &self.txq0ml())
            .field("txq1ml", &self.txq1ml())
            .finish()
    }
}
impl W {}
/**TX Queue Message Lost Status Register

You can [`read`](crate::Reg::read) this register and get [`cfdtxqmsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtxqmsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdtxqmstsSpec;
impl crate::RegisterSpec for CfdtxqmstsSpec {
    type Ux = u32;
}
///`read()` method returns [`cfdtxqmsts::R`](R) reader structure
impl crate::Readable for CfdtxqmstsSpec {}
///`write(|w| ..)` method takes [`cfdtxqmsts::W`](W) writer structure
impl crate::Writable for CfdtxqmstsSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDTXQMSTS to value 0
impl crate::Resettable for CfdtxqmstsSpec {}
