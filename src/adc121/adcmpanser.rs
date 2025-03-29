///Register `ADCMPANSER` reader
pub type R = crate::R<AdcmpanserSpec>;
///Register `ADCMPANSER` writer
pub type W = crate::W<AdcmpanserSpec>;
/**Temperature Sensor Output Compare Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmptsa {
    ///0: Exclude the temperature sensor output from the compare Window A target range.
    _0 = 0,
    ///1: Include the temperature sensor output in the compare Window A target range.
    _1 = 1,
}
impl From<Cmptsa> for bool {
    #[inline(always)]
    fn from(variant: Cmptsa) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPTSA` reader - Temperature Sensor Output Compare Select
pub type CmptsaR = crate::BitReader<Cmptsa>;
impl CmptsaR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmptsa {
        match self.bits {
            false => Cmptsa::_0,
            true => Cmptsa::_1,
        }
    }
    ///Exclude the temperature sensor output from the compare Window A target range.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmptsa::_0
    }
    ///Include the temperature sensor output in the compare Window A target range.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmptsa::_1
    }
}
///Field `CMPTSA` writer - Temperature Sensor Output Compare Select
pub type CmptsaW<'a, REG> = crate::BitWriter<'a, REG, Cmptsa>;
impl<'a, REG> CmptsaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Exclude the temperature sensor output from the compare Window A target range.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmptsa::_0)
    }
    ///Include the temperature sensor output in the compare Window A target range.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmptsa::_1)
    }
}
/**Internal Reference Voltage Compare Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpoca {
    ///0: Exclude the internal reference voltage from the compare Window A target range.
    _0 = 0,
    ///1: Include the internal reference voltage in the compare Window A target range.
    _1 = 1,
}
impl From<Cmpoca> for bool {
    #[inline(always)]
    fn from(variant: Cmpoca) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPOCA` reader - Internal Reference Voltage Compare Select
pub type CmpocaR = crate::BitReader<Cmpoca>;
impl CmpocaR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmpoca {
        match self.bits {
            false => Cmpoca::_0,
            true => Cmpoca::_1,
        }
    }
    ///Exclude the internal reference voltage from the compare Window A target range.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpoca::_0
    }
    ///Include the internal reference voltage in the compare Window A target range.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpoca::_1
    }
}
///Field `CMPOCA` writer - Internal Reference Voltage Compare Select
pub type CmpocaW<'a, REG> = crate::BitWriter<'a, REG, Cmpoca>;
impl<'a, REG> CmpocaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Exclude the internal reference voltage from the compare Window A target range.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpoca::_0)
    }
    ///Include the internal reference voltage in the compare Window A target range.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpoca::_1)
    }
}
impl R {
    ///Bit 0 - Temperature Sensor Output Compare Select
    #[inline(always)]
    pub fn cmptsa(&self) -> CmptsaR {
        CmptsaR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Internal Reference Voltage Compare Select
    #[inline(always)]
    pub fn cmpoca(&self) -> CmpocaR {
        CmpocaR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCMPANSER")
            .field("cmptsa", &self.cmptsa())
            .field("cmpoca", &self.cmpoca())
            .finish()
    }
}
impl W {
    ///Bit 0 - Temperature Sensor Output Compare Select
    #[inline(always)]
    pub fn cmptsa(&mut self) -> CmptsaW<AdcmpanserSpec> {
        CmptsaW::new(self, 0)
    }
    ///Bit 1 - Internal Reference Voltage Compare Select
    #[inline(always)]
    pub fn cmpoca(&mut self) -> CmpocaW<AdcmpanserSpec> {
        CmpocaW::new(self, 1)
    }
}
/**A/D Compare Function Window A Extended Input Select Register

You can [`read`](crate::Reg::read) this register and get [`adcmpanser::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpanser::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AdcmpanserSpec;
impl crate::RegisterSpec for AdcmpanserSpec {
    type Ux = u8;
}
///`read()` method returns [`adcmpanser::R`](R) reader structure
impl crate::Readable for AdcmpanserSpec {}
///`write(|w| ..)` method takes [`adcmpanser::W`](W) writer structure
impl crate::Writable for AdcmpanserSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADCMPANSER to value 0
impl crate::Resettable for AdcmpanserSpec {}
