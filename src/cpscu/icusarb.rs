///Register `ICUSARB` reader
pub type R = crate::R<IcusarbSpec>;
///Register `ICUSARB` writer
pub type W = crate::W<IcusarbSpec>;
/**Security attributes of registers for nonmaskable interrupt

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sanmi {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Sanmi> for bool {
    #[inline(always)]
    fn from(variant: Sanmi) -> Self {
        variant as u8 != 0
    }
}
///Field `SANMI` reader - Security attributes of registers for nonmaskable interrupt
pub type SanmiR = crate::BitReader<Sanmi>;
impl SanmiR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sanmi {
        match self.bits {
            false => Sanmi::_0,
            true => Sanmi::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sanmi::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sanmi::_1
    }
}
///Field `SANMI` writer - Security attributes of registers for nonmaskable interrupt
pub type SanmiW<'a, REG> = crate::BitWriter<'a, REG, Sanmi>;
impl<'a, REG> SanmiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sanmi::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sanmi::_1)
    }
}
impl R {
    ///Bit 0 - Security attributes of registers for nonmaskable interrupt
    #[inline(always)]
    pub fn sanmi(&self) -> SanmiR {
        SanmiR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICUSARB").field("sanmi", &self.sanmi()).finish()
    }
}
impl W {
    ///Bit 0 - Security attributes of registers for nonmaskable interrupt
    #[inline(always)]
    pub fn sanmi(&mut self) -> SanmiW<IcusarbSpec> {
        SanmiW::new(self, 0)
    }
}
/**Interrupt Controller Unit Security Attribution Register B

You can [`read`](crate::Reg::read) this register and get [`icusarb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icusarb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IcusarbSpec;
impl crate::RegisterSpec for IcusarbSpec {
    type Ux = u32;
}
///`read()` method returns [`icusarb::R`](R) reader structure
impl crate::Readable for IcusarbSpec {}
///`write(|w| ..)` method takes [`icusarb::W`](W) writer structure
impl crate::Writable for IcusarbSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICUSARB to value 0xffff_ffff
impl crate::Resettable for IcusarbSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
