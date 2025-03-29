///Register `EORR` reader
pub type R = crate::R<EorrSpec>;
///Register `EORR` writer
pub type W = crate::W<EorrSpec>;
/**Pmn Event Output Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eorr00 {
    ///0: No effect on output
    _0 = 0,
    ///1: Low output
    _1 = 1,
}
impl From<Eorr00> for bool {
    #[inline(always)]
    fn from(variant: Eorr00) -> Self {
        variant as u8 != 0
    }
}
///Field `EORR00` reader - Pmn Event Output Reset
pub type Eorr00R = crate::BitReader<Eorr00>;
impl Eorr00R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eorr00 {
        match self.bits {
            false => Eorr00::_0,
            true => Eorr00::_1,
        }
    }
    ///No effect on output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eorr00::_0
    }
    ///Low output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eorr00::_1
    }
}
///Field `EORR00` writer - Pmn Event Output Reset
pub type Eorr00W<'a, REG> = crate::BitWriter<'a, REG, Eorr00>;
impl<'a, REG> Eorr00W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr00::_0)
    }
    ///Low output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr00::_1)
    }
}
/**Pmn Event Output Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eorr01 {
    ///0: No effect on output
    _0 = 0,
    ///1: Low output
    _1 = 1,
}
impl From<Eorr01> for bool {
    #[inline(always)]
    fn from(variant: Eorr01) -> Self {
        variant as u8 != 0
    }
}
///Field `EORR01` reader - Pmn Event Output Reset
pub type Eorr01R = crate::BitReader<Eorr01>;
impl Eorr01R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eorr01 {
        match self.bits {
            false => Eorr01::_0,
            true => Eorr01::_1,
        }
    }
    ///No effect on output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eorr01::_0
    }
    ///Low output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eorr01::_1
    }
}
///Field `EORR01` writer - Pmn Event Output Reset
pub type Eorr01W<'a, REG> = crate::BitWriter<'a, REG, Eorr01>;
impl<'a, REG> Eorr01W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr01::_0)
    }
    ///Low output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr01::_1)
    }
}
/**Pmn Event Output Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eorr02 {
    ///0: No effect on output
    _0 = 0,
    ///1: Low output
    _1 = 1,
}
impl From<Eorr02> for bool {
    #[inline(always)]
    fn from(variant: Eorr02) -> Self {
        variant as u8 != 0
    }
}
///Field `EORR02` reader - Pmn Event Output Reset
pub type Eorr02R = crate::BitReader<Eorr02>;
impl Eorr02R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eorr02 {
        match self.bits {
            false => Eorr02::_0,
            true => Eorr02::_1,
        }
    }
    ///No effect on output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eorr02::_0
    }
    ///Low output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eorr02::_1
    }
}
///Field `EORR02` writer - Pmn Event Output Reset
pub type Eorr02W<'a, REG> = crate::BitWriter<'a, REG, Eorr02>;
impl<'a, REG> Eorr02W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr02::_0)
    }
    ///Low output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr02::_1)
    }
}
/**Pmn Event Output Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eorr03 {
    ///0: No effect on output
    _0 = 0,
    ///1: Low output
    _1 = 1,
}
impl From<Eorr03> for bool {
    #[inline(always)]
    fn from(variant: Eorr03) -> Self {
        variant as u8 != 0
    }
}
///Field `EORR03` reader - Pmn Event Output Reset
pub type Eorr03R = crate::BitReader<Eorr03>;
impl Eorr03R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eorr03 {
        match self.bits {
            false => Eorr03::_0,
            true => Eorr03::_1,
        }
    }
    ///No effect on output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eorr03::_0
    }
    ///Low output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eorr03::_1
    }
}
///Field `EORR03` writer - Pmn Event Output Reset
pub type Eorr03W<'a, REG> = crate::BitWriter<'a, REG, Eorr03>;
impl<'a, REG> Eorr03W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr03::_0)
    }
    ///Low output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr03::_1)
    }
}
/**Pmn Event Output Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eorr04 {
    ///0: No effect on output
    _0 = 0,
    ///1: Low output
    _1 = 1,
}
impl From<Eorr04> for bool {
    #[inline(always)]
    fn from(variant: Eorr04) -> Self {
        variant as u8 != 0
    }
}
///Field `EORR04` reader - Pmn Event Output Reset
pub type Eorr04R = crate::BitReader<Eorr04>;
impl Eorr04R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eorr04 {
        match self.bits {
            false => Eorr04::_0,
            true => Eorr04::_1,
        }
    }
    ///No effect on output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eorr04::_0
    }
    ///Low output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eorr04::_1
    }
}
///Field `EORR04` writer - Pmn Event Output Reset
pub type Eorr04W<'a, REG> = crate::BitWriter<'a, REG, Eorr04>;
impl<'a, REG> Eorr04W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr04::_0)
    }
    ///Low output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr04::_1)
    }
}
/**Pmn Event Output Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eorr05 {
    ///0: No effect on output
    _0 = 0,
    ///1: Low output
    _1 = 1,
}
impl From<Eorr05> for bool {
    #[inline(always)]
    fn from(variant: Eorr05) -> Self {
        variant as u8 != 0
    }
}
///Field `EORR05` reader - Pmn Event Output Reset
pub type Eorr05R = crate::BitReader<Eorr05>;
impl Eorr05R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eorr05 {
        match self.bits {
            false => Eorr05::_0,
            true => Eorr05::_1,
        }
    }
    ///No effect on output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eorr05::_0
    }
    ///Low output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eorr05::_1
    }
}
///Field `EORR05` writer - Pmn Event Output Reset
pub type Eorr05W<'a, REG> = crate::BitWriter<'a, REG, Eorr05>;
impl<'a, REG> Eorr05W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr05::_0)
    }
    ///Low output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr05::_1)
    }
}
/**Pmn Event Output Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eorr06 {
    ///0: No effect on output
    _0 = 0,
    ///1: Low output
    _1 = 1,
}
impl From<Eorr06> for bool {
    #[inline(always)]
    fn from(variant: Eorr06) -> Self {
        variant as u8 != 0
    }
}
///Field `EORR06` reader - Pmn Event Output Reset
pub type Eorr06R = crate::BitReader<Eorr06>;
impl Eorr06R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eorr06 {
        match self.bits {
            false => Eorr06::_0,
            true => Eorr06::_1,
        }
    }
    ///No effect on output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eorr06::_0
    }
    ///Low output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eorr06::_1
    }
}
///Field `EORR06` writer - Pmn Event Output Reset
pub type Eorr06W<'a, REG> = crate::BitWriter<'a, REG, Eorr06>;
impl<'a, REG> Eorr06W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr06::_0)
    }
    ///Low output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr06::_1)
    }
}
/**Pmn Event Output Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eorr07 {
    ///0: No effect on output
    _0 = 0,
    ///1: Low output
    _1 = 1,
}
impl From<Eorr07> for bool {
    #[inline(always)]
    fn from(variant: Eorr07) -> Self {
        variant as u8 != 0
    }
}
///Field `EORR07` reader - Pmn Event Output Reset
pub type Eorr07R = crate::BitReader<Eorr07>;
impl Eorr07R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eorr07 {
        match self.bits {
            false => Eorr07::_0,
            true => Eorr07::_1,
        }
    }
    ///No effect on output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eorr07::_0
    }
    ///Low output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eorr07::_1
    }
}
///Field `EORR07` writer - Pmn Event Output Reset
pub type Eorr07W<'a, REG> = crate::BitWriter<'a, REG, Eorr07>;
impl<'a, REG> Eorr07W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr07::_0)
    }
    ///Low output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr07::_1)
    }
}
/**Pmn Event Output Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eorr08 {
    ///0: No effect on output
    _0 = 0,
    ///1: Low output
    _1 = 1,
}
impl From<Eorr08> for bool {
    #[inline(always)]
    fn from(variant: Eorr08) -> Self {
        variant as u8 != 0
    }
}
///Field `EORR08` reader - Pmn Event Output Reset
pub type Eorr08R = crate::BitReader<Eorr08>;
impl Eorr08R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eorr08 {
        match self.bits {
            false => Eorr08::_0,
            true => Eorr08::_1,
        }
    }
    ///No effect on output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eorr08::_0
    }
    ///Low output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eorr08::_1
    }
}
///Field `EORR08` writer - Pmn Event Output Reset
pub type Eorr08W<'a, REG> = crate::BitWriter<'a, REG, Eorr08>;
impl<'a, REG> Eorr08W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr08::_0)
    }
    ///Low output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr08::_1)
    }
}
/**Pmn Event Output Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eorr09 {
    ///0: No effect on output
    _0 = 0,
    ///1: Low output
    _1 = 1,
}
impl From<Eorr09> for bool {
    #[inline(always)]
    fn from(variant: Eorr09) -> Self {
        variant as u8 != 0
    }
}
///Field `EORR09` reader - Pmn Event Output Reset
pub type Eorr09R = crate::BitReader<Eorr09>;
impl Eorr09R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eorr09 {
        match self.bits {
            false => Eorr09::_0,
            true => Eorr09::_1,
        }
    }
    ///No effect on output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eorr09::_0
    }
    ///Low output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eorr09::_1
    }
}
///Field `EORR09` writer - Pmn Event Output Reset
pub type Eorr09W<'a, REG> = crate::BitWriter<'a, REG, Eorr09>;
impl<'a, REG> Eorr09W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr09::_0)
    }
    ///Low output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr09::_1)
    }
}
/**Pmn Event Output Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eorr10 {
    ///0: No effect on output
    _0 = 0,
    ///1: Low output
    _1 = 1,
}
impl From<Eorr10> for bool {
    #[inline(always)]
    fn from(variant: Eorr10) -> Self {
        variant as u8 != 0
    }
}
///Field `EORR10` reader - Pmn Event Output Reset
pub type Eorr10R = crate::BitReader<Eorr10>;
impl Eorr10R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eorr10 {
        match self.bits {
            false => Eorr10::_0,
            true => Eorr10::_1,
        }
    }
    ///No effect on output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eorr10::_0
    }
    ///Low output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eorr10::_1
    }
}
///Field `EORR10` writer - Pmn Event Output Reset
pub type Eorr10W<'a, REG> = crate::BitWriter<'a, REG, Eorr10>;
impl<'a, REG> Eorr10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr10::_0)
    }
    ///Low output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr10::_1)
    }
}
/**Pmn Event Output Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eorr11 {
    ///0: No effect on output
    _0 = 0,
    ///1: Low output
    _1 = 1,
}
impl From<Eorr11> for bool {
    #[inline(always)]
    fn from(variant: Eorr11) -> Self {
        variant as u8 != 0
    }
}
///Field `EORR11` reader - Pmn Event Output Reset
pub type Eorr11R = crate::BitReader<Eorr11>;
impl Eorr11R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eorr11 {
        match self.bits {
            false => Eorr11::_0,
            true => Eorr11::_1,
        }
    }
    ///No effect on output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eorr11::_0
    }
    ///Low output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eorr11::_1
    }
}
///Field `EORR11` writer - Pmn Event Output Reset
pub type Eorr11W<'a, REG> = crate::BitWriter<'a, REG, Eorr11>;
impl<'a, REG> Eorr11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr11::_0)
    }
    ///Low output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr11::_1)
    }
}
/**Pmn Event Output Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eorr12 {
    ///0: No effect on output
    _0 = 0,
    ///1: Low output
    _1 = 1,
}
impl From<Eorr12> for bool {
    #[inline(always)]
    fn from(variant: Eorr12) -> Self {
        variant as u8 != 0
    }
}
///Field `EORR12` reader - Pmn Event Output Reset
pub type Eorr12R = crate::BitReader<Eorr12>;
impl Eorr12R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eorr12 {
        match self.bits {
            false => Eorr12::_0,
            true => Eorr12::_1,
        }
    }
    ///No effect on output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eorr12::_0
    }
    ///Low output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eorr12::_1
    }
}
///Field `EORR12` writer - Pmn Event Output Reset
pub type Eorr12W<'a, REG> = crate::BitWriter<'a, REG, Eorr12>;
impl<'a, REG> Eorr12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr12::_0)
    }
    ///Low output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr12::_1)
    }
}
/**Pmn Event Output Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eorr13 {
    ///0: No effect on output
    _0 = 0,
    ///1: Low output
    _1 = 1,
}
impl From<Eorr13> for bool {
    #[inline(always)]
    fn from(variant: Eorr13) -> Self {
        variant as u8 != 0
    }
}
///Field `EORR13` reader - Pmn Event Output Reset
pub type Eorr13R = crate::BitReader<Eorr13>;
impl Eorr13R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eorr13 {
        match self.bits {
            false => Eorr13::_0,
            true => Eorr13::_1,
        }
    }
    ///No effect on output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eorr13::_0
    }
    ///Low output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eorr13::_1
    }
}
///Field `EORR13` writer - Pmn Event Output Reset
pub type Eorr13W<'a, REG> = crate::BitWriter<'a, REG, Eorr13>;
impl<'a, REG> Eorr13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr13::_0)
    }
    ///Low output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr13::_1)
    }
}
/**Pmn Event Output Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eorr14 {
    ///0: No effect on output
    _0 = 0,
    ///1: Low output
    _1 = 1,
}
impl From<Eorr14> for bool {
    #[inline(always)]
    fn from(variant: Eorr14) -> Self {
        variant as u8 != 0
    }
}
///Field `EORR14` reader - Pmn Event Output Reset
pub type Eorr14R = crate::BitReader<Eorr14>;
impl Eorr14R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eorr14 {
        match self.bits {
            false => Eorr14::_0,
            true => Eorr14::_1,
        }
    }
    ///No effect on output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eorr14::_0
    }
    ///Low output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eorr14::_1
    }
}
///Field `EORR14` writer - Pmn Event Output Reset
pub type Eorr14W<'a, REG> = crate::BitWriter<'a, REG, Eorr14>;
impl<'a, REG> Eorr14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr14::_0)
    }
    ///Low output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr14::_1)
    }
}
/**Pmn Event Output Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eorr15 {
    ///0: No effect on output
    _0 = 0,
    ///1: Low output
    _1 = 1,
}
impl From<Eorr15> for bool {
    #[inline(always)]
    fn from(variant: Eorr15) -> Self {
        variant as u8 != 0
    }
}
///Field `EORR15` reader - Pmn Event Output Reset
pub type Eorr15R = crate::BitReader<Eorr15>;
impl Eorr15R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eorr15 {
        match self.bits {
            false => Eorr15::_0,
            true => Eorr15::_1,
        }
    }
    ///No effect on output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eorr15::_0
    }
    ///Low output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eorr15::_1
    }
}
///Field `EORR15` writer - Pmn Event Output Reset
pub type Eorr15W<'a, REG> = crate::BitWriter<'a, REG, Eorr15>;
impl<'a, REG> Eorr15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect on output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr15::_0)
    }
    ///Low output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr15::_1)
    }
}
impl R {
    ///Bit 0 - Pmn Event Output Reset
    #[inline(always)]
    pub fn eorr00(&self) -> Eorr00R {
        Eorr00R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Pmn Event Output Reset
    #[inline(always)]
    pub fn eorr01(&self) -> Eorr01R {
        Eorr01R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Pmn Event Output Reset
    #[inline(always)]
    pub fn eorr02(&self) -> Eorr02R {
        Eorr02R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Pmn Event Output Reset
    #[inline(always)]
    pub fn eorr03(&self) -> Eorr03R {
        Eorr03R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Pmn Event Output Reset
    #[inline(always)]
    pub fn eorr04(&self) -> Eorr04R {
        Eorr04R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Pmn Event Output Reset
    #[inline(always)]
    pub fn eorr05(&self) -> Eorr05R {
        Eorr05R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Pmn Event Output Reset
    #[inline(always)]
    pub fn eorr06(&self) -> Eorr06R {
        Eorr06R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Pmn Event Output Reset
    #[inline(always)]
    pub fn eorr07(&self) -> Eorr07R {
        Eorr07R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Pmn Event Output Reset
    #[inline(always)]
    pub fn eorr08(&self) -> Eorr08R {
        Eorr08R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Pmn Event Output Reset
    #[inline(always)]
    pub fn eorr09(&self) -> Eorr09R {
        Eorr09R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Pmn Event Output Reset
    #[inline(always)]
    pub fn eorr10(&self) -> Eorr10R {
        Eorr10R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Pmn Event Output Reset
    #[inline(always)]
    pub fn eorr11(&self) -> Eorr11R {
        Eorr11R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Pmn Event Output Reset
    #[inline(always)]
    pub fn eorr12(&self) -> Eorr12R {
        Eorr12R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Pmn Event Output Reset
    #[inline(always)]
    pub fn eorr13(&self) -> Eorr13R {
        Eorr13R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Pmn Event Output Reset
    #[inline(always)]
    pub fn eorr14(&self) -> Eorr14R {
        Eorr14R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Pmn Event Output Reset
    #[inline(always)]
    pub fn eorr15(&self) -> Eorr15R {
        Eorr15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EORR")
            .field("eorr00", &self.eorr00())
            .field("eorr01", &self.eorr01())
            .field("eorr02", &self.eorr02())
            .field("eorr03", &self.eorr03())
            .field("eorr04", &self.eorr04())
            .field("eorr05", &self.eorr05())
            .field("eorr06", &self.eorr06())
            .field("eorr07", &self.eorr07())
            .field("eorr08", &self.eorr08())
            .field("eorr09", &self.eorr09())
            .field("eorr10", &self.eorr10())
            .field("eorr11", &self.eorr11())
            .field("eorr12", &self.eorr12())
            .field("eorr13", &self.eorr13())
            .field("eorr14", &self.eorr14())
            .field("eorr15", &self.eorr15())
            .finish()
    }
}
impl W {
    ///Bit 0 - Pmn Event Output Reset
    #[inline(always)]
    pub fn eorr00(&mut self) -> Eorr00W<EorrSpec> {
        Eorr00W::new(self, 0)
    }
    ///Bit 1 - Pmn Event Output Reset
    #[inline(always)]
    pub fn eorr01(&mut self) -> Eorr01W<EorrSpec> {
        Eorr01W::new(self, 1)
    }
    ///Bit 2 - Pmn Event Output Reset
    #[inline(always)]
    pub fn eorr02(&mut self) -> Eorr02W<EorrSpec> {
        Eorr02W::new(self, 2)
    }
    ///Bit 3 - Pmn Event Output Reset
    #[inline(always)]
    pub fn eorr03(&mut self) -> Eorr03W<EorrSpec> {
        Eorr03W::new(self, 3)
    }
    ///Bit 4 - Pmn Event Output Reset
    #[inline(always)]
    pub fn eorr04(&mut self) -> Eorr04W<EorrSpec> {
        Eorr04W::new(self, 4)
    }
    ///Bit 5 - Pmn Event Output Reset
    #[inline(always)]
    pub fn eorr05(&mut self) -> Eorr05W<EorrSpec> {
        Eorr05W::new(self, 5)
    }
    ///Bit 6 - Pmn Event Output Reset
    #[inline(always)]
    pub fn eorr06(&mut self) -> Eorr06W<EorrSpec> {
        Eorr06W::new(self, 6)
    }
    ///Bit 7 - Pmn Event Output Reset
    #[inline(always)]
    pub fn eorr07(&mut self) -> Eorr07W<EorrSpec> {
        Eorr07W::new(self, 7)
    }
    ///Bit 8 - Pmn Event Output Reset
    #[inline(always)]
    pub fn eorr08(&mut self) -> Eorr08W<EorrSpec> {
        Eorr08W::new(self, 8)
    }
    ///Bit 9 - Pmn Event Output Reset
    #[inline(always)]
    pub fn eorr09(&mut self) -> Eorr09W<EorrSpec> {
        Eorr09W::new(self, 9)
    }
    ///Bit 10 - Pmn Event Output Reset
    #[inline(always)]
    pub fn eorr10(&mut self) -> Eorr10W<EorrSpec> {
        Eorr10W::new(self, 10)
    }
    ///Bit 11 - Pmn Event Output Reset
    #[inline(always)]
    pub fn eorr11(&mut self) -> Eorr11W<EorrSpec> {
        Eorr11W::new(self, 11)
    }
    ///Bit 12 - Pmn Event Output Reset
    #[inline(always)]
    pub fn eorr12(&mut self) -> Eorr12W<EorrSpec> {
        Eorr12W::new(self, 12)
    }
    ///Bit 13 - Pmn Event Output Reset
    #[inline(always)]
    pub fn eorr13(&mut self) -> Eorr13W<EorrSpec> {
        Eorr13W::new(self, 13)
    }
    ///Bit 14 - Pmn Event Output Reset
    #[inline(always)]
    pub fn eorr14(&mut self) -> Eorr14W<EorrSpec> {
        Eorr14W::new(self, 14)
    }
    ///Bit 15 - Pmn Event Output Reset
    #[inline(always)]
    pub fn eorr15(&mut self) -> Eorr15W<EorrSpec> {
        Eorr15W::new(self, 15)
    }
}
/**Port Control Register 4

You can [`read`](crate::Reg::read) this register and get [`eorr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eorr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct EorrSpec;
impl crate::RegisterSpec for EorrSpec {
    type Ux = u16;
}
///`read()` method returns [`eorr::R`](R) reader structure
impl crate::Readable for EorrSpec {}
///`write(|w| ..)` method takes [`eorr::W`](W) writer structure
impl crate::Writable for EorrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EORR to value 0
impl crate::Resettable for EorrSpec {}
