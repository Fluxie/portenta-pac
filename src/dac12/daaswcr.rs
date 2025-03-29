///Register `DAASWCR` reader
pub type R = crate::R<DaaswcrSpec>;
///Register `DAASWCR` writer
pub type W = crate::W<DaaswcrSpec>;
/**D/A Amplifier Stabilization Wait 0

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Daasw0 {
    ///0: Amplifier stabilization wait off (output) for channel 0
    _0 = 0,
    ///1: Amplifier stabilization wait on (high-Z) for channel 0
    _1 = 1,
}
impl From<Daasw0> for bool {
    #[inline(always)]
    fn from(variant: Daasw0) -> Self {
        variant as u8 != 0
    }
}
///Field `DAASW0` reader - D/A Amplifier Stabilization Wait 0
pub type Daasw0R = crate::BitReader<Daasw0>;
impl Daasw0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Daasw0 {
        match self.bits {
            false => Daasw0::_0,
            true => Daasw0::_1,
        }
    }
    ///Amplifier stabilization wait off (output) for channel 0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Daasw0::_0
    }
    ///Amplifier stabilization wait on (high-Z) for channel 0
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Daasw0::_1
    }
}
///Field `DAASW0` writer - D/A Amplifier Stabilization Wait 0
pub type Daasw0W<'a, REG> = crate::BitWriter<'a, REG, Daasw0>;
impl<'a, REG> Daasw0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Amplifier stabilization wait off (output) for channel 0
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Daasw0::_0)
    }
    ///Amplifier stabilization wait on (high-Z) for channel 0
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Daasw0::_1)
    }
}
/**D/A Amplifier Stabilization Wait 1

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Daasw1 {
    ///0: Amplifier stabilization wait off (output) for channel 1
    _0 = 0,
    ///1: Amplifier stabilization wait on (high-Z) for channel 1
    _1 = 1,
}
impl From<Daasw1> for bool {
    #[inline(always)]
    fn from(variant: Daasw1) -> Self {
        variant as u8 != 0
    }
}
///Field `DAASW1` reader - D/A Amplifier Stabilization Wait 1
pub type Daasw1R = crate::BitReader<Daasw1>;
impl Daasw1R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Daasw1 {
        match self.bits {
            false => Daasw1::_0,
            true => Daasw1::_1,
        }
    }
    ///Amplifier stabilization wait off (output) for channel 1
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Daasw1::_0
    }
    ///Amplifier stabilization wait on (high-Z) for channel 1
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Daasw1::_1
    }
}
///Field `DAASW1` writer - D/A Amplifier Stabilization Wait 1
pub type Daasw1W<'a, REG> = crate::BitWriter<'a, REG, Daasw1>;
impl<'a, REG> Daasw1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Amplifier stabilization wait off (output) for channel 1
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Daasw1::_0)
    }
    ///Amplifier stabilization wait on (high-Z) for channel 1
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Daasw1::_1)
    }
}
impl R {
    ///Bit 6 - D/A Amplifier Stabilization Wait 0
    #[inline(always)]
    pub fn daasw0(&self) -> Daasw0R {
        Daasw0R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - D/A Amplifier Stabilization Wait 1
    #[inline(always)]
    pub fn daasw1(&self) -> Daasw1R {
        Daasw1R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAASWCR")
            .field("daasw0", &self.daasw0())
            .field("daasw1", &self.daasw1())
            .finish()
    }
}
impl W {
    ///Bit 6 - D/A Amplifier Stabilization Wait 0
    #[inline(always)]
    pub fn daasw0(&mut self) -> Daasw0W<DaaswcrSpec> {
        Daasw0W::new(self, 6)
    }
    ///Bit 7 - D/A Amplifier Stabilization Wait 1
    #[inline(always)]
    pub fn daasw1(&mut self) -> Daasw1W<DaaswcrSpec> {
        Daasw1W::new(self, 7)
    }
}
/**D/A Amplifier Stabilization Wait Control Register

You can [`read`](crate::Reg::read) this register and get [`daaswcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daaswcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DaaswcrSpec;
impl crate::RegisterSpec for DaaswcrSpec {
    type Ux = u8;
}
///`read()` method returns [`daaswcr::R`](R) reader structure
impl crate::Readable for DaaswcrSpec {}
///`write(|w| ..)` method takes [`daaswcr::W`](W) writer structure
impl crate::Writable for DaaswcrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DAASWCR to value 0
impl crate::Resettable for DaaswcrSpec {}
