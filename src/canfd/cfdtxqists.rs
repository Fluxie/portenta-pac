///Register `CFDTXQISTS` reader
pub type R = crate::R<CfdtxqistsSpec>;
///Register `CFDTXQISTS` writer
pub type W = crate::W<CfdtxqistsSpec>;
/**TXQ Interrupt Status Flag for Channel 0

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Txq0isf {
    ///0: TXQ Interrupt flag is not set
    _0 = 0,
    ///1: TXQ Interrupt flag is set
    _1 = 1,
}
impl From<Txq0isf> for u8 {
    #[inline(always)]
    fn from(variant: Txq0isf) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Txq0isf {
    type Ux = u8;
}
impl crate::IsEnum for Txq0isf {}
///Field `TXQ0ISF` reader - TXQ Interrupt Status Flag for Channel 0
pub type Txq0isfR = crate::FieldReader<Txq0isf>;
impl Txq0isfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<Txq0isf> {
        match self.bits {
            0 => Some(Txq0isf::_0),
            1 => Some(Txq0isf::_1),
            _ => None,
        }
    }
    ///TXQ Interrupt flag is not set
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Txq0isf::_0
    }
    ///TXQ Interrupt flag is set
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Txq0isf::_1
    }
}
/**TXQ Interrupt Status Flag for Channel 1

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Txq1isf {
    ///0: TXQ Interrupt flag is not set
    _0 = 0,
    ///1: TXQ Interrupt flag is set
    _1 = 1,
}
impl From<Txq1isf> for u8 {
    #[inline(always)]
    fn from(variant: Txq1isf) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Txq1isf {
    type Ux = u8;
}
impl crate::IsEnum for Txq1isf {}
///Field `TXQ1ISF` reader - TXQ Interrupt Status Flag for Channel 1
pub type Txq1isfR = crate::FieldReader<Txq1isf>;
impl Txq1isfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<Txq1isf> {
        match self.bits {
            0 => Some(Txq1isf::_0),
            1 => Some(Txq1isf::_1),
            _ => None,
        }
    }
    ///TXQ Interrupt flag is not set
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Txq1isf::_0
    }
    ///TXQ Interrupt flag is set
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Txq1isf::_1
    }
}
impl R {
    ///Bits 0:3 - TXQ Interrupt Status Flag for Channel 0
    #[inline(always)]
    pub fn txq0isf(&self) -> Txq0isfR {
        Txq0isfR::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - TXQ Interrupt Status Flag for Channel 1
    #[inline(always)]
    pub fn txq1isf(&self) -> Txq1isfR {
        Txq1isfR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDTXQISTS")
            .field("txq0isf", &self.txq0isf())
            .field("txq1isf", &self.txq1isf())
            .finish()
    }
}
impl W {}
/**TX Queue Interrupt Status Register

You can [`read`](crate::Reg::read) this register and get [`cfdtxqists::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtxqists::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdtxqistsSpec;
impl crate::RegisterSpec for CfdtxqistsSpec {
    type Ux = u32;
}
///`read()` method returns [`cfdtxqists::R`](R) reader structure
impl crate::Readable for CfdtxqistsSpec {}
///`write(|w| ..)` method takes [`cfdtxqists::W`](W) writer structure
impl crate::Writable for CfdtxqistsSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDTXQISTS to value 0
impl crate::Resettable for CfdtxqistsSpec {}
