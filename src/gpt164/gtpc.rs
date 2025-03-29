///Register `GTPC` reader
pub type R = crate::R<GtpcSpec>;
///Register `GTPC` writer
pub type W = crate::W<GtpcSpec>;
/**Period Count Function Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pcen {
    ///0: Period count function is disabled
    _0 = 0,
    ///1: Period count function is enabled
    _1 = 1,
}
impl From<Pcen> for bool {
    #[inline(always)]
    fn from(variant: Pcen) -> Self {
        variant as u8 != 0
    }
}
///Field `PCEN` reader - Period Count Function Enable
pub type PcenR = crate::BitReader<Pcen>;
impl PcenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pcen {
        match self.bits {
            false => Pcen::_0,
            true => Pcen::_1,
        }
    }
    ///Period count function is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pcen::_0
    }
    ///Period count function is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pcen::_1
    }
}
///Field `PCEN` writer - Period Count Function Enable
pub type PcenW<'a, REG> = crate::BitWriter<'a, REG, Pcen>;
impl<'a, REG> PcenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Period count function is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pcen::_0)
    }
    ///Period count function is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pcen::_1)
    }
}
/**Automatic Stop Function Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Astp {
    ///0: Automatic stop function is disabled
    _0 = 0,
    ///1: Automatic stop function is enabled
    _1 = 1,
}
impl From<Astp> for bool {
    #[inline(always)]
    fn from(variant: Astp) -> Self {
        variant as u8 != 0
    }
}
///Field `ASTP` reader - Automatic Stop Function Enable
pub type AstpR = crate::BitReader<Astp>;
impl AstpR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Astp {
        match self.bits {
            false => Astp::_0,
            true => Astp::_1,
        }
    }
    ///Automatic stop function is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Astp::_0
    }
    ///Automatic stop function is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Astp::_1
    }
}
///Field `ASTP` writer - Automatic Stop Function Enable
pub type AstpW<'a, REG> = crate::BitWriter<'a, REG, Astp>;
impl<'a, REG> AstpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Automatic stop function is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Astp::_0)
    }
    ///Automatic stop function is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Astp::_1)
    }
}
///Field `PCNT` reader - Period Counter
pub type PcntR = crate::FieldReader<u16>;
///Field `PCNT` writer - Period Counter
pub type PcntW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bit 0 - Period Count Function Enable
    #[inline(always)]
    pub fn pcen(&self) -> PcenR {
        PcenR::new((self.bits & 1) != 0)
    }
    ///Bit 8 - Automatic Stop Function Enable
    #[inline(always)]
    pub fn astp(&self) -> AstpR {
        AstpR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 16:27 - Period Counter
    #[inline(always)]
    pub fn pcnt(&self) -> PcntR {
        PcntR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GTPC")
            .field("pcen", &self.pcen())
            .field("astp", &self.astp())
            .field("pcnt", &self.pcnt())
            .finish()
    }
}
impl W {
    ///Bit 0 - Period Count Function Enable
    #[inline(always)]
    pub fn pcen(&mut self) -> PcenW<GtpcSpec> {
        PcenW::new(self, 0)
    }
    ///Bit 8 - Automatic Stop Function Enable
    #[inline(always)]
    pub fn astp(&mut self) -> AstpW<GtpcSpec> {
        AstpW::new(self, 8)
    }
    ///Bits 16:27 - Period Counter
    #[inline(always)]
    pub fn pcnt(&mut self) -> PcntW<GtpcSpec> {
        PcntW::new(self, 16)
    }
}
/**General PWM Timer Period Count Register

You can [`read`](crate::Reg::read) this register and get [`gtpc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtpc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GtpcSpec;
impl crate::RegisterSpec for GtpcSpec {
    type Ux = u32;
}
///`read()` method returns [`gtpc::R`](R) reader structure
impl crate::Readable for GtpcSpec {}
///`write(|w| ..)` method takes [`gtpc::W`](W) writer structure
impl crate::Writable for GtpcSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTPC to value 0
impl crate::Resettable for GtpcSpec {}
