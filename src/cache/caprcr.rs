///Register `CAPRCR` reader
pub type R = crate::R<CaprcrSpec>;
///Register `CAPRCR` writer
pub type W = crate::W<CaprcrSpec>;
/**Register Write Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prcr {
    ///0: Disable writes to protected registers
    _0 = 0,
    ///1: Enable writes to protected registers
    _1 = 1,
}
impl From<Prcr> for bool {
    #[inline(always)]
    fn from(variant: Prcr) -> Self {
        variant as u8 != 0
    }
}
///Field `PRCR` reader - Register Write Control
pub type PrcrR = crate::BitReader<Prcr>;
impl PrcrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Prcr {
        match self.bits {
            false => Prcr::_0,
            true => Prcr::_1,
        }
    }
    ///Disable writes to protected registers
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Prcr::_0
    }
    ///Enable writes to protected registers
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Prcr::_1
    }
}
///Field `PRCR` writer - Register Write Control
pub type PrcrW<'a, REG> = crate::BitWriter<'a, REG, Prcr>;
impl<'a, REG> PrcrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable writes to protected registers
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Prcr::_0)
    }
    ///Enable writes to protected registers
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Prcr::_1)
    }
}
///Field `KW` reader - Write key code
pub type KwR = crate::FieldReader;
///Field `KW` writer - Write key code
pub type KwW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bit 0 - Register Write Control
    #[inline(always)]
    pub fn prcr(&self) -> PrcrR {
        PrcrR::new((self.bits & 1) != 0)
    }
    ///Bits 1:7 - Write key code
    #[inline(always)]
    pub fn kw(&self) -> KwR {
        KwR::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAPRCR")
            .field("prcr", &self.prcr())
            .field("kw", &self.kw())
            .finish()
    }
}
impl W {
    ///Bit 0 - Register Write Control
    #[inline(always)]
    pub fn prcr(&mut self) -> PrcrW<CaprcrSpec> {
        PrcrW::new(self, 0)
    }
    ///Bits 1:7 - Write key code
    #[inline(always)]
    pub fn kw(&mut self) -> KwW<CaprcrSpec> {
        KwW::new(self, 1)
    }
}
/**Cache Protection Register

You can [`read`](crate::Reg::read) this register and get [`caprcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`caprcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CaprcrSpec;
impl crate::RegisterSpec for CaprcrSpec {
    type Ux = u32;
}
///`read()` method returns [`caprcr::R`](R) reader structure
impl crate::Readable for CaprcrSpec {}
///`write(|w| ..)` method takes [`caprcr::W`](W) writer structure
impl crate::Writable for CaprcrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CAPRCR to value 0
impl crate::Resettable for CaprcrSpec {}
