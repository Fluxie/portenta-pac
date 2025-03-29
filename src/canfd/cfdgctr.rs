///Register `CFDGCTR` reader
pub type R = crate::R<CfdgctrSpec>;
///Register `CFDGCTR` writer
pub type W = crate::W<CfdgctrSpec>;
/**Global Mode Control

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gmdc {
    ///0: Global operation mode request
    _00 = 0,
    ///1: Global reset mode request
    _01 = 1,
    ///2: Global halt mode request
    _10 = 2,
    ///3: Keep current value
    _11 = 3,
}
impl From<Gmdc> for u8 {
    #[inline(always)]
    fn from(variant: Gmdc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gmdc {
    type Ux = u8;
}
impl crate::IsEnum for Gmdc {}
///Field `GMDC` reader - Global Mode Control
pub type GmdcR = crate::FieldReader<Gmdc>;
impl GmdcR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Gmdc {
        match self.bits {
            0 => Gmdc::_00,
            1 => Gmdc::_01,
            2 => Gmdc::_10,
            3 => Gmdc::_11,
            _ => unreachable!(),
        }
    }
    ///Global operation mode request
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Gmdc::_00
    }
    ///Global reset mode request
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Gmdc::_01
    }
    ///Global halt mode request
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Gmdc::_10
    }
    ///Keep current value
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Gmdc::_11
    }
}
///Field `GMDC` writer - Global Mode Control
pub type GmdcW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gmdc, crate::Safe>;
impl<'a, REG> GmdcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Global operation mode request
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Gmdc::_00)
    }
    ///Global reset mode request
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Gmdc::_01)
    }
    ///Global halt mode request
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Gmdc::_10)
    }
    ///Keep current value
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Gmdc::_11)
    }
}
/**Global Sleep Request

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gslpr {
    ///0: Global sleep request disabled
    _0 = 0,
    ///1: Global sleep request enabled
    _1 = 1,
}
impl From<Gslpr> for bool {
    #[inline(always)]
    fn from(variant: Gslpr) -> Self {
        variant as u8 != 0
    }
}
///Field `GSLPR` reader - Global Sleep Request
pub type GslprR = crate::BitReader<Gslpr>;
impl GslprR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Gslpr {
        match self.bits {
            false => Gslpr::_0,
            true => Gslpr::_1,
        }
    }
    ///Global sleep request disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Gslpr::_0
    }
    ///Global sleep request enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Gslpr::_1
    }
}
///Field `GSLPR` writer - Global Sleep Request
pub type GslprW<'a, REG> = crate::BitWriter<'a, REG, Gslpr>;
impl<'a, REG> GslprW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Global sleep request disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Gslpr::_0)
    }
    ///Global sleep request enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Gslpr::_1)
    }
}
/**DLC Check Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Deie {
    ///0: DLC check interrupt disabled
    _0 = 0,
    ///1: DLC check interrupt enabled
    _1 = 1,
}
impl From<Deie> for bool {
    #[inline(always)]
    fn from(variant: Deie) -> Self {
        variant as u8 != 0
    }
}
///Field `DEIE` reader - DLC Check Interrupt Enable
pub type DeieR = crate::BitReader<Deie>;
impl DeieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Deie {
        match self.bits {
            false => Deie::_0,
            true => Deie::_1,
        }
    }
    ///DLC check interrupt disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Deie::_0
    }
    ///DLC check interrupt enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Deie::_1
    }
}
///Field `DEIE` writer - DLC Check Interrupt Enable
pub type DeieW<'a, REG> = crate::BitWriter<'a, REG, Deie>;
impl<'a, REG> DeieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DLC check interrupt disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Deie::_0)
    }
    ///DLC check interrupt enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Deie::_1)
    }
}
/**Message Lost Error Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Meie {
    ///0: Message lost error interrupt disabled
    _0 = 0,
    ///1: Message lost error interrupt enabled
    _1 = 1,
}
impl From<Meie> for bool {
    #[inline(always)]
    fn from(variant: Meie) -> Self {
        variant as u8 != 0
    }
}
///Field `MEIE` reader - Message Lost Error Interrupt Enable
pub type MeieR = crate::BitReader<Meie>;
impl MeieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Meie {
        match self.bits {
            false => Meie::_0,
            true => Meie::_1,
        }
    }
    ///Message lost error interrupt disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Meie::_0
    }
    ///Message lost error interrupt enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Meie::_1
    }
}
///Field `MEIE` writer - Message Lost Error Interrupt Enable
pub type MeieW<'a, REG> = crate::BitWriter<'a, REG, Meie>;
impl<'a, REG> MeieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Message lost error interrupt disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Meie::_0)
    }
    ///Message lost error interrupt enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Meie::_1)
    }
}
/**TX History List Entry Lost Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Thleie {
    ///0: TX history list entry lost interrupt disabled
    _0 = 0,
    ///1: TX history list entry lost interrupt enabled
    _1 = 1,
}
impl From<Thleie> for bool {
    #[inline(always)]
    fn from(variant: Thleie) -> Self {
        variant as u8 != 0
    }
}
///Field `THLEIE` reader - TX History List Entry Lost Interrupt Enable
pub type ThleieR = crate::BitReader<Thleie>;
impl ThleieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Thleie {
        match self.bits {
            false => Thleie::_0,
            true => Thleie::_1,
        }
    }
    ///TX history list entry lost interrupt disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Thleie::_0
    }
    ///TX history list entry lost interrupt enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Thleie::_1
    }
}
///Field `THLEIE` writer - TX History List Entry Lost Interrupt Enable
pub type ThleieW<'a, REG> = crate::BitWriter<'a, REG, Thleie>;
impl<'a, REG> ThleieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///TX history list entry lost interrupt disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Thleie::_0)
    }
    ///TX history list entry lost interrupt enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Thleie::_1)
    }
}
/**CANFD Message Payload Overflow Flag Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpofie {
    ///0: CANFD message payload overflow flag interrupt disabled
    _0 = 0,
    ///1: CANFD message payload overflow flag interrupt enabled
    _1 = 1,
}
impl From<Cmpofie> for bool {
    #[inline(always)]
    fn from(variant: Cmpofie) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPOFIE` reader - CANFD Message Payload Overflow Flag Interrupt Enable
pub type CmpofieR = crate::BitReader<Cmpofie>;
impl CmpofieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmpofie {
        match self.bits {
            false => Cmpofie::_0,
            true => Cmpofie::_1,
        }
    }
    ///CANFD message payload overflow flag interrupt disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpofie::_0
    }
    ///CANFD message payload overflow flag interrupt enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpofie::_1
    }
}
///Field `CMPOFIE` writer - CANFD Message Payload Overflow Flag Interrupt Enable
pub type CmpofieW<'a, REG> = crate::BitWriter<'a, REG, Cmpofie>;
impl<'a, REG> CmpofieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CANFD message payload overflow flag interrupt disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpofie::_0)
    }
    ///CANFD message payload overflow flag interrupt enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpofie::_1)
    }
}
/**TXQ Message Lost Error Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Qmeie {
    ///0: TXQ message lost error interrupt disabled
    _0 = 0,
    ///1: TXQ message lost error interrupt enabled
    _1 = 1,
}
impl From<Qmeie> for bool {
    #[inline(always)]
    fn from(variant: Qmeie) -> Self {
        variant as u8 != 0
    }
}
///Field `QMEIE` reader - TXQ Message Lost Error Interrupt Enable
pub type QmeieR = crate::BitReader<Qmeie>;
impl QmeieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Qmeie {
        match self.bits {
            false => Qmeie::_0,
            true => Qmeie::_1,
        }
    }
    ///TXQ message lost error interrupt disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Qmeie::_0
    }
    ///TXQ message lost error interrupt enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Qmeie::_1
    }
}
///Field `QMEIE` writer - TXQ Message Lost Error Interrupt Enable
pub type QmeieW<'a, REG> = crate::BitWriter<'a, REG, Qmeie>;
impl<'a, REG> QmeieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///TXQ message lost error interrupt disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Qmeie::_0)
    }
    ///TXQ message lost error interrupt enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Qmeie::_1)
    }
}
/**GW FIFO Message Overwrite Error Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Moweie {
    ///0: GW FIFO message overwrite error interrupt disabled
    _0 = 0,
    ///1: GW FIFO message overwrite error interrupt enabled
    _1 = 1,
}
impl From<Moweie> for bool {
    #[inline(always)]
    fn from(variant: Moweie) -> Self {
        variant as u8 != 0
    }
}
///Field `MOWEIE` reader - GW FIFO Message Overwrite Error Interrupt Enable
pub type MoweieR = crate::BitReader<Moweie>;
impl MoweieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Moweie {
        match self.bits {
            false => Moweie::_0,
            true => Moweie::_1,
        }
    }
    ///GW FIFO message overwrite error interrupt disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Moweie::_0
    }
    ///GW FIFO message overwrite error interrupt enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Moweie::_1
    }
}
///Field `MOWEIE` writer - GW FIFO Message Overwrite Error Interrupt Enable
pub type MoweieW<'a, REG> = crate::BitWriter<'a, REG, Moweie>;
impl<'a, REG> MoweieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///GW FIFO message overwrite error interrupt disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Moweie::_0)
    }
    ///GW FIFO message overwrite error interrupt enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Moweie::_1)
    }
}
/**Timestamp Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tsrst {
    ///0: Timestamp not reset
    _0 = 0,
    ///1: Timestamp reset
    _1 = 1,
}
impl From<Tsrst> for bool {
    #[inline(always)]
    fn from(variant: Tsrst) -> Self {
        variant as u8 != 0
    }
}
///Field `TSRST` reader - Timestamp Reset
pub type TsrstR = crate::BitReader<Tsrst>;
impl TsrstR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tsrst {
        match self.bits {
            false => Tsrst::_0,
            true => Tsrst::_1,
        }
    }
    ///Timestamp not reset
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tsrst::_0
    }
    ///Timestamp reset
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tsrst::_1
    }
}
///Field `TSRST` writer - Timestamp Reset
pub type TsrstW<'a, REG> = crate::BitWriter<'a, REG, Tsrst>;
impl<'a, REG> TsrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Timestamp not reset
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tsrst::_0)
    }
    ///Timestamp reset
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tsrst::_1)
    }
}
impl R {
    ///Bits 0:1 - Global Mode Control
    #[inline(always)]
    pub fn gmdc(&self) -> GmdcR {
        GmdcR::new((self.bits & 3) as u8)
    }
    ///Bit 2 - Global Sleep Request
    #[inline(always)]
    pub fn gslpr(&self) -> GslprR {
        GslprR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 8 - DLC Check Interrupt Enable
    #[inline(always)]
    pub fn deie(&self) -> DeieR {
        DeieR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Message Lost Error Interrupt Enable
    #[inline(always)]
    pub fn meie(&self) -> MeieR {
        MeieR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - TX History List Entry Lost Interrupt Enable
    #[inline(always)]
    pub fn thleie(&self) -> ThleieR {
        ThleieR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - CANFD Message Payload Overflow Flag Interrupt Enable
    #[inline(always)]
    pub fn cmpofie(&self) -> CmpofieR {
        CmpofieR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 14 - TXQ Message Lost Error Interrupt Enable
    #[inline(always)]
    pub fn qmeie(&self) -> QmeieR {
        QmeieR::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - GW FIFO Message Overwrite Error Interrupt Enable
    #[inline(always)]
    pub fn moweie(&self) -> MoweieR {
        MoweieR::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Timestamp Reset
    #[inline(always)]
    pub fn tsrst(&self) -> TsrstR {
        TsrstR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDGCTR")
            .field("gmdc", &self.gmdc())
            .field("gslpr", &self.gslpr())
            .field("deie", &self.deie())
            .field("meie", &self.meie())
            .field("thleie", &self.thleie())
            .field("cmpofie", &self.cmpofie())
            .field("qmeie", &self.qmeie())
            .field("moweie", &self.moweie())
            .field("tsrst", &self.tsrst())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Global Mode Control
    #[inline(always)]
    pub fn gmdc(&mut self) -> GmdcW<CfdgctrSpec> {
        GmdcW::new(self, 0)
    }
    ///Bit 2 - Global Sleep Request
    #[inline(always)]
    pub fn gslpr(&mut self) -> GslprW<CfdgctrSpec> {
        GslprW::new(self, 2)
    }
    ///Bit 8 - DLC Check Interrupt Enable
    #[inline(always)]
    pub fn deie(&mut self) -> DeieW<CfdgctrSpec> {
        DeieW::new(self, 8)
    }
    ///Bit 9 - Message Lost Error Interrupt Enable
    #[inline(always)]
    pub fn meie(&mut self) -> MeieW<CfdgctrSpec> {
        MeieW::new(self, 9)
    }
    ///Bit 10 - TX History List Entry Lost Interrupt Enable
    #[inline(always)]
    pub fn thleie(&mut self) -> ThleieW<CfdgctrSpec> {
        ThleieW::new(self, 10)
    }
    ///Bit 11 - CANFD Message Payload Overflow Flag Interrupt Enable
    #[inline(always)]
    pub fn cmpofie(&mut self) -> CmpofieW<CfdgctrSpec> {
        CmpofieW::new(self, 11)
    }
    ///Bit 14 - TXQ Message Lost Error Interrupt Enable
    #[inline(always)]
    pub fn qmeie(&mut self) -> QmeieW<CfdgctrSpec> {
        QmeieW::new(self, 14)
    }
    ///Bit 15 - GW FIFO Message Overwrite Error Interrupt Enable
    #[inline(always)]
    pub fn moweie(&mut self) -> MoweieW<CfdgctrSpec> {
        MoweieW::new(self, 15)
    }
    ///Bit 16 - Timestamp Reset
    #[inline(always)]
    pub fn tsrst(&mut self) -> TsrstW<CfdgctrSpec> {
        TsrstW::new(self, 16)
    }
}
/**Global Control Register

You can [`read`](crate::Reg::read) this register and get [`cfdgctr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdgctr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdgctrSpec;
impl crate::RegisterSpec for CfdgctrSpec {
    type Ux = u32;
}
///`read()` method returns [`cfdgctr::R`](R) reader structure
impl crate::Readable for CfdgctrSpec {}
///`write(|w| ..)` method takes [`cfdgctr::W`](W) writer structure
impl crate::Writable for CfdgctrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDGCTR to value 0x05
impl crate::Resettable for CfdgctrSpec {
    const RESET_VALUE: u32 = 0x05;
}
