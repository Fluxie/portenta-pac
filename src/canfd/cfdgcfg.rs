///Register `CFDGCFG` reader
pub type R = crate::R<CfdgcfgSpec>;
///Register `CFDGCFG` writer
pub type W = crate::W<CfdgcfgSpec>;
/**Transmission Priority

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tpri {
    ///0: ID priority
    _0 = 0,
    ///1: Message buffer number priority
    _1 = 1,
}
impl From<Tpri> for bool {
    #[inline(always)]
    fn from(variant: Tpri) -> Self {
        variant as u8 != 0
    }
}
///Field `TPRI` reader - Transmission Priority
pub type TpriR = crate::BitReader<Tpri>;
impl TpriR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tpri {
        match self.bits {
            false => Tpri::_0,
            true => Tpri::_1,
        }
    }
    ///ID priority
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tpri::_0
    }
    ///Message buffer number priority
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tpri::_1
    }
}
///Field `TPRI` writer - Transmission Priority
pub type TpriW<'a, REG> = crate::BitWriter<'a, REG, Tpri>;
impl<'a, REG> TpriW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ID priority
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tpri::_0)
    }
    ///Message buffer number priority
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tpri::_1)
    }
}
/**DLC Check Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dce {
    ///0: DLC check disabled
    _0 = 0,
    ///1: DLC check enabled
    _1 = 1,
}
impl From<Dce> for bool {
    #[inline(always)]
    fn from(variant: Dce) -> Self {
        variant as u8 != 0
    }
}
///Field `DCE` reader - DLC Check Enable
pub type DceR = crate::BitReader<Dce>;
impl DceR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dce {
        match self.bits {
            false => Dce::_0,
            true => Dce::_1,
        }
    }
    ///DLC check disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dce::_0
    }
    ///DLC check enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dce::_1
    }
}
///Field `DCE` writer - DLC Check Enable
pub type DceW<'a, REG> = crate::BitWriter<'a, REG, Dce>;
impl<'a, REG> DceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DLC check disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dce::_0)
    }
    ///DLC check enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dce::_1)
    }
}
/**DLC Replacement Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dre {
    ///0: DLC replacement disabled
    _0 = 0,
    ///1: DLC replacement enabled
    _1 = 1,
}
impl From<Dre> for bool {
    #[inline(always)]
    fn from(variant: Dre) -> Self {
        variant as u8 != 0
    }
}
///Field `DRE` reader - DLC Replacement Enable
pub type DreR = crate::BitReader<Dre>;
impl DreR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dre {
        match self.bits {
            false => Dre::_0,
            true => Dre::_1,
        }
    }
    ///DLC replacement disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dre::_0
    }
    ///DLC replacement enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dre::_1
    }
}
///Field `DRE` writer - DLC Replacement Enable
pub type DreW<'a, REG> = crate::BitWriter<'a, REG, Dre>;
impl<'a, REG> DreW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DLC replacement disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dre::_0)
    }
    ///DLC replacement enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dre::_1)
    }
}
/**Mirror Mode Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mme {
    ///0: Mirror mode disabled
    _0 = 0,
    ///1: Mirror mode enabled
    _1 = 1,
}
impl From<Mme> for bool {
    #[inline(always)]
    fn from(variant: Mme) -> Self {
        variant as u8 != 0
    }
}
///Field `MME` reader - Mirror Mode Enable
pub type MmeR = crate::BitReader<Mme>;
impl MmeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mme {
        match self.bits {
            false => Mme::_0,
            true => Mme::_1,
        }
    }
    ///Mirror mode disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mme::_0
    }
    ///Mirror mode enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mme::_1
    }
}
///Field `MME` writer - Mirror Mode Enable
pub type MmeW<'a, REG> = crate::BitWriter<'a, REG, Mme>;
impl<'a, REG> MmeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Mirror mode disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mme::_0)
    }
    ///Mirror mode enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mme::_1)
    }
}
/**Data Link Controller Clock Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dcs {
    ///0: Internal clean clock
    _0 = 0,
    ///1: External clock source connected to CANMCLK pin
    _1 = 1,
}
impl From<Dcs> for bool {
    #[inline(always)]
    fn from(variant: Dcs) -> Self {
        variant as u8 != 0
    }
}
///Field `DCS` reader - Data Link Controller Clock Select
pub type DcsR = crate::BitReader<Dcs>;
impl DcsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dcs {
        match self.bits {
            false => Dcs::_0,
            true => Dcs::_1,
        }
    }
    ///Internal clean clock
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dcs::_0
    }
    ///External clock source connected to CANMCLK pin
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dcs::_1
    }
}
///Field `DCS` writer - Data Link Controller Clock Select
pub type DcsW<'a, REG> = crate::BitWriter<'a, REG, Dcs>;
impl<'a, REG> DcsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Internal clean clock
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dcs::_0)
    }
    ///External clock source connected to CANMCLK pin
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dcs::_1)
    }
}
/**CAN-FD Message Payload Overflow Configuration

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpoc {
    ///0: Message is rejected
    _0 = 0,
    ///1: Message payload is cut to fit to configured message size
    _1 = 1,
}
impl From<Cmpoc> for bool {
    #[inline(always)]
    fn from(variant: Cmpoc) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPOC` reader - CAN-FD Message Payload Overflow Configuration
pub type CmpocR = crate::BitReader<Cmpoc>;
impl CmpocR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmpoc {
        match self.bits {
            false => Cmpoc::_0,
            true => Cmpoc::_1,
        }
    }
    ///Message is rejected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpoc::_0
    }
    ///Message payload is cut to fit to configured message size
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpoc::_1
    }
}
///Field `CMPOC` writer - CAN-FD Message Payload Overflow Configuration
pub type CmpocW<'a, REG> = crate::BitWriter<'a, REG, Cmpoc>;
impl<'a, REG> CmpocW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Message is rejected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpoc::_0)
    }
    ///Message payload is cut to fit to configured message size
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpoc::_1)
    }
}
///Field `TSP` reader - Timestamp Prescaler
pub type TspR = crate::FieldReader;
///Field `TSP` writer - Timestamp Prescaler
pub type TspW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
/**Timestamp Source Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tsss {
    ///0: Source clock for timestamp counter is peripheral clock
    _0 = 0,
    ///1: Source clock for timestamp counter is bit time clock
    _1 = 1,
}
impl From<Tsss> for bool {
    #[inline(always)]
    fn from(variant: Tsss) -> Self {
        variant as u8 != 0
    }
}
///Field `TSSS` reader - Timestamp Source Select
pub type TsssR = crate::BitReader<Tsss>;
impl TsssR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tsss {
        match self.bits {
            false => Tsss::_0,
            true => Tsss::_1,
        }
    }
    ///Source clock for timestamp counter is peripheral clock
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tsss::_0
    }
    ///Source clock for timestamp counter is bit time clock
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tsss::_1
    }
}
///Field `TSSS` writer - Timestamp Source Select
pub type TsssW<'a, REG> = crate::BitWriter<'a, REG, Tsss>;
impl<'a, REG> TsssW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Source clock for timestamp counter is peripheral clock
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tsss::_0)
    }
    ///Source clock for timestamp counter is bit time clock
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tsss::_1)
    }
}
/**Timestamp Bit Time Channel Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tsbtcs {
    ///0: Select clock from channel 0
    _000 = 0,
    ///1: Select clock from channel 1
    _001 = 1,
    ///2: Setting prohibited
    Others = 2,
}
impl From<Tsbtcs> for u8 {
    #[inline(always)]
    fn from(variant: Tsbtcs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tsbtcs {
    type Ux = u8;
}
impl crate::IsEnum for Tsbtcs {}
///Field `TSBTCS` reader - Timestamp Bit Time Channel Select
pub type TsbtcsR = crate::FieldReader<Tsbtcs>;
impl TsbtcsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tsbtcs {
        match self.bits {
            0 => Tsbtcs::_000,
            1 => Tsbtcs::_001,
            _ => Tsbtcs::Others,
        }
    }
    ///Select clock from channel 0
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Tsbtcs::_000
    }
    ///Select clock from channel 1
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Tsbtcs::_001
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Tsbtcs::Others)
    }
}
///Field `TSBTCS` writer - Timestamp Bit Time Channel Select
pub type TsbtcsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Tsbtcs, crate::Safe>;
impl<'a, REG> TsbtcsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Select clock from channel 0
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Tsbtcs::_000)
    }
    ///Select clock from channel 1
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Tsbtcs::_001)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Tsbtcs::Others)
    }
}
///Field `ITRCP` reader - Interval Timer Reference Clock Prescaler
pub type ItrcpR = crate::FieldReader<u16>;
///Field `ITRCP` writer - Interval Timer Reference Clock Prescaler
pub type ItrcpW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bit 0 - Transmission Priority
    #[inline(always)]
    pub fn tpri(&self) -> TpriR {
        TpriR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DLC Check Enable
    #[inline(always)]
    pub fn dce(&self) -> DceR {
        DceR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - DLC Replacement Enable
    #[inline(always)]
    pub fn dre(&self) -> DreR {
        DreR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Mirror Mode Enable
    #[inline(always)]
    pub fn mme(&self) -> MmeR {
        MmeR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Data Link Controller Clock Select
    #[inline(always)]
    pub fn dcs(&self) -> DcsR {
        DcsR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - CAN-FD Message Payload Overflow Configuration
    #[inline(always)]
    pub fn cmpoc(&self) -> CmpocR {
        CmpocR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 8:11 - Timestamp Prescaler
    #[inline(always)]
    pub fn tsp(&self) -> TspR {
        TspR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 12 - Timestamp Source Select
    #[inline(always)]
    pub fn tsss(&self) -> TsssR {
        TsssR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 13:15 - Timestamp Bit Time Channel Select
    #[inline(always)]
    pub fn tsbtcs(&self) -> TsbtcsR {
        TsbtcsR::new(((self.bits >> 13) & 7) as u8)
    }
    ///Bits 16:31 - Interval Timer Reference Clock Prescaler
    #[inline(always)]
    pub fn itrcp(&self) -> ItrcpR {
        ItrcpR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDGCFG")
            .field("tpri", &self.tpri())
            .field("dce", &self.dce())
            .field("dre", &self.dre())
            .field("mme", &self.mme())
            .field("dcs", &self.dcs())
            .field("cmpoc", &self.cmpoc())
            .field("tsp", &self.tsp())
            .field("tsss", &self.tsss())
            .field("tsbtcs", &self.tsbtcs())
            .field("itrcp", &self.itrcp())
            .finish()
    }
}
impl W {
    ///Bit 0 - Transmission Priority
    #[inline(always)]
    pub fn tpri(&mut self) -> TpriW<CfdgcfgSpec> {
        TpriW::new(self, 0)
    }
    ///Bit 1 - DLC Check Enable
    #[inline(always)]
    pub fn dce(&mut self) -> DceW<CfdgcfgSpec> {
        DceW::new(self, 1)
    }
    ///Bit 2 - DLC Replacement Enable
    #[inline(always)]
    pub fn dre(&mut self) -> DreW<CfdgcfgSpec> {
        DreW::new(self, 2)
    }
    ///Bit 3 - Mirror Mode Enable
    #[inline(always)]
    pub fn mme(&mut self) -> MmeW<CfdgcfgSpec> {
        MmeW::new(self, 3)
    }
    ///Bit 4 - Data Link Controller Clock Select
    #[inline(always)]
    pub fn dcs(&mut self) -> DcsW<CfdgcfgSpec> {
        DcsW::new(self, 4)
    }
    ///Bit 5 - CAN-FD Message Payload Overflow Configuration
    #[inline(always)]
    pub fn cmpoc(&mut self) -> CmpocW<CfdgcfgSpec> {
        CmpocW::new(self, 5)
    }
    ///Bits 8:11 - Timestamp Prescaler
    #[inline(always)]
    pub fn tsp(&mut self) -> TspW<CfdgcfgSpec> {
        TspW::new(self, 8)
    }
    ///Bit 12 - Timestamp Source Select
    #[inline(always)]
    pub fn tsss(&mut self) -> TsssW<CfdgcfgSpec> {
        TsssW::new(self, 12)
    }
    ///Bits 13:15 - Timestamp Bit Time Channel Select
    #[inline(always)]
    pub fn tsbtcs(&mut self) -> TsbtcsW<CfdgcfgSpec> {
        TsbtcsW::new(self, 13)
    }
    ///Bits 16:31 - Interval Timer Reference Clock Prescaler
    #[inline(always)]
    pub fn itrcp(&mut self) -> ItrcpW<CfdgcfgSpec> {
        ItrcpW::new(self, 16)
    }
}
/**Global Configuration Register

You can [`read`](crate::Reg::read) this register and get [`cfdgcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdgcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdgcfgSpec;
impl crate::RegisterSpec for CfdgcfgSpec {
    type Ux = u32;
}
///`read()` method returns [`cfdgcfg::R`](R) reader structure
impl crate::Readable for CfdgcfgSpec {}
///`write(|w| ..)` method takes [`cfdgcfg::W`](W) writer structure
impl crate::Writable for CfdgcfgSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDGCFG to value 0
impl crate::Resettable for CfdgcfgSpec {}
