///Register `SRAMPRCR2` reader
pub type R = crate::R<Sramprcr2Spec>;
///Register `SRAMPRCR2` writer
pub type W = crate::W<Sramprcr2Spec>;
/**Register Write Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sramprcr2 {
    ///0: Disable writes to the protectedregisters
    _0 = 0,
    ///1: Enable writes to the protected registers
    _1 = 1,
}
impl From<Sramprcr2> for bool {
    #[inline(always)]
    fn from(variant: Sramprcr2) -> Self {
        variant as u8 != 0
    }
}
///Field `SRAMPRCR2` reader - Register Write Control
pub type Sramprcr2R = crate::BitReader<Sramprcr2>;
impl Sramprcr2R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sramprcr2 {
        match self.bits {
            false => Sramprcr2::_0,
            true => Sramprcr2::_1,
        }
    }
    ///Disable writes to the protectedregisters
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sramprcr2::_0
    }
    ///Enable writes to the protected registers
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sramprcr2::_1
    }
}
///Field `SRAMPRCR2` writer - Register Write Control
pub type Sramprcr2W<'a, REG> = crate::BitWriter<'a, REG, Sramprcr2>;
impl<'a, REG> Sramprcr2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable writes to the protectedregisters
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sramprcr2::_0)
    }
    ///Enable writes to the protected registers
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sramprcr2::_1)
    }
}
///Field `KW` writer - Write Key Code
pub type KwW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bit 0 - Register Write Control
    #[inline(always)]
    pub fn sramprcr2(&self) -> Sramprcr2R {
        Sramprcr2R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRAMPRCR2").field("sramprcr2", &self.sramprcr2()).finish()
    }
}
impl W {
    ///Bit 0 - Register Write Control
    #[inline(always)]
    pub fn sramprcr2(&mut self) -> Sramprcr2W<Sramprcr2Spec> {
        Sramprcr2W::new(self, 0)
    }
    ///Bits 1:7 - Write Key Code
    #[inline(always)]
    pub fn kw(&mut self) -> KwW<Sramprcr2Spec> {
        KwW::new(self, 1)
    }
}
/**SRAM Protection Register 2

You can [`read`](crate::Reg::read) this register and get [`sramprcr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sramprcr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Sramprcr2Spec;
impl crate::RegisterSpec for Sramprcr2Spec {
    type Ux = u8;
}
///`read()` method returns [`sramprcr2::R`](R) reader structure
impl crate::Readable for Sramprcr2Spec {}
///`write(|w| ..)` method takes [`sramprcr2::W`](W) writer structure
impl crate::Writable for Sramprcr2Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SRAMPRCR2 to value 0
impl crate::Resettable for Sramprcr2Spec {}
