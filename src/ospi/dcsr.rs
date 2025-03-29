///Register `DCSR` reader
pub type R = crate::R<DcsrSpec>;
///Register `DCSR` writer
pub type W = crate::W<DcsrSpec>;
///Field `DALEN` reader - Transfer data length setting
pub type DalenR = crate::FieldReader;
///Field `DALEN` writer - Transfer data length setting
pub type DalenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DMLEN` reader - Dummy cycle setting
pub type DmlenR = crate::FieldReader;
///Field `DMLEN` writer - Dummy cycle setting
pub type DmlenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
/**Access Device setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acdv {
    ///0: Send commands to device 0.
    _0 = 0,
    ///1: Send commands to device 1.
    _1 = 1,
}
impl From<Acdv> for bool {
    #[inline(always)]
    fn from(variant: Acdv) -> Self {
        variant as u8 != 0
    }
}
///Field `ACDV` reader - Access Device setting
pub type AcdvR = crate::BitReader<Acdv>;
impl AcdvR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Acdv {
        match self.bits {
            false => Acdv::_0,
            true => Acdv::_1,
        }
    }
    ///Send commands to device 0.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Acdv::_0
    }
    ///Send commands to device 1.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Acdv::_1
    }
}
///Field `ACDV` writer - Access Device setting
pub type AcdvW<'a, REG> = crate::BitWriter<'a, REG, Acdv>;
impl<'a, REG> AcdvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Send commands to device 0.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Acdv::_0)
    }
    ///Send commands to device 1.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Acdv::_1)
    }
}
///Field `CMDLEN` reader - Transfer command length setting
pub type CmdlenR = crate::FieldReader;
///Field `CMDLEN` writer - Transfer command length setting
pub type CmdlenW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
/**Data order setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Daor {
    ///0: byte0, byte1, byte2, byte3
    _0 = 0,
    ///1: byte1, byte0, byte3, byte2
    _1 = 1,
}
impl From<Daor> for bool {
    #[inline(always)]
    fn from(variant: Daor) -> Self {
        variant as u8 != 0
    }
}
///Field `DAOR` reader - Data order setting
pub type DaorR = crate::BitReader<Daor>;
impl DaorR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Daor {
        match self.bits {
            false => Daor::_0,
            true => Daor::_1,
        }
    }
    ///byte0, byte1, byte2, byte3
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Daor::_0
    }
    ///byte1, byte0, byte3, byte2
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Daor::_1
    }
}
///Field `DAOR` writer - Data order setting
pub type DaorW<'a, REG> = crate::BitWriter<'a, REG, Daor>;
impl<'a, REG> DaorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///byte0, byte1, byte2, byte3
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Daor::_0)
    }
    ///byte1, byte0, byte3, byte2
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Daor::_1)
    }
}
///Field `ADLEN` reader - Transfer address length setting
pub type AdlenR = crate::FieldReader;
///Field `ADLEN` writer - Transfer address length setting
pub type AdlenW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
/**DOPI single byte setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dopi {
    ///0: Each cycle has two bytes data. (normal DOPI mode)
    _0 = 0,
    ///1: Each cycle has one byte data. (The byte data changes at the rising edge of the clock and does not change at the falling edge of the clock.)
    _1 = 1,
}
impl From<Dopi> for bool {
    #[inline(always)]
    fn from(variant: Dopi) -> Self {
        variant as u8 != 0
    }
}
///Field `DOPI` reader - DOPI single byte setting
pub type DopiR = crate::BitReader<Dopi>;
impl DopiR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dopi {
        match self.bits {
            false => Dopi::_0,
            true => Dopi::_1,
        }
    }
    ///Each cycle has two bytes data. (normal DOPI mode)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dopi::_0
    }
    ///Each cycle has one byte data. (The byte data changes at the rising edge of the clock and does not change at the falling edge of the clock.)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dopi::_1
    }
}
///Field `DOPI` writer - DOPI single byte setting
pub type DopiW<'a, REG> = crate::BitWriter<'a, REG, Dopi>;
impl<'a, REG> DopiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Each cycle has two bytes data. (normal DOPI mode)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dopi::_0)
    }
    ///Each cycle has one byte data. (The byte data changes at the rising edge of the clock and does not change at the falling edge of the clock.)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dopi::_1)
    }
}
/**Data Access Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acda {
    ///0: Register access Do not arrange the transfer data.
    _0 = 0,
    ///1: Data access
    _1 = 1,
}
impl From<Acda> for bool {
    #[inline(always)]
    fn from(variant: Acda) -> Self {
        variant as u8 != 0
    }
}
///Field `ACDA` reader - Data Access Control
pub type AcdaR = crate::BitReader<Acda>;
impl AcdaR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Acda {
        match self.bits {
            false => Acda::_0,
            true => Acda::_1,
        }
    }
    ///Register access Do not arrange the transfer data.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Acda::_0
    }
    ///Data access
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Acda::_1
    }
}
///Field `ACDA` writer - Data Access Control
pub type AcdaW<'a, REG> = crate::BitWriter<'a, REG, Acda>;
impl<'a, REG> AcdaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Register access Do not arrange the transfer data.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Acda::_0)
    }
    ///Data access
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Acda::_1)
    }
}
/**Preamble bit enable for OctaRAM

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pren {
    ///0: No check preamble bit from OctaRAM
    _0 = 0,
    ///1: Check preamble bit from OctaRAM
    _1 = 1,
}
impl From<Pren> for bool {
    #[inline(always)]
    fn from(variant: Pren) -> Self {
        variant as u8 != 0
    }
}
///Field `PREN` reader - Preamble bit enable for OctaRAM
pub type PrenR = crate::BitReader<Pren>;
impl PrenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pren {
        match self.bits {
            false => Pren::_0,
            true => Pren::_1,
        }
    }
    ///No check preamble bit from OctaRAM
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pren::_0
    }
    ///Check preamble bit from OctaRAM
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pren::_1
    }
}
///Field `PREN` writer - Preamble bit enable for OctaRAM
pub type PrenW<'a, REG> = crate::BitWriter<'a, REG, Pren>;
impl<'a, REG> PrenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No check preamble bit from OctaRAM
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pren::_0)
    }
    ///Check preamble bit from OctaRAM
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pren::_1)
    }
}
impl R {
    ///Bits 0:7 - Transfer data length setting
    #[inline(always)]
    pub fn dalen(&self) -> DalenR {
        DalenR::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Dummy cycle setting
    #[inline(always)]
    pub fn dmlen(&self) -> DmlenR {
        DmlenR::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bit 19 - Access Device setting
    #[inline(always)]
    pub fn acdv(&self) -> AcdvR {
        AcdvR::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bits 20:22 - Transfer command length setting
    #[inline(always)]
    pub fn cmdlen(&self) -> CmdlenR {
        CmdlenR::new(((self.bits >> 20) & 7) as u8)
    }
    ///Bit 23 - Data order setting
    #[inline(always)]
    pub fn daor(&self) -> DaorR {
        DaorR::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 24:26 - Transfer address length setting
    #[inline(always)]
    pub fn adlen(&self) -> AdlenR {
        AdlenR::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bit 27 - DOPI single byte setting
    #[inline(always)]
    pub fn dopi(&self) -> DopiR {
        DopiR::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Data Access Control
    #[inline(always)]
    pub fn acda(&self) -> AcdaR {
        AcdaR::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Preamble bit enable for OctaRAM
    #[inline(always)]
    pub fn pren(&self) -> PrenR {
        PrenR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCSR")
            .field("dalen", &self.dalen())
            .field("dmlen", &self.dmlen())
            .field("acdv", &self.acdv())
            .field("cmdlen", &self.cmdlen())
            .field("daor", &self.daor())
            .field("adlen", &self.adlen())
            .field("dopi", &self.dopi())
            .field("acda", &self.acda())
            .field("pren", &self.pren())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Transfer data length setting
    #[inline(always)]
    pub fn dalen(&mut self) -> DalenW<DcsrSpec> {
        DalenW::new(self, 0)
    }
    ///Bits 8:15 - Dummy cycle setting
    #[inline(always)]
    pub fn dmlen(&mut self) -> DmlenW<DcsrSpec> {
        DmlenW::new(self, 8)
    }
    ///Bit 19 - Access Device setting
    #[inline(always)]
    pub fn acdv(&mut self) -> AcdvW<DcsrSpec> {
        AcdvW::new(self, 19)
    }
    ///Bits 20:22 - Transfer command length setting
    #[inline(always)]
    pub fn cmdlen(&mut self) -> CmdlenW<DcsrSpec> {
        CmdlenW::new(self, 20)
    }
    ///Bit 23 - Data order setting
    #[inline(always)]
    pub fn daor(&mut self) -> DaorW<DcsrSpec> {
        DaorW::new(self, 23)
    }
    ///Bits 24:26 - Transfer address length setting
    #[inline(always)]
    pub fn adlen(&mut self) -> AdlenW<DcsrSpec> {
        AdlenW::new(self, 24)
    }
    ///Bit 27 - DOPI single byte setting
    #[inline(always)]
    pub fn dopi(&mut self) -> DopiW<DcsrSpec> {
        DopiW::new(self, 27)
    }
    ///Bit 28 - Data Access Control
    #[inline(always)]
    pub fn acda(&mut self) -> AcdaW<DcsrSpec> {
        AcdaW::new(self, 28)
    }
    ///Bit 29 - Preamble bit enable for OctaRAM
    #[inline(always)]
    pub fn pren(&mut self) -> PrenW<DcsrSpec> {
        PrenW::new(self, 29)
    }
}
/**Device Command Setting Register

You can [`read`](crate::Reg::read) this register and get [`dcsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DcsrSpec;
impl crate::RegisterSpec for DcsrSpec {
    type Ux = u32;
}
///`read()` method returns [`dcsr::R`](R) reader structure
impl crate::Readable for DcsrSpec {}
///`write(|w| ..)` method takes [`dcsr::W`](W) writer structure
impl crate::Writable for DcsrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DCSR to value 0
impl crate::Resettable for DcsrSpec {}
