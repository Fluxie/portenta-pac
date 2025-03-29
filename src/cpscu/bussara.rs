///Register `BUSSARA` reader
pub type R = crate::R<BussaraSpec>;
///Register `BUSSARA` writer
pub type W = crate::W<BussaraSpec>;
/**BUS Security Attribution A0

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bussa0 {
    ///0: Secure
    _0 = 0,
    ///1: Non-Secure
    _1 = 1,
}
impl From<Bussa0> for bool {
    #[inline(always)]
    fn from(variant: Bussa0) -> Self {
        variant as u8 != 0
    }
}
///Field `BUSSA0` reader - BUS Security Attribution A0
pub type Bussa0R = crate::BitReader<Bussa0>;
impl Bussa0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Bussa0 {
        match self.bits {
            false => Bussa0::_0,
            true => Bussa0::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bussa0::_0
    }
    ///Non-Secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bussa0::_1
    }
}
///Field `BUSSA0` writer - BUS Security Attribution A0
pub type Bussa0W<'a, REG> = crate::BitWriter<'a, REG, Bussa0>;
impl<'a, REG> Bussa0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bussa0::_0)
    }
    ///Non-Secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bussa0::_1)
    }
}
impl R {
    ///Bit 0 - BUS Security Attribution A0
    #[inline(always)]
    pub fn bussa0(&self) -> Bussa0R {
        Bussa0R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BUSSARA").field("bussa0", &self.bussa0()).finish()
    }
}
impl W {
    ///Bit 0 - BUS Security Attribution A0
    #[inline(always)]
    pub fn bussa0(&mut self) -> Bussa0W<BussaraSpec> {
        Bussa0W::new(self, 0)
    }
}
/**BUS Security Attribution Register A

You can [`read`](crate::Reg::read) this register and get [`bussara::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bussara::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BussaraSpec;
impl crate::RegisterSpec for BussaraSpec {
    type Ux = u32;
}
///`read()` method returns [`bussara::R`](R) reader structure
impl crate::Readable for BussaraSpec {}
///`write(|w| ..)` method takes [`bussara::W`](W) writer structure
impl crate::Writable for BussaraSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BUSSARA to value 0xffff_ffff
impl crate::Resettable for BussaraSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
