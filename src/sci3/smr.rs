///Register `SMR` reader
pub type R = crate::R<SmrSpec>;
///Register `SMR` writer
pub type W = crate::W<SmrSpec>;
/**Clock Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cks {
    ///0: PCLK clock (n = 0)
    _00 = 0,
    ///1: PCLK/4 clock (n = 1)
    _01 = 1,
    ///2: PCLK/16 clock (n = 2)
    _10 = 2,
    ///3: PCLK/64 clock (n = 3)
    _11 = 3,
}
impl From<Cks> for u8 {
    #[inline(always)]
    fn from(variant: Cks) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cks {
    type Ux = u8;
}
impl crate::IsEnum for Cks {}
///Field `CKS` reader - Clock Select
pub type CksR = crate::FieldReader<Cks>;
impl CksR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cks {
        match self.bits {
            0 => Cks::_00,
            1 => Cks::_01,
            2 => Cks::_10,
            3 => Cks::_11,
            _ => unreachable!(),
        }
    }
    ///PCLK clock (n = 0)
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Cks::_00
    }
    ///PCLK/4 clock (n = 1)
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Cks::_01
    }
    ///PCLK/16 clock (n = 2)
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Cks::_10
    }
    ///PCLK/64 clock (n = 3)
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Cks::_11
    }
}
///Field `CKS` writer - Clock Select
pub type CksW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cks, crate::Safe>;
impl<'a, REG> CksW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PCLK clock (n = 0)
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Cks::_00)
    }
    ///PCLK/4 clock (n = 1)
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Cks::_01)
    }
    ///PCLK/16 clock (n = 2)
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Cks::_10)
    }
    ///PCLK/64 clock (n = 3)
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Cks::_11)
    }
}
/**Multi-Processor Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mp {
    ///0: Disable multi-processor communications function
    _0 = 0,
    ///1: Enable multi-processor communications function
    _1 = 1,
}
impl From<Mp> for bool {
    #[inline(always)]
    fn from(variant: Mp) -> Self {
        variant as u8 != 0
    }
}
///Field `MP` reader - Multi-Processor Mode
pub type MpR = crate::BitReader<Mp>;
impl MpR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mp {
        match self.bits {
            false => Mp::_0,
            true => Mp::_1,
        }
    }
    ///Disable multi-processor communications function
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mp::_0
    }
    ///Enable multi-processor communications function
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mp::_1
    }
}
///Field `MP` writer - Multi-Processor Mode
pub type MpW<'a, REG> = crate::BitWriter<'a, REG, Mp>;
impl<'a, REG> MpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable multi-processor communications function
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mp::_0)
    }
    ///Enable multi-processor communications function
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mp::_1)
    }
}
/**Stop Bit Length

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stop {
    ///0: 1 stop bit
    _0 = 0,
    ///1: 2 stop bits
    _1 = 1,
}
impl From<Stop> for bool {
    #[inline(always)]
    fn from(variant: Stop) -> Self {
        variant as u8 != 0
    }
}
///Field `STOP` reader - Stop Bit Length
pub type StopR = crate::BitReader<Stop>;
impl StopR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Stop {
        match self.bits {
            false => Stop::_0,
            true => Stop::_1,
        }
    }
    ///1 stop bit
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Stop::_0
    }
    ///2 stop bits
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Stop::_1
    }
}
///Field `STOP` writer - Stop Bit Length
pub type StopW<'a, REG> = crate::BitWriter<'a, REG, Stop>;
impl<'a, REG> StopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///1 stop bit
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Stop::_0)
    }
    ///2 stop bits
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Stop::_1)
    }
}
/**Parity Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pm {
    ///0: Even parity
    _0 = 0,
    ///1: Odd parity
    _1 = 1,
}
impl From<Pm> for bool {
    #[inline(always)]
    fn from(variant: Pm) -> Self {
        variant as u8 != 0
    }
}
///Field `PM` reader - Parity Mode
pub type PmR = crate::BitReader<Pm>;
impl PmR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pm {
        match self.bits {
            false => Pm::_0,
            true => Pm::_1,
        }
    }
    ///Even parity
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pm::_0
    }
    ///Odd parity
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pm::_1
    }
}
///Field `PM` writer - Parity Mode
pub type PmW<'a, REG> = crate::BitWriter<'a, REG, Pm>;
impl<'a, REG> PmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Even parity
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pm::_0)
    }
    ///Odd parity
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pm::_1)
    }
}
/**Parity Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pe {
    ///0: When transmitting: Do not add parity bit When receiving: Do not check parity bit
    _0 = 0,
    ///1: When transmitting: Add parity bit When receiving: Check parity bit
    _1 = 1,
}
impl From<Pe> for bool {
    #[inline(always)]
    fn from(variant: Pe) -> Self {
        variant as u8 != 0
    }
}
///Field `PE` reader - Parity Enable
pub type PeR = crate::BitReader<Pe>;
impl PeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pe {
        match self.bits {
            false => Pe::_0,
            true => Pe::_1,
        }
    }
    ///When transmitting: Do not add parity bit When receiving: Do not check parity bit
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pe::_0
    }
    ///When transmitting: Add parity bit When receiving: Check parity bit
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pe::_1
    }
}
///Field `PE` writer - Parity Enable
pub type PeW<'a, REG> = crate::BitWriter<'a, REG, Pe>;
impl<'a, REG> PeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///When transmitting: Do not add parity bit When receiving: Do not check parity bit
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pe::_0)
    }
    ///When transmitting: Add parity bit When receiving: Check parity bit
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pe::_1)
    }
}
/**Character Length

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chr {
    ///0: SCMR.CHR1 = 0: Transmit/receive in 9-bit data length SCMR.CHR1 = 1: Transmit/receive in 8-bit data length (initial value)
    _0 = 0,
    ///1: SCMR.CHR1 = 0: Transmit/receive in 9-bit data length SCMR.CHR1 = 1: Transmit/receive in 7-bit data length
    _1 = 1,
}
impl From<Chr> for bool {
    #[inline(always)]
    fn from(variant: Chr) -> Self {
        variant as u8 != 0
    }
}
///Field `CHR` reader - Character Length
pub type ChrR = crate::BitReader<Chr>;
impl ChrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Chr {
        match self.bits {
            false => Chr::_0,
            true => Chr::_1,
        }
    }
    ///SCMR.CHR1 = 0: Transmit/receive in 9-bit data length SCMR.CHR1 = 1: Transmit/receive in 8-bit data length (initial value)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Chr::_0
    }
    ///SCMR.CHR1 = 0: Transmit/receive in 9-bit data length SCMR.CHR1 = 1: Transmit/receive in 7-bit data length
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Chr::_1
    }
}
///Field `CHR` writer - Character Length
pub type ChrW<'a, REG> = crate::BitWriter<'a, REG, Chr>;
impl<'a, REG> ChrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SCMR.CHR1 = 0: Transmit/receive in 9-bit data length SCMR.CHR1 = 1: Transmit/receive in 8-bit data length (initial value)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Chr::_0)
    }
    ///SCMR.CHR1 = 0: Transmit/receive in 9-bit data length SCMR.CHR1 = 1: Transmit/receive in 7-bit data length
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Chr::_1)
    }
}
/**Communication Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cm {
    ///0: Asynchronous mode or simple IIC mode
    _0 = 0,
    ///1: Clock synchronous mode or simple SPI mode
    _1 = 1,
}
impl From<Cm> for bool {
    #[inline(always)]
    fn from(variant: Cm) -> Self {
        variant as u8 != 0
    }
}
///Field `CM` reader - Communication Mode
pub type CmR = crate::BitReader<Cm>;
impl CmR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cm {
        match self.bits {
            false => Cm::_0,
            true => Cm::_1,
        }
    }
    ///Asynchronous mode or simple IIC mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cm::_0
    }
    ///Clock synchronous mode or simple SPI mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cm::_1
    }
}
///Field `CM` writer - Communication Mode
pub type CmW<'a, REG> = crate::BitWriter<'a, REG, Cm>;
impl<'a, REG> CmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Asynchronous mode or simple IIC mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cm::_0)
    }
    ///Clock synchronous mode or simple SPI mode
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cm::_1)
    }
}
impl R {
    ///Bits 0:1 - Clock Select
    #[inline(always)]
    pub fn cks(&self) -> CksR {
        CksR::new(self.bits & 3)
    }
    ///Bit 2 - Multi-Processor Mode
    #[inline(always)]
    pub fn mp(&self) -> MpR {
        MpR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Stop Bit Length
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Parity Mode
    #[inline(always)]
    pub fn pm(&self) -> PmR {
        PmR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Parity Enable
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Character Length
    #[inline(always)]
    pub fn chr(&self) -> ChrR {
        ChrR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Communication Mode
    #[inline(always)]
    pub fn cm(&self) -> CmR {
        CmR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMR")
            .field("cks", &self.cks())
            .field("mp", &self.mp())
            .field("stop", &self.stop())
            .field("pm", &self.pm())
            .field("pe", &self.pe())
            .field("chr", &self.chr())
            .field("cm", &self.cm())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Clock Select
    #[inline(always)]
    pub fn cks(&mut self) -> CksW<SmrSpec> {
        CksW::new(self, 0)
    }
    ///Bit 2 - Multi-Processor Mode
    #[inline(always)]
    pub fn mp(&mut self) -> MpW<SmrSpec> {
        MpW::new(self, 2)
    }
    ///Bit 3 - Stop Bit Length
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<SmrSpec> {
        StopW::new(self, 3)
    }
    ///Bit 4 - Parity Mode
    #[inline(always)]
    pub fn pm(&mut self) -> PmW<SmrSpec> {
        PmW::new(self, 4)
    }
    ///Bit 5 - Parity Enable
    #[inline(always)]
    pub fn pe(&mut self) -> PeW<SmrSpec> {
        PeW::new(self, 5)
    }
    ///Bit 6 - Character Length
    #[inline(always)]
    pub fn chr(&mut self) -> ChrW<SmrSpec> {
        ChrW::new(self, 6)
    }
    ///Bit 7 - Communication Mode
    #[inline(always)]
    pub fn cm(&mut self) -> CmW<SmrSpec> {
        CmW::new(self, 7)
    }
}
/**Serial Mode Register for Non-Smart Card Interface Mode (SCMR.SMIF = 0)

You can [`read`](crate::Reg::read) this register and get [`smr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SmrSpec;
impl crate::RegisterSpec for SmrSpec {
    type Ux = u8;
}
///`read()` method returns [`smr::R`](R) reader structure
impl crate::Readable for SmrSpec {}
///`write(|w| ..)` method takes [`smr::W`](W) writer structure
impl crate::Writable for SmrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SMR to value 0
impl crate::Resettable for SmrSpec {}
