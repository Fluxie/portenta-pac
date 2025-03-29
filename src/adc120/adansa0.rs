///Register `ADANSA0` reader
pub type R = crate::R<Adansa0Spec>;
///Register `ADANSA0` writer
pub type W = crate::W<Adansa0Spec>;
/**A/D Conversion Channels Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansa00 {
    ///0: Do not select associated input channel.
    _0 = 0,
    ///1: Select associated input channel.
    _1 = 1,
}
impl From<Ansa00> for bool {
    #[inline(always)]
    fn from(variant: Ansa00) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSA00` reader - A/D Conversion Channels Select
pub type Ansa00R = crate::BitReader<Ansa00>;
impl Ansa00R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ansa00 {
        match self.bits {
            false => Ansa00::_0,
            true => Ansa00::_1,
        }
    }
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansa00::_0
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansa00::_1
    }
}
///Field `ANSA00` writer - A/D Conversion Channels Select
pub type Ansa00W<'a, REG> = crate::BitWriter<'a, REG, Ansa00>;
impl<'a, REG> Ansa00W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa00::_0)
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa00::_1)
    }
}
/**A/D Conversion Channels Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansa01 {
    ///0: Do not select associated input channel.
    _0 = 0,
    ///1: Select associated input channel.
    _1 = 1,
}
impl From<Ansa01> for bool {
    #[inline(always)]
    fn from(variant: Ansa01) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSA01` reader - A/D Conversion Channels Select
pub type Ansa01R = crate::BitReader<Ansa01>;
impl Ansa01R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ansa01 {
        match self.bits {
            false => Ansa01::_0,
            true => Ansa01::_1,
        }
    }
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansa01::_0
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansa01::_1
    }
}
///Field `ANSA01` writer - A/D Conversion Channels Select
pub type Ansa01W<'a, REG> = crate::BitWriter<'a, REG, Ansa01>;
impl<'a, REG> Ansa01W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa01::_0)
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa01::_1)
    }
}
/**A/D Conversion Channels Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansa02 {
    ///0: Do not select associated input channel.
    _0 = 0,
    ///1: Select associated input channel.
    _1 = 1,
}
impl From<Ansa02> for bool {
    #[inline(always)]
    fn from(variant: Ansa02) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSA02` reader - A/D Conversion Channels Select
pub type Ansa02R = crate::BitReader<Ansa02>;
impl Ansa02R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ansa02 {
        match self.bits {
            false => Ansa02::_0,
            true => Ansa02::_1,
        }
    }
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansa02::_0
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansa02::_1
    }
}
///Field `ANSA02` writer - A/D Conversion Channels Select
pub type Ansa02W<'a, REG> = crate::BitWriter<'a, REG, Ansa02>;
impl<'a, REG> Ansa02W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa02::_0)
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa02::_1)
    }
}
/**A/D Conversion Channels Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansa03 {
    ///0: Do not select associated input channel.
    _0 = 0,
    ///1: Select associated input channel.
    _1 = 1,
}
impl From<Ansa03> for bool {
    #[inline(always)]
    fn from(variant: Ansa03) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSA03` reader - A/D Conversion Channels Select
pub type Ansa03R = crate::BitReader<Ansa03>;
impl Ansa03R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ansa03 {
        match self.bits {
            false => Ansa03::_0,
            true => Ansa03::_1,
        }
    }
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansa03::_0
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansa03::_1
    }
}
///Field `ANSA03` writer - A/D Conversion Channels Select
pub type Ansa03W<'a, REG> = crate::BitWriter<'a, REG, Ansa03>;
impl<'a, REG> Ansa03W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa03::_0)
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa03::_1)
    }
}
/**A/D Conversion Channels Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansa04 {
    ///0: Do not select associated input channel.
    _0 = 0,
    ///1: Select associated input channel.
    _1 = 1,
}
impl From<Ansa04> for bool {
    #[inline(always)]
    fn from(variant: Ansa04) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSA04` reader - A/D Conversion Channels Select
pub type Ansa04R = crate::BitReader<Ansa04>;
impl Ansa04R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ansa04 {
        match self.bits {
            false => Ansa04::_0,
            true => Ansa04::_1,
        }
    }
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansa04::_0
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansa04::_1
    }
}
///Field `ANSA04` writer - A/D Conversion Channels Select
pub type Ansa04W<'a, REG> = crate::BitWriter<'a, REG, Ansa04>;
impl<'a, REG> Ansa04W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa04::_0)
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa04::_1)
    }
}
/**A/D Conversion Channels Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansa05 {
    ///0: Do not select associated input channel.
    _0 = 0,
    ///1: Select associated input channel.
    _1 = 1,
}
impl From<Ansa05> for bool {
    #[inline(always)]
    fn from(variant: Ansa05) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSA05` reader - A/D Conversion Channels Select
pub type Ansa05R = crate::BitReader<Ansa05>;
impl Ansa05R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ansa05 {
        match self.bits {
            false => Ansa05::_0,
            true => Ansa05::_1,
        }
    }
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansa05::_0
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansa05::_1
    }
}
///Field `ANSA05` writer - A/D Conversion Channels Select
pub type Ansa05W<'a, REG> = crate::BitWriter<'a, REG, Ansa05>;
impl<'a, REG> Ansa05W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa05::_0)
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa05::_1)
    }
}
/**A/D Conversion Channels Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansa06 {
    ///0: Do not select associated input channel.
    _0 = 0,
    ///1: Select associated input channel.
    _1 = 1,
}
impl From<Ansa06> for bool {
    #[inline(always)]
    fn from(variant: Ansa06) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSA06` reader - A/D Conversion Channels Select
pub type Ansa06R = crate::BitReader<Ansa06>;
impl Ansa06R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ansa06 {
        match self.bits {
            false => Ansa06::_0,
            true => Ansa06::_1,
        }
    }
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansa06::_0
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansa06::_1
    }
}
///Field `ANSA06` writer - A/D Conversion Channels Select
pub type Ansa06W<'a, REG> = crate::BitWriter<'a, REG, Ansa06>;
impl<'a, REG> Ansa06W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa06::_0)
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa06::_1)
    }
}
/**A/D Conversion Channels Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansa07 {
    ///0: Do not select associated input channel.
    _0 = 0,
    ///1: Select associated input channel.
    _1 = 1,
}
impl From<Ansa07> for bool {
    #[inline(always)]
    fn from(variant: Ansa07) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSA07` reader - A/D Conversion Channels Select
pub type Ansa07R = crate::BitReader<Ansa07>;
impl Ansa07R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ansa07 {
        match self.bits {
            false => Ansa07::_0,
            true => Ansa07::_1,
        }
    }
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansa07::_0
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansa07::_1
    }
}
///Field `ANSA07` writer - A/D Conversion Channels Select
pub type Ansa07W<'a, REG> = crate::BitWriter<'a, REG, Ansa07>;
impl<'a, REG> Ansa07W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa07::_0)
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa07::_1)
    }
}
/**A/D Conversion Channels Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansa08 {
    ///0: Do not select associated input channel.
    _0 = 0,
    ///1: Select associated input channel.
    _1 = 1,
}
impl From<Ansa08> for bool {
    #[inline(always)]
    fn from(variant: Ansa08) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSA08` reader - A/D Conversion Channels Select
pub type Ansa08R = crate::BitReader<Ansa08>;
impl Ansa08R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ansa08 {
        match self.bits {
            false => Ansa08::_0,
            true => Ansa08::_1,
        }
    }
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansa08::_0
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansa08::_1
    }
}
///Field `ANSA08` writer - A/D Conversion Channels Select
pub type Ansa08W<'a, REG> = crate::BitWriter<'a, REG, Ansa08>;
impl<'a, REG> Ansa08W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa08::_0)
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa08::_1)
    }
}
/**A/D Conversion Channels Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansa09 {
    ///0: Do not select associated input channel.
    _0 = 0,
    ///1: Select associated input channel.
    _1 = 1,
}
impl From<Ansa09> for bool {
    #[inline(always)]
    fn from(variant: Ansa09) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSA09` reader - A/D Conversion Channels Select
pub type Ansa09R = crate::BitReader<Ansa09>;
impl Ansa09R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ansa09 {
        match self.bits {
            false => Ansa09::_0,
            true => Ansa09::_1,
        }
    }
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansa09::_0
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansa09::_1
    }
}
///Field `ANSA09` writer - A/D Conversion Channels Select
pub type Ansa09W<'a, REG> = crate::BitWriter<'a, REG, Ansa09>;
impl<'a, REG> Ansa09W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa09::_0)
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa09::_1)
    }
}
/**A/D Conversion Channels Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansa10 {
    ///0: Do not select associated input channel.
    _0 = 0,
    ///1: Select associated input channel.
    _1 = 1,
}
impl From<Ansa10> for bool {
    #[inline(always)]
    fn from(variant: Ansa10) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSA10` reader - A/D Conversion Channels Select
pub type Ansa10R = crate::BitReader<Ansa10>;
impl Ansa10R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ansa10 {
        match self.bits {
            false => Ansa10::_0,
            true => Ansa10::_1,
        }
    }
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansa10::_0
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansa10::_1
    }
}
///Field `ANSA10` writer - A/D Conversion Channels Select
pub type Ansa10W<'a, REG> = crate::BitWriter<'a, REG, Ansa10>;
impl<'a, REG> Ansa10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa10::_0)
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa10::_1)
    }
}
/**A/D Conversion Channels Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansa12 {
    ///0: Do not select associated input channel.
    _0 = 0,
    ///1: Select associated input channel.
    _1 = 1,
}
impl From<Ansa12> for bool {
    #[inline(always)]
    fn from(variant: Ansa12) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSA12` reader - A/D Conversion Channels Select
pub type Ansa12R = crate::BitReader<Ansa12>;
impl Ansa12R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ansa12 {
        match self.bits {
            false => Ansa12::_0,
            true => Ansa12::_1,
        }
    }
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansa12::_0
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansa12::_1
    }
}
///Field `ANSA12` writer - A/D Conversion Channels Select
pub type Ansa12W<'a, REG> = crate::BitWriter<'a, REG, Ansa12>;
impl<'a, REG> Ansa12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa12::_0)
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa12::_1)
    }
}
/**A/D Conversion Channels Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ansa13 {
    ///0: Do not select associated input channel.
    _0 = 0,
    ///1: Select associated input channel.
    _1 = 1,
}
impl From<Ansa13> for bool {
    #[inline(always)]
    fn from(variant: Ansa13) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSA13` reader - A/D Conversion Channels Select
pub type Ansa13R = crate::BitReader<Ansa13>;
impl Ansa13R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ansa13 {
        match self.bits {
            false => Ansa13::_0,
            true => Ansa13::_1,
        }
    }
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ansa13::_0
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ansa13::_1
    }
}
///Field `ANSA13` writer - A/D Conversion Channels Select
pub type Ansa13W<'a, REG> = crate::BitWriter<'a, REG, Ansa13>;
impl<'a, REG> Ansa13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not select associated input channel.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa13::_0)
    }
    ///Select associated input channel.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ansa13::_1)
    }
}
impl R {
    ///Bit 0 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansa00(&self) -> Ansa00R {
        Ansa00R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansa01(&self) -> Ansa01R {
        Ansa01R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansa02(&self) -> Ansa02R {
        Ansa02R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansa03(&self) -> Ansa03R {
        Ansa03R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansa04(&self) -> Ansa04R {
        Ansa04R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansa05(&self) -> Ansa05R {
        Ansa05R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansa06(&self) -> Ansa06R {
        Ansa06R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansa07(&self) -> Ansa07R {
        Ansa07R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansa08(&self) -> Ansa08R {
        Ansa08R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansa09(&self) -> Ansa09R {
        Ansa09R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansa10(&self) -> Ansa10R {
        Ansa10R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 12 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansa12(&self) -> Ansa12R {
        Ansa12R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansa13(&self) -> Ansa13R {
        Ansa13R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADANSA0")
            .field("ansa00", &self.ansa00())
            .field("ansa01", &self.ansa01())
            .field("ansa02", &self.ansa02())
            .field("ansa03", &self.ansa03())
            .field("ansa04", &self.ansa04())
            .field("ansa05", &self.ansa05())
            .field("ansa06", &self.ansa06())
            .field("ansa07", &self.ansa07())
            .field("ansa08", &self.ansa08())
            .field("ansa09", &self.ansa09())
            .field("ansa10", &self.ansa10())
            .field("ansa12", &self.ansa12())
            .field("ansa13", &self.ansa13())
            .finish()
    }
}
impl W {
    ///Bit 0 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansa00(&mut self) -> Ansa00W<Adansa0Spec> {
        Ansa00W::new(self, 0)
    }
    ///Bit 1 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansa01(&mut self) -> Ansa01W<Adansa0Spec> {
        Ansa01W::new(self, 1)
    }
    ///Bit 2 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansa02(&mut self) -> Ansa02W<Adansa0Spec> {
        Ansa02W::new(self, 2)
    }
    ///Bit 3 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansa03(&mut self) -> Ansa03W<Adansa0Spec> {
        Ansa03W::new(self, 3)
    }
    ///Bit 4 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansa04(&mut self) -> Ansa04W<Adansa0Spec> {
        Ansa04W::new(self, 4)
    }
    ///Bit 5 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansa05(&mut self) -> Ansa05W<Adansa0Spec> {
        Ansa05W::new(self, 5)
    }
    ///Bit 6 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansa06(&mut self) -> Ansa06W<Adansa0Spec> {
        Ansa06W::new(self, 6)
    }
    ///Bit 7 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansa07(&mut self) -> Ansa07W<Adansa0Spec> {
        Ansa07W::new(self, 7)
    }
    ///Bit 8 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansa08(&mut self) -> Ansa08W<Adansa0Spec> {
        Ansa08W::new(self, 8)
    }
    ///Bit 9 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansa09(&mut self) -> Ansa09W<Adansa0Spec> {
        Ansa09W::new(self, 9)
    }
    ///Bit 10 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansa10(&mut self) -> Ansa10W<Adansa0Spec> {
        Ansa10W::new(self, 10)
    }
    ///Bit 12 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansa12(&mut self) -> Ansa12W<Adansa0Spec> {
        Ansa12W::new(self, 12)
    }
    ///Bit 13 - A/D Conversion Channels Select
    #[inline(always)]
    pub fn ansa13(&mut self) -> Ansa13W<Adansa0Spec> {
        Ansa13W::new(self, 13)
    }
}
/**A/D Channel Select Register A0

You can [`read`](crate::Reg::read) this register and get [`adansa0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adansa0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Adansa0Spec;
impl crate::RegisterSpec for Adansa0Spec {
    type Ux = u16;
}
///`read()` method returns [`adansa0::R`](R) reader structure
impl crate::Readable for Adansa0Spec {}
///`write(|w| ..)` method takes [`adansa0::W`](W) writer structure
impl crate::Writable for Adansa0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADANSA0 to value 0
impl crate::Resettable for Adansa0Spec {}
