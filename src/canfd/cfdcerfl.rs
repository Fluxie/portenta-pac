///Register `CFDC%sERFL` reader
pub type R = crate::R<CfdcerflSpec>;
///Register `CFDC%sERFL` writer
pub type W = crate::W<CfdcerflSpec>;
/**Bus Error Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bef {
    ///0: Channel bus error not detected
    _0 = 0,
    ///1: Channel bus error detected
    _1 = 1,
}
impl From<Bef> for bool {
    #[inline(always)]
    fn from(variant: Bef) -> Self {
        variant as u8 != 0
    }
}
///Field `BEF` reader - Bus Error Flag
pub type BefR = crate::BitReader<Bef>;
impl BefR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Bef {
        match self.bits {
            false => Bef::_0,
            true => Bef::_1,
        }
    }
    ///Channel bus error not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bef::_0
    }
    ///Channel bus error detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bef::_1
    }
}
///Field `BEF` writer - Bus Error Flag
pub type BefW<'a, REG> = crate::BitWriter<'a, REG, Bef>;
impl<'a, REG> BefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Channel bus error not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bef::_0)
    }
    ///Channel bus error detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bef::_1)
    }
}
/**Error Warning Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ewf {
    ///0: Channel error warning not detected
    _0 = 0,
    ///1: Channel error warning detected
    _1 = 1,
}
impl From<Ewf> for bool {
    #[inline(always)]
    fn from(variant: Ewf) -> Self {
        variant as u8 != 0
    }
}
///Field `EWF` reader - Error Warning Flag
pub type EwfR = crate::BitReader<Ewf>;
impl EwfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ewf {
        match self.bits {
            false => Ewf::_0,
            true => Ewf::_1,
        }
    }
    ///Channel error warning not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ewf::_0
    }
    ///Channel error warning detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ewf::_1
    }
}
///Field `EWF` writer - Error Warning Flag
pub type EwfW<'a, REG> = crate::BitWriter<'a, REG, Ewf>;
impl<'a, REG> EwfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Channel error warning not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ewf::_0)
    }
    ///Channel error warning detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ewf::_1)
    }
}
/**Error Passive Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Epf {
    ///0: Channel error passive not detected
    _0 = 0,
    ///1: Channel error passive detected
    _1 = 1,
}
impl From<Epf> for bool {
    #[inline(always)]
    fn from(variant: Epf) -> Self {
        variant as u8 != 0
    }
}
///Field `EPF` reader - Error Passive Flag
pub type EpfR = crate::BitReader<Epf>;
impl EpfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Epf {
        match self.bits {
            false => Epf::_0,
            true => Epf::_1,
        }
    }
    ///Channel error passive not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Epf::_0
    }
    ///Channel error passive detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Epf::_1
    }
}
///Field `EPF` writer - Error Passive Flag
pub type EpfW<'a, REG> = crate::BitWriter<'a, REG, Epf>;
impl<'a, REG> EpfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Channel error passive not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Epf::_0)
    }
    ///Channel error passive detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Epf::_1)
    }
}
/**Bus-Off Entry Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Boef {
    ///0: Channel bus-off entry not detected
    _0 = 0,
    ///1: Channel bus-off entry detected
    _1 = 1,
}
impl From<Boef> for bool {
    #[inline(always)]
    fn from(variant: Boef) -> Self {
        variant as u8 != 0
    }
}
///Field `BOEF` reader - Bus-Off Entry Flag
pub type BoefR = crate::BitReader<Boef>;
impl BoefR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Boef {
        match self.bits {
            false => Boef::_0,
            true => Boef::_1,
        }
    }
    ///Channel bus-off entry not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Boef::_0
    }
    ///Channel bus-off entry detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Boef::_1
    }
}
///Field `BOEF` writer - Bus-Off Entry Flag
pub type BoefW<'a, REG> = crate::BitWriter<'a, REG, Boef>;
impl<'a, REG> BoefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Channel bus-off entry not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Boef::_0)
    }
    ///Channel bus-off entry detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Boef::_1)
    }
}
/**Bus-Off Recovery Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Borf {
    ///0: Channel bus-off recovery not detected
    _0 = 0,
    ///1: Channel bus-off recovery detected
    _1 = 1,
}
impl From<Borf> for bool {
    #[inline(always)]
    fn from(variant: Borf) -> Self {
        variant as u8 != 0
    }
}
///Field `BORF` reader - Bus-Off Recovery Flag
pub type BorfR = crate::BitReader<Borf>;
impl BorfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Borf {
        match self.bits {
            false => Borf::_0,
            true => Borf::_1,
        }
    }
    ///Channel bus-off recovery not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Borf::_0
    }
    ///Channel bus-off recovery detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Borf::_1
    }
}
///Field `BORF` writer - Bus-Off Recovery Flag
pub type BorfW<'a, REG> = crate::BitWriter<'a, REG, Borf>;
impl<'a, REG> BorfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Channel bus-off recovery not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Borf::_0)
    }
    ///Channel bus-off recovery detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Borf::_1)
    }
}
/**Overload Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ovlf {
    ///0: Channel overload not detected
    _0 = 0,
    ///1: Channel overload detected
    _1 = 1,
}
impl From<Ovlf> for bool {
    #[inline(always)]
    fn from(variant: Ovlf) -> Self {
        variant as u8 != 0
    }
}
///Field `OVLF` reader - Overload Flag
pub type OvlfR = crate::BitReader<Ovlf>;
impl OvlfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ovlf {
        match self.bits {
            false => Ovlf::_0,
            true => Ovlf::_1,
        }
    }
    ///Channel overload not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ovlf::_0
    }
    ///Channel overload detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ovlf::_1
    }
}
///Field `OVLF` writer - Overload Flag
pub type OvlfW<'a, REG> = crate::BitWriter<'a, REG, Ovlf>;
impl<'a, REG> OvlfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Channel overload not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ovlf::_0)
    }
    ///Channel overload detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ovlf::_1)
    }
}
/**Bus Lock Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Blf {
    ///0: Channel bus lock not detected
    _0 = 0,
    ///1: Channel bus lock detected
    _1 = 1,
}
impl From<Blf> for bool {
    #[inline(always)]
    fn from(variant: Blf) -> Self {
        variant as u8 != 0
    }
}
///Field `BLF` reader - Bus Lock Flag
pub type BlfR = crate::BitReader<Blf>;
impl BlfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Blf {
        match self.bits {
            false => Blf::_0,
            true => Blf::_1,
        }
    }
    ///Channel bus lock not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Blf::_0
    }
    ///Channel bus lock detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Blf::_1
    }
}
///Field `BLF` writer - Bus Lock Flag
pub type BlfW<'a, REG> = crate::BitWriter<'a, REG, Blf>;
impl<'a, REG> BlfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Channel bus lock not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Blf::_0)
    }
    ///Channel bus lock detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Blf::_1)
    }
}
/**Arbitration Lost Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alf {
    ///0: Channel arbitration lost not detected
    _0 = 0,
    ///1: Channel arbitration lost detected
    _1 = 1,
}
impl From<Alf> for bool {
    #[inline(always)]
    fn from(variant: Alf) -> Self {
        variant as u8 != 0
    }
}
///Field `ALF` reader - Arbitration Lost Flag
pub type AlfR = crate::BitReader<Alf>;
impl AlfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Alf {
        match self.bits {
            false => Alf::_0,
            true => Alf::_1,
        }
    }
    ///Channel arbitration lost not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Alf::_0
    }
    ///Channel arbitration lost detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Alf::_1
    }
}
///Field `ALF` writer - Arbitration Lost Flag
pub type AlfW<'a, REG> = crate::BitWriter<'a, REG, Alf>;
impl<'a, REG> AlfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Channel arbitration lost not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Alf::_0)
    }
    ///Channel arbitration lost detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Alf::_1)
    }
}
/**Stuff Error

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Serr {
    ///0: Channel stuff error not detected
    _0 = 0,
    ///1: Channel stuff error detected
    _1 = 1,
}
impl From<Serr> for bool {
    #[inline(always)]
    fn from(variant: Serr) -> Self {
        variant as u8 != 0
    }
}
///Field `SERR` reader - Stuff Error
pub type SerrR = crate::BitReader<Serr>;
impl SerrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Serr {
        match self.bits {
            false => Serr::_0,
            true => Serr::_1,
        }
    }
    ///Channel stuff error not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Serr::_0
    }
    ///Channel stuff error detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Serr::_1
    }
}
///Field `SERR` writer - Stuff Error
pub type SerrW<'a, REG> = crate::BitWriter<'a, REG, Serr>;
impl<'a, REG> SerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Channel stuff error not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Serr::_0)
    }
    ///Channel stuff error detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Serr::_1)
    }
}
/**Form Error

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ferr {
    ///0: Channel form error not detected
    _0 = 0,
    ///1: Channel form error detected
    _1 = 1,
}
impl From<Ferr> for bool {
    #[inline(always)]
    fn from(variant: Ferr) -> Self {
        variant as u8 != 0
    }
}
///Field `FERR` reader - Form Error
pub type FerrR = crate::BitReader<Ferr>;
impl FerrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ferr {
        match self.bits {
            false => Ferr::_0,
            true => Ferr::_1,
        }
    }
    ///Channel form error not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ferr::_0
    }
    ///Channel form error detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ferr::_1
    }
}
///Field `FERR` writer - Form Error
pub type FerrW<'a, REG> = crate::BitWriter<'a, REG, Ferr>;
impl<'a, REG> FerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Channel form error not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ferr::_0)
    }
    ///Channel form error detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ferr::_1)
    }
}
/**Acknowledge Error

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aerr {
    ///0: Channel acknowledge error not detected
    _0 = 0,
    ///1: Channel acknowledge error detected
    _1 = 1,
}
impl From<Aerr> for bool {
    #[inline(always)]
    fn from(variant: Aerr) -> Self {
        variant as u8 != 0
    }
}
///Field `AERR` reader - Acknowledge Error
pub type AerrR = crate::BitReader<Aerr>;
impl AerrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Aerr {
        match self.bits {
            false => Aerr::_0,
            true => Aerr::_1,
        }
    }
    ///Channel acknowledge error not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Aerr::_0
    }
    ///Channel acknowledge error detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Aerr::_1
    }
}
///Field `AERR` writer - Acknowledge Error
pub type AerrW<'a, REG> = crate::BitWriter<'a, REG, Aerr>;
impl<'a, REG> AerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Channel acknowledge error not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Aerr::_0)
    }
    ///Channel acknowledge error detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Aerr::_1)
    }
}
/**CRC Error

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cerr {
    ///0: Channel CRC error not detected
    _0 = 0,
    ///1: Channel CRC error detected
    _1 = 1,
}
impl From<Cerr> for bool {
    #[inline(always)]
    fn from(variant: Cerr) -> Self {
        variant as u8 != 0
    }
}
///Field `CERR` reader - CRC Error
pub type CerrR = crate::BitReader<Cerr>;
impl CerrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cerr {
        match self.bits {
            false => Cerr::_0,
            true => Cerr::_1,
        }
    }
    ///Channel CRC error not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cerr::_0
    }
    ///Channel CRC error detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cerr::_1
    }
}
///Field `CERR` writer - CRC Error
pub type CerrW<'a, REG> = crate::BitWriter<'a, REG, Cerr>;
impl<'a, REG> CerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Channel CRC error not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cerr::_0)
    }
    ///Channel CRC error detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cerr::_1)
    }
}
/**Bit 1 Error

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B1err {
    ///0: Channel bit 1 error not detected
    _0 = 0,
    ///1: Channel bit 1 error detected
    _1 = 1,
}
impl From<B1err> for bool {
    #[inline(always)]
    fn from(variant: B1err) -> Self {
        variant as u8 != 0
    }
}
///Field `B1ERR` reader - Bit 1 Error
pub type B1errR = crate::BitReader<B1err>;
impl B1errR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> B1err {
        match self.bits {
            false => B1err::_0,
            true => B1err::_1,
        }
    }
    ///Channel bit 1 error not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == B1err::_0
    }
    ///Channel bit 1 error detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == B1err::_1
    }
}
///Field `B1ERR` writer - Bit 1 Error
pub type B1errW<'a, REG> = crate::BitWriter<'a, REG, B1err>;
impl<'a, REG> B1errW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Channel bit 1 error not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(B1err::_0)
    }
    ///Channel bit 1 error detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(B1err::_1)
    }
}
/**Bit 0 Error

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B0err {
    ///0: Channel bit 0 error not detected
    _0 = 0,
    ///1: Channel bit 0 error detected
    _1 = 1,
}
impl From<B0err> for bool {
    #[inline(always)]
    fn from(variant: B0err) -> Self {
        variant as u8 != 0
    }
}
///Field `B0ERR` reader - Bit 0 Error
pub type B0errR = crate::BitReader<B0err>;
impl B0errR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> B0err {
        match self.bits {
            false => B0err::_0,
            true => B0err::_1,
        }
    }
    ///Channel bit 0 error not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == B0err::_0
    }
    ///Channel bit 0 error detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == B0err::_1
    }
}
///Field `B0ERR` writer - Bit 0 Error
pub type B0errW<'a, REG> = crate::BitWriter<'a, REG, B0err>;
impl<'a, REG> B0errW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Channel bit 0 error not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(B0err::_0)
    }
    ///Channel bit 0 error detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(B0err::_1)
    }
}
/**Acknowledge Delimiter Error

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aderr {
    ///0: Channel acknowledge delimiter error not detected
    _0 = 0,
    ///1: Channel acknowledge delimiter error detected
    _1 = 1,
}
impl From<Aderr> for bool {
    #[inline(always)]
    fn from(variant: Aderr) -> Self {
        variant as u8 != 0
    }
}
///Field `ADERR` reader - Acknowledge Delimiter Error
pub type AderrR = crate::BitReader<Aderr>;
impl AderrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Aderr {
        match self.bits {
            false => Aderr::_0,
            true => Aderr::_1,
        }
    }
    ///Channel acknowledge delimiter error not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Aderr::_0
    }
    ///Channel acknowledge delimiter error detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Aderr::_1
    }
}
///Field `ADERR` writer - Acknowledge Delimiter Error
pub type AderrW<'a, REG> = crate::BitWriter<'a, REG, Aderr>;
impl<'a, REG> AderrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Channel acknowledge delimiter error not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Aderr::_0)
    }
    ///Channel acknowledge delimiter error detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Aderr::_1)
    }
}
///Field `CRCREG` reader - CRC Register value
pub type CrcregR = crate::FieldReader<u16>;
impl R {
    ///Bit 0 - Bus Error Flag
    #[inline(always)]
    pub fn bef(&self) -> BefR {
        BefR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Error Warning Flag
    #[inline(always)]
    pub fn ewf(&self) -> EwfR {
        EwfR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Error Passive Flag
    #[inline(always)]
    pub fn epf(&self) -> EpfR {
        EpfR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Bus-Off Entry Flag
    #[inline(always)]
    pub fn boef(&self) -> BoefR {
        BoefR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Bus-Off Recovery Flag
    #[inline(always)]
    pub fn borf(&self) -> BorfR {
        BorfR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Overload Flag
    #[inline(always)]
    pub fn ovlf(&self) -> OvlfR {
        OvlfR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Bus Lock Flag
    #[inline(always)]
    pub fn blf(&self) -> BlfR {
        BlfR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Arbitration Lost Flag
    #[inline(always)]
    pub fn alf(&self) -> AlfR {
        AlfR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Stuff Error
    #[inline(always)]
    pub fn serr(&self) -> SerrR {
        SerrR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Form Error
    #[inline(always)]
    pub fn ferr(&self) -> FerrR {
        FerrR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Acknowledge Error
    #[inline(always)]
    pub fn aerr(&self) -> AerrR {
        AerrR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - CRC Error
    #[inline(always)]
    pub fn cerr(&self) -> CerrR {
        CerrR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Bit 1 Error
    #[inline(always)]
    pub fn b1err(&self) -> B1errR {
        B1errR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Bit 0 Error
    #[inline(always)]
    pub fn b0err(&self) -> B0errR {
        B0errR::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Acknowledge Delimiter Error
    #[inline(always)]
    pub fn aderr(&self) -> AderrR {
        AderrR::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bits 16:30 - CRC Register value
    #[inline(always)]
    pub fn crcreg(&self) -> CrcregR {
        CrcregR::new(((self.bits >> 16) & 0x7fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDCERFL")
            .field("bef", &self.bef())
            .field("ewf", &self.ewf())
            .field("epf", &self.epf())
            .field("boef", &self.boef())
            .field("borf", &self.borf())
            .field("ovlf", &self.ovlf())
            .field("blf", &self.blf())
            .field("alf", &self.alf())
            .field("serr", &self.serr())
            .field("ferr", &self.ferr())
            .field("aerr", &self.aerr())
            .field("cerr", &self.cerr())
            .field("b1err", &self.b1err())
            .field("b0err", &self.b0err())
            .field("aderr", &self.aderr())
            .field("crcreg", &self.crcreg())
            .finish()
    }
}
impl W {
    ///Bit 0 - Bus Error Flag
    #[inline(always)]
    pub fn bef(&mut self) -> BefW<CfdcerflSpec> {
        BefW::new(self, 0)
    }
    ///Bit 1 - Error Warning Flag
    #[inline(always)]
    pub fn ewf(&mut self) -> EwfW<CfdcerflSpec> {
        EwfW::new(self, 1)
    }
    ///Bit 2 - Error Passive Flag
    #[inline(always)]
    pub fn epf(&mut self) -> EpfW<CfdcerflSpec> {
        EpfW::new(self, 2)
    }
    ///Bit 3 - Bus-Off Entry Flag
    #[inline(always)]
    pub fn boef(&mut self) -> BoefW<CfdcerflSpec> {
        BoefW::new(self, 3)
    }
    ///Bit 4 - Bus-Off Recovery Flag
    #[inline(always)]
    pub fn borf(&mut self) -> BorfW<CfdcerflSpec> {
        BorfW::new(self, 4)
    }
    ///Bit 5 - Overload Flag
    #[inline(always)]
    pub fn ovlf(&mut self) -> OvlfW<CfdcerflSpec> {
        OvlfW::new(self, 5)
    }
    ///Bit 6 - Bus Lock Flag
    #[inline(always)]
    pub fn blf(&mut self) -> BlfW<CfdcerflSpec> {
        BlfW::new(self, 6)
    }
    ///Bit 7 - Arbitration Lost Flag
    #[inline(always)]
    pub fn alf(&mut self) -> AlfW<CfdcerflSpec> {
        AlfW::new(self, 7)
    }
    ///Bit 8 - Stuff Error
    #[inline(always)]
    pub fn serr(&mut self) -> SerrW<CfdcerflSpec> {
        SerrW::new(self, 8)
    }
    ///Bit 9 - Form Error
    #[inline(always)]
    pub fn ferr(&mut self) -> FerrW<CfdcerflSpec> {
        FerrW::new(self, 9)
    }
    ///Bit 10 - Acknowledge Error
    #[inline(always)]
    pub fn aerr(&mut self) -> AerrW<CfdcerflSpec> {
        AerrW::new(self, 10)
    }
    ///Bit 11 - CRC Error
    #[inline(always)]
    pub fn cerr(&mut self) -> CerrW<CfdcerflSpec> {
        CerrW::new(self, 11)
    }
    ///Bit 12 - Bit 1 Error
    #[inline(always)]
    pub fn b1err(&mut self) -> B1errW<CfdcerflSpec> {
        B1errW::new(self, 12)
    }
    ///Bit 13 - Bit 0 Error
    #[inline(always)]
    pub fn b0err(&mut self) -> B0errW<CfdcerflSpec> {
        B0errW::new(self, 13)
    }
    ///Bit 14 - Acknowledge Delimiter Error
    #[inline(always)]
    pub fn aderr(&mut self) -> AderrW<CfdcerflSpec> {
        AderrW::new(self, 14)
    }
}
/**Channel %s Error Flag Registers

You can [`read`](crate::Reg::read) this register and get [`cfdcerfl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcerfl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdcerflSpec;
impl crate::RegisterSpec for CfdcerflSpec {
    type Ux = u32;
}
///`read()` method returns [`cfdcerfl::R`](R) reader structure
impl crate::Readable for CfdcerflSpec {}
///`write(|w| ..)` method takes [`cfdcerfl::W`](W) writer structure
impl crate::Writable for CfdcerflSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDC%sERFL to value 0
impl crate::Resettable for CfdcerflSpec {}
