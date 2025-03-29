///Register `MSTPCRC` reader
pub type R = crate::R<MstpcrcSpec>;
///Register `MSTPCRC` writer
pub type W = crate::W<MstpcrcSpec>;
/**Clock Frequency Accuracy Measurement Circuit Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpc0 {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<Mstpc0> for bool {
    #[inline(always)]
    fn from(variant: Mstpc0) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPC0` reader - Clock Frequency Accuracy Measurement Circuit Module Stop
pub type Mstpc0R = crate::BitReader<Mstpc0>;
impl Mstpc0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mstpc0 {
        match self.bits {
            false => Mstpc0::_0,
            true => Mstpc0::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpc0::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpc0::_1
    }
}
///Field `MSTPC0` writer - Clock Frequency Accuracy Measurement Circuit Module Stop
pub type Mstpc0W<'a, REG> = crate::BitWriter<'a, REG, Mstpc0>;
impl<'a, REG> Mstpc0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpc0::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpc0::_1)
    }
}
/**Cyclic Redundancy Check Calculator Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpc1 {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<Mstpc1> for bool {
    #[inline(always)]
    fn from(variant: Mstpc1) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPC1` reader - Cyclic Redundancy Check Calculator Module Stop
pub type Mstpc1R = crate::BitReader<Mstpc1>;
impl Mstpc1R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mstpc1 {
        match self.bits {
            false => Mstpc1::_0,
            true => Mstpc1::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpc1::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpc1::_1
    }
}
///Field `MSTPC1` writer - Cyclic Redundancy Check Calculator Module Stop
pub type Mstpc1W<'a, REG> = crate::BitWriter<'a, REG, Mstpc1>;
impl<'a, REG> Mstpc1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpc1::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpc1::_1)
    }
}
/**Capacitive Touch Sensing Unit Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpc3 {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<Mstpc3> for bool {
    #[inline(always)]
    fn from(variant: Mstpc3) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPC3` reader - Capacitive Touch Sensing Unit Module Stop
pub type Mstpc3R = crate::BitReader<Mstpc3>;
impl Mstpc3R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mstpc3 {
        match self.bits {
            false => Mstpc3::_0,
            true => Mstpc3::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpc3::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpc3::_1
    }
}
///Field `MSTPC3` writer - Capacitive Touch Sensing Unit Module Stop
pub type Mstpc3W<'a, REG> = crate::BitWriter<'a, REG, Mstpc3>;
impl<'a, REG> Mstpc3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpc3::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpc3::_1)
    }
}
/**Serial Sound Interface Enhanced Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpc8 {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<Mstpc8> for bool {
    #[inline(always)]
    fn from(variant: Mstpc8) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPC8` reader - Serial Sound Interface Enhanced Module Stop
pub type Mstpc8R = crate::BitReader<Mstpc8>;
impl Mstpc8R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mstpc8 {
        match self.bits {
            false => Mstpc8::_0,
            true => Mstpc8::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpc8::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpc8::_1
    }
}
///Field `MSTPC8` writer - Serial Sound Interface Enhanced Module Stop
pub type Mstpc8W<'a, REG> = crate::BitWriter<'a, REG, Mstpc8>;
impl<'a, REG> Mstpc8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpc8::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpc8::_1)
    }
}
/**Secure Digital HOST IF / Multi Media Card 0 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpc12 {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<Mstpc12> for bool {
    #[inline(always)]
    fn from(variant: Mstpc12) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPC12` reader - Secure Digital HOST IF / Multi Media Card 0 Module Stop
pub type Mstpc12R = crate::BitReader<Mstpc12>;
impl Mstpc12R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mstpc12 {
        match self.bits {
            false => Mstpc12::_0,
            true => Mstpc12::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpc12::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpc12::_1
    }
}
///Field `MSTPC12` writer - Secure Digital HOST IF / Multi Media Card 0 Module Stop
pub type Mstpc12W<'a, REG> = crate::BitWriter<'a, REG, Mstpc12>;
impl<'a, REG> Mstpc12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpc12::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpc12::_1)
    }
}
/**Data Operation Circuit Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpc13 {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<Mstpc13> for bool {
    #[inline(always)]
    fn from(variant: Mstpc13) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPC13` reader - Data Operation Circuit Module Stop
pub type Mstpc13R = crate::BitReader<Mstpc13>;
impl Mstpc13R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mstpc13 {
        match self.bits {
            false => Mstpc13::_0,
            true => Mstpc13::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpc13::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpc13::_1
    }
}
///Field `MSTPC13` writer - Data Operation Circuit Module Stop
pub type Mstpc13W<'a, REG> = crate::BitWriter<'a, REG, Mstpc13>;
impl<'a, REG> Mstpc13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpc13::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpc13::_1)
    }
}
/**Event Link Controller Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpc14 {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<Mstpc14> for bool {
    #[inline(always)]
    fn from(variant: Mstpc14) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPC14` reader - Event Link Controller Module Stop
pub type Mstpc14R = crate::BitReader<Mstpc14>;
impl Mstpc14R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mstpc14 {
        match self.bits {
            false => Mstpc14::_0,
            true => Mstpc14::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpc14::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpc14::_1
    }
}
///Field `MSTPC14` writer - Event Link Controller Module Stop
pub type Mstpc14W<'a, REG> = crate::BitWriter<'a, REG, Mstpc14>;
impl<'a, REG> Mstpc14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpc14::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpc14::_1)
    }
}
/**CANFD Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpc27 {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<Mstpc27> for bool {
    #[inline(always)]
    fn from(variant: Mstpc27) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPC27` reader - CANFD Module Stop
pub type Mstpc27R = crate::BitReader<Mstpc27>;
impl Mstpc27R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mstpc27 {
        match self.bits {
            false => Mstpc27::_0,
            true => Mstpc27::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpc27::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpc27::_1
    }
}
///Field `MSTPC27` writer - CANFD Module Stop
pub type Mstpc27W<'a, REG> = crate::BitWriter<'a, REG, Mstpc27>;
impl<'a, REG> Mstpc27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpc27::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpc27::_1)
    }
}
/**SCE9 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpc31 {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<Mstpc31> for bool {
    #[inline(always)]
    fn from(variant: Mstpc31) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPC31` reader - SCE9 Module Stop
pub type Mstpc31R = crate::BitReader<Mstpc31>;
impl Mstpc31R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mstpc31 {
        match self.bits {
            false => Mstpc31::_0,
            true => Mstpc31::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpc31::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpc31::_1
    }
}
///Field `MSTPC31` writer - SCE9 Module Stop
pub type Mstpc31W<'a, REG> = crate::BitWriter<'a, REG, Mstpc31>;
impl<'a, REG> Mstpc31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpc31::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpc31::_1)
    }
}
impl R {
    ///Bit 0 - Clock Frequency Accuracy Measurement Circuit Module Stop
    #[inline(always)]
    pub fn mstpc0(&self) -> Mstpc0R {
        Mstpc0R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Cyclic Redundancy Check Calculator Module Stop
    #[inline(always)]
    pub fn mstpc1(&self) -> Mstpc1R {
        Mstpc1R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - Capacitive Touch Sensing Unit Module Stop
    #[inline(always)]
    pub fn mstpc3(&self) -> Mstpc3R {
        Mstpc3R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 8 - Serial Sound Interface Enhanced Module Stop
    #[inline(always)]
    pub fn mstpc8(&self) -> Mstpc8R {
        Mstpc8R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - Secure Digital HOST IF / Multi Media Card 0 Module Stop
    #[inline(always)]
    pub fn mstpc12(&self) -> Mstpc12R {
        Mstpc12R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Data Operation Circuit Module Stop
    #[inline(always)]
    pub fn mstpc13(&self) -> Mstpc13R {
        Mstpc13R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Event Link Controller Module Stop
    #[inline(always)]
    pub fn mstpc14(&self) -> Mstpc14R {
        Mstpc14R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 27 - CANFD Module Stop
    #[inline(always)]
    pub fn mstpc27(&self) -> Mstpc27R {
        Mstpc27R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 31 - SCE9 Module Stop
    #[inline(always)]
    pub fn mstpc31(&self) -> Mstpc31R {
        Mstpc31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MSTPCRC")
            .field("mstpc0", &self.mstpc0())
            .field("mstpc1", &self.mstpc1())
            .field("mstpc3", &self.mstpc3())
            .field("mstpc8", &self.mstpc8())
            .field("mstpc12", &self.mstpc12())
            .field("mstpc13", &self.mstpc13())
            .field("mstpc14", &self.mstpc14())
            .field("mstpc27", &self.mstpc27())
            .field("mstpc31", &self.mstpc31())
            .finish()
    }
}
impl W {
    ///Bit 0 - Clock Frequency Accuracy Measurement Circuit Module Stop
    #[inline(always)]
    pub fn mstpc0(&mut self) -> Mstpc0W<MstpcrcSpec> {
        Mstpc0W::new(self, 0)
    }
    ///Bit 1 - Cyclic Redundancy Check Calculator Module Stop
    #[inline(always)]
    pub fn mstpc1(&mut self) -> Mstpc1W<MstpcrcSpec> {
        Mstpc1W::new(self, 1)
    }
    ///Bit 3 - Capacitive Touch Sensing Unit Module Stop
    #[inline(always)]
    pub fn mstpc3(&mut self) -> Mstpc3W<MstpcrcSpec> {
        Mstpc3W::new(self, 3)
    }
    ///Bit 8 - Serial Sound Interface Enhanced Module Stop
    #[inline(always)]
    pub fn mstpc8(&mut self) -> Mstpc8W<MstpcrcSpec> {
        Mstpc8W::new(self, 8)
    }
    ///Bit 12 - Secure Digital HOST IF / Multi Media Card 0 Module Stop
    #[inline(always)]
    pub fn mstpc12(&mut self) -> Mstpc12W<MstpcrcSpec> {
        Mstpc12W::new(self, 12)
    }
    ///Bit 13 - Data Operation Circuit Module Stop
    #[inline(always)]
    pub fn mstpc13(&mut self) -> Mstpc13W<MstpcrcSpec> {
        Mstpc13W::new(self, 13)
    }
    ///Bit 14 - Event Link Controller Module Stop
    #[inline(always)]
    pub fn mstpc14(&mut self) -> Mstpc14W<MstpcrcSpec> {
        Mstpc14W::new(self, 14)
    }
    ///Bit 27 - CANFD Module Stop
    #[inline(always)]
    pub fn mstpc27(&mut self) -> Mstpc27W<MstpcrcSpec> {
        Mstpc27W::new(self, 27)
    }
    ///Bit 31 - SCE9 Module Stop
    #[inline(always)]
    pub fn mstpc31(&mut self) -> Mstpc31W<MstpcrcSpec> {
        Mstpc31W::new(self, 31)
    }
}
/**Module Stop Control Register C

You can [`read`](crate::Reg::read) this register and get [`mstpcrc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mstpcrc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MstpcrcSpec;
impl crate::RegisterSpec for MstpcrcSpec {
    type Ux = u32;
}
///`read()` method returns [`mstpcrc::R`](R) reader structure
impl crate::Readable for MstpcrcSpec {}
///`write(|w| ..)` method takes [`mstpcrc::W`](W) writer structure
impl crate::Writable for MstpcrcSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MSTPCRC to value 0xffff_ffff
impl crate::Resettable for MstpcrcSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
