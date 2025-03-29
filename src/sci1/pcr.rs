///Register `PCR` reader
pub type R = crate::R<PcrSpec>;
///Register `PCR` writer
pub type W = crate::W<PcrSpec>;
/**TXDXn Signal Polarity Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txdxps {
    ///0: The polarity of TXDXn signal is not inverted for output.
    _0 = 0,
    ///1: The polarity of TXDXn signal is inverted for output.
    _1 = 1,
}
impl From<Txdxps> for bool {
    #[inline(always)]
    fn from(variant: Txdxps) -> Self {
        variant as u8 != 0
    }
}
///Field `TXDXPS` reader - TXDXn Signal Polarity Select
pub type TxdxpsR = crate::BitReader<Txdxps>;
impl TxdxpsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Txdxps {
        match self.bits {
            false => Txdxps::_0,
            true => Txdxps::_1,
        }
    }
    ///The polarity of TXDXn signal is not inverted for output.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Txdxps::_0
    }
    ///The polarity of TXDXn signal is inverted for output.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Txdxps::_1
    }
}
///Field `TXDXPS` writer - TXDXn Signal Polarity Select
pub type TxdxpsW<'a, REG> = crate::BitWriter<'a, REG, Txdxps>;
impl<'a, REG> TxdxpsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The polarity of TXDXn signal is not inverted for output.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Txdxps::_0)
    }
    ///The polarity of TXDXn signal is inverted for output.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Txdxps::_1)
    }
}
/**RXDXn Signal Polarity Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxdxps {
    ///0: The polarity of RXDXn signal is not inverted for input.
    _0 = 0,
    ///1: The polarity of RXDXn signal is inverted for input.
    _1 = 1,
}
impl From<Rxdxps> for bool {
    #[inline(always)]
    fn from(variant: Rxdxps) -> Self {
        variant as u8 != 0
    }
}
///Field `RXDXPS` reader - RXDXn Signal Polarity Select
pub type RxdxpsR = crate::BitReader<Rxdxps>;
impl RxdxpsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rxdxps {
        match self.bits {
            false => Rxdxps::_0,
            true => Rxdxps::_1,
        }
    }
    ///The polarity of RXDXn signal is not inverted for input.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rxdxps::_0
    }
    ///The polarity of RXDXn signal is inverted for input.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rxdxps::_1
    }
}
///Field `RXDXPS` writer - RXDXn Signal Polarity Select
pub type RxdxpsW<'a, REG> = crate::BitWriter<'a, REG, Rxdxps>;
impl<'a, REG> RxdxpsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The polarity of RXDXn signal is not inverted for input.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rxdxps::_0)
    }
    ///The polarity of RXDXn signal is inverted for input.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxdxps::_1)
    }
}
/**TXDXn/RXDXn Pin Multiplexing Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sharps {
    ///0: The TXDXn and RXDXn pins are independent.
    _0 = 0,
    ///1: The TXDXn and RXDXn signals are multiplexed on the same pin.
    _1 = 1,
}
impl From<Sharps> for bool {
    #[inline(always)]
    fn from(variant: Sharps) -> Self {
        variant as u8 != 0
    }
}
///Field `SHARPS` reader - TXDXn/RXDXn Pin Multiplexing Select
pub type SharpsR = crate::BitReader<Sharps>;
impl SharpsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sharps {
        match self.bits {
            false => Sharps::_0,
            true => Sharps::_1,
        }
    }
    ///The TXDXn and RXDXn pins are independent.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sharps::_0
    }
    ///The TXDXn and RXDXn signals are multiplexed on the same pin.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sharps::_1
    }
}
///Field `SHARPS` writer - TXDXn/RXDXn Pin Multiplexing Select
pub type SharpsW<'a, REG> = crate::BitWriter<'a, REG, Sharps>;
impl<'a, REG> SharpsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The TXDXn and RXDXn pins are independent.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sharps::_0)
    }
    ///The TXDXn and RXDXn signals are multiplexed on the same pin.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sharps::_1)
    }
}
impl R {
    ///Bit 0 - TXDXn Signal Polarity Select
    #[inline(always)]
    pub fn txdxps(&self) -> TxdxpsR {
        TxdxpsR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - RXDXn Signal Polarity Select
    #[inline(always)]
    pub fn rxdxps(&self) -> RxdxpsR {
        RxdxpsR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - TXDXn/RXDXn Pin Multiplexing Select
    #[inline(always)]
    pub fn sharps(&self) -> SharpsR {
        SharpsR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCR")
            .field("txdxps", &self.txdxps())
            .field("rxdxps", &self.rxdxps())
            .field("sharps", &self.sharps())
            .finish()
    }
}
impl W {
    ///Bit 0 - TXDXn Signal Polarity Select
    #[inline(always)]
    pub fn txdxps(&mut self) -> TxdxpsW<PcrSpec> {
        TxdxpsW::new(self, 0)
    }
    ///Bit 1 - RXDXn Signal Polarity Select
    #[inline(always)]
    pub fn rxdxps(&mut self) -> RxdxpsW<PcrSpec> {
        RxdxpsW::new(self, 1)
    }
    ///Bit 4 - TXDXn/RXDXn Pin Multiplexing Select
    #[inline(always)]
    pub fn sharps(&mut self) -> SharpsW<PcrSpec> {
        SharpsW::new(self, 4)
    }
}
/**Port Control Register

You can [`read`](crate::Reg::read) this register and get [`pcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PcrSpec;
impl crate::RegisterSpec for PcrSpec {
    type Ux = u8;
}
///`read()` method returns [`pcr::R`](R) reader structure
impl crate::Readable for PcrSpec {}
///`write(|w| ..)` method takes [`pcr::W`](W) writer structure
impl crate::Writable for PcrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PCR to value 0
impl crate::Resettable for PcrSpec {}
