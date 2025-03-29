///Register `SPTR` reader
pub type R = crate::R<SptrSpec>;
///Register `SPTR` writer
pub type W = crate::W<SptrSpec>;
///Field `RXDMON` reader - Serial Input Data Monitor
pub type RxdmonR = crate::BitReader;
///Field `SPB2DT` reader - Serial Port Break Data Select
pub type Spb2dtR = crate::BitReader;
///Field `SPB2DT` writer - Serial Port Break Data Select
pub type Spb2dtW<'a, REG> = crate::BitWriter<'a, REG>;
/**Serial Port Break I/O

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spb2io {
    ///0: Do not output value of SPB2DT bit on TXDn pin
    _0 = 0,
    ///1: Output value of SPB2DT bit on TXDn pin
    _1 = 1,
}
impl From<Spb2io> for bool {
    #[inline(always)]
    fn from(variant: Spb2io) -> Self {
        variant as u8 != 0
    }
}
///Field `SPB2IO` reader - Serial Port Break I/O
pub type Spb2ioR = crate::BitReader<Spb2io>;
impl Spb2ioR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Spb2io {
        match self.bits {
            false => Spb2io::_0,
            true => Spb2io::_1,
        }
    }
    ///Do not output value of SPB2DT bit on TXDn pin
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Spb2io::_0
    }
    ///Output value of SPB2DT bit on TXDn pin
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Spb2io::_1
    }
}
///Field `SPB2IO` writer - Serial Port Break I/O
pub type Spb2ioW<'a, REG> = crate::BitWriter<'a, REG, Spb2io>;
impl<'a, REG> Spb2ioW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not output value of SPB2DT bit on TXDn pin
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Spb2io::_0)
    }
    ///Output value of SPB2DT bit on TXDn pin
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Spb2io::_1)
    }
}
/**RXD invert bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rinv {
    ///0: Received data from RXDn is not inverted and input.
    _0 = 0,
    ///1: Received data from RXDn is inverted and input.
    _1 = 1,
}
impl From<Rinv> for bool {
    #[inline(always)]
    fn from(variant: Rinv) -> Self {
        variant as u8 != 0
    }
}
///Field `RINV` reader - RXD invert bit
pub type RinvR = crate::BitReader<Rinv>;
impl RinvR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rinv {
        match self.bits {
            false => Rinv::_0,
            true => Rinv::_1,
        }
    }
    ///Received data from RXDn is not inverted and input.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rinv::_0
    }
    ///Received data from RXDn is inverted and input.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rinv::_1
    }
}
///Field `RINV` writer - RXD invert bit
pub type RinvW<'a, REG> = crate::BitWriter<'a, REG, Rinv>;
impl<'a, REG> RinvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Received data from RXDn is not inverted and input.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rinv::_0)
    }
    ///Received data from RXDn is inverted and input.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rinv::_1)
    }
}
/**TXD invert bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tinv {
    ///0: Transmit data is not inverted and output to TXDn.
    _0 = 0,
    ///1: Transmit data is inverted and output to TXDn.
    _1 = 1,
}
impl From<Tinv> for bool {
    #[inline(always)]
    fn from(variant: Tinv) -> Self {
        variant as u8 != 0
    }
}
///Field `TINV` reader - TXD invert bit
pub type TinvR = crate::BitReader<Tinv>;
impl TinvR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tinv {
        match self.bits {
            false => Tinv::_0,
            true => Tinv::_1,
        }
    }
    ///Transmit data is not inverted and output to TXDn.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tinv::_0
    }
    ///Transmit data is inverted and output to TXDn.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tinv::_1
    }
}
///Field `TINV` writer - TXD invert bit
pub type TinvW<'a, REG> = crate::BitWriter<'a, REG, Tinv>;
impl<'a, REG> TinvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Transmit data is not inverted and output to TXDn.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tinv::_0)
    }
    ///Transmit data is inverted and output to TXDn.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tinv::_1)
    }
}
/**Adjust receive sampling timing enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Asen {
    ///0: Adjust sampling timing disable.
    _0 = 0,
    ///1: Adjust sampling timing enable.
    _1 = 1,
}
impl From<Asen> for bool {
    #[inline(always)]
    fn from(variant: Asen) -> Self {
        variant as u8 != 0
    }
}
///Field `ASEN` reader - Adjust receive sampling timing enable
pub type AsenR = crate::BitReader<Asen>;
impl AsenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Asen {
        match self.bits {
            false => Asen::_0,
            true => Asen::_1,
        }
    }
    ///Adjust sampling timing disable.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Asen::_0
    }
    ///Adjust sampling timing enable.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Asen::_1
    }
}
///Field `ASEN` writer - Adjust receive sampling timing enable
pub type AsenW<'a, REG> = crate::BitWriter<'a, REG, Asen>;
impl<'a, REG> AsenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Adjust sampling timing disable.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Asen::_0)
    }
    ///Adjust sampling timing enable.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Asen::_1)
    }
}
/**Adjust transmit timing enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aten {
    ///0: Adjust transmit timing disable.
    _0 = 0,
    ///1: Adjust transmit timing enable.
    _1 = 1,
}
impl From<Aten> for bool {
    #[inline(always)]
    fn from(variant: Aten) -> Self {
        variant as u8 != 0
    }
}
///Field `ATEN` reader - Adjust transmit timing enable
pub type AtenR = crate::BitReader<Aten>;
impl AtenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Aten {
        match self.bits {
            false => Aten::_0,
            true => Aten::_1,
        }
    }
    ///Adjust transmit timing disable.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Aten::_0
    }
    ///Adjust transmit timing enable.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Aten::_1
    }
}
///Field `ATEN` writer - Adjust transmit timing enable
pub type AtenW<'a, REG> = crate::BitWriter<'a, REG, Aten>;
impl<'a, REG> AtenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Adjust transmit timing disable.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Aten::_0)
    }
    ///Adjust transmit timing enable.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Aten::_1)
    }
}
impl R {
    ///Bit 0 - Serial Input Data Monitor
    #[inline(always)]
    pub fn rxdmon(&self) -> RxdmonR {
        RxdmonR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Serial Port Break Data Select
    #[inline(always)]
    pub fn spb2dt(&self) -> Spb2dtR {
        Spb2dtR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Serial Port Break I/O
    #[inline(always)]
    pub fn spb2io(&self) -> Spb2ioR {
        Spb2ioR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - RXD invert bit
    #[inline(always)]
    pub fn rinv(&self) -> RinvR {
        RinvR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TXD invert bit
    #[inline(always)]
    pub fn tinv(&self) -> TinvR {
        TinvR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Adjust receive sampling timing enable
    #[inline(always)]
    pub fn asen(&self) -> AsenR {
        AsenR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Adjust transmit timing enable
    #[inline(always)]
    pub fn aten(&self) -> AtenR {
        AtenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPTR")
            .field("rxdmon", &self.rxdmon())
            .field("spb2dt", &self.spb2dt())
            .field("spb2io", &self.spb2io())
            .field("rinv", &self.rinv())
            .field("tinv", &self.tinv())
            .field("asen", &self.asen())
            .field("aten", &self.aten())
            .finish()
    }
}
impl W {
    ///Bit 1 - Serial Port Break Data Select
    #[inline(always)]
    pub fn spb2dt(&mut self) -> Spb2dtW<SptrSpec> {
        Spb2dtW::new(self, 1)
    }
    ///Bit 2 - Serial Port Break I/O
    #[inline(always)]
    pub fn spb2io(&mut self) -> Spb2ioW<SptrSpec> {
        Spb2ioW::new(self, 2)
    }
    ///Bit 4 - RXD invert bit
    #[inline(always)]
    pub fn rinv(&mut self) -> RinvW<SptrSpec> {
        RinvW::new(self, 4)
    }
    ///Bit 5 - TXD invert bit
    #[inline(always)]
    pub fn tinv(&mut self) -> TinvW<SptrSpec> {
        TinvW::new(self, 5)
    }
    ///Bit 6 - Adjust receive sampling timing enable
    #[inline(always)]
    pub fn asen(&mut self) -> AsenW<SptrSpec> {
        AsenW::new(self, 6)
    }
    ///Bit 7 - Adjust transmit timing enable
    #[inline(always)]
    pub fn aten(&mut self) -> AtenW<SptrSpec> {
        AtenW::new(self, 7)
    }
}
/**Serial Port Register

You can [`read`](crate::Reg::read) this register and get [`sptr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sptr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SptrSpec;
impl crate::RegisterSpec for SptrSpec {
    type Ux = u8;
}
///`read()` method returns [`sptr::R`](R) reader structure
impl crate::Readable for SptrSpec {}
///`write(|w| ..)` method takes [`sptr::W`](W) writer structure
impl crate::Writable for SptrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SPTR to value 0x03
impl crate::Resettable for SptrSpec {
    const RESET_VALUE: u8 = 0x03;
}
