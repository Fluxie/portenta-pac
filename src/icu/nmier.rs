///Register `NMIER` reader
pub type R = crate::R<NmierSpec>;
///Register `NMIER` writer
pub type W = crate::W<NmierSpec>;
/**IWDT Underflow/Refresh Error Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iwdten {
    ///0: Disabled
    _0 = 0,
    ///1: Enabled.
    _1 = 1,
}
impl From<Iwdten> for bool {
    #[inline(always)]
    fn from(variant: Iwdten) -> Self {
        variant as u8 != 0
    }
}
///Field `IWDTEN` reader - IWDT Underflow/Refresh Error Interrupt Enable
pub type IwdtenR = crate::BitReader<Iwdten>;
impl IwdtenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Iwdten {
        match self.bits {
            false => Iwdten::_0,
            true => Iwdten::_1,
        }
    }
    ///Disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Iwdten::_0
    }
    ///Enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Iwdten::_1
    }
}
///Field `IWDTEN` writer - IWDT Underflow/Refresh Error Interrupt Enable
pub type IwdtenW<'a, REG> = crate::BitWriter<'a, REG, Iwdten>;
impl<'a, REG> IwdtenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Iwdten::_0)
    }
    ///Enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Iwdten::_1)
    }
}
/**WDT Underflow/Refresh Error Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wdten {
    ///0: Disabled
    _0 = 0,
    ///1: Enabled
    _1 = 1,
}
impl From<Wdten> for bool {
    #[inline(always)]
    fn from(variant: Wdten) -> Self {
        variant as u8 != 0
    }
}
///Field `WDTEN` reader - WDT Underflow/Refresh Error Interrupt Enable
pub type WdtenR = crate::BitReader<Wdten>;
impl WdtenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Wdten {
        match self.bits {
            false => Wdten::_0,
            true => Wdten::_1,
        }
    }
    ///Disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wdten::_0
    }
    ///Enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wdten::_1
    }
}
///Field `WDTEN` writer - WDT Underflow/Refresh Error Interrupt Enable
pub type WdtenW<'a, REG> = crate::BitWriter<'a, REG, Wdten>;
impl<'a, REG> WdtenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wdten::_0)
    }
    ///Enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wdten::_1)
    }
}
/**Voltage monitor 1 Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lvd1en {
    ///0: Disabled
    _0 = 0,
    ///1: Enabled
    _1 = 1,
}
impl From<Lvd1en> for bool {
    #[inline(always)]
    fn from(variant: Lvd1en) -> Self {
        variant as u8 != 0
    }
}
///Field `LVD1EN` reader - Voltage monitor 1 Interrupt Enable
pub type Lvd1enR = crate::BitReader<Lvd1en>;
impl Lvd1enR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Lvd1en {
        match self.bits {
            false => Lvd1en::_0,
            true => Lvd1en::_1,
        }
    }
    ///Disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lvd1en::_0
    }
    ///Enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lvd1en::_1
    }
}
///Field `LVD1EN` writer - Voltage monitor 1 Interrupt Enable
pub type Lvd1enW<'a, REG> = crate::BitWriter<'a, REG, Lvd1en>;
impl<'a, REG> Lvd1enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd1en::_0)
    }
    ///Enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd1en::_1)
    }
}
/**Voltage monitor 2 Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lvd2en {
    ///0: Disabled
    _0 = 0,
    ///1: Enabled
    _1 = 1,
}
impl From<Lvd2en> for bool {
    #[inline(always)]
    fn from(variant: Lvd2en) -> Self {
        variant as u8 != 0
    }
}
///Field `LVD2EN` reader - Voltage monitor 2 Interrupt Enable
pub type Lvd2enR = crate::BitReader<Lvd2en>;
impl Lvd2enR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Lvd2en {
        match self.bits {
            false => Lvd2en::_0,
            true => Lvd2en::_1,
        }
    }
    ///Disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lvd2en::_0
    }
    ///Enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lvd2en::_1
    }
}
///Field `LVD2EN` writer - Voltage monitor 2 Interrupt Enable
pub type Lvd2enW<'a, REG> = crate::BitWriter<'a, REG, Lvd2en>;
impl<'a, REG> Lvd2enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd2en::_0)
    }
    ///Enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd2en::_1)
    }
}
/**Main Clock Oscillation Stop Detection Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Osten {
    ///0: Disabled
    _0 = 0,
    ///1: Enabled
    _1 = 1,
}
impl From<Osten> for bool {
    #[inline(always)]
    fn from(variant: Osten) -> Self {
        variant as u8 != 0
    }
}
///Field `OSTEN` reader - Main Clock Oscillation Stop Detection Interrupt Enable
pub type OstenR = crate::BitReader<Osten>;
impl OstenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Osten {
        match self.bits {
            false => Osten::_0,
            true => Osten::_1,
        }
    }
    ///Disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Osten::_0
    }
    ///Enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Osten::_1
    }
}
///Field `OSTEN` writer - Main Clock Oscillation Stop Detection Interrupt Enable
pub type OstenW<'a, REG> = crate::BitWriter<'a, REG, Osten>;
impl<'a, REG> OstenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Osten::_0)
    }
    ///Enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Osten::_1)
    }
}
/**NMI Pin Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nmien {
    ///0: Disabled
    _0 = 0,
    ///1: Enabled
    _1 = 1,
}
impl From<Nmien> for bool {
    #[inline(always)]
    fn from(variant: Nmien) -> Self {
        variant as u8 != 0
    }
}
///Field `NMIEN` reader - NMI Pin Interrupt Enable
pub type NmienR = crate::BitReader<Nmien>;
impl NmienR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Nmien {
        match self.bits {
            false => Nmien::_0,
            true => Nmien::_1,
        }
    }
    ///Disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Nmien::_0
    }
    ///Enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Nmien::_1
    }
}
///Field `NMIEN` writer - NMI Pin Interrupt Enable
pub type NmienW<'a, REG> = crate::BitWriter<'a, REG, Nmien>;
impl<'a, REG> NmienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Nmien::_0)
    }
    ///Enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Nmien::_1)
    }
}
/**SRAM Parity Error Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rpeen {
    ///0: Disabled
    _0 = 0,
    ///1: Enabled
    _1 = 1,
}
impl From<Rpeen> for bool {
    #[inline(always)]
    fn from(variant: Rpeen) -> Self {
        variant as u8 != 0
    }
}
///Field `RPEEN` reader - SRAM Parity Error Interrupt Enable
pub type RpeenR = crate::BitReader<Rpeen>;
impl RpeenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rpeen {
        match self.bits {
            false => Rpeen::_0,
            true => Rpeen::_1,
        }
    }
    ///Disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rpeen::_0
    }
    ///Enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rpeen::_1
    }
}
///Field `RPEEN` writer - SRAM Parity Error Interrupt Enable
pub type RpeenW<'a, REG> = crate::BitWriter<'a, REG, Rpeen>;
impl<'a, REG> RpeenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rpeen::_0)
    }
    ///Enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rpeen::_1)
    }
}
/**SRAM ECC Error Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reccen {
    ///0: Disabled
    _0 = 0,
    ///1: Enabled
    _1 = 1,
}
impl From<Reccen> for bool {
    #[inline(always)]
    fn from(variant: Reccen) -> Self {
        variant as u8 != 0
    }
}
///Field `RECCEN` reader - SRAM ECC Error Interrupt Enable
pub type ReccenR = crate::BitReader<Reccen>;
impl ReccenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Reccen {
        match self.bits {
            false => Reccen::_0,
            true => Reccen::_1,
        }
    }
    ///Disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Reccen::_0
    }
    ///Enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Reccen::_1
    }
}
///Field `RECCEN` writer - SRAM ECC Error Interrupt Enable
pub type ReccenW<'a, REG> = crate::BitWriter<'a, REG, Reccen>;
impl<'a, REG> ReccenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Reccen::_0)
    }
    ///Enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Reccen::_1)
    }
}
/**Bus Master MPU Error Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Busmen {
    ///0: Disabled
    _0 = 0,
    ///1: Enabled
    _1 = 1,
}
impl From<Busmen> for bool {
    #[inline(always)]
    fn from(variant: Busmen) -> Self {
        variant as u8 != 0
    }
}
///Field `BUSMEN` reader - Bus Master MPU Error Interrupt Enable
pub type BusmenR = crate::BitReader<Busmen>;
impl BusmenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Busmen {
        match self.bits {
            false => Busmen::_0,
            true => Busmen::_1,
        }
    }
    ///Disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Busmen::_0
    }
    ///Enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Busmen::_1
    }
}
///Field `BUSMEN` writer - Bus Master MPU Error Interrupt Enable
pub type BusmenW<'a, REG> = crate::BitWriter<'a, REG, Busmen>;
impl<'a, REG> BusmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Busmen::_0)
    }
    ///Enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Busmen::_1)
    }
}
/**

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tzfen {
    ///0: Disabled
    _0 = 0,
    ///1: Enabled
    _1 = 1,
}
impl From<Tzfen> for bool {
    #[inline(always)]
    fn from(variant: Tzfen) -> Self {
        variant as u8 != 0
    }
}
///Field `TZFEN` reader -
pub type TzfenR = crate::BitReader<Tzfen>;
impl TzfenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tzfen {
        match self.bits {
            false => Tzfen::_0,
            true => Tzfen::_1,
        }
    }
    ///Disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tzfen::_0
    }
    ///Enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tzfen::_1
    }
}
///Field `TZFEN` writer -
pub type TzfenW<'a, REG> = crate::BitWriter<'a, REG, Tzfen>;
impl<'a, REG> TzfenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tzfen::_0)
    }
    ///Enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tzfen::_1)
    }
}
/**

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpeen {
    ///0: Disabled
    _0 = 0,
    ///1: Enabled
    _1 = 1,
}
impl From<Cpeen> for bool {
    #[inline(always)]
    fn from(variant: Cpeen) -> Self {
        variant as u8 != 0
    }
}
///Field `CPEEN` reader -
pub type CpeenR = crate::BitReader<Cpeen>;
impl CpeenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cpeen {
        match self.bits {
            false => Cpeen::_0,
            true => Cpeen::_1,
        }
    }
    ///Disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cpeen::_0
    }
    ///Enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cpeen::_1
    }
}
///Field `CPEEN` writer -
pub type CpeenW<'a, REG> = crate::BitWriter<'a, REG, Cpeen>;
impl<'a, REG> CpeenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cpeen::_0)
    }
    ///Enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cpeen::_1)
    }
}
impl R {
    ///Bit 0 - IWDT Underflow/Refresh Error Interrupt Enable
    #[inline(always)]
    pub fn iwdten(&self) -> IwdtenR {
        IwdtenR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - WDT Underflow/Refresh Error Interrupt Enable
    #[inline(always)]
    pub fn wdten(&self) -> WdtenR {
        WdtenR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Voltage monitor 1 Interrupt Enable
    #[inline(always)]
    pub fn lvd1en(&self) -> Lvd1enR {
        Lvd1enR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Voltage monitor 2 Interrupt Enable
    #[inline(always)]
    pub fn lvd2en(&self) -> Lvd2enR {
        Lvd2enR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 6 - Main Clock Oscillation Stop Detection Interrupt Enable
    #[inline(always)]
    pub fn osten(&self) -> OstenR {
        OstenR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - NMI Pin Interrupt Enable
    #[inline(always)]
    pub fn nmien(&self) -> NmienR {
        NmienR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - SRAM Parity Error Interrupt Enable
    #[inline(always)]
    pub fn rpeen(&self) -> RpeenR {
        RpeenR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - SRAM ECC Error Interrupt Enable
    #[inline(always)]
    pub fn reccen(&self) -> ReccenR {
        ReccenR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - Bus Master MPU Error Interrupt Enable
    #[inline(always)]
    pub fn busmen(&self) -> BusmenR {
        BusmenR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 13
    #[inline(always)]
    pub fn tzfen(&self) -> TzfenR {
        TzfenR::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 15
    #[inline(always)]
    pub fn cpeen(&self) -> CpeenR {
        CpeenR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NMIER")
            .field("iwdten", &self.iwdten())
            .field("wdten", &self.wdten())
            .field("lvd1en", &self.lvd1en())
            .field("lvd2en", &self.lvd2en())
            .field("osten", &self.osten())
            .field("nmien", &self.nmien())
            .field("rpeen", &self.rpeen())
            .field("reccen", &self.reccen())
            .field("busmen", &self.busmen())
            .field("tzfen", &self.tzfen())
            .field("cpeen", &self.cpeen())
            .finish()
    }
}
impl W {
    ///Bit 0 - IWDT Underflow/Refresh Error Interrupt Enable
    #[inline(always)]
    pub fn iwdten(&mut self) -> IwdtenW<NmierSpec> {
        IwdtenW::new(self, 0)
    }
    ///Bit 1 - WDT Underflow/Refresh Error Interrupt Enable
    #[inline(always)]
    pub fn wdten(&mut self) -> WdtenW<NmierSpec> {
        WdtenW::new(self, 1)
    }
    ///Bit 2 - Voltage monitor 1 Interrupt Enable
    #[inline(always)]
    pub fn lvd1en(&mut self) -> Lvd1enW<NmierSpec> {
        Lvd1enW::new(self, 2)
    }
    ///Bit 3 - Voltage monitor 2 Interrupt Enable
    #[inline(always)]
    pub fn lvd2en(&mut self) -> Lvd2enW<NmierSpec> {
        Lvd2enW::new(self, 3)
    }
    ///Bit 6 - Main Clock Oscillation Stop Detection Interrupt Enable
    #[inline(always)]
    pub fn osten(&mut self) -> OstenW<NmierSpec> {
        OstenW::new(self, 6)
    }
    ///Bit 7 - NMI Pin Interrupt Enable
    #[inline(always)]
    pub fn nmien(&mut self) -> NmienW<NmierSpec> {
        NmienW::new(self, 7)
    }
    ///Bit 8 - SRAM Parity Error Interrupt Enable
    #[inline(always)]
    pub fn rpeen(&mut self) -> RpeenW<NmierSpec> {
        RpeenW::new(self, 8)
    }
    ///Bit 9 - SRAM ECC Error Interrupt Enable
    #[inline(always)]
    pub fn reccen(&mut self) -> ReccenW<NmierSpec> {
        ReccenW::new(self, 9)
    }
    ///Bit 11 - Bus Master MPU Error Interrupt Enable
    #[inline(always)]
    pub fn busmen(&mut self) -> BusmenW<NmierSpec> {
        BusmenW::new(self, 11)
    }
    ///Bit 13
    #[inline(always)]
    pub fn tzfen(&mut self) -> TzfenW<NmierSpec> {
        TzfenW::new(self, 13)
    }
    ///Bit 15
    #[inline(always)]
    pub fn cpeen(&mut self) -> CpeenW<NmierSpec> {
        CpeenW::new(self, 15)
    }
}
/**Non-Maskable Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`nmier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct NmierSpec;
impl crate::RegisterSpec for NmierSpec {
    type Ux = u16;
}
///`read()` method returns [`nmier::R`](R) reader structure
impl crate::Readable for NmierSpec {}
///`write(|w| ..)` method takes [`nmier::W`](W) writer structure
impl crate::Writable for NmierSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets NMIER to value 0
impl crate::Resettable for NmierSpec {}
