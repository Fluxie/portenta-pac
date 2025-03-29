///Register `LVDSAR` reader
pub type R = crate::R<LvdsarSpec>;
///Register `LVDSAR` writer
pub type W = crate::W<LvdsarSpec>;
/**Non Secure Attribute bit 0

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nonsec0 {
    ///0: Secure
    _0 = 0,
    ///1: Non Secure
    _1 = 1,
}
impl From<Nonsec0> for bool {
    #[inline(always)]
    fn from(variant: Nonsec0) -> Self {
        variant as u8 != 0
    }
}
///Field `NONSEC0` reader - Non Secure Attribute bit 0
pub type Nonsec0R = crate::BitReader<Nonsec0>;
impl Nonsec0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Nonsec0 {
        match self.bits {
            false => Nonsec0::_0,
            true => Nonsec0::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Nonsec0::_0
    }
    ///Non Secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Nonsec0::_1
    }
}
///Field `NONSEC0` writer - Non Secure Attribute bit 0
pub type Nonsec0W<'a, REG> = crate::BitWriter<'a, REG, Nonsec0>;
impl<'a, REG> Nonsec0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Nonsec0::_0)
    }
    ///Non Secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Nonsec0::_1)
    }
}
/**Non Secure Attribute bit 1

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nonsec1 {
    ///0: Secure
    _0 = 0,
    ///1: Non Secure
    _1 = 1,
}
impl From<Nonsec1> for bool {
    #[inline(always)]
    fn from(variant: Nonsec1) -> Self {
        variant as u8 != 0
    }
}
///Field `NONSEC1` reader - Non Secure Attribute bit 1
pub type Nonsec1R = crate::BitReader<Nonsec1>;
impl Nonsec1R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Nonsec1 {
        match self.bits {
            false => Nonsec1::_0,
            true => Nonsec1::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Nonsec1::_0
    }
    ///Non Secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Nonsec1::_1
    }
}
///Field `NONSEC1` writer - Non Secure Attribute bit 1
pub type Nonsec1W<'a, REG> = crate::BitWriter<'a, REG, Nonsec1>;
impl<'a, REG> Nonsec1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Nonsec1::_0)
    }
    ///Non Secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Nonsec1::_1)
    }
}
impl R {
    ///Bit 0 - Non Secure Attribute bit 0
    #[inline(always)]
    pub fn nonsec0(&self) -> Nonsec0R {
        Nonsec0R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Non Secure Attribute bit 1
    #[inline(always)]
    pub fn nonsec1(&self) -> Nonsec1R {
        Nonsec1R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LVDSAR")
            .field("nonsec0", &self.nonsec0())
            .field("nonsec1", &self.nonsec1())
            .finish()
    }
}
impl W {
    ///Bit 0 - Non Secure Attribute bit 0
    #[inline(always)]
    pub fn nonsec0(&mut self) -> Nonsec0W<LvdsarSpec> {
        Nonsec0W::new(self, 0)
    }
    ///Bit 1 - Non Secure Attribute bit 1
    #[inline(always)]
    pub fn nonsec1(&mut self) -> Nonsec1W<LvdsarSpec> {
        Nonsec1W::new(self, 1)
    }
}
/**Low Voltage Detection Security Attribution Register

You can [`read`](crate::Reg::read) this register and get [`lvdsar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvdsar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LvdsarSpec;
impl crate::RegisterSpec for LvdsarSpec {
    type Ux = u32;
}
///`read()` method returns [`lvdsar::R`](R) reader structure
impl crate::Readable for LvdsarSpec {}
///`write(|w| ..)` method takes [`lvdsar::W`](W) writer structure
impl crate::Writable for LvdsarSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LVDSAR to value 0xffff_ffff
impl crate::Resettable for LvdsarSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
