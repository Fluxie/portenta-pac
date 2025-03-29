///Register `DAAMPCR` reader
pub type R = crate::R<DaampcrSpec>;
///Register `DAAMPCR` writer
pub type W = crate::W<DaampcrSpec>;
/**Amplifier Control 0

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Daamp0 {
    ///0: Do not use channel 0 output amplifier
    _0 = 0,
    ///1: Use channel 0 output amplifier
    _1 = 1,
}
impl From<Daamp0> for bool {
    #[inline(always)]
    fn from(variant: Daamp0) -> Self {
        variant as u8 != 0
    }
}
///Field `DAAMP0` reader - Amplifier Control 0
pub type Daamp0R = crate::BitReader<Daamp0>;
impl Daamp0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Daamp0 {
        match self.bits {
            false => Daamp0::_0,
            true => Daamp0::_1,
        }
    }
    ///Do not use channel 0 output amplifier
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Daamp0::_0
    }
    ///Use channel 0 output amplifier
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Daamp0::_1
    }
}
///Field `DAAMP0` writer - Amplifier Control 0
pub type Daamp0W<'a, REG> = crate::BitWriter<'a, REG, Daamp0>;
impl<'a, REG> Daamp0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not use channel 0 output amplifier
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Daamp0::_0)
    }
    ///Use channel 0 output amplifier
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Daamp0::_1)
    }
}
/**Amplifier Control 1

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Daamp1 {
    ///0: Do not use channel 1 output amplifier
    _0 = 0,
    ///1: Use channel 1 output amplifier
    _1 = 1,
}
impl From<Daamp1> for bool {
    #[inline(always)]
    fn from(variant: Daamp1) -> Self {
        variant as u8 != 0
    }
}
///Field `DAAMP1` reader - Amplifier Control 1
pub type Daamp1R = crate::BitReader<Daamp1>;
impl Daamp1R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Daamp1 {
        match self.bits {
            false => Daamp1::_0,
            true => Daamp1::_1,
        }
    }
    ///Do not use channel 1 output amplifier
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Daamp1::_0
    }
    ///Use channel 1 output amplifier
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Daamp1::_1
    }
}
///Field `DAAMP1` writer - Amplifier Control 1
pub type Daamp1W<'a, REG> = crate::BitWriter<'a, REG, Daamp1>;
impl<'a, REG> Daamp1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not use channel 1 output amplifier
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Daamp1::_0)
    }
    ///Use channel 1 output amplifier
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Daamp1::_1)
    }
}
impl R {
    ///Bit 6 - Amplifier Control 0
    #[inline(always)]
    pub fn daamp0(&self) -> Daamp0R {
        Daamp0R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Amplifier Control 1
    #[inline(always)]
    pub fn daamp1(&self) -> Daamp1R {
        Daamp1R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAAMPCR")
            .field("daamp0", &self.daamp0())
            .field("daamp1", &self.daamp1())
            .finish()
    }
}
impl W {
    ///Bit 6 - Amplifier Control 0
    #[inline(always)]
    pub fn daamp0(&mut self) -> Daamp0W<DaampcrSpec> {
        Daamp0W::new(self, 6)
    }
    ///Bit 7 - Amplifier Control 1
    #[inline(always)]
    pub fn daamp1(&mut self) -> Daamp1W<DaampcrSpec> {
        Daamp1W::new(self, 7)
    }
}
/**D/A Output Amplifier Control Register

You can [`read`](crate::Reg::read) this register and get [`daampcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daampcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DaampcrSpec;
impl crate::RegisterSpec for DaampcrSpec {
    type Ux = u8;
}
///`read()` method returns [`daampcr::R`](R) reader structure
impl crate::Readable for DaampcrSpec {}
///`write(|w| ..)` method takes [`daampcr::W`](W) writer structure
impl crate::Writable for DaampcrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DAAMPCR to value 0
impl crate::Resettable for DaampcrSpec {}
