///Register `LVD2SR` reader
pub type R = crate::R<Lvd2srSpec>;
///Register `LVD2SR` writer
pub type W = crate::W<Lvd2srSpec>;
/**Voltage Monitor 2 Voltage Variation Detection Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Det {
    ///0: Not detected
    _0 = 0,
    ///1: Vdet2 crossing is detected
    _1 = 1,
}
impl From<Det> for bool {
    #[inline(always)]
    fn from(variant: Det) -> Self {
        variant as u8 != 0
    }
}
///Field `DET` reader - Voltage Monitor 2 Voltage Variation Detection Flag
pub type DetR = crate::BitReader<Det>;
impl DetR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Det {
        match self.bits {
            false => Det::_0,
            true => Det::_1,
        }
    }
    ///Not detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Det::_0
    }
    ///Vdet2 crossing is detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Det::_1
    }
}
///Field `DET` writer - Voltage Monitor 2 Voltage Variation Detection Flag
pub type DetW<'a, REG> = crate::BitWriter<'a, REG, Det>;
impl<'a, REG> DetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Not detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Det::_0)
    }
    ///Vdet2 crossing is detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Det::_1)
    }
}
/**Voltage Monitor 2 Signal Monitor Flag

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mon {
    ///0: VCC < Vdet2
    _0 = 0,
    ///1: VCC>= Vdet2 or MON is disabled
    _1 = 1,
}
impl From<Mon> for bool {
    #[inline(always)]
    fn from(variant: Mon) -> Self {
        variant as u8 != 0
    }
}
///Field `MON` reader - Voltage Monitor 2 Signal Monitor Flag
pub type MonR = crate::BitReader<Mon>;
impl MonR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Mon {
        match self.bits {
            false => Mon::_0,
            true => Mon::_1,
        }
    }
    ///VCC < Vdet2
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mon::_0
    }
    ///VCC>= Vdet2 or MON is disabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mon::_1
    }
}
impl R {
    ///Bit 0 - Voltage Monitor 2 Voltage Variation Detection Flag
    #[inline(always)]
    pub fn det(&self) -> DetR {
        DetR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Voltage Monitor 2 Signal Monitor Flag
    #[inline(always)]
    pub fn mon(&self) -> MonR {
        MonR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LVD2SR")
            .field("det", &self.det())
            .field("mon", &self.mon())
            .finish()
    }
}
impl W {
    ///Bit 0 - Voltage Monitor 2 Voltage Variation Detection Flag
    #[inline(always)]
    pub fn det(&mut self) -> DetW<Lvd2srSpec> {
        DetW::new(self, 0)
    }
}
/**Voltage Monitor 2 Circuit Status Register

You can [`read`](crate::Reg::read) this register and get [`lvd2sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvd2sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Lvd2srSpec;
impl crate::RegisterSpec for Lvd2srSpec {
    type Ux = u8;
}
///`read()` method returns [`lvd2sr::R`](R) reader structure
impl crate::Readable for Lvd2srSpec {}
///`write(|w| ..)` method takes [`lvd2sr::W`](W) writer structure
impl crate::Writable for Lvd2srSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LVD2SR to value 0x02
impl crate::Resettable for Lvd2srSpec {
    const RESET_VALUE: u8 = 0x02;
}
