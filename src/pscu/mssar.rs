///Register `MSSAR` reader
pub type R = crate::R<MssarSpec>;
///Register `MSSAR` writer
pub type W = crate::W<MssarSpec>;
/**The MSTPCRC.MSTPC14 bit security attribution

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mssar0 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Mssar0> for bool {
    #[inline(always)]
    fn from(variant: Mssar0) -> Self {
        variant as u8 != 0
    }
}
///Field `MSSAR0` reader - The MSTPCRC.MSTPC14 bit security attribution
pub type Mssar0R = crate::BitReader<Mssar0>;
impl Mssar0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mssar0 {
        match self.bits {
            false => Mssar0::_0,
            true => Mssar0::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mssar0::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mssar0::_1
    }
}
///Field `MSSAR0` writer - The MSTPCRC.MSTPC14 bit security attribution
pub type Mssar0W<'a, REG> = crate::BitWriter<'a, REG, Mssar0>;
impl<'a, REG> Mssar0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mssar0::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mssar0::_1)
    }
}
/**The MSTPCRA.MSTPA22 bit security attribution

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mssar1 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Mssar1> for bool {
    #[inline(always)]
    fn from(variant: Mssar1) -> Self {
        variant as u8 != 0
    }
}
///Field `MSSAR1` reader - The MSTPCRA.MSTPA22 bit security attribution
pub type Mssar1R = crate::BitReader<Mssar1>;
impl Mssar1R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mssar1 {
        match self.bits {
            false => Mssar1::_0,
            true => Mssar1::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mssar1::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mssar1::_1
    }
}
///Field `MSSAR1` writer - The MSTPCRA.MSTPA22 bit security attribution
pub type Mssar1W<'a, REG> = crate::BitWriter<'a, REG, Mssar1>;
impl<'a, REG> Mssar1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mssar1::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mssar1::_1)
    }
}
/**The MSTPCRA.MSTPA7 bit security attribution

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mssar2 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Mssar2> for bool {
    #[inline(always)]
    fn from(variant: Mssar2) -> Self {
        variant as u8 != 0
    }
}
///Field `MSSAR2` reader - The MSTPCRA.MSTPA7 bit security attribution
pub type Mssar2R = crate::BitReader<Mssar2>;
impl Mssar2R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mssar2 {
        match self.bits {
            false => Mssar2::_0,
            true => Mssar2::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mssar2::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mssar2::_1
    }
}
///Field `MSSAR2` writer - The MSTPCRA.MSTPA7 bit security attribution
pub type Mssar2W<'a, REG> = crate::BitWriter<'a, REG, Mssar2>;
impl<'a, REG> Mssar2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mssar2::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mssar2::_1)
    }
}
/**The MSTPCRA.MSTPA0 bit security attribution

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mssar3 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Mssar3> for bool {
    #[inline(always)]
    fn from(variant: Mssar3) -> Self {
        variant as u8 != 0
    }
}
///Field `MSSAR3` reader - The MSTPCRA.MSTPA0 bit security attribution
pub type Mssar3R = crate::BitReader<Mssar3>;
impl Mssar3R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mssar3 {
        match self.bits {
            false => Mssar3::_0,
            true => Mssar3::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mssar3::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mssar3::_1
    }
}
///Field `MSSAR3` writer - The MSTPCRA.MSTPA0 bit security attribution
pub type Mssar3W<'a, REG> = crate::BitWriter<'a, REG, Mssar3>;
impl<'a, REG> Mssar3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mssar3::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mssar3::_1)
    }
}
impl R {
    ///Bit 0 - The MSTPCRC.MSTPC14 bit security attribution
    #[inline(always)]
    pub fn mssar0(&self) -> Mssar0R {
        Mssar0R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - The MSTPCRA.MSTPA22 bit security attribution
    #[inline(always)]
    pub fn mssar1(&self) -> Mssar1R {
        Mssar1R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - The MSTPCRA.MSTPA7 bit security attribution
    #[inline(always)]
    pub fn mssar2(&self) -> Mssar2R {
        Mssar2R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - The MSTPCRA.MSTPA0 bit security attribution
    #[inline(always)]
    pub fn mssar3(&self) -> Mssar3R {
        Mssar3R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MSSAR")
            .field("mssar0", &self.mssar0())
            .field("mssar1", &self.mssar1())
            .field("mssar2", &self.mssar2())
            .field("mssar3", &self.mssar3())
            .finish()
    }
}
impl W {
    ///Bit 0 - The MSTPCRC.MSTPC14 bit security attribution
    #[inline(always)]
    pub fn mssar0(&mut self) -> Mssar0W<MssarSpec> {
        Mssar0W::new(self, 0)
    }
    ///Bit 1 - The MSTPCRA.MSTPA22 bit security attribution
    #[inline(always)]
    pub fn mssar1(&mut self) -> Mssar1W<MssarSpec> {
        Mssar1W::new(self, 1)
    }
    ///Bit 2 - The MSTPCRA.MSTPA7 bit security attribution
    #[inline(always)]
    pub fn mssar2(&mut self) -> Mssar2W<MssarSpec> {
        Mssar2W::new(self, 2)
    }
    ///Bit 3 - The MSTPCRA.MSTPA0 bit security attribution
    #[inline(always)]
    pub fn mssar3(&mut self) -> Mssar3W<MssarSpec> {
        Mssar3W::new(self, 3)
    }
}
/**Module Stop Security Attribution Register

You can [`read`](crate::Reg::read) this register and get [`mssar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mssar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MssarSpec;
impl crate::RegisterSpec for MssarSpec {
    type Ux = u32;
}
///`read()` method returns [`mssar::R`](R) reader structure
impl crate::Readable for MssarSpec {}
///`write(|w| ..)` method takes [`mssar::W`](W) writer structure
impl crate::Writable for MssarSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MSSAR to value 0xffff_ffff
impl crate::Resettable for MssarSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
