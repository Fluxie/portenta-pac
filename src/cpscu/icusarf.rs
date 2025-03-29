///Register `ICUSARF` reader
pub type R = crate::R<IcusarfSpec>;
///Register `ICUSARF` writer
pub type W = crate::W<IcusarfSpec>;
/**Security attributes of registers for WUPEN1.b0

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saagt3udwup {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saagt3udwup> for bool {
    #[inline(always)]
    fn from(variant: Saagt3udwup) -> Self {
        variant as u8 != 0
    }
}
///Field `SAAGT3UDWUP` reader - Security attributes of registers for WUPEN1.b0
pub type Saagt3udwupR = crate::BitReader<Saagt3udwup>;
impl Saagt3udwupR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saagt3udwup {
        match self.bits {
            false => Saagt3udwup::_0,
            true => Saagt3udwup::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saagt3udwup::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saagt3udwup::_1
    }
}
///Field `SAAGT3UDWUP` writer - Security attributes of registers for WUPEN1.b0
pub type Saagt3udwupW<'a, REG> = crate::BitWriter<'a, REG, Saagt3udwup>;
impl<'a, REG> Saagt3udwupW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saagt3udwup::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saagt3udwup::_1)
    }
}
/**Security attributes of registers for WUPEN1.b1

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saagt3cawup {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saagt3cawup> for bool {
    #[inline(always)]
    fn from(variant: Saagt3cawup) -> Self {
        variant as u8 != 0
    }
}
///Field `SAAGT3CAWUP` reader - Security attributes of registers for WUPEN1.b1
pub type Saagt3cawupR = crate::BitReader<Saagt3cawup>;
impl Saagt3cawupR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saagt3cawup {
        match self.bits {
            false => Saagt3cawup::_0,
            true => Saagt3cawup::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saagt3cawup::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saagt3cawup::_1
    }
}
///Field `SAAGT3CAWUP` writer - Security attributes of registers for WUPEN1.b1
pub type Saagt3cawupW<'a, REG> = crate::BitWriter<'a, REG, Saagt3cawup>;
impl<'a, REG> Saagt3cawupW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saagt3cawup::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saagt3cawup::_1)
    }
}
/**Security attributes of registers for WUPEN1.b2

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saagt3cbwup {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saagt3cbwup> for bool {
    #[inline(always)]
    fn from(variant: Saagt3cbwup) -> Self {
        variant as u8 != 0
    }
}
///Field `SAAGT3CBWUP` reader - Security attributes of registers for WUPEN1.b2
pub type Saagt3cbwupR = crate::BitReader<Saagt3cbwup>;
impl Saagt3cbwupR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saagt3cbwup {
        match self.bits {
            false => Saagt3cbwup::_0,
            true => Saagt3cbwup::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saagt3cbwup::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saagt3cbwup::_1
    }
}
///Field `SAAGT3CBWUP` writer - Security attributes of registers for WUPEN1.b2
pub type Saagt3cbwupW<'a, REG> = crate::BitWriter<'a, REG, Saagt3cbwup>;
impl<'a, REG> Saagt3cbwupW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saagt3cbwup::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saagt3cbwup::_1)
    }
}
impl R {
    ///Bit 0 - Security attributes of registers for WUPEN1.b0
    #[inline(always)]
    pub fn saagt3udwup(&self) -> Saagt3udwupR {
        Saagt3udwupR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Security attributes of registers for WUPEN1.b1
    #[inline(always)]
    pub fn saagt3cawup(&self) -> Saagt3cawupR {
        Saagt3cawupR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Security attributes of registers for WUPEN1.b2
    #[inline(always)]
    pub fn saagt3cbwup(&self) -> Saagt3cbwupR {
        Saagt3cbwupR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICUSARF")
            .field("saagt3udwup", &self.saagt3udwup())
            .field("saagt3cawup", &self.saagt3cawup())
            .field("saagt3cbwup", &self.saagt3cbwup())
            .finish()
    }
}
impl W {
    ///Bit 0 - Security attributes of registers for WUPEN1.b0
    #[inline(always)]
    pub fn saagt3udwup(&mut self) -> Saagt3udwupW<IcusarfSpec> {
        Saagt3udwupW::new(self, 0)
    }
    ///Bit 1 - Security attributes of registers for WUPEN1.b1
    #[inline(always)]
    pub fn saagt3cawup(&mut self) -> Saagt3cawupW<IcusarfSpec> {
        Saagt3cawupW::new(self, 1)
    }
    ///Bit 2 - Security attributes of registers for WUPEN1.b2
    #[inline(always)]
    pub fn saagt3cbwup(&mut self) -> Saagt3cbwupW<IcusarfSpec> {
        Saagt3cbwupW::new(self, 2)
    }
}
/**Interrupt Controller Unit Security Attribution Register F

You can [`read`](crate::Reg::read) this register and get [`icusarf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icusarf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IcusarfSpec;
impl crate::RegisterSpec for IcusarfSpec {
    type Ux = u32;
}
///`read()` method returns [`icusarf::R`](R) reader structure
impl crate::Readable for IcusarfSpec {}
///`write(|w| ..)` method takes [`icusarf::W`](W) writer structure
impl crate::Writable for IcusarfSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICUSARF to value 0xffff_ffff
impl crate::Resettable for IcusarfSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
