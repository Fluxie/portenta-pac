///Register `ADCMPSER` reader
pub type R = crate::R<AdcmpserSpec>;
///Register `ADCMPSER` writer
pub type W = crate::W<AdcmpserSpec>;
/**Compare Window A Temperature Sensor Output Compare Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpsttsa {
    ///0: Comparison conditions are not met.
    _0 = 0,
    ///1: Comparison conditions are met.
    _1 = 1,
}
impl From<Cmpsttsa> for bool {
    #[inline(always)]
    fn from(variant: Cmpsttsa) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPSTTSA` reader - Compare Window A Temperature Sensor Output Compare Flag
pub type CmpsttsaR = crate::BitReader<Cmpsttsa>;
impl CmpsttsaR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmpsttsa {
        match self.bits {
            false => Cmpsttsa::_0,
            true => Cmpsttsa::_1,
        }
    }
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpsttsa::_0
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpsttsa::_1
    }
}
///Field `CMPSTTSA` writer - Compare Window A Temperature Sensor Output Compare Flag
pub type CmpsttsaW<'a, REG> = crate::BitWriter<'a, REG, Cmpsttsa>;
impl<'a, REG> CmpsttsaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpsttsa::_0)
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpsttsa::_1)
    }
}
/**Compare Window A Internal Reference Voltage Compare Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpstoca {
    ///0: Comparison conditions are not met.
    _0 = 0,
    ///1: Comparison conditions are met.
    _1 = 1,
}
impl From<Cmpstoca> for bool {
    #[inline(always)]
    fn from(variant: Cmpstoca) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPSTOCA` reader - Compare Window A Internal Reference Voltage Compare Flag
pub type CmpstocaR = crate::BitReader<Cmpstoca>;
impl CmpstocaR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmpstoca {
        match self.bits {
            false => Cmpstoca::_0,
            true => Cmpstoca::_1,
        }
    }
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpstoca::_0
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpstoca::_1
    }
}
///Field `CMPSTOCA` writer - Compare Window A Internal Reference Voltage Compare Flag
pub type CmpstocaW<'a, REG> = crate::BitWriter<'a, REG, Cmpstoca>;
impl<'a, REG> CmpstocaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstoca::_0)
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpstoca::_1)
    }
}
impl R {
    ///Bit 0 - Compare Window A Temperature Sensor Output Compare Flag
    #[inline(always)]
    pub fn cmpsttsa(&self) -> CmpsttsaR {
        CmpsttsaR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Compare Window A Internal Reference Voltage Compare Flag
    #[inline(always)]
    pub fn cmpstoca(&self) -> CmpstocaR {
        CmpstocaR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCMPSER")
            .field("cmpsttsa", &self.cmpsttsa())
            .field("cmpstoca", &self.cmpstoca())
            .finish()
    }
}
impl W {
    ///Bit 0 - Compare Window A Temperature Sensor Output Compare Flag
    #[inline(always)]
    pub fn cmpsttsa(&mut self) -> CmpsttsaW<AdcmpserSpec> {
        CmpsttsaW::new(self, 0)
    }
    ///Bit 1 - Compare Window A Internal Reference Voltage Compare Flag
    #[inline(always)]
    pub fn cmpstoca(&mut self) -> CmpstocaW<AdcmpserSpec> {
        CmpstocaW::new(self, 1)
    }
}
/**A/D Compare Function Window A Extended Input Channel Status Register

You can [`read`](crate::Reg::read) this register and get [`adcmpser::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpser::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AdcmpserSpec;
impl crate::RegisterSpec for AdcmpserSpec {
    type Ux = u8;
}
///`read()` method returns [`adcmpser::R`](R) reader structure
impl crate::Readable for AdcmpserSpec {}
///`write(|w| ..)` method takes [`adcmpser::W`](W) writer structure
impl crate::Writable for AdcmpserSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADCMPSER to value 0
impl crate::Resettable for AdcmpserSpec {}
