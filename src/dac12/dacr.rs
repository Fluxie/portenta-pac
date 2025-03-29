///Register `DACR` reader
pub type R = crate::R<DacrSpec>;
///Register `DACR` writer
pub type W = crate::W<DacrSpec>;
/**D/A Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dae {
    ///0: Control D/A conversion of channels 0 and 1 individually
    _0 = 0,
    ///1: Control D/A conversion of channels 0 and 1 collectively
    _1 = 1,
}
impl From<Dae> for bool {
    #[inline(always)]
    fn from(variant: Dae) -> Self {
        variant as u8 != 0
    }
}
///Field `DAE` reader - D/A Enable
pub type DaeR = crate::BitReader<Dae>;
impl DaeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dae {
        match self.bits {
            false => Dae::_0,
            true => Dae::_1,
        }
    }
    ///Control D/A conversion of channels 0 and 1 individually
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dae::_0
    }
    ///Control D/A conversion of channels 0 and 1 collectively
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dae::_1
    }
}
///Field `DAE` writer - D/A Enable
pub type DaeW<'a, REG> = crate::BitWriter<'a, REG, Dae>;
impl<'a, REG> DaeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Control D/A conversion of channels 0 and 1 individually
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dae::_0)
    }
    ///Control D/A conversion of channels 0 and 1 collectively
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dae::_1)
    }
}
/**D/A Output Enable 0

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Daoe0 {
    ///0: Disable D/A conversion and analog output of channel 0 (DA0)
    _0 = 0,
    ///1: Enable D/A conversion and analog output of channel 0 (DA0)
    _1 = 1,
}
impl From<Daoe0> for bool {
    #[inline(always)]
    fn from(variant: Daoe0) -> Self {
        variant as u8 != 0
    }
}
///Field `DAOE0` reader - D/A Output Enable 0
pub type Daoe0R = crate::BitReader<Daoe0>;
impl Daoe0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Daoe0 {
        match self.bits {
            false => Daoe0::_0,
            true => Daoe0::_1,
        }
    }
    ///Disable D/A conversion and analog output of channel 0 (DA0)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Daoe0::_0
    }
    ///Enable D/A conversion and analog output of channel 0 (DA0)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Daoe0::_1
    }
}
///Field `DAOE0` writer - D/A Output Enable 0
pub type Daoe0W<'a, REG> = crate::BitWriter<'a, REG, Daoe0>;
impl<'a, REG> Daoe0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable D/A conversion and analog output of channel 0 (DA0)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Daoe0::_0)
    }
    ///Enable D/A conversion and analog output of channel 0 (DA0)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Daoe0::_1)
    }
}
/**D/A Output Enable 1

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Daoe1 {
    ///0: Disable D/A conversion and analog output of channel 1 (DA1)
    _0 = 0,
    ///1: Enable D/A conversion and analog output of channel 1 (DA1)
    _1 = 1,
}
impl From<Daoe1> for bool {
    #[inline(always)]
    fn from(variant: Daoe1) -> Self {
        variant as u8 != 0
    }
}
///Field `DAOE1` reader - D/A Output Enable 1
pub type Daoe1R = crate::BitReader<Daoe1>;
impl Daoe1R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Daoe1 {
        match self.bits {
            false => Daoe1::_0,
            true => Daoe1::_1,
        }
    }
    ///Disable D/A conversion and analog output of channel 1 (DA1)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Daoe1::_0
    }
    ///Enable D/A conversion and analog output of channel 1 (DA1)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Daoe1::_1
    }
}
///Field `DAOE1` writer - D/A Output Enable 1
pub type Daoe1W<'a, REG> = crate::BitWriter<'a, REG, Daoe1>;
impl<'a, REG> Daoe1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable D/A conversion and analog output of channel 1 (DA1)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Daoe1::_0)
    }
    ///Enable D/A conversion and analog output of channel 1 (DA1)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Daoe1::_1)
    }
}
impl R {
    ///Bit 5 - D/A Enable
    #[inline(always)]
    pub fn dae(&self) -> DaeR {
        DaeR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - D/A Output Enable 0
    #[inline(always)]
    pub fn daoe0(&self) -> Daoe0R {
        Daoe0R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - D/A Output Enable 1
    #[inline(always)]
    pub fn daoe1(&self) -> Daoe1R {
        Daoe1R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DACR")
            .field("dae", &self.dae())
            .field("daoe0", &self.daoe0())
            .field("daoe1", &self.daoe1())
            .finish()
    }
}
impl W {
    ///Bit 5 - D/A Enable
    #[inline(always)]
    pub fn dae(&mut self) -> DaeW<DacrSpec> {
        DaeW::new(self, 5)
    }
    ///Bit 6 - D/A Output Enable 0
    #[inline(always)]
    pub fn daoe0(&mut self) -> Daoe0W<DacrSpec> {
        Daoe0W::new(self, 6)
    }
    ///Bit 7 - D/A Output Enable 1
    #[inline(always)]
    pub fn daoe1(&mut self) -> Daoe1W<DacrSpec> {
        Daoe1W::new(self, 7)
    }
}
/**D/A Control Register

You can [`read`](crate::Reg::read) this register and get [`dacr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dacr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DacrSpec;
impl crate::RegisterSpec for DacrSpec {
    type Ux = u8;
}
///`read()` method returns [`dacr::R`](R) reader structure
impl crate::Readable for DacrSpec {}
///`write(|w| ..)` method takes [`dacr::W`](W) writer structure
impl crate::Writable for DacrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DACR to value 0x1f
impl crate::Resettable for DacrSpec {
    const RESET_VALUE: u8 = 0x1f;
}
