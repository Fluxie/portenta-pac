///Register `EESR` reader
pub type R = crate::R<EesrSpec>;
///Register `EESR` writer
pub type W = crate::W<EesrSpec>;
/**CRC Error Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cerf {
    ///0: CRC error not detected
    _0 = 0,
    ///1: CRC error detected.
    _1 = 1,
}
impl From<Cerf> for bool {
    #[inline(always)]
    fn from(variant: Cerf) -> Self {
        variant as u8 != 0
    }
}
///Field `CERF` reader - CRC Error Flag
pub type CerfR = crate::BitReader<Cerf>;
impl CerfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cerf {
        match self.bits {
            false => Cerf::_0,
            true => Cerf::_1,
        }
    }
    ///CRC error not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cerf::_0
    }
    ///CRC error detected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cerf::_1
    }
}
///Field `CERF` writer - CRC Error Flag
pub type CerfW<'a, REG> = crate::BitWriter<'a, REG, Cerf>;
impl<'a, REG> CerfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CRC error not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cerf::_0)
    }
    ///CRC error detected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cerf::_1)
    }
}
/**PHY-LSI Receive Error Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pre {
    ///0: PHY-LSI receive error not detected
    _0 = 0,
    ///1: PHY-LSI receive error detected.
    _1 = 1,
}
impl From<Pre> for bool {
    #[inline(always)]
    fn from(variant: Pre) -> Self {
        variant as u8 != 0
    }
}
///Field `PRE` reader - PHY-LSI Receive Error Flag
pub type PreR = crate::BitReader<Pre>;
impl PreR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pre {
        match self.bits {
            false => Pre::_0,
            true => Pre::_1,
        }
    }
    ///PHY-LSI receive error not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pre::_0
    }
    ///PHY-LSI receive error detected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pre::_1
    }
}
///Field `PRE` writer - PHY-LSI Receive Error Flag
pub type PreW<'a, REG> = crate::BitWriter<'a, REG, Pre>;
impl<'a, REG> PreW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PHY-LSI receive error not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pre::_0)
    }
    ///PHY-LSI receive error detected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pre::_1)
    }
}
/**Frame-Too-Short Error Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtsf {
    ///0: Frame-too-short error not detected
    _0 = 0,
    ///1: Frame-too-short error detected.
    _1 = 1,
}
impl From<Rtsf> for bool {
    #[inline(always)]
    fn from(variant: Rtsf) -> Self {
        variant as u8 != 0
    }
}
///Field `RTSF` reader - Frame-Too-Short Error Flag
pub type RtsfR = crate::BitReader<Rtsf>;
impl RtsfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rtsf {
        match self.bits {
            false => Rtsf::_0,
            true => Rtsf::_1,
        }
    }
    ///Frame-too-short error not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rtsf::_0
    }
    ///Frame-too-short error detected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rtsf::_1
    }
}
///Field `RTSF` writer - Frame-Too-Short Error Flag
pub type RtsfW<'a, REG> = crate::BitWriter<'a, REG, Rtsf>;
impl<'a, REG> RtsfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Frame-too-short error not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rtsf::_0)
    }
    ///Frame-too-short error detected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtsf::_1)
    }
}
/**Frame-Too-Long Error Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtlf {
    ///0: Frame-too-long error not detected
    _0 = 0,
    ///1: Frame-too-long error detected.
    _1 = 1,
}
impl From<Rtlf> for bool {
    #[inline(always)]
    fn from(variant: Rtlf) -> Self {
        variant as u8 != 0
    }
}
///Field `RTLF` reader - Frame-Too-Long Error Flag
pub type RtlfR = crate::BitReader<Rtlf>;
impl RtlfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rtlf {
        match self.bits {
            false => Rtlf::_0,
            true => Rtlf::_1,
        }
    }
    ///Frame-too-long error not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rtlf::_0
    }
    ///Frame-too-long error detected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rtlf::_1
    }
}
///Field `RTLF` writer - Frame-Too-Long Error Flag
pub type RtlfW<'a, REG> = crate::BitWriter<'a, REG, Rtlf>;
impl<'a, REG> RtlfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Frame-too-long error not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rtlf::_0)
    }
    ///Frame-too-long error detected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtlf::_1)
    }
}
/**Alignment Error Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rrf {
    ///0: Alignment error not detected
    _0 = 0,
    ///1: Alignment error detected.
    _1 = 1,
}
impl From<Rrf> for bool {
    #[inline(always)]
    fn from(variant: Rrf) -> Self {
        variant as u8 != 0
    }
}
///Field `RRF` reader - Alignment Error Flag
pub type RrfR = crate::BitReader<Rrf>;
impl RrfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rrf {
        match self.bits {
            false => Rrf::_0,
            true => Rrf::_1,
        }
    }
    ///Alignment error not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rrf::_0
    }
    ///Alignment error detected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rrf::_1
    }
}
///Field `RRF` writer - Alignment Error Flag
pub type RrfW<'a, REG> = crate::BitWriter<'a, REG, Rrf>;
impl<'a, REG> RrfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Alignment error not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rrf::_0)
    }
    ///Alignment error detected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rrf::_1)
    }
}
/**Multicast Address Frame Receive Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rmaf {
    ///0: Multicast address frame not received
    _0 = 0,
    ///1: Multicast address frame received.
    _1 = 1,
}
impl From<Rmaf> for bool {
    #[inline(always)]
    fn from(variant: Rmaf) -> Self {
        variant as u8 != 0
    }
}
///Field `RMAF` reader - Multicast Address Frame Receive Flag
pub type RmafR = crate::BitReader<Rmaf>;
impl RmafR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rmaf {
        match self.bits {
            false => Rmaf::_0,
            true => Rmaf::_1,
        }
    }
    ///Multicast address frame not received
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rmaf::_0
    }
    ///Multicast address frame received.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rmaf::_1
    }
}
///Field `RMAF` writer - Multicast Address Frame Receive Flag
pub type RmafW<'a, REG> = crate::BitWriter<'a, REG, Rmaf>;
impl<'a, REG> RmafW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Multicast address frame not received
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rmaf::_0)
    }
    ///Multicast address frame received.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rmaf::_1)
    }
}
/**Transmit Retry Over Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tro {
    ///0: Transmit retry-over condition not detected
    _0 = 0,
    ///1: Transmit retry-over condition detected.
    _1 = 1,
}
impl From<Tro> for bool {
    #[inline(always)]
    fn from(variant: Tro) -> Self {
        variant as u8 != 0
    }
}
///Field `TRO` reader - Transmit Retry Over Flag
pub type TroR = crate::BitReader<Tro>;
impl TroR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tro {
        match self.bits {
            false => Tro::_0,
            true => Tro::_1,
        }
    }
    ///Transmit retry-over condition not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tro::_0
    }
    ///Transmit retry-over condition detected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tro::_1
    }
}
///Field `TRO` writer - Transmit Retry Over Flag
pub type TroW<'a, REG> = crate::BitWriter<'a, REG, Tro>;
impl<'a, REG> TroW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Transmit retry-over condition not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tro::_0)
    }
    ///Transmit retry-over condition detected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tro::_1)
    }
}
/**Late Collision Detect Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cd {
    ///0: Late collision not detected
    _0 = 0,
    ///1: Late collision detected during frame transmission.
    _1 = 1,
}
impl From<Cd> for bool {
    #[inline(always)]
    fn from(variant: Cd) -> Self {
        variant as u8 != 0
    }
}
///Field `CD` reader - Late Collision Detect Flag
pub type CdR = crate::BitReader<Cd>;
impl CdR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cd {
        match self.bits {
            false => Cd::_0,
            true => Cd::_1,
        }
    }
    ///Late collision not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cd::_0
    }
    ///Late collision detected during frame transmission.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cd::_1
    }
}
///Field `CD` writer - Late Collision Detect Flag
pub type CdW<'a, REG> = crate::BitWriter<'a, REG, Cd>;
impl<'a, REG> CdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Late collision not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cd::_0)
    }
    ///Late collision detected during frame transmission.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cd::_1)
    }
}
/**Loss of Carrier Detect Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dlc {
    ///0: Loss of carrier not detected
    _0 = 0,
    ///1: Loss of carrier detected during frame transmission.
    _1 = 1,
}
impl From<Dlc> for bool {
    #[inline(always)]
    fn from(variant: Dlc) -> Self {
        variant as u8 != 0
    }
}
///Field `DLC` reader - Loss of Carrier Detect Flag
pub type DlcR = crate::BitReader<Dlc>;
impl DlcR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dlc {
        match self.bits {
            false => Dlc::_0,
            true => Dlc::_1,
        }
    }
    ///Loss of carrier not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dlc::_0
    }
    ///Loss of carrier detected during frame transmission.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dlc::_1
    }
}
///Field `DLC` writer - Loss of Carrier Detect Flag
pub type DlcW<'a, REG> = crate::BitWriter<'a, REG, Dlc>;
impl<'a, REG> DlcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Loss of carrier not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dlc::_0)
    }
    ///Loss of carrier detected during frame transmission.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dlc::_1)
    }
}
/**Carrier Not Detect Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cnd {
    ///0: Carrier detected when transmission started
    _0 = 0,
    ///1: Carrier not detected during preamble transmission.
    _1 = 1,
}
impl From<Cnd> for bool {
    #[inline(always)]
    fn from(variant: Cnd) -> Self {
        variant as u8 != 0
    }
}
///Field `CND` reader - Carrier Not Detect Flag
pub type CndR = crate::BitReader<Cnd>;
impl CndR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cnd {
        match self.bits {
            false => Cnd::_0,
            true => Cnd::_1,
        }
    }
    ///Carrier detected when transmission started
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cnd::_0
    }
    ///Carrier not detected during preamble transmission.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cnd::_1
    }
}
///Field `CND` writer - Carrier Not Detect Flag
pub type CndW<'a, REG> = crate::BitWriter<'a, REG, Cnd>;
impl<'a, REG> CndW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Carrier detected when transmission started
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cnd::_0)
    }
    ///Carrier not detected during preamble transmission.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cnd::_1)
    }
}
/**Receive FIFO Overflow Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfof {
    ///0: No overflow occurred
    _0 = 0,
    ///1: Overflow occurred.
    _1 = 1,
}
impl From<Rfof> for bool {
    #[inline(always)]
    fn from(variant: Rfof) -> Self {
        variant as u8 != 0
    }
}
///Field `RFOF` reader - Receive FIFO Overflow Flag
pub type RfofR = crate::BitReader<Rfof>;
impl RfofR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rfof {
        match self.bits {
            false => Rfof::_0,
            true => Rfof::_1,
        }
    }
    ///No overflow occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rfof::_0
    }
    ///Overflow occurred.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rfof::_1
    }
}
///Field `RFOF` writer - Receive FIFO Overflow Flag
pub type RfofW<'a, REG> = crate::BitWriter<'a, REG, Rfof>;
impl<'a, REG> RfofW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No overflow occurred
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rfof::_0)
    }
    ///Overflow occurred.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rfof::_1)
    }
}
/**Receive Descriptor Empty Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rde {
    ///0: EDMAC detected that the receive descriptor valid bit (RD0.RACT) is 1
    _0 = 0,
    ///1: EDMAC detected that the receive descriptor valid bit (RD0.RACT) is 0.
    _1 = 1,
}
impl From<Rde> for bool {
    #[inline(always)]
    fn from(variant: Rde) -> Self {
        variant as u8 != 0
    }
}
///Field `RDE` reader - Receive Descriptor Empty Flag
pub type RdeR = crate::BitReader<Rde>;
impl RdeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rde {
        match self.bits {
            false => Rde::_0,
            true => Rde::_1,
        }
    }
    ///EDMAC detected that the receive descriptor valid bit (RD0.RACT) is 1
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rde::_0
    }
    ///EDMAC detected that the receive descriptor valid bit (RD0.RACT) is 0.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rde::_1
    }
}
///Field `RDE` writer - Receive Descriptor Empty Flag
pub type RdeW<'a, REG> = crate::BitWriter<'a, REG, Rde>;
impl<'a, REG> RdeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///EDMAC detected that the receive descriptor valid bit (RD0.RACT) is 1
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rde::_0)
    }
    ///EDMAC detected that the receive descriptor valid bit (RD0.RACT) is 0.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rde::_1)
    }
}
/**Frame Receive Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fr {
    ///0: Frame not received
    _0 = 0,
    ///1: Frame received and update of the receive descriptor is complete.
    _1 = 1,
}
impl From<Fr> for bool {
    #[inline(always)]
    fn from(variant: Fr) -> Self {
        variant as u8 != 0
    }
}
///Field `FR` reader - Frame Receive Flag
pub type FrR = crate::BitReader<Fr>;
impl FrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Fr {
        match self.bits {
            false => Fr::_0,
            true => Fr::_1,
        }
    }
    ///Frame not received
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Fr::_0
    }
    ///Frame received and update of the receive descriptor is complete.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Fr::_1
    }
}
///Field `FR` writer - Frame Receive Flag
pub type FrW<'a, REG> = crate::BitWriter<'a, REG, Fr>;
impl<'a, REG> FrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Frame not received
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Fr::_0)
    }
    ///Frame received and update of the receive descriptor is complete.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Fr::_1)
    }
}
/**Transmit FIFO Underflow Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tfuf {
    ///0: No underflow occurred
    _0 = 0,
    ///1: Underflow occurred.
    _1 = 1,
}
impl From<Tfuf> for bool {
    #[inline(always)]
    fn from(variant: Tfuf) -> Self {
        variant as u8 != 0
    }
}
///Field `TFUF` reader - Transmit FIFO Underflow Flag
pub type TfufR = crate::BitReader<Tfuf>;
impl TfufR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tfuf {
        match self.bits {
            false => Tfuf::_0,
            true => Tfuf::_1,
        }
    }
    ///No underflow occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tfuf::_0
    }
    ///Underflow occurred.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tfuf::_1
    }
}
///Field `TFUF` writer - Transmit FIFO Underflow Flag
pub type TfufW<'a, REG> = crate::BitWriter<'a, REG, Tfuf>;
impl<'a, REG> TfufW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No underflow occurred
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tfuf::_0)
    }
    ///Underflow occurred.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tfuf::_1)
    }
}
/**Transmit Descriptor Empty Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tde {
    ///0: EDMAC detected that the transmit descriptor valid bit (TD0.TACT) is 1
    _0 = 0,
    ///1: EDMAC detected that the transmit descriptor valid bit (TD0.TACT) is 0.
    _1 = 1,
}
impl From<Tde> for bool {
    #[inline(always)]
    fn from(variant: Tde) -> Self {
        variant as u8 != 0
    }
}
///Field `TDE` reader - Transmit Descriptor Empty Flag
pub type TdeR = crate::BitReader<Tde>;
impl TdeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tde {
        match self.bits {
            false => Tde::_0,
            true => Tde::_1,
        }
    }
    ///EDMAC detected that the transmit descriptor valid bit (TD0.TACT) is 1
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tde::_0
    }
    ///EDMAC detected that the transmit descriptor valid bit (TD0.TACT) is 0.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tde::_1
    }
}
///Field `TDE` writer - Transmit Descriptor Empty Flag
pub type TdeW<'a, REG> = crate::BitWriter<'a, REG, Tde>;
impl<'a, REG> TdeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///EDMAC detected that the transmit descriptor valid bit (TD0.TACT) is 1
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tde::_0)
    }
    ///EDMAC detected that the transmit descriptor valid bit (TD0.TACT) is 0.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tde::_1)
    }
}
/**Frame Transfer Complete Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tc {
    ///0: Transfer not complete or no transfer requested
    _0 = 0,
    ///1: All frames indicated in the transmit descriptor were completely transferred to the transmit FIFO.
    _1 = 1,
}
impl From<Tc> for bool {
    #[inline(always)]
    fn from(variant: Tc) -> Self {
        variant as u8 != 0
    }
}
///Field `TC` reader - Frame Transfer Complete Flag
pub type TcR = crate::BitReader<Tc>;
impl TcR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tc {
        match self.bits {
            false => Tc::_0,
            true => Tc::_1,
        }
    }
    ///Transfer not complete or no transfer requested
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tc::_0
    }
    ///All frames indicated in the transmit descriptor were completely transferred to the transmit FIFO.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tc::_1
    }
}
///Field `TC` writer - Frame Transfer Complete Flag
pub type TcW<'a, REG> = crate::BitWriter<'a, REG, Tc>;
impl<'a, REG> TcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Transfer not complete or no transfer requested
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tc::_0)
    }
    ///All frames indicated in the transmit descriptor were completely transferred to the transmit FIFO.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tc::_1)
    }
}
/**ETHERC Status Register Source Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eci {
    ///0: ETHERC status interrupt source not detected
    _0 = 0,
    ///1: ETHERC status interrupt source detected.
    _1 = 1,
}
impl From<Eci> for bool {
    #[inline(always)]
    fn from(variant: Eci) -> Self {
        variant as u8 != 0
    }
}
///Field `ECI` reader - ETHERC Status Register Source Flag
pub type EciR = crate::BitReader<Eci>;
impl EciR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eci {
        match self.bits {
            false => Eci::_0,
            true => Eci::_1,
        }
    }
    ///ETHERC status interrupt source not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eci::_0
    }
    ///ETHERC status interrupt source detected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eci::_1
    }
}
/**Address Error Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ade {
    ///0: Invalid memory address not detected (normal operation)
    _0 = 0,
    ///1: Invalid memory address detected.
    _1 = 1,
}
impl From<Ade> for bool {
    #[inline(always)]
    fn from(variant: Ade) -> Self {
        variant as u8 != 0
    }
}
///Field `ADE` reader - Address Error Flag
pub type AdeR = crate::BitReader<Ade>;
impl AdeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ade {
        match self.bits {
            false => Ade::_0,
            true => Ade::_1,
        }
    }
    ///Invalid memory address not detected (normal operation)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ade::_0
    }
    ///Invalid memory address detected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ade::_1
    }
}
///Field `ADE` writer - Address Error Flag
pub type AdeW<'a, REG> = crate::BitWriter<'a, REG, Ade>;
impl<'a, REG> AdeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Invalid memory address not detected (normal operation)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ade::_0)
    }
    ///Invalid memory address detected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ade::_1)
    }
}
/**Receive Frame Counter Overflow Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfcof {
    ///0: Receive frame counter did not overflow
    _0 = 0,
    ///1: Receive frame counter overflowed.
    _1 = 1,
}
impl From<Rfcof> for bool {
    #[inline(always)]
    fn from(variant: Rfcof) -> Self {
        variant as u8 != 0
    }
}
///Field `RFCOF` reader - Receive Frame Counter Overflow Flag
pub type RfcofR = crate::BitReader<Rfcof>;
impl RfcofR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rfcof {
        match self.bits {
            false => Rfcof::_0,
            true => Rfcof::_1,
        }
    }
    ///Receive frame counter did not overflow
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rfcof::_0
    }
    ///Receive frame counter overflowed.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rfcof::_1
    }
}
///Field `RFCOF` writer - Receive Frame Counter Overflow Flag
pub type RfcofW<'a, REG> = crate::BitWriter<'a, REG, Rfcof>;
impl<'a, REG> RfcofW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Receive frame counter did not overflow
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rfcof::_0)
    }
    ///Receive frame counter overflowed.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rfcof::_1)
    }
}
/**Receive Abort Detect Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rabt {
    ///0: Frame reception not aborted or no reception requested
    _0 = 0,
    ///1: Frame reception aborted.
    _1 = 1,
}
impl From<Rabt> for bool {
    #[inline(always)]
    fn from(variant: Rabt) -> Self {
        variant as u8 != 0
    }
}
///Field `RABT` reader - Receive Abort Detect Flag
pub type RabtR = crate::BitReader<Rabt>;
impl RabtR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rabt {
        match self.bits {
            false => Rabt::_0,
            true => Rabt::_1,
        }
    }
    ///Frame reception not aborted or no reception requested
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rabt::_0
    }
    ///Frame reception aborted.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rabt::_1
    }
}
///Field `RABT` writer - Receive Abort Detect Flag
pub type RabtW<'a, REG> = crate::BitWriter<'a, REG, Rabt>;
impl<'a, REG> RabtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Frame reception not aborted or no reception requested
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rabt::_0)
    }
    ///Frame reception aborted.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rabt::_1)
    }
}
/**Transmit Abort Detect Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tabt {
    ///0: Frame transmission not aborted or no transmission requested.
    _0 = 0,
    ///1: Frame transmission aborted.
    _1 = 1,
}
impl From<Tabt> for bool {
    #[inline(always)]
    fn from(variant: Tabt) -> Self {
        variant as u8 != 0
    }
}
///Field `TABT` reader - Transmit Abort Detect Flag
pub type TabtR = crate::BitReader<Tabt>;
impl TabtR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tabt {
        match self.bits {
            false => Tabt::_0,
            true => Tabt::_1,
        }
    }
    ///Frame transmission not aborted or no transmission requested.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tabt::_0
    }
    ///Frame transmission aborted.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tabt::_1
    }
}
///Field `TABT` writer - Transmit Abort Detect Flag
pub type TabtW<'a, REG> = crate::BitWriter<'a, REG, Tabt>;
impl<'a, REG> TabtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Frame transmission not aborted or no transmission requested.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tabt::_0)
    }
    ///Frame transmission aborted.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tabt::_1)
    }
}
/**Write-Back Complete Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Twb {
    ///0: Write-back not complete or no transmission requested
    _0 = 0,
    ///1: Write-back to the transmit descriptor completed.
    _1 = 1,
}
impl From<Twb> for bool {
    #[inline(always)]
    fn from(variant: Twb) -> Self {
        variant as u8 != 0
    }
}
///Field `TWB` reader - Write-Back Complete Flag
pub type TwbR = crate::BitReader<Twb>;
impl TwbR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Twb {
        match self.bits {
            false => Twb::_0,
            true => Twb::_1,
        }
    }
    ///Write-back not complete or no transmission requested
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Twb::_0
    }
    ///Write-back to the transmit descriptor completed.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Twb::_1
    }
}
///Field `TWB` writer - Write-Back Complete Flag
pub type TwbW<'a, REG> = crate::BitWriter<'a, REG, Twb>;
impl<'a, REG> TwbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Write-back not complete or no transmission requested
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Twb::_0)
    }
    ///Write-back to the transmit descriptor completed.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Twb::_1)
    }
}
impl R {
    ///Bit 0 - CRC Error Flag
    #[inline(always)]
    pub fn cerf(&self) -> CerfR {
        CerfR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - PHY-LSI Receive Error Flag
    #[inline(always)]
    pub fn pre(&self) -> PreR {
        PreR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Frame-Too-Short Error Flag
    #[inline(always)]
    pub fn rtsf(&self) -> RtsfR {
        RtsfR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Frame-Too-Long Error Flag
    #[inline(always)]
    pub fn rtlf(&self) -> RtlfR {
        RtlfR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Alignment Error Flag
    #[inline(always)]
    pub fn rrf(&self) -> RrfR {
        RrfR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 7 - Multicast Address Frame Receive Flag
    #[inline(always)]
    pub fn rmaf(&self) -> RmafR {
        RmafR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Transmit Retry Over Flag
    #[inline(always)]
    pub fn tro(&self) -> TroR {
        TroR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Late Collision Detect Flag
    #[inline(always)]
    pub fn cd(&self) -> CdR {
        CdR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Loss of Carrier Detect Flag
    #[inline(always)]
    pub fn dlc(&self) -> DlcR {
        DlcR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Carrier Not Detect Flag
    #[inline(always)]
    pub fn cnd(&self) -> CndR {
        CndR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 16 - Receive FIFO Overflow Flag
    #[inline(always)]
    pub fn rfof(&self) -> RfofR {
        RfofR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Receive Descriptor Empty Flag
    #[inline(always)]
    pub fn rde(&self) -> RdeR {
        RdeR::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Frame Receive Flag
    #[inline(always)]
    pub fn fr(&self) -> FrR {
        FrR::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Transmit FIFO Underflow Flag
    #[inline(always)]
    pub fn tfuf(&self) -> TfufR {
        TfufR::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Transmit Descriptor Empty Flag
    #[inline(always)]
    pub fn tde(&self) -> TdeR {
        TdeR::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Frame Transfer Complete Flag
    #[inline(always)]
    pub fn tc(&self) -> TcR {
        TcR::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - ETHERC Status Register Source Flag
    #[inline(always)]
    pub fn eci(&self) -> EciR {
        EciR::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Address Error Flag
    #[inline(always)]
    pub fn ade(&self) -> AdeR {
        AdeR::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Receive Frame Counter Overflow Flag
    #[inline(always)]
    pub fn rfcof(&self) -> RfcofR {
        RfcofR::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Receive Abort Detect Flag
    #[inline(always)]
    pub fn rabt(&self) -> RabtR {
        RabtR::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Transmit Abort Detect Flag
    #[inline(always)]
    pub fn tabt(&self) -> TabtR {
        TabtR::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 30 - Write-Back Complete Flag
    #[inline(always)]
    pub fn twb(&self) -> TwbR {
        TwbR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EESR")
            .field("cerf", &self.cerf())
            .field("pre", &self.pre())
            .field("rtsf", &self.rtsf())
            .field("rtlf", &self.rtlf())
            .field("rrf", &self.rrf())
            .field("rmaf", &self.rmaf())
            .field("tro", &self.tro())
            .field("cd", &self.cd())
            .field("dlc", &self.dlc())
            .field("cnd", &self.cnd())
            .field("rfof", &self.rfof())
            .field("rde", &self.rde())
            .field("fr", &self.fr())
            .field("tfuf", &self.tfuf())
            .field("tde", &self.tde())
            .field("tc", &self.tc())
            .field("eci", &self.eci())
            .field("ade", &self.ade())
            .field("rfcof", &self.rfcof())
            .field("rabt", &self.rabt())
            .field("tabt", &self.tabt())
            .field("twb", &self.twb())
            .finish()
    }
}
impl W {
    ///Bit 0 - CRC Error Flag
    #[inline(always)]
    pub fn cerf(&mut self) -> CerfW<EesrSpec> {
        CerfW::new(self, 0)
    }
    ///Bit 1 - PHY-LSI Receive Error Flag
    #[inline(always)]
    pub fn pre(&mut self) -> PreW<EesrSpec> {
        PreW::new(self, 1)
    }
    ///Bit 2 - Frame-Too-Short Error Flag
    #[inline(always)]
    pub fn rtsf(&mut self) -> RtsfW<EesrSpec> {
        RtsfW::new(self, 2)
    }
    ///Bit 3 - Frame-Too-Long Error Flag
    #[inline(always)]
    pub fn rtlf(&mut self) -> RtlfW<EesrSpec> {
        RtlfW::new(self, 3)
    }
    ///Bit 4 - Alignment Error Flag
    #[inline(always)]
    pub fn rrf(&mut self) -> RrfW<EesrSpec> {
        RrfW::new(self, 4)
    }
    ///Bit 7 - Multicast Address Frame Receive Flag
    #[inline(always)]
    pub fn rmaf(&mut self) -> RmafW<EesrSpec> {
        RmafW::new(self, 7)
    }
    ///Bit 8 - Transmit Retry Over Flag
    #[inline(always)]
    pub fn tro(&mut self) -> TroW<EesrSpec> {
        TroW::new(self, 8)
    }
    ///Bit 9 - Late Collision Detect Flag
    #[inline(always)]
    pub fn cd(&mut self) -> CdW<EesrSpec> {
        CdW::new(self, 9)
    }
    ///Bit 10 - Loss of Carrier Detect Flag
    #[inline(always)]
    pub fn dlc(&mut self) -> DlcW<EesrSpec> {
        DlcW::new(self, 10)
    }
    ///Bit 11 - Carrier Not Detect Flag
    #[inline(always)]
    pub fn cnd(&mut self) -> CndW<EesrSpec> {
        CndW::new(self, 11)
    }
    ///Bit 16 - Receive FIFO Overflow Flag
    #[inline(always)]
    pub fn rfof(&mut self) -> RfofW<EesrSpec> {
        RfofW::new(self, 16)
    }
    ///Bit 17 - Receive Descriptor Empty Flag
    #[inline(always)]
    pub fn rde(&mut self) -> RdeW<EesrSpec> {
        RdeW::new(self, 17)
    }
    ///Bit 18 - Frame Receive Flag
    #[inline(always)]
    pub fn fr(&mut self) -> FrW<EesrSpec> {
        FrW::new(self, 18)
    }
    ///Bit 19 - Transmit FIFO Underflow Flag
    #[inline(always)]
    pub fn tfuf(&mut self) -> TfufW<EesrSpec> {
        TfufW::new(self, 19)
    }
    ///Bit 20 - Transmit Descriptor Empty Flag
    #[inline(always)]
    pub fn tde(&mut self) -> TdeW<EesrSpec> {
        TdeW::new(self, 20)
    }
    ///Bit 21 - Frame Transfer Complete Flag
    #[inline(always)]
    pub fn tc(&mut self) -> TcW<EesrSpec> {
        TcW::new(self, 21)
    }
    ///Bit 23 - Address Error Flag
    #[inline(always)]
    pub fn ade(&mut self) -> AdeW<EesrSpec> {
        AdeW::new(self, 23)
    }
    ///Bit 24 - Receive Frame Counter Overflow Flag
    #[inline(always)]
    pub fn rfcof(&mut self) -> RfcofW<EesrSpec> {
        RfcofW::new(self, 24)
    }
    ///Bit 25 - Receive Abort Detect Flag
    #[inline(always)]
    pub fn rabt(&mut self) -> RabtW<EesrSpec> {
        RabtW::new(self, 25)
    }
    ///Bit 26 - Transmit Abort Detect Flag
    #[inline(always)]
    pub fn tabt(&mut self) -> TabtW<EesrSpec> {
        TabtW::new(self, 26)
    }
    ///Bit 30 - Write-Back Complete Flag
    #[inline(always)]
    pub fn twb(&mut self) -> TwbW<EesrSpec> {
        TwbW::new(self, 30)
    }
}
/**ETHERC/EDMAC Status Register

You can [`read`](crate::Reg::read) this register and get [`eesr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eesr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct EesrSpec;
impl crate::RegisterSpec for EesrSpec {
    type Ux = u32;
}
///`read()` method returns [`eesr::R`](R) reader structure
impl crate::Readable for EesrSpec {}
///`write(|w| ..)` method takes [`eesr::W`](W) writer structure
impl crate::Writable for EesrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EESR to value 0
impl crate::Resettable for EesrSpec {}
