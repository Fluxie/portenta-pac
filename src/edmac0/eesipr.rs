///Register `EESIPR` reader
pub type R = crate::R<EesiprSpec>;
///Register `EESIPR` writer
pub type W = crate::W<EesiprSpec>;
/**CRC Error Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cerfip {
    ///0: Disable CRC error interrupt requests
    _0 = 0,
    ///1: Enable CRC error interrupt requests.
    _1 = 1,
}
impl From<Cerfip> for bool {
    #[inline(always)]
    fn from(variant: Cerfip) -> Self {
        variant as u8 != 0
    }
}
///Field `CERFIP` reader - CRC Error Interrupt Request Enable
pub type CerfipR = crate::BitReader<Cerfip>;
impl CerfipR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cerfip {
        match self.bits {
            false => Cerfip::_0,
            true => Cerfip::_1,
        }
    }
    ///Disable CRC error interrupt requests
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cerfip::_0
    }
    ///Enable CRC error interrupt requests.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cerfip::_1
    }
}
///Field `CERFIP` writer - CRC Error Interrupt Request Enable
pub type CerfipW<'a, REG> = crate::BitWriter<'a, REG, Cerfip>;
impl<'a, REG> CerfipW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable CRC error interrupt requests
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cerfip::_0)
    }
    ///Enable CRC error interrupt requests.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cerfip::_1)
    }
}
/**PHY-LSI Receive Error Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Preip {
    ///0: Disable PHY-LSI receive error interrupt requests
    _0 = 0,
    ///1: Enable PHY-LSI receive error interrupt requests.
    _1 = 1,
}
impl From<Preip> for bool {
    #[inline(always)]
    fn from(variant: Preip) -> Self {
        variant as u8 != 0
    }
}
///Field `PREIP` reader - PHY-LSI Receive Error Interrupt Request Enable
pub type PreipR = crate::BitReader<Preip>;
impl PreipR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Preip {
        match self.bits {
            false => Preip::_0,
            true => Preip::_1,
        }
    }
    ///Disable PHY-LSI receive error interrupt requests
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Preip::_0
    }
    ///Enable PHY-LSI receive error interrupt requests.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Preip::_1
    }
}
///Field `PREIP` writer - PHY-LSI Receive Error Interrupt Request Enable
pub type PreipW<'a, REG> = crate::BitWriter<'a, REG, Preip>;
impl<'a, REG> PreipW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable PHY-LSI receive error interrupt requests
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Preip::_0)
    }
    ///Enable PHY-LSI receive error interrupt requests.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Preip::_1)
    }
}
/**Frame-Too-Short Error Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtsfip {
    ///0: Disable frame-too-short error interrupt requests
    _0 = 0,
    ///1: Enable frame-too-short error interrupt requests.
    _1 = 1,
}
impl From<Rtsfip> for bool {
    #[inline(always)]
    fn from(variant: Rtsfip) -> Self {
        variant as u8 != 0
    }
}
///Field `RTSFIP` reader - Frame-Too-Short Error Interrupt Request Enable
pub type RtsfipR = crate::BitReader<Rtsfip>;
impl RtsfipR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rtsfip {
        match self.bits {
            false => Rtsfip::_0,
            true => Rtsfip::_1,
        }
    }
    ///Disable frame-too-short error interrupt requests
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rtsfip::_0
    }
    ///Enable frame-too-short error interrupt requests.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rtsfip::_1
    }
}
///Field `RTSFIP` writer - Frame-Too-Short Error Interrupt Request Enable
pub type RtsfipW<'a, REG> = crate::BitWriter<'a, REG, Rtsfip>;
impl<'a, REG> RtsfipW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable frame-too-short error interrupt requests
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rtsfip::_0)
    }
    ///Enable frame-too-short error interrupt requests.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtsfip::_1)
    }
}
/**Frame-Too-Long Error Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtlfip {
    ///0: Disable frame-too-long error interrupt requests
    _0 = 0,
    ///1: Enable frame-too-long error interrupt requests.
    _1 = 1,
}
impl From<Rtlfip> for bool {
    #[inline(always)]
    fn from(variant: Rtlfip) -> Self {
        variant as u8 != 0
    }
}
///Field `RTLFIP` reader - Frame-Too-Long Error Interrupt Request Enable
pub type RtlfipR = crate::BitReader<Rtlfip>;
impl RtlfipR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rtlfip {
        match self.bits {
            false => Rtlfip::_0,
            true => Rtlfip::_1,
        }
    }
    ///Disable frame-too-long error interrupt requests
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rtlfip::_0
    }
    ///Enable frame-too-long error interrupt requests.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rtlfip::_1
    }
}
///Field `RTLFIP` writer - Frame-Too-Long Error Interrupt Request Enable
pub type RtlfipW<'a, REG> = crate::BitWriter<'a, REG, Rtlfip>;
impl<'a, REG> RtlfipW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable frame-too-long error interrupt requests
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rtlfip::_0)
    }
    ///Enable frame-too-long error interrupt requests.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtlfip::_1)
    }
}
/**Alignment Error Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rrfip {
    ///0: Disable alignment error interrupt requests
    _0 = 0,
    ///1: Enable alignment error interrupt requests.
    _1 = 1,
}
impl From<Rrfip> for bool {
    #[inline(always)]
    fn from(variant: Rrfip) -> Self {
        variant as u8 != 0
    }
}
///Field `RRFIP` reader - Alignment Error Interrupt Request Enable
pub type RrfipR = crate::BitReader<Rrfip>;
impl RrfipR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rrfip {
        match self.bits {
            false => Rrfip::_0,
            true => Rrfip::_1,
        }
    }
    ///Disable alignment error interrupt requests
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rrfip::_0
    }
    ///Enable alignment error interrupt requests.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rrfip::_1
    }
}
///Field `RRFIP` writer - Alignment Error Interrupt Request Enable
pub type RrfipW<'a, REG> = crate::BitWriter<'a, REG, Rrfip>;
impl<'a, REG> RrfipW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable alignment error interrupt requests
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rrfip::_0)
    }
    ///Enable alignment error interrupt requests.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rrfip::_1)
    }
}
/**Multicast Address Frame Receive Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rmafip {
    ///0: Disable multicast address frame receive interrupt requests
    _0 = 0,
    ///1: Enable multicast address frame receive interrupt requests.
    _1 = 1,
}
impl From<Rmafip> for bool {
    #[inline(always)]
    fn from(variant: Rmafip) -> Self {
        variant as u8 != 0
    }
}
///Field `RMAFIP` reader - Multicast Address Frame Receive Interrupt Request Enable
pub type RmafipR = crate::BitReader<Rmafip>;
impl RmafipR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rmafip {
        match self.bits {
            false => Rmafip::_0,
            true => Rmafip::_1,
        }
    }
    ///Disable multicast address frame receive interrupt requests
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rmafip::_0
    }
    ///Enable multicast address frame receive interrupt requests.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rmafip::_1
    }
}
///Field `RMAFIP` writer - Multicast Address Frame Receive Interrupt Request Enable
pub type RmafipW<'a, REG> = crate::BitWriter<'a, REG, Rmafip>;
impl<'a, REG> RmafipW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable multicast address frame receive interrupt requests
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rmafip::_0)
    }
    ///Enable multicast address frame receive interrupt requests.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rmafip::_1)
    }
}
/**Transmit Retry Over Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Troip {
    ///0: Disable transmit retry over interrupt requests
    _0 = 0,
    ///1: Enable transmit retry over interrupt requests.
    _1 = 1,
}
impl From<Troip> for bool {
    #[inline(always)]
    fn from(variant: Troip) -> Self {
        variant as u8 != 0
    }
}
///Field `TROIP` reader - Transmit Retry Over Interrupt Request Enable
pub type TroipR = crate::BitReader<Troip>;
impl TroipR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Troip {
        match self.bits {
            false => Troip::_0,
            true => Troip::_1,
        }
    }
    ///Disable transmit retry over interrupt requests
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Troip::_0
    }
    ///Enable transmit retry over interrupt requests.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Troip::_1
    }
}
///Field `TROIP` writer - Transmit Retry Over Interrupt Request Enable
pub type TroipW<'a, REG> = crate::BitWriter<'a, REG, Troip>;
impl<'a, REG> TroipW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable transmit retry over interrupt requests
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Troip::_0)
    }
    ///Enable transmit retry over interrupt requests.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Troip::_1)
    }
}
/**Late Collision Detect Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cdip {
    ///0: Disable late collision detected interrupt requests
    _0 = 0,
    ///1: Enable late collision detected interrupt requests.
    _1 = 1,
}
impl From<Cdip> for bool {
    #[inline(always)]
    fn from(variant: Cdip) -> Self {
        variant as u8 != 0
    }
}
///Field `CDIP` reader - Late Collision Detect Interrupt Request Enable
pub type CdipR = crate::BitReader<Cdip>;
impl CdipR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cdip {
        match self.bits {
            false => Cdip::_0,
            true => Cdip::_1,
        }
    }
    ///Disable late collision detected interrupt requests
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cdip::_0
    }
    ///Enable late collision detected interrupt requests.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cdip::_1
    }
}
///Field `CDIP` writer - Late Collision Detect Interrupt Request Enable
pub type CdipW<'a, REG> = crate::BitWriter<'a, REG, Cdip>;
impl<'a, REG> CdipW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable late collision detected interrupt requests
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cdip::_0)
    }
    ///Enable late collision detected interrupt requests.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cdip::_1)
    }
}
/**Loss of Carrier Detect Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dlcip {
    ///0: Disable loss of carrier detected interrupt requests
    _0 = 0,
    ///1: Enable loss of carrier detected interrupt requests.
    _1 = 1,
}
impl From<Dlcip> for bool {
    #[inline(always)]
    fn from(variant: Dlcip) -> Self {
        variant as u8 != 0
    }
}
///Field `DLCIP` reader - Loss of Carrier Detect Interrupt Request Enable
pub type DlcipR = crate::BitReader<Dlcip>;
impl DlcipR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dlcip {
        match self.bits {
            false => Dlcip::_0,
            true => Dlcip::_1,
        }
    }
    ///Disable loss of carrier detected interrupt requests
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dlcip::_0
    }
    ///Enable loss of carrier detected interrupt requests.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dlcip::_1
    }
}
///Field `DLCIP` writer - Loss of Carrier Detect Interrupt Request Enable
pub type DlcipW<'a, REG> = crate::BitWriter<'a, REG, Dlcip>;
impl<'a, REG> DlcipW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable loss of carrier detected interrupt requests
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dlcip::_0)
    }
    ///Enable loss of carrier detected interrupt requests.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dlcip::_1)
    }
}
/**Carrier Not Detect Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cndip {
    ///0: Disable carrier not detected interrupt requests
    _0 = 0,
    ///1: Enable carrier not detected interrupt requests.
    _1 = 1,
}
impl From<Cndip> for bool {
    #[inline(always)]
    fn from(variant: Cndip) -> Self {
        variant as u8 != 0
    }
}
///Field `CNDIP` reader - Carrier Not Detect Interrupt Request Enable
pub type CndipR = crate::BitReader<Cndip>;
impl CndipR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cndip {
        match self.bits {
            false => Cndip::_0,
            true => Cndip::_1,
        }
    }
    ///Disable carrier not detected interrupt requests
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cndip::_0
    }
    ///Enable carrier not detected interrupt requests.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cndip::_1
    }
}
///Field `CNDIP` writer - Carrier Not Detect Interrupt Request Enable
pub type CndipW<'a, REG> = crate::BitWriter<'a, REG, Cndip>;
impl<'a, REG> CndipW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable carrier not detected interrupt requests
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cndip::_0)
    }
    ///Enable carrier not detected interrupt requests.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cndip::_1)
    }
}
/**Receive FIFO Overflow Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfofip {
    ///0: Disable overflow interrupt requests
    _0 = 0,
    ///1: Enable overflow interrupt requests.
    _1 = 1,
}
impl From<Rfofip> for bool {
    #[inline(always)]
    fn from(variant: Rfofip) -> Self {
        variant as u8 != 0
    }
}
///Field `RFOFIP` reader - Receive FIFO Overflow Interrupt Request Enable
pub type RfofipR = crate::BitReader<Rfofip>;
impl RfofipR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rfofip {
        match self.bits {
            false => Rfofip::_0,
            true => Rfofip::_1,
        }
    }
    ///Disable overflow interrupt requests
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rfofip::_0
    }
    ///Enable overflow interrupt requests.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rfofip::_1
    }
}
///Field `RFOFIP` writer - Receive FIFO Overflow Interrupt Request Enable
pub type RfofipW<'a, REG> = crate::BitWriter<'a, REG, Rfofip>;
impl<'a, REG> RfofipW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable overflow interrupt requests
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rfofip::_0)
    }
    ///Enable overflow interrupt requests.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rfofip::_1)
    }
}
/**Receive Descriptor Empty Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdeip {
    ///0: Disable receive descriptor empty interrupt requests
    _0 = 0,
    ///1: Enable receive descriptor empty interrupt requests.
    _1 = 1,
}
impl From<Rdeip> for bool {
    #[inline(always)]
    fn from(variant: Rdeip) -> Self {
        variant as u8 != 0
    }
}
///Field `RDEIP` reader - Receive Descriptor Empty Interrupt Request Enable
pub type RdeipR = crate::BitReader<Rdeip>;
impl RdeipR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rdeip {
        match self.bits {
            false => Rdeip::_0,
            true => Rdeip::_1,
        }
    }
    ///Disable receive descriptor empty interrupt requests
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rdeip::_0
    }
    ///Enable receive descriptor empty interrupt requests.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rdeip::_1
    }
}
///Field `RDEIP` writer - Receive Descriptor Empty Interrupt Request Enable
pub type RdeipW<'a, REG> = crate::BitWriter<'a, REG, Rdeip>;
impl<'a, REG> RdeipW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable receive descriptor empty interrupt requests
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rdeip::_0)
    }
    ///Enable receive descriptor empty interrupt requests.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rdeip::_1)
    }
}
/**Frame Receive Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Frip {
    ///0: Disable frame reception interrupt requests
    _0 = 0,
    ///1: Enable frame reception interrupt requests.
    _1 = 1,
}
impl From<Frip> for bool {
    #[inline(always)]
    fn from(variant: Frip) -> Self {
        variant as u8 != 0
    }
}
///Field `FRIP` reader - Frame Receive Interrupt Request Enable
pub type FripR = crate::BitReader<Frip>;
impl FripR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Frip {
        match self.bits {
            false => Frip::_0,
            true => Frip::_1,
        }
    }
    ///Disable frame reception interrupt requests
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Frip::_0
    }
    ///Enable frame reception interrupt requests.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Frip::_1
    }
}
///Field `FRIP` writer - Frame Receive Interrupt Request Enable
pub type FripW<'a, REG> = crate::BitWriter<'a, REG, Frip>;
impl<'a, REG> FripW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable frame reception interrupt requests
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Frip::_0)
    }
    ///Enable frame reception interrupt requests.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Frip::_1)
    }
}
/**Transmit FIFO Underflow Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tfufip {
    ///0: Disable underflow interrupt requests
    _0 = 0,
    ///1: Enable underflow interrupt requests.
    _1 = 1,
}
impl From<Tfufip> for bool {
    #[inline(always)]
    fn from(variant: Tfufip) -> Self {
        variant as u8 != 0
    }
}
///Field `TFUFIP` reader - Transmit FIFO Underflow Interrupt Request Enable
pub type TfufipR = crate::BitReader<Tfufip>;
impl TfufipR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tfufip {
        match self.bits {
            false => Tfufip::_0,
            true => Tfufip::_1,
        }
    }
    ///Disable underflow interrupt requests
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tfufip::_0
    }
    ///Enable underflow interrupt requests.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tfufip::_1
    }
}
///Field `TFUFIP` writer - Transmit FIFO Underflow Interrupt Request Enable
pub type TfufipW<'a, REG> = crate::BitWriter<'a, REG, Tfufip>;
impl<'a, REG> TfufipW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable underflow interrupt requests
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tfufip::_0)
    }
    ///Enable underflow interrupt requests.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tfufip::_1)
    }
}
/**Transmit Descriptor Empty Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tdeip {
    ///0: Disable transmit descriptor empty interrupt requests
    _0 = 0,
    ///1: Enable transmit descriptor empty interrupt requests.
    _1 = 1,
}
impl From<Tdeip> for bool {
    #[inline(always)]
    fn from(variant: Tdeip) -> Self {
        variant as u8 != 0
    }
}
///Field `TDEIP` reader - Transmit Descriptor Empty Interrupt Request Enable
pub type TdeipR = crate::BitReader<Tdeip>;
impl TdeipR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tdeip {
        match self.bits {
            false => Tdeip::_0,
            true => Tdeip::_1,
        }
    }
    ///Disable transmit descriptor empty interrupt requests
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tdeip::_0
    }
    ///Enable transmit descriptor empty interrupt requests.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tdeip::_1
    }
}
///Field `TDEIP` writer - Transmit Descriptor Empty Interrupt Request Enable
pub type TdeipW<'a, REG> = crate::BitWriter<'a, REG, Tdeip>;
impl<'a, REG> TdeipW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable transmit descriptor empty interrupt requests
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tdeip::_0)
    }
    ///Enable transmit descriptor empty interrupt requests.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tdeip::_1)
    }
}
/**Frame Transfer Complete Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcip {
    ///0: Disable frame transmission complete interrupt requests
    _0 = 0,
    ///1: Enable frame transmission complete interrupt requests.
    _1 = 1,
}
impl From<Tcip> for bool {
    #[inline(always)]
    fn from(variant: Tcip) -> Self {
        variant as u8 != 0
    }
}
///Field `TCIP` reader - Frame Transfer Complete Interrupt Request Enable
pub type TcipR = crate::BitReader<Tcip>;
impl TcipR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tcip {
        match self.bits {
            false => Tcip::_0,
            true => Tcip::_1,
        }
    }
    ///Disable frame transmission complete interrupt requests
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tcip::_0
    }
    ///Enable frame transmission complete interrupt requests.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tcip::_1
    }
}
///Field `TCIP` writer - Frame Transfer Complete Interrupt Request Enable
pub type TcipW<'a, REG> = crate::BitWriter<'a, REG, Tcip>;
impl<'a, REG> TcipW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable frame transmission complete interrupt requests
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tcip::_0)
    }
    ///Enable frame transmission complete interrupt requests.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tcip::_1)
    }
}
/**ETHERC Status Register Source Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eciip {
    ///0: Disable ETHERC status interrupt requests
    _0 = 0,
    ///1: Enable ETHERC status interrupt requests.
    _1 = 1,
}
impl From<Eciip> for bool {
    #[inline(always)]
    fn from(variant: Eciip) -> Self {
        variant as u8 != 0
    }
}
///Field `ECIIP` reader - ETHERC Status Register Source Interrupt Request Enable
pub type EciipR = crate::BitReader<Eciip>;
impl EciipR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eciip {
        match self.bits {
            false => Eciip::_0,
            true => Eciip::_1,
        }
    }
    ///Disable ETHERC status interrupt requests
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eciip::_0
    }
    ///Enable ETHERC status interrupt requests.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eciip::_1
    }
}
///Field `ECIIP` writer - ETHERC Status Register Source Interrupt Request Enable
pub type EciipW<'a, REG> = crate::BitWriter<'a, REG, Eciip>;
impl<'a, REG> EciipW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable ETHERC status interrupt requests
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eciip::_0)
    }
    ///Enable ETHERC status interrupt requests.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eciip::_1)
    }
}
/**Address Error Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adeip {
    ///0: Disable address error interrupt requests
    _0 = 0,
    ///1: Enable address error interrupt requests.
    _1 = 1,
}
impl From<Adeip> for bool {
    #[inline(always)]
    fn from(variant: Adeip) -> Self {
        variant as u8 != 0
    }
}
///Field `ADEIP` reader - Address Error Interrupt Request Enable
pub type AdeipR = crate::BitReader<Adeip>;
impl AdeipR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Adeip {
        match self.bits {
            false => Adeip::_0,
            true => Adeip::_1,
        }
    }
    ///Disable address error interrupt requests
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Adeip::_0
    }
    ///Enable address error interrupt requests.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Adeip::_1
    }
}
///Field `ADEIP` writer - Address Error Interrupt Request Enable
pub type AdeipW<'a, REG> = crate::BitWriter<'a, REG, Adeip>;
impl<'a, REG> AdeipW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable address error interrupt requests
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Adeip::_0)
    }
    ///Enable address error interrupt requests.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Adeip::_1)
    }
}
/**Receive Frame Counter Overflow Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfcofip {
    ///0: Disable receive frame counter overflow interrupt requests
    _0 = 0,
    ///1: Enable receive frame counter overflow interrupt requests.
    _1 = 1,
}
impl From<Rfcofip> for bool {
    #[inline(always)]
    fn from(variant: Rfcofip) -> Self {
        variant as u8 != 0
    }
}
///Field `RFCOFIP` reader - Receive Frame Counter Overflow Interrupt Request Enable
pub type RfcofipR = crate::BitReader<Rfcofip>;
impl RfcofipR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rfcofip {
        match self.bits {
            false => Rfcofip::_0,
            true => Rfcofip::_1,
        }
    }
    ///Disable receive frame counter overflow interrupt requests
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rfcofip::_0
    }
    ///Enable receive frame counter overflow interrupt requests.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rfcofip::_1
    }
}
///Field `RFCOFIP` writer - Receive Frame Counter Overflow Interrupt Request Enable
pub type RfcofipW<'a, REG> = crate::BitWriter<'a, REG, Rfcofip>;
impl<'a, REG> RfcofipW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable receive frame counter overflow interrupt requests
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rfcofip::_0)
    }
    ///Enable receive frame counter overflow interrupt requests.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rfcofip::_1)
    }
}
/**Receive Abort Detect Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rabtip {
    ///0: Disable receive abort detected interrupt requests
    _0 = 0,
    ///1: Enable receive abort detected interrupt requests.
    _1 = 1,
}
impl From<Rabtip> for bool {
    #[inline(always)]
    fn from(variant: Rabtip) -> Self {
        variant as u8 != 0
    }
}
///Field `RABTIP` reader - Receive Abort Detect Interrupt Request Enable
pub type RabtipR = crate::BitReader<Rabtip>;
impl RabtipR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rabtip {
        match self.bits {
            false => Rabtip::_0,
            true => Rabtip::_1,
        }
    }
    ///Disable receive abort detected interrupt requests
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rabtip::_0
    }
    ///Enable receive abort detected interrupt requests.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rabtip::_1
    }
}
///Field `RABTIP` writer - Receive Abort Detect Interrupt Request Enable
pub type RabtipW<'a, REG> = crate::BitWriter<'a, REG, Rabtip>;
impl<'a, REG> RabtipW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable receive abort detected interrupt requests
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rabtip::_0)
    }
    ///Enable receive abort detected interrupt requests.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rabtip::_1)
    }
}
/**Transmit Abort Detect Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tabtip {
    ///0: Disable transmit abort detected interrupt requests
    _0 = 0,
    ///1: Enable transmit abort detected interrupt requests.
    _1 = 1,
}
impl From<Tabtip> for bool {
    #[inline(always)]
    fn from(variant: Tabtip) -> Self {
        variant as u8 != 0
    }
}
///Field `TABTIP` reader - Transmit Abort Detect Interrupt Request Enable
pub type TabtipR = crate::BitReader<Tabtip>;
impl TabtipR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tabtip {
        match self.bits {
            false => Tabtip::_0,
            true => Tabtip::_1,
        }
    }
    ///Disable transmit abort detected interrupt requests
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tabtip::_0
    }
    ///Enable transmit abort detected interrupt requests.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tabtip::_1
    }
}
///Field `TABTIP` writer - Transmit Abort Detect Interrupt Request Enable
pub type TabtipW<'a, REG> = crate::BitWriter<'a, REG, Tabtip>;
impl<'a, REG> TabtipW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable transmit abort detected interrupt requests
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tabtip::_0)
    }
    ///Enable transmit abort detected interrupt requests.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tabtip::_1)
    }
}
/**Write-Back Complete Interrupt Request Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Twbip {
    ///0: Disable write-back complete interrupt requests
    _0 = 0,
    ///1: Enable write-back complete interrupt requests.
    _1 = 1,
}
impl From<Twbip> for bool {
    #[inline(always)]
    fn from(variant: Twbip) -> Self {
        variant as u8 != 0
    }
}
///Field `TWBIP` reader - Write-Back Complete Interrupt Request Enable
pub type TwbipR = crate::BitReader<Twbip>;
impl TwbipR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Twbip {
        match self.bits {
            false => Twbip::_0,
            true => Twbip::_1,
        }
    }
    ///Disable write-back complete interrupt requests
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Twbip::_0
    }
    ///Enable write-back complete interrupt requests.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Twbip::_1
    }
}
///Field `TWBIP` writer - Write-Back Complete Interrupt Request Enable
pub type TwbipW<'a, REG> = crate::BitWriter<'a, REG, Twbip>;
impl<'a, REG> TwbipW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable write-back complete interrupt requests
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Twbip::_0)
    }
    ///Enable write-back complete interrupt requests.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Twbip::_1)
    }
}
impl R {
    ///Bit 0 - CRC Error Interrupt Request Enable
    #[inline(always)]
    pub fn cerfip(&self) -> CerfipR {
        CerfipR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - PHY-LSI Receive Error Interrupt Request Enable
    #[inline(always)]
    pub fn preip(&self) -> PreipR {
        PreipR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Frame-Too-Short Error Interrupt Request Enable
    #[inline(always)]
    pub fn rtsfip(&self) -> RtsfipR {
        RtsfipR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Frame-Too-Long Error Interrupt Request Enable
    #[inline(always)]
    pub fn rtlfip(&self) -> RtlfipR {
        RtlfipR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Alignment Error Interrupt Request Enable
    #[inline(always)]
    pub fn rrfip(&self) -> RrfipR {
        RrfipR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 7 - Multicast Address Frame Receive Interrupt Request Enable
    #[inline(always)]
    pub fn rmafip(&self) -> RmafipR {
        RmafipR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Transmit Retry Over Interrupt Request Enable
    #[inline(always)]
    pub fn troip(&self) -> TroipR {
        TroipR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Late Collision Detect Interrupt Request Enable
    #[inline(always)]
    pub fn cdip(&self) -> CdipR {
        CdipR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Loss of Carrier Detect Interrupt Request Enable
    #[inline(always)]
    pub fn dlcip(&self) -> DlcipR {
        DlcipR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Carrier Not Detect Interrupt Request Enable
    #[inline(always)]
    pub fn cndip(&self) -> CndipR {
        CndipR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 16 - Receive FIFO Overflow Interrupt Request Enable
    #[inline(always)]
    pub fn rfofip(&self) -> RfofipR {
        RfofipR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Receive Descriptor Empty Interrupt Request Enable
    #[inline(always)]
    pub fn rdeip(&self) -> RdeipR {
        RdeipR::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Frame Receive Interrupt Request Enable
    #[inline(always)]
    pub fn frip(&self) -> FripR {
        FripR::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Transmit FIFO Underflow Interrupt Request Enable
    #[inline(always)]
    pub fn tfufip(&self) -> TfufipR {
        TfufipR::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Transmit Descriptor Empty Interrupt Request Enable
    #[inline(always)]
    pub fn tdeip(&self) -> TdeipR {
        TdeipR::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Frame Transfer Complete Interrupt Request Enable
    #[inline(always)]
    pub fn tcip(&self) -> TcipR {
        TcipR::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - ETHERC Status Register Source Interrupt Request Enable
    #[inline(always)]
    pub fn eciip(&self) -> EciipR {
        EciipR::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Address Error Interrupt Request Enable
    #[inline(always)]
    pub fn adeip(&self) -> AdeipR {
        AdeipR::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Receive Frame Counter Overflow Interrupt Request Enable
    #[inline(always)]
    pub fn rfcofip(&self) -> RfcofipR {
        RfcofipR::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Receive Abort Detect Interrupt Request Enable
    #[inline(always)]
    pub fn rabtip(&self) -> RabtipR {
        RabtipR::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Transmit Abort Detect Interrupt Request Enable
    #[inline(always)]
    pub fn tabtip(&self) -> TabtipR {
        TabtipR::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 30 - Write-Back Complete Interrupt Request Enable
    #[inline(always)]
    pub fn twbip(&self) -> TwbipR {
        TwbipR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EESIPR")
            .field("cerfip", &self.cerfip())
            .field("preip", &self.preip())
            .field("rtsfip", &self.rtsfip())
            .field("rtlfip", &self.rtlfip())
            .field("rrfip", &self.rrfip())
            .field("rmafip", &self.rmafip())
            .field("troip", &self.troip())
            .field("cdip", &self.cdip())
            .field("dlcip", &self.dlcip())
            .field("cndip", &self.cndip())
            .field("rfofip", &self.rfofip())
            .field("rdeip", &self.rdeip())
            .field("frip", &self.frip())
            .field("tfufip", &self.tfufip())
            .field("tdeip", &self.tdeip())
            .field("tcip", &self.tcip())
            .field("eciip", &self.eciip())
            .field("adeip", &self.adeip())
            .field("rfcofip", &self.rfcofip())
            .field("rabtip", &self.rabtip())
            .field("tabtip", &self.tabtip())
            .field("twbip", &self.twbip())
            .finish()
    }
}
impl W {
    ///Bit 0 - CRC Error Interrupt Request Enable
    #[inline(always)]
    pub fn cerfip(&mut self) -> CerfipW<EesiprSpec> {
        CerfipW::new(self, 0)
    }
    ///Bit 1 - PHY-LSI Receive Error Interrupt Request Enable
    #[inline(always)]
    pub fn preip(&mut self) -> PreipW<EesiprSpec> {
        PreipW::new(self, 1)
    }
    ///Bit 2 - Frame-Too-Short Error Interrupt Request Enable
    #[inline(always)]
    pub fn rtsfip(&mut self) -> RtsfipW<EesiprSpec> {
        RtsfipW::new(self, 2)
    }
    ///Bit 3 - Frame-Too-Long Error Interrupt Request Enable
    #[inline(always)]
    pub fn rtlfip(&mut self) -> RtlfipW<EesiprSpec> {
        RtlfipW::new(self, 3)
    }
    ///Bit 4 - Alignment Error Interrupt Request Enable
    #[inline(always)]
    pub fn rrfip(&mut self) -> RrfipW<EesiprSpec> {
        RrfipW::new(self, 4)
    }
    ///Bit 7 - Multicast Address Frame Receive Interrupt Request Enable
    #[inline(always)]
    pub fn rmafip(&mut self) -> RmafipW<EesiprSpec> {
        RmafipW::new(self, 7)
    }
    ///Bit 8 - Transmit Retry Over Interrupt Request Enable
    #[inline(always)]
    pub fn troip(&mut self) -> TroipW<EesiprSpec> {
        TroipW::new(self, 8)
    }
    ///Bit 9 - Late Collision Detect Interrupt Request Enable
    #[inline(always)]
    pub fn cdip(&mut self) -> CdipW<EesiprSpec> {
        CdipW::new(self, 9)
    }
    ///Bit 10 - Loss of Carrier Detect Interrupt Request Enable
    #[inline(always)]
    pub fn dlcip(&mut self) -> DlcipW<EesiprSpec> {
        DlcipW::new(self, 10)
    }
    ///Bit 11 - Carrier Not Detect Interrupt Request Enable
    #[inline(always)]
    pub fn cndip(&mut self) -> CndipW<EesiprSpec> {
        CndipW::new(self, 11)
    }
    ///Bit 16 - Receive FIFO Overflow Interrupt Request Enable
    #[inline(always)]
    pub fn rfofip(&mut self) -> RfofipW<EesiprSpec> {
        RfofipW::new(self, 16)
    }
    ///Bit 17 - Receive Descriptor Empty Interrupt Request Enable
    #[inline(always)]
    pub fn rdeip(&mut self) -> RdeipW<EesiprSpec> {
        RdeipW::new(self, 17)
    }
    ///Bit 18 - Frame Receive Interrupt Request Enable
    #[inline(always)]
    pub fn frip(&mut self) -> FripW<EesiprSpec> {
        FripW::new(self, 18)
    }
    ///Bit 19 - Transmit FIFO Underflow Interrupt Request Enable
    #[inline(always)]
    pub fn tfufip(&mut self) -> TfufipW<EesiprSpec> {
        TfufipW::new(self, 19)
    }
    ///Bit 20 - Transmit Descriptor Empty Interrupt Request Enable
    #[inline(always)]
    pub fn tdeip(&mut self) -> TdeipW<EesiprSpec> {
        TdeipW::new(self, 20)
    }
    ///Bit 21 - Frame Transfer Complete Interrupt Request Enable
    #[inline(always)]
    pub fn tcip(&mut self) -> TcipW<EesiprSpec> {
        TcipW::new(self, 21)
    }
    ///Bit 22 - ETHERC Status Register Source Interrupt Request Enable
    #[inline(always)]
    pub fn eciip(&mut self) -> EciipW<EesiprSpec> {
        EciipW::new(self, 22)
    }
    ///Bit 23 - Address Error Interrupt Request Enable
    #[inline(always)]
    pub fn adeip(&mut self) -> AdeipW<EesiprSpec> {
        AdeipW::new(self, 23)
    }
    ///Bit 24 - Receive Frame Counter Overflow Interrupt Request Enable
    #[inline(always)]
    pub fn rfcofip(&mut self) -> RfcofipW<EesiprSpec> {
        RfcofipW::new(self, 24)
    }
    ///Bit 25 - Receive Abort Detect Interrupt Request Enable
    #[inline(always)]
    pub fn rabtip(&mut self) -> RabtipW<EesiprSpec> {
        RabtipW::new(self, 25)
    }
    ///Bit 26 - Transmit Abort Detect Interrupt Request Enable
    #[inline(always)]
    pub fn tabtip(&mut self) -> TabtipW<EesiprSpec> {
        TabtipW::new(self, 26)
    }
    ///Bit 30 - Write-Back Complete Interrupt Request Enable
    #[inline(always)]
    pub fn twbip(&mut self) -> TwbipW<EesiprSpec> {
        TwbipW::new(self, 30)
    }
}
/**ETHERC/EDMAC Status Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`eesipr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eesipr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct EesiprSpec;
impl crate::RegisterSpec for EesiprSpec {
    type Ux = u32;
}
///`read()` method returns [`eesipr::R`](R) reader structure
impl crate::Readable for EesiprSpec {}
///`write(|w| ..)` method takes [`eesipr::W`](W) writer structure
impl crate::Writable for EesiprSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EESIPR to value 0
impl crate::Resettable for EesiprSpec {}
