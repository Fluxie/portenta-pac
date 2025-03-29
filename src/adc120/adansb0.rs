///Register `ADANSB0` reader
pub type R = crate::R<Adansb0Spec>;
///Register `ADANSB0` writer
pub type W = crate::W<Adansb0Spec>;
/**A/D Conversion Channels Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansb00 {
    ///0: Do not select associated input channel.
    _0 = 0,
    ///1: Select associated input channel.
    _1 = 1,
}
impl From<Ansb00> for bool {
    #[inline(always)]
    fn from(variant: Ansb00) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSB00` reader - A/D Conversion Channels Select
pub type Ansb00R = crate::BitReader<Ansb00>;
impl Ansb00R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ansb00 {
        match self.bits {
            false => Ansb00::_0,
            true => Ansb00::_1,
        }
    }
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansb00::_0
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansb00::_1
    }
}
///Field `ANSB00` writer - A/D Conversion Channels Select
pub type Ansb00W<'a, REG> = crate::BitWriter<'a, REG, Ansb00>;
impl<'a, REG> Ansb00W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb00::_0)
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb00::_1)
    }
}
/**A/D Conversion Channels Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansb01 {
    ///0: Do not select associated input channel.
    _0 = 0,
    ///1: Select associated input channel.
    _1 = 1,
}
impl From<Ansb01> for bool {
    #[inline(always)]
    fn from(variant: Ansb01) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSB01` reader - A/D Conversion Channels Select
pub type Ansb01R = crate::BitReader<Ansb01>;
impl Ansb01R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ansb01 {
        match self.bits {
            false => Ansb01::_0,
            true => Ansb01::_1,
        }
    }
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansb01::_0
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansb01::_1
    }
}
///Field `ANSB01` writer - A/D Conversion Channels Select
pub type Ansb01W<'a, REG> = crate::BitWriter<'a, REG, Ansb01>;
impl<'a, REG> Ansb01W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb01::_0)
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb01::_1)
    }
}
/**A/D Conversion Channels Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansb02 {
    ///0: Do not select associated input channel.
    _0 = 0,
    ///1: Select associated input channel.
    _1 = 1,
}
impl From<Ansb02> for bool {
    #[inline(always)]
    fn from(variant: Ansb02) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSB02` reader - A/D Conversion Channels Select
pub type Ansb02R = crate::BitReader<Ansb02>;
impl Ansb02R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ansb02 {
        match self.bits {
            false => Ansb02::_0,
            true => Ansb02::_1,
        }
    }
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansb02::_0
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansb02::_1
    }
}
///Field `ANSB02` writer - A/D Conversion Channels Select
pub type Ansb02W<'a, REG> = crate::BitWriter<'a, REG, Ansb02>;
impl<'a, REG> Ansb02W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb02::_0)
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb02::_1)
    }
}
/**A/D Conversion Channels Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansb03 {
    ///0: Do not select associated input channel.
    _0 = 0,
    ///1: Select associated input channel.
    _1 = 1,
}
impl From<Ansb03> for bool {
    #[inline(always)]
    fn from(variant: Ansb03) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSB03` reader - A/D Conversion Channels Select
pub type Ansb03R = crate::BitReader<Ansb03>;
impl Ansb03R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ansb03 {
        match self.bits {
            false => Ansb03::_0,
            true => Ansb03::_1,
        }
    }
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansb03::_0
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansb03::_1
    }
}
///Field `ANSB03` writer - A/D Conversion Channels Select
pub type Ansb03W<'a, REG> = crate::BitWriter<'a, REG, Ansb03>;
impl<'a, REG> Ansb03W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb03::_0)
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb03::_1)
    }
}
/**A/D Conversion Channels Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansb04 {
    ///0: Do not select associated input channel.
    _0 = 0,
    ///1: Select associated input channel.
    _1 = 1,
}
impl From<Ansb04> for bool {
    #[inline(always)]
    fn from(variant: Ansb04) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSB04` reader - A/D Conversion Channels Select
pub type Ansb04R = crate::BitReader<Ansb04>;
impl Ansb04R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ansb04 {
        match self.bits {
            false => Ansb04::_0,
            true => Ansb04::_1,
        }
    }
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansb04::_0
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansb04::_1
    }
}
///Field `ANSB04` writer - A/D Conversion Channels Select
pub type Ansb04W<'a, REG> = crate::BitWriter<'a, REG, Ansb04>;
impl<'a, REG> Ansb04W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb04::_0)
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb04::_1)
    }
}
/**A/D Conversion Channels Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansb05 {
    ///0: Do not select associated input channel.
    _0 = 0,
    ///1: Select associated input channel.
    _1 = 1,
}
impl From<Ansb05> for bool {
    #[inline(always)]
    fn from(variant: Ansb05) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSB05` reader - A/D Conversion Channels Select
pub type Ansb05R = crate::BitReader<Ansb05>;
impl Ansb05R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ansb05 {
        match self.bits {
            false => Ansb05::_0,
            true => Ansb05::_1,
        }
    }
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansb05::_0
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansb05::_1
    }
}
///Field `ANSB05` writer - A/D Conversion Channels Select
pub type Ansb05W<'a, REG> = crate::BitWriter<'a, REG, Ansb05>;
impl<'a, REG> Ansb05W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb05::_0)
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb05::_1)
    }
}
/**A/D Conversion Channels Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansb06 {
    ///0: Do not select associated input channel.
    _0 = 0,
    ///1: Select associated input channel.
    _1 = 1,
}
impl From<Ansb06> for bool {
    #[inline(always)]
    fn from(variant: Ansb06) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSB06` reader - A/D Conversion Channels Select
pub type Ansb06R = crate::BitReader<Ansb06>;
impl Ansb06R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ansb06 {
        match self.bits {
            false => Ansb06::_0,
            true => Ansb06::_1,
        }
    }
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansb06::_0
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansb06::_1
    }
}
///Field `ANSB06` writer - A/D Conversion Channels Select
pub type Ansb06W<'a, REG> = crate::BitWriter<'a, REG, Ansb06>;
impl<'a, REG> Ansb06W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb06::_0)
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb06::_1)
    }
}
/**A/D Conversion Channels Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansb07 {
    ///0: Do not select associated input channel.
    _0 = 0,
    ///1: Select associated input channel.
    _1 = 1,
}
impl From<Ansb07> for bool {
    #[inline(always)]
    fn from(variant: Ansb07) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSB07` reader - A/D Conversion Channels Select
pub type Ansb07R = crate::BitReader<Ansb07>;
impl Ansb07R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ansb07 {
        match self.bits {
            false => Ansb07::_0,
            true => Ansb07::_1,
        }
    }
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansb07::_0
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansb07::_1
    }
}
///Field `ANSB07` writer - A/D Conversion Channels Select
pub type Ansb07W<'a, REG> = crate::BitWriter<'a, REG, Ansb07>;
impl<'a, REG> Ansb07W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb07::_0)
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb07::_1)
    }
}
/**A/D Conversion Channels Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansb08 {
    ///0: Do not select associated input channel.
    _0 = 0,
    ///1: Select associated input channel.
    _1 = 1,
}
impl From<Ansb08> for bool {
    #[inline(always)]
    fn from(variant: Ansb08) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSB08` reader - A/D Conversion Channels Select
pub type Ansb08R = crate::BitReader<Ansb08>;
impl Ansb08R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ansb08 {
        match self.bits {
            false => Ansb08::_0,
            true => Ansb08::_1,
        }
    }
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansb08::_0
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansb08::_1
    }
}
///Field `ANSB08` writer - A/D Conversion Channels Select
pub type Ansb08W<'a, REG> = crate::BitWriter<'a, REG, Ansb08>;
impl<'a, REG> Ansb08W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb08::_0)
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb08::_1)
    }
}
/**A/D Conversion Channels Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansb09 {
    ///0: Do not select associated input channel.
    _0 = 0,
    ///1: Select associated input channel.
    _1 = 1,
}
impl From<Ansb09> for bool {
    #[inline(always)]
    fn from(variant: Ansb09) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSB09` reader - A/D Conversion Channels Select
pub type Ansb09R = crate::BitReader<Ansb09>;
impl Ansb09R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ansb09 {
        match self.bits {
            false => Ansb09::_0,
            true => Ansb09::_1,
        }
    }
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansb09::_0
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansb09::_1
    }
}
///Field `ANSB09` writer - A/D Conversion Channels Select
pub type Ansb09W<'a, REG> = crate::BitWriter<'a, REG, Ansb09>;
impl<'a, REG> Ansb09W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb09::_0)
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb09::_1)
    }
}
/**A/D Conversion Channels Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansb10 {
    ///0: Do not select associated input channel.
    _0 = 0,
    ///1: Select associated input channel.
    _1 = 1,
}
impl From<Ansb10> for bool {
    #[inline(always)]
    fn from(variant: Ansb10) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSB10` reader - A/D Conversion Channels Select
pub type Ansb10R = crate::BitReader<Ansb10>;
impl Ansb10R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ansb10 {
        match self.bits {
            false => Ansb10::_0,
            true => Ansb10::_1,
        }
    }
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansb10::_0
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansb10::_1
    }
}
///Field `ANSB10` writer - A/D Conversion Channels Select
pub type Ansb10W<'a, REG> = crate::BitWriter<'a, REG, Ansb10>;
impl<'a, REG> Ansb10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb10::_0)
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb10::_1)
    }
}
/**A/D Conversion Channels Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansb12 {
    ///0: Do not select associated input channel.
    _0 = 0,
    ///1: Select associated input channel.
    _1 = 1,
}
impl From<Ansb12> for bool {
    #[inline(always)]
    fn from(variant: Ansb12) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSB12` reader - A/D Conversion Channels Select
pub type Ansb12R = crate::BitReader<Ansb12>;
impl Ansb12R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ansb12 {
        match self.bits {
            false => Ansb12::_0,
            true => Ansb12::_1,
        }
    }
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansb12::_0
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansb12::_1
    }
}
///Field `ANSB12` writer - A/D Conversion Channels Select
pub type Ansb12W<'a, REG> = crate::BitWriter<'a, REG, Ansb12>;
impl<'a, REG> Ansb12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb12::_0)
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb12::_1)
    }
}
/**A/D Conversion Channels Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansb13 {
    ///0: Do not select associated input channel.
    _0 = 0,
    ///1: Select associated input channel.
    _1 = 1,
}
impl From<Ansb13> for bool {
    #[inline(always)]
    fn from(variant: Ansb13) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSB13` reader - A/D Conversion Channels Select
pub type Ansb13R = crate::BitReader<Ansb13>;
impl Ansb13R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ansb13 {
        match self.bits {
            false => Ansb13::_0,
            true => Ansb13::_1,
        }
    }
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansb13::_0
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansb13::_1
    }
}
///Field `ANSB13` writer - A/D Conversion Channels Select
pub type Ansb13W<'a, REG> = crate::BitWriter<'a, REG, Ansb13>;
impl<'a, REG> Ansb13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb13::_0)
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansb13::_1)
    }
}
impl R {
    ///Bit 0 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansb00(&self) -> Ansb00R {
        Ansb00R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansb01(&self) -> Ansb01R {
        Ansb01R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansb02(&self) -> Ansb02R {
        Ansb02R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansb03(&self) -> Ansb03R {
        Ansb03R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansb04(&self) -> Ansb04R {
        Ansb04R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansb05(&self) -> Ansb05R {
        Ansb05R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansb06(&self) -> Ansb06R {
        Ansb06R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansb07(&self) -> Ansb07R {
        Ansb07R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansb08(&self) -> Ansb08R {
        Ansb08R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansb09(&self) -> Ansb09R {
        Ansb09R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansb10(&self) -> Ansb10R {
        Ansb10R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 12 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansb12(&self) -> Ansb12R {
        Ansb12R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansb13(&self) -> Ansb13R {
        Ansb13R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADANSB0")
            .field("ansb00", &self.ansb00())
            .field("ansb01", &self.ansb01())
            .field("ansb02", &self.ansb02())
            .field("ansb03", &self.ansb03())
            .field("ansb04", &self.ansb04())
            .field("ansb05", &self.ansb05())
            .field("ansb06", &self.ansb06())
            .field("ansb07", &self.ansb07())
            .field("ansb08", &self.ansb08())
            .field("ansb09", &self.ansb09())
            .field("ansb10", &self.ansb10())
            .field("ansb12", &self.ansb12())
            .field("ansb13", &self.ansb13())
            .finish()
    }
}
impl W {
    ///Bit 0 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansb00(&mut self) -> Ansb00W<Adansb0Spec> {
        Ansb00W::new(self, 0)
    }
    ///Bit 1 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansb01(&mut self) -> Ansb01W<Adansb0Spec> {
        Ansb01W::new(self, 1)
    }
    ///Bit 2 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansb02(&mut self) -> Ansb02W<Adansb0Spec> {
        Ansb02W::new(self, 2)
    }
    ///Bit 3 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansb03(&mut self) -> Ansb03W<Adansb0Spec> {
        Ansb03W::new(self, 3)
    }
    ///Bit 4 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansb04(&mut self) -> Ansb04W<Adansb0Spec> {
        Ansb04W::new(self, 4)
    }
    ///Bit 5 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansb05(&mut self) -> Ansb05W<Adansb0Spec> {
        Ansb05W::new(self, 5)
    }
    ///Bit 6 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansb06(&mut self) -> Ansb06W<Adansb0Spec> {
        Ansb06W::new(self, 6)
    }
    ///Bit 7 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansb07(&mut self) -> Ansb07W<Adansb0Spec> {
        Ansb07W::new(self, 7)
    }
    ///Bit 8 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansb08(&mut self) -> Ansb08W<Adansb0Spec> {
        Ansb08W::new(self, 8)
    }
    ///Bit 9 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansb09(&mut self) -> Ansb09W<Adansb0Spec> {
        Ansb09W::new(self, 9)
    }
    ///Bit 10 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansb10(&mut self) -> Ansb10W<Adansb0Spec> {
        Ansb10W::new(self, 10)
    }
    ///Bit 12 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansb12(&mut self) -> Ansb12W<Adansb0Spec> {
        Ansb12W::new(self, 12)
    }
    ///Bit 13 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansb13(&mut self) -> Ansb13W<Adansb0Spec> {
        Ansb13W::new(self, 13)
    }
}
/**A/D Channel Select Register B0

You can [`read`](crate::Reg::read) this register and get [`adansb0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adansb0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Adansb0Spec;
impl crate::RegisterSpec for Adansb0Spec {
    type Ux = u16;
}
///`read()` method returns [`adansb0::R`](R) reader structure
impl crate::Readable for Adansb0Spec {}
///`write(|w| ..)` method takes [`adansb0::W`](W) writer structure
impl crate::Writable for Adansb0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADANSB0 to value 0
impl crate::Resettable for Adansb0Spec {}
