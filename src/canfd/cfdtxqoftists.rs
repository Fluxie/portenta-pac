///Register `CFDTXQOFTISTS` reader
pub type R = crate::R<CfdtxqoftistsSpec>;
///Register `CFDTXQOFTISTS` writer
pub type W = crate::W<CfdtxqoftistsSpec>;
/**TXQ One Frame TX Interrupt Status Flag for Channel 0

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Txq0oftisf {
    ///0: TXQ One Frame TX Interrupt flag is not set
    _0 = 0,
    ///1: TXQ One Frame TX Interrupt flag is set
    _1 = 1,
}
impl From<Txq0oftisf> for u8 {
    #[inline(always)]
    fn from(variant: Txq0oftisf) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Txq0oftisf {
    type Ux = u8;
}
impl crate::IsEnum for Txq0oftisf {}
///Field `TXQ0OFTISF` reader - TXQ One Frame TX Interrupt Status Flag for Channel 0
pub type Txq0oftisfR = crate::FieldReader<Txq0oftisf>;
impl Txq0oftisfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<Txq0oftisf> {
        match self.bits {
            0 => Some(Txq0oftisf::_0),
            1 => Some(Txq0oftisf::_1),
            _ => None,
        }
    }
    ///TXQ One Frame TX Interrupt flag is not set
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Txq0oftisf::_0
    }
    ///TXQ One Frame TX Interrupt flag is set
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Txq0oftisf::_1
    }
}
/**TXQ One Frame TX Interrupt Status Flag for Channel 1

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Txq1oftisf {
    ///0: TXQ One Frame TX Interrupt flag is not set
    _0 = 0,
    ///1: TXQ One Frame TX Interrupt flag is set
    _1 = 1,
}
impl From<Txq1oftisf> for u8 {
    #[inline(always)]
    fn from(variant: Txq1oftisf) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Txq1oftisf {
    type Ux = u8;
}
impl crate::IsEnum for Txq1oftisf {}
///Field `TXQ1OFTISF` reader - TXQ One Frame TX Interrupt Status Flag for Channel 1
pub type Txq1oftisfR = crate::FieldReader<Txq1oftisf>;
impl Txq1oftisfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<Txq1oftisf> {
        match self.bits {
            0 => Some(Txq1oftisf::_0),
            1 => Some(Txq1oftisf::_1),
            _ => None,
        }
    }
    ///TXQ One Frame TX Interrupt flag is not set
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Txq1oftisf::_0
    }
    ///TXQ One Frame TX Interrupt flag is set
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Txq1oftisf::_1
    }
}
impl R {
    ///Bits 0:3 - TXQ One Frame TX Interrupt Status Flag for Channel 0
    #[inline(always)]
    pub fn txq0oftisf(&self) -> Txq0oftisfR {
        Txq0oftisfR::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - TXQ One Frame TX Interrupt Status Flag for Channel 1
    #[inline(always)]
    pub fn txq1oftisf(&self) -> Txq1oftisfR {
        Txq1oftisfR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDTXQOFTISTS")
            .field("txq0oftisf", &self.txq0oftisf())
            .field("txq1oftisf", &self.txq1oftisf())
            .finish()
    }
}
impl W {}
/**TX Queue One Frame TX Interrupt Status Register

You can [`read`](crate::Reg::read) this register and get [`cfdtxqoftists::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtxqoftists::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdtxqoftistsSpec;
impl crate::RegisterSpec for CfdtxqoftistsSpec {
    type Ux = u32;
}
///`read()` method returns [`cfdtxqoftists::R`](R) reader structure
impl crate::Readable for CfdtxqoftistsSpec {}
///`write(|w| ..)` method takes [`cfdtxqoftists::W`](W) writer structure
impl crate::Writable for CfdtxqoftistsSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDTXQOFTISTS to value 0
impl crate::Resettable for CfdtxqoftistsSpec {}
