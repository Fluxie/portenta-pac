///Register `BBFSAR` reader
pub type R = crate::R<BbfsarSpec>;
///Register `BBFSAR` writer
pub type W = crate::W<BbfsarSpec>;
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
/**Non Secure Attribute bit 16

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nonsec16 {
    ///0: Secure
    _0 = 0,
    ///1: Non Secure
    _1 = 1,
}
impl From<Nonsec16> for bool {
    #[inline(always)]
    fn from(variant: Nonsec16) -> Self {
        variant as u8 != 0
    }
}
///Field `NONSEC16` reader - Non Secure Attribute bit 16
pub type Nonsec16R = crate::BitReader<Nonsec16>;
impl Nonsec16R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Nonsec16 {
        match self.bits {
            false => Nonsec16::_0,
            true => Nonsec16::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Nonsec16::_0
    }
    ///Non Secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Nonsec16::_1
    }
}
///Field `NONSEC16` writer - Non Secure Attribute bit 16
pub type Nonsec16W<'a, REG> = crate::BitWriter<'a, REG, Nonsec16>;
impl<'a, REG> Nonsec16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Nonsec16::_0)
    }
    ///Non Secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Nonsec16::_1)
    }
}
/**Non Secure Attribute bit 17

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nonsec17 {
    ///0: Secure
    _0 = 0,
    ///1: Non Secure
    _1 = 1,
}
impl From<Nonsec17> for bool {
    #[inline(always)]
    fn from(variant: Nonsec17) -> Self {
        variant as u8 != 0
    }
}
///Field `NONSEC17` reader - Non Secure Attribute bit 17
pub type Nonsec17R = crate::BitReader<Nonsec17>;
impl Nonsec17R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Nonsec17 {
        match self.bits {
            false => Nonsec17::_0,
            true => Nonsec17::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Nonsec17::_0
    }
    ///Non Secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Nonsec17::_1
    }
}
///Field `NONSEC17` writer - Non Secure Attribute bit 17
pub type Nonsec17W<'a, REG> = crate::BitWriter<'a, REG, Nonsec17>;
impl<'a, REG> Nonsec17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Nonsec17::_0)
    }
    ///Non Secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Nonsec17::_1)
    }
}
/**Non Secure Attribute bit 18

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nonsec18 {
    ///0: Secure
    _0 = 0,
    ///1: Non Secure
    _1 = 1,
}
impl From<Nonsec18> for bool {
    #[inline(always)]
    fn from(variant: Nonsec18) -> Self {
        variant as u8 != 0
    }
}
///Field `NONSEC18` reader - Non Secure Attribute bit 18
pub type Nonsec18R = crate::BitReader<Nonsec18>;
impl Nonsec18R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Nonsec18 {
        match self.bits {
            false => Nonsec18::_0,
            true => Nonsec18::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Nonsec18::_0
    }
    ///Non Secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Nonsec18::_1
    }
}
///Field `NONSEC18` writer - Non Secure Attribute bit 18
pub type Nonsec18W<'a, REG> = crate::BitWriter<'a, REG, Nonsec18>;
impl<'a, REG> Nonsec18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Nonsec18::_0)
    }
    ///Non Secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Nonsec18::_1)
    }
}
/**Non Secure Attribute bit 19

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nonsec19 {
    ///0: Secure
    _0 = 0,
    ///1: Non Secure
    _1 = 1,
}
impl From<Nonsec19> for bool {
    #[inline(always)]
    fn from(variant: Nonsec19) -> Self {
        variant as u8 != 0
    }
}
///Field `NONSEC19` reader - Non Secure Attribute bit 19
pub type Nonsec19R = crate::BitReader<Nonsec19>;
impl Nonsec19R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Nonsec19 {
        match self.bits {
            false => Nonsec19::_0,
            true => Nonsec19::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Nonsec19::_0
    }
    ///Non Secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Nonsec19::_1
    }
}
///Field `NONSEC19` writer - Non Secure Attribute bit 19
pub type Nonsec19W<'a, REG> = crate::BitWriter<'a, REG, Nonsec19>;
impl<'a, REG> Nonsec19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Nonsec19::_0)
    }
    ///Non Secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Nonsec19::_1)
    }
}
/**Non Secure Attribute bit 20

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nonsec20 {
    ///0: Secure
    _0 = 0,
    ///1: Non Secure
    _1 = 1,
}
impl From<Nonsec20> for bool {
    #[inline(always)]
    fn from(variant: Nonsec20) -> Self {
        variant as u8 != 0
    }
}
///Field `NONSEC20` reader - Non Secure Attribute bit 20
pub type Nonsec20R = crate::BitReader<Nonsec20>;
impl Nonsec20R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Nonsec20 {
        match self.bits {
            false => Nonsec20::_0,
            true => Nonsec20::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Nonsec20::_0
    }
    ///Non Secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Nonsec20::_1
    }
}
///Field `NONSEC20` writer - Non Secure Attribute bit 20
pub type Nonsec20W<'a, REG> = crate::BitWriter<'a, REG, Nonsec20>;
impl<'a, REG> Nonsec20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Nonsec20::_0)
    }
    ///Non Secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Nonsec20::_1)
    }
}
/**Non Secure Attribute bit 21

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nonsec21 {
    ///0: Secure
    _0 = 0,
    ///1: Non Secure
    _1 = 1,
}
impl From<Nonsec21> for bool {
    #[inline(always)]
    fn from(variant: Nonsec21) -> Self {
        variant as u8 != 0
    }
}
///Field `NONSEC21` reader - Non Secure Attribute bit 21
pub type Nonsec21R = crate::BitReader<Nonsec21>;
impl Nonsec21R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Nonsec21 {
        match self.bits {
            false => Nonsec21::_0,
            true => Nonsec21::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Nonsec21::_0
    }
    ///Non Secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Nonsec21::_1
    }
}
///Field `NONSEC21` writer - Non Secure Attribute bit 21
pub type Nonsec21W<'a, REG> = crate::BitWriter<'a, REG, Nonsec21>;
impl<'a, REG> Nonsec21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Nonsec21::_0)
    }
    ///Non Secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Nonsec21::_1)
    }
}
/**Non Secure Attribute bit 22

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nonsec22 {
    ///0: Secure
    _0 = 0,
    ///1: Non Secure
    _1 = 1,
}
impl From<Nonsec22> for bool {
    #[inline(always)]
    fn from(variant: Nonsec22) -> Self {
        variant as u8 != 0
    }
}
///Field `NONSEC22` reader - Non Secure Attribute bit 22
pub type Nonsec22R = crate::BitReader<Nonsec22>;
impl Nonsec22R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Nonsec22 {
        match self.bits {
            false => Nonsec22::_0,
            true => Nonsec22::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Nonsec22::_0
    }
    ///Non Secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Nonsec22::_1
    }
}
///Field `NONSEC22` writer - Non Secure Attribute bit 22
pub type Nonsec22W<'a, REG> = crate::BitWriter<'a, REG, Nonsec22>;
impl<'a, REG> Nonsec22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Nonsec22::_0)
    }
    ///Non Secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Nonsec22::_1)
    }
}
/**Non Secure Attribute bit 23

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nonsec23 {
    ///0: Secure
    _0 = 0,
    ///1: Non Secure
    _1 = 1,
}
impl From<Nonsec23> for bool {
    #[inline(always)]
    fn from(variant: Nonsec23) -> Self {
        variant as u8 != 0
    }
}
///Field `NONSEC23` reader - Non Secure Attribute bit 23
pub type Nonsec23R = crate::BitReader<Nonsec23>;
impl Nonsec23R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Nonsec23 {
        match self.bits {
            false => Nonsec23::_0,
            true => Nonsec23::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Nonsec23::_0
    }
    ///Non Secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Nonsec23::_1
    }
}
///Field `NONSEC23` writer - Non Secure Attribute bit 23
pub type Nonsec23W<'a, REG> = crate::BitWriter<'a, REG, Nonsec23>;
impl<'a, REG> Nonsec23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Nonsec23::_0)
    }
    ///Non Secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Nonsec23::_1)
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
    ///Bit 2 - Non Secure Attribute bit 2
    #[inline(always)]
    pub fn nonsec2(&self) -> Nonsec2R {
        Nonsec2R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 16 - Non Secure Attribute bit 16
    #[inline(always)]
    pub fn nonsec16(&self) -> Nonsec16R {
        Nonsec16R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Non Secure Attribute bit 17
    #[inline(always)]
    pub fn nonsec17(&self) -> Nonsec17R {
        Nonsec17R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Non Secure Attribute bit 18
    #[inline(always)]
    pub fn nonsec18(&self) -> Nonsec18R {
        Nonsec18R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Non Secure Attribute bit 19
    #[inline(always)]
    pub fn nonsec19(&self) -> Nonsec19R {
        Nonsec19R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Non Secure Attribute bit 20
    #[inline(always)]
    pub fn nonsec20(&self) -> Nonsec20R {
        Nonsec20R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Non Secure Attribute bit 21
    #[inline(always)]
    pub fn nonsec21(&self) -> Nonsec21R {
        Nonsec21R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Non Secure Attribute bit 22
    #[inline(always)]
    pub fn nonsec22(&self) -> Nonsec22R {
        Nonsec22R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Non Secure Attribute bit 23
    #[inline(always)]
    pub fn nonsec23(&self) -> Nonsec23R {
        Nonsec23R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BBFSAR")
            .field("nonsec0", &self.nonsec0())
            .field("nonsec1", &self.nonsec1())
            .field("nonsec2", &self.nonsec2())
            .field("nonsec16", &self.nonsec16())
            .field("nonsec17", &self.nonsec17())
            .field("nonsec18", &self.nonsec18())
            .field("nonsec19", &self.nonsec19())
            .field("nonsec20", &self.nonsec20())
            .field("nonsec21", &self.nonsec21())
            .field("nonsec22", &self.nonsec22())
            .field("nonsec23", &self.nonsec23())
            .finish()
    }
}
impl W {
    ///Bit 0 - Non Secure Attribute bit 0
    #[inline(always)]
    pub fn nonsec0(&mut self) -> Nonsec0W<BbfsarSpec> {
        Nonsec0W::new(self, 0)
    }
    ///Bit 1 - Non Secure Attribute bit 1
    #[inline(always)]
    pub fn nonsec1(&mut self) -> Nonsec1W<BbfsarSpec> {
        Nonsec1W::new(self, 1)
    }
    ///Bit 2 - Non Secure Attribute bit 2
    #[inline(always)]
    pub fn nonsec2(&mut self) -> Nonsec2W<BbfsarSpec> {
        Nonsec2W::new(self, 2)
    }
    ///Bit 16 - Non Secure Attribute bit 16
    #[inline(always)]
    pub fn nonsec16(&mut self) -> Nonsec16W<BbfsarSpec> {
        Nonsec16W::new(self, 16)
    }
    ///Bit 17 - Non Secure Attribute bit 17
    #[inline(always)]
    pub fn nonsec17(&mut self) -> Nonsec17W<BbfsarSpec> {
        Nonsec17W::new(self, 17)
    }
    ///Bit 18 - Non Secure Attribute bit 18
    #[inline(always)]
    pub fn nonsec18(&mut self) -> Nonsec18W<BbfsarSpec> {
        Nonsec18W::new(self, 18)
    }
    ///Bit 19 - Non Secure Attribute bit 19
    #[inline(always)]
    pub fn nonsec19(&mut self) -> Nonsec19W<BbfsarSpec> {
        Nonsec19W::new(self, 19)
    }
    ///Bit 20 - Non Secure Attribute bit 20
    #[inline(always)]
    pub fn nonsec20(&mut self) -> Nonsec20W<BbfsarSpec> {
        Nonsec20W::new(self, 20)
    }
    ///Bit 21 - Non Secure Attribute bit 21
    #[inline(always)]
    pub fn nonsec21(&mut self) -> Nonsec21W<BbfsarSpec> {
        Nonsec21W::new(self, 21)
    }
    ///Bit 22 - Non Secure Attribute bit 22
    #[inline(always)]
    pub fn nonsec22(&mut self) -> Nonsec22W<BbfsarSpec> {
        Nonsec22W::new(self, 22)
    }
    ///Bit 23 - Non Secure Attribute bit 23
    #[inline(always)]
    pub fn nonsec23(&mut self) -> Nonsec23W<BbfsarSpec> {
        Nonsec23W::new(self, 23)
    }
}
/**Battery Backup Function Security Attribute Register

You can [`read`](crate::Reg::read) this register and get [`bbfsar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bbfsar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BbfsarSpec;
impl crate::RegisterSpec for BbfsarSpec {
    type Ux = u32;
}
///`read()` method returns [`bbfsar::R`](R) reader structure
impl crate::Readable for BbfsarSpec {}
///`write(|w| ..)` method takes [`bbfsar::W`](W) writer structure
impl crate::Writable for BbfsarSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BBFSAR to value 0xffff
impl crate::Resettable for BbfsarSpec {
    const RESET_VALUE: u32 = 0xffff;
}
