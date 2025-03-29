///Register `SPSR` reader
pub type R = crate::R<SpsrSpec>;
///Register `SPSR` writer
pub type W = crate::W<SpsrSpec>;
/**Overrun Error Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ovrf {
    ///0: No overrun error occurred
    _0 = 0,
    ///1: Overrun error occurred
    _1 = 1,
}
impl From<Ovrf> for bool {
    #[inline(always)]
    fn from(variant: Ovrf) -> Self {
        variant as u8 != 0
    }
}
///Field `OVRF` reader - Overrun Error Flag
pub type OvrfR = crate::BitReader<Ovrf>;
impl OvrfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ovrf {
        match self.bits {
            false => Ovrf::_0,
            true => Ovrf::_1,
        }
    }
    ///No overrun error occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ovrf::_0
    }
    ///Overrun error occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ovrf::_1
    }
}
///Field `OVRF` writer - Overrun Error Flag
pub type OvrfW<'a, REG> = crate::BitWriter<'a, REG, Ovrf>;
impl<'a, REG> OvrfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No overrun error occurred
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ovrf::_0)
    }
    ///Overrun error occurred
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ovrf::_1)
    }
}
/**SPI Idle Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Idlnf {
    ///0: SPI is in the idle state
    _0 = 0,
    ///1: SPI is in the transfer state
    _1 = 1,
}
impl From<Idlnf> for bool {
    #[inline(always)]
    fn from(variant: Idlnf) -> Self {
        variant as u8 != 0
    }
}
///Field `IDLNF` reader - SPI Idle Flag
pub type IdlnfR = crate::BitReader<Idlnf>;
impl IdlnfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Idlnf {
        match self.bits {
            false => Idlnf::_0,
            true => Idlnf::_1,
        }
    }
    ///SPI is in the idle state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Idlnf::_0
    }
    ///SPI is in the transfer state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Idlnf::_1
    }
}
/**Mode Fault Error Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Modf {
    ///0: No mode fault or underrun error occurred
    _0 = 0,
    ///1: Mode fault error or underrun error occurred
    _1 = 1,
}
impl From<Modf> for bool {
    #[inline(always)]
    fn from(variant: Modf) -> Self {
        variant as u8 != 0
    }
}
///Field `MODF` reader - Mode Fault Error Flag
pub type ModfR = crate::BitReader<Modf>;
impl ModfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Modf {
        match self.bits {
            false => Modf::_0,
            true => Modf::_1,
        }
    }
    ///No mode fault or underrun error occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Modf::_0
    }
    ///Mode fault error or underrun error occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Modf::_1
    }
}
///Field `MODF` writer - Mode Fault Error Flag
pub type ModfW<'a, REG> = crate::BitWriter<'a, REG, Modf>;
impl<'a, REG> ModfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No mode fault or underrun error occurred
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Modf::_0)
    }
    ///Mode fault error or underrun error occurred
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Modf::_1)
    }
}
/**Parity Error Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Perf {
    ///0: No parity error occurred
    _0 = 0,
    ///1: Parity error occurred
    _1 = 1,
}
impl From<Perf> for bool {
    #[inline(always)]
    fn from(variant: Perf) -> Self {
        variant as u8 != 0
    }
}
///Field `PERF` reader - Parity Error Flag
pub type PerfR = crate::BitReader<Perf>;
impl PerfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Perf {
        match self.bits {
            false => Perf::_0,
            true => Perf::_1,
        }
    }
    ///No parity error occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Perf::_0
    }
    ///Parity error occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Perf::_1
    }
}
///Field `PERF` writer - Parity Error Flag
pub type PerfW<'a, REG> = crate::BitWriter<'a, REG, Perf>;
impl<'a, REG> PerfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No parity error occurred
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Perf::_0)
    }
    ///Parity error occurred
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Perf::_1)
    }
}
/**Underrun Error Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Udrf {
    ///0: Mode fault error occurred (MODF = 1)
    _0 = 0,
    ///1: Underrun error occurred (MODF = 1)
    _1 = 1,
}
impl From<Udrf> for bool {
    #[inline(always)]
    fn from(variant: Udrf) -> Self {
        variant as u8 != 0
    }
}
///Field `UDRF` reader - Underrun Error Flag
pub type UdrfR = crate::BitReader<Udrf>;
impl UdrfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Udrf {
        match self.bits {
            false => Udrf::_0,
            true => Udrf::_1,
        }
    }
    ///Mode fault error occurred (MODF = 1)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Udrf::_0
    }
    ///Underrun error occurred (MODF = 1)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Udrf::_1
    }
}
///Field `UDRF` writer - Underrun Error Flag
pub type UdrfW<'a, REG> = crate::BitWriter<'a, REG, Udrf>;
impl<'a, REG> UdrfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Mode fault error occurred (MODF = 1)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Udrf::_0)
    }
    ///Underrun error occurred (MODF = 1)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Udrf::_1)
    }
}
/**SPI Transmit Buffer Empty Flag

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sptef {
    ///0: Data is in the transmit buffer
    _0 = 0,
    ///1: No data is in the transmit buffer
    _1 = 1,
}
impl From<Sptef> for bool {
    #[inline(always)]
    fn from(variant: Sptef) -> Self {
        variant as u8 != 0
    }
}
///Field `SPTEF` reader - SPI Transmit Buffer Empty Flag
pub type SptefR = crate::BitReader<Sptef>;
impl SptefR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sptef {
        match self.bits {
            false => Sptef::_0,
            true => Sptef::_1,
        }
    }
    ///Data is in the transmit buffer
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sptef::_0
    }
    ///No data is in the transmit buffer
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sptef::_1
    }
}
///Field `SPTEF` writer - SPI Transmit Buffer Empty Flag
pub type SptefW<'a, REG> = crate::BitWriter<'a, REG, Sptef>;
impl<'a, REG> SptefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Data is in the transmit buffer
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sptef::_0)
    }
    ///No data is in the transmit buffer
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sptef::_1)
    }
}
/**Communication End Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cendf {
    ///0: Not communicating or communicating
    _0 = 0,
    ///1: Communication completed
    _1 = 1,
}
impl From<Cendf> for bool {
    #[inline(always)]
    fn from(variant: Cendf) -> Self {
        variant as u8 != 0
    }
}
///Field `CENDF` reader - Communication End Flag
pub type CendfR = crate::BitReader<Cendf>;
impl CendfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cendf {
        match self.bits {
            false => Cendf::_0,
            true => Cendf::_1,
        }
    }
    ///Not communicating or communicating
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cendf::_0
    }
    ///Communication completed
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cendf::_1
    }
}
///Field `CENDF` writer - Communication End Flag
pub type CendfW<'a, REG> = crate::BitWriter<'a, REG, Cendf>;
impl<'a, REG> CendfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Not communicating or communicating
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cendf::_0)
    }
    ///Communication completed
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cendf::_1)
    }
}
/**SPI Receive Buffer Full Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sprf {
    ///0: No valid data is in SPDR/SPDR_HA
    _0 = 0,
    ///1: Valid data is in SPDR/SPDR_HA
    _1 = 1,
}
impl From<Sprf> for bool {
    #[inline(always)]
    fn from(variant: Sprf) -> Self {
        variant as u8 != 0
    }
}
///Field `SPRF` reader - SPI Receive Buffer Full Flag
pub type SprfR = crate::BitReader<Sprf>;
impl SprfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sprf {
        match self.bits {
            false => Sprf::_0,
            true => Sprf::_1,
        }
    }
    ///No valid data is in SPDR/SPDR_HA
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sprf::_0
    }
    ///Valid data is in SPDR/SPDR_HA
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sprf::_1
    }
}
///Field `SPRF` writer - SPI Receive Buffer Full Flag
pub type SprfW<'a, REG> = crate::BitWriter<'a, REG, Sprf>;
impl<'a, REG> SprfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No valid data is in SPDR/SPDR_HA
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sprf::_0)
    }
    ///Valid data is in SPDR/SPDR_HA
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sprf::_1)
    }
}
impl R {
    ///Bit 0 - Overrun Error Flag
    #[inline(always)]
    pub fn ovrf(&self) -> OvrfR {
        OvrfR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SPI Idle Flag
    #[inline(always)]
    pub fn idlnf(&self) -> IdlnfR {
        IdlnfR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Mode Fault Error Flag
    #[inline(always)]
    pub fn modf(&self) -> ModfR {
        ModfR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Parity Error Flag
    #[inline(always)]
    pub fn perf(&self) -> PerfR {
        PerfR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Underrun Error Flag
    #[inline(always)]
    pub fn udrf(&self) -> UdrfR {
        UdrfR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - SPI Transmit Buffer Empty Flag
    #[inline(always)]
    pub fn sptef(&self) -> SptefR {
        SptefR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Communication End Flag
    #[inline(always)]
    pub fn cendf(&self) -> CendfR {
        CendfR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - SPI Receive Buffer Full Flag
    #[inline(always)]
    pub fn sprf(&self) -> SprfR {
        SprfR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPSR")
            .field("ovrf", &self.ovrf())
            .field("idlnf", &self.idlnf())
            .field("modf", &self.modf())
            .field("perf", &self.perf())
            .field("udrf", &self.udrf())
            .field("sptef", &self.sptef())
            .field("cendf", &self.cendf())
            .field("sprf", &self.sprf())
            .finish()
    }
}
impl W {
    ///Bit 0 - Overrun Error Flag
    #[inline(always)]
    pub fn ovrf(&mut self) -> OvrfW<SpsrSpec> {
        OvrfW::new(self, 0)
    }
    ///Bit 2 - Mode Fault Error Flag
    #[inline(always)]
    pub fn modf(&mut self) -> ModfW<SpsrSpec> {
        ModfW::new(self, 2)
    }
    ///Bit 3 - Parity Error Flag
    #[inline(always)]
    pub fn perf(&mut self) -> PerfW<SpsrSpec> {
        PerfW::new(self, 3)
    }
    ///Bit 4 - Underrun Error Flag
    #[inline(always)]
    pub fn udrf(&mut self) -> UdrfW<SpsrSpec> {
        UdrfW::new(self, 4)
    }
    ///Bit 5 - SPI Transmit Buffer Empty Flag
    #[inline(always)]
    pub fn sptef(&mut self) -> SptefW<SpsrSpec> {
        SptefW::new(self, 5)
    }
    ///Bit 6 - Communication End Flag
    #[inline(always)]
    pub fn cendf(&mut self) -> CendfW<SpsrSpec> {
        CendfW::new(self, 6)
    }
    ///Bit 7 - SPI Receive Buffer Full Flag
    #[inline(always)]
    pub fn sprf(&mut self) -> SprfW<SpsrSpec> {
        SprfW::new(self, 7)
    }
}
/**SPI Status Register

You can [`read`](crate::Reg::read) this register and get [`spsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SpsrSpec;
impl crate::RegisterSpec for SpsrSpec {
    type Ux = u8;
}
///`read()` method returns [`spsr::R`](R) reader structure
impl crate::Readable for SpsrSpec {}
///`write(|w| ..)` method takes [`spsr::W`](W) writer structure
impl crate::Writable for SpsrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SPSR to value 0x20
impl crate::Resettable for SpsrSpec {
    const RESET_VALUE: u8 = 0x20;
}
