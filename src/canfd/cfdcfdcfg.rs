///Register `CFDC%sFDCFG` reader
pub type R = crate::R<CfdcfdcfgSpec>;
///Register `CFDC%sFDCFG` writer
pub type W = crate::W<CfdcfdcfgSpec>;
/**Error Occurrence Counter Configuration

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Eoccfg {
    ///0: All transmitter or receiver CAN frames
    _000 = 0,
    ///1: All transmitter CAN frames
    _001 = 1,
    ///2: All receiver CAN frames
    _010 = 2,
    ///3: Reserved
    _011 = 3,
    ///4: Only transmitter or receiver CAN-FD data-phase (fast bits)
    _100 = 4,
    ///5: Only transmitter CAN-FD data-phase (fast bits)
    _101 = 5,
    ///6: Only receiver CAN-FD data-phase (fast bits)
    _110 = 6,
    ///7: Reserved
    _111 = 7,
}
impl From<Eoccfg> for u8 {
    #[inline(always)]
    fn from(variant: Eoccfg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Eoccfg {
    type Ux = u8;
}
impl crate::IsEnum for Eoccfg {}
///Field `EOCCFG` reader - Error Occurrence Counter Configuration
pub type EoccfgR = crate::FieldReader<Eoccfg>;
impl EoccfgR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eoccfg {
        match self.bits {
            0 => Eoccfg::_000,
            1 => Eoccfg::_001,
            2 => Eoccfg::_010,
            3 => Eoccfg::_011,
            4 => Eoccfg::_100,
            5 => Eoccfg::_101,
            6 => Eoccfg::_110,
            7 => Eoccfg::_111,
            _ => unreachable!(),
        }
    }
    ///All transmitter or receiver CAN frames
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Eoccfg::_000
    }
    ///All transmitter CAN frames
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Eoccfg::_001
    }
    ///All receiver CAN frames
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Eoccfg::_010
    }
    ///Reserved
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Eoccfg::_011
    }
    ///Only transmitter or receiver CAN-FD data-phase (fast bits)
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Eoccfg::_100
    }
    ///Only transmitter CAN-FD data-phase (fast bits)
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Eoccfg::_101
    }
    ///Only receiver CAN-FD data-phase (fast bits)
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Eoccfg::_110
    }
    ///Reserved
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == Eoccfg::_111
    }
}
///Field `EOCCFG` writer - Error Occurrence Counter Configuration
pub type EoccfgW<'a, REG> = crate::FieldWriter<'a, REG, 3, Eoccfg, crate::Safe>;
impl<'a, REG> EoccfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///All transmitter or receiver CAN frames
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Eoccfg::_000)
    }
    ///All transmitter CAN frames
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Eoccfg::_001)
    }
    ///All receiver CAN frames
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Eoccfg::_010)
    }
    ///Reserved
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Eoccfg::_011)
    }
    ///Only transmitter or receiver CAN-FD data-phase (fast bits)
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Eoccfg::_100)
    }
    ///Only transmitter CAN-FD data-phase (fast bits)
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Eoccfg::_101)
    }
    ///Only receiver CAN-FD data-phase (fast bits)
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Eoccfg::_110)
    }
    ///Reserved
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(Eoccfg::_111)
    }
}
/**Transceiver Delay Compensation Offset Configuration

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tdcoc {
    ///0: Measured + offset
    _0 = 0,
    ///1: Offset-only
    _1 = 1,
}
impl From<Tdcoc> for bool {
    #[inline(always)]
    fn from(variant: Tdcoc) -> Self {
        variant as u8 != 0
    }
}
///Field `TDCOC` reader - Transceiver Delay Compensation Offset Configuration
pub type TdcocR = crate::BitReader<Tdcoc>;
impl TdcocR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tdcoc {
        match self.bits {
            false => Tdcoc::_0,
            true => Tdcoc::_1,
        }
    }
    ///Measured + offset
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tdcoc::_0
    }
    ///Offset-only
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tdcoc::_1
    }
}
///Field `TDCOC` writer - Transceiver Delay Compensation Offset Configuration
pub type TdcocW<'a, REG> = crate::BitWriter<'a, REG, Tdcoc>;
impl<'a, REG> TdcocW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Measured + offset
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tdcoc::_0)
    }
    ///Offset-only
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tdcoc::_1)
    }
}
/**Transceiver Delay Compensation Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tdce {
    ///0: Transceiver delay compensation disabled
    _0 = 0,
    ///1: Transceiver delay compensation enabled
    _1 = 1,
}
impl From<Tdce> for bool {
    #[inline(always)]
    fn from(variant: Tdce) -> Self {
        variant as u8 != 0
    }
}
///Field `TDCE` reader - Transceiver Delay Compensation Enable
pub type TdceR = crate::BitReader<Tdce>;
impl TdceR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tdce {
        match self.bits {
            false => Tdce::_0,
            true => Tdce::_1,
        }
    }
    ///Transceiver delay compensation disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tdce::_0
    }
    ///Transceiver delay compensation enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tdce::_1
    }
}
///Field `TDCE` writer - Transceiver Delay Compensation Enable
pub type TdceW<'a, REG> = crate::BitWriter<'a, REG, Tdce>;
impl<'a, REG> TdceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Transceiver delay compensation disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tdce::_0)
    }
    ///Transceiver delay compensation enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tdce::_1)
    }
}
/**Error State Indication Configuration

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Esic {
    ///0: The ESI bit in the frame represents the error state of the node itself
    _0 = 0,
    ///1: The ESI bit in the frame represents the error state of the message buffer if the node itself is not in error passive. If the node is in error passive, then the ESI bit is driven by the node itself.
    _1 = 1,
}
impl From<Esic> for bool {
    #[inline(always)]
    fn from(variant: Esic) -> Self {
        variant as u8 != 0
    }
}
///Field `ESIC` reader - Error State Indication Configuration
pub type EsicR = crate::BitReader<Esic>;
impl EsicR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Esic {
        match self.bits {
            false => Esic::_0,
            true => Esic::_1,
        }
    }
    ///The ESI bit in the frame represents the error state of the node itself
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Esic::_0
    }
    ///The ESI bit in the frame represents the error state of the message buffer if the node itself is not in error passive. If the node is in error passive, then the ESI bit is driven by the node itself.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Esic::_1
    }
}
///Field `ESIC` writer - Error State Indication Configuration
pub type EsicW<'a, REG> = crate::BitWriter<'a, REG, Esic>;
impl<'a, REG> EsicW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The ESI bit in the frame represents the error state of the node itself
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Esic::_0)
    }
    ///The ESI bit in the frame represents the error state of the message buffer if the node itself is not in error passive. If the node is in error passive, then the ESI bit is driven by the node itself.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Esic::_1)
    }
}
///Field `TDCO` reader - Transceiver Delay Compensation Offset
pub type TdcoR = crate::FieldReader;
///Field `TDCO` writer - Transceiver Delay Compensation Offset
pub type TdcoW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
/**CAN2.0, CAN-FD Multi-Gateway Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gwen {
    ///0: Multi-gateway disabled
    _0 = 0,
    ///1: Multi-gateway enable
    _1 = 1,
}
impl From<Gwen> for bool {
    #[inline(always)]
    fn from(variant: Gwen) -> Self {
        variant as u8 != 0
    }
}
///Field `GWEN` reader - CAN2.0, CAN-FD Multi-Gateway Enable
pub type GwenR = crate::BitReader<Gwen>;
impl GwenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Gwen {
        match self.bits {
            false => Gwen::_0,
            true => Gwen::_1,
        }
    }
    ///Multi-gateway disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Gwen::_0
    }
    ///Multi-gateway enable
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Gwen::_1
    }
}
///Field `GWEN` writer - CAN2.0, CAN-FD Multi-Gateway Enable
pub type GwenW<'a, REG> = crate::BitWriter<'a, REG, Gwen>;
impl<'a, REG> GwenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Multi-gateway disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Gwen::_0)
    }
    ///Multi-gateway enable
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Gwen::_1)
    }
}
/**Gateway FDF Configuration Bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gwfdf {
    ///0: Gateway frame is transmitted as Classical CAN frame
    _0 = 0,
    ///1: Gateway frame is transmitted as CAN-FD frame
    _1 = 1,
}
impl From<Gwfdf> for bool {
    #[inline(always)]
    fn from(variant: Gwfdf) -> Self {
        variant as u8 != 0
    }
}
///Field `GWFDF` reader - Gateway FDF Configuration Bit
pub type GwfdfR = crate::BitReader<Gwfdf>;
impl GwfdfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Gwfdf {
        match self.bits {
            false => Gwfdf::_0,
            true => Gwfdf::_1,
        }
    }
    ///Gateway frame is transmitted as Classical CAN frame
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Gwfdf::_0
    }
    ///Gateway frame is transmitted as CAN-FD frame
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Gwfdf::_1
    }
}
///Field `GWFDF` writer - Gateway FDF Configuration Bit
pub type GwfdfW<'a, REG> = crate::BitWriter<'a, REG, Gwfdf>;
impl<'a, REG> GwfdfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Gateway frame is transmitted as Classical CAN frame
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Gwfdf::_0)
    }
    ///Gateway frame is transmitted as CAN-FD frame
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Gwfdf::_1)
    }
}
/**Gateway BRS Configuration Bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gwbrs {
    ///0: Gateway frame is transmitted with BRS = 0
    _0 = 0,
    ///1: Gateway frame is transmitted with BRS = 1
    _1 = 1,
}
impl From<Gwbrs> for bool {
    #[inline(always)]
    fn from(variant: Gwbrs) -> Self {
        variant as u8 != 0
    }
}
///Field `GWBRS` reader - Gateway BRS Configuration Bit
pub type GwbrsR = crate::BitReader<Gwbrs>;
impl GwbrsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Gwbrs {
        match self.bits {
            false => Gwbrs::_0,
            true => Gwbrs::_1,
        }
    }
    ///Gateway frame is transmitted with BRS = 0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Gwbrs::_0
    }
    ///Gateway frame is transmitted with BRS = 1
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Gwbrs::_1
    }
}
///Field `GWBRS` writer - Gateway BRS Configuration Bit
pub type GwbrsW<'a, REG> = crate::BitWriter<'a, REG, Gwbrs>;
impl<'a, REG> GwbrsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Gateway frame is transmitted with BRS = 0
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Gwbrs::_0)
    }
    ///Gateway frame is transmitted with BRS = 1
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Gwbrs::_1)
    }
}
/**FD-Only Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fdoe {
    ///0: FD-only mode disabled
    _0 = 0,
    ///1: FD-only mode enabled
    _1 = 1,
}
impl From<Fdoe> for bool {
    #[inline(always)]
    fn from(variant: Fdoe) -> Self {
        variant as u8 != 0
    }
}
///Field `FDOE` reader - FD-Only Enable
pub type FdoeR = crate::BitReader<Fdoe>;
impl FdoeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Fdoe {
        match self.bits {
            false => Fdoe::_0,
            true => Fdoe::_1,
        }
    }
    ///FD-only mode disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Fdoe::_0
    }
    ///FD-only mode enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Fdoe::_1
    }
}
///Field `FDOE` writer - FD-Only Enable
pub type FdoeW<'a, REG> = crate::BitWriter<'a, REG, Fdoe>;
impl<'a, REG> FdoeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///FD-only mode disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Fdoe::_0)
    }
    ///FD-only mode enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Fdoe::_1)
    }
}
/**RX Edge Filter Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refe {
    ///0: RX edge filter disabled
    _0 = 0,
    ///1: RX edge filter enabled
    _1 = 1,
}
impl From<Refe> for bool {
    #[inline(always)]
    fn from(variant: Refe) -> Self {
        variant as u8 != 0
    }
}
///Field `REFE` reader - RX Edge Filter Enable
pub type RefeR = crate::BitReader<Refe>;
impl RefeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Refe {
        match self.bits {
            false => Refe::_0,
            true => Refe::_1,
        }
    }
    ///RX edge filter disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Refe::_0
    }
    ///RX edge filter enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Refe::_1
    }
}
///Field `REFE` writer - RX Edge Filter Enable
pub type RefeW<'a, REG> = crate::BitWriter<'a, REG, Refe>;
impl<'a, REG> RefeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///RX edge filter disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Refe::_0)
    }
    ///RX edge filter enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Refe::_1)
    }
}
/**Classical CAN Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cloe {
    ///0: Classical CAN mode disabled
    _0 = 0,
    ///1: Classical CAN mode enabled
    _1 = 1,
}
impl From<Cloe> for bool {
    #[inline(always)]
    fn from(variant: Cloe) -> Self {
        variant as u8 != 0
    }
}
///Field `CLOE` reader - Classical CAN Enable
pub type CloeR = crate::BitReader<Cloe>;
impl CloeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cloe {
        match self.bits {
            false => Cloe::_0,
            true => Cloe::_1,
        }
    }
    ///Classical CAN mode disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cloe::_0
    }
    ///Classical CAN mode enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cloe::_1
    }
}
///Field `CLOE` writer - Classical CAN Enable
pub type CloeW<'a, REG> = crate::BitWriter<'a, REG, Cloe>;
impl<'a, REG> CloeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Classical CAN mode disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cloe::_0)
    }
    ///Classical CAN mode enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cloe::_1)
    }
}
/**CAN-FD Frame Distinction Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cfdte {
    ///0: CAN-FD frame distinction disabled
    _0 = 0,
    ///1: CAN-FD frame distinction enabled
    _1 = 1,
}
impl From<Cfdte> for bool {
    #[inline(always)]
    fn from(variant: Cfdte) -> Self {
        variant as u8 != 0
    }
}
///Field `CFDTE` reader - CAN-FD Frame Distinction Enable
pub type CfdteR = crate::BitReader<Cfdte>;
impl CfdteR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cfdte {
        match self.bits {
            false => Cfdte::_0,
            true => Cfdte::_1,
        }
    }
    ///CAN-FD frame distinction disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cfdte::_0
    }
    ///CAN-FD frame distinction enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cfdte::_1
    }
}
///Field `CFDTE` writer - CAN-FD Frame Distinction Enable
pub type CfdteW<'a, REG> = crate::BitWriter<'a, REG, Cfdte>;
impl<'a, REG> CfdteW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CAN-FD frame distinction disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cfdte::_0)
    }
    ///CAN-FD frame distinction enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cfdte::_1)
    }
}
impl R {
    ///Bits 0:2 - Error Occurrence Counter Configuration
    #[inline(always)]
    pub fn eoccfg(&self) -> EoccfgR {
        EoccfgR::new((self.bits & 7) as u8)
    }
    ///Bit 8 - Transceiver Delay Compensation Offset Configuration
    #[inline(always)]
    pub fn tdcoc(&self) -> TdcocR {
        TdcocR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Transceiver Delay Compensation Enable
    #[inline(always)]
    pub fn tdce(&self) -> TdceR {
        TdceR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Error State Indication Configuration
    #[inline(always)]
    pub fn esic(&self) -> EsicR {
        EsicR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bits 16:23 - Transceiver Delay Compensation Offset
    #[inline(always)]
    pub fn tdco(&self) -> TdcoR {
        TdcoR::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bit 24 - CAN2.0, CAN-FD Multi-Gateway Enable
    #[inline(always)]
    pub fn gwen(&self) -> GwenR {
        GwenR::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Gateway FDF Configuration Bit
    #[inline(always)]
    pub fn gwfdf(&self) -> GwfdfR {
        GwfdfR::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Gateway BRS Configuration Bit
    #[inline(always)]
    pub fn gwbrs(&self) -> GwbrsR {
        GwbrsR::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 28 - FD-Only Enable
    #[inline(always)]
    pub fn fdoe(&self) -> FdoeR {
        FdoeR::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - RX Edge Filter Enable
    #[inline(always)]
    pub fn refe(&self) -> RefeR {
        RefeR::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Classical CAN Enable
    #[inline(always)]
    pub fn cloe(&self) -> CloeR {
        CloeR::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - CAN-FD Frame Distinction Enable
    #[inline(always)]
    pub fn cfdte(&self) -> CfdteR {
        CfdteR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDCFDCFG")
            .field("eoccfg", &self.eoccfg())
            .field("tdcoc", &self.tdcoc())
            .field("tdce", &self.tdce())
            .field("esic", &self.esic())
            .field("tdco", &self.tdco())
            .field("gwen", &self.gwen())
            .field("gwfdf", &self.gwfdf())
            .field("gwbrs", &self.gwbrs())
            .field("fdoe", &self.fdoe())
            .field("refe", &self.refe())
            .field("cloe", &self.cloe())
            .field("cfdte", &self.cfdte())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Error Occurrence Counter Configuration
    #[inline(always)]
    pub fn eoccfg(&mut self) -> EoccfgW<CfdcfdcfgSpec> {
        EoccfgW::new(self, 0)
    }
    ///Bit 8 - Transceiver Delay Compensation Offset Configuration
    #[inline(always)]
    pub fn tdcoc(&mut self) -> TdcocW<CfdcfdcfgSpec> {
        TdcocW::new(self, 8)
    }
    ///Bit 9 - Transceiver Delay Compensation Enable
    #[inline(always)]
    pub fn tdce(&mut self) -> TdceW<CfdcfdcfgSpec> {
        TdceW::new(self, 9)
    }
    ///Bit 10 - Error State Indication Configuration
    #[inline(always)]
    pub fn esic(&mut self) -> EsicW<CfdcfdcfgSpec> {
        EsicW::new(self, 10)
    }
    ///Bits 16:23 - Transceiver Delay Compensation Offset
    #[inline(always)]
    pub fn tdco(&mut self) -> TdcoW<CfdcfdcfgSpec> {
        TdcoW::new(self, 16)
    }
    ///Bit 24 - CAN2.0, CAN-FD Multi-Gateway Enable
    #[inline(always)]
    pub fn gwen(&mut self) -> GwenW<CfdcfdcfgSpec> {
        GwenW::new(self, 24)
    }
    ///Bit 25 - Gateway FDF Configuration Bit
    #[inline(always)]
    pub fn gwfdf(&mut self) -> GwfdfW<CfdcfdcfgSpec> {
        GwfdfW::new(self, 25)
    }
    ///Bit 26 - Gateway BRS Configuration Bit
    #[inline(always)]
    pub fn gwbrs(&mut self) -> GwbrsW<CfdcfdcfgSpec> {
        GwbrsW::new(self, 26)
    }
    ///Bit 28 - FD-Only Enable
    #[inline(always)]
    pub fn fdoe(&mut self) -> FdoeW<CfdcfdcfgSpec> {
        FdoeW::new(self, 28)
    }
    ///Bit 29 - RX Edge Filter Enable
    #[inline(always)]
    pub fn refe(&mut self) -> RefeW<CfdcfdcfgSpec> {
        RefeW::new(self, 29)
    }
    ///Bit 30 - Classical CAN Enable
    #[inline(always)]
    pub fn cloe(&mut self) -> CloeW<CfdcfdcfgSpec> {
        CloeW::new(self, 30)
    }
    ///Bit 31 - CAN-FD Frame Distinction Enable
    #[inline(always)]
    pub fn cfdte(&mut self) -> CfdteW<CfdcfdcfgSpec> {
        CfdteW::new(self, 31)
    }
}
/**Channel %s CAN-FD Configuration Register

You can [`read`](crate::Reg::read) this register and get [`cfdcfdcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcfdcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdcfdcfgSpec;
impl crate::RegisterSpec for CfdcfdcfgSpec {
    type Ux = u32;
}
///`read()` method returns [`cfdcfdcfg::R`](R) reader structure
impl crate::Readable for CfdcfdcfgSpec {}
///`write(|w| ..)` method takes [`cfdcfdcfg::W`](W) writer structure
impl crate::Writable for CfdcfdcfgSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDC%sFDCFG to value 0
impl crate::Resettable for CfdcfdcfgSpec {}
