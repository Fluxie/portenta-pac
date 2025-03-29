///Register `MSTPCRB` reader
pub type R = crate::R<MstpcrbSpec>;
///Register `MSTPCRB` writer
pub type W = crate::W<MstpcrbSpec>;
/**CEC Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpb3 {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<Mstpb3> for bool {
    #[inline(always)]
    fn from(variant: Mstpb3) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPB3` reader - CEC Module Stop
pub type Mstpb3R = crate::BitReader<Mstpb3>;
impl Mstpb3R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mstpb3 {
        match self.bits {
            false => Mstpb3::_0,
            true => Mstpb3::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpb3::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpb3::_1
    }
}
///Field `MSTPB3` writer - CEC Module Stop
pub type Mstpb3W<'a, REG> = crate::BitWriter<'a, REG, Mstpb3>;
impl<'a, REG> Mstpb3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb3::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb3::_1)
    }
}
/**Quad Serial Peripheral Interface Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpb6 {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<Mstpb6> for bool {
    #[inline(always)]
    fn from(variant: Mstpb6) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPB6` reader - Quad Serial Peripheral Interface Module Stop
pub type Mstpb6R = crate::BitReader<Mstpb6>;
impl Mstpb6R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mstpb6 {
        match self.bits {
            false => Mstpb6::_0,
            true => Mstpb6::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpb6::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpb6::_1
    }
}
///Field `MSTPB6` writer - Quad Serial Peripheral Interface Module Stop
pub type Mstpb6W<'a, REG> = crate::BitWriter<'a, REG, Mstpb6>;
impl<'a, REG> Mstpb6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb6::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb6::_1)
    }
}
/**I2C Bus Interface 2 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpb7 {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<Mstpb7> for bool {
    #[inline(always)]
    fn from(variant: Mstpb7) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPB7` reader - I2C Bus Interface 2 Module Stop
pub type Mstpb7R = crate::BitReader<Mstpb7>;
impl Mstpb7R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mstpb7 {
        match self.bits {
            false => Mstpb7::_0,
            true => Mstpb7::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpb7::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpb7::_1
    }
}
///Field `MSTPB7` writer - I2C Bus Interface 2 Module Stop
pub type Mstpb7W<'a, REG> = crate::BitWriter<'a, REG, Mstpb7>;
impl<'a, REG> Mstpb7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb7::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb7::_1)
    }
}
/**I2C Bus Interface 1 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpb8 {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<Mstpb8> for bool {
    #[inline(always)]
    fn from(variant: Mstpb8) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPB8` reader - I2C Bus Interface 1 Module Stop
pub type Mstpb8R = crate::BitReader<Mstpb8>;
impl Mstpb8R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mstpb8 {
        match self.bits {
            false => Mstpb8::_0,
            true => Mstpb8::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpb8::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpb8::_1
    }
}
///Field `MSTPB8` writer - I2C Bus Interface 1 Module Stop
pub type Mstpb8W<'a, REG> = crate::BitWriter<'a, REG, Mstpb8>;
impl<'a, REG> Mstpb8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb8::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb8::_1)
    }
}
/**I2C Bus Interface 0 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpb9 {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<Mstpb9> for bool {
    #[inline(always)]
    fn from(variant: Mstpb9) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPB9` reader - I2C Bus Interface 0 Module Stop
pub type Mstpb9R = crate::BitReader<Mstpb9>;
impl Mstpb9R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mstpb9 {
        match self.bits {
            false => Mstpb9::_0,
            true => Mstpb9::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpb9::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpb9::_1
    }
}
///Field `MSTPB9` writer - I2C Bus Interface 0 Module Stop
pub type Mstpb9W<'a, REG> = crate::BitWriter<'a, REG, Mstpb9>;
impl<'a, REG> Mstpb9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb9::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb9::_1)
    }
}
/**Universal Serial Bus 2.0 FS Interface 0 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpb11 {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<Mstpb11> for bool {
    #[inline(always)]
    fn from(variant: Mstpb11) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPB11` reader - Universal Serial Bus 2.0 FS Interface 0 Module Stop
pub type Mstpb11R = crate::BitReader<Mstpb11>;
impl Mstpb11R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mstpb11 {
        match self.bits {
            false => Mstpb11::_0,
            true => Mstpb11::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpb11::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpb11::_1
    }
}
///Field `MSTPB11` writer - Universal Serial Bus 2.0 FS Interface 0 Module Stop
pub type Mstpb11W<'a, REG> = crate::BitWriter<'a, REG, Mstpb11>;
impl<'a, REG> Mstpb11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb11::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb11::_1)
    }
}
/**Universal Serial Bus 2.0 HS Interface Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpb12 {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<Mstpb12> for bool {
    #[inline(always)]
    fn from(variant: Mstpb12) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPB12` reader - Universal Serial Bus 2.0 HS Interface Module Stop
pub type Mstpb12R = crate::BitReader<Mstpb12>;
impl Mstpb12R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mstpb12 {
        match self.bits {
            false => Mstpb12::_0,
            true => Mstpb12::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpb12::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpb12::_1
    }
}
///Field `MSTPB12` writer - Universal Serial Bus 2.0 HS Interface Module Stop
pub type Mstpb12W<'a, REG> = crate::BitWriter<'a, REG, Mstpb12>;
impl<'a, REG> Mstpb12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb12::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb12::_1)
    }
}
/**ETHERC0 and EDMAC0 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpb15 {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<Mstpb15> for bool {
    #[inline(always)]
    fn from(variant: Mstpb15) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPB15` reader - ETHERC0 and EDMAC0 Module Stop
pub type Mstpb15R = crate::BitReader<Mstpb15>;
impl Mstpb15R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mstpb15 {
        match self.bits {
            false => Mstpb15::_0,
            true => Mstpb15::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpb15::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpb15::_1
    }
}
///Field `MSTPB15` writer - ETHERC0 and EDMAC0 Module Stop
pub type Mstpb15W<'a, REG> = crate::BitWriter<'a, REG, Mstpb15>;
impl<'a, REG> Mstpb15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb15::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb15::_1)
    }
}
/**OSPI Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpb16 {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<Mstpb16> for bool {
    #[inline(always)]
    fn from(variant: Mstpb16) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPB16` reader - OSPI Module Stop
pub type Mstpb16R = crate::BitReader<Mstpb16>;
impl Mstpb16R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mstpb16 {
        match self.bits {
            false => Mstpb16::_0,
            true => Mstpb16::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpb16::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpb16::_1
    }
}
///Field `MSTPB16` writer - OSPI Module Stop
pub type Mstpb16W<'a, REG> = crate::BitWriter<'a, REG, Mstpb16>;
impl<'a, REG> Mstpb16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb16::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb16::_1)
    }
}
/**Serial Peripheral Interface 1 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpb18 {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<Mstpb18> for bool {
    #[inline(always)]
    fn from(variant: Mstpb18) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPB18` reader - Serial Peripheral Interface 1 Module Stop
pub type Mstpb18R = crate::BitReader<Mstpb18>;
impl Mstpb18R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mstpb18 {
        match self.bits {
            false => Mstpb18::_0,
            true => Mstpb18::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpb18::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpb18::_1
    }
}
///Field `MSTPB18` writer - Serial Peripheral Interface 1 Module Stop
pub type Mstpb18W<'a, REG> = crate::BitWriter<'a, REG, Mstpb18>;
impl<'a, REG> Mstpb18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb18::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb18::_1)
    }
}
/**Serial Peripheral Interface 0 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpb19 {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<Mstpb19> for bool {
    #[inline(always)]
    fn from(variant: Mstpb19) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPB19` reader - Serial Peripheral Interface 0 Module Stop
pub type Mstpb19R = crate::BitReader<Mstpb19>;
impl Mstpb19R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mstpb19 {
        match self.bits {
            false => Mstpb19::_0,
            true => Mstpb19::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpb19::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpb19::_1
    }
}
///Field `MSTPB19` writer - Serial Peripheral Interface 0 Module Stop
pub type Mstpb19W<'a, REG> = crate::BitWriter<'a, REG, Mstpb19>;
impl<'a, REG> Mstpb19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb19::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb19::_1)
    }
}
/**Serial Communication Interface 9 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpb22 {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<Mstpb22> for bool {
    #[inline(always)]
    fn from(variant: Mstpb22) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPB22` reader - Serial Communication Interface 9 Module Stop
pub type Mstpb22R = crate::BitReader<Mstpb22>;
impl Mstpb22R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mstpb22 {
        match self.bits {
            false => Mstpb22::_0,
            true => Mstpb22::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpb22::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpb22::_1
    }
}
///Field `MSTPB22` writer - Serial Communication Interface 9 Module Stop
pub type Mstpb22W<'a, REG> = crate::BitWriter<'a, REG, Mstpb22>;
impl<'a, REG> Mstpb22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb22::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb22::_1)
    }
}
/**Serial Communication Interface 8 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpb23 {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<Mstpb23> for bool {
    #[inline(always)]
    fn from(variant: Mstpb23) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPB23` reader - Serial Communication Interface 8 Module Stop
pub type Mstpb23R = crate::BitReader<Mstpb23>;
impl Mstpb23R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mstpb23 {
        match self.bits {
            false => Mstpb23::_0,
            true => Mstpb23::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpb23::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpb23::_1
    }
}
///Field `MSTPB23` writer - Serial Communication Interface 8 Module Stop
pub type Mstpb23W<'a, REG> = crate::BitWriter<'a, REG, Mstpb23>;
impl<'a, REG> Mstpb23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb23::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb23::_1)
    }
}
/**Serial Communication Interface 7 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpb24 {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<Mstpb24> for bool {
    #[inline(always)]
    fn from(variant: Mstpb24) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPB24` reader - Serial Communication Interface 7 Module Stop
pub type Mstpb24R = crate::BitReader<Mstpb24>;
impl Mstpb24R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mstpb24 {
        match self.bits {
            false => Mstpb24::_0,
            true => Mstpb24::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpb24::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpb24::_1
    }
}
///Field `MSTPB24` writer - Serial Communication Interface 7 Module Stop
pub type Mstpb24W<'a, REG> = crate::BitWriter<'a, REG, Mstpb24>;
impl<'a, REG> Mstpb24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb24::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb24::_1)
    }
}
/**Serial Communication Interface 6 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpb25 {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<Mstpb25> for bool {
    #[inline(always)]
    fn from(variant: Mstpb25) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPB25` reader - Serial Communication Interface 6 Module Stop
pub type Mstpb25R = crate::BitReader<Mstpb25>;
impl Mstpb25R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mstpb25 {
        match self.bits {
            false => Mstpb25::_0,
            true => Mstpb25::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpb25::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpb25::_1
    }
}
///Field `MSTPB25` writer - Serial Communication Interface 6 Module Stop
pub type Mstpb25W<'a, REG> = crate::BitWriter<'a, REG, Mstpb25>;
impl<'a, REG> Mstpb25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb25::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb25::_1)
    }
}
/**Serial Communication Interface 5 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpb26 {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<Mstpb26> for bool {
    #[inline(always)]
    fn from(variant: Mstpb26) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPB26` reader - Serial Communication Interface 5 Module Stop
pub type Mstpb26R = crate::BitReader<Mstpb26>;
impl Mstpb26R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mstpb26 {
        match self.bits {
            false => Mstpb26::_0,
            true => Mstpb26::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpb26::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpb26::_1
    }
}
///Field `MSTPB26` writer - Serial Communication Interface 5 Module Stop
pub type Mstpb26W<'a, REG> = crate::BitWriter<'a, REG, Mstpb26>;
impl<'a, REG> Mstpb26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb26::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb26::_1)
    }
}
/**Serial Communication Interface 4 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpb27 {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<Mstpb27> for bool {
    #[inline(always)]
    fn from(variant: Mstpb27) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPB27` reader - Serial Communication Interface 4 Module Stop
pub type Mstpb27R = crate::BitReader<Mstpb27>;
impl Mstpb27R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mstpb27 {
        match self.bits {
            false => Mstpb27::_0,
            true => Mstpb27::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpb27::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpb27::_1
    }
}
///Field `MSTPB27` writer - Serial Communication Interface 4 Module Stop
pub type Mstpb27W<'a, REG> = crate::BitWriter<'a, REG, Mstpb27>;
impl<'a, REG> Mstpb27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb27::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb27::_1)
    }
}
/**Serial Communication Interface 3 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpb28 {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<Mstpb28> for bool {
    #[inline(always)]
    fn from(variant: Mstpb28) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPB28` reader - Serial Communication Interface 3 Module Stop
pub type Mstpb28R = crate::BitReader<Mstpb28>;
impl Mstpb28R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mstpb28 {
        match self.bits {
            false => Mstpb28::_0,
            true => Mstpb28::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpb28::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpb28::_1
    }
}
///Field `MSTPB28` writer - Serial Communication Interface 3 Module Stop
pub type Mstpb28W<'a, REG> = crate::BitWriter<'a, REG, Mstpb28>;
impl<'a, REG> Mstpb28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb28::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb28::_1)
    }
}
/**Serial Communication Interface 2 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpb29 {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<Mstpb29> for bool {
    #[inline(always)]
    fn from(variant: Mstpb29) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPB29` reader - Serial Communication Interface 2 Module Stop
pub type Mstpb29R = crate::BitReader<Mstpb29>;
impl Mstpb29R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mstpb29 {
        match self.bits {
            false => Mstpb29::_0,
            true => Mstpb29::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpb29::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpb29::_1
    }
}
///Field `MSTPB29` writer - Serial Communication Interface 2 Module Stop
pub type Mstpb29W<'a, REG> = crate::BitWriter<'a, REG, Mstpb29>;
impl<'a, REG> Mstpb29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb29::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb29::_1)
    }
}
/**Serial Communication Interface 1 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpb30 {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<Mstpb30> for bool {
    #[inline(always)]
    fn from(variant: Mstpb30) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPB30` reader - Serial Communication Interface 1 Module Stop
pub type Mstpb30R = crate::BitReader<Mstpb30>;
impl Mstpb30R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mstpb30 {
        match self.bits {
            false => Mstpb30::_0,
            true => Mstpb30::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpb30::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpb30::_1
    }
}
///Field `MSTPB30` writer - Serial Communication Interface 1 Module Stop
pub type Mstpb30W<'a, REG> = crate::BitWriter<'a, REG, Mstpb30>;
impl<'a, REG> Mstpb30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb30::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb30::_1)
    }
}
/**Serial Communication Interface 0 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpb31 {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<Mstpb31> for bool {
    #[inline(always)]
    fn from(variant: Mstpb31) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPB31` reader - Serial Communication Interface 0 Module Stop
pub type Mstpb31R = crate::BitReader<Mstpb31>;
impl Mstpb31R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mstpb31 {
        match self.bits {
            false => Mstpb31::_0,
            true => Mstpb31::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpb31::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpb31::_1
    }
}
///Field `MSTPB31` writer - Serial Communication Interface 0 Module Stop
pub type Mstpb31W<'a, REG> = crate::BitWriter<'a, REG, Mstpb31>;
impl<'a, REG> Mstpb31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb31::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb31::_1)
    }
}
impl R {
    ///Bit 3 - CEC Module Stop
    #[inline(always)]
    pub fn mstpb3(&self) -> Mstpb3R {
        Mstpb3R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 6 - Quad Serial Peripheral Interface Module Stop
    #[inline(always)]
    pub fn mstpb6(&self) -> Mstpb6R {
        Mstpb6R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - I2C Bus Interface 2 Module Stop
    #[inline(always)]
    pub fn mstpb7(&self) -> Mstpb7R {
        Mstpb7R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - I2C Bus Interface 1 Module Stop
    #[inline(always)]
    pub fn mstpb8(&self) -> Mstpb8R {
        Mstpb8R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - I2C Bus Interface 0 Module Stop
    #[inline(always)]
    pub fn mstpb9(&self) -> Mstpb9R {
        Mstpb9R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - Universal Serial Bus 2.0 FS Interface 0 Module Stop
    #[inline(always)]
    pub fn mstpb11(&self) -> Mstpb11R {
        Mstpb11R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Universal Serial Bus 2.0 HS Interface Module Stop
    #[inline(always)]
    pub fn mstpb12(&self) -> Mstpb12R {
        Mstpb12R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 15 - ETHERC0 and EDMAC0 Module Stop
    #[inline(always)]
    pub fn mstpb15(&self) -> Mstpb15R {
        Mstpb15R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - OSPI Module Stop
    #[inline(always)]
    pub fn mstpb16(&self) -> Mstpb16R {
        Mstpb16R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - Serial Peripheral Interface 1 Module Stop
    #[inline(always)]
    pub fn mstpb18(&self) -> Mstpb18R {
        Mstpb18R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Serial Peripheral Interface 0 Module Stop
    #[inline(always)]
    pub fn mstpb19(&self) -> Mstpb19R {
        Mstpb19R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 22 - Serial Communication Interface 9 Module Stop
    #[inline(always)]
    pub fn mstpb22(&self) -> Mstpb22R {
        Mstpb22R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Serial Communication Interface 8 Module Stop
    #[inline(always)]
    pub fn mstpb23(&self) -> Mstpb23R {
        Mstpb23R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Serial Communication Interface 7 Module Stop
    #[inline(always)]
    pub fn mstpb24(&self) -> Mstpb24R {
        Mstpb24R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Serial Communication Interface 6 Module Stop
    #[inline(always)]
    pub fn mstpb25(&self) -> Mstpb25R {
        Mstpb25R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Serial Communication Interface 5 Module Stop
    #[inline(always)]
    pub fn mstpb26(&self) -> Mstpb26R {
        Mstpb26R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Serial Communication Interface 4 Module Stop
    #[inline(always)]
    pub fn mstpb27(&self) -> Mstpb27R {
        Mstpb27R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Serial Communication Interface 3 Module Stop
    #[inline(always)]
    pub fn mstpb28(&self) -> Mstpb28R {
        Mstpb28R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Serial Communication Interface 2 Module Stop
    #[inline(always)]
    pub fn mstpb29(&self) -> Mstpb29R {
        Mstpb29R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Serial Communication Interface 1 Module Stop
    #[inline(always)]
    pub fn mstpb30(&self) -> Mstpb30R {
        Mstpb30R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Serial Communication Interface 0 Module Stop
    #[inline(always)]
    pub fn mstpb31(&self) -> Mstpb31R {
        Mstpb31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MSTPCRB")
            .field("mstpb3", &self.mstpb3())
            .field("mstpb6", &self.mstpb6())
            .field("mstpb7", &self.mstpb7())
            .field("mstpb8", &self.mstpb8())
            .field("mstpb9", &self.mstpb9())
            .field("mstpb11", &self.mstpb11())
            .field("mstpb12", &self.mstpb12())
            .field("mstpb15", &self.mstpb15())
            .field("mstpb16", &self.mstpb16())
            .field("mstpb18", &self.mstpb18())
            .field("mstpb19", &self.mstpb19())
            .field("mstpb22", &self.mstpb22())
            .field("mstpb23", &self.mstpb23())
            .field("mstpb24", &self.mstpb24())
            .field("mstpb25", &self.mstpb25())
            .field("mstpb26", &self.mstpb26())
            .field("mstpb27", &self.mstpb27())
            .field("mstpb28", &self.mstpb28())
            .field("mstpb29", &self.mstpb29())
            .field("mstpb30", &self.mstpb30())
            .field("mstpb31", &self.mstpb31())
            .finish()
    }
}
impl W {
    ///Bit 3 - CEC Module Stop
    #[inline(always)]
    pub fn mstpb3(&mut self) -> Mstpb3W<MstpcrbSpec> {
        Mstpb3W::new(self, 3)
    }
    ///Bit 6 - Quad Serial Peripheral Interface Module Stop
    #[inline(always)]
    pub fn mstpb6(&mut self) -> Mstpb6W<MstpcrbSpec> {
        Mstpb6W::new(self, 6)
    }
    ///Bit 7 - I2C Bus Interface 2 Module Stop
    #[inline(always)]
    pub fn mstpb7(&mut self) -> Mstpb7W<MstpcrbSpec> {
        Mstpb7W::new(self, 7)
    }
    ///Bit 8 - I2C Bus Interface 1 Module Stop
    #[inline(always)]
    pub fn mstpb8(&mut self) -> Mstpb8W<MstpcrbSpec> {
        Mstpb8W::new(self, 8)
    }
    ///Bit 9 - I2C Bus Interface 0 Module Stop
    #[inline(always)]
    pub fn mstpb9(&mut self) -> Mstpb9W<MstpcrbSpec> {
        Mstpb9W::new(self, 9)
    }
    ///Bit 11 - Universal Serial Bus 2.0 FS Interface 0 Module Stop
    #[inline(always)]
    pub fn mstpb11(&mut self) -> Mstpb11W<MstpcrbSpec> {
        Mstpb11W::new(self, 11)
    }
    ///Bit 12 - Universal Serial Bus 2.0 HS Interface Module Stop
    #[inline(always)]
    pub fn mstpb12(&mut self) -> Mstpb12W<MstpcrbSpec> {
        Mstpb12W::new(self, 12)
    }
    ///Bit 15 - ETHERC0 and EDMAC0 Module Stop
    #[inline(always)]
    pub fn mstpb15(&mut self) -> Mstpb15W<MstpcrbSpec> {
        Mstpb15W::new(self, 15)
    }
    ///Bit 16 - OSPI Module Stop
    #[inline(always)]
    pub fn mstpb16(&mut self) -> Mstpb16W<MstpcrbSpec> {
        Mstpb16W::new(self, 16)
    }
    ///Bit 18 - Serial Peripheral Interface 1 Module Stop
    #[inline(always)]
    pub fn mstpb18(&mut self) -> Mstpb18W<MstpcrbSpec> {
        Mstpb18W::new(self, 18)
    }
    ///Bit 19 - Serial Peripheral Interface 0 Module Stop
    #[inline(always)]
    pub fn mstpb19(&mut self) -> Mstpb19W<MstpcrbSpec> {
        Mstpb19W::new(self, 19)
    }
    ///Bit 22 - Serial Communication Interface 9 Module Stop
    #[inline(always)]
    pub fn mstpb22(&mut self) -> Mstpb22W<MstpcrbSpec> {
        Mstpb22W::new(self, 22)
    }
    ///Bit 23 - Serial Communication Interface 8 Module Stop
    #[inline(always)]
    pub fn mstpb23(&mut self) -> Mstpb23W<MstpcrbSpec> {
        Mstpb23W::new(self, 23)
    }
    ///Bit 24 - Serial Communication Interface 7 Module Stop
    #[inline(always)]
    pub fn mstpb24(&mut self) -> Mstpb24W<MstpcrbSpec> {
        Mstpb24W::new(self, 24)
    }
    ///Bit 25 - Serial Communication Interface 6 Module Stop
    #[inline(always)]
    pub fn mstpb25(&mut self) -> Mstpb25W<MstpcrbSpec> {
        Mstpb25W::new(self, 25)
    }
    ///Bit 26 - Serial Communication Interface 5 Module Stop
    #[inline(always)]
    pub fn mstpb26(&mut self) -> Mstpb26W<MstpcrbSpec> {
        Mstpb26W::new(self, 26)
    }
    ///Bit 27 - Serial Communication Interface 4 Module Stop
    #[inline(always)]
    pub fn mstpb27(&mut self) -> Mstpb27W<MstpcrbSpec> {
        Mstpb27W::new(self, 27)
    }
    ///Bit 28 - Serial Communication Interface 3 Module Stop
    #[inline(always)]
    pub fn mstpb28(&mut self) -> Mstpb28W<MstpcrbSpec> {
        Mstpb28W::new(self, 28)
    }
    ///Bit 29 - Serial Communication Interface 2 Module Stop
    #[inline(always)]
    pub fn mstpb29(&mut self) -> Mstpb29W<MstpcrbSpec> {
        Mstpb29W::new(self, 29)
    }
    ///Bit 30 - Serial Communication Interface 1 Module Stop
    #[inline(always)]
    pub fn mstpb30(&mut self) -> Mstpb30W<MstpcrbSpec> {
        Mstpb30W::new(self, 30)
    }
    ///Bit 31 - Serial Communication Interface 0 Module Stop
    #[inline(always)]
    pub fn mstpb31(&mut self) -> Mstpb31W<MstpcrbSpec> {
        Mstpb31W::new(self, 31)
    }
}
/**Module Stop Control Register B

You can [`read`](crate::Reg::read) this register and get [`mstpcrb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mstpcrb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MstpcrbSpec;
impl crate::RegisterSpec for MstpcrbSpec {
    type Ux = u32;
}
///`read()` method returns [`mstpcrb::R`](R) reader structure
impl crate::Readable for MstpcrbSpec {}
///`write(|w| ..)` method takes [`mstpcrb::W`](W) writer structure
impl crate::Writable for MstpcrbSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MSTPCRB to value 0xffff_ffff
impl crate::Resettable for MstpcrbSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
