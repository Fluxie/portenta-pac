///Register `CFDCFCC%s` reader
pub type R = crate::R<CfdcfccSpec>;
///Register `CFDCFCC%s` writer
pub type W = crate::W<CfdcfccSpec>;
/**Common FIFO Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cfe {
    ///0: FIFO disabled
    _0 = 0,
    ///1: FIFO enabled
    _1 = 1,
}
impl From<Cfe> for bool {
    #[inline(always)]
    fn from(variant: Cfe) -> Self {
        variant as u8 != 0
    }
}
///Field `CFE` reader - Common FIFO Enable
pub type CfeR = crate::BitReader<Cfe>;
impl CfeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cfe {
        match self.bits {
            false => Cfe::_0,
            true => Cfe::_1,
        }
    }
    ///FIFO disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cfe::_0
    }
    ///FIFO enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cfe::_1
    }
}
///Field `CFE` writer - Common FIFO Enable
pub type CfeW<'a, REG> = crate::BitWriter<'a, REG, Cfe>;
impl<'a, REG> CfeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///FIFO disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cfe::_0)
    }
    ///FIFO enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cfe::_1)
    }
}
/**Common FIFO RX Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cfrxie {
    ///0: FIFO interrupt generation disabled for Frame RX
    _0 = 0,
    ///1: FIFO interrupt generation enabled for Frame RX
    _1 = 1,
}
impl From<Cfrxie> for bool {
    #[inline(always)]
    fn from(variant: Cfrxie) -> Self {
        variant as u8 != 0
    }
}
///Field `CFRXIE` reader - Common FIFO RX Interrupt Enable
pub type CfrxieR = crate::BitReader<Cfrxie>;
impl CfrxieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cfrxie {
        match self.bits {
            false => Cfrxie::_0,
            true => Cfrxie::_1,
        }
    }
    ///FIFO interrupt generation disabled for Frame RX
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cfrxie::_0
    }
    ///FIFO interrupt generation enabled for Frame RX
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cfrxie::_1
    }
}
///Field `CFRXIE` writer - Common FIFO RX Interrupt Enable
pub type CfrxieW<'a, REG> = crate::BitWriter<'a, REG, Cfrxie>;
impl<'a, REG> CfrxieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///FIFO interrupt generation disabled for Frame RX
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cfrxie::_0)
    }
    ///FIFO interrupt generation enabled for Frame RX
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cfrxie::_1)
    }
}
/**Common FIFO TX Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cftxie {
    ///0: FIFO interrupt generation disabled for Frame TX
    _0 = 0,
    ///1: FIFO interrupt generation enabled for Frame TX
    _1 = 1,
}
impl From<Cftxie> for bool {
    #[inline(always)]
    fn from(variant: Cftxie) -> Self {
        variant as u8 != 0
    }
}
///Field `CFTXIE` reader - Common FIFO TX Interrupt Enable
pub type CftxieR = crate::BitReader<Cftxie>;
impl CftxieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cftxie {
        match self.bits {
            false => Cftxie::_0,
            true => Cftxie::_1,
        }
    }
    ///FIFO interrupt generation disabled for Frame TX
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cftxie::_0
    }
    ///FIFO interrupt generation enabled for Frame TX
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cftxie::_1
    }
}
///Field `CFTXIE` writer - Common FIFO TX Interrupt Enable
pub type CftxieW<'a, REG> = crate::BitWriter<'a, REG, Cftxie>;
impl<'a, REG> CftxieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///FIFO interrupt generation disabled for Frame TX
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cftxie::_0)
    }
    ///FIFO interrupt generation enabled for Frame TX
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cftxie::_1)
    }
}
/**Common FIFO Payload Data Size Configuration

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cfpls {
    ///0: 8 bytes
    _000 = 0,
    ///1: 12 bytes
    _001 = 1,
    ///2: 16 bytes
    _010 = 2,
    ///3: 20 bytes
    _011 = 3,
    ///4: 24 bytes
    _100 = 4,
    ///5: 32 bytes
    _101 = 5,
    ///6: 48 bytes
    _110 = 6,
    ///7: 64 bytes
    _111 = 7,
}
impl From<Cfpls> for u8 {
    #[inline(always)]
    fn from(variant: Cfpls) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cfpls {
    type Ux = u8;
}
impl crate::IsEnum for Cfpls {}
///Field `CFPLS` reader - Common FIFO Payload Data Size Configuration
pub type CfplsR = crate::FieldReader<Cfpls>;
impl CfplsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cfpls {
        match self.bits {
            0 => Cfpls::_000,
            1 => Cfpls::_001,
            2 => Cfpls::_010,
            3 => Cfpls::_011,
            4 => Cfpls::_100,
            5 => Cfpls::_101,
            6 => Cfpls::_110,
            7 => Cfpls::_111,
            _ => unreachable!(),
        }
    }
    ///8 bytes
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Cfpls::_000
    }
    ///12 bytes
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Cfpls::_001
    }
    ///16 bytes
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Cfpls::_010
    }
    ///20 bytes
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Cfpls::_011
    }
    ///24 bytes
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Cfpls::_100
    }
    ///32 bytes
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Cfpls::_101
    }
    ///48 bytes
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Cfpls::_110
    }
    ///64 bytes
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == Cfpls::_111
    }
}
///Field `CFPLS` writer - Common FIFO Payload Data Size Configuration
pub type CfplsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Cfpls, crate::Safe>;
impl<'a, REG> CfplsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///8 bytes
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Cfpls::_000)
    }
    ///12 bytes
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Cfpls::_001)
    }
    ///16 bytes
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Cfpls::_010)
    }
    ///20 bytes
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Cfpls::_011)
    }
    ///24 bytes
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Cfpls::_100)
    }
    ///32 bytes
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Cfpls::_101)
    }
    ///48 bytes
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Cfpls::_110)
    }
    ///64 bytes
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(Cfpls::_111)
    }
}
/**Common FIFO Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cfm {
    ///0: RX FIFO mode
    _00 = 0,
    ///1: TX FIFO mode
    _01 = 1,
    ///2: CAN – CAN GW FIFO mode
    _10 = 2,
    ///3: Reserved
    _11 = 3,
}
impl From<Cfm> for u8 {
    #[inline(always)]
    fn from(variant: Cfm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cfm {
    type Ux = u8;
}
impl crate::IsEnum for Cfm {}
///Field `CFM` reader - Common FIFO Mode
pub type CfmR = crate::FieldReader<Cfm>;
impl CfmR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cfm {
        match self.bits {
            0 => Cfm::_00,
            1 => Cfm::_01,
            2 => Cfm::_10,
            3 => Cfm::_11,
            _ => unreachable!(),
        }
    }
    ///RX FIFO mode
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Cfm::_00
    }
    ///TX FIFO mode
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Cfm::_01
    }
    ///CAN – CAN GW FIFO mode
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Cfm::_10
    }
    ///Reserved
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Cfm::_11
    }
}
///Field `CFM` writer - Common FIFO Mode
pub type CfmW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cfm, crate::Safe>;
impl<'a, REG> CfmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///RX FIFO mode
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Cfm::_00)
    }
    ///TX FIFO mode
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Cfm::_01)
    }
    ///CAN – CAN GW FIFO mode
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Cfm::_10)
    }
    ///Reserved
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Cfm::_11)
    }
}
/**Common FIFO Interval Timer Source Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cfitss {
    ///0: Reference clock (× 1 / × 10 period)
    _0 = 0,
    ///1: Bit time clock of related channel (FIFO is linked to fixed channel)
    _1 = 1,
}
impl From<Cfitss> for bool {
    #[inline(always)]
    fn from(variant: Cfitss) -> Self {
        variant as u8 != 0
    }
}
///Field `CFITSS` reader - Common FIFO Interval Timer Source Select
pub type CfitssR = crate::BitReader<Cfitss>;
impl CfitssR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cfitss {
        match self.bits {
            false => Cfitss::_0,
            true => Cfitss::_1,
        }
    }
    ///Reference clock (× 1 / × 10 period)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cfitss::_0
    }
    ///Bit time clock of related channel (FIFO is linked to fixed channel)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cfitss::_1
    }
}
///Field `CFITSS` writer - Common FIFO Interval Timer Source Select
pub type CfitssW<'a, REG> = crate::BitWriter<'a, REG, Cfitss>;
impl<'a, REG> CfitssW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reference clock (× 1 / × 10 period)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cfitss::_0)
    }
    ///Bit time clock of related channel (FIFO is linked to fixed channel)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cfitss::_1)
    }
}
/**Common FIFO Interval Timer Resolution

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cfitr {
    ///0: Reference clock period × 1
    _0 = 0,
    ///1: Reference clock period × 10
    _1 = 1,
}
impl From<Cfitr> for bool {
    #[inline(always)]
    fn from(variant: Cfitr) -> Self {
        variant as u8 != 0
    }
}
///Field `CFITR` reader - Common FIFO Interval Timer Resolution
pub type CfitrR = crate::BitReader<Cfitr>;
impl CfitrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cfitr {
        match self.bits {
            false => Cfitr::_0,
            true => Cfitr::_1,
        }
    }
    ///Reference clock period × 1
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cfitr::_0
    }
    ///Reference clock period × 10
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cfitr::_1
    }
}
///Field `CFITR` writer - Common FIFO Interval Timer Resolution
pub type CfitrW<'a, REG> = crate::BitWriter<'a, REG, Cfitr>;
impl<'a, REG> CfitrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reference clock period × 1
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cfitr::_0)
    }
    ///Reference clock period × 10
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cfitr::_1)
    }
}
/**Common FIFO Interrupt Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cfim {
    ///0: RX FIFO mode: RX interrupt generated when Common FIFO counter reaches CFIGCV value from a lower value TX FIFO mode: TX interrupt generated when Common FIFO transmits the last message successfully GW FIFO mode: For RX interrupt flag: Interrupt generated when FIFO counter increments and reaches the value configured in CFIGCV For TX interrupt flag: Interrupt generated when FIFO transmits the last message successfully
    _0 = 0,
    ///1: RX FIFO mode: RX interrupt generated at the end of every received message storage TX FIFO mode: interrupt generated for every successfully transmitted message GW FIFO mode: For RX interrupt flag: Interrupt generated when a message is stored in the FIFO For TX interrupt flag: Interrupt generated when a message is successfully transmitted from the FIFO
    _1 = 1,
}
impl From<Cfim> for bool {
    #[inline(always)]
    fn from(variant: Cfim) -> Self {
        variant as u8 != 0
    }
}
///Field `CFIM` reader - Common FIFO Interrupt Mode
pub type CfimR = crate::BitReader<Cfim>;
impl CfimR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cfim {
        match self.bits {
            false => Cfim::_0,
            true => Cfim::_1,
        }
    }
    ///RX FIFO mode: RX interrupt generated when Common FIFO counter reaches CFIGCV value from a lower value TX FIFO mode: TX interrupt generated when Common FIFO transmits the last message successfully GW FIFO mode: For RX interrupt flag: Interrupt generated when FIFO counter increments and reaches the value configured in CFIGCV For TX interrupt flag: Interrupt generated when FIFO transmits the last message successfully
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cfim::_0
    }
    ///RX FIFO mode: RX interrupt generated at the end of every received message storage TX FIFO mode: interrupt generated for every successfully transmitted message GW FIFO mode: For RX interrupt flag: Interrupt generated when a message is stored in the FIFO For TX interrupt flag: Interrupt generated when a message is successfully transmitted from the FIFO
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cfim::_1
    }
}
///Field `CFIM` writer - Common FIFO Interrupt Mode
pub type CfimW<'a, REG> = crate::BitWriter<'a, REG, Cfim>;
impl<'a, REG> CfimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///RX FIFO mode: RX interrupt generated when Common FIFO counter reaches CFIGCV value from a lower value TX FIFO mode: TX interrupt generated when Common FIFO transmits the last message successfully GW FIFO mode: For RX interrupt flag: Interrupt generated when FIFO counter increments and reaches the value configured in CFIGCV For TX interrupt flag: Interrupt generated when FIFO transmits the last message successfully
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cfim::_0)
    }
    ///RX FIFO mode: RX interrupt generated at the end of every received message storage TX FIFO mode: interrupt generated for every successfully transmitted message GW FIFO mode: For RX interrupt flag: Interrupt generated when a message is stored in the FIFO For TX interrupt flag: Interrupt generated when a message is successfully transmitted from the FIFO
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cfim::_1)
    }
}
/**Common FIFO Interrupt Generation Counter Value

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cfigcv {
    ///0: Interrupt generated when FIFO is 1/8th full
    _000 = 0,
    ///1: Interrupt generated when FIFO is 1/4th full
    _001 = 1,
    ///2: Interrupt generated when FIFO is 3/8th full
    _010 = 2,
    ///3: Interrupt generated when FIFO is 1/2 full
    _011 = 3,
    ///4: Interrupt generated when FIFO is 5/8th full
    _100 = 4,
    ///5: Interrupt generated when FIFO is 3/4th full
    _101 = 5,
    ///6: Interrupt generated when FIFO is 7/8th full
    _110 = 6,
    ///7: Interrupt generated when FIFO is full
    _111 = 7,
}
impl From<Cfigcv> for u8 {
    #[inline(always)]
    fn from(variant: Cfigcv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cfigcv {
    type Ux = u8;
}
impl crate::IsEnum for Cfigcv {}
///Field `CFIGCV` reader - Common FIFO Interrupt Generation Counter Value
pub type CfigcvR = crate::FieldReader<Cfigcv>;
impl CfigcvR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cfigcv {
        match self.bits {
            0 => Cfigcv::_000,
            1 => Cfigcv::_001,
            2 => Cfigcv::_010,
            3 => Cfigcv::_011,
            4 => Cfigcv::_100,
            5 => Cfigcv::_101,
            6 => Cfigcv::_110,
            7 => Cfigcv::_111,
            _ => unreachable!(),
        }
    }
    ///Interrupt generated when FIFO is 1/8th full
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Cfigcv::_000
    }
    ///Interrupt generated when FIFO is 1/4th full
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Cfigcv::_001
    }
    ///Interrupt generated when FIFO is 3/8th full
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Cfigcv::_010
    }
    ///Interrupt generated when FIFO is 1/2 full
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Cfigcv::_011
    }
    ///Interrupt generated when FIFO is 5/8th full
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Cfigcv::_100
    }
    ///Interrupt generated when FIFO is 3/4th full
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Cfigcv::_101
    }
    ///Interrupt generated when FIFO is 7/8th full
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Cfigcv::_110
    }
    ///Interrupt generated when FIFO is full
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == Cfigcv::_111
    }
}
///Field `CFIGCV` writer - Common FIFO Interrupt Generation Counter Value
pub type CfigcvW<'a, REG> = crate::FieldWriter<'a, REG, 3, Cfigcv, crate::Safe>;
impl<'a, REG> CfigcvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Interrupt generated when FIFO is 1/8th full
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Cfigcv::_000)
    }
    ///Interrupt generated when FIFO is 1/4th full
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Cfigcv::_001)
    }
    ///Interrupt generated when FIFO is 3/8th full
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Cfigcv::_010)
    }
    ///Interrupt generated when FIFO is 1/2 full
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Cfigcv::_011)
    }
    ///Interrupt generated when FIFO is 5/8th full
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Cfigcv::_100)
    }
    ///Interrupt generated when FIFO is 3/4th full
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Cfigcv::_101)
    }
    ///Interrupt generated when FIFO is 7/8th full
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Cfigcv::_110)
    }
    ///Interrupt generated when FIFO is full
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(Cfigcv::_111)
    }
}
///Field `CFTML` reader - Common FIFO TX Message Buffer Link
pub type CftmlR = crate::FieldReader;
///Field `CFTML` writer - Common FIFO TX Message Buffer Link
pub type CftmlW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
/**Common FIFO Depth Configuration

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cfdc {
    ///0: FIFO Depth = 0 messages
    _000 = 0,
    ///1: FIFO Depth = 4 messages
    _001 = 1,
    ///2: FIFO Depth = 8 messages
    _010 = 2,
    ///3: FIFO Depth = 16 messages
    _011 = 3,
    ///4: FIFO Depth = 32 messages
    _100 = 4,
    ///5: FIFO Depth = 48 messages
    _101 = 5,
    ///6: FIFO Depth = 64 messages
    _110 = 6,
    ///7: FIFO Depth = 128 messages
    _111 = 7,
}
impl From<Cfdc> for u8 {
    #[inline(always)]
    fn from(variant: Cfdc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cfdc {
    type Ux = u8;
}
impl crate::IsEnum for Cfdc {}
///Field `CFDC` reader - Common FIFO Depth Configuration
pub type CfdcR = crate::FieldReader<Cfdc>;
impl CfdcR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cfdc {
        match self.bits {
            0 => Cfdc::_000,
            1 => Cfdc::_001,
            2 => Cfdc::_010,
            3 => Cfdc::_011,
            4 => Cfdc::_100,
            5 => Cfdc::_101,
            6 => Cfdc::_110,
            7 => Cfdc::_111,
            _ => unreachable!(),
        }
    }
    ///FIFO Depth = 0 messages
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Cfdc::_000
    }
    ///FIFO Depth = 4 messages
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Cfdc::_001
    }
    ///FIFO Depth = 8 messages
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Cfdc::_010
    }
    ///FIFO Depth = 16 messages
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Cfdc::_011
    }
    ///FIFO Depth = 32 messages
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Cfdc::_100
    }
    ///FIFO Depth = 48 messages
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Cfdc::_101
    }
    ///FIFO Depth = 64 messages
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Cfdc::_110
    }
    ///FIFO Depth = 128 messages
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == Cfdc::_111
    }
}
///Field `CFDC` writer - Common FIFO Depth Configuration
pub type CfdcW<'a, REG> = crate::FieldWriter<'a, REG, 3, Cfdc, crate::Safe>;
impl<'a, REG> CfdcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///FIFO Depth = 0 messages
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Cfdc::_000)
    }
    ///FIFO Depth = 4 messages
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Cfdc::_001)
    }
    ///FIFO Depth = 8 messages
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Cfdc::_010)
    }
    ///FIFO Depth = 16 messages
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Cfdc::_011)
    }
    ///FIFO Depth = 32 messages
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Cfdc::_100)
    }
    ///FIFO Depth = 48 messages
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Cfdc::_101)
    }
    ///FIFO Depth = 64 messages
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Cfdc::_110)
    }
    ///FIFO Depth = 128 messages
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(Cfdc::_111)
    }
}
///Field `CFITT` reader - Common FIFO Interval Transmission Time
pub type CfittR = crate::FieldReader;
///Field `CFITT` writer - Common FIFO Interval Transmission Time
pub type CfittW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bit 0 - Common FIFO Enable
    #[inline(always)]
    pub fn cfe(&self) -> CfeR {
        CfeR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Common FIFO RX Interrupt Enable
    #[inline(always)]
    pub fn cfrxie(&self) -> CfrxieR {
        CfrxieR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Common FIFO TX Interrupt Enable
    #[inline(always)]
    pub fn cftxie(&self) -> CftxieR {
        CftxieR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 4:6 - Common FIFO Payload Data Size Configuration
    #[inline(always)]
    pub fn cfpls(&self) -> CfplsR {
        CfplsR::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 8:9 - Common FIFO Mode
    #[inline(always)]
    pub fn cfm(&self) -> CfmR {
        CfmR::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 10 - Common FIFO Interval Timer Source Select
    #[inline(always)]
    pub fn cfitss(&self) -> CfitssR {
        CfitssR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Common FIFO Interval Timer Resolution
    #[inline(always)]
    pub fn cfitr(&self) -> CfitrR {
        CfitrR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Common FIFO Interrupt Mode
    #[inline(always)]
    pub fn cfim(&self) -> CfimR {
        CfimR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 13:15 - Common FIFO Interrupt Generation Counter Value
    #[inline(always)]
    pub fn cfigcv(&self) -> CfigcvR {
        CfigcvR::new(((self.bits >> 13) & 7) as u8)
    }
    ///Bits 16:20 - Common FIFO TX Message Buffer Link
    #[inline(always)]
    pub fn cftml(&self) -> CftmlR {
        CftmlR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bits 21:23 - Common FIFO Depth Configuration
    #[inline(always)]
    pub fn cfdc(&self) -> CfdcR {
        CfdcR::new(((self.bits >> 21) & 7) as u8)
    }
    ///Bits 24:31 - Common FIFO Interval Transmission Time
    #[inline(always)]
    pub fn cfitt(&self) -> CfittR {
        CfittR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDCFCC")
            .field("cfe", &self.cfe())
            .field("cfrxie", &self.cfrxie())
            .field("cftxie", &self.cftxie())
            .field("cfpls", &self.cfpls())
            .field("cfm", &self.cfm())
            .field("cfitss", &self.cfitss())
            .field("cfitr", &self.cfitr())
            .field("cfim", &self.cfim())
            .field("cfigcv", &self.cfigcv())
            .field("cftml", &self.cftml())
            .field("cfdc", &self.cfdc())
            .field("cfitt", &self.cfitt())
            .finish()
    }
}
impl W {
    ///Bit 0 - Common FIFO Enable
    #[inline(always)]
    pub fn cfe(&mut self) -> CfeW<CfdcfccSpec> {
        CfeW::new(self, 0)
    }
    ///Bit 1 - Common FIFO RX Interrupt Enable
    #[inline(always)]
    pub fn cfrxie(&mut self) -> CfrxieW<CfdcfccSpec> {
        CfrxieW::new(self, 1)
    }
    ///Bit 2 - Common FIFO TX Interrupt Enable
    #[inline(always)]
    pub fn cftxie(&mut self) -> CftxieW<CfdcfccSpec> {
        CftxieW::new(self, 2)
    }
    ///Bits 4:6 - Common FIFO Payload Data Size Configuration
    #[inline(always)]
    pub fn cfpls(&mut self) -> CfplsW<CfdcfccSpec> {
        CfplsW::new(self, 4)
    }
    ///Bits 8:9 - Common FIFO Mode
    #[inline(always)]
    pub fn cfm(&mut self) -> CfmW<CfdcfccSpec> {
        CfmW::new(self, 8)
    }
    ///Bit 10 - Common FIFO Interval Timer Source Select
    #[inline(always)]
    pub fn cfitss(&mut self) -> CfitssW<CfdcfccSpec> {
        CfitssW::new(self, 10)
    }
    ///Bit 11 - Common FIFO Interval Timer Resolution
    #[inline(always)]
    pub fn cfitr(&mut self) -> CfitrW<CfdcfccSpec> {
        CfitrW::new(self, 11)
    }
    ///Bit 12 - Common FIFO Interrupt Mode
    #[inline(always)]
    pub fn cfim(&mut self) -> CfimW<CfdcfccSpec> {
        CfimW::new(self, 12)
    }
    ///Bits 13:15 - Common FIFO Interrupt Generation Counter Value
    #[inline(always)]
    pub fn cfigcv(&mut self) -> CfigcvW<CfdcfccSpec> {
        CfigcvW::new(self, 13)
    }
    ///Bits 16:20 - Common FIFO TX Message Buffer Link
    #[inline(always)]
    pub fn cftml(&mut self) -> CftmlW<CfdcfccSpec> {
        CftmlW::new(self, 16)
    }
    ///Bits 21:23 - Common FIFO Depth Configuration
    #[inline(always)]
    pub fn cfdc(&mut self) -> CfdcW<CfdcfccSpec> {
        CfdcW::new(self, 21)
    }
    ///Bits 24:31 - Common FIFO Interval Transmission Time
    #[inline(always)]
    pub fn cfitt(&mut self) -> CfittW<CfdcfccSpec> {
        CfittW::new(self, 24)
    }
}
/**Common FIFO Configuration/Control Registers %s

You can [`read`](crate::Reg::read) this register and get [`cfdcfcc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcfcc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdcfccSpec;
impl crate::RegisterSpec for CfdcfccSpec {
    type Ux = u32;
}
///`read()` method returns [`cfdcfcc::R`](R) reader structure
impl crate::Readable for CfdcfccSpec {}
///`write(|w| ..)` method takes [`cfdcfcc::W`](W) writer structure
impl crate::Writable for CfdcfccSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDCFCC%s to value 0
impl crate::Resettable for CfdcfccSpec {}
