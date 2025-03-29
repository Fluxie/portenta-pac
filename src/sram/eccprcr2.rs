///Register `ECCPRCR2` reader
pub type R = crate::R<Eccprcr2Spec>;
///Register `ECCPRCR2` writer
pub type W = crate::W<Eccprcr2Spec>;
/**Register Write Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eccprcr2 {
    ///0: Disable writes to the protected registers
    _0 = 0,
    ///1: Enable writes to the protected registers
    _1 = 1,
}
impl From<Eccprcr2> for bool {
    #[inline(always)]
    fn from(variant: Eccprcr2) -> Self {
        variant as u8 != 0
    }
}
///Field `ECCPRCR2` reader - Register Write Control
pub type Eccprcr2R = crate::BitReader<Eccprcr2>;
impl Eccprcr2R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eccprcr2 {
        match self.bits {
            false => Eccprcr2::_0,
            true => Eccprcr2::_1,
        }
    }
    ///Disable writes to the protected registers
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eccprcr2::_0
    }
    ///Enable writes to the protected registers
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eccprcr2::_1
    }
}
///Field `ECCPRCR2` writer - Register Write Control
pub type Eccprcr2W<'a, REG> = crate::BitWriter<'a, REG, Eccprcr2>;
impl<'a, REG> Eccprcr2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable writes to the protected registers
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eccprcr2::_0)
    }
    ///Enable writes to the protected registers
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eccprcr2::_1)
    }
}
/**Write Key Code

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Kw2 {
    ///120: Enable write to the ECCPRCR2 bit
    _0x78 = 120,
    ///0: Disable write to the ECCPRCR2 bit
    Others = 0,
}
impl From<Kw2> for u8 {
    #[inline(always)]
    fn from(variant: Kw2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Kw2 {
    type Ux = u8;
}
impl crate::IsEnum for Kw2 {}
///Field `KW2` writer - Write Key Code
pub type Kw2W<'a, REG> = crate::FieldWriter<'a, REG, 7, Kw2, crate::Safe>;
impl<'a, REG> Kw2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Enable write to the ECCPRCR2 bit
    #[inline(always)]
    pub fn _0x78(self) -> &'a mut crate::W<REG> {
        self.variant(Kw2::_0x78)
    }
    ///Disable write to the ECCPRCR2 bit
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Kw2::Others)
    }
}
impl R {
    ///Bit 0 - Register Write Control
    #[inline(always)]
    pub fn eccprcr2(&self) -> Eccprcr2R {
        Eccprcr2R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECCPRCR2").field("eccprcr2", &self.eccprcr2()).finish()
    }
}
impl W {
    ///Bit 0 - Register Write Control
    #[inline(always)]
    pub fn eccprcr2(&mut self) -> Eccprcr2W<Eccprcr2Spec> {
        Eccprcr2W::new(self, 0)
    }
    ///Bits 1:7 - Write Key Code
    #[inline(always)]
    pub fn kw2(&mut self) -> Kw2W<Eccprcr2Spec> {
        Kw2W::new(self, 1)
    }
}
/**ECC Protection Register 2

You can [`read`](crate::Reg::read) this register and get [`eccprcr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eccprcr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Eccprcr2Spec;
impl crate::RegisterSpec for Eccprcr2Spec {
    type Ux = u8;
}
///`read()` method returns [`eccprcr2::R`](R) reader structure
impl crate::Readable for Eccprcr2Spec {}
///`write(|w| ..)` method takes [`eccprcr2::W`](W) writer structure
impl crate::Writable for Eccprcr2Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ECCPRCR2 to value 0
impl crate::Resettable for Eccprcr2Spec {}
