///Register `CFDRFCC%s` reader
pub type R = crate::R<CfdrfccSpec>;
///Register `CFDRFCC%s` writer
pub type W = crate::W<CfdrfccSpec>;
/**RX FIFO Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfe {
    ///0: FIFO disabled
    _0 = 0,
    ///1: FIFO enabled
    _1 = 1,
}
impl From<Rfe> for bool {
    #[inline(always)]
    fn from(variant: Rfe) -> Self {
        variant as u8 != 0
    }
}
///Field `RFE` reader - RX FIFO Enable
pub type RfeR = crate::BitReader<Rfe>;
impl RfeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rfe {
        match self.bits {
            false => Rfe::_0,
            true => Rfe::_1,
        }
    }
    ///FIFO disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rfe::_0
    }
    ///FIFO enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rfe::_1
    }
}
///Field `RFE` writer - RX FIFO Enable
pub type RfeW<'a, REG> = crate::BitWriter<'a, REG, Rfe>;
impl<'a, REG> RfeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///FIFO disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rfe::_0)
    }
    ///FIFO enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rfe::_1)
    }
}
/**RX FIFO Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfie {
    ///0: FIFO interrupt generation disabled
    _0 = 0,
    ///1: FIFO interrupt generation enabled
    _1 = 1,
}
impl From<Rfie> for bool {
    #[inline(always)]
    fn from(variant: Rfie) -> Self {
        variant as u8 != 0
    }
}
///Field `RFIE` reader - RX FIFO Interrupt Enable
pub type RfieR = crate::BitReader<Rfie>;
impl RfieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rfie {
        match self.bits {
            false => Rfie::_0,
            true => Rfie::_1,
        }
    }
    ///FIFO interrupt generation disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rfie::_0
    }
    ///FIFO interrupt generation enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rfie::_1
    }
}
///Field `RFIE` writer - RX FIFO Interrupt Enable
pub type RfieW<'a, REG> = crate::BitWriter<'a, REG, Rfie>;
impl<'a, REG> RfieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///FIFO interrupt generation disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rfie::_0)
    }
    ///FIFO interrupt generation enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rfie::_1)
    }
}
/**Rx FIFO Payload Data Size Configuration

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rfpls {
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
impl From<Rfpls> for u8 {
    #[inline(always)]
    fn from(variant: Rfpls) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rfpls {
    type Ux = u8;
}
impl crate::IsEnum for Rfpls {}
///Field `RFPLS` reader - Rx FIFO Payload Data Size Configuration
pub type RfplsR = crate::FieldReader<Rfpls>;
impl RfplsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rfpls {
        match self.bits {
            0 => Rfpls::_000,
            1 => Rfpls::_001,
            2 => Rfpls::_010,
            3 => Rfpls::_011,
            4 => Rfpls::_100,
            5 => Rfpls::_101,
            6 => Rfpls::_110,
            7 => Rfpls::_111,
            _ => unreachable!(),
        }
    }
    ///8 bytes
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Rfpls::_000
    }
    ///12 bytes
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Rfpls::_001
    }
    ///16 bytes
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Rfpls::_010
    }
    ///20 bytes
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Rfpls::_011
    }
    ///24 bytes
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Rfpls::_100
    }
    ///32 bytes
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Rfpls::_101
    }
    ///48 bytes
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Rfpls::_110
    }
    ///64 bytes
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == Rfpls::_111
    }
}
///Field `RFPLS` writer - Rx FIFO Payload Data Size Configuration
pub type RfplsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Rfpls, crate::Safe>;
impl<'a, REG> RfplsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///8 bytes
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Rfpls::_000)
    }
    ///12 bytes
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Rfpls::_001)
    }
    ///16 bytes
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Rfpls::_010)
    }
    ///20 bytes
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Rfpls::_011)
    }
    ///24 bytes
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Rfpls::_100)
    }
    ///32 bytes
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Rfpls::_101)
    }
    ///48 bytes
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Rfpls::_110)
    }
    ///64 bytes
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(Rfpls::_111)
    }
}
/**RX FIFO Depth Configuration

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rfdc {
    ///0: FIFO Depth = 0 message
    _000 = 0,
    ///1: FIFO Depth = 4 messages
    _001 = 1,
    ///2: FIFO Depth = 8 messages
    _010 = 2,
    ///3: FIFO Depth = 16 messages
    _011 = 3,
    ///4: FIFO Depth = 32 essages
    _100 = 4,
    ///5: FIFO Depth = 48 messages
    _101 = 5,
    ///6: FIFO Depth = 64 messages
    _110 = 6,
    ///7: FIFO Depth = 128 messages
    _111 = 7,
}
impl From<Rfdc> for u8 {
    #[inline(always)]
    fn from(variant: Rfdc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rfdc {
    type Ux = u8;
}
impl crate::IsEnum for Rfdc {}
///Field `RFDC` reader - RX FIFO Depth Configuration
pub type RfdcR = crate::FieldReader<Rfdc>;
impl RfdcR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rfdc {
        match self.bits {
            0 => Rfdc::_000,
            1 => Rfdc::_001,
            2 => Rfdc::_010,
            3 => Rfdc::_011,
            4 => Rfdc::_100,
            5 => Rfdc::_101,
            6 => Rfdc::_110,
            7 => Rfdc::_111,
            _ => unreachable!(),
        }
    }
    ///FIFO Depth = 0 message
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Rfdc::_000
    }
    ///FIFO Depth = 4 messages
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Rfdc::_001
    }
    ///FIFO Depth = 8 messages
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Rfdc::_010
    }
    ///FIFO Depth = 16 messages
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Rfdc::_011
    }
    ///FIFO Depth = 32 essages
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Rfdc::_100
    }
    ///FIFO Depth = 48 messages
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Rfdc::_101
    }
    ///FIFO Depth = 64 messages
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Rfdc::_110
    }
    ///FIFO Depth = 128 messages
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == Rfdc::_111
    }
}
///Field `RFDC` writer - RX FIFO Depth Configuration
pub type RfdcW<'a, REG> = crate::FieldWriter<'a, REG, 3, Rfdc, crate::Safe>;
impl<'a, REG> RfdcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///FIFO Depth = 0 message
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Rfdc::_000)
    }
    ///FIFO Depth = 4 messages
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Rfdc::_001)
    }
    ///FIFO Depth = 8 messages
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Rfdc::_010)
    }
    ///FIFO Depth = 16 messages
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Rfdc::_011)
    }
    ///FIFO Depth = 32 essages
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Rfdc::_100)
    }
    ///FIFO Depth = 48 messages
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Rfdc::_101)
    }
    ///FIFO Depth = 64 messages
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Rfdc::_110)
    }
    ///FIFO Depth = 128 messages
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(Rfdc::_111)
    }
}
/**RX FIFO Interrupt Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfim {
    ///0: Interrupt generated when RX FIFO counter reaches RFIGCV value from values smaller than RFIGCV
    _0 = 0,
    ///1: Interrupt generated at the end of every received message storage
    _1 = 1,
}
impl From<Rfim> for bool {
    #[inline(always)]
    fn from(variant: Rfim) -> Self {
        variant as u8 != 0
    }
}
///Field `RFIM` reader - RX FIFO Interrupt Mode
pub type RfimR = crate::BitReader<Rfim>;
impl RfimR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rfim {
        match self.bits {
            false => Rfim::_0,
            true => Rfim::_1,
        }
    }
    ///Interrupt generated when RX FIFO counter reaches RFIGCV value from values smaller than RFIGCV
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rfim::_0
    }
    ///Interrupt generated at the end of every received message storage
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rfim::_1
    }
}
///Field `RFIM` writer - RX FIFO Interrupt Mode
pub type RfimW<'a, REG> = crate::BitWriter<'a, REG, Rfim>;
impl<'a, REG> RfimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt generated when RX FIFO counter reaches RFIGCV value from values smaller than RFIGCV
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rfim::_0)
    }
    ///Interrupt generated at the end of every received message storage
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rfim::_1)
    }
}
/**RX FIFO Interrupt Generation Counter Value

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rfigcv {
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
impl From<Rfigcv> for u8 {
    #[inline(always)]
    fn from(variant: Rfigcv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rfigcv {
    type Ux = u8;
}
impl crate::IsEnum for Rfigcv {}
///Field `RFIGCV` reader - RX FIFO Interrupt Generation Counter Value
pub type RfigcvR = crate::FieldReader<Rfigcv>;
impl RfigcvR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rfigcv {
        match self.bits {
            0 => Rfigcv::_000,
            1 => Rfigcv::_001,
            2 => Rfigcv::_010,
            3 => Rfigcv::_011,
            4 => Rfigcv::_100,
            5 => Rfigcv::_101,
            6 => Rfigcv::_110,
            7 => Rfigcv::_111,
            _ => unreachable!(),
        }
    }
    ///Interrupt generated when FIFO is 1/8th full
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Rfigcv::_000
    }
    ///Interrupt generated when FIFO is 1/4th full
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Rfigcv::_001
    }
    ///Interrupt generated when FIFO is 3/8th full
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Rfigcv::_010
    }
    ///Interrupt generated when FIFO is 1/2 full
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Rfigcv::_011
    }
    ///Interrupt generated when FIFO is 5/8th full
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Rfigcv::_100
    }
    ///Interrupt generated when FIFO is 3/4th full
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Rfigcv::_101
    }
    ///Interrupt generated when FIFO is 7/8th full
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Rfigcv::_110
    }
    ///Interrupt generated when FIFO is full
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == Rfigcv::_111
    }
}
///Field `RFIGCV` writer - RX FIFO Interrupt Generation Counter Value
pub type RfigcvW<'a, REG> = crate::FieldWriter<'a, REG, 3, Rfigcv, crate::Safe>;
impl<'a, REG> RfigcvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Interrupt generated when FIFO is 1/8th full
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Rfigcv::_000)
    }
    ///Interrupt generated when FIFO is 1/4th full
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Rfigcv::_001)
    }
    ///Interrupt generated when FIFO is 3/8th full
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Rfigcv::_010)
    }
    ///Interrupt generated when FIFO is 1/2 full
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Rfigcv::_011)
    }
    ///Interrupt generated when FIFO is 5/8th full
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Rfigcv::_100)
    }
    ///Interrupt generated when FIFO is 3/4th full
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Rfigcv::_101)
    }
    ///Interrupt generated when FIFO is 7/8th full
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Rfigcv::_110)
    }
    ///Interrupt generated when FIFO is full
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(Rfigcv::_111)
    }
}
/**RX FIFO Full Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rffie {
    ///0: FIFO interrupt generation disabled
    _0 = 0,
    ///1: FIFO interrupt generation enabled
    _1 = 1,
}
impl From<Rffie> for bool {
    #[inline(always)]
    fn from(variant: Rffie) -> Self {
        variant as u8 != 0
    }
}
///Field `RFFIE` reader - RX FIFO Full Interrupt Enable
pub type RffieR = crate::BitReader<Rffie>;
impl RffieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rffie {
        match self.bits {
            false => Rffie::_0,
            true => Rffie::_1,
        }
    }
    ///FIFO interrupt generation disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rffie::_0
    }
    ///FIFO interrupt generation enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rffie::_1
    }
}
///Field `RFFIE` writer - RX FIFO Full Interrupt Enable
pub type RffieW<'a, REG> = crate::BitWriter<'a, REG, Rffie>;
impl<'a, REG> RffieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///FIFO interrupt generation disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rffie::_0)
    }
    ///FIFO interrupt generation enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rffie::_1)
    }
}
impl R {
    ///Bit 0 - RX FIFO Enable
    #[inline(always)]
    pub fn rfe(&self) -> RfeR {
        RfeR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - RX FIFO Interrupt Enable
    #[inline(always)]
    pub fn rfie(&self) -> RfieR {
        RfieR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 4:6 - Rx FIFO Payload Data Size Configuration
    #[inline(always)]
    pub fn rfpls(&self) -> RfplsR {
        RfplsR::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 8:10 - RX FIFO Depth Configuration
    #[inline(always)]
    pub fn rfdc(&self) -> RfdcR {
        RfdcR::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bit 12 - RX FIFO Interrupt Mode
    #[inline(always)]
    pub fn rfim(&self) -> RfimR {
        RfimR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 13:15 - RX FIFO Interrupt Generation Counter Value
    #[inline(always)]
    pub fn rfigcv(&self) -> RfigcvR {
        RfigcvR::new(((self.bits >> 13) & 7) as u8)
    }
    ///Bit 16 - RX FIFO Full Interrupt Enable
    #[inline(always)]
    pub fn rffie(&self) -> RffieR {
        RffieR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDRFCC")
            .field("rfe", &self.rfe())
            .field("rfie", &self.rfie())
            .field("rfpls", &self.rfpls())
            .field("rfdc", &self.rfdc())
            .field("rfim", &self.rfim())
            .field("rfigcv", &self.rfigcv())
            .field("rffie", &self.rffie())
            .finish()
    }
}
impl W {
    ///Bit 0 - RX FIFO Enable
    #[inline(always)]
    pub fn rfe(&mut self) -> RfeW<CfdrfccSpec> {
        RfeW::new(self, 0)
    }
    ///Bit 1 - RX FIFO Interrupt Enable
    #[inline(always)]
    pub fn rfie(&mut self) -> RfieW<CfdrfccSpec> {
        RfieW::new(self, 1)
    }
    ///Bits 4:6 - Rx FIFO Payload Data Size Configuration
    #[inline(always)]
    pub fn rfpls(&mut self) -> RfplsW<CfdrfccSpec> {
        RfplsW::new(self, 4)
    }
    ///Bits 8:10 - RX FIFO Depth Configuration
    #[inline(always)]
    pub fn rfdc(&mut self) -> RfdcW<CfdrfccSpec> {
        RfdcW::new(self, 8)
    }
    ///Bit 12 - RX FIFO Interrupt Mode
    #[inline(always)]
    pub fn rfim(&mut self) -> RfimW<CfdrfccSpec> {
        RfimW::new(self, 12)
    }
    ///Bits 13:15 - RX FIFO Interrupt Generation Counter Value
    #[inline(always)]
    pub fn rfigcv(&mut self) -> RfigcvW<CfdrfccSpec> {
        RfigcvW::new(self, 13)
    }
    ///Bit 16 - RX FIFO Full Interrupt Enable
    #[inline(always)]
    pub fn rffie(&mut self) -> RffieW<CfdrfccSpec> {
        RffieW::new(self, 16)
    }
}
/**RX FIFO Configuration/Control Registers %s

You can [`read`](crate::Reg::read) this register and get [`cfdrfcc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdrfcc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdrfccSpec;
impl crate::RegisterSpec for CfdrfccSpec {
    type Ux = u32;
}
///`read()` method returns [`cfdrfcc::R`](R) reader structure
impl crate::Readable for CfdrfccSpec {}
///`write(|w| ..)` method takes [`cfdrfcc::W`](W) writer structure
impl crate::Writable for CfdrfccSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDRFCC%s to value 0
impl crate::Resettable for CfdrfccSpec {}
