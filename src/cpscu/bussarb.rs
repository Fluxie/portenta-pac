///Register `BUSSARB` reader
pub type R = crate::R<BussarbSpec>;
///Register `BUSSARB` writer
pub type W = crate::W<BussarbSpec>;
/**BUS Security Attribution B0

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bussb0 {
    ///0: Secure
    _0 = 0,
    ///1: Non-Secure
    _1 = 1,
}
impl From<Bussb0> for bool {
    #[inline(always)]
    fn from(variant: Bussb0) -> Self {
        variant as u8 != 0
    }
}
///Field `BUSSB0` reader - BUS Security Attribution B0
pub type Bussb0R = crate::BitReader<Bussb0>;
impl Bussb0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Bussb0 {
        match self.bits {
            false => Bussb0::_0,
            true => Bussb0::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bussb0::_0
    }
    ///Non-Secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bussb0::_1
    }
}
///Field `BUSSB0` writer - BUS Security Attribution B0
pub type Bussb0W<'a, REG> = crate::BitWriter<'a, REG, Bussb0>;
impl<'a, REG> Bussb0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bussb0::_0)
    }
    ///Non-Secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bussb0::_1)
    }
}
impl R {
    ///Bit 0 - BUS Security Attribution B0
    #[inline(always)]
    pub fn bussb0(&self) -> Bussb0R {
        Bussb0R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BUSSARB").field("bussb0", &self.bussb0()).finish()
    }
}
impl W {
    ///Bit 0 - BUS Security Attribution B0
    #[inline(always)]
    pub fn bussb0(&mut self) -> Bussb0W<BussarbSpec> {
        Bussb0W::new(self, 0)
    }
}
/**BUS Security Attribution Register B

You can [`read`](crate::Reg::read) this register and get [`bussarb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bussarb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BussarbSpec;
impl crate::RegisterSpec for BussarbSpec {
    type Ux = u32;
}
///`read()` method returns [`bussarb::R`](R) reader structure
impl crate::Readable for BussarbSpec {}
///`write(|w| ..)` method takes [`bussarb::W`](W) writer structure
impl crate::Writable for BussarbSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BUSSARB to value 0xffff_ffff
impl crate::Resettable for BussarbSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
