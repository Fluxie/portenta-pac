///Register `CFDC%sFDSTS` reader
pub type R = crate::R<CfdcfdstsSpec>;
///Register `CFDC%sFDSTS` writer
pub type W = crate::W<CfdcfdstsSpec>;
///Field `TDCR` reader - Transceiver Delay Compensation Result
pub type TdcrR = crate::FieldReader;
/**Error Occurrence Counter Overflow

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eoco {
    ///0: Error occurrence counter has not overflowed
    _0 = 0,
    ///1: Error occurrence counter has overflowed
    _1 = 1,
}
impl From<Eoco> for bool {
    #[inline(always)]
    fn from(variant: Eoco) -> Self {
        variant as u8 != 0
    }
}
///Field `EOCO` reader - Error Occurrence Counter Overflow
pub type EocoR = crate::BitReader<Eoco>;
impl EocoR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eoco {
        match self.bits {
            false => Eoco::_0,
            true => Eoco::_1,
        }
    }
    ///Error occurrence counter has not overflowed
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eoco::_0
    }
    ///Error occurrence counter has overflowed
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eoco::_1
    }
}
///Field `EOCO` writer - Error Occurrence Counter Overflow
pub type EocoW<'a, REG> = crate::BitWriter<'a, REG, Eoco>;
impl<'a, REG> EocoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Error occurrence counter has not overflowed
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eoco::_0)
    }
    ///Error occurrence counter has overflowed
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eoco::_1)
    }
}
/**Successful Occurrence Counter Overflow

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Soco {
    ///0: Successful occurrence counter has not overflowed
    _0 = 0,
    ///1: Successful occurrence counter has overflowed
    _1 = 1,
}
impl From<Soco> for bool {
    #[inline(always)]
    fn from(variant: Soco) -> Self {
        variant as u8 != 0
    }
}
///Field `SOCO` reader - Successful Occurrence Counter Overflow
pub type SocoR = crate::BitReader<Soco>;
impl SocoR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Soco {
        match self.bits {
            false => Soco::_0,
            true => Soco::_1,
        }
    }
    ///Successful occurrence counter has not overflowed
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Soco::_0
    }
    ///Successful occurrence counter has overflowed
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Soco::_1
    }
}
///Field `SOCO` writer - Successful Occurrence Counter Overflow
pub type SocoW<'a, REG> = crate::BitWriter<'a, REG, Soco>;
impl<'a, REG> SocoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Successful occurrence counter has not overflowed
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Soco::_0)
    }
    ///Successful occurrence counter has overflowed
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Soco::_1)
    }
}
/**Transceiver Delay Compensation Violation Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tdcvf {
    ///0: Transceiver delay compensation violation has not occurred
    _0 = 0,
    ///1: Transceiver delay compensation violation has occurred
    _1 = 1,
}
impl From<Tdcvf> for bool {
    #[inline(always)]
    fn from(variant: Tdcvf) -> Self {
        variant as u8 != 0
    }
}
///Field `TDCVF` reader - Transceiver Delay Compensation Violation Flag
pub type TdcvfR = crate::BitReader<Tdcvf>;
impl TdcvfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tdcvf {
        match self.bits {
            false => Tdcvf::_0,
            true => Tdcvf::_1,
        }
    }
    ///Transceiver delay compensation violation has not occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tdcvf::_0
    }
    ///Transceiver delay compensation violation has occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tdcvf::_1
    }
}
///Field `TDCVF` writer - Transceiver Delay Compensation Violation Flag
pub type TdcvfW<'a, REG> = crate::BitWriter<'a, REG, Tdcvf>;
impl<'a, REG> TdcvfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Transceiver delay compensation violation has not occurred
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tdcvf::_0)
    }
    ///Transceiver delay compensation violation has occurred
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tdcvf::_1)
    }
}
///Field `EOC` reader - Error Occurrence Counter
pub type EocR = crate::FieldReader;
///Field `SOC` reader - Successful occurrence counter
pub type SocR = crate::FieldReader;
impl R {
    ///Bits 0:7 - Transceiver Delay Compensation Result
    #[inline(always)]
    pub fn tdcr(&self) -> TdcrR {
        TdcrR::new((self.bits & 0xff) as u8)
    }
    ///Bit 8 - Error Occurrence Counter Overflow
    #[inline(always)]
    pub fn eoco(&self) -> EocoR {
        EocoR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Successful Occurrence Counter Overflow
    #[inline(always)]
    pub fn soco(&self) -> SocoR {
        SocoR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 15 - Transceiver Delay Compensation Violation Flag
    #[inline(always)]
    pub fn tdcvf(&self) -> TdcvfR {
        TdcvfR::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:23 - Error Occurrence Counter
    #[inline(always)]
    pub fn eoc(&self) -> EocR {
        EocR::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Successful occurrence counter
    #[inline(always)]
    pub fn soc(&self) -> SocR {
        SocR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDCFDSTS")
            .field("tdcr", &self.tdcr())
            .field("eoco", &self.eoco())
            .field("soco", &self.soco())
            .field("tdcvf", &self.tdcvf())
            .field("eoc", &self.eoc())
            .field("soc", &self.soc())
            .finish()
    }
}
impl W {
    ///Bit 8 - Error Occurrence Counter Overflow
    #[inline(always)]
    pub fn eoco(&mut self) -> EocoW<CfdcfdstsSpec> {
        EocoW::new(self, 8)
    }
    ///Bit 9 - Successful Occurrence Counter Overflow
    #[inline(always)]
    pub fn soco(&mut self) -> SocoW<CfdcfdstsSpec> {
        SocoW::new(self, 9)
    }
    ///Bit 15 - Transceiver Delay Compensation Violation Flag
    #[inline(always)]
    pub fn tdcvf(&mut self) -> TdcvfW<CfdcfdstsSpec> {
        TdcvfW::new(self, 15)
    }
}
/**Channel %s CANFD Status Register

You can [`read`](crate::Reg::read) this register and get [`cfdcfdsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcfdsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdcfdstsSpec;
impl crate::RegisterSpec for CfdcfdstsSpec {
    type Ux = u32;
}
///`read()` method returns [`cfdcfdsts::R`](R) reader structure
impl crate::Readable for CfdcfdstsSpec {}
///`write(|w| ..)` method takes [`cfdcfdsts::W`](W) writer structure
impl crate::Writable for CfdcfdstsSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDC%sFDSTS to value 0
impl crate::Resettable for CfdcfdstsSpec {}
