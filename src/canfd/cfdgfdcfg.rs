///Register `CFDGFDCFG` reader
pub type R = crate::R<CfdgfdcfgSpec>;
///Register `CFDGFDCFG` writer
pub type W = crate::W<CfdgfdcfgSpec>;
/**RES Bit Protocol Exception Disable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rped {
    ///0: Protocol exception event detection enabled
    _0 = 0,
    ///1: Protocol exception event detection disabled
    _1 = 1,
}
impl From<Rped> for bool {
    #[inline(always)]
    fn from(variant: Rped) -> Self {
        variant as u8 != 0
    }
}
///Field `RPED` reader - RES Bit Protocol Exception Disable
pub type RpedR = crate::BitReader<Rped>;
impl RpedR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rped {
        match self.bits {
            false => Rped::_0,
            true => Rped::_1,
        }
    }
    ///Protocol exception event detection enabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rped::_0
    }
    ///Protocol exception event detection disabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rped::_1
    }
}
///Field `RPED` writer - RES Bit Protocol Exception Disable
pub type RpedW<'a, REG> = crate::BitWriter<'a, REG, Rped>;
impl<'a, REG> RpedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Protocol exception event detection enabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rped::_0)
    }
    ///Protocol exception event detection disabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rped::_1)
    }
}
/**Timestamp Capture Configuration

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tsccfg {
    ///0: Timestamp capture at the sample point of SOF (start of frame)
    _00 = 0,
    ///1: Timestamp capture at frame valid indication
    _01 = 1,
    ///2: Timestamp capture at the sample point of RES bit
    _10 = 2,
    ///3: Reserved
    _11 = 3,
}
impl From<Tsccfg> for u8 {
    #[inline(always)]
    fn from(variant: Tsccfg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tsccfg {
    type Ux = u8;
}
impl crate::IsEnum for Tsccfg {}
///Field `TSCCFG` reader - Timestamp Capture Configuration
pub type TsccfgR = crate::FieldReader<Tsccfg>;
impl TsccfgR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tsccfg {
        match self.bits {
            0 => Tsccfg::_00,
            1 => Tsccfg::_01,
            2 => Tsccfg::_10,
            3 => Tsccfg::_11,
            _ => unreachable!(),
        }
    }
    ///Timestamp capture at the sample point of SOF (start of frame)
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Tsccfg::_00
    }
    ///Timestamp capture at frame valid indication
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Tsccfg::_01
    }
    ///Timestamp capture at the sample point of RES bit
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Tsccfg::_10
    }
    ///Reserved
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Tsccfg::_11
    }
}
///Field `TSCCFG` writer - Timestamp Capture Configuration
pub type TsccfgW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tsccfg, crate::Safe>;
impl<'a, REG> TsccfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Timestamp capture at the sample point of SOF (start of frame)
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Tsccfg::_00)
    }
    ///Timestamp capture at frame valid indication
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Tsccfg::_01)
    }
    ///Timestamp capture at the sample point of RES bit
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Tsccfg::_10)
    }
    ///Reserved
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Tsccfg::_11)
    }
}
impl R {
    ///Bit 0 - RES Bit Protocol Exception Disable
    #[inline(always)]
    pub fn rped(&self) -> RpedR {
        RpedR::new((self.bits & 1) != 0)
    }
    ///Bits 8:9 - Timestamp Capture Configuration
    #[inline(always)]
    pub fn tsccfg(&self) -> TsccfgR {
        TsccfgR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDGFDCFG")
            .field("rped", &self.rped())
            .field("tsccfg", &self.tsccfg())
            .finish()
    }
}
impl W {
    ///Bit 0 - RES Bit Protocol Exception Disable
    #[inline(always)]
    pub fn rped(&mut self) -> RpedW<CfdgfdcfgSpec> {
        RpedW::new(self, 0)
    }
    ///Bits 8:9 - Timestamp Capture Configuration
    #[inline(always)]
    pub fn tsccfg(&mut self) -> TsccfgW<CfdgfdcfgSpec> {
        TsccfgW::new(self, 8)
    }
}
/**Global FD Configuration Register

You can [`read`](crate::Reg::read) this register and get [`cfdgfdcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdgfdcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdgfdcfgSpec;
impl crate::RegisterSpec for CfdgfdcfgSpec {
    type Ux = u32;
}
///`read()` method returns [`cfdgfdcfg::R`](R) reader structure
impl crate::Readable for CfdgfdcfgSpec {}
///`write(|w| ..)` method takes [`cfdgfdcfg::W`](W) writer structure
impl crate::Writable for CfdgfdcfgSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDGFDCFG to value 0
impl crate::Resettable for CfdgfdcfgSpec {}
