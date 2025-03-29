///Register `P40%sPFS` reader
pub type R = crate::R<P40pfsSpec>;
///Register `P40%sPFS` writer
pub type W = crate::W<P40pfsSpec>;
/**Port Output Data

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Podr {
    ///0: Output low
    _0 = 0,
    ///1: Output high
    _1 = 1,
}
impl From<Podr> for bool {
    #[inline(always)]
    fn from(variant: Podr) -> Self {
        variant as u8 != 0
    }
}
///Field `PODR` reader - Port Output Data
pub type PodrR = crate::BitReader<Podr>;
impl PodrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Podr {
        match self.bits {
            false => Podr::_0,
            true => Podr::_1,
        }
    }
    ///Output low
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Podr::_0
    }
    ///Output high
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Podr::_1
    }
}
///Field `PODR` writer - Port Output Data
pub type PodrW<'a, REG> = crate::BitWriter<'a, REG, Podr>;
impl<'a, REG> PodrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Output low
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Podr::_0)
    }
    ///Output high
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Podr::_1)
    }
}
/**Port State

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pidr {
    ///0: Low level
    _0 = 0,
    ///1: High level
    _1 = 1,
}
impl From<Pidr> for bool {
    #[inline(always)]
    fn from(variant: Pidr) -> Self {
        variant as u8 != 0
    }
}
///Field `PIDR` reader - Port State
pub type PidrR = crate::BitReader<Pidr>;
impl PidrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pidr {
        match self.bits {
            false => Pidr::_0,
            true => Pidr::_1,
        }
    }
    ///Low level
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pidr::_0
    }
    ///High level
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pidr::_1
    }
}
/**Port Direction

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdr {
    ///0: Input (functions as an input pin)
    _0 = 0,
    ///1: Output (functions as an output pin)
    _1 = 1,
}
impl From<Pdr> for bool {
    #[inline(always)]
    fn from(variant: Pdr) -> Self {
        variant as u8 != 0
    }
}
///Field `PDR` reader - Port Direction
pub type PdrR = crate::BitReader<Pdr>;
impl PdrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pdr {
        match self.bits {
            false => Pdr::_0,
            true => Pdr::_1,
        }
    }
    ///Input (functions as an input pin)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pdr::_0
    }
    ///Output (functions as an output pin)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pdr::_1
    }
}
///Field `PDR` writer - Port Direction
pub type PdrW<'a, REG> = crate::BitWriter<'a, REG, Pdr>;
impl<'a, REG> PdrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Input (functions as an input pin)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr::_0)
    }
    ///Output (functions as an output pin)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr::_1)
    }
}
/**Pull-up Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pcr {
    ///0: Disable input pull-up
    _0 = 0,
    ///1: Enable input pull-up
    _1 = 1,
}
impl From<Pcr> for bool {
    #[inline(always)]
    fn from(variant: Pcr) -> Self {
        variant as u8 != 0
    }
}
///Field `PCR` reader - Pull-up Control
pub type PcrR = crate::BitReader<Pcr>;
impl PcrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pcr {
        match self.bits {
            false => Pcr::_0,
            true => Pcr::_1,
        }
    }
    ///Disable input pull-up
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pcr::_0
    }
    ///Enable input pull-up
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pcr::_1
    }
}
///Field `PCR` writer - Pull-up Control
pub type PcrW<'a, REG> = crate::BitWriter<'a, REG, Pcr>;
impl<'a, REG> PcrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable input pull-up
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pcr::_0)
    }
    ///Enable input pull-up
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pcr::_1)
    }
}
/**N-Channel Open-Drain Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ncodr {
    ///0: Output CMOS
    _0 = 0,
    ///1: Output NMOS open-drain
    _1 = 1,
}
impl From<Ncodr> for bool {
    #[inline(always)]
    fn from(variant: Ncodr) -> Self {
        variant as u8 != 0
    }
}
///Field `NCODR` reader - N-Channel Open-Drain Control
pub type NcodrR = crate::BitReader<Ncodr>;
impl NcodrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ncodr {
        match self.bits {
            false => Ncodr::_0,
            true => Ncodr::_1,
        }
    }
    ///Output CMOS
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ncodr::_0
    }
    ///Output NMOS open-drain
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ncodr::_1
    }
}
///Field `NCODR` writer - N-Channel Open-Drain Control
pub type NcodrW<'a, REG> = crate::BitWriter<'a, REG, Ncodr>;
impl<'a, REG> NcodrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Output CMOS
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncodr::_0)
    }
    ///Output NMOS open-drain
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncodr::_1)
    }
}
/**Event on Falling/Event on Rising

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Eofr {
    ///0: Don't care
    _00 = 0,
    ///1: Detect rising edge
    _01 = 1,
    ///2: Detect falling edge
    _10 = 2,
    ///3: Detect both edges
    _11 = 3,
}
impl From<Eofr> for u8 {
    #[inline(always)]
    fn from(variant: Eofr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Eofr {
    type Ux = u8;
}
impl crate::IsEnum for Eofr {}
///Field `EOFR` reader - Event on Falling/Event on Rising
pub type EofrR = crate::FieldReader<Eofr>;
impl EofrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eofr {
        match self.bits {
            0 => Eofr::_00,
            1 => Eofr::_01,
            2 => Eofr::_10,
            3 => Eofr::_11,
            _ => unreachable!(),
        }
    }
    ///Don't care
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Eofr::_00
    }
    ///Detect rising edge
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Eofr::_01
    }
    ///Detect falling edge
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Eofr::_10
    }
    ///Detect both edges
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Eofr::_11
    }
}
///Field `EOFR` writer - Event on Falling/Event on Rising
pub type EofrW<'a, REG> = crate::FieldWriter<'a, REG, 2, Eofr, crate::Safe>;
impl<'a, REG> EofrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Don't care
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Eofr::_00)
    }
    ///Detect rising edge
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Eofr::_01)
    }
    ///Detect falling edge
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Eofr::_10)
    }
    ///Detect both edges
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Eofr::_11)
    }
}
/**IRQ Input Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Isel {
    ///0: Do not use as IRQn input pin
    _0 = 0,
    ///1: Use as IRQn input pin
    _1 = 1,
}
impl From<Isel> for bool {
    #[inline(always)]
    fn from(variant: Isel) -> Self {
        variant as u8 != 0
    }
}
///Field `ISEL` reader - IRQ Input Enable
pub type IselR = crate::BitReader<Isel>;
impl IselR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Isel {
        match self.bits {
            false => Isel::_0,
            true => Isel::_1,
        }
    }
    ///Do not use as IRQn input pin
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Isel::_0
    }
    ///Use as IRQn input pin
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Isel::_1
    }
}
///Field `ISEL` writer - IRQ Input Enable
pub type IselW<'a, REG> = crate::BitWriter<'a, REG, Isel>;
impl<'a, REG> IselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not use as IRQn input pin
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Isel::_0)
    }
    ///Use as IRQn input pin
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Isel::_1)
    }
}
/**Analog Input Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Asel {
    ///0: Do not use as analog pin
    _0 = 0,
    ///1: Use as analog pin
    _1 = 1,
}
impl From<Asel> for bool {
    #[inline(always)]
    fn from(variant: Asel) -> Self {
        variant as u8 != 0
    }
}
///Field `ASEL` reader - Analog Input Enable
pub type AselR = crate::BitReader<Asel>;
impl AselR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Asel {
        match self.bits {
            false => Asel::_0,
            true => Asel::_1,
        }
    }
    ///Do not use as analog pin
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Asel::_0
    }
    ///Use as analog pin
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Asel::_1
    }
}
///Field `ASEL` writer - Analog Input Enable
pub type AselW<'a, REG> = crate::BitWriter<'a, REG, Asel>;
impl<'a, REG> AselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not use as analog pin
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Asel::_0)
    }
    ///Use as analog pin
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Asel::_1)
    }
}
/**Port Mode Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pmr {
    ///0: Use as general I/O pin
    _0 = 0,
    ///1: Use as I/O port for peripheral functions
    _1 = 1,
}
impl From<Pmr> for bool {
    #[inline(always)]
    fn from(variant: Pmr) -> Self {
        variant as u8 != 0
    }
}
///Field `PMR` reader - Port Mode Control
pub type PmrR = crate::BitReader<Pmr>;
impl PmrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pmr {
        match self.bits {
            false => Pmr::_0,
            true => Pmr::_1,
        }
    }
    ///Use as general I/O pin
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pmr::_0
    }
    ///Use as I/O port for peripheral functions
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pmr::_1
    }
}
///Field `PMR` writer - Port Mode Control
pub type PmrW<'a, REG> = crate::BitWriter<'a, REG, Pmr>;
impl<'a, REG> PmrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Use as general I/O pin
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pmr::_0)
    }
    ///Use as I/O port for peripheral functions
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pmr::_1)
    }
}
///Field `PSEL` reader - Peripheral Select
pub type PselR = crate::FieldReader;
///Field `PSEL` writer - Peripheral Select
pub type PselW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bit 0 - Port Output Data
    #[inline(always)]
    pub fn podr(&self) -> PodrR {
        PodrR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Port State
    #[inline(always)]
    pub fn pidr(&self) -> PidrR {
        PidrR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Port Direction
    #[inline(always)]
    pub fn pdr(&self) -> PdrR {
        PdrR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - Pull-up Control
    #[inline(always)]
    pub fn pcr(&self) -> PcrR {
        PcrR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - N-Channel Open-Drain Control
    #[inline(always)]
    pub fn ncodr(&self) -> NcodrR {
        NcodrR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 12:13 - Event on Falling/Event on Rising
    #[inline(always)]
    pub fn eofr(&self) -> EofrR {
        EofrR::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bit 14 - IRQ Input Enable
    #[inline(always)]
    pub fn isel(&self) -> IselR {
        IselR::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Analog Input Enable
    #[inline(always)]
    pub fn asel(&self) -> AselR {
        AselR::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Port Mode Control
    #[inline(always)]
    pub fn pmr(&self) -> PmrR {
        PmrR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 24:28 - Peripheral Select
    #[inline(always)]
    pub fn psel(&self) -> PselR {
        PselR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P40PFS")
            .field("podr", &self.podr())
            .field("pidr", &self.pidr())
            .field("pdr", &self.pdr())
            .field("pcr", &self.pcr())
            .field("ncodr", &self.ncodr())
            .field("eofr", &self.eofr())
            .field("isel", &self.isel())
            .field("asel", &self.asel())
            .field("pmr", &self.pmr())
            .field("psel", &self.psel())
            .finish()
    }
}
impl W {
    ///Bit 0 - Port Output Data
    #[inline(always)]
    pub fn podr(&mut self) -> PodrW<P40pfsSpec> {
        PodrW::new(self, 0)
    }
    ///Bit 2 - Port Direction
    #[inline(always)]
    pub fn pdr(&mut self) -> PdrW<P40pfsSpec> {
        PdrW::new(self, 2)
    }
    ///Bit 4 - Pull-up Control
    #[inline(always)]
    pub fn pcr(&mut self) -> PcrW<P40pfsSpec> {
        PcrW::new(self, 4)
    }
    ///Bit 6 - N-Channel Open-Drain Control
    #[inline(always)]
    pub fn ncodr(&mut self) -> NcodrW<P40pfsSpec> {
        NcodrW::new(self, 6)
    }
    ///Bits 12:13 - Event on Falling/Event on Rising
    #[inline(always)]
    pub fn eofr(&mut self) -> EofrW<P40pfsSpec> {
        EofrW::new(self, 12)
    }
    ///Bit 14 - IRQ Input Enable
    #[inline(always)]
    pub fn isel(&mut self) -> IselW<P40pfsSpec> {
        IselW::new(self, 14)
    }
    ///Bit 15 - Analog Input Enable
    #[inline(always)]
    pub fn asel(&mut self) -> AselW<P40pfsSpec> {
        AselW::new(self, 15)
    }
    ///Bit 16 - Port Mode Control
    #[inline(always)]
    pub fn pmr(&mut self) -> PmrW<P40pfsSpec> {
        PmrW::new(self, 16)
    }
    ///Bits 24:28 - Peripheral Select
    #[inline(always)]
    pub fn psel(&mut self) -> PselW<P40pfsSpec> {
        PselW::new(self, 24)
    }
}
/**Port 40%s Pin Function Select Register

You can [`read`](crate::Reg::read) this register and get [`p40pfs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p40pfs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct P40pfsSpec;
impl crate::RegisterSpec for P40pfsSpec {
    type Ux = u32;
}
///`read()` method returns [`p40pfs::R`](R) reader structure
impl crate::Readable for P40pfsSpec {}
///`write(|w| ..)` method takes [`p40pfs::W`](W) writer structure
impl crate::Writable for P40pfsSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets P40%sPFS to value 0
impl crate::Resettable for P40pfsSpec {}
