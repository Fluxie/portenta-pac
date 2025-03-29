///Register `TSCR` reader
pub type R = crate::R<TscrSpec>;
///Register `TSCR` writer
pub type W = crate::W<TscrSpec>;
/**Temperature Sensor Output Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tsoe {
    ///0: Disable output from the temperature sensor to the ADC12
    _0 = 0,
    ///1: Enable output from the temperature sensor to the ADC12
    _1 = 1,
}
impl From<Tsoe> for bool {
    #[inline(always)]
    fn from(variant: Tsoe) -> Self {
        variant as u8 != 0
    }
}
///Field `TSOE` reader - Temperature Sensor Output Enable
pub type TsoeR = crate::BitReader<Tsoe>;
impl TsoeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tsoe {
        match self.bits {
            false => Tsoe::_0,
            true => Tsoe::_1,
        }
    }
    ///Disable output from the temperature sensor to the ADC12
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tsoe::_0
    }
    ///Enable output from the temperature sensor to the ADC12
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tsoe::_1
    }
}
///Field `TSOE` writer - Temperature Sensor Output Enable
pub type TsoeW<'a, REG> = crate::BitWriter<'a, REG, Tsoe>;
impl<'a, REG> TsoeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable output from the temperature sensor to the ADC12
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tsoe::_0)
    }
    ///Enable output from the temperature sensor to the ADC12
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tsoe::_1)
    }
}
/**Temperature Sensor Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tsen {
    ///0: Stop the temperature sensor
    _0 = 0,
    ///1: Start the temperature sensor.
    _1 = 1,
}
impl From<Tsen> for bool {
    #[inline(always)]
    fn from(variant: Tsen) -> Self {
        variant as u8 != 0
    }
}
///Field `TSEN` reader - Temperature Sensor Enable
pub type TsenR = crate::BitReader<Tsen>;
impl TsenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tsen {
        match self.bits {
            false => Tsen::_0,
            true => Tsen::_1,
        }
    }
    ///Stop the temperature sensor
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tsen::_0
    }
    ///Start the temperature sensor.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tsen::_1
    }
}
///Field `TSEN` writer - Temperature Sensor Enable
pub type TsenW<'a, REG> = crate::BitWriter<'a, REG, Tsen>;
impl<'a, REG> TsenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Stop the temperature sensor
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tsen::_0)
    }
    ///Start the temperature sensor.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tsen::_1)
    }
}
impl R {
    ///Bit 4 - Temperature Sensor Output Enable
    #[inline(always)]
    pub fn tsoe(&self) -> TsoeR {
        TsoeR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 7 - Temperature Sensor Enable
    #[inline(always)]
    pub fn tsen(&self) -> TsenR {
        TsenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TSCR")
            .field("tsoe", &self.tsoe())
            .field("tsen", &self.tsen())
            .finish()
    }
}
impl W {
    ///Bit 4 - Temperature Sensor Output Enable
    #[inline(always)]
    pub fn tsoe(&mut self) -> TsoeW<TscrSpec> {
        TsoeW::new(self, 4)
    }
    ///Bit 7 - Temperature Sensor Enable
    #[inline(always)]
    pub fn tsen(&mut self) -> TsenW<TscrSpec> {
        TsenW::new(self, 7)
    }
}
/**Temperature Sensor Control Register

You can [`read`](crate::Reg::read) this register and get [`tscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TscrSpec;
impl crate::RegisterSpec for TscrSpec {
    type Ux = u8;
}
///`read()` method returns [`tscr::R`](R) reader structure
impl crate::Readable for TscrSpec {}
///`write(|w| ..)` method takes [`tscr::W`](W) writer structure
impl crate::Writable for TscrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TSCR to value 0
impl crate::Resettable for TscrSpec {}
