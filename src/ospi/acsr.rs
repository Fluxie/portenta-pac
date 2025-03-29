///Register `ACSR` reader
pub type R = crate::R<AcsrSpec>;
///Register `ACSR` writer
pub type W = crate::W<AcsrSpec>;
/**Auto-calibration status of device 0

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Acsr0 {
    ///0: Initial state
    _000 = 0,
    ///1: Reserved
    _001 = 1,
    ///2: Reserved
    _010 = 2,
    ///3: Normal end
    _011 = 3,
    ///4: Error end
    _100 = 4,
}
impl From<Acsr0> for u8 {
    #[inline(always)]
    fn from(variant: Acsr0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Acsr0 {
    type Ux = u8;
}
impl crate::IsEnum for Acsr0 {}
///Field `ACSR0` reader - Auto-calibration status of device 0
pub type Acsr0R = crate::FieldReader<Acsr0>;
impl Acsr0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<Acsr0> {
        match self.bits {
            0 => Some(Acsr0::_000),
            1 => Some(Acsr0::_001),
            2 => Some(Acsr0::_010),
            3 => Some(Acsr0::_011),
            4 => Some(Acsr0::_100),
            _ => None,
        }
    }
    ///Initial state
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Acsr0::_000
    }
    ///Reserved
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Acsr0::_001
    }
    ///Reserved
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Acsr0::_010
    }
    ///Normal end
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Acsr0::_011
    }
    ///Error end
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Acsr0::_100
    }
}
///Field `ACSR0` writer - Auto-calibration status of device 0
pub type Acsr0W<'a, REG> = crate::FieldWriter<'a, REG, 3, Acsr0>;
impl<'a, REG> Acsr0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Initial state
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Acsr0::_000)
    }
    ///Reserved
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Acsr0::_001)
    }
    ///Reserved
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Acsr0::_010)
    }
    ///Normal end
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Acsr0::_011)
    }
    ///Error end
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Acsr0::_100)
    }
}
/**Auto-calibration status of device 1

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Acsr1 {
    ///0: Initial state
    _000 = 0,
    ///1: Reserved
    _001 = 1,
    ///2: Reserved
    _010 = 2,
    ///3: Normal end
    _011 = 3,
    ///4: Error end
    _100 = 4,
}
impl From<Acsr1> for u8 {
    #[inline(always)]
    fn from(variant: Acsr1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Acsr1 {
    type Ux = u8;
}
impl crate::IsEnum for Acsr1 {}
///Field `ACSR1` reader - Auto-calibration status of device 1
pub type Acsr1R = crate::FieldReader<Acsr1>;
impl Acsr1R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<Acsr1> {
        match self.bits {
            0 => Some(Acsr1::_000),
            1 => Some(Acsr1::_001),
            2 => Some(Acsr1::_010),
            3 => Some(Acsr1::_011),
            4 => Some(Acsr1::_100),
            _ => None,
        }
    }
    ///Initial state
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Acsr1::_000
    }
    ///Reserved
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Acsr1::_001
    }
    ///Reserved
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Acsr1::_010
    }
    ///Normal end
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Acsr1::_011
    }
    ///Error end
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Acsr1::_100
    }
}
///Field `ACSR1` writer - Auto-calibration status of device 1
pub type Acsr1W<'a, REG> = crate::FieldWriter<'a, REG, 3, Acsr1>;
impl<'a, REG> Acsr1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Initial state
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Acsr1::_000)
    }
    ///Reserved
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Acsr1::_001)
    }
    ///Reserved
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Acsr1::_010)
    }
    ///Normal end
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Acsr1::_011)
    }
    ///Error end
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Acsr1::_100)
    }
}
impl R {
    ///Bits 0:2 - Auto-calibration status of device 0
    #[inline(always)]
    pub fn acsr0(&self) -> Acsr0R {
        Acsr0R::new((self.bits & 7) as u8)
    }
    ///Bits 3:5 - Auto-calibration status of device 1
    #[inline(always)]
    pub fn acsr1(&self) -> Acsr1R {
        Acsr1R::new(((self.bits >> 3) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ACSR")
            .field("acsr0", &self.acsr0())
            .field("acsr1", &self.acsr1())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Auto-calibration status of device 0
    #[inline(always)]
    pub fn acsr0(&mut self) -> Acsr0W<AcsrSpec> {
        Acsr0W::new(self, 0)
    }
    ///Bits 3:5 - Auto-calibration status of device 1
    #[inline(always)]
    pub fn acsr1(&mut self) -> Acsr1W<AcsrSpec> {
        Acsr1W::new(self, 3)
    }
}
/**Auto-Calibration Status Register

You can [`read`](crate::Reg::read) this register and get [`acsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AcsrSpec;
impl crate::RegisterSpec for AcsrSpec {
    type Ux = u32;
}
///`read()` method returns [`acsr::R`](R) reader structure
impl crate::Readable for AcsrSpec {}
///`write(|w| ..)` method takes [`acsr::W`](W) writer structure
impl crate::Writable for AcsrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ACSR to value 0
impl crate::Resettable for AcsrSpec {}
