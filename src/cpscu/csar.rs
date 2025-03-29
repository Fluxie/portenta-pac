///Register `CSAR` reader
pub type R = crate::R<CsarSpec>;
///Register `CSAR` writer
pub type W = crate::W<CsarSpec>;
/**Security Attributes of Registers for Cache Control

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cachesa {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Cachesa> for bool {
    #[inline(always)]
    fn from(variant: Cachesa) -> Self {
        variant as u8 != 0
    }
}
///Field `CACHESA` reader - Security Attributes of Registers for Cache Control
pub type CachesaR = crate::BitReader<Cachesa>;
impl CachesaR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cachesa {
        match self.bits {
            false => Cachesa::_0,
            true => Cachesa::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cachesa::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cachesa::_1
    }
}
///Field `CACHESA` writer - Security Attributes of Registers for Cache Control
pub type CachesaW<'a, REG> = crate::BitWriter<'a, REG, Cachesa>;
impl<'a, REG> CachesaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cachesa::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cachesa::_1)
    }
}
/**Security Attributes of Registers for Cache Line Configuration

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cachelsa {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Cachelsa> for bool {
    #[inline(always)]
    fn from(variant: Cachelsa) -> Self {
        variant as u8 != 0
    }
}
///Field `CACHELSA` reader - Security Attributes of Registers for Cache Line Configuration
pub type CachelsaR = crate::BitReader<Cachelsa>;
impl CachelsaR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cachelsa {
        match self.bits {
            false => Cachelsa::_0,
            true => Cachelsa::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cachelsa::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cachelsa::_1
    }
}
///Field `CACHELSA` writer - Security Attributes of Registers for Cache Line Configuration
pub type CachelsaW<'a, REG> = crate::BitWriter<'a, REG, Cachelsa>;
impl<'a, REG> CachelsaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cachelsa::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cachelsa::_1)
    }
}
/**Security Attributes of Registers for Cache Error

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cacheesa {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Cacheesa> for bool {
    #[inline(always)]
    fn from(variant: Cacheesa) -> Self {
        variant as u8 != 0
    }
}
///Field `CACHEESA` reader - Security Attributes of Registers for Cache Error
pub type CacheesaR = crate::BitReader<Cacheesa>;
impl CacheesaR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cacheesa {
        match self.bits {
            false => Cacheesa::_0,
            true => Cacheesa::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cacheesa::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cacheesa::_1
    }
}
///Field `CACHEESA` writer - Security Attributes of Registers for Cache Error
pub type CacheesaW<'a, REG> = crate::BitWriter<'a, REG, Cacheesa>;
impl<'a, REG> CacheesaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cacheesa::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cacheesa::_1)
    }
}
impl R {
    ///Bit 0 - Security Attributes of Registers for Cache Control
    #[inline(always)]
    pub fn cachesa(&self) -> CachesaR {
        CachesaR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Security Attributes of Registers for Cache Line Configuration
    #[inline(always)]
    pub fn cachelsa(&self) -> CachelsaR {
        CachelsaR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Security Attributes of Registers for Cache Error
    #[inline(always)]
    pub fn cacheesa(&self) -> CacheesaR {
        CacheesaR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSAR")
            .field("cachesa", &self.cachesa())
            .field("cachelsa", &self.cachelsa())
            .field("cacheesa", &self.cacheesa())
            .finish()
    }
}
impl W {
    ///Bit 0 - Security Attributes of Registers for Cache Control
    #[inline(always)]
    pub fn cachesa(&mut self) -> CachesaW<CsarSpec> {
        CachesaW::new(self, 0)
    }
    ///Bit 1 - Security Attributes of Registers for Cache Line Configuration
    #[inline(always)]
    pub fn cachelsa(&mut self) -> CachelsaW<CsarSpec> {
        CachelsaW::new(self, 1)
    }
    ///Bit 2 - Security Attributes of Registers for Cache Error
    #[inline(always)]
    pub fn cacheesa(&mut self) -> CacheesaW<CsarSpec> {
        CacheesaW::new(self, 2)
    }
}
/**Cache Security Attribution Register

You can [`read`](crate::Reg::read) this register and get [`csar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CsarSpec;
impl crate::RegisterSpec for CsarSpec {
    type Ux = u32;
}
///`read()` method returns [`csar::R`](R) reader structure
impl crate::Readable for CsarSpec {}
///`write(|w| ..)` method takes [`csar::W`](W) writer structure
impl crate::Writable for CsarSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSAR to value 0xffff_ffff
impl crate::Resettable for CsarSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
