///Register `SPCMD%s` reader
pub type R = crate::R<SpcmdSpec>;
///Register `SPCMD%s` writer
pub type W = crate::W<SpcmdSpec>;
/**RSPCK Phase Setting

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpha {
    ///0: Select data sampling on leading edge, data change on trailing edge
    _0 = 0,
    ///1: Select data change on leading edge, data sampling on trailing edge
    _1 = 1,
}
impl From<Cpha> for bool {
    #[inline(always)]
    fn from(variant: Cpha) -> Self {
        variant as u8 != 0
    }
}
///Field `CPHA` reader - RSPCK Phase Setting
pub type CphaR = crate::BitReader<Cpha>;
impl CphaR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cpha {
        match self.bits {
            false => Cpha::_0,
            true => Cpha::_1,
        }
    }
    ///Select data sampling on leading edge, data change on trailing edge
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cpha::_0
    }
    ///Select data change on leading edge, data sampling on trailing edge
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cpha::_1
    }
}
///Field `CPHA` writer - RSPCK Phase Setting
pub type CphaW<'a, REG> = crate::BitWriter<'a, REG, Cpha>;
impl<'a, REG> CphaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Select data sampling on leading edge, data change on trailing edge
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cpha::_0)
    }
    ///Select data change on leading edge, data sampling on trailing edge
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cpha::_1)
    }
}
/**RSPCK Polarity Setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpol {
    ///0: Set RSPCK low during idle
    _0 = 0,
    ///1: Set RSPCK high during idle
    _1 = 1,
}
impl From<Cpol> for bool {
    #[inline(always)]
    fn from(variant: Cpol) -> Self {
        variant as u8 != 0
    }
}
///Field `CPOL` reader - RSPCK Polarity Setting
pub type CpolR = crate::BitReader<Cpol>;
impl CpolR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cpol {
        match self.bits {
            false => Cpol::_0,
            true => Cpol::_1,
        }
    }
    ///Set RSPCK low during idle
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cpol::_0
    }
    ///Set RSPCK high during idle
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cpol::_1
    }
}
///Field `CPOL` writer - RSPCK Polarity Setting
pub type CpolW<'a, REG> = crate::BitWriter<'a, REG, Cpol>;
impl<'a, REG> CpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Set RSPCK low during idle
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cpol::_0)
    }
    ///Set RSPCK high during idle
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cpol::_1)
    }
}
/**Bit Rate Division Setting

Value on reset: 3*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Brdv {
    ///0: Base bit rate
    _00 = 0,
    ///1: Base bit rate divided by 2
    _01 = 1,
    ///2: Base bit rate divided by 4
    _10 = 2,
    ///3: Base bit rate divided by 8
    _11 = 3,
}
impl From<Brdv> for u8 {
    #[inline(always)]
    fn from(variant: Brdv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Brdv {
    type Ux = u8;
}
impl crate::IsEnum for Brdv {}
///Field `BRDV` reader - Bit Rate Division Setting
pub type BrdvR = crate::FieldReader<Brdv>;
impl BrdvR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Brdv {
        match self.bits {
            0 => Brdv::_00,
            1 => Brdv::_01,
            2 => Brdv::_10,
            3 => Brdv::_11,
            _ => unreachable!(),
        }
    }
    ///Base bit rate
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Brdv::_00
    }
    ///Base bit rate divided by 2
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Brdv::_01
    }
    ///Base bit rate divided by 4
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Brdv::_10
    }
    ///Base bit rate divided by 8
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Brdv::_11
    }
}
///Field `BRDV` writer - Bit Rate Division Setting
pub type BrdvW<'a, REG> = crate::FieldWriter<'a, REG, 2, Brdv, crate::Safe>;
impl<'a, REG> BrdvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Base bit rate
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Brdv::_00)
    }
    ///Base bit rate divided by 2
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Brdv::_01)
    }
    ///Base bit rate divided by 4
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Brdv::_10)
    }
    ///Base bit rate divided by 8
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Brdv::_11)
    }
}
/**SSL Signal Assertion Setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ssla {
    ///0: SSL0
    _000 = 0,
    ///1: SSL1
    _001 = 1,
    ///2: SSL2
    _010 = 2,
    ///3: SSL3
    _011 = 3,
    ///4: Setting prohibited
    Others = 4,
}
impl From<Ssla> for u8 {
    #[inline(always)]
    fn from(variant: Ssla) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ssla {
    type Ux = u8;
}
impl crate::IsEnum for Ssla {}
///Field `SSLA` reader - SSL Signal Assertion Setting
pub type SslaR = crate::FieldReader<Ssla>;
impl SslaR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ssla {
        match self.bits {
            0 => Ssla::_000,
            1 => Ssla::_001,
            2 => Ssla::_010,
            3 => Ssla::_011,
            _ => Ssla::Others,
        }
    }
    ///SSL0
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Ssla::_000
    }
    ///SSL1
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Ssla::_001
    }
    ///SSL2
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Ssla::_010
    }
    ///SSL3
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Ssla::_011
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Ssla::Others)
    }
}
///Field `SSLA` writer - SSL Signal Assertion Setting
pub type SslaW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ssla, crate::Safe>;
impl<'a, REG> SslaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///SSL0
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Ssla::_000)
    }
    ///SSL1
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Ssla::_001)
    }
    ///SSL2
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Ssla::_010)
    }
    ///SSL3
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Ssla::_011)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Ssla::Others)
    }
}
/**SSL Signal Level Keeping

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sslkp {
    ///0: Negate all SSL signals on completion of transfer
    _0 = 0,
    ///1: Keep SSL signal level from the end of transfer until the beginning of the next access
    _1 = 1,
}
impl From<Sslkp> for bool {
    #[inline(always)]
    fn from(variant: Sslkp) -> Self {
        variant as u8 != 0
    }
}
///Field `SSLKP` reader - SSL Signal Level Keeping
pub type SslkpR = crate::BitReader<Sslkp>;
impl SslkpR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sslkp {
        match self.bits {
            false => Sslkp::_0,
            true => Sslkp::_1,
        }
    }
    ///Negate all SSL signals on completion of transfer
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sslkp::_0
    }
    ///Keep SSL signal level from the end of transfer until the beginning of the next access
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sslkp::_1
    }
}
///Field `SSLKP` writer - SSL Signal Level Keeping
pub type SslkpW<'a, REG> = crate::BitWriter<'a, REG, Sslkp>;
impl<'a, REG> SslkpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Negate all SSL signals on completion of transfer
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sslkp::_0)
    }
    ///Keep SSL signal level from the end of transfer until the beginning of the next access
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sslkp::_1)
    }
}
/**SPI Data Length Setting

Value on reset: 7*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Spb {
    ///0: 20 bits
    _0x0 = 0,
    ///1: 24 bits
    _0x1 = 1,
    ///2: 32 bits
    _0x2 = 2,
    ///3: 32 bits
    _0x3 = 3,
    ///8: 9 bits
    _0x8 = 8,
    ///9: 10 bits
    _0x9 = 9,
    ///10: 11 bits
    _0xA = 10,
    ///11: 12 bits
    _0xB = 11,
    ///12: 13 bits
    _0xC = 12,
    ///13: 14 bits
    _0xD = 13,
    ///14: 15 bits
    _0xE = 14,
    ///15: 16 bits
    _0xF = 15,
    ///4: 8 bits
    Others = 4,
}
impl From<Spb> for u8 {
    #[inline(always)]
    fn from(variant: Spb) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Spb {
    type Ux = u8;
}
impl crate::IsEnum for Spb {}
///Field `SPB` reader - SPI Data Length Setting
pub type SpbR = crate::FieldReader<Spb>;
impl SpbR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Spb {
        match self.bits {
            0 => Spb::_0x0,
            1 => Spb::_0x1,
            2 => Spb::_0x2,
            3 => Spb::_0x3,
            8 => Spb::_0x8,
            9 => Spb::_0x9,
            10 => Spb::_0xA,
            11 => Spb::_0xB,
            12 => Spb::_0xC,
            13 => Spb::_0xD,
            14 => Spb::_0xE,
            15 => Spb::_0xF,
            _ => Spb::Others,
        }
    }
    ///20 bits
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == Spb::_0x0
    }
    ///24 bits
    #[inline(always)]
    pub fn is_0x1(&self) -> bool {
        *self == Spb::_0x1
    }
    ///32 bits
    #[inline(always)]
    pub fn is_0x2(&self) -> bool {
        *self == Spb::_0x2
    }
    ///32 bits
    #[inline(always)]
    pub fn is_0x3(&self) -> bool {
        *self == Spb::_0x3
    }
    ///9 bits
    #[inline(always)]
    pub fn is_0x8(&self) -> bool {
        *self == Spb::_0x8
    }
    ///10 bits
    #[inline(always)]
    pub fn is_0x9(&self) -> bool {
        *self == Spb::_0x9
    }
    ///11 bits
    #[inline(always)]
    pub fn is_0x_a(&self) -> bool {
        *self == Spb::_0xA
    }
    ///12 bits
    #[inline(always)]
    pub fn is_0x_b(&self) -> bool {
        *self == Spb::_0xB
    }
    ///13 bits
    #[inline(always)]
    pub fn is_0x_c(&self) -> bool {
        *self == Spb::_0xC
    }
    ///14 bits
    #[inline(always)]
    pub fn is_0x_d(&self) -> bool {
        *self == Spb::_0xD
    }
    ///15 bits
    #[inline(always)]
    pub fn is_0x_e(&self) -> bool {
        *self == Spb::_0xE
    }
    ///16 bits
    #[inline(always)]
    pub fn is_0x_f(&self) -> bool {
        *self == Spb::_0xF
    }
    ///8 bits
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Spb::Others)
    }
}
///Field `SPB` writer - SPI Data Length Setting
pub type SpbW<'a, REG> = crate::FieldWriter<'a, REG, 4, Spb, crate::Safe>;
impl<'a, REG> SpbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///20 bits
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Spb::_0x0)
    }
    ///24 bits
    #[inline(always)]
    pub fn _0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Spb::_0x1)
    }
    ///32 bits
    #[inline(always)]
    pub fn _0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Spb::_0x2)
    }
    ///32 bits
    #[inline(always)]
    pub fn _0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Spb::_0x3)
    }
    ///9 bits
    #[inline(always)]
    pub fn _0x8(self) -> &'a mut crate::W<REG> {
        self.variant(Spb::_0x8)
    }
    ///10 bits
    #[inline(always)]
    pub fn _0x9(self) -> &'a mut crate::W<REG> {
        self.variant(Spb::_0x9)
    }
    ///11 bits
    #[inline(always)]
    pub fn _0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(Spb::_0xA)
    }
    ///12 bits
    #[inline(always)]
    pub fn _0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(Spb::_0xB)
    }
    ///13 bits
    #[inline(always)]
    pub fn _0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(Spb::_0xC)
    }
    ///14 bits
    #[inline(always)]
    pub fn _0x_d(self) -> &'a mut crate::W<REG> {
        self.variant(Spb::_0xD)
    }
    ///15 bits
    #[inline(always)]
    pub fn _0x_e(self) -> &'a mut crate::W<REG> {
        self.variant(Spb::_0xE)
    }
    ///16 bits
    #[inline(always)]
    pub fn _0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(Spb::_0xF)
    }
    ///8 bits
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Spb::Others)
    }
}
/**SPI LSB First

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lsbf {
    ///0: MSB-first
    _0 = 0,
    ///1: LSB-first
    _1 = 1,
}
impl From<Lsbf> for bool {
    #[inline(always)]
    fn from(variant: Lsbf) -> Self {
        variant as u8 != 0
    }
}
///Field `LSBF` reader - SPI LSB First
pub type LsbfR = crate::BitReader<Lsbf>;
impl LsbfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Lsbf {
        match self.bits {
            false => Lsbf::_0,
            true => Lsbf::_1,
        }
    }
    ///MSB-first
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lsbf::_0
    }
    ///LSB-first
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lsbf::_1
    }
}
///Field `LSBF` writer - SPI LSB First
pub type LsbfW<'a, REG> = crate::BitWriter<'a, REG, Lsbf>;
impl<'a, REG> LsbfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///MSB-first
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Lsbf::_0)
    }
    ///LSB-first
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Lsbf::_1)
    }
}
/**SPI Next-Access Delay Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spnden {
    ///0: Select next-access delay of 1 RSPCK + 2 PCLKA
    _0 = 0,
    ///1: Select next-access delay equal to the setting in the SPI Next-Access Delay Register (SPND)
    _1 = 1,
}
impl From<Spnden> for bool {
    #[inline(always)]
    fn from(variant: Spnden) -> Self {
        variant as u8 != 0
    }
}
///Field `SPNDEN` reader - SPI Next-Access Delay Enable
pub type SpndenR = crate::BitReader<Spnden>;
impl SpndenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Spnden {
        match self.bits {
            false => Spnden::_0,
            true => Spnden::_1,
        }
    }
    ///Select next-access delay of 1 RSPCK + 2 PCLKA
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Spnden::_0
    }
    ///Select next-access delay equal to the setting in the SPI Next-Access Delay Register (SPND)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Spnden::_1
    }
}
///Field `SPNDEN` writer - SPI Next-Access Delay Enable
pub type SpndenW<'a, REG> = crate::BitWriter<'a, REG, Spnden>;
impl<'a, REG> SpndenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Select next-access delay of 1 RSPCK + 2 PCLKA
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Spnden::_0)
    }
    ///Select next-access delay equal to the setting in the SPI Next-Access Delay Register (SPND)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Spnden::_1)
    }
}
/**SSL Negation Delay Setting Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slnden {
    ///0: Select SSL negation delay of 1 RSPCK
    _0 = 0,
    ///1: Select SSL negation delay equal to the setting in the SPI Slave Select Negation Delay Register (SSLND)
    _1 = 1,
}
impl From<Slnden> for bool {
    #[inline(always)]
    fn from(variant: Slnden) -> Self {
        variant as u8 != 0
    }
}
///Field `SLNDEN` reader - SSL Negation Delay Setting Enable
pub type SlndenR = crate::BitReader<Slnden>;
impl SlndenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Slnden {
        match self.bits {
            false => Slnden::_0,
            true => Slnden::_1,
        }
    }
    ///Select SSL negation delay of 1 RSPCK
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Slnden::_0
    }
    ///Select SSL negation delay equal to the setting in the SPI Slave Select Negation Delay Register (SSLND)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Slnden::_1
    }
}
///Field `SLNDEN` writer - SSL Negation Delay Setting Enable
pub type SlndenW<'a, REG> = crate::BitWriter<'a, REG, Slnden>;
impl<'a, REG> SlndenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Select SSL negation delay of 1 RSPCK
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Slnden::_0)
    }
    ///Select SSL negation delay equal to the setting in the SPI Slave Select Negation Delay Register (SSLND)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Slnden::_1)
    }
}
/**RSPCK Delay Setting Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sckden {
    ///0: Select RSPCK delay of 1 RSPCK
    _0 = 0,
    ///1: Select RSPCK delay equal to the setting in the SPI Clock Delay Register (SPCKD)
    _1 = 1,
}
impl From<Sckden> for bool {
    #[inline(always)]
    fn from(variant: Sckden) -> Self {
        variant as u8 != 0
    }
}
///Field `SCKDEN` reader - RSPCK Delay Setting Enable
pub type SckdenR = crate::BitReader<Sckden>;
impl SckdenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sckden {
        match self.bits {
            false => Sckden::_0,
            true => Sckden::_1,
        }
    }
    ///Select RSPCK delay of 1 RSPCK
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sckden::_0
    }
    ///Select RSPCK delay equal to the setting in the SPI Clock Delay Register (SPCKD)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sckden::_1
    }
}
///Field `SCKDEN` writer - RSPCK Delay Setting Enable
pub type SckdenW<'a, REG> = crate::BitWriter<'a, REG, Sckden>;
impl<'a, REG> SckdenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Select RSPCK delay of 1 RSPCK
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sckden::_0)
    }
    ///Select RSPCK delay equal to the setting in the SPI Clock Delay Register (SPCKD)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sckden::_1)
    }
}
impl R {
    ///Bit 0 - RSPCK Phase Setting
    #[inline(always)]
    pub fn cpha(&self) -> CphaR {
        CphaR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - RSPCK Polarity Setting
    #[inline(always)]
    pub fn cpol(&self) -> CpolR {
        CpolR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - Bit Rate Division Setting
    #[inline(always)]
    pub fn brdv(&self) -> BrdvR {
        BrdvR::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:6 - SSL Signal Assertion Setting
    #[inline(always)]
    pub fn ssla(&self) -> SslaR {
        SslaR::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - SSL Signal Level Keeping
    #[inline(always)]
    pub fn sslkp(&self) -> SslkpR {
        SslkpR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:11 - SPI Data Length Setting
    #[inline(always)]
    pub fn spb(&self) -> SpbR {
        SpbR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 12 - SPI LSB First
    #[inline(always)]
    pub fn lsbf(&self) -> LsbfR {
        LsbfR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - SPI Next-Access Delay Enable
    #[inline(always)]
    pub fn spnden(&self) -> SpndenR {
        SpndenR::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - SSL Negation Delay Setting Enable
    #[inline(always)]
    pub fn slnden(&self) -> SlndenR {
        SlndenR::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - RSPCK Delay Setting Enable
    #[inline(always)]
    pub fn sckden(&self) -> SckdenR {
        SckdenR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPCMD")
            .field("cpha", &self.cpha())
            .field("cpol", &self.cpol())
            .field("brdv", &self.brdv())
            .field("ssla", &self.ssla())
            .field("sslkp", &self.sslkp())
            .field("spb", &self.spb())
            .field("lsbf", &self.lsbf())
            .field("spnden", &self.spnden())
            .field("slnden", &self.slnden())
            .field("sckden", &self.sckden())
            .finish()
    }
}
impl W {
    ///Bit 0 - RSPCK Phase Setting
    #[inline(always)]
    pub fn cpha(&mut self) -> CphaW<SpcmdSpec> {
        CphaW::new(self, 0)
    }
    ///Bit 1 - RSPCK Polarity Setting
    #[inline(always)]
    pub fn cpol(&mut self) -> CpolW<SpcmdSpec> {
        CpolW::new(self, 1)
    }
    ///Bits 2:3 - Bit Rate Division Setting
    #[inline(always)]
    pub fn brdv(&mut self) -> BrdvW<SpcmdSpec> {
        BrdvW::new(self, 2)
    }
    ///Bits 4:6 - SSL Signal Assertion Setting
    #[inline(always)]
    pub fn ssla(&mut self) -> SslaW<SpcmdSpec> {
        SslaW::new(self, 4)
    }
    ///Bit 7 - SSL Signal Level Keeping
    #[inline(always)]
    pub fn sslkp(&mut self) -> SslkpW<SpcmdSpec> {
        SslkpW::new(self, 7)
    }
    ///Bits 8:11 - SPI Data Length Setting
    #[inline(always)]
    pub fn spb(&mut self) -> SpbW<SpcmdSpec> {
        SpbW::new(self, 8)
    }
    ///Bit 12 - SPI LSB First
    #[inline(always)]
    pub fn lsbf(&mut self) -> LsbfW<SpcmdSpec> {
        LsbfW::new(self, 12)
    }
    ///Bit 13 - SPI Next-Access Delay Enable
    #[inline(always)]
    pub fn spnden(&mut self) -> SpndenW<SpcmdSpec> {
        SpndenW::new(self, 13)
    }
    ///Bit 14 - SSL Negation Delay Setting Enable
    #[inline(always)]
    pub fn slnden(&mut self) -> SlndenW<SpcmdSpec> {
        SlndenW::new(self, 14)
    }
    ///Bit 15 - RSPCK Delay Setting Enable
    #[inline(always)]
    pub fn sckden(&mut self) -> SckdenW<SpcmdSpec> {
        SckdenW::new(self, 15)
    }
}
/**SPI Command Register %s

You can [`read`](crate::Reg::read) this register and get [`spcmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spcmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SpcmdSpec;
impl crate::RegisterSpec for SpcmdSpec {
    type Ux = u16;
}
///`read()` method returns [`spcmd::R`](R) reader structure
impl crate::Readable for SpcmdSpec {}
///`write(|w| ..)` method takes [`spcmd::W`](W) writer structure
impl crate::Writable for SpcmdSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SPCMD%s to value 0x070d
impl crate::Resettable for SpcmdSpec {
    const RESET_VALUE: u16 = 0x070d;
}
