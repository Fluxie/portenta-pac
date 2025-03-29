///Register `ICUSARD` reader
pub type R = crate::R<IcusardSpec>;
///Register `ICUSARD` writer
pub type W = crate::W<IcusardSpec>;
/**Security attributes of registers for SELSR0

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saselsr0 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Saselsr0> for bool {
    #[inline(always)]
    fn from(variant: Saselsr0) -> Self {
        variant as u8 != 0
    }
}
///Field `SASELSR0` reader - Security attributes of registers for SELSR0
pub type Saselsr0R = crate::BitReader<Saselsr0>;
impl Saselsr0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Saselsr0 {
        match self.bits {
            false => Saselsr0::_0,
            true => Saselsr0::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Saselsr0::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Saselsr0::_1
    }
}
///Field `SASELSR0` writer - Security attributes of registers for SELSR0
pub type Saselsr0W<'a, REG> = crate::BitWriter<'a, REG, Saselsr0>;
impl<'a, REG> Saselsr0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Saselsr0::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Saselsr0::_1)
    }
}
impl R {
    ///Bit 0 - Security attributes of registers for SELSR0
    #[inline(always)]
    pub fn saselsr0(&self) -> Saselsr0R {
        Saselsr0R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICUSARD").field("saselsr0", &self.saselsr0()).finish()
    }
}
impl W {
    ///Bit 0 - Security attributes of registers for SELSR0
    #[inline(always)]
    pub fn saselsr0(&mut self) -> Saselsr0W<IcusardSpec> {
        Saselsr0W::new(self, 0)
    }
}
/**Interrupt Controller Unit Security Attribution Register D

You can [`read`](crate::Reg::read) this register and get [`icusard::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icusard::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IcusardSpec;
impl crate::RegisterSpec for IcusardSpec {
    type Ux = u32;
}
///`read()` method returns [`icusard::R`](R) reader structure
impl crate::Readable for IcusardSpec {}
///`write(|w| ..)` method takes [`icusard::W`](W) writer structure
impl crate::Writable for IcusardSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICUSARD to value 0xffff_ffff
impl crate::Resettable for IcusardSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
