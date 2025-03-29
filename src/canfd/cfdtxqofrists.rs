///Register `CFDTXQOFRISTS` reader
pub type R = crate::R<CfdtxqofristsSpec>;
///Register `CFDTXQOFRISTS` writer
pub type W = crate::W<CfdtxqofristsSpec>;
/**TXQ One Frame RX Interrupt Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Txq0ofrisf {
    ///0: TXQ One Frame RX Interrupt flag is not set
    _0 = 0,
    ///1: TXQ One Frame RX Interrupt flag is set
    _1 = 1,
}
impl From<Txq0ofrisf> for u8 {
    #[inline(always)]
    fn from(variant: Txq0ofrisf) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Txq0ofrisf {
    type Ux = u8;
}
impl crate::IsEnum for Txq0ofrisf {}
///Field `TXQ0OFRISF` reader - TXQ One Frame RX Interrupt Status Flag
pub type Txq0ofrisfR = crate::FieldReader<Txq0ofrisf>;
impl Txq0ofrisfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<Txq0ofrisf> {
        match self.bits {
            0 => Some(Txq0ofrisf::_0),
            1 => Some(Txq0ofrisf::_1),
            _ => None,
        }
    }
    ///TXQ One Frame RX Interrupt flag is not set
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Txq0ofrisf::_0
    }
    ///TXQ One Frame RX Interrupt flag is set
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Txq0ofrisf::_1
    }
}
/**TXQ One Frame RX Interrupt Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Txq1ofrisf {
    ///0: TXQ One Frame RX Interrupt flag is not set
    _0 = 0,
    ///1: TXQ One Frame RX Interrupt flag is set
    _1 = 1,
}
impl From<Txq1ofrisf> for u8 {
    #[inline(always)]
    fn from(variant: Txq1ofrisf) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Txq1ofrisf {
    type Ux = u8;
}
impl crate::IsEnum for Txq1ofrisf {}
///Field `TXQ1OFRISF` reader - TXQ One Frame RX Interrupt Status Flag
pub type Txq1ofrisfR = crate::FieldReader<Txq1ofrisf>;
impl Txq1ofrisfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<Txq1ofrisf> {
        match self.bits {
            0 => Some(Txq1ofrisf::_0),
            1 => Some(Txq1ofrisf::_1),
            _ => None,
        }
    }
    ///TXQ One Frame RX Interrupt flag is not set
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Txq1ofrisf::_0
    }
    ///TXQ One Frame RX Interrupt flag is set
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Txq1ofrisf::_1
    }
}
impl R {
    ///Bits 0:2 - TXQ One Frame RX Interrupt Status Flag
    #[inline(always)]
    pub fn txq0ofrisf(&self) -> Txq0ofrisfR {
        Txq0ofrisfR::new((self.bits & 7) as u8)
    }
    ///Bits 4:6 - TXQ One Frame RX Interrupt Status Flag
    #[inline(always)]
    pub fn txq1ofrisf(&self) -> Txq1ofrisfR {
        Txq1ofrisfR::new(((self.bits >> 4) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDTXQOFRISTS")
            .field("txq0ofrisf", &self.txq0ofrisf())
            .field("txq1ofrisf", &self.txq1ofrisf())
            .finish()
    }
}
impl W {}
/**TX Queue One Frame RX Interrupt Status Register

You can [`read`](crate::Reg::read) this register and get [`cfdtxqofrists::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdtxqofrists::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdtxqofristsSpec;
impl crate::RegisterSpec for CfdtxqofristsSpec {
    type Ux = u32;
}
///`read()` method returns [`cfdtxqofrists::R`](R) reader structure
impl crate::Readable for CfdtxqofristsSpec {}
///`write(|w| ..)` method takes [`cfdtxqofrists::W`](W) writer structure
impl crate::Writable for CfdtxqofristsSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDTXQOFRISTS to value 0
impl crate::Resettable for CfdtxqofristsSpec {}
