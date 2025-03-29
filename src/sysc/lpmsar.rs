///Register `LPMSAR` reader
pub type R = crate::R<LpmsarSpec>;
///Register `LPMSAR` writer
pub type W = crate::W<LpmsarSpec>;
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
/**Non Secure Attribute bit 2

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nonsec2 {
    ///0: Secure
    _0 = 0,
    ///1: Non Secure
    _1 = 1,
}
impl From<Nonsec2> for bool {
    #[inline(always)]
    fn from(variant: Nonsec2) -> Self {
        variant as u8 != 0
    }
}
///Field `NONSEC2` reader - Non Secure Attribute bit 2
pub type Nonsec2R = crate::BitReader<Nonsec2>;
impl Nonsec2R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Nonsec2 {
        match self.bits {
            false => Nonsec2::_0,
            true => Nonsec2::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Nonsec2::_0
    }
    ///Non Secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Nonsec2::_1
    }
}
///Field `NONSEC2` writer - Non Secure Attribute bit 2
pub type Nonsec2W<'a, REG> = crate::BitWriter<'a, REG, Nonsec2>;
impl<'a, REG> Nonsec2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Nonsec2::_0)
    }
    ///Non Secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Nonsec2::_1)
    }
}
/**Non Secure Attribute bit 4

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nonsec4 {
    ///0: Secure
    _0 = 0,
    ///1: Non Secure
    _1 = 1,
}
impl From<Nonsec4> for bool {
    #[inline(always)]
    fn from(variant: Nonsec4) -> Self {
        variant as u8 != 0
    }
}
///Field `NONSEC4` reader - Non Secure Attribute bit 4
pub type Nonsec4R = crate::BitReader<Nonsec4>;
impl Nonsec4R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Nonsec4 {
        match self.bits {
            false => Nonsec4::_0,
            true => Nonsec4::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Nonsec4::_0
    }
    ///Non Secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Nonsec4::_1
    }
}
///Field `NONSEC4` writer - Non Secure Attribute bit 4
pub type Nonsec4W<'a, REG> = crate::BitWriter<'a, REG, Nonsec4>;
impl<'a, REG> Nonsec4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Nonsec4::_0)
    }
    ///Non Secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Nonsec4::_1)
    }
}
/**Non Secure Attribute bit 8

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nonsec8 {
    ///0: Secure
    _0 = 0,
    ///1: Non Secure
    _1 = 1,
}
impl From<Nonsec8> for bool {
    #[inline(always)]
    fn from(variant: Nonsec8) -> Self {
        variant as u8 != 0
    }
}
///Field `NONSEC8` reader - Non Secure Attribute bit 8
pub type Nonsec8R = crate::BitReader<Nonsec8>;
impl Nonsec8R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Nonsec8 {
        match self.bits {
            false => Nonsec8::_0,
            true => Nonsec8::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Nonsec8::_0
    }
    ///Non Secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Nonsec8::_1
    }
}
///Field `NONSEC8` writer - Non Secure Attribute bit 8
pub type Nonsec8W<'a, REG> = crate::BitWriter<'a, REG, Nonsec8>;
impl<'a, REG> Nonsec8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Nonsec8::_0)
    }
    ///Non Secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Nonsec8::_1)
    }
}
/**Non Secure Attribute bit 9

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nonsec9 {
    ///0: Secure
    _0 = 0,
    ///1: Non Secure
    _1 = 1,
}
impl From<Nonsec9> for bool {
    #[inline(always)]
    fn from(variant: Nonsec9) -> Self {
        variant as u8 != 0
    }
}
///Field `NONSEC9` reader - Non Secure Attribute bit 9
pub type Nonsec9R = crate::BitReader<Nonsec9>;
impl Nonsec9R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Nonsec9 {
        match self.bits {
            false => Nonsec9::_0,
            true => Nonsec9::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Nonsec9::_0
    }
    ///Non Secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Nonsec9::_1
    }
}
///Field `NONSEC9` writer - Non Secure Attribute bit 9
pub type Nonsec9W<'a, REG> = crate::BitWriter<'a, REG, Nonsec9>;
impl<'a, REG> Nonsec9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Nonsec9::_0)
    }
    ///Non Secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Nonsec9::_1)
    }
}
impl R {
    ///Bit 0 - Non Secure Attribute bit 0
    #[inline(always)]
    pub fn nonsec0(&self) -> Nonsec0R {
        Nonsec0R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - Non Secure Attribute bit 2
    #[inline(always)]
    pub fn nonsec2(&self) -> Nonsec2R {
        Nonsec2R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - Non Secure Attribute bit 4
    #[inline(always)]
    pub fn nonsec4(&self) -> Nonsec4R {
        Nonsec4R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - Non Secure Attribute bit 8
    #[inline(always)]
    pub fn nonsec8(&self) -> Nonsec8R {
        Nonsec8R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Non Secure Attribute bit 9
    #[inline(always)]
    pub fn nonsec9(&self) -> Nonsec9R {
        Nonsec9R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPMSAR")
            .field("nonsec0", &self.nonsec0())
            .field("nonsec2", &self.nonsec2())
            .field("nonsec4", &self.nonsec4())
            .field("nonsec8", &self.nonsec8())
            .field("nonsec9", &self.nonsec9())
            .finish()
    }
}
impl W {
    ///Bit 0 - Non Secure Attribute bit 0
    #[inline(always)]
    pub fn nonsec0(&mut self) -> Nonsec0W<LpmsarSpec> {
        Nonsec0W::new(self, 0)
    }
    ///Bit 2 - Non Secure Attribute bit 2
    #[inline(always)]
    pub fn nonsec2(&mut self) -> Nonsec2W<LpmsarSpec> {
        Nonsec2W::new(self, 2)
    }
    ///Bit 4 - Non Secure Attribute bit 4
    #[inline(always)]
    pub fn nonsec4(&mut self) -> Nonsec4W<LpmsarSpec> {
        Nonsec4W::new(self, 4)
    }
    ///Bit 8 - Non Secure Attribute bit 8
    #[inline(always)]
    pub fn nonsec8(&mut self) -> Nonsec8W<LpmsarSpec> {
        Nonsec8W::new(self, 8)
    }
    ///Bit 9 - Non Secure Attribute bit 9
    #[inline(always)]
    pub fn nonsec9(&mut self) -> Nonsec9W<LpmsarSpec> {
        Nonsec9W::new(self, 9)
    }
}
/**Low Power Mode Security Attribution Register

You can [`read`](crate::Reg::read) this register and get [`lpmsar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpmsar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LpmsarSpec;
impl crate::RegisterSpec for LpmsarSpec {
    type Ux = u32;
}
///`read()` method returns [`lpmsar::R`](R) reader structure
impl crate::Readable for LpmsarSpec {}
///`write(|w| ..)` method takes [`lpmsar::W`](W) writer structure
impl crate::Writable for LpmsarSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LPMSAR to value 0xffff_ffff
impl crate::Resettable for LpmsarSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
