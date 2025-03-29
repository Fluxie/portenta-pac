///Register `SSLP` reader
pub type R = crate::R<SslpSpec>;
///Register `SSLP` writer
pub type W = crate::W<SslpSpec>;
/**SSLn0 Signal Polarity Setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ssl0p {
    ///0: Set SSLn0 signal to active-low
    _0 = 0,
    ///1: Set SSLn0 signal to active-high
    _1 = 1,
}
impl From<Ssl0p> for bool {
    #[inline(always)]
    fn from(variant: Ssl0p) -> Self {
        variant as u8 != 0
    }
}
///Field `SSL0P` reader - SSLn0 Signal Polarity Setting
pub type Ssl0pR = crate::BitReader<Ssl0p>;
impl Ssl0pR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ssl0p {
        match self.bits {
            false => Ssl0p::_0,
            true => Ssl0p::_1,
        }
    }
    ///Set SSLn0 signal to active-low
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ssl0p::_0
    }
    ///Set SSLn0 signal to active-high
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ssl0p::_1
    }
}
///Field `SSL0P` writer - SSLn0 Signal Polarity Setting
pub type Ssl0pW<'a, REG> = crate::BitWriter<'a, REG, Ssl0p>;
impl<'a, REG> Ssl0pW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Set SSLn0 signal to active-low
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ssl0p::_0)
    }
    ///Set SSLn0 signal to active-high
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ssl0p::_1)
    }
}
/**SSLn1 Signal Polarity Setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ssl1p {
    ///0: Set SSLn1 signal to active-low
    _0 = 0,
    ///1: Set SSLn1 signal to active-high
    _1 = 1,
}
impl From<Ssl1p> for bool {
    #[inline(always)]
    fn from(variant: Ssl1p) -> Self {
        variant as u8 != 0
    }
}
///Field `SSL1P` reader - SSLn1 Signal Polarity Setting
pub type Ssl1pR = crate::BitReader<Ssl1p>;
impl Ssl1pR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ssl1p {
        match self.bits {
            false => Ssl1p::_0,
            true => Ssl1p::_1,
        }
    }
    ///Set SSLn1 signal to active-low
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ssl1p::_0
    }
    ///Set SSLn1 signal to active-high
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ssl1p::_1
    }
}
///Field `SSL1P` writer - SSLn1 Signal Polarity Setting
pub type Ssl1pW<'a, REG> = crate::BitWriter<'a, REG, Ssl1p>;
impl<'a, REG> Ssl1pW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Set SSLn1 signal to active-low
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ssl1p::_0)
    }
    ///Set SSLn1 signal to active-high
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ssl1p::_1)
    }
}
/**SSLn2 Signal Polarity Setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ssl2p {
    ///0: Set SSLn2 signal to active-low
    _0 = 0,
    ///1: Set SSLn2 signal to active-high
    _1 = 1,
}
impl From<Ssl2p> for bool {
    #[inline(always)]
    fn from(variant: Ssl2p) -> Self {
        variant as u8 != 0
    }
}
///Field `SSL2P` reader - SSLn2 Signal Polarity Setting
pub type Ssl2pR = crate::BitReader<Ssl2p>;
impl Ssl2pR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ssl2p {
        match self.bits {
            false => Ssl2p::_0,
            true => Ssl2p::_1,
        }
    }
    ///Set SSLn2 signal to active-low
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ssl2p::_0
    }
    ///Set SSLn2 signal to active-high
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ssl2p::_1
    }
}
///Field `SSL2P` writer - SSLn2 Signal Polarity Setting
pub type Ssl2pW<'a, REG> = crate::BitWriter<'a, REG, Ssl2p>;
impl<'a, REG> Ssl2pW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Set SSLn2 signal to active-low
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ssl2p::_0)
    }
    ///Set SSLn2 signal to active-high
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ssl2p::_1)
    }
}
/**SSLn3 Signal Polarity Setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ssl3p {
    ///0: Set SSLn3 signal to active-low
    _0 = 0,
    ///1: Set SSLn3 signal to active-high
    _1 = 1,
}
impl From<Ssl3p> for bool {
    #[inline(always)]
    fn from(variant: Ssl3p) -> Self {
        variant as u8 != 0
    }
}
///Field `SSL3P` reader - SSLn3 Signal Polarity Setting
pub type Ssl3pR = crate::BitReader<Ssl3p>;
impl Ssl3pR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ssl3p {
        match self.bits {
            false => Ssl3p::_0,
            true => Ssl3p::_1,
        }
    }
    ///Set SSLn3 signal to active-low
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ssl3p::_0
    }
    ///Set SSLn3 signal to active-high
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ssl3p::_1
    }
}
///Field `SSL3P` writer - SSLn3 Signal Polarity Setting
pub type Ssl3pW<'a, REG> = crate::BitWriter<'a, REG, Ssl3p>;
impl<'a, REG> Ssl3pW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Set SSLn3 signal to active-low
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ssl3p::_0)
    }
    ///Set SSLn3 signal to active-high
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ssl3p::_1)
    }
}
impl R {
    ///Bit 0 - SSLn0 Signal Polarity Setting
    #[inline(always)]
    pub fn ssl0p(&self) -> Ssl0pR {
        Ssl0pR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SSLn1 Signal Polarity Setting
    #[inline(always)]
    pub fn ssl1p(&self) -> Ssl1pR {
        Ssl1pR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - SSLn2 Signal Polarity Setting
    #[inline(always)]
    pub fn ssl2p(&self) -> Ssl2pR {
        Ssl2pR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - SSLn3 Signal Polarity Setting
    #[inline(always)]
    pub fn ssl3p(&self) -> Ssl3pR {
        Ssl3pR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SSLP")
            .field("ssl0p", &self.ssl0p())
            .field("ssl1p", &self.ssl1p())
            .field("ssl2p", &self.ssl2p())
            .field("ssl3p", &self.ssl3p())
            .finish()
    }
}
impl W {
    ///Bit 0 - SSLn0 Signal Polarity Setting
    #[inline(always)]
    pub fn ssl0p(&mut self) -> Ssl0pW<SslpSpec> {
        Ssl0pW::new(self, 0)
    }
    ///Bit 1 - SSLn1 Signal Polarity Setting
    #[inline(always)]
    pub fn ssl1p(&mut self) -> Ssl1pW<SslpSpec> {
        Ssl1pW::new(self, 1)
    }
    ///Bit 2 - SSLn2 Signal Polarity Setting
    #[inline(always)]
    pub fn ssl2p(&mut self) -> Ssl2pW<SslpSpec> {
        Ssl2pW::new(self, 2)
    }
    ///Bit 3 - SSLn3 Signal Polarity Setting
    #[inline(always)]
    pub fn ssl3p(&mut self) -> Ssl3pW<SslpSpec> {
        Ssl3pW::new(self, 3)
    }
}
/**SPI Slave Select Polarity Register

You can [`read`](crate::Reg::read) this register and get [`sslp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sslp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SslpSpec;
impl crate::RegisterSpec for SslpSpec {
    type Ux = u8;
}
///`read()` method returns [`sslp::R`](R) reader structure
impl crate::Readable for SslpSpec {}
///`write(|w| ..)` method takes [`sslp::W`](W) writer structure
impl crate::Writable for SslpSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SSLP to value 0
impl crate::Resettable for SslpSpec {}
