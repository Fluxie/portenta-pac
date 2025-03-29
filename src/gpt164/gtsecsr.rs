///Register `GTSECSR` reader
pub type R = crate::R<GtsecsrSpec>;
///Register `GTSECSR` writer
pub type W = crate::W<GtsecsrSpec>;
/**Channel 0 Operation Enable Bit Simultaneous Control Channel Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Secsel0 {
    ///0: Disable simultaneous control
    _0 = 0,
    ///1: Enable simultaneous control
    _1 = 1,
}
impl From<Secsel0> for bool {
    #[inline(always)]
    fn from(variant: Secsel0) -> Self {
        variant as u8 != 0
    }
}
///Field `SECSEL0` reader - Channel 0 Operation Enable Bit Simultaneous Control Channel Select
pub type Secsel0R = crate::BitReader<Secsel0>;
impl Secsel0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Secsel0 {
        match self.bits {
            false => Secsel0::_0,
            true => Secsel0::_1,
        }
    }
    ///Disable simultaneous control
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Secsel0::_0
    }
    ///Enable simultaneous control
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Secsel0::_1
    }
}
///Field `SECSEL0` writer - Channel 0 Operation Enable Bit Simultaneous Control Channel Select
pub type Secsel0W<'a, REG> = crate::BitWriter<'a, REG, Secsel0>;
impl<'a, REG> Secsel0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable simultaneous control
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Secsel0::_0)
    }
    ///Enable simultaneous control
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Secsel0::_1)
    }
}
/**Channel 1 Operation Enable Bit Simultaneous Control Channel Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Secsel1 {
    ///0: Disable simultaneous control
    _0 = 0,
    ///1: Enable simultaneous control
    _1 = 1,
}
impl From<Secsel1> for bool {
    #[inline(always)]
    fn from(variant: Secsel1) -> Self {
        variant as u8 != 0
    }
}
///Field `SECSEL1` reader - Channel 1 Operation Enable Bit Simultaneous Control Channel Select
pub type Secsel1R = crate::BitReader<Secsel1>;
impl Secsel1R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Secsel1 {
        match self.bits {
            false => Secsel1::_0,
            true => Secsel1::_1,
        }
    }
    ///Disable simultaneous control
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Secsel1::_0
    }
    ///Enable simultaneous control
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Secsel1::_1
    }
}
///Field `SECSEL1` writer - Channel 1 Operation Enable Bit Simultaneous Control Channel Select
pub type Secsel1W<'a, REG> = crate::BitWriter<'a, REG, Secsel1>;
impl<'a, REG> Secsel1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable simultaneous control
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Secsel1::_0)
    }
    ///Enable simultaneous control
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Secsel1::_1)
    }
}
/**Channel 2 Operation Enable Bit Simultaneous Control Channel Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Secsel2 {
    ///0: Disable simultaneous control
    _0 = 0,
    ///1: Enable simultaneous control
    _1 = 1,
}
impl From<Secsel2> for bool {
    #[inline(always)]
    fn from(variant: Secsel2) -> Self {
        variant as u8 != 0
    }
}
///Field `SECSEL2` reader - Channel 2 Operation Enable Bit Simultaneous Control Channel Select
pub type Secsel2R = crate::BitReader<Secsel2>;
impl Secsel2R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Secsel2 {
        match self.bits {
            false => Secsel2::_0,
            true => Secsel2::_1,
        }
    }
    ///Disable simultaneous control
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Secsel2::_0
    }
    ///Enable simultaneous control
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Secsel2::_1
    }
}
///Field `SECSEL2` writer - Channel 2 Operation Enable Bit Simultaneous Control Channel Select
pub type Secsel2W<'a, REG> = crate::BitWriter<'a, REG, Secsel2>;
impl<'a, REG> Secsel2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable simultaneous control
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Secsel2::_0)
    }
    ///Enable simultaneous control
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Secsel2::_1)
    }
}
/**Channel 3 Operation Enable Bit Simultaneous Control Channel Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Secsel3 {
    ///0: Disable simultaneous control
    _0 = 0,
    ///1: Enable simultaneous control
    _1 = 1,
}
impl From<Secsel3> for bool {
    #[inline(always)]
    fn from(variant: Secsel3) -> Self {
        variant as u8 != 0
    }
}
///Field `SECSEL3` reader - Channel 3 Operation Enable Bit Simultaneous Control Channel Select
pub type Secsel3R = crate::BitReader<Secsel3>;
impl Secsel3R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Secsel3 {
        match self.bits {
            false => Secsel3::_0,
            true => Secsel3::_1,
        }
    }
    ///Disable simultaneous control
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Secsel3::_0
    }
    ///Enable simultaneous control
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Secsel3::_1
    }
}
///Field `SECSEL3` writer - Channel 3 Operation Enable Bit Simultaneous Control Channel Select
pub type Secsel3W<'a, REG> = crate::BitWriter<'a, REG, Secsel3>;
impl<'a, REG> Secsel3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable simultaneous control
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Secsel3::_0)
    }
    ///Enable simultaneous control
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Secsel3::_1)
    }
}
/**Channel 4 Operation Enable Bit Simultaneous Control Channel Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Secsel4 {
    ///0: Disable simultaneous control
    _0 = 0,
    ///1: Enable simultaneous control
    _1 = 1,
}
impl From<Secsel4> for bool {
    #[inline(always)]
    fn from(variant: Secsel4) -> Self {
        variant as u8 != 0
    }
}
///Field `SECSEL4` reader - Channel 4 Operation Enable Bit Simultaneous Control Channel Select
pub type Secsel4R = crate::BitReader<Secsel4>;
impl Secsel4R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Secsel4 {
        match self.bits {
            false => Secsel4::_0,
            true => Secsel4::_1,
        }
    }
    ///Disable simultaneous control
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Secsel4::_0
    }
    ///Enable simultaneous control
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Secsel4::_1
    }
}
///Field `SECSEL4` writer - Channel 4 Operation Enable Bit Simultaneous Control Channel Select
pub type Secsel4W<'a, REG> = crate::BitWriter<'a, REG, Secsel4>;
impl<'a, REG> Secsel4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable simultaneous control
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Secsel4::_0)
    }
    ///Enable simultaneous control
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Secsel4::_1)
    }
}
/**Channel 5 Operation Enable Bit Simultaneous Control Channel Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Secsel5 {
    ///0: Disable simultaneous control
    _0 = 0,
    ///1: Enable simultaneous control
    _1 = 1,
}
impl From<Secsel5> for bool {
    #[inline(always)]
    fn from(variant: Secsel5) -> Self {
        variant as u8 != 0
    }
}
///Field `SECSEL5` reader - Channel 5 Operation Enable Bit Simultaneous Control Channel Select
pub type Secsel5R = crate::BitReader<Secsel5>;
impl Secsel5R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Secsel5 {
        match self.bits {
            false => Secsel5::_0,
            true => Secsel5::_1,
        }
    }
    ///Disable simultaneous control
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Secsel5::_0
    }
    ///Enable simultaneous control
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Secsel5::_1
    }
}
///Field `SECSEL5` writer - Channel 5 Operation Enable Bit Simultaneous Control Channel Select
pub type Secsel5W<'a, REG> = crate::BitWriter<'a, REG, Secsel5>;
impl<'a, REG> Secsel5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable simultaneous control
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Secsel5::_0)
    }
    ///Enable simultaneous control
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Secsel5::_1)
    }
}
/**Channel 6 Operation Enable Bit Simultaneous Control Channel Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Secsel6 {
    ///0: Disable simultaneous control
    _0 = 0,
    ///1: Enable simultaneous control
    _1 = 1,
}
impl From<Secsel6> for bool {
    #[inline(always)]
    fn from(variant: Secsel6) -> Self {
        variant as u8 != 0
    }
}
///Field `SECSEL6` reader - Channel 6 Operation Enable Bit Simultaneous Control Channel Select
pub type Secsel6R = crate::BitReader<Secsel6>;
impl Secsel6R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Secsel6 {
        match self.bits {
            false => Secsel6::_0,
            true => Secsel6::_1,
        }
    }
    ///Disable simultaneous control
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Secsel6::_0
    }
    ///Enable simultaneous control
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Secsel6::_1
    }
}
///Field `SECSEL6` writer - Channel 6 Operation Enable Bit Simultaneous Control Channel Select
pub type Secsel6W<'a, REG> = crate::BitWriter<'a, REG, Secsel6>;
impl<'a, REG> Secsel6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable simultaneous control
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Secsel6::_0)
    }
    ///Enable simultaneous control
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Secsel6::_1)
    }
}
/**Channel 7 Operation Enable Bit Simultaneous Control Channel Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Secsel7 {
    ///0: Disable simultaneous control
    _0 = 0,
    ///1: Enable simultaneous control
    _1 = 1,
}
impl From<Secsel7> for bool {
    #[inline(always)]
    fn from(variant: Secsel7) -> Self {
        variant as u8 != 0
    }
}
///Field `SECSEL7` reader - Channel 7 Operation Enable Bit Simultaneous Control Channel Select
pub type Secsel7R = crate::BitReader<Secsel7>;
impl Secsel7R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Secsel7 {
        match self.bits {
            false => Secsel7::_0,
            true => Secsel7::_1,
        }
    }
    ///Disable simultaneous control
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Secsel7::_0
    }
    ///Enable simultaneous control
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Secsel7::_1
    }
}
///Field `SECSEL7` writer - Channel 7 Operation Enable Bit Simultaneous Control Channel Select
pub type Secsel7W<'a, REG> = crate::BitWriter<'a, REG, Secsel7>;
impl<'a, REG> Secsel7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable simultaneous control
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Secsel7::_0)
    }
    ///Enable simultaneous control
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Secsel7::_1)
    }
}
/**Channel 8 Operation Enable Bit Simultaneous Control Channel Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Secsel8 {
    ///0: Disable simultaneous control
    _0 = 0,
    ///1: Enable simultaneous control
    _1 = 1,
}
impl From<Secsel8> for bool {
    #[inline(always)]
    fn from(variant: Secsel8) -> Self {
        variant as u8 != 0
    }
}
///Field `SECSEL8` reader - Channel 8 Operation Enable Bit Simultaneous Control Channel Select
pub type Secsel8R = crate::BitReader<Secsel8>;
impl Secsel8R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Secsel8 {
        match self.bits {
            false => Secsel8::_0,
            true => Secsel8::_1,
        }
    }
    ///Disable simultaneous control
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Secsel8::_0
    }
    ///Enable simultaneous control
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Secsel8::_1
    }
}
///Field `SECSEL8` writer - Channel 8 Operation Enable Bit Simultaneous Control Channel Select
pub type Secsel8W<'a, REG> = crate::BitWriter<'a, REG, Secsel8>;
impl<'a, REG> Secsel8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable simultaneous control
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Secsel8::_0)
    }
    ///Enable simultaneous control
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Secsel8::_1)
    }
}
/**Channel 9 Operation Enable Bit Simultaneous Control Channel Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Secsel9 {
    ///0: Disable simultaneous control
    _0 = 0,
    ///1: Enable simultaneous control
    _1 = 1,
}
impl From<Secsel9> for bool {
    #[inline(always)]
    fn from(variant: Secsel9) -> Self {
        variant as u8 != 0
    }
}
///Field `SECSEL9` reader - Channel 9 Operation Enable Bit Simultaneous Control Channel Select
pub type Secsel9R = crate::BitReader<Secsel9>;
impl Secsel9R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Secsel9 {
        match self.bits {
            false => Secsel9::_0,
            true => Secsel9::_1,
        }
    }
    ///Disable simultaneous control
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Secsel9::_0
    }
    ///Enable simultaneous control
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Secsel9::_1
    }
}
///Field `SECSEL9` writer - Channel 9 Operation Enable Bit Simultaneous Control Channel Select
pub type Secsel9W<'a, REG> = crate::BitWriter<'a, REG, Secsel9>;
impl<'a, REG> Secsel9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable simultaneous control
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Secsel9::_0)
    }
    ///Enable simultaneous control
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Secsel9::_1)
    }
}
impl R {
    ///Bit 0 - Channel 0 Operation Enable Bit Simultaneous Control Channel Select
    #[inline(always)]
    pub fn secsel0(&self) -> Secsel0R {
        Secsel0R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Channel 1 Operation Enable Bit Simultaneous Control Channel Select
    #[inline(always)]
    pub fn secsel1(&self) -> Secsel1R {
        Secsel1R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Channel 2 Operation Enable Bit Simultaneous Control Channel Select
    #[inline(always)]
    pub fn secsel2(&self) -> Secsel2R {
        Secsel2R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Channel 3 Operation Enable Bit Simultaneous Control Channel Select
    #[inline(always)]
    pub fn secsel3(&self) -> Secsel3R {
        Secsel3R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Channel 4 Operation Enable Bit Simultaneous Control Channel Select
    #[inline(always)]
    pub fn secsel4(&self) -> Secsel4R {
        Secsel4R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Channel 5 Operation Enable Bit Simultaneous Control Channel Select
    #[inline(always)]
    pub fn secsel5(&self) -> Secsel5R {
        Secsel5R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Channel 6 Operation Enable Bit Simultaneous Control Channel Select
    #[inline(always)]
    pub fn secsel6(&self) -> Secsel6R {
        Secsel6R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Channel 7 Operation Enable Bit Simultaneous Control Channel Select
    #[inline(always)]
    pub fn secsel7(&self) -> Secsel7R {
        Secsel7R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Channel 8 Operation Enable Bit Simultaneous Control Channel Select
    #[inline(always)]
    pub fn secsel8(&self) -> Secsel8R {
        Secsel8R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Channel 9 Operation Enable Bit Simultaneous Control Channel Select
    #[inline(always)]
    pub fn secsel9(&self) -> Secsel9R {
        Secsel9R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GTSECSR")
            .field("secsel0", &self.secsel0())
            .field("secsel1", &self.secsel1())
            .field("secsel2", &self.secsel2())
            .field("secsel3", &self.secsel3())
            .field("secsel4", &self.secsel4())
            .field("secsel5", &self.secsel5())
            .field("secsel6", &self.secsel6())
            .field("secsel7", &self.secsel7())
            .field("secsel8", &self.secsel8())
            .field("secsel9", &self.secsel9())
            .finish()
    }
}
impl W {
    ///Bit 0 - Channel 0 Operation Enable Bit Simultaneous Control Channel Select
    #[inline(always)]
    pub fn secsel0(&mut self) -> Secsel0W<GtsecsrSpec> {
        Secsel0W::new(self, 0)
    }
    ///Bit 1 - Channel 1 Operation Enable Bit Simultaneous Control Channel Select
    #[inline(always)]
    pub fn secsel1(&mut self) -> Secsel1W<GtsecsrSpec> {
        Secsel1W::new(self, 1)
    }
    ///Bit 2 - Channel 2 Operation Enable Bit Simultaneous Control Channel Select
    #[inline(always)]
    pub fn secsel2(&mut self) -> Secsel2W<GtsecsrSpec> {
        Secsel2W::new(self, 2)
    }
    ///Bit 3 - Channel 3 Operation Enable Bit Simultaneous Control Channel Select
    #[inline(always)]
    pub fn secsel3(&mut self) -> Secsel3W<GtsecsrSpec> {
        Secsel3W::new(self, 3)
    }
    ///Bit 4 - Channel 4 Operation Enable Bit Simultaneous Control Channel Select
    #[inline(always)]
    pub fn secsel4(&mut self) -> Secsel4W<GtsecsrSpec> {
        Secsel4W::new(self, 4)
    }
    ///Bit 5 - Channel 5 Operation Enable Bit Simultaneous Control Channel Select
    #[inline(always)]
    pub fn secsel5(&mut self) -> Secsel5W<GtsecsrSpec> {
        Secsel5W::new(self, 5)
    }
    ///Bit 6 - Channel 6 Operation Enable Bit Simultaneous Control Channel Select
    #[inline(always)]
    pub fn secsel6(&mut self) -> Secsel6W<GtsecsrSpec> {
        Secsel6W::new(self, 6)
    }
    ///Bit 7 - Channel 7 Operation Enable Bit Simultaneous Control Channel Select
    #[inline(always)]
    pub fn secsel7(&mut self) -> Secsel7W<GtsecsrSpec> {
        Secsel7W::new(self, 7)
    }
    ///Bit 8 - Channel 8 Operation Enable Bit Simultaneous Control Channel Select
    #[inline(always)]
    pub fn secsel8(&mut self) -> Secsel8W<GtsecsrSpec> {
        Secsel8W::new(self, 8)
    }
    ///Bit 9 - Channel 9 Operation Enable Bit Simultaneous Control Channel Select
    #[inline(always)]
    pub fn secsel9(&mut self) -> Secsel9W<GtsecsrSpec> {
        Secsel9W::new(self, 9)
    }
}
/**General PWM Timer Operation Enable Bit Simultaneous Control Channel Select Register

You can [`read`](crate::Reg::read) this register and get [`gtsecsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtsecsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GtsecsrSpec;
impl crate::RegisterSpec for GtsecsrSpec {
    type Ux = u32;
}
///`read()` method returns [`gtsecsr::R`](R) reader structure
impl crate::Readable for GtsecsrSpec {}
///`write(|w| ..)` method takes [`gtsecsr::W`](W) writer structure
impl crate::Writable for GtsecsrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTSECSR to value 0
impl crate::Resettable for GtsecsrSpec {}
