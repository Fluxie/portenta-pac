///Register `SPDCR2` reader
pub type R = crate::R<Spdcr2Spec>;
///Register `SPDCR2` writer
pub type W = crate::W<Spdcr2Spec>;
/**Byte Swap Operating Mode Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bysw {
    ///0: Byte Swap OFF
    _0 = 0,
    ///1: Byte Swap ON
    _1 = 1,
}
impl From<Bysw> for bool {
    #[inline(always)]
    fn from(variant: Bysw) -> Self {
        variant as u8 != 0
    }
}
///Field `BYSW` reader - Byte Swap Operating Mode Select
pub type ByswR = crate::BitReader<Bysw>;
impl ByswR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Bysw {
        match self.bits {
            false => Bysw::_0,
            true => Bysw::_1,
        }
    }
    ///Byte Swap OFF
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bysw::_0
    }
    ///Byte Swap ON
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bysw::_1
    }
}
///Field `BYSW` writer - Byte Swap Operating Mode Select
pub type ByswW<'a, REG> = crate::BitWriter<'a, REG, Bysw>;
impl<'a, REG> ByswW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Byte Swap OFF
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bysw::_0)
    }
    ///Byte Swap ON
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bysw::_1)
    }
}
/**Serial Data Invert Bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sinv {
    ///0: Not invert serial data
    _0 = 0,
    ///1: Invert serial data
    _1 = 1,
}
impl From<Sinv> for bool {
    #[inline(always)]
    fn from(variant: Sinv) -> Self {
        variant as u8 != 0
    }
}
///Field `SINV` reader - Serial Data Invert Bit
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
    ///Not invert serial data
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sinv::_0
    }
    ///Invert serial data
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sinv::_1
    }
}
///Field `SINV` writer - Serial Data Invert Bit
pub type SinvW<'a, REG> = crate::BitWriter<'a, REG, Sinv>;
impl<'a, REG> SinvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Not invert serial data
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sinv::_0)
    }
    ///Invert serial data
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sinv::_1)
    }
}
impl R {
    ///Bit 0 - Byte Swap Operating Mode Select
    #[inline(always)]
    pub fn bysw(&self) -> ByswR {
        ByswR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Serial Data Invert Bit
    #[inline(always)]
    pub fn sinv(&self) -> SinvR {
        SinvR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPDCR2")
            .field("bysw", &self.bysw())
            .field("sinv", &self.sinv())
            .finish()
    }
}
impl W {
    ///Bit 0 - Byte Swap Operating Mode Select
    #[inline(always)]
    pub fn bysw(&mut self) -> ByswW<Spdcr2Spec> {
        ByswW::new(self, 0)
    }
    ///Bit 1 - Serial Data Invert Bit
    #[inline(always)]
    pub fn sinv(&mut self) -> SinvW<Spdcr2Spec> {
        SinvW::new(self, 1)
    }
}
/**SPI Data Control Register 2

You can [`read`](crate::Reg::read) this register and get [`spdcr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spdcr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Spdcr2Spec;
impl crate::RegisterSpec for Spdcr2Spec {
    type Ux = u8;
}
///`read()` method returns [`spdcr2::R`](R) reader structure
impl crate::Readable for Spdcr2Spec {}
///`write(|w| ..)` method takes [`spdcr2::W`](W) writer structure
impl crate::Writable for Spdcr2Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SPDCR2 to value 0
impl crate::Resettable for Spdcr2Spec {}
