///Register `MMPUSARB` reader
pub type R = crate::R<MmpusarbSpec>;
///Register `MMPUSARB` writer
pub type W = crate::W<MmpusarbSpec>;
/**MMPUB Security Attribution

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mmpubsa0 {
    ///0: Secure
    _0 = 0,
    ///1: Non-Secure
    _1 = 1,
}
impl From<Mmpubsa0> for bool {
    #[inline(always)]
    fn from(variant: Mmpubsa0) -> Self {
        variant as u8 != 0
    }
}
///Field `MMPUBSA0` reader - MMPUB Security Attribution
pub type Mmpubsa0R = crate::BitReader<Mmpubsa0>;
impl Mmpubsa0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mmpubsa0 {
        match self.bits {
            false => Mmpubsa0::_0,
            true => Mmpubsa0::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mmpubsa0::_0
    }
    ///Non-Secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mmpubsa0::_1
    }
}
///Field `MMPUBSA0` writer - MMPUB Security Attribution
pub type Mmpubsa0W<'a, REG> = crate::BitWriter<'a, REG, Mmpubsa0>;
impl<'a, REG> Mmpubsa0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mmpubsa0::_0)
    }
    ///Non-Secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mmpubsa0::_1)
    }
}
impl R {
    ///Bit 0 - MMPUB Security Attribution
    #[inline(always)]
    pub fn mmpubsa0(&self) -> Mmpubsa0R {
        Mmpubsa0R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMPUSARB").field("mmpubsa0", &self.mmpubsa0()).finish()
    }
}
impl W {
    ///Bit 0 - MMPUB Security Attribution
    #[inline(always)]
    pub fn mmpubsa0(&mut self) -> Mmpubsa0W<MmpusarbSpec> {
        Mmpubsa0W::new(self, 0)
    }
}
/**Master Memory Protection Unit Security Attribution Register B

You can [`read`](crate::Reg::read) this register and get [`mmpusarb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmpusarb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MmpusarbSpec;
impl crate::RegisterSpec for MmpusarbSpec {
    type Ux = u32;
}
///`read()` method returns [`mmpusarb::R`](R) reader structure
impl crate::Readable for MmpusarbSpec {}
///`write(|w| ..)` method takes [`mmpusarb::W`](W) writer structure
impl crate::Writable for MmpusarbSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MMPUSARB to value 0xffff_ffff
impl crate::Resettable for MmpusarbSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
