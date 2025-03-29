///Register `SCMR` reader
pub type R = crate::R<ScmrSpec>;
///Register `SCMR` writer
pub type W = crate::W<ScmrSpec>;
/**Smart Card Interface Mode Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smif {
    ///0: Non-smart card interface mode (asynchronous mode, clock synchronous mode, simple SPI mode, or simple IIC mode)
    _0 = 0,
    ///1: Smart card interface mode
    _1 = 1,
}
impl From<Smif> for bool {
    #[inline(always)]
    fn from(variant: Smif) -> Self {
        variant as u8 != 0
    }
}
///Field `SMIF` reader - Smart Card Interface Mode Select
pub type SmifR = crate::BitReader<Smif>;
impl SmifR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Smif {
        match self.bits {
            false => Smif::_0,
            true => Smif::_1,
        }
    }
    ///Non-smart card interface mode (asynchronous mode, clock synchronous mode, simple SPI mode, or simple IIC mode)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Smif::_0
    }
    ///Smart card interface mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Smif::_1
    }
}
///Field `SMIF` writer - Smart Card Interface Mode Select
pub type SmifW<'a, REG> = crate::BitWriter<'a, REG, Smif>;
impl<'a, REG> SmifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Non-smart card interface mode (asynchronous mode, clock synchronous mode, simple SPI mode, or simple IIC mode)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Smif::_0)
    }
    ///Smart card interface mode
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Smif::_1)
    }
}
/**Transmitted/Received Data Invert

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sinv {
    ///0: TDR contents are transmitted as they are. Received data is stored as received in the RDR register.
    _0 = 0,
    ///1: TDR register contents are inverted before transmission. Receive data is stored in inverted form in the RDR register.
    _1 = 1,
}
impl From<Sinv> for bool {
    #[inline(always)]
    fn from(variant: Sinv) -> Self {
        variant as u8 != 0
    }
}
///Field `SINV` reader - Transmitted/Received Data Invert
pub type SinvR = crate::BitReader<Sinv>;
impl SinvR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sinv {
        match self.bits {
            false => Sinv::_0,
            true => Sinv::_1,
        }
    }
    ///TDR contents are transmitted as they are. Received data is stored as received in the RDR register.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sinv::_0
    }
    ///TDR register contents are inverted before transmission. Receive data is stored in inverted form in the RDR register.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sinv::_1
    }
}
///Field `SINV` writer - Transmitted/Received Data Invert
pub type SinvW<'a, REG> = crate::BitWriter<'a, REG, Sinv>;
impl<'a, REG> SinvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///TDR contents are transmitted as they are. Received data is stored as received in the RDR register.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sinv::_0)
    }
    ///TDR register contents are inverted before transmission. Receive data is stored in inverted form in the RDR register.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sinv::_1)
    }
}
/**Transmitted/Received Data Transfer Direction

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdir {
    ///0: Transfer LSB-first
    _0 = 0,
    ///1: Transfer MSB-first
    _1 = 1,
}
impl From<Sdir> for bool {
    #[inline(always)]
    fn from(variant: Sdir) -> Self {
        variant as u8 != 0
    }
}
///Field `SDIR` reader - Transmitted/Received Data Transfer Direction
pub type SdirR = crate::BitReader<Sdir>;
impl SdirR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sdir {
        match self.bits {
            false => Sdir::_0,
            true => Sdir::_1,
        }
    }
    ///Transfer LSB-first
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sdir::_0
    }
    ///Transfer MSB-first
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sdir::_1
    }
}
///Field `SDIR` writer - Transmitted/Received Data Transfer Direction
pub type SdirW<'a, REG> = crate::BitWriter<'a, REG, Sdir>;
impl<'a, REG> SdirW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Transfer LSB-first
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sdir::_0)
    }
    ///Transfer MSB-first
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sdir::_1)
    }
}
/**Character Length 1

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chr1 {
    ///0: SMR.CHR = 0: Transmit/receive in 9-bit data length SMR.CHR = 1: Transmit/receive in 9-bit data length
    _0 = 0,
    ///1: SMR.CHR = 0: Transmit/receive in 8-bit data length (initial value) SMR.CHR = 1: Transmit/receive in 7-bit data length
    _1 = 1,
}
impl From<Chr1> for bool {
    #[inline(always)]
    fn from(variant: Chr1) -> Self {
        variant as u8 != 0
    }
}
///Field `CHR1` reader - Character Length 1
pub type Chr1R = crate::BitReader<Chr1>;
impl Chr1R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Chr1 {
        match self.bits {
            false => Chr1::_0,
            true => Chr1::_1,
        }
    }
    ///SMR.CHR = 0: Transmit/receive in 9-bit data length SMR.CHR = 1: Transmit/receive in 9-bit data length
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Chr1::_0
    }
    ///SMR.CHR = 0: Transmit/receive in 8-bit data length (initial value) SMR.CHR = 1: Transmit/receive in 7-bit data length
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Chr1::_1
    }
}
///Field `CHR1` writer - Character Length 1
pub type Chr1W<'a, REG> = crate::BitWriter<'a, REG, Chr1>;
impl<'a, REG> Chr1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SMR.CHR = 0: Transmit/receive in 9-bit data length SMR.CHR = 1: Transmit/receive in 9-bit data length
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Chr1::_0)
    }
    ///SMR.CHR = 0: Transmit/receive in 8-bit data length (initial value) SMR.CHR = 1: Transmit/receive in 7-bit data length
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Chr1::_1)
    }
}
///Field `BCP2` reader - Base Clock Pulse 2
pub type Bcp2R = crate::BitReader;
///Field `BCP2` writer - Base Clock Pulse 2
pub type Bcp2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Smart Card Interface Mode Select
    #[inline(always)]
    pub fn smif(&self) -> SmifR {
        SmifR::new((self.bits & 1) != 0)
    }
    ///Bit 2 - Transmitted/Received Data Invert
    #[inline(always)]
    pub fn sinv(&self) -> SinvR {
        SinvR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Transmitted/Received Data Transfer Direction
    #[inline(always)]
    pub fn sdir(&self) -> SdirR {
        SdirR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Character Length 1
    #[inline(always)]
    pub fn chr1(&self) -> Chr1R {
        Chr1R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 7 - Base Clock Pulse 2
    #[inline(always)]
    pub fn bcp2(&self) -> Bcp2R {
        Bcp2R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCMR")
            .field("smif", &self.smif())
            .field("sinv", &self.sinv())
            .field("sdir", &self.sdir())
            .field("chr1", &self.chr1())
            .field("bcp2", &self.bcp2())
            .finish()
    }
}
impl W {
    ///Bit 0 - Smart Card Interface Mode Select
    #[inline(always)]
    pub fn smif(&mut self) -> SmifW<ScmrSpec> {
        SmifW::new(self, 0)
    }
    ///Bit 2 - Transmitted/Received Data Invert
    #[inline(always)]
    pub fn sinv(&mut self) -> SinvW<ScmrSpec> {
        SinvW::new(self, 2)
    }
    ///Bit 3 - Transmitted/Received Data Transfer Direction
    #[inline(always)]
    pub fn sdir(&mut self) -> SdirW<ScmrSpec> {
        SdirW::new(self, 3)
    }
    ///Bit 4 - Character Length 1
    #[inline(always)]
    pub fn chr1(&mut self) -> Chr1W<ScmrSpec> {
        Chr1W::new(self, 4)
    }
    ///Bit 7 - Base Clock Pulse 2
    #[inline(always)]
    pub fn bcp2(&mut self) -> Bcp2W<ScmrSpec> {
        Bcp2W::new(self, 7)
    }
}
/**Smart Card Mode Register

You can [`read`](crate::Reg::read) this register and get [`scmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ScmrSpec;
impl crate::RegisterSpec for ScmrSpec {
    type Ux = u8;
}
///`read()` method returns [`scmr::R`](R) reader structure
impl crate::Readable for ScmrSpec {}
///`write(|w| ..)` method takes [`scmr::W`](W) writer structure
impl crate::Writable for ScmrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SCMR to value 0xf2
impl crate::Resettable for ScmrSpec {
    const RESET_VALUE: u8 = 0xf2;
}
