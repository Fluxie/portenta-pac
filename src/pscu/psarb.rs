///Register `PSARB` reader
pub type R = crate::R<PsarbSpec>;
///Register `PSARB` writer
pub type W = crate::W<PsarbSpec>;
/**CEC and the MSTPCRB.MSTPB3 bit security attribution

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psarb3 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Psarb3> for bool {
    #[inline(always)]
    fn from(variant: Psarb3) -> Self {
        variant as u8 != 0
    }
}
///Field `PSARB3` reader - CEC and the MSTPCRB.MSTPB3 bit security attribution
pub type Psarb3R = crate::BitReader<Psarb3>;
impl Psarb3R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Psarb3 {
        match self.bits {
            false => Psarb3::_0,
            true => Psarb3::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psarb3::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psarb3::_1
    }
}
///Field `PSARB3` writer - CEC and the MSTPCRB.MSTPB3 bit security attribution
pub type Psarb3W<'a, REG> = crate::BitWriter<'a, REG, Psarb3>;
impl<'a, REG> Psarb3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psarb3::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psarb3::_1)
    }
}
///Field `PSARB6` reader - QSPI and the MSTPCRB.MSTPB6 bit security attribution
pub type Psarb6R = crate::BitReader;
/**IIC2 and the MSTPCRB.MSTPB7 bit security attribution

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psarb7 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Psarb7> for bool {
    #[inline(always)]
    fn from(variant: Psarb7) -> Self {
        variant as u8 != 0
    }
}
///Field `PSARB7` reader - IIC2 and the MSTPCRB.MSTPB7 bit security attribution
pub type Psarb7R = crate::BitReader<Psarb7>;
impl Psarb7R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Psarb7 {
        match self.bits {
            false => Psarb7::_0,
            true => Psarb7::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psarb7::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psarb7::_1
    }
}
///Field `PSARB7` writer - IIC2 and the MSTPCRB.MSTPB7 bit security attribution
pub type Psarb7W<'a, REG> = crate::BitWriter<'a, REG, Psarb7>;
impl<'a, REG> Psarb7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psarb7::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psarb7::_1)
    }
}
/**IIC1 and the MSTPCRB.MSTPB8 bit security attribution

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psarb8 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Psarb8> for bool {
    #[inline(always)]
    fn from(variant: Psarb8) -> Self {
        variant as u8 != 0
    }
}
///Field `PSARB8` reader - IIC1 and the MSTPCRB.MSTPB8 bit security attribution
pub type Psarb8R = crate::BitReader<Psarb8>;
impl Psarb8R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Psarb8 {
        match self.bits {
            false => Psarb8::_0,
            true => Psarb8::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psarb8::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psarb8::_1
    }
}
///Field `PSARB8` writer - IIC1 and the MSTPCRB.MSTPB8 bit security attribution
pub type Psarb8W<'a, REG> = crate::BitWriter<'a, REG, Psarb8>;
impl<'a, REG> Psarb8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psarb8::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psarb8::_1)
    }
}
/**IIC0 and the MSTPCRB.MSTPB9 bit security attribution

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psarb9 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Psarb9> for bool {
    #[inline(always)]
    fn from(variant: Psarb9) -> Self {
        variant as u8 != 0
    }
}
///Field `PSARB9` reader - IIC0 and the MSTPCRB.MSTPB9 bit security attribution
pub type Psarb9R = crate::BitReader<Psarb9>;
impl Psarb9R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Psarb9 {
        match self.bits {
            false => Psarb9::_0,
            true => Psarb9::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psarb9::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psarb9::_1
    }
}
///Field `PSARB9` writer - IIC0 and the MSTPCRB.MSTPB9 bit security attribution
pub type Psarb9W<'a, REG> = crate::BitWriter<'a, REG, Psarb9>;
impl<'a, REG> Psarb9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psarb9::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psarb9::_1)
    }
}
/**USBFS and the MSTPCRB.MSTPB11 bit security attribution

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psarb11 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Psarb11> for bool {
    #[inline(always)]
    fn from(variant: Psarb11) -> Self {
        variant as u8 != 0
    }
}
///Field `PSARB11` reader - USBFS and the MSTPCRB.MSTPB11 bit security attribution
pub type Psarb11R = crate::BitReader<Psarb11>;
impl Psarb11R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Psarb11 {
        match self.bits {
            false => Psarb11::_0,
            true => Psarb11::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psarb11::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psarb11::_1
    }
}
///Field `PSARB11` writer - USBFS and the MSTPCRB.MSTPB11 bit security attribution
pub type Psarb11W<'a, REG> = crate::BitWriter<'a, REG, Psarb11>;
impl<'a, REG> Psarb11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psarb11::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psarb11::_1)
    }
}
/**USBHS and the MSTPCRB.MSTPB12 bit security attribution

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psarb12 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Psarb12> for bool {
    #[inline(always)]
    fn from(variant: Psarb12) -> Self {
        variant as u8 != 0
    }
}
///Field `PSARB12` reader - USBHS and the MSTPCRB.MSTPB12 bit security attribution
pub type Psarb12R = crate::BitReader<Psarb12>;
impl Psarb12R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Psarb12 {
        match self.bits {
            false => Psarb12::_0,
            true => Psarb12::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psarb12::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psarb12::_1
    }
}
///Field `PSARB12` writer - USBHS and the MSTPCRB.MSTPB12 bit security attribution
pub type Psarb12W<'a, REG> = crate::BitWriter<'a, REG, Psarb12>;
impl<'a, REG> Psarb12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psarb12::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psarb12::_1)
    }
}
///Field `PSARB15` reader - ETHER0/EDMAC0, the MSTPCRB.MSTPB15 bit and the PFENET.PHYMODE0 bit security attribution
pub type Psarb15R = crate::BitReader;
///Field `PSARB16` reader - OSPI and the MSTPCRB.MSTPB16 bit security attribution
pub type Psarb16R = crate::BitReader;
/**SPI1 and the MSTPCRB.MSTPB18 bit security attribution

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psarb18 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Psarb18> for bool {
    #[inline(always)]
    fn from(variant: Psarb18) -> Self {
        variant as u8 != 0
    }
}
///Field `PSARB18` reader - SPI1 and the MSTPCRB.MSTPB18 bit security attribution
pub type Psarb18R = crate::BitReader<Psarb18>;
impl Psarb18R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Psarb18 {
        match self.bits {
            false => Psarb18::_0,
            true => Psarb18::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psarb18::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psarb18::_1
    }
}
///Field `PSARB18` writer - SPI1 and the MSTPCRB.MSTPB18 bit security attribution
pub type Psarb18W<'a, REG> = crate::BitWriter<'a, REG, Psarb18>;
impl<'a, REG> Psarb18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psarb18::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psarb18::_1)
    }
}
/**SPI0 and the MSTPCRB.MSTPB19 bit security attribution

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psarb19 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Psarb19> for bool {
    #[inline(always)]
    fn from(variant: Psarb19) -> Self {
        variant as u8 != 0
    }
}
///Field `PSARB19` reader - SPI0 and the MSTPCRB.MSTPB19 bit security attribution
pub type Psarb19R = crate::BitReader<Psarb19>;
impl Psarb19R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Psarb19 {
        match self.bits {
            false => Psarb19::_0,
            true => Psarb19::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psarb19::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psarb19::_1
    }
}
///Field `PSARB19` writer - SPI0 and the MSTPCRB.MSTPB19 bit security attribution
pub type Psarb19W<'a, REG> = crate::BitWriter<'a, REG, Psarb19>;
impl<'a, REG> Psarb19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psarb19::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psarb19::_1)
    }
}
/**SCI9 and the MSTPCRB.MSTPB22 bit security attribution

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psarb22 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Psarb22> for bool {
    #[inline(always)]
    fn from(variant: Psarb22) -> Self {
        variant as u8 != 0
    }
}
///Field `PSARB22` reader - SCI9 and the MSTPCRB.MSTPB22 bit security attribution
pub type Psarb22R = crate::BitReader<Psarb22>;
impl Psarb22R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Psarb22 {
        match self.bits {
            false => Psarb22::_0,
            true => Psarb22::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psarb22::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psarb22::_1
    }
}
///Field `PSARB22` writer - SCI9 and the MSTPCRB.MSTPB22 bit security attribution
pub type Psarb22W<'a, REG> = crate::BitWriter<'a, REG, Psarb22>;
impl<'a, REG> Psarb22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psarb22::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psarb22::_1)
    }
}
/**SCI8 and the MSTPCRB.MSTPB23 bit security attribution

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psarb23 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Psarb23> for bool {
    #[inline(always)]
    fn from(variant: Psarb23) -> Self {
        variant as u8 != 0
    }
}
///Field `PSARB23` reader - SCI8 and the MSTPCRB.MSTPB23 bit security attribution
pub type Psarb23R = crate::BitReader<Psarb23>;
impl Psarb23R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Psarb23 {
        match self.bits {
            false => Psarb23::_0,
            true => Psarb23::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psarb23::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psarb23::_1
    }
}
///Field `PSARB23` writer - SCI8 and the MSTPCRB.MSTPB23 bit security attribution
pub type Psarb23W<'a, REG> = crate::BitWriter<'a, REG, Psarb23>;
impl<'a, REG> Psarb23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psarb23::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psarb23::_1)
    }
}
/**SCI7 and the MSTPCRB.MSTPB24 bit security attribution

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psarb24 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Psarb24> for bool {
    #[inline(always)]
    fn from(variant: Psarb24) -> Self {
        variant as u8 != 0
    }
}
///Field `PSARB24` reader - SCI7 and the MSTPCRB.MSTPB24 bit security attribution
pub type Psarb24R = crate::BitReader<Psarb24>;
impl Psarb24R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Psarb24 {
        match self.bits {
            false => Psarb24::_0,
            true => Psarb24::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psarb24::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psarb24::_1
    }
}
///Field `PSARB24` writer - SCI7 and the MSTPCRB.MSTPB24 bit security attribution
pub type Psarb24W<'a, REG> = crate::BitWriter<'a, REG, Psarb24>;
impl<'a, REG> Psarb24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psarb24::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psarb24::_1)
    }
}
/**SCI6 and the MSTPCRB.MSTPB25 bit security attribution

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psarb25 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Psarb25> for bool {
    #[inline(always)]
    fn from(variant: Psarb25) -> Self {
        variant as u8 != 0
    }
}
///Field `PSARB25` reader - SCI6 and the MSTPCRB.MSTPB25 bit security attribution
pub type Psarb25R = crate::BitReader<Psarb25>;
impl Psarb25R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Psarb25 {
        match self.bits {
            false => Psarb25::_0,
            true => Psarb25::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psarb25::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psarb25::_1
    }
}
///Field `PSARB25` writer - SCI6 and the MSTPCRB.MSTPB25 bit security attribution
pub type Psarb25W<'a, REG> = crate::BitWriter<'a, REG, Psarb25>;
impl<'a, REG> Psarb25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psarb25::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psarb25::_1)
    }
}
/**SCI5 and the MSTPCRB.MSTPB26 bit security attribution

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psarb26 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Psarb26> for bool {
    #[inline(always)]
    fn from(variant: Psarb26) -> Self {
        variant as u8 != 0
    }
}
///Field `PSARB26` reader - SCI5 and the MSTPCRB.MSTPB26 bit security attribution
pub type Psarb26R = crate::BitReader<Psarb26>;
impl Psarb26R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Psarb26 {
        match self.bits {
            false => Psarb26::_0,
            true => Psarb26::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psarb26::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psarb26::_1
    }
}
///Field `PSARB26` writer - SCI5 and the MSTPCRB.MSTPB26 bit security attribution
pub type Psarb26W<'a, REG> = crate::BitWriter<'a, REG, Psarb26>;
impl<'a, REG> Psarb26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psarb26::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psarb26::_1)
    }
}
/**SCI4 and the MSTPCRB.MSTPB27 bit security attribution

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psarb27 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Psarb27> for bool {
    #[inline(always)]
    fn from(variant: Psarb27) -> Self {
        variant as u8 != 0
    }
}
///Field `PSARB27` reader - SCI4 and the MSTPCRB.MSTPB27 bit security attribution
pub type Psarb27R = crate::BitReader<Psarb27>;
impl Psarb27R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Psarb27 {
        match self.bits {
            false => Psarb27::_0,
            true => Psarb27::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psarb27::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psarb27::_1
    }
}
///Field `PSARB27` writer - SCI4 and the MSTPCRB.MSTPB27 bit security attribution
pub type Psarb27W<'a, REG> = crate::BitWriter<'a, REG, Psarb27>;
impl<'a, REG> Psarb27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psarb27::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psarb27::_1)
    }
}
/**SCI3 and the MSTPCRB.MSTPB28 bit security attribution

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psarb28 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Psarb28> for bool {
    #[inline(always)]
    fn from(variant: Psarb28) -> Self {
        variant as u8 != 0
    }
}
///Field `PSARB28` reader - SCI3 and the MSTPCRB.MSTPB28 bit security attribution
pub type Psarb28R = crate::BitReader<Psarb28>;
impl Psarb28R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Psarb28 {
        match self.bits {
            false => Psarb28::_0,
            true => Psarb28::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psarb28::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psarb28::_1
    }
}
///Field `PSARB28` writer - SCI3 and the MSTPCRB.MSTPB28 bit security attribution
pub type Psarb28W<'a, REG> = crate::BitWriter<'a, REG, Psarb28>;
impl<'a, REG> Psarb28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psarb28::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psarb28::_1)
    }
}
/**SCI2 and the MSTPCRB.MSTPB29 bit security attribution

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psarb29 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Psarb29> for bool {
    #[inline(always)]
    fn from(variant: Psarb29) -> Self {
        variant as u8 != 0
    }
}
///Field `PSARB29` reader - SCI2 and the MSTPCRB.MSTPB29 bit security attribution
pub type Psarb29R = crate::BitReader<Psarb29>;
impl Psarb29R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Psarb29 {
        match self.bits {
            false => Psarb29::_0,
            true => Psarb29::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psarb29::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psarb29::_1
    }
}
///Field `PSARB29` writer - SCI2 and the MSTPCRB.MSTPB29 bit security attribution
pub type Psarb29W<'a, REG> = crate::BitWriter<'a, REG, Psarb29>;
impl<'a, REG> Psarb29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psarb29::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psarb29::_1)
    }
}
/**SCI1 and the MSTPCRB.MSTPB30 bit security attribution

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psarb30 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Psarb30> for bool {
    #[inline(always)]
    fn from(variant: Psarb30) -> Self {
        variant as u8 != 0
    }
}
///Field `PSARB30` reader - SCI1 and the MSTPCRB.MSTPB30 bit security attribution
pub type Psarb30R = crate::BitReader<Psarb30>;
impl Psarb30R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Psarb30 {
        match self.bits {
            false => Psarb30::_0,
            true => Psarb30::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psarb30::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psarb30::_1
    }
}
///Field `PSARB30` writer - SCI1 and the MSTPCRB.MSTPB30 bit security attribution
pub type Psarb30W<'a, REG> = crate::BitWriter<'a, REG, Psarb30>;
impl<'a, REG> Psarb30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psarb30::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psarb30::_1)
    }
}
/**SCI0 and the MSTPCRB.MSTPB31 bit security attribution

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psarb31 {
    ///0: Secure
    _0 = 0,
    ///1: Non-secure
    _1 = 1,
}
impl From<Psarb31> for bool {
    #[inline(always)]
    fn from(variant: Psarb31) -> Self {
        variant as u8 != 0
    }
}
///Field `PSARB31` reader - SCI0 and the MSTPCRB.MSTPB31 bit security attribution
pub type Psarb31R = crate::BitReader<Psarb31>;
impl Psarb31R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Psarb31 {
        match self.bits {
            false => Psarb31::_0,
            true => Psarb31::_1,
        }
    }
    ///Secure
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Psarb31::_0
    }
    ///Non-secure
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Psarb31::_1
    }
}
///Field `PSARB31` writer - SCI0 and the MSTPCRB.MSTPB31 bit security attribution
pub type Psarb31W<'a, REG> = crate::BitWriter<'a, REG, Psarb31>;
impl<'a, REG> Psarb31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Secure
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Psarb31::_0)
    }
    ///Non-secure
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Psarb31::_1)
    }
}
impl R {
    ///Bit 3 - CEC and the MSTPCRB.MSTPB3 bit security attribution
    #[inline(always)]
    pub fn psarb3(&self) -> Psarb3R {
        Psarb3R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 6 - QSPI and the MSTPCRB.MSTPB6 bit security attribution
    #[inline(always)]
    pub fn psarb6(&self) -> Psarb6R {
        Psarb6R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - IIC2 and the MSTPCRB.MSTPB7 bit security attribution
    #[inline(always)]
    pub fn psarb7(&self) -> Psarb7R {
        Psarb7R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - IIC1 and the MSTPCRB.MSTPB8 bit security attribution
    #[inline(always)]
    pub fn psarb8(&self) -> Psarb8R {
        Psarb8R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - IIC0 and the MSTPCRB.MSTPB9 bit security attribution
    #[inline(always)]
    pub fn psarb9(&self) -> Psarb9R {
        Psarb9R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - USBFS and the MSTPCRB.MSTPB11 bit security attribution
    #[inline(always)]
    pub fn psarb11(&self) -> Psarb11R {
        Psarb11R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - USBHS and the MSTPCRB.MSTPB12 bit security attribution
    #[inline(always)]
    pub fn psarb12(&self) -> Psarb12R {
        Psarb12R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 15 - ETHER0/EDMAC0, the MSTPCRB.MSTPB15 bit and the PFENET.PHYMODE0 bit security attribution
    #[inline(always)]
    pub fn psarb15(&self) -> Psarb15R {
        Psarb15R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - OSPI and the MSTPCRB.MSTPB16 bit security attribution
    #[inline(always)]
    pub fn psarb16(&self) -> Psarb16R {
        Psarb16R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - SPI1 and the MSTPCRB.MSTPB18 bit security attribution
    #[inline(always)]
    pub fn psarb18(&self) -> Psarb18R {
        Psarb18R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - SPI0 and the MSTPCRB.MSTPB19 bit security attribution
    #[inline(always)]
    pub fn psarb19(&self) -> Psarb19R {
        Psarb19R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 22 - SCI9 and the MSTPCRB.MSTPB22 bit security attribution
    #[inline(always)]
    pub fn psarb22(&self) -> Psarb22R {
        Psarb22R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - SCI8 and the MSTPCRB.MSTPB23 bit security attribution
    #[inline(always)]
    pub fn psarb23(&self) -> Psarb23R {
        Psarb23R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - SCI7 and the MSTPCRB.MSTPB24 bit security attribution
    #[inline(always)]
    pub fn psarb24(&self) -> Psarb24R {
        Psarb24R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - SCI6 and the MSTPCRB.MSTPB25 bit security attribution
    #[inline(always)]
    pub fn psarb25(&self) -> Psarb25R {
        Psarb25R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - SCI5 and the MSTPCRB.MSTPB26 bit security attribution
    #[inline(always)]
    pub fn psarb26(&self) -> Psarb26R {
        Psarb26R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - SCI4 and the MSTPCRB.MSTPB27 bit security attribution
    #[inline(always)]
    pub fn psarb27(&self) -> Psarb27R {
        Psarb27R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - SCI3 and the MSTPCRB.MSTPB28 bit security attribution
    #[inline(always)]
    pub fn psarb28(&self) -> Psarb28R {
        Psarb28R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - SCI2 and the MSTPCRB.MSTPB29 bit security attribution
    #[inline(always)]
    pub fn psarb29(&self) -> Psarb29R {
        Psarb29R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - SCI1 and the MSTPCRB.MSTPB30 bit security attribution
    #[inline(always)]
    pub fn psarb30(&self) -> Psarb30R {
        Psarb30R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - SCI0 and the MSTPCRB.MSTPB31 bit security attribution
    #[inline(always)]
    pub fn psarb31(&self) -> Psarb31R {
        Psarb31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSARB")
            .field("psarb3", &self.psarb3())
            .field("psarb6", &self.psarb6())
            .field("psarb7", &self.psarb7())
            .field("psarb8", &self.psarb8())
            .field("psarb9", &self.psarb9())
            .field("psarb11", &self.psarb11())
            .field("psarb12", &self.psarb12())
            .field("psarb15", &self.psarb15())
            .field("psarb16", &self.psarb16())
            .field("psarb18", &self.psarb18())
            .field("psarb19", &self.psarb19())
            .field("psarb22", &self.psarb22())
            .field("psarb23", &self.psarb23())
            .field("psarb24", &self.psarb24())
            .field("psarb25", &self.psarb25())
            .field("psarb26", &self.psarb26())
            .field("psarb27", &self.psarb27())
            .field("psarb28", &self.psarb28())
            .field("psarb29", &self.psarb29())
            .field("psarb30", &self.psarb30())
            .field("psarb31", &self.psarb31())
            .finish()
    }
}
impl W {
    ///Bit 3 - CEC and the MSTPCRB.MSTPB3 bit security attribution
    #[inline(always)]
    pub fn psarb3(&mut self) -> Psarb3W<PsarbSpec> {
        Psarb3W::new(self, 3)
    }
    ///Bit 7 - IIC2 and the MSTPCRB.MSTPB7 bit security attribution
    #[inline(always)]
    pub fn psarb7(&mut self) -> Psarb7W<PsarbSpec> {
        Psarb7W::new(self, 7)
    }
    ///Bit 8 - IIC1 and the MSTPCRB.MSTPB8 bit security attribution
    #[inline(always)]
    pub fn psarb8(&mut self) -> Psarb8W<PsarbSpec> {
        Psarb8W::new(self, 8)
    }
    ///Bit 9 - IIC0 and the MSTPCRB.MSTPB9 bit security attribution
    #[inline(always)]
    pub fn psarb9(&mut self) -> Psarb9W<PsarbSpec> {
        Psarb9W::new(self, 9)
    }
    ///Bit 11 - USBFS and the MSTPCRB.MSTPB11 bit security attribution
    #[inline(always)]
    pub fn psarb11(&mut self) -> Psarb11W<PsarbSpec> {
        Psarb11W::new(self, 11)
    }
    ///Bit 12 - USBHS and the MSTPCRB.MSTPB12 bit security attribution
    #[inline(always)]
    pub fn psarb12(&mut self) -> Psarb12W<PsarbSpec> {
        Psarb12W::new(self, 12)
    }
    ///Bit 18 - SPI1 and the MSTPCRB.MSTPB18 bit security attribution
    #[inline(always)]
    pub fn psarb18(&mut self) -> Psarb18W<PsarbSpec> {
        Psarb18W::new(self, 18)
    }
    ///Bit 19 - SPI0 and the MSTPCRB.MSTPB19 bit security attribution
    #[inline(always)]
    pub fn psarb19(&mut self) -> Psarb19W<PsarbSpec> {
        Psarb19W::new(self, 19)
    }
    ///Bit 22 - SCI9 and the MSTPCRB.MSTPB22 bit security attribution
    #[inline(always)]
    pub fn psarb22(&mut self) -> Psarb22W<PsarbSpec> {
        Psarb22W::new(self, 22)
    }
    ///Bit 23 - SCI8 and the MSTPCRB.MSTPB23 bit security attribution
    #[inline(always)]
    pub fn psarb23(&mut self) -> Psarb23W<PsarbSpec> {
        Psarb23W::new(self, 23)
    }
    ///Bit 24 - SCI7 and the MSTPCRB.MSTPB24 bit security attribution
    #[inline(always)]
    pub fn psarb24(&mut self) -> Psarb24W<PsarbSpec> {
        Psarb24W::new(self, 24)
    }
    ///Bit 25 - SCI6 and the MSTPCRB.MSTPB25 bit security attribution
    #[inline(always)]
    pub fn psarb25(&mut self) -> Psarb25W<PsarbSpec> {
        Psarb25W::new(self, 25)
    }
    ///Bit 26 - SCI5 and the MSTPCRB.MSTPB26 bit security attribution
    #[inline(always)]
    pub fn psarb26(&mut self) -> Psarb26W<PsarbSpec> {
        Psarb26W::new(self, 26)
    }
    ///Bit 27 - SCI4 and the MSTPCRB.MSTPB27 bit security attribution
    #[inline(always)]
    pub fn psarb27(&mut self) -> Psarb27W<PsarbSpec> {
        Psarb27W::new(self, 27)
    }
    ///Bit 28 - SCI3 and the MSTPCRB.MSTPB28 bit security attribution
    #[inline(always)]
    pub fn psarb28(&mut self) -> Psarb28W<PsarbSpec> {
        Psarb28W::new(self, 28)
    }
    ///Bit 29 - SCI2 and the MSTPCRB.MSTPB29 bit security attribution
    #[inline(always)]
    pub fn psarb29(&mut self) -> Psarb29W<PsarbSpec> {
        Psarb29W::new(self, 29)
    }
    ///Bit 30 - SCI1 and the MSTPCRB.MSTPB30 bit security attribution
    #[inline(always)]
    pub fn psarb30(&mut self) -> Psarb30W<PsarbSpec> {
        Psarb30W::new(self, 30)
    }
    ///Bit 31 - SCI0 and the MSTPCRB.MSTPB31 bit security attribution
    #[inline(always)]
    pub fn psarb31(&mut self) -> Psarb31W<PsarbSpec> {
        Psarb31W::new(self, 31)
    }
}
/**Peripheral Security Attribution Register B

You can [`read`](crate::Reg::read) this register and get [`psarb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psarb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PsarbSpec;
impl crate::RegisterSpec for PsarbSpec {
    type Ux = u32;
}
///`read()` method returns [`psarb::R`](R) reader structure
impl crate::Readable for PsarbSpec {}
///`write(|w| ..)` method takes [`psarb::W`](W) writer structure
impl crate::Writable for PsarbSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PSARB to value 0xffff_ffff
impl crate::Resettable for PsarbSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
