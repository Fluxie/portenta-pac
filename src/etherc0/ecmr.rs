///Register `ECMR` reader
pub type R = crate::R<EcmrSpec>;
///Register `ECMR` writer
pub type W = crate::W<EcmrSpec>;
/**Promiscuous Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prm {
    ///0: Disable promiscuous mode
    _0 = 0,
    ///1: Enable promiscuous mode.
    _1 = 1,
}
impl From<Prm> for bool {
    #[inline(always)]
    fn from(variant: Prm) -> Self {
        variant as u8 != 0
    }
}
///Field `PRM` reader - Promiscuous Mode
pub type PrmR = crate::BitReader<Prm>;
impl PrmR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Prm {
        match self.bits {
            false => Prm::_0,
            true => Prm::_1,
        }
    }
    ///Disable promiscuous mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Prm::_0
    }
    ///Enable promiscuous mode.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Prm::_1
    }
}
///Field `PRM` writer - Promiscuous Mode
pub type PrmW<'a, REG> = crate::BitWriter<'a, REG, Prm>;
impl<'a, REG> PrmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable promiscuous mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Prm::_0)
    }
    ///Enable promiscuous mode.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Prm::_1)
    }
}
/**Duplex Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dm {
    ///0: Half-duplex mode
    _0 = 0,
    ///1: Full-duplex mode.
    _1 = 1,
}
impl From<Dm> for bool {
    #[inline(always)]
    fn from(variant: Dm) -> Self {
        variant as u8 != 0
    }
}
///Field `DM` reader - Duplex Mode
pub type DmR = crate::BitReader<Dm>;
impl DmR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dm {
        match self.bits {
            false => Dm::_0,
            true => Dm::_1,
        }
    }
    ///Half-duplex mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dm::_0
    }
    ///Full-duplex mode.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dm::_1
    }
}
///Field `DM` writer - Duplex Mode
pub type DmW<'a, REG> = crate::BitWriter<'a, REG, Dm>;
impl<'a, REG> DmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Half-duplex mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dm::_0)
    }
    ///Full-duplex mode.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dm::_1)
    }
}
/**Bit Rate

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtm {
    ///0: 10 Mbps
    _0 = 0,
    ///1: 100 Mbps.
    _1 = 1,
}
impl From<Rtm> for bool {
    #[inline(always)]
    fn from(variant: Rtm) -> Self {
        variant as u8 != 0
    }
}
///Field `RTM` reader - Bit Rate
pub type RtmR = crate::BitReader<Rtm>;
impl RtmR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rtm {
        match self.bits {
            false => Rtm::_0,
            true => Rtm::_1,
        }
    }
    ///10 Mbps
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rtm::_0
    }
    ///100 Mbps.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rtm::_1
    }
}
///Field `RTM` writer - Bit Rate
pub type RtmW<'a, REG> = crate::BitWriter<'a, REG, Rtm>;
impl<'a, REG> RtmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///10 Mbps
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rtm::_0)
    }
    ///100 Mbps.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtm::_1)
    }
}
/**Internal Loopback Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ilb {
    ///0: Perform normal data transmission or reception
    _0 = 0,
    ///1: Loop data back in the ETHERC when full-duplex mode is selected.
    _1 = 1,
}
impl From<Ilb> for bool {
    #[inline(always)]
    fn from(variant: Ilb) -> Self {
        variant as u8 != 0
    }
}
///Field `ILB` reader - Internal Loopback Mode
pub type IlbR = crate::BitReader<Ilb>;
impl IlbR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ilb {
        match self.bits {
            false => Ilb::_0,
            true => Ilb::_1,
        }
    }
    ///Perform normal data transmission or reception
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ilb::_0
    }
    ///Loop data back in the ETHERC when full-duplex mode is selected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ilb::_1
    }
}
///Field `ILB` writer - Internal Loopback Mode
pub type IlbW<'a, REG> = crate::BitWriter<'a, REG, Ilb>;
impl<'a, REG> IlbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Perform normal data transmission or reception
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ilb::_0)
    }
    ///Loop data back in the ETHERC when full-duplex mode is selected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ilb::_1)
    }
}
/**Transmission Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te {
    ///0: Disable transmit function
    _0 = 0,
    ///1: Enable transmit function.
    _1 = 1,
}
impl From<Te> for bool {
    #[inline(always)]
    fn from(variant: Te) -> Self {
        variant as u8 != 0
    }
}
///Field `TE` reader - Transmission Enable
pub type TeR = crate::BitReader<Te>;
impl TeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Te {
        match self.bits {
            false => Te::_0,
            true => Te::_1,
        }
    }
    ///Disable transmit function
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Te::_0
    }
    ///Enable transmit function.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Te::_1
    }
}
///Field `TE` writer - Transmission Enable
pub type TeW<'a, REG> = crate::BitWriter<'a, REG, Te>;
impl<'a, REG> TeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable transmit function
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Te::_0)
    }
    ///Enable transmit function.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Te::_1)
    }
}
/**Reception Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Re {
    ///0: Disable receive function
    _0 = 0,
    ///1: Enable receive function.
    _1 = 1,
}
impl From<Re> for bool {
    #[inline(always)]
    fn from(variant: Re) -> Self {
        variant as u8 != 0
    }
}
///Field `RE` reader - Reception Enable
pub type ReR = crate::BitReader<Re>;
impl ReR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Re {
        match self.bits {
            false => Re::_0,
            true => Re::_1,
        }
    }
    ///Disable receive function
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Re::_0
    }
    ///Enable receive function.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Re::_1
    }
}
///Field `RE` writer - Reception Enable
pub type ReW<'a, REG> = crate::BitWriter<'a, REG, Re>;
impl<'a, REG> ReW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable receive function
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Re::_0)
    }
    ///Enable receive function.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Re::_1)
    }
}
/**Magic Packet Detection Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mpde {
    ///0: Disable Magic Packet detection
    _0 = 0,
    ///1: Enable Magic Packet detection.
    _1 = 1,
}
impl From<Mpde> for bool {
    #[inline(always)]
    fn from(variant: Mpde) -> Self {
        variant as u8 != 0
    }
}
///Field `MPDE` reader - Magic Packet Detection Enable
pub type MpdeR = crate::BitReader<Mpde>;
impl MpdeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mpde {
        match self.bits {
            false => Mpde::_0,
            true => Mpde::_1,
        }
    }
    ///Disable Magic Packet detection
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mpde::_0
    }
    ///Enable Magic Packet detection.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mpde::_1
    }
}
///Field `MPDE` writer - Magic Packet Detection Enable
pub type MpdeW<'a, REG> = crate::BitWriter<'a, REG, Mpde>;
impl<'a, REG> MpdeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable Magic Packet detection
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mpde::_0)
    }
    ///Enable Magic Packet detection.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mpde::_1)
    }
}
/**CRC Error Frame Receive Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prcef {
    ///0: Notify EDMAC of a CRC error
    _0 = 0,
    ///1: Do not notify EDMAC of a CRC error.
    _1 = 1,
}
impl From<Prcef> for bool {
    #[inline(always)]
    fn from(variant: Prcef) -> Self {
        variant as u8 != 0
    }
}
///Field `PRCEF` reader - CRC Error Frame Receive Mode
pub type PrcefR = crate::BitReader<Prcef>;
impl PrcefR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Prcef {
        match self.bits {
            false => Prcef::_0,
            true => Prcef::_1,
        }
    }
    ///Notify EDMAC of a CRC error
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Prcef::_0
    }
    ///Do not notify EDMAC of a CRC error.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Prcef::_1
    }
}
///Field `PRCEF` writer - CRC Error Frame Receive Mode
pub type PrcefW<'a, REG> = crate::BitWriter<'a, REG, Prcef>;
impl<'a, REG> PrcefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Notify EDMAC of a CRC error
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Prcef::_0)
    }
    ///Do not notify EDMAC of a CRC error.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Prcef::_1)
    }
}
/**Transmit Flow Control Operating Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txf {
    ///0: Disable automatic PAUSE frame transmission (PAUSE frame is not automatically transmitted)
    _0 = 0,
    ///1: Enable automatic PAUSE frame transmission (PAUSE frame is automatically transmitted as required).
    _1 = 1,
}
impl From<Txf> for bool {
    #[inline(always)]
    fn from(variant: Txf) -> Self {
        variant as u8 != 0
    }
}
///Field `TXF` reader - Transmit Flow Control Operating Mode
pub type TxfR = crate::BitReader<Txf>;
impl TxfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Txf {
        match self.bits {
            false => Txf::_0,
            true => Txf::_1,
        }
    }
    ///Disable automatic PAUSE frame transmission (PAUSE frame is not automatically transmitted)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Txf::_0
    }
    ///Enable automatic PAUSE frame transmission (PAUSE frame is automatically transmitted as required).
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Txf::_1
    }
}
///Field `TXF` writer - Transmit Flow Control Operating Mode
pub type TxfW<'a, REG> = crate::BitWriter<'a, REG, Txf>;
impl<'a, REG> TxfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable automatic PAUSE frame transmission (PAUSE frame is not automatically transmitted)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Txf::_0)
    }
    ///Enable automatic PAUSE frame transmission (PAUSE frame is automatically transmitted as required).
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Txf::_1)
    }
}
/**Receive Flow Control Operating Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxf {
    ///0: Disable PAUSE frame detection
    _0 = 0,
    ///1: Enable PAUSE frame detection.
    _1 = 1,
}
impl From<Rxf> for bool {
    #[inline(always)]
    fn from(variant: Rxf) -> Self {
        variant as u8 != 0
    }
}
///Field `RXF` reader - Receive Flow Control Operating Mode
pub type RxfR = crate::BitReader<Rxf>;
impl RxfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rxf {
        match self.bits {
            false => Rxf::_0,
            true => Rxf::_1,
        }
    }
    ///Disable PAUSE frame detection
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rxf::_0
    }
    ///Enable PAUSE frame detection.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rxf::_1
    }
}
///Field `RXF` writer - Receive Flow Control Operating Mode
pub type RxfW<'a, REG> = crate::BitWriter<'a, REG, Rxf>;
impl<'a, REG> RxfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable PAUSE frame detection
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rxf::_0)
    }
    ///Enable PAUSE frame detection.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxf::_1)
    }
}
/**PAUSE Frame Receive Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pfr {
    ///0: Do not transfer PAUSE frame to the EDMAC
    _0 = 0,
    ///1: Transfer PAUSE frame to the EDMAC.
    _1 = 1,
}
impl From<Pfr> for bool {
    #[inline(always)]
    fn from(variant: Pfr) -> Self {
        variant as u8 != 0
    }
}
///Field `PFR` reader - PAUSE Frame Receive Mode
pub type PfrR = crate::BitReader<Pfr>;
impl PfrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pfr {
        match self.bits {
            false => Pfr::_0,
            true => Pfr::_1,
        }
    }
    ///Do not transfer PAUSE frame to the EDMAC
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pfr::_0
    }
    ///Transfer PAUSE frame to the EDMAC.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pfr::_1
    }
}
///Field `PFR` writer - PAUSE Frame Receive Mode
pub type PfrW<'a, REG> = crate::BitWriter<'a, REG, Pfr>;
impl<'a, REG> PfrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not transfer PAUSE frame to the EDMAC
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pfr::_0)
    }
    ///Transfer PAUSE frame to the EDMAC.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pfr::_1)
    }
}
/**0 Time PAUSE Frame Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Zpf {
    ///0: Do not use PAUSE frames that containing a pause_time parameter of 0
    _0 = 0,
    ///1: Use PAUSE frames that containing a pause_time parameter of 0.
    _1 = 1,
}
impl From<Zpf> for bool {
    #[inline(always)]
    fn from(variant: Zpf) -> Self {
        variant as u8 != 0
    }
}
///Field `ZPF` reader - 0 Time PAUSE Frame Enable
pub type ZpfR = crate::BitReader<Zpf>;
impl ZpfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Zpf {
        match self.bits {
            false => Zpf::_0,
            true => Zpf::_1,
        }
    }
    ///Do not use PAUSE frames that containing a pause_time parameter of 0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Zpf::_0
    }
    ///Use PAUSE frames that containing a pause_time parameter of 0.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Zpf::_1
    }
}
///Field `ZPF` writer - 0 Time PAUSE Frame Enable
pub type ZpfW<'a, REG> = crate::BitWriter<'a, REG, Zpf>;
impl<'a, REG> ZpfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not use PAUSE frames that containing a pause_time parameter of 0
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Zpf::_0)
    }
    ///Use PAUSE frames that containing a pause_time parameter of 0.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Zpf::_1)
    }
}
/**PAUSE Frame Transmit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tpc {
    ///0: Transmit PAUSE frame even during a PAUSE period
    _0 = 0,
    ///1: Do not transmit PAUSE frame during a PAUSE period.
    _1 = 1,
}
impl From<Tpc> for bool {
    #[inline(always)]
    fn from(variant: Tpc) -> Self {
        variant as u8 != 0
    }
}
///Field `TPC` reader - PAUSE Frame Transmit
pub type TpcR = crate::BitReader<Tpc>;
impl TpcR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tpc {
        match self.bits {
            false => Tpc::_0,
            true => Tpc::_1,
        }
    }
    ///Transmit PAUSE frame even during a PAUSE period
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tpc::_0
    }
    ///Do not transmit PAUSE frame during a PAUSE period.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tpc::_1
    }
}
///Field `TPC` writer - PAUSE Frame Transmit
pub type TpcW<'a, REG> = crate::BitWriter<'a, REG, Tpc>;
impl<'a, REG> TpcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Transmit PAUSE frame even during a PAUSE period
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tpc::_0)
    }
    ///Do not transmit PAUSE frame during a PAUSE period.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tpc::_1)
    }
}
impl R {
    ///Bit 0 - Promiscuous Mode
    #[inline(always)]
    pub fn prm(&self) -> PrmR {
        PrmR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Duplex Mode
    #[inline(always)]
    pub fn dm(&self) -> DmR {
        DmR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Bit Rate
    #[inline(always)]
    pub fn rtm(&self) -> RtmR {
        RtmR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Internal Loopback Mode
    #[inline(always)]
    pub fn ilb(&self) -> IlbR {
        IlbR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - Transmission Enable
    #[inline(always)]
    pub fn te(&self) -> TeR {
        TeR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Reception Enable
    #[inline(always)]
    pub fn re(&self) -> ReR {
        ReR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 9 - Magic Packet Detection Enable
    #[inline(always)]
    pub fn mpde(&self) -> MpdeR {
        MpdeR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 12 - CRC Error Frame Receive Mode
    #[inline(always)]
    pub fn prcef(&self) -> PrcefR {
        PrcefR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 16 - Transmit Flow Control Operating Mode
    #[inline(always)]
    pub fn txf(&self) -> TxfR {
        TxfR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Receive Flow Control Operating Mode
    #[inline(always)]
    pub fn rxf(&self) -> RxfR {
        RxfR::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - PAUSE Frame Receive Mode
    #[inline(always)]
    pub fn pfr(&self) -> PfrR {
        PfrR::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - 0 Time PAUSE Frame Enable
    #[inline(always)]
    pub fn zpf(&self) -> ZpfR {
        ZpfR::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - PAUSE Frame Transmit
    #[inline(always)]
    pub fn tpc(&self) -> TpcR {
        TpcR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECMR")
            .field("prm", &self.prm())
            .field("dm", &self.dm())
            .field("rtm", &self.rtm())
            .field("ilb", &self.ilb())
            .field("te", &self.te())
            .field("re", &self.re())
            .field("mpde", &self.mpde())
            .field("prcef", &self.prcef())
            .field("txf", &self.txf())
            .field("rxf", &self.rxf())
            .field("pfr", &self.pfr())
            .field("zpf", &self.zpf())
            .field("tpc", &self.tpc())
            .finish()
    }
}
impl W {
    ///Bit 0 - Promiscuous Mode
    #[inline(always)]
    pub fn prm(&mut self) -> PrmW<EcmrSpec> {
        PrmW::new(self, 0)
    }
    ///Bit 1 - Duplex Mode
    #[inline(always)]
    pub fn dm(&mut self) -> DmW<EcmrSpec> {
        DmW::new(self, 1)
    }
    ///Bit 2 - Bit Rate
    #[inline(always)]
    pub fn rtm(&mut self) -> RtmW<EcmrSpec> {
        RtmW::new(self, 2)
    }
    ///Bit 3 - Internal Loopback Mode
    #[inline(always)]
    pub fn ilb(&mut self) -> IlbW<EcmrSpec> {
        IlbW::new(self, 3)
    }
    ///Bit 5 - Transmission Enable
    #[inline(always)]
    pub fn te(&mut self) -> TeW<EcmrSpec> {
        TeW::new(self, 5)
    }
    ///Bit 6 - Reception Enable
    #[inline(always)]
    pub fn re(&mut self) -> ReW<EcmrSpec> {
        ReW::new(self, 6)
    }
    ///Bit 9 - Magic Packet Detection Enable
    #[inline(always)]
    pub fn mpde(&mut self) -> MpdeW<EcmrSpec> {
        MpdeW::new(self, 9)
    }
    ///Bit 12 - CRC Error Frame Receive Mode
    #[inline(always)]
    pub fn prcef(&mut self) -> PrcefW<EcmrSpec> {
        PrcefW::new(self, 12)
    }
    ///Bit 16 - Transmit Flow Control Operating Mode
    #[inline(always)]
    pub fn txf(&mut self) -> TxfW<EcmrSpec> {
        TxfW::new(self, 16)
    }
    ///Bit 17 - Receive Flow Control Operating Mode
    #[inline(always)]
    pub fn rxf(&mut self) -> RxfW<EcmrSpec> {
        RxfW::new(self, 17)
    }
    ///Bit 18 - PAUSE Frame Receive Mode
    #[inline(always)]
    pub fn pfr(&mut self) -> PfrW<EcmrSpec> {
        PfrW::new(self, 18)
    }
    ///Bit 19 - 0 Time PAUSE Frame Enable
    #[inline(always)]
    pub fn zpf(&mut self) -> ZpfW<EcmrSpec> {
        ZpfW::new(self, 19)
    }
    ///Bit 20 - PAUSE Frame Transmit
    #[inline(always)]
    pub fn tpc(&mut self) -> TpcW<EcmrSpec> {
        TpcW::new(self, 20)
    }
}
/**ETHERC Mode Register

You can [`read`](crate::Reg::read) this register and get [`ecmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct EcmrSpec;
impl crate::RegisterSpec for EcmrSpec {
    type Ux = u32;
}
///`read()` method returns [`ecmr::R`](R) reader structure
impl crate::Readable for EcmrSpec {}
///`write(|w| ..)` method takes [`ecmr::W`](W) writer structure
impl crate::Writable for EcmrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ECMR to value 0
impl crate::Resettable for EcmrSpec {}
