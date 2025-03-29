///Register `ICUSARC` reader
pub type R = crate::R<IcusarcSpec>;
///Register `ICUSARC` writer
pub type W = crate::W<IcusarcSpec>;
/**Security attributes of registers for DMAC channel

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sadmac0 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Sadmac0> for bool {
    #[inline(always)]
    fn from(variant: Sadmac0) -> Self {
        variant as u8 != 0
    }
}
///Field `SADMAC0` reader - Security attributes of registers for DMAC channel
pub type Sadmac0R = crate::BitReader<Sadmac0>;
impl Sadmac0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sadmac0 {
        match self.bits {
            false => Sadmac0::_0,
            true => Sadmac0::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sadmac0::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sadmac0::_1
    }
}
///Field `SADMAC0` writer - Security attributes of registers for DMAC channel
pub type Sadmac0W<'a, REG> = crate::BitWriter<'a, REG, Sadmac0>;
impl<'a, REG> Sadmac0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sadmac0::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sadmac0::_1)
    }
}
/**Security attributes of registers for DMAC channel

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sadmac1 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Sadmac1> for bool {
    #[inline(always)]
    fn from(variant: Sadmac1) -> Self {
        variant as u8 != 0
    }
}
///Field `SADMAC1` reader - Security attributes of registers for DMAC channel
pub type Sadmac1R = crate::BitReader<Sadmac1>;
impl Sadmac1R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sadmac1 {
        match self.bits {
            false => Sadmac1::_0,
            true => Sadmac1::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sadmac1::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sadmac1::_1
    }
}
///Field `SADMAC1` writer - Security attributes of registers for DMAC channel
pub type Sadmac1W<'a, REG> = crate::BitWriter<'a, REG, Sadmac1>;
impl<'a, REG> Sadmac1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sadmac1::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sadmac1::_1)
    }
}
/**Security attributes of registers for DMAC channel

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sadmac2 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Sadmac2> for bool {
    #[inline(always)]
    fn from(variant: Sadmac2) -> Self {
        variant as u8 != 0
    }
}
///Field `SADMAC2` reader - Security attributes of registers for DMAC channel
pub type Sadmac2R = crate::BitReader<Sadmac2>;
impl Sadmac2R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sadmac2 {
        match self.bits {
            false => Sadmac2::_0,
            true => Sadmac2::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sadmac2::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sadmac2::_1
    }
}
///Field `SADMAC2` writer - Security attributes of registers for DMAC channel
pub type Sadmac2W<'a, REG> = crate::BitWriter<'a, REG, Sadmac2>;
impl<'a, REG> Sadmac2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sadmac2::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sadmac2::_1)
    }
}
/**Security attributes of registers for DMAC channel

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sadmac3 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Sadmac3> for bool {
    #[inline(always)]
    fn from(variant: Sadmac3) -> Self {
        variant as u8 != 0
    }
}
///Field `SADMAC3` reader - Security attributes of registers for DMAC channel
pub type Sadmac3R = crate::BitReader<Sadmac3>;
impl Sadmac3R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sadmac3 {
        match self.bits {
            false => Sadmac3::_0,
            true => Sadmac3::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sadmac3::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sadmac3::_1
    }
}
///Field `SADMAC3` writer - Security attributes of registers for DMAC channel
pub type Sadmac3W<'a, REG> = crate::BitWriter<'a, REG, Sadmac3>;
impl<'a, REG> Sadmac3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sadmac3::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sadmac3::_1)
    }
}
/**Security attributes of registers for DMAC channel

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sadmac4 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Sadmac4> for bool {
    #[inline(always)]
    fn from(variant: Sadmac4) -> Self {
        variant as u8 != 0
    }
}
///Field `SADMAC4` reader - Security attributes of registers for DMAC channel
pub type Sadmac4R = crate::BitReader<Sadmac4>;
impl Sadmac4R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sadmac4 {
        match self.bits {
            false => Sadmac4::_0,
            true => Sadmac4::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sadmac4::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sadmac4::_1
    }
}
///Field `SADMAC4` writer - Security attributes of registers for DMAC channel
pub type Sadmac4W<'a, REG> = crate::BitWriter<'a, REG, Sadmac4>;
impl<'a, REG> Sadmac4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sadmac4::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sadmac4::_1)
    }
}
/**Security attributes of registers for DMAC channel

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sadmac5 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Sadmac5> for bool {
    #[inline(always)]
    fn from(variant: Sadmac5) -> Self {
        variant as u8 != 0
    }
}
///Field `SADMAC5` reader - Security attributes of registers for DMAC channel
pub type Sadmac5R = crate::BitReader<Sadmac5>;
impl Sadmac5R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sadmac5 {
        match self.bits {
            false => Sadmac5::_0,
            true => Sadmac5::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sadmac5::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sadmac5::_1
    }
}
///Field `SADMAC5` writer - Security attributes of registers for DMAC channel
pub type Sadmac5W<'a, REG> = crate::BitWriter<'a, REG, Sadmac5>;
impl<'a, REG> Sadmac5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sadmac5::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sadmac5::_1)
    }
}
/**Security attributes of registers for DMAC channel

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sadmac6 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Sadmac6> for bool {
    #[inline(always)]
    fn from(variant: Sadmac6) -> Self {
        variant as u8 != 0
    }
}
///Field `SADMAC6` reader - Security attributes of registers for DMAC channel
pub type Sadmac6R = crate::BitReader<Sadmac6>;
impl Sadmac6R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sadmac6 {
        match self.bits {
            false => Sadmac6::_0,
            true => Sadmac6::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sadmac6::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sadmac6::_1
    }
}
///Field `SADMAC6` writer - Security attributes of registers for DMAC channel
pub type Sadmac6W<'a, REG> = crate::BitWriter<'a, REG, Sadmac6>;
impl<'a, REG> Sadmac6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sadmac6::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sadmac6::_1)
    }
}
/**Security attributes of registers for DMAC channel

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sadmac7 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Sadmac7> for bool {
    #[inline(always)]
    fn from(variant: Sadmac7) -> Self {
        variant as u8 != 0
    }
}
///Field `SADMAC7` reader - Security attributes of registers for DMAC channel
pub type Sadmac7R = crate::BitReader<Sadmac7>;
impl Sadmac7R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sadmac7 {
        match self.bits {
            false => Sadmac7::_0,
            true => Sadmac7::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sadmac7::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sadmac7::_1
    }
}
///Field `SADMAC7` writer - Security attributes of registers for DMAC channel
pub type Sadmac7W<'a, REG> = crate::BitWriter<'a, REG, Sadmac7>;
impl<'a, REG> Sadmac7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sadmac7::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sadmac7::_1)
    }
}
impl R {
    ///Bit 0 - Security attributes of registers for DMAC channel
    #[inline(always)]
    pub fn sadmac0(&self) -> Sadmac0R {
        Sadmac0R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Security attributes of registers for DMAC channel
    #[inline(always)]
    pub fn sadmac1(&self) -> Sadmac1R {
        Sadmac1R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Security attributes of registers for DMAC channel
    #[inline(always)]
    pub fn sadmac2(&self) -> Sadmac2R {
        Sadmac2R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Security attributes of registers for DMAC channel
    #[inline(always)]
    pub fn sadmac3(&self) -> Sadmac3R {
        Sadmac3R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Security attributes of registers for DMAC channel
    #[inline(always)]
    pub fn sadmac4(&self) -> Sadmac4R {
        Sadmac4R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Security attributes of registers for DMAC channel
    #[inline(always)]
    pub fn sadmac5(&self) -> Sadmac5R {
        Sadmac5R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Security attributes of registers for DMAC channel
    #[inline(always)]
    pub fn sadmac6(&self) -> Sadmac6R {
        Sadmac6R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Security attributes of registers for DMAC channel
    #[inline(always)]
    pub fn sadmac7(&self) -> Sadmac7R {
        Sadmac7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICUSARC")
            .field("sadmac0", &self.sadmac0())
            .field("sadmac1", &self.sadmac1())
            .field("sadmac2", &self.sadmac2())
            .field("sadmac3", &self.sadmac3())
            .field("sadmac4", &self.sadmac4())
            .field("sadmac5", &self.sadmac5())
            .field("sadmac6", &self.sadmac6())
            .field("sadmac7", &self.sadmac7())
            .finish()
    }
}
impl W {
    ///Bit 0 - Security attributes of registers for DMAC channel
    #[inline(always)]
    pub fn sadmac0(&mut self) -> Sadmac0W<IcusarcSpec> {
        Sadmac0W::new(self, 0)
    }
    ///Bit 1 - Security attributes of registers for DMAC channel
    #[inline(always)]
    pub fn sadmac1(&mut self) -> Sadmac1W<IcusarcSpec> {
        Sadmac1W::new(self, 1)
    }
    ///Bit 2 - Security attributes of registers for DMAC channel
    #[inline(always)]
    pub fn sadmac2(&mut self) -> Sadmac2W<IcusarcSpec> {
        Sadmac2W::new(self, 2)
    }
    ///Bit 3 - Security attributes of registers for DMAC channel
    #[inline(always)]
    pub fn sadmac3(&mut self) -> Sadmac3W<IcusarcSpec> {
        Sadmac3W::new(self, 3)
    }
    ///Bit 4 - Security attributes of registers for DMAC channel
    #[inline(always)]
    pub fn sadmac4(&mut self) -> Sadmac4W<IcusarcSpec> {
        Sadmac4W::new(self, 4)
    }
    ///Bit 5 - Security attributes of registers for DMAC channel
    #[inline(always)]
    pub fn sadmac5(&mut self) -> Sadmac5W<IcusarcSpec> {
        Sadmac5W::new(self, 5)
    }
    ///Bit 6 - Security attributes of registers for DMAC channel
    #[inline(always)]
    pub fn sadmac6(&mut self) -> Sadmac6W<IcusarcSpec> {
        Sadmac6W::new(self, 6)
    }
    ///Bit 7 - Security attributes of registers for DMAC channel
    #[inline(always)]
    pub fn sadmac7(&mut self) -> Sadmac7W<IcusarcSpec> {
        Sadmac7W::new(self, 7)
    }
}
/**Interrupt Controller Unit Security Attribution Register C

You can [`read`](crate::Reg::read) this register and get [`icusarc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icusarc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IcusarcSpec;
impl crate::RegisterSpec for IcusarcSpec {
    type Ux = u32;
}
///`read()` method returns [`icusarc::R`](R) reader structure
impl crate::Readable for IcusarcSpec {}
///`write(|w| ..)` method takes [`icusarc::W`](W) writer structure
impl crate::Writable for IcusarcSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICUSARC to value 0xffff_ffff
impl crate::Resettable for IcusarcSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
