///Register `SSICR` reader
pub type R = crate::R<SsicrSpec>;
///Register `SSICR` writer
pub type W = crate::W<SsicrSpec>;
/**Reception Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ren {
    ///0: Disables reception
    _0 = 0,
    ///1: Enables reception (starts reception)
    _1 = 1,
}
impl From<Ren> for bool {
    #[inline(always)]
    fn from(variant: Ren) -> Self {
        variant as u8 != 0
    }
}
///Field `REN` reader - Reception Enable
pub type RenR = crate::BitReader<Ren>;
impl RenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ren {
        match self.bits {
            false => Ren::_0,
            true => Ren::_1,
        }
    }
    ///Disables reception
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ren::_0
    }
    ///Enables reception (starts reception)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ren::_1
    }
}
///Field `REN` writer - Reception Enable
pub type RenW<'a, REG> = crate::BitWriter<'a, REG, Ren>;
impl<'a, REG> RenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables reception
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ren::_0)
    }
    ///Enables reception (starts reception)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ren::_1)
    }
}
/**Transmission Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ten {
    ///0: Disables transmission
    _0 = 0,
    ///1: Enables transmission (starts transmission)
    _1 = 1,
}
impl From<Ten> for bool {
    #[inline(always)]
    fn from(variant: Ten) -> Self {
        variant as u8 != 0
    }
}
///Field `TEN` reader - Transmission Enable
pub type TenR = crate::BitReader<Ten>;
impl TenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ten {
        match self.bits {
            false => Ten::_0,
            true => Ten::_1,
        }
    }
    ///Disables transmission
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ten::_0
    }
    ///Enables transmission (starts transmission)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ten::_1
    }
}
///Field `TEN` writer - Transmission Enable
pub type TenW<'a, REG> = crate::BitWriter<'a, REG, Ten>;
impl<'a, REG> TenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables transmission
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ten::_0)
    }
    ///Enables transmission (starts transmission)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ten::_1)
    }
}
/**Mute Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Muen {
    ///0: Disables muting on the next frame boundary
    _0 = 0,
    ///1: Enables muting on the next frame boundary
    _1 = 1,
}
impl From<Muen> for bool {
    #[inline(always)]
    fn from(variant: Muen) -> Self {
        variant as u8 != 0
    }
}
///Field `MUEN` reader - Mute Enable
pub type MuenR = crate::BitReader<Muen>;
impl MuenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Muen {
        match self.bits {
            false => Muen::_0,
            true => Muen::_1,
        }
    }
    ///Disables muting on the next frame boundary
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Muen::_0
    }
    ///Enables muting on the next frame boundary
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Muen::_1
    }
}
///Field `MUEN` writer - Mute Enable
pub type MuenW<'a, REG> = crate::BitWriter<'a, REG, Muen>;
impl<'a, REG> MuenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables muting on the next frame boundary
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Muen::_0)
    }
    ///Enables muting on the next frame boundary
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Muen::_1)
    }
}
/**Selects Bit Clock Division Ratio

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ckdv {
    ///0: AUDIO_MCK
    _0x0 = 0,
    ///1: AUDIO_MCK/2
    _0x1 = 1,
    ///2: AUDIO_MCK/4
    _0x2 = 2,
    ///3: AUDIO_MCK/8
    _0x3 = 3,
    ///4: AUDIO_MCK/16
    _0x4 = 4,
    ///5: AUDIO_MCK/32
    _0x5 = 5,
    ///6: AUDIO_MCK/64
    _0x6 = 6,
    ///7: AUDIO_MCK/128
    _0x7 = 7,
    ///8: AUDIO_MCK/6
    _0x8 = 8,
    ///9: AUDIO_MCK/12
    _0x9 = 9,
    ///10: AUDIO_MCK/24
    _0xA = 10,
    ///11: AUDIO_MCK/48
    _0xB = 11,
    ///12: AUDIO_MCK/96
    _0xC = 12,
    ///13: Setting prohibited
    Others = 13,
}
impl From<Ckdv> for u8 {
    #[inline(always)]
    fn from(variant: Ckdv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ckdv {
    type Ux = u8;
}
impl crate::IsEnum for Ckdv {}
///Field `CKDV` reader - Selects Bit Clock Division Ratio
pub type CkdvR = crate::FieldReader<Ckdv>;
impl CkdvR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ckdv {
        match self.bits {
            0 => Ckdv::_0x0,
            1 => Ckdv::_0x1,
            2 => Ckdv::_0x2,
            3 => Ckdv::_0x3,
            4 => Ckdv::_0x4,
            5 => Ckdv::_0x5,
            6 => Ckdv::_0x6,
            7 => Ckdv::_0x7,
            8 => Ckdv::_0x8,
            9 => Ckdv::_0x9,
            10 => Ckdv::_0xA,
            11 => Ckdv::_0xB,
            12 => Ckdv::_0xC,
            _ => Ckdv::Others,
        }
    }
    ///AUDIO_MCK
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == Ckdv::_0x0
    }
    ///AUDIO_MCK/2
    #[inline(always)]
    pub fn is_0x1(&self) -> bool {
        *self == Ckdv::_0x1
    }
    ///AUDIO_MCK/4
    #[inline(always)]
    pub fn is_0x2(&self) -> bool {
        *self == Ckdv::_0x2
    }
    ///AUDIO_MCK/8
    #[inline(always)]
    pub fn is_0x3(&self) -> bool {
        *self == Ckdv::_0x3
    }
    ///AUDIO_MCK/16
    #[inline(always)]
    pub fn is_0x4(&self) -> bool {
        *self == Ckdv::_0x4
    }
    ///AUDIO_MCK/32
    #[inline(always)]
    pub fn is_0x5(&self) -> bool {
        *self == Ckdv::_0x5
    }
    ///AUDIO_MCK/64
    #[inline(always)]
    pub fn is_0x6(&self) -> bool {
        *self == Ckdv::_0x6
    }
    ///AUDIO_MCK/128
    #[inline(always)]
    pub fn is_0x7(&self) -> bool {
        *self == Ckdv::_0x7
    }
    ///AUDIO_MCK/6
    #[inline(always)]
    pub fn is_0x8(&self) -> bool {
        *self == Ckdv::_0x8
    }
    ///AUDIO_MCK/12
    #[inline(always)]
    pub fn is_0x9(&self) -> bool {
        *self == Ckdv::_0x9
    }
    ///AUDIO_MCK/24
    #[inline(always)]
    pub fn is_0x_a(&self) -> bool {
        *self == Ckdv::_0xA
    }
    ///AUDIO_MCK/48
    #[inline(always)]
    pub fn is_0x_b(&self) -> bool {
        *self == Ckdv::_0xB
    }
    ///AUDIO_MCK/96
    #[inline(always)]
    pub fn is_0x_c(&self) -> bool {
        *self == Ckdv::_0xC
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Ckdv::Others)
    }
}
///Field `CKDV` writer - Selects Bit Clock Division Ratio
pub type CkdvW<'a, REG> = crate::FieldWriter<'a, REG, 4, Ckdv, crate::Safe>;
impl<'a, REG> CkdvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///AUDIO_MCK
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ckdv::_0x0)
    }
    ///AUDIO_MCK/2
    #[inline(always)]
    pub fn _0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ckdv::_0x1)
    }
    ///AUDIO_MCK/4
    #[inline(always)]
    pub fn _0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Ckdv::_0x2)
    }
    ///AUDIO_MCK/8
    #[inline(always)]
    pub fn _0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Ckdv::_0x3)
    }
    ///AUDIO_MCK/16
    #[inline(always)]
    pub fn _0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Ckdv::_0x4)
    }
    ///AUDIO_MCK/32
    #[inline(always)]
    pub fn _0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Ckdv::_0x5)
    }
    ///AUDIO_MCK/64
    #[inline(always)]
    pub fn _0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Ckdv::_0x6)
    }
    ///AUDIO_MCK/128
    #[inline(always)]
    pub fn _0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Ckdv::_0x7)
    }
    ///AUDIO_MCK/6
    #[inline(always)]
    pub fn _0x8(self) -> &'a mut crate::W<REG> {
        self.variant(Ckdv::_0x8)
    }
    ///AUDIO_MCK/12
    #[inline(always)]
    pub fn _0x9(self) -> &'a mut crate::W<REG> {
        self.variant(Ckdv::_0x9)
    }
    ///AUDIO_MCK/24
    #[inline(always)]
    pub fn _0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(Ckdv::_0xA)
    }
    ///AUDIO_MCK/48
    #[inline(always)]
    pub fn _0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(Ckdv::_0xB)
    }
    ///AUDIO_MCK/96
    #[inline(always)]
    pub fn _0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(Ckdv::_0xC)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Ckdv::Others)
    }
}
/**Selects Serial Data Delay

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Del {
    ///0: Delay of 1 cycle of SSIBCK0 between SSILRCK0/SSIFS0 and SSITXD0/SSIRXD0/SSIDATA0
    _0 = 0,
    ///1: No delay between SSILRCK0/SSIFS0 and SSITXD0/SSIRXD0/SSIDATA0
    _1 = 1,
}
impl From<Del> for bool {
    #[inline(always)]
    fn from(variant: Del) -> Self {
        variant as u8 != 0
    }
}
///Field `DEL` reader - Selects Serial Data Delay
pub type DelR = crate::BitReader<Del>;
impl DelR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Del {
        match self.bits {
            false => Del::_0,
            true => Del::_1,
        }
    }
    ///Delay of 1 cycle of SSIBCK0 between SSILRCK0/SSIFS0 and SSITXD0/SSIRXD0/SSIDATA0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Del::_0
    }
    ///No delay between SSILRCK0/SSIFS0 and SSITXD0/SSIRXD0/SSIDATA0
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Del::_1
    }
}
///Field `DEL` writer - Selects Serial Data Delay
pub type DelW<'a, REG> = crate::BitWriter<'a, REG, Del>;
impl<'a, REG> DelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Delay of 1 cycle of SSIBCK0 between SSILRCK0/SSIFS0 and SSITXD0/SSIRXD0/SSIDATA0
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Del::_0)
    }
    ///No delay between SSILRCK0/SSIFS0 and SSITXD0/SSIRXD0/SSIDATA0
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Del::_1)
    }
}
/**Selects Placement Data Alignment

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdta {
    ///0: Left-justifies placement data (SSIFTDR, SSIFRDR)
    _0 = 0,
    ///1: Right-justifies placement data (SSIFTDR, SSIFRDR)
    _1 = 1,
}
impl From<Pdta> for bool {
    #[inline(always)]
    fn from(variant: Pdta) -> Self {
        variant as u8 != 0
    }
}
///Field `PDTA` reader - Selects Placement Data Alignment
pub type PdtaR = crate::BitReader<Pdta>;
impl PdtaR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pdta {
        match self.bits {
            false => Pdta::_0,
            true => Pdta::_1,
        }
    }
    ///Left-justifies placement data (SSIFTDR, SSIFRDR)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pdta::_0
    }
    ///Right-justifies placement data (SSIFTDR, SSIFRDR)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pdta::_1
    }
}
///Field `PDTA` writer - Selects Placement Data Alignment
pub type PdtaW<'a, REG> = crate::BitWriter<'a, REG, Pdta>;
impl<'a, REG> PdtaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Left-justifies placement data (SSIFTDR, SSIFRDR)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pdta::_0)
    }
    ///Right-justifies placement data (SSIFTDR, SSIFRDR)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pdta::_1)
    }
}
/**Selects Serial Data Alignment

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdta {
    ///0: Transmits and receives serial data first and then padding bits
    _0 = 0,
    ///1: Transmit and receives padding bits first and then serial data
    _1 = 1,
}
impl From<Sdta> for bool {
    #[inline(always)]
    fn from(variant: Sdta) -> Self {
        variant as u8 != 0
    }
}
///Field `SDTA` reader - Selects Serial Data Alignment
pub type SdtaR = crate::BitReader<Sdta>;
impl SdtaR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sdta {
        match self.bits {
            false => Sdta::_0,
            true => Sdta::_1,
        }
    }
    ///Transmits and receives serial data first and then padding bits
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sdta::_0
    }
    ///Transmit and receives padding bits first and then serial data
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sdta::_1
    }
}
///Field `SDTA` writer - Selects Serial Data Alignment
pub type SdtaW<'a, REG> = crate::BitWriter<'a, REG, Sdta>;
impl<'a, REG> SdtaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Transmits and receives serial data first and then padding bits
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sdta::_0)
    }
    ///Transmit and receives padding bits first and then serial data
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sdta::_1)
    }
}
/**Selects Serial Padding Polarity

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spdp {
    ///0: Padding data is at a low level
    _0 = 0,
    ///1: Padding data is at a high level
    _1 = 1,
}
impl From<Spdp> for bool {
    #[inline(always)]
    fn from(variant: Spdp) -> Self {
        variant as u8 != 0
    }
}
///Field `SPDP` reader - Selects Serial Padding Polarity
pub type SpdpR = crate::BitReader<Spdp>;
impl SpdpR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Spdp {
        match self.bits {
            false => Spdp::_0,
            true => Spdp::_1,
        }
    }
    ///Padding data is at a low level
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Spdp::_0
    }
    ///Padding data is at a high level
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Spdp::_1
    }
}
///Field `SPDP` writer - Selects Serial Padding Polarity
pub type SpdpW<'a, REG> = crate::BitWriter<'a, REG, Spdp>;
impl<'a, REG> SpdpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Padding data is at a low level
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Spdp::_0)
    }
    ///Padding data is at a high level
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Spdp::_1)
    }
}
/**Selects the Initial Value and Polarity of LR Clock/Frame Synchronization Signal

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lrckp {
    ///0: The initial value is at a high level. The start trigger for a frame is synchronized with a falling edge of SSILRCK0/SSIFS0.
    _0 = 0,
    ///1: The initial value is at a low level. The start trigger for a frame is synchronized with a rising edge of SSILRCK0/SSIFS0.
    _1 = 1,
}
impl From<Lrckp> for bool {
    #[inline(always)]
    fn from(variant: Lrckp) -> Self {
        variant as u8 != 0
    }
}
///Field `LRCKP` reader - Selects the Initial Value and Polarity of LR Clock/Frame Synchronization Signal
pub type LrckpR = crate::BitReader<Lrckp>;
impl LrckpR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Lrckp {
        match self.bits {
            false => Lrckp::_0,
            true => Lrckp::_1,
        }
    }
    ///The initial value is at a high level. The start trigger for a frame is synchronized with a falling edge of SSILRCK0/SSIFS0.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lrckp::_0
    }
    ///The initial value is at a low level. The start trigger for a frame is synchronized with a rising edge of SSILRCK0/SSIFS0.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lrckp::_1
    }
}
///Field `LRCKP` writer - Selects the Initial Value and Polarity of LR Clock/Frame Synchronization Signal
pub type LrckpW<'a, REG> = crate::BitWriter<'a, REG, Lrckp>;
impl<'a, REG> LrckpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The initial value is at a high level. The start trigger for a frame is synchronized with a falling edge of SSILRCK0/SSIFS0.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Lrckp::_0)
    }
    ///The initial value is at a low level. The start trigger for a frame is synchronized with a rising edge of SSILRCK0/SSIFS0.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Lrckp::_1)
    }
}
/**Selects Bit Clock Polarity

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bckp {
    ///0: SSILRCK0/SSIFS0 and SSITXD0/SSIRXD0/SSIDATA0 change at a falling edge (SSILRCK0/SSIFS0 and SSIRXD0/SSIDATA0 are sampled at a rising edge of SSIBCK0).
    _0 = 0,
    ///1: SSILRCK0/SSIFS0 and SSITXD0/SSIRXD0/SSIDATA0 change at a rising edge (SSILRCK0/SSIFS0 and SSIRXD0/SSIDATA0 are sampled at a falling edge of SSIBCK0).
    _1 = 1,
}
impl From<Bckp> for bool {
    #[inline(always)]
    fn from(variant: Bckp) -> Self {
        variant as u8 != 0
    }
}
///Field `BCKP` reader - Selects Bit Clock Polarity
pub type BckpR = crate::BitReader<Bckp>;
impl BckpR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Bckp {
        match self.bits {
            false => Bckp::_0,
            true => Bckp::_1,
        }
    }
    ///SSILRCK0/SSIFS0 and SSITXD0/SSIRXD0/SSIDATA0 change at a falling edge (SSILRCK0/SSIFS0 and SSIRXD0/SSIDATA0 are sampled at a rising edge of SSIBCK0).
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bckp::_0
    }
    ///SSILRCK0/SSIFS0 and SSITXD0/SSIRXD0/SSIDATA0 change at a rising edge (SSILRCK0/SSIFS0 and SSIRXD0/SSIDATA0 are sampled at a falling edge of SSIBCK0).
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bckp::_1
    }
}
///Field `BCKP` writer - Selects Bit Clock Polarity
pub type BckpW<'a, REG> = crate::BitWriter<'a, REG, Bckp>;
impl<'a, REG> BckpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SSILRCK0/SSIFS0 and SSITXD0/SSIRXD0/SSIDATA0 change at a falling edge (SSILRCK0/SSIFS0 and SSIRXD0/SSIDATA0 are sampled at a rising edge of SSIBCK0).
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bckp::_0)
    }
    ///SSILRCK0/SSIFS0 and SSITXD0/SSIRXD0/SSIDATA0 change at a rising edge (SSILRCK0/SSIFS0 and SSIRXD0/SSIDATA0 are sampled at a falling edge of SSIBCK0).
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bckp::_1)
    }
}
/**Master Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mst {
    ///0: Slave-mode communication
    _0 = 0,
    ///1: Master-mode communication
    _1 = 1,
}
impl From<Mst> for bool {
    #[inline(always)]
    fn from(variant: Mst) -> Self {
        variant as u8 != 0
    }
}
///Field `MST` reader - Master Enable
pub type MstR = crate::BitReader<Mst>;
impl MstR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mst {
        match self.bits {
            false => Mst::_0,
            true => Mst::_1,
        }
    }
    ///Slave-mode communication
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mst::_0
    }
    ///Master-mode communication
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mst::_1
    }
}
///Field `MST` writer - Master Enable
pub type MstW<'a, REG> = crate::BitWriter<'a, REG, Mst>;
impl<'a, REG> MstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Slave-mode communication
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mst::_0)
    }
    ///Master-mode communication
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mst::_1)
    }
}
/**Selects System Word Length

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Swl {
    ///0: 8 bits
    _000 = 0,
    ///1: 16 bits
    _001 = 1,
    ///2: 24 bits
    _010 = 2,
    ///3: 32 bits
    _011 = 3,
    ///4: 48 bits
    _100 = 4,
    ///5: 64 bits
    _101 = 5,
    ///6: 128 bits
    _110 = 6,
    ///7: 256 bits
    _111 = 7,
}
impl From<Swl> for u8 {
    #[inline(always)]
    fn from(variant: Swl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Swl {
    type Ux = u8;
}
impl crate::IsEnum for Swl {}
///Field `SWL` reader - Selects System Word Length
pub type SwlR = crate::FieldReader<Swl>;
impl SwlR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Swl {
        match self.bits {
            0 => Swl::_000,
            1 => Swl::_001,
            2 => Swl::_010,
            3 => Swl::_011,
            4 => Swl::_100,
            5 => Swl::_101,
            6 => Swl::_110,
            7 => Swl::_111,
            _ => unreachable!(),
        }
    }
    ///8 bits
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Swl::_000
    }
    ///16 bits
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Swl::_001
    }
    ///24 bits
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Swl::_010
    }
    ///32 bits
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Swl::_011
    }
    ///48 bits
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Swl::_100
    }
    ///64 bits
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Swl::_101
    }
    ///128 bits
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Swl::_110
    }
    ///256 bits
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == Swl::_111
    }
}
///Field `SWL` writer - Selects System Word Length
pub type SwlW<'a, REG> = crate::FieldWriter<'a, REG, 3, Swl, crate::Safe>;
impl<'a, REG> SwlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///8 bits
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Swl::_000)
    }
    ///16 bits
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Swl::_001)
    }
    ///24 bits
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Swl::_010)
    }
    ///32 bits
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Swl::_011)
    }
    ///48 bits
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Swl::_100)
    }
    ///64 bits
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Swl::_101)
    }
    ///128 bits
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Swl::_110)
    }
    ///256 bits
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(Swl::_111)
    }
}
/**Selects Data Word Length

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dwl {
    ///0: 8 bits
    _000 = 0,
    ///1: 16 bits
    _001 = 1,
    ///2: 18 bits
    _010 = 2,
    ///3: 20 bits
    _011 = 3,
    ///4: 22 bits
    _100 = 4,
    ///5: 24 bits
    _101 = 5,
    ///6: 32 bits
    _110 = 6,
    ///7: Setting prohibited
    _111 = 7,
}
impl From<Dwl> for u8 {
    #[inline(always)]
    fn from(variant: Dwl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dwl {
    type Ux = u8;
}
impl crate::IsEnum for Dwl {}
///Field `DWL` reader - Selects Data Word Length
pub type DwlR = crate::FieldReader<Dwl>;
impl DwlR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dwl {
        match self.bits {
            0 => Dwl::_000,
            1 => Dwl::_001,
            2 => Dwl::_010,
            3 => Dwl::_011,
            4 => Dwl::_100,
            5 => Dwl::_101,
            6 => Dwl::_110,
            7 => Dwl::_111,
            _ => unreachable!(),
        }
    }
    ///8 bits
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Dwl::_000
    }
    ///16 bits
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Dwl::_001
    }
    ///18 bits
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Dwl::_010
    }
    ///20 bits
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Dwl::_011
    }
    ///22 bits
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Dwl::_100
    }
    ///24 bits
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Dwl::_101
    }
    ///32 bits
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Dwl::_110
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == Dwl::_111
    }
}
///Field `DWL` writer - Selects Data Word Length
pub type DwlW<'a, REG> = crate::FieldWriter<'a, REG, 3, Dwl, crate::Safe>;
impl<'a, REG> DwlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///8 bits
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Dwl::_000)
    }
    ///16 bits
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Dwl::_001)
    }
    ///18 bits
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Dwl::_010)
    }
    ///20 bits
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Dwl::_011)
    }
    ///22 bits
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Dwl::_100)
    }
    ///24 bits
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Dwl::_101)
    }
    ///32 bits
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Dwl::_110)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(Dwl::_111)
    }
}
///Field `FRM` reader - Selects Frame Word Number
pub type FrmR = crate::FieldReader;
///Field `FRM` writer - Selects Frame Word Number
pub type FrmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
/**Idle Mode Interrupt Output Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iien {
    ///0: Disables idle mode interrupt output
    _0 = 0,
    ///1: Enables idle mode interrupt output
    _1 = 1,
}
impl From<Iien> for bool {
    #[inline(always)]
    fn from(variant: Iien) -> Self {
        variant as u8 != 0
    }
}
///Field `IIEN` reader - Idle Mode Interrupt Output Enable
pub type IienR = crate::BitReader<Iien>;
impl IienR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Iien {
        match self.bits {
            false => Iien::_0,
            true => Iien::_1,
        }
    }
    ///Disables idle mode interrupt output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Iien::_0
    }
    ///Enables idle mode interrupt output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Iien::_1
    }
}
///Field `IIEN` writer - Idle Mode Interrupt Output Enable
pub type IienW<'a, REG> = crate::BitWriter<'a, REG, Iien>;
impl<'a, REG> IienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables idle mode interrupt output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Iien::_0)
    }
    ///Enables idle mode interrupt output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Iien::_1)
    }
}
/**Receive Overflow Interrupt Output Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Roien {
    ///0: Disables receive overflow interrupt output
    _0 = 0,
    ///1: Enables receive overflow interrupt output
    _1 = 1,
}
impl From<Roien> for bool {
    #[inline(always)]
    fn from(variant: Roien) -> Self {
        variant as u8 != 0
    }
}
///Field `ROIEN` reader - Receive Overflow Interrupt Output Enable
pub type RoienR = crate::BitReader<Roien>;
impl RoienR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Roien {
        match self.bits {
            false => Roien::_0,
            true => Roien::_1,
        }
    }
    ///Disables receive overflow interrupt output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Roien::_0
    }
    ///Enables receive overflow interrupt output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Roien::_1
    }
}
///Field `ROIEN` writer - Receive Overflow Interrupt Output Enable
pub type RoienW<'a, REG> = crate::BitWriter<'a, REG, Roien>;
impl<'a, REG> RoienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables receive overflow interrupt output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Roien::_0)
    }
    ///Enables receive overflow interrupt output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Roien::_1)
    }
}
/**Receive Underflow Interrupt Output Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ruien {
    ///0: Disables receive underflow interrupt output
    _0 = 0,
    ///1: Enables receive underflow interrupt output
    _1 = 1,
}
impl From<Ruien> for bool {
    #[inline(always)]
    fn from(variant: Ruien) -> Self {
        variant as u8 != 0
    }
}
///Field `RUIEN` reader - Receive Underflow Interrupt Output Enable
pub type RuienR = crate::BitReader<Ruien>;
impl RuienR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ruien {
        match self.bits {
            false => Ruien::_0,
            true => Ruien::_1,
        }
    }
    ///Disables receive underflow interrupt output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ruien::_0
    }
    ///Enables receive underflow interrupt output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ruien::_1
    }
}
///Field `RUIEN` writer - Receive Underflow Interrupt Output Enable
pub type RuienW<'a, REG> = crate::BitWriter<'a, REG, Ruien>;
impl<'a, REG> RuienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables receive underflow interrupt output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ruien::_0)
    }
    ///Enables receive underflow interrupt output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ruien::_1)
    }
}
/**Transmit Overflow Interrupt Output Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Toien {
    ///0: Disables transmit overflow interrupt output
    _0 = 0,
    ///1: Enables transmit overflow interrupt output
    _1 = 1,
}
impl From<Toien> for bool {
    #[inline(always)]
    fn from(variant: Toien) -> Self {
        variant as u8 != 0
    }
}
///Field `TOIEN` reader - Transmit Overflow Interrupt Output Enable
pub type ToienR = crate::BitReader<Toien>;
impl ToienR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Toien {
        match self.bits {
            false => Toien::_0,
            true => Toien::_1,
        }
    }
    ///Disables transmit overflow interrupt output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Toien::_0
    }
    ///Enables transmit overflow interrupt output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Toien::_1
    }
}
///Field `TOIEN` writer - Transmit Overflow Interrupt Output Enable
pub type ToienW<'a, REG> = crate::BitWriter<'a, REG, Toien>;
impl<'a, REG> ToienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables transmit overflow interrupt output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Toien::_0)
    }
    ///Enables transmit overflow interrupt output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Toien::_1)
    }
}
/**Transmit Underflow Interrupt Output Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tuien {
    ///0: Disables transmit underflow interrupt output
    _0 = 0,
    ///1: Enables transmit underflow interrupt output
    _1 = 1,
}
impl From<Tuien> for bool {
    #[inline(always)]
    fn from(variant: Tuien) -> Self {
        variant as u8 != 0
    }
}
///Field `TUIEN` reader - Transmit Underflow Interrupt Output Enable
pub type TuienR = crate::BitReader<Tuien>;
impl TuienR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tuien {
        match self.bits {
            false => Tuien::_0,
            true => Tuien::_1,
        }
    }
    ///Disables transmit underflow interrupt output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tuien::_0
    }
    ///Enables transmit underflow interrupt output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tuien::_1
    }
}
///Field `TUIEN` writer - Transmit Underflow Interrupt Output Enable
pub type TuienW<'a, REG> = crate::BitWriter<'a, REG, Tuien>;
impl<'a, REG> TuienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables transmit underflow interrupt output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tuien::_0)
    }
    ///Enables transmit underflow interrupt output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tuien::_1)
    }
}
/**Selects an Audio Clock for Master-mode Communication

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cks {
    ///0: Selects the AUDIO_CLK input
    _0 = 0,
    ///1: Selects the GTIOC2A (GPT output)
    _1 = 1,
}
impl From<Cks> for bool {
    #[inline(always)]
    fn from(variant: Cks) -> Self {
        variant as u8 != 0
    }
}
///Field `CKS` reader - Selects an Audio Clock for Master-mode Communication
pub type CksR = crate::BitReader<Cks>;
impl CksR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cks {
        match self.bits {
            false => Cks::_0,
            true => Cks::_1,
        }
    }
    ///Selects the AUDIO_CLK input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cks::_0
    }
    ///Selects the GTIOC2A (GPT output)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cks::_1
    }
}
///Field `CKS` writer - Selects an Audio Clock for Master-mode Communication
pub type CksW<'a, REG> = crate::BitWriter<'a, REG, Cks>;
impl<'a, REG> CksW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Selects the AUDIO_CLK input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cks::_0)
    }
    ///Selects the GTIOC2A (GPT output)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cks::_1)
    }
}
impl R {
    ///Bit 0 - Reception Enable
    #[inline(always)]
    pub fn ren(&self) -> RenR {
        RenR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Transmission Enable
    #[inline(always)]
    pub fn ten(&self) -> TenR {
        TenR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - Mute Enable
    #[inline(always)]
    pub fn muen(&self) -> MuenR {
        MuenR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:7 - Selects Bit Clock Division Ratio
    #[inline(always)]
    pub fn ckdv(&self) -> CkdvR {
        CkdvR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bit 8 - Selects Serial Data Delay
    #[inline(always)]
    pub fn del(&self) -> DelR {
        DelR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Selects Placement Data Alignment
    #[inline(always)]
    pub fn pdta(&self) -> PdtaR {
        PdtaR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Selects Serial Data Alignment
    #[inline(always)]
    pub fn sdta(&self) -> SdtaR {
        SdtaR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Selects Serial Padding Polarity
    #[inline(always)]
    pub fn spdp(&self) -> SpdpR {
        SpdpR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Selects the Initial Value and Polarity of LR Clock/Frame Synchronization Signal
    #[inline(always)]
    pub fn lrckp(&self) -> LrckpR {
        LrckpR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Selects Bit Clock Polarity
    #[inline(always)]
    pub fn bckp(&self) -> BckpR {
        BckpR::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Master Enable
    #[inline(always)]
    pub fn mst(&self) -> MstR {
        MstR::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bits 16:18 - Selects System Word Length
    #[inline(always)]
    pub fn swl(&self) -> SwlR {
        SwlR::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bits 19:21 - Selects Data Word Length
    #[inline(always)]
    pub fn dwl(&self) -> DwlR {
        DwlR::new(((self.bits >> 19) & 7) as u8)
    }
    ///Bits 22:23 - Selects Frame Word Number
    #[inline(always)]
    pub fn frm(&self) -> FrmR {
        FrmR::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bit 25 - Idle Mode Interrupt Output Enable
    #[inline(always)]
    pub fn iien(&self) -> IienR {
        IienR::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Receive Overflow Interrupt Output Enable
    #[inline(always)]
    pub fn roien(&self) -> RoienR {
        RoienR::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Receive Underflow Interrupt Output Enable
    #[inline(always)]
    pub fn ruien(&self) -> RuienR {
        RuienR::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Transmit Overflow Interrupt Output Enable
    #[inline(always)]
    pub fn toien(&self) -> ToienR {
        ToienR::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Transmit Underflow Interrupt Output Enable
    #[inline(always)]
    pub fn tuien(&self) -> TuienR {
        TuienR::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Selects an Audio Clock for Master-mode Communication
    #[inline(always)]
    pub fn cks(&self) -> CksR {
        CksR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SSICR")
            .field("ren", &self.ren())
            .field("ten", &self.ten())
            .field("muen", &self.muen())
            .field("ckdv", &self.ckdv())
            .field("del", &self.del())
            .field("pdta", &self.pdta())
            .field("sdta", &self.sdta())
            .field("spdp", &self.spdp())
            .field("lrckp", &self.lrckp())
            .field("bckp", &self.bckp())
            .field("mst", &self.mst())
            .field("swl", &self.swl())
            .field("dwl", &self.dwl())
            .field("frm", &self.frm())
            .field("iien", &self.iien())
            .field("roien", &self.roien())
            .field("ruien", &self.ruien())
            .field("toien", &self.toien())
            .field("tuien", &self.tuien())
            .field("cks", &self.cks())
            .finish()
    }
}
impl W {
    ///Bit 0 - Reception Enable
    #[inline(always)]
    pub fn ren(&mut self) -> RenW<SsicrSpec> {
        RenW::new(self, 0)
    }
    ///Bit 1 - Transmission Enable
    #[inline(always)]
    pub fn ten(&mut self) -> TenW<SsicrSpec> {
        TenW::new(self, 1)
    }
    ///Bit 3 - Mute Enable
    #[inline(always)]
    pub fn muen(&mut self) -> MuenW<SsicrSpec> {
        MuenW::new(self, 3)
    }
    ///Bits 4:7 - Selects Bit Clock Division Ratio
    #[inline(always)]
    pub fn ckdv(&mut self) -> CkdvW<SsicrSpec> {
        CkdvW::new(self, 4)
    }
    ///Bit 8 - Selects Serial Data Delay
    #[inline(always)]
    pub fn del(&mut self) -> DelW<SsicrSpec> {
        DelW::new(self, 8)
    }
    ///Bit 9 - Selects Placement Data Alignment
    #[inline(always)]
    pub fn pdta(&mut self) -> PdtaW<SsicrSpec> {
        PdtaW::new(self, 9)
    }
    ///Bit 10 - Selects Serial Data Alignment
    #[inline(always)]
    pub fn sdta(&mut self) -> SdtaW<SsicrSpec> {
        SdtaW::new(self, 10)
    }
    ///Bit 11 - Selects Serial Padding Polarity
    #[inline(always)]
    pub fn spdp(&mut self) -> SpdpW<SsicrSpec> {
        SpdpW::new(self, 11)
    }
    ///Bit 12 - Selects the Initial Value and Polarity of LR Clock/Frame Synchronization Signal
    #[inline(always)]
    pub fn lrckp(&mut self) -> LrckpW<SsicrSpec> {
        LrckpW::new(self, 12)
    }
    ///Bit 13 - Selects Bit Clock Polarity
    #[inline(always)]
    pub fn bckp(&mut self) -> BckpW<SsicrSpec> {
        BckpW::new(self, 13)
    }
    ///Bit 14 - Master Enable
    #[inline(always)]
    pub fn mst(&mut self) -> MstW<SsicrSpec> {
        MstW::new(self, 14)
    }
    ///Bits 16:18 - Selects System Word Length
    #[inline(always)]
    pub fn swl(&mut self) -> SwlW<SsicrSpec> {
        SwlW::new(self, 16)
    }
    ///Bits 19:21 - Selects Data Word Length
    #[inline(always)]
    pub fn dwl(&mut self) -> DwlW<SsicrSpec> {
        DwlW::new(self, 19)
    }
    ///Bits 22:23 - Selects Frame Word Number
    #[inline(always)]
    pub fn frm(&mut self) -> FrmW<SsicrSpec> {
        FrmW::new(self, 22)
    }
    ///Bit 25 - Idle Mode Interrupt Output Enable
    #[inline(always)]
    pub fn iien(&mut self) -> IienW<SsicrSpec> {
        IienW::new(self, 25)
    }
    ///Bit 26 - Receive Overflow Interrupt Output Enable
    #[inline(always)]
    pub fn roien(&mut self) -> RoienW<SsicrSpec> {
        RoienW::new(self, 26)
    }
    ///Bit 27 - Receive Underflow Interrupt Output Enable
    #[inline(always)]
    pub fn ruien(&mut self) -> RuienW<SsicrSpec> {
        RuienW::new(self, 27)
    }
    ///Bit 28 - Transmit Overflow Interrupt Output Enable
    #[inline(always)]
    pub fn toien(&mut self) -> ToienW<SsicrSpec> {
        ToienW::new(self, 28)
    }
    ///Bit 29 - Transmit Underflow Interrupt Output Enable
    #[inline(always)]
    pub fn tuien(&mut self) -> TuienW<SsicrSpec> {
        TuienW::new(self, 29)
    }
    ///Bit 30 - Selects an Audio Clock for Master-mode Communication
    #[inline(always)]
    pub fn cks(&mut self) -> CksW<SsicrSpec> {
        CksW::new(self, 30)
    }
}
/**Control Register

You can [`read`](crate::Reg::read) this register and get [`ssicr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssicr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SsicrSpec;
impl crate::RegisterSpec for SsicrSpec {
    type Ux = u32;
}
///`read()` method returns [`ssicr::R`](R) reader structure
impl crate::Readable for SsicrSpec {}
///`write(|w| ..)` method takes [`ssicr::W`](W) writer structure
impl crate::Writable for SsicrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SSICR to value 0
impl crate::Resettable for SsicrSpec {}
