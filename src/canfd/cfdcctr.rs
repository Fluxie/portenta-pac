///Register `CFDC%sCTR` reader
pub type R = crate::R<CfdcctrSpec>;
///Register `CFDC%sCTR` writer
pub type W = crate::W<CfdcctrSpec>;
/**Channel Mode Control

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Chmdc {
    ///0: Channel operation mode request
    _00 = 0,
    ///1: Channel reset request
    _01 = 1,
    ///2: Channel halt request
    _10 = 2,
    ///3: Keep current value
    _11 = 3,
}
impl From<Chmdc> for u8 {
    #[inline(always)]
    fn from(variant: Chmdc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Chmdc {
    type Ux = u8;
}
impl crate::IsEnum for Chmdc {}
///Field `CHMDC` reader - Channel Mode Control
pub type ChmdcR = crate::FieldReader<Chmdc>;
impl ChmdcR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Chmdc {
        match self.bits {
            0 => Chmdc::_00,
            1 => Chmdc::_01,
            2 => Chmdc::_10,
            3 => Chmdc::_11,
            _ => unreachable!(),
        }
    }
    ///Channel operation mode request
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Chmdc::_00
    }
    ///Channel reset request
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Chmdc::_01
    }
    ///Channel halt request
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Chmdc::_10
    }
    ///Keep current value
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Chmdc::_11
    }
}
///Field `CHMDC` writer - Channel Mode Control
pub type ChmdcW<'a, REG> = crate::FieldWriter<'a, REG, 2, Chmdc, crate::Safe>;
impl<'a, REG> ChmdcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Channel operation mode request
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Chmdc::_00)
    }
    ///Channel reset request
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Chmdc::_01)
    }
    ///Channel halt request
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Chmdc::_10)
    }
    ///Keep current value
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Chmdc::_11)
    }
}
/**Channel Sleep Request

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cslpr {
    ///0: Channel sleep request disabled
    _0 = 0,
    ///1: Channel sleep request enabled
    _1 = 1,
}
impl From<Cslpr> for bool {
    #[inline(always)]
    fn from(variant: Cslpr) -> Self {
        variant as u8 != 0
    }
}
///Field `CSLPR` reader - Channel Sleep Request
pub type CslprR = crate::BitReader<Cslpr>;
impl CslprR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cslpr {
        match self.bits {
            false => Cslpr::_0,
            true => Cslpr::_1,
        }
    }
    ///Channel sleep request disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cslpr::_0
    }
    ///Channel sleep request enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cslpr::_1
    }
}
///Field `CSLPR` writer - Channel Sleep Request
pub type CslprW<'a, REG> = crate::BitWriter<'a, REG, Cslpr>;
impl<'a, REG> CslprW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Channel sleep request disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cslpr::_0)
    }
    ///Channel sleep request enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cslpr::_1)
    }
}
/**Return from Bus-Off

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtbo {
    ///0: Channel is not forced to return from bus-off
    _0 = 0,
    ///1: Channel is forced to return from bus-off
    _1 = 1,
}
impl From<Rtbo> for bool {
    #[inline(always)]
    fn from(variant: Rtbo) -> Self {
        variant as u8 != 0
    }
}
///Field `RTBO` reader - Return from Bus-Off
pub type RtboR = crate::BitReader<Rtbo>;
impl RtboR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rtbo {
        match self.bits {
            false => Rtbo::_0,
            true => Rtbo::_1,
        }
    }
    ///Channel is not forced to return from bus-off
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rtbo::_0
    }
    ///Channel is forced to return from bus-off
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rtbo::_1
    }
}
///Field `RTBO` writer - Return from Bus-Off
pub type RtboW<'a, REG> = crate::BitWriter<'a, REG, Rtbo>;
impl<'a, REG> RtboW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Channel is not forced to return from bus-off
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rtbo::_0)
    }
    ///Channel is forced to return from bus-off
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtbo::_1)
    }
}
/**Bus Error Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Beie {
    ///0: Bus error interrupt disabled
    _0 = 0,
    ///1: Bus error interrupt enabled
    _1 = 1,
}
impl From<Beie> for bool {
    #[inline(always)]
    fn from(variant: Beie) -> Self {
        variant as u8 != 0
    }
}
///Field `BEIE` reader - Bus Error Interrupt Enable
pub type BeieR = crate::BitReader<Beie>;
impl BeieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Beie {
        match self.bits {
            false => Beie::_0,
            true => Beie::_1,
        }
    }
    ///Bus error interrupt disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Beie::_0
    }
    ///Bus error interrupt enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Beie::_1
    }
}
///Field `BEIE` writer - Bus Error Interrupt Enable
pub type BeieW<'a, REG> = crate::BitWriter<'a, REG, Beie>;
impl<'a, REG> BeieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Bus error interrupt disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Beie::_0)
    }
    ///Bus error interrupt enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Beie::_1)
    }
}
/**Error Warning Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ewie {
    ///0: Error warning interrupt disabled
    _0 = 0,
    ///1: Error warning interrupt enabled
    _1 = 1,
}
impl From<Ewie> for bool {
    #[inline(always)]
    fn from(variant: Ewie) -> Self {
        variant as u8 != 0
    }
}
///Field `EWIE` reader - Error Warning Interrupt Enable
pub type EwieR = crate::BitReader<Ewie>;
impl EwieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ewie {
        match self.bits {
            false => Ewie::_0,
            true => Ewie::_1,
        }
    }
    ///Error warning interrupt disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ewie::_0
    }
    ///Error warning interrupt enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ewie::_1
    }
}
///Field `EWIE` writer - Error Warning Interrupt Enable
pub type EwieW<'a, REG> = crate::BitWriter<'a, REG, Ewie>;
impl<'a, REG> EwieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Error warning interrupt disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ewie::_0)
    }
    ///Error warning interrupt enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ewie::_1)
    }
}
/**Error Passive Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Epie {
    ///0: Error passive interrupt disabled
    _0 = 0,
    ///1: Error passive interrupt enabled
    _1 = 1,
}
impl From<Epie> for bool {
    #[inline(always)]
    fn from(variant: Epie) -> Self {
        variant as u8 != 0
    }
}
///Field `EPIE` reader - Error Passive Interrupt Enable
pub type EpieR = crate::BitReader<Epie>;
impl EpieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Epie {
        match self.bits {
            false => Epie::_0,
            true => Epie::_1,
        }
    }
    ///Error passive interrupt disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Epie::_0
    }
    ///Error passive interrupt enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Epie::_1
    }
}
///Field `EPIE` writer - Error Passive Interrupt Enable
pub type EpieW<'a, REG> = crate::BitWriter<'a, REG, Epie>;
impl<'a, REG> EpieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Error passive interrupt disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Epie::_0)
    }
    ///Error passive interrupt enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Epie::_1)
    }
}
/**Bus-Off Entry Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Boeie {
    ///0: Bus-off entry interrupt disabled
    _0 = 0,
    ///1: Bus-off entry interrupt enabled
    _1 = 1,
}
impl From<Boeie> for bool {
    #[inline(always)]
    fn from(variant: Boeie) -> Self {
        variant as u8 != 0
    }
}
///Field `BOEIE` reader - Bus-Off Entry Interrupt Enable
pub type BoeieR = crate::BitReader<Boeie>;
impl BoeieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Boeie {
        match self.bits {
            false => Boeie::_0,
            true => Boeie::_1,
        }
    }
    ///Bus-off entry interrupt disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Boeie::_0
    }
    ///Bus-off entry interrupt enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Boeie::_1
    }
}
///Field `BOEIE` writer - Bus-Off Entry Interrupt Enable
pub type BoeieW<'a, REG> = crate::BitWriter<'a, REG, Boeie>;
impl<'a, REG> BoeieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Bus-off entry interrupt disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Boeie::_0)
    }
    ///Bus-off entry interrupt enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Boeie::_1)
    }
}
/**Bus-Off Recovery Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Borie {
    ///0: Bus-off recovery interrupt disabled
    _0 = 0,
    ///1: Bus-off recovery interrupt enabled
    _1 = 1,
}
impl From<Borie> for bool {
    #[inline(always)]
    fn from(variant: Borie) -> Self {
        variant as u8 != 0
    }
}
///Field `BORIE` reader - Bus-Off Recovery Interrupt Enable
pub type BorieR = crate::BitReader<Borie>;
impl BorieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Borie {
        match self.bits {
            false => Borie::_0,
            true => Borie::_1,
        }
    }
    ///Bus-off recovery interrupt disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Borie::_0
    }
    ///Bus-off recovery interrupt enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Borie::_1
    }
}
///Field `BORIE` writer - Bus-Off Recovery Interrupt Enable
pub type BorieW<'a, REG> = crate::BitWriter<'a, REG, Borie>;
impl<'a, REG> BorieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Bus-off recovery interrupt disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Borie::_0)
    }
    ///Bus-off recovery interrupt enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Borie::_1)
    }
}
/**Overload Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Olie {
    ///0: Overload interrupt disabled
    _0 = 0,
    ///1: Overload interrupt enabled
    _1 = 1,
}
impl From<Olie> for bool {
    #[inline(always)]
    fn from(variant: Olie) -> Self {
        variant as u8 != 0
    }
}
///Field `OLIE` reader - Overload Interrupt Enable
pub type OlieR = crate::BitReader<Olie>;
impl OlieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Olie {
        match self.bits {
            false => Olie::_0,
            true => Olie::_1,
        }
    }
    ///Overload interrupt disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Olie::_0
    }
    ///Overload interrupt enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Olie::_1
    }
}
///Field `OLIE` writer - Overload Interrupt Enable
pub type OlieW<'a, REG> = crate::BitWriter<'a, REG, Olie>;
impl<'a, REG> OlieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Overload interrupt disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Olie::_0)
    }
    ///Overload interrupt enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Olie::_1)
    }
}
/**Bus Lock Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Blie {
    ///0: Bus lock interrupt disabled
    _0 = 0,
    ///1: Bus lock interrupt enabled
    _1 = 1,
}
impl From<Blie> for bool {
    #[inline(always)]
    fn from(variant: Blie) -> Self {
        variant as u8 != 0
    }
}
///Field `BLIE` reader - Bus Lock Interrupt Enable
pub type BlieR = crate::BitReader<Blie>;
impl BlieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Blie {
        match self.bits {
            false => Blie::_0,
            true => Blie::_1,
        }
    }
    ///Bus lock interrupt disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Blie::_0
    }
    ///Bus lock interrupt enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Blie::_1
    }
}
///Field `BLIE` writer - Bus Lock Interrupt Enable
pub type BlieW<'a, REG> = crate::BitWriter<'a, REG, Blie>;
impl<'a, REG> BlieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Bus lock interrupt disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Blie::_0)
    }
    ///Bus lock interrupt enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Blie::_1)
    }
}
/**Arbitration Lost Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alie {
    ///0: Arbitration lost interrupt disabled
    _0 = 0,
    ///1: Arbitration lost interrupt enabled
    _1 = 1,
}
impl From<Alie> for bool {
    #[inline(always)]
    fn from(variant: Alie) -> Self {
        variant as u8 != 0
    }
}
///Field `ALIE` reader - Arbitration Lost Interrupt Enable
pub type AlieR = crate::BitReader<Alie>;
impl AlieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Alie {
        match self.bits {
            false => Alie::_0,
            true => Alie::_1,
        }
    }
    ///Arbitration lost interrupt disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Alie::_0
    }
    ///Arbitration lost interrupt enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Alie::_1
    }
}
///Field `ALIE` writer - Arbitration Lost Interrupt Enable
pub type AlieW<'a, REG> = crate::BitWriter<'a, REG, Alie>;
impl<'a, REG> AlieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Arbitration lost interrupt disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Alie::_0)
    }
    ///Arbitration lost interrupt enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Alie::_1)
    }
}
/**Transmission Abort Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Taie {
    ///0: TX abort interrupt disabled
    _0 = 0,
    ///1: TX abort interrupt enabled
    _1 = 1,
}
impl From<Taie> for bool {
    #[inline(always)]
    fn from(variant: Taie) -> Self {
        variant as u8 != 0
    }
}
///Field `TAIE` reader - Transmission Abort Interrupt Enable
pub type TaieR = crate::BitReader<Taie>;
impl TaieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Taie {
        match self.bits {
            false => Taie::_0,
            true => Taie::_1,
        }
    }
    ///TX abort interrupt disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Taie::_0
    }
    ///TX abort interrupt enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Taie::_1
    }
}
///Field `TAIE` writer - Transmission Abort Interrupt Enable
pub type TaieW<'a, REG> = crate::BitWriter<'a, REG, Taie>;
impl<'a, REG> TaieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///TX abort interrupt disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Taie::_0)
    }
    ///TX abort interrupt enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Taie::_1)
    }
}
/**Error Occurrence Counter Overflow Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eocoie {
    ///0: Error occurrence counter overflow interrupt disabled
    _0 = 0,
    ///1: Error occurrence counter overflow interrupt enabled
    _1 = 1,
}
impl From<Eocoie> for bool {
    #[inline(always)]
    fn from(variant: Eocoie) -> Self {
        variant as u8 != 0
    }
}
///Field `EOCOIE` reader - Error Occurrence Counter Overflow Interrupt Enable
pub type EocoieR = crate::BitReader<Eocoie>;
impl EocoieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eocoie {
        match self.bits {
            false => Eocoie::_0,
            true => Eocoie::_1,
        }
    }
    ///Error occurrence counter overflow interrupt disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eocoie::_0
    }
    ///Error occurrence counter overflow interrupt enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eocoie::_1
    }
}
///Field `EOCOIE` writer - Error Occurrence Counter Overflow Interrupt Enable
pub type EocoieW<'a, REG> = crate::BitWriter<'a, REG, Eocoie>;
impl<'a, REG> EocoieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Error occurrence counter overflow interrupt disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eocoie::_0)
    }
    ///Error occurrence counter overflow interrupt enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eocoie::_1)
    }
}
/**Successful Occurrence Counter Overflow Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Socoie {
    ///0: Successful occurrence counter overflow interrupt disabled
    _0 = 0,
    ///1: Successful occurrence counter overflow interrupt enabled
    _1 = 1,
}
impl From<Socoie> for bool {
    #[inline(always)]
    fn from(variant: Socoie) -> Self {
        variant as u8 != 0
    }
}
///Field `SOCOIE` reader - Successful Occurrence Counter Overflow Interrupt Enable
pub type SocoieR = crate::BitReader<Socoie>;
impl SocoieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Socoie {
        match self.bits {
            false => Socoie::_0,
            true => Socoie::_1,
        }
    }
    ///Successful occurrence counter overflow interrupt disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Socoie::_0
    }
    ///Successful occurrence counter overflow interrupt enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Socoie::_1
    }
}
///Field `SOCOIE` writer - Successful Occurrence Counter Overflow Interrupt Enable
pub type SocoieW<'a, REG> = crate::BitWriter<'a, REG, Socoie>;
impl<'a, REG> SocoieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Successful occurrence counter overflow interrupt disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Socoie::_0)
    }
    ///Successful occurrence counter overflow interrupt enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Socoie::_1)
    }
}
/**Transceiver Delay Compensation Violation Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tdcvfie {
    ///0: Transceiver delay compensation violation interrupt disabled
    _0 = 0,
    ///1: Transceiver delay compensation violation interrupt enabled
    _1 = 1,
}
impl From<Tdcvfie> for bool {
    #[inline(always)]
    fn from(variant: Tdcvfie) -> Self {
        variant as u8 != 0
    }
}
///Field `TDCVFIE` reader - Transceiver Delay Compensation Violation Interrupt Enable
pub type TdcvfieR = crate::BitReader<Tdcvfie>;
impl TdcvfieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tdcvfie {
        match self.bits {
            false => Tdcvfie::_0,
            true => Tdcvfie::_1,
        }
    }
    ///Transceiver delay compensation violation interrupt disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tdcvfie::_0
    }
    ///Transceiver delay compensation violation interrupt enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tdcvfie::_1
    }
}
///Field `TDCVFIE` writer - Transceiver Delay Compensation Violation Interrupt Enable
pub type TdcvfieW<'a, REG> = crate::BitWriter<'a, REG, Tdcvfie>;
impl<'a, REG> TdcvfieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Transceiver delay compensation violation interrupt disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tdcvfie::_0)
    }
    ///Transceiver delay compensation violation interrupt enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tdcvfie::_1)
    }
}
/**Channel Bus-Off Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bom {
    ///0: Normal mode (comply with ISO 11898-1)
    _00 = 0,
    ///1: Entry to Halt mode automatically at bus-off start
    _01 = 1,
    ///2: Entry to Halt mode automatically at bus-off end
    _10 = 2,
    ///3: Entry to Halt mode (during bus-off recovery period) by software
    _11 = 3,
}
impl From<Bom> for u8 {
    #[inline(always)]
    fn from(variant: Bom) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bom {
    type Ux = u8;
}
impl crate::IsEnum for Bom {}
///Field `BOM` reader - Channel Bus-Off Mode
pub type BomR = crate::FieldReader<Bom>;
impl BomR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Bom {
        match self.bits {
            0 => Bom::_00,
            1 => Bom::_01,
            2 => Bom::_10,
            3 => Bom::_11,
            _ => unreachable!(),
        }
    }
    ///Normal mode (comply with ISO 11898-1)
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Bom::_00
    }
    ///Entry to Halt mode automatically at bus-off start
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Bom::_01
    }
    ///Entry to Halt mode automatically at bus-off end
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Bom::_10
    }
    ///Entry to Halt mode (during bus-off recovery period) by software
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Bom::_11
    }
}
///Field `BOM` writer - Channel Bus-Off Mode
pub type BomW<'a, REG> = crate::FieldWriter<'a, REG, 2, Bom, crate::Safe>;
impl<'a, REG> BomW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Normal mode (comply with ISO 11898-1)
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Bom::_00)
    }
    ///Entry to Halt mode automatically at bus-off start
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Bom::_01)
    }
    ///Entry to Halt mode automatically at bus-off end
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Bom::_10)
    }
    ///Entry to Halt mode (during bus-off recovery period) by software
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Bom::_11)
    }
}
/**Channel Error Display

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errd {
    ///0: Only the first set of error codes displayed
    _0 = 0,
    ///1: Accumulated error codes displayed
    _1 = 1,
}
impl From<Errd> for bool {
    #[inline(always)]
    fn from(variant: Errd) -> Self {
        variant as u8 != 0
    }
}
///Field `ERRD` reader - Channel Error Display
pub type ErrdR = crate::BitReader<Errd>;
impl ErrdR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Errd {
        match self.bits {
            false => Errd::_0,
            true => Errd::_1,
        }
    }
    ///Only the first set of error codes displayed
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Errd::_0
    }
    ///Accumulated error codes displayed
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Errd::_1
    }
}
///Field `ERRD` writer - Channel Error Display
pub type ErrdW<'a, REG> = crate::BitWriter<'a, REG, Errd>;
impl<'a, REG> ErrdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Only the first set of error codes displayed
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Errd::_0)
    }
    ///Accumulated error codes displayed
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Errd::_1)
    }
}
/**Channel Test Mode Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctme {
    ///0: Channel test mode disabled
    _0 = 0,
    ///1: Channel test mode enabled
    _1 = 1,
}
impl From<Ctme> for bool {
    #[inline(always)]
    fn from(variant: Ctme) -> Self {
        variant as u8 != 0
    }
}
///Field `CTME` reader - Channel Test Mode Enable
pub type CtmeR = crate::BitReader<Ctme>;
impl CtmeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ctme {
        match self.bits {
            false => Ctme::_0,
            true => Ctme::_1,
        }
    }
    ///Channel test mode disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ctme::_0
    }
    ///Channel test mode enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ctme::_1
    }
}
///Field `CTME` writer - Channel Test Mode Enable
pub type CtmeW<'a, REG> = crate::BitWriter<'a, REG, Ctme>;
impl<'a, REG> CtmeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Channel test mode disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ctme::_0)
    }
    ///Channel test mode enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ctme::_1)
    }
}
/**Channel Test Mode Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctms {
    ///0: Basic test mode
    _00 = 0,
    ///1: Listen-only mode
    _01 = 1,
    ///2: Self-test mode 0 (External loopback mode)
    _10 = 2,
    ///3: Self-test mode 1 (Internal loopback mode)
    _11 = 3,
}
impl From<Ctms> for u8 {
    #[inline(always)]
    fn from(variant: Ctms) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctms {
    type Ux = u8;
}
impl crate::IsEnum for Ctms {}
///Field `CTMS` reader - Channel Test Mode Select
pub type CtmsR = crate::FieldReader<Ctms>;
impl CtmsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ctms {
        match self.bits {
            0 => Ctms::_00,
            1 => Ctms::_01,
            2 => Ctms::_10,
            3 => Ctms::_11,
            _ => unreachable!(),
        }
    }
    ///Basic test mode
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Ctms::_00
    }
    ///Listen-only mode
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Ctms::_01
    }
    ///Self-test mode 0 (External loopback mode)
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Ctms::_10
    }
    ///Self-test mode 1 (Internal loopback mode)
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Ctms::_11
    }
}
///Field `CTMS` writer - Channel Test Mode Select
pub type CtmsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ctms, crate::Safe>;
impl<'a, REG> CtmsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Basic test mode
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Ctms::_00)
    }
    ///Listen-only mode
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Ctms::_01)
    }
    ///Self-test mode 0 (External loopback mode)
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Ctms::_10)
    }
    ///Self-test mode 1 (Internal loopback mode)
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Ctms::_11)
    }
}
/**CRC Error Test

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crct {
    ///0: First data bit of reception stream not inverted
    _0 = 0,
    ///1: First data bit of reception stream inverted
    _1 = 1,
}
impl From<Crct> for bool {
    #[inline(always)]
    fn from(variant: Crct) -> Self {
        variant as u8 != 0
    }
}
///Field `CRCT` reader - CRC Error Test
pub type CrctR = crate::BitReader<Crct>;
impl CrctR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Crct {
        match self.bits {
            false => Crct::_0,
            true => Crct::_1,
        }
    }
    ///First data bit of reception stream not inverted
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Crct::_0
    }
    ///First data bit of reception stream inverted
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Crct::_1
    }
}
///Field `CRCT` writer - CRC Error Test
pub type CrctW<'a, REG> = crate::BitWriter<'a, REG, Crct>;
impl<'a, REG> CrctW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///First data bit of reception stream not inverted
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Crct::_0)
    }
    ///First data bit of reception stream inverted
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Crct::_1)
    }
}
/**Restricted Operation Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rom {
    ///0: Restricted operation mode disabled
    _0 = 0,
    ///1: Restricted operation mode enabled
    _1 = 1,
}
impl From<Rom> for bool {
    #[inline(always)]
    fn from(variant: Rom) -> Self {
        variant as u8 != 0
    }
}
///Field `ROM` reader - Restricted Operation Mode
pub type RomR = crate::BitReader<Rom>;
impl RomR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rom {
        match self.bits {
            false => Rom::_0,
            true => Rom::_1,
        }
    }
    ///Restricted operation mode disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rom::_0
    }
    ///Restricted operation mode enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rom::_1
    }
}
///Field `ROM` writer - Restricted Operation Mode
pub type RomW<'a, REG> = crate::BitWriter<'a, REG, Rom>;
impl<'a, REG> RomW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Restricted operation mode disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rom::_0)
    }
    ///Restricted operation mode enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rom::_1)
    }
}
impl R {
    ///Bits 0:1 - Channel Mode Control
    #[inline(always)]
    pub fn chmdc(&self) -> ChmdcR {
        ChmdcR::new((self.bits & 3) as u8)
    }
    ///Bit 2 - Channel Sleep Request
    #[inline(always)]
    pub fn cslpr(&self) -> CslprR {
        CslprR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Return from Bus-Off
    #[inline(always)]
    pub fn rtbo(&self) -> RtboR {
        RtboR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 8 - Bus Error Interrupt Enable
    #[inline(always)]
    pub fn beie(&self) -> BeieR {
        BeieR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Error Warning Interrupt Enable
    #[inline(always)]
    pub fn ewie(&self) -> EwieR {
        EwieR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Error Passive Interrupt Enable
    #[inline(always)]
    pub fn epie(&self) -> EpieR {
        EpieR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Bus-Off Entry Interrupt Enable
    #[inline(always)]
    pub fn boeie(&self) -> BoeieR {
        BoeieR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Bus-Off Recovery Interrupt Enable
    #[inline(always)]
    pub fn borie(&self) -> BorieR {
        BorieR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Overload Interrupt Enable
    #[inline(always)]
    pub fn olie(&self) -> OlieR {
        OlieR::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Bus Lock Interrupt Enable
    #[inline(always)]
    pub fn blie(&self) -> BlieR {
        BlieR::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Arbitration Lost Interrupt Enable
    #[inline(always)]
    pub fn alie(&self) -> AlieR {
        AlieR::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Transmission Abort Interrupt Enable
    #[inline(always)]
    pub fn taie(&self) -> TaieR {
        TaieR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Error Occurrence Counter Overflow Interrupt Enable
    #[inline(always)]
    pub fn eocoie(&self) -> EocoieR {
        EocoieR::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Successful Occurrence Counter Overflow Interrupt Enable
    #[inline(always)]
    pub fn socoie(&self) -> SocoieR {
        SocoieR::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Transceiver Delay Compensation Violation Interrupt Enable
    #[inline(always)]
    pub fn tdcvfie(&self) -> TdcvfieR {
        TdcvfieR::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bits 21:22 - Channel Bus-Off Mode
    #[inline(always)]
    pub fn bom(&self) -> BomR {
        BomR::new(((self.bits >> 21) & 3) as u8)
    }
    ///Bit 23 - Channel Error Display
    #[inline(always)]
    pub fn errd(&self) -> ErrdR {
        ErrdR::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Channel Test Mode Enable
    #[inline(always)]
    pub fn ctme(&self) -> CtmeR {
        CtmeR::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 25:26 - Channel Test Mode Select
    #[inline(always)]
    pub fn ctms(&self) -> CtmsR {
        CtmsR::new(((self.bits >> 25) & 3) as u8)
    }
    ///Bit 30 - CRC Error Test
    #[inline(always)]
    pub fn crct(&self) -> CrctR {
        CrctR::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Restricted Operation Mode
    #[inline(always)]
    pub fn rom(&self) -> RomR {
        RomR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDCCTR")
            .field("chmdc", &self.chmdc())
            .field("cslpr", &self.cslpr())
            .field("rtbo", &self.rtbo())
            .field("beie", &self.beie())
            .field("ewie", &self.ewie())
            .field("epie", &self.epie())
            .field("boeie", &self.boeie())
            .field("borie", &self.borie())
            .field("olie", &self.olie())
            .field("blie", &self.blie())
            .field("alie", &self.alie())
            .field("taie", &self.taie())
            .field("eocoie", &self.eocoie())
            .field("socoie", &self.socoie())
            .field("tdcvfie", &self.tdcvfie())
            .field("bom", &self.bom())
            .field("errd", &self.errd())
            .field("ctme", &self.ctme())
            .field("ctms", &self.ctms())
            .field("crct", &self.crct())
            .field("rom", &self.rom())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Channel Mode Control
    #[inline(always)]
    pub fn chmdc(&mut self) -> ChmdcW<CfdcctrSpec> {
        ChmdcW::new(self, 0)
    }
    ///Bit 2 - Channel Sleep Request
    #[inline(always)]
    pub fn cslpr(&mut self) -> CslprW<CfdcctrSpec> {
        CslprW::new(self, 2)
    }
    ///Bit 3 - Return from Bus-Off
    #[inline(always)]
    pub fn rtbo(&mut self) -> RtboW<CfdcctrSpec> {
        RtboW::new(self, 3)
    }
    ///Bit 8 - Bus Error Interrupt Enable
    #[inline(always)]
    pub fn beie(&mut self) -> BeieW<CfdcctrSpec> {
        BeieW::new(self, 8)
    }
    ///Bit 9 - Error Warning Interrupt Enable
    #[inline(always)]
    pub fn ewie(&mut self) -> EwieW<CfdcctrSpec> {
        EwieW::new(self, 9)
    }
    ///Bit 10 - Error Passive Interrupt Enable
    #[inline(always)]
    pub fn epie(&mut self) -> EpieW<CfdcctrSpec> {
        EpieW::new(self, 10)
    }
    ///Bit 11 - Bus-Off Entry Interrupt Enable
    #[inline(always)]
    pub fn boeie(&mut self) -> BoeieW<CfdcctrSpec> {
        BoeieW::new(self, 11)
    }
    ///Bit 12 - Bus-Off Recovery Interrupt Enable
    #[inline(always)]
    pub fn borie(&mut self) -> BorieW<CfdcctrSpec> {
        BorieW::new(self, 12)
    }
    ///Bit 13 - Overload Interrupt Enable
    #[inline(always)]
    pub fn olie(&mut self) -> OlieW<CfdcctrSpec> {
        OlieW::new(self, 13)
    }
    ///Bit 14 - Bus Lock Interrupt Enable
    #[inline(always)]
    pub fn blie(&mut self) -> BlieW<CfdcctrSpec> {
        BlieW::new(self, 14)
    }
    ///Bit 15 - Arbitration Lost Interrupt Enable
    #[inline(always)]
    pub fn alie(&mut self) -> AlieW<CfdcctrSpec> {
        AlieW::new(self, 15)
    }
    ///Bit 16 - Transmission Abort Interrupt Enable
    #[inline(always)]
    pub fn taie(&mut self) -> TaieW<CfdcctrSpec> {
        TaieW::new(self, 16)
    }
    ///Bit 17 - Error Occurrence Counter Overflow Interrupt Enable
    #[inline(always)]
    pub fn eocoie(&mut self) -> EocoieW<CfdcctrSpec> {
        EocoieW::new(self, 17)
    }
    ///Bit 18 - Successful Occurrence Counter Overflow Interrupt Enable
    #[inline(always)]
    pub fn socoie(&mut self) -> SocoieW<CfdcctrSpec> {
        SocoieW::new(self, 18)
    }
    ///Bit 19 - Transceiver Delay Compensation Violation Interrupt Enable
    #[inline(always)]
    pub fn tdcvfie(&mut self) -> TdcvfieW<CfdcctrSpec> {
        TdcvfieW::new(self, 19)
    }
    ///Bits 21:22 - Channel Bus-Off Mode
    #[inline(always)]
    pub fn bom(&mut self) -> BomW<CfdcctrSpec> {
        BomW::new(self, 21)
    }
    ///Bit 23 - Channel Error Display
    #[inline(always)]
    pub fn errd(&mut self) -> ErrdW<CfdcctrSpec> {
        ErrdW::new(self, 23)
    }
    ///Bit 24 - Channel Test Mode Enable
    #[inline(always)]
    pub fn ctme(&mut self) -> CtmeW<CfdcctrSpec> {
        CtmeW::new(self, 24)
    }
    ///Bits 25:26 - Channel Test Mode Select
    #[inline(always)]
    pub fn ctms(&mut self) -> CtmsW<CfdcctrSpec> {
        CtmsW::new(self, 25)
    }
    ///Bit 30 - CRC Error Test
    #[inline(always)]
    pub fn crct(&mut self) -> CrctW<CfdcctrSpec> {
        CrctW::new(self, 30)
    }
    ///Bit 31 - Restricted Operation Mode
    #[inline(always)]
    pub fn rom(&mut self) -> RomW<CfdcctrSpec> {
        RomW::new(self, 31)
    }
}
/**Channel %s Control Registers

You can [`read`](crate::Reg::read) this register and get [`cfdcctr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcctr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdcctrSpec;
impl crate::RegisterSpec for CfdcctrSpec {
    type Ux = u32;
}
///`read()` method returns [`cfdcctr::R`](R) reader structure
impl crate::Readable for CfdcctrSpec {}
///`write(|w| ..)` method takes [`cfdcctr::W`](W) writer structure
impl crate::Writable for CfdcctrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDC%sCTR to value 0x05
impl crate::Resettable for CfdcctrSpec {
    const RESET_VALUE: u32 = 0x05;
}
