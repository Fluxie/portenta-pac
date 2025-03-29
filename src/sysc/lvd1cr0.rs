///Register `LVD1CR0` reader
pub type R = crate::R<Lvd1cr0Spec>;
///Register `LVD1CR0` writer
pub type W = crate::W<Lvd1cr0Spec>;
/**Voltage Monitor 1 Interrupt/Reset Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rie {
    ///0: Disable
    _0 = 0,
    ///1: Enable
    _1 = 1,
}
impl From<Rie> for bool {
    #[inline(always)]
    fn from(variant: Rie) -> Self {
        variant as u8 != 0
    }
}
///Field `RIE` reader - Voltage Monitor 1 Interrupt/Reset Enable
pub type RieR = crate::BitReader<Rie>;
impl RieR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rie {
        match self.bits {
            false => Rie::_0,
            true => Rie::_1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rie::_0
    }
    ///Enable
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rie::_1
    }
}
///Field `RIE` writer - Voltage Monitor 1 Interrupt/Reset Enable
pub type RieW<'a, REG> = crate::BitWriter<'a, REG, Rie>;
impl<'a, REG> RieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rie::_0)
    }
    ///Enable
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rie::_1)
    }
}
/**Voltage monitor 1 Digital Filter Disabled Mode Select

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dfdis {
    ///0: Enable the digital filter
    _0 = 0,
    ///1: Disable the digital filter
    _1 = 1,
}
impl From<Dfdis> for bool {
    #[inline(always)]
    fn from(variant: Dfdis) -> Self {
        variant as u8 != 0
    }
}
///Field `DFDIS` reader - Voltage monitor 1 Digital Filter Disabled Mode Select
pub type DfdisR = crate::BitReader<Dfdis>;
impl DfdisR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dfdis {
        match self.bits {
            false => Dfdis::_0,
            true => Dfdis::_1,
        }
    }
    ///Enable the digital filter
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dfdis::_0
    }
    ///Disable the digital filter
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dfdis::_1
    }
}
///Field `DFDIS` writer - Voltage monitor 1 Digital Filter Disabled Mode Select
pub type DfdisW<'a, REG> = crate::BitWriter<'a, REG, Dfdis>;
impl<'a, REG> DfdisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Enable the digital filter
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dfdis::_0)
    }
    ///Disable the digital filter
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dfdis::_1)
    }
}
/**Voltage Monitor 1 Circuit Comparison Result Output Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpe {
    ///0: Disable voltage monitor 1 circuit comparison result output
    _0 = 0,
    ///1: Enable voltage monitor 1 circuit comparison result output
    _1 = 1,
}
impl From<Cmpe> for bool {
    #[inline(always)]
    fn from(variant: Cmpe) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPE` reader - Voltage Monitor 1 Circuit Comparison Result Output Enable
pub type CmpeR = crate::BitReader<Cmpe>;
impl CmpeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cmpe {
        match self.bits {
            false => Cmpe::_0,
            true => Cmpe::_1,
        }
    }
    ///Disable voltage monitor 1 circuit comparison result output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpe::_0
    }
    ///Enable voltage monitor 1 circuit comparison result output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpe::_1
    }
}
///Field `CMPE` writer - Voltage Monitor 1 Circuit Comparison Result Output Enable
pub type CmpeW<'a, REG> = crate::BitWriter<'a, REG, Cmpe>;
impl<'a, REG> CmpeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable voltage monitor 1 circuit comparison result output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpe::_0)
    }
    ///Enable voltage monitor 1 circuit comparison result output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpe::_1)
    }
}
/**Sampling Clock Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fsamp {
    ///0: 1/2 LOCO frequency
    _00 = 0,
    ///1: 1/4 LOCO frequency
    _01 = 1,
    ///2: 1/8 LOCO frequency
    _10 = 2,
    ///3: 1/16 LOCO frequency
    _11 = 3,
}
impl From<Fsamp> for u8 {
    #[inline(always)]
    fn from(variant: Fsamp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fsamp {
    type Ux = u8;
}
impl crate::IsEnum for Fsamp {}
///Field `FSAMP` reader - Sampling Clock Select
pub type FsampR = crate::FieldReader<Fsamp>;
impl FsampR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Fsamp {
        match self.bits {
            0 => Fsamp::_00,
            1 => Fsamp::_01,
            2 => Fsamp::_10,
            3 => Fsamp::_11,
            _ => unreachable!(),
        }
    }
    ///1/2 LOCO frequency
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Fsamp::_00
    }
    ///1/4 LOCO frequency
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Fsamp::_01
    }
    ///1/8 LOCO frequency
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Fsamp::_10
    }
    ///1/16 LOCO frequency
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Fsamp::_11
    }
}
///Field `FSAMP` writer - Sampling Clock Select
pub type FsampW<'a, REG> = crate::FieldWriter<'a, REG, 2, Fsamp, crate::Safe>;
impl<'a, REG> FsampW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///1/2 LOCO frequency
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Fsamp::_00)
    }
    ///1/4 LOCO frequency
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Fsamp::_01)
    }
    ///1/8 LOCO frequency
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Fsamp::_10)
    }
    ///1/16 LOCO frequency
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Fsamp::_11)
    }
}
/**Voltage Monitor 1 Circuit Mode Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ri {
    ///0: Generate voltage monitor 1 interrupt on Vdet1 crossing
    _0 = 0,
    ///1: Enable voltage monitor 1 reset when the voltage falls to and below Vdet1
    _1 = 1,
}
impl From<Ri> for bool {
    #[inline(always)]
    fn from(variant: Ri) -> Self {
        variant as u8 != 0
    }
}
///Field `RI` reader - Voltage Monitor 1 Circuit Mode Select
pub type RiR = crate::BitReader<Ri>;
impl RiR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ri {
        match self.bits {
            false => Ri::_0,
            true => Ri::_1,
        }
    }
    ///Generate voltage monitor 1 interrupt on Vdet1 crossing
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ri::_0
    }
    ///Enable voltage monitor 1 reset when the voltage falls to and below Vdet1
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ri::_1
    }
}
///Field `RI` writer - Voltage Monitor 1 Circuit Mode Select
pub type RiW<'a, REG> = crate::BitWriter<'a, REG, Ri>;
impl<'a, REG> RiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Generate voltage monitor 1 interrupt on Vdet1 crossing
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ri::_0)
    }
    ///Enable voltage monitor 1 reset when the voltage falls to and below Vdet1
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ri::_1)
    }
}
/**Voltage Monitor 1 Reset Negate Select

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rn {
    ///0: Negate after a stabilization time (tLVD1) when VCC > Vdet1 is detected
    _0 = 0,
    ///1: Negate after a stabilization time (tLVD1) on assertion of the LVD1 reset
    _1 = 1,
}
impl From<Rn> for bool {
    #[inline(always)]
    fn from(variant: Rn) -> Self {
        variant as u8 != 0
    }
}
///Field `RN` reader - Voltage Monitor 1 Reset Negate Select
pub type RnR = crate::BitReader<Rn>;
impl RnR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rn {
        match self.bits {
            false => Rn::_0,
            true => Rn::_1,
        }
    }
    ///Negate after a stabilization time (tLVD1) when VCC > Vdet1 is detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rn::_0
    }
    ///Negate after a stabilization time (tLVD1) on assertion of the LVD1 reset
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rn::_1
    }
}
///Field `RN` writer - Voltage Monitor 1 Reset Negate Select
pub type RnW<'a, REG> = crate::BitWriter<'a, REG, Rn>;
impl<'a, REG> RnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Negate after a stabilization time (tLVD1) when VCC > Vdet1 is detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rn::_0)
    }
    ///Negate after a stabilization time (tLVD1) on assertion of the LVD1 reset
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rn::_1)
    }
}
impl R {
    ///Bit 0 - Voltage Monitor 1 Interrupt/Reset Enable
    #[inline(always)]
    pub fn rie(&self) -> RieR {
        RieR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Voltage monitor 1 Digital Filter Disabled Mode Select
    #[inline(always)]
    pub fn dfdis(&self) -> DfdisR {
        DfdisR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Voltage Monitor 1 Circuit Comparison Result Output Enable
    #[inline(always)]
    pub fn cmpe(&self) -> CmpeR {
        CmpeR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 4:5 - Sampling Clock Select
    #[inline(always)]
    pub fn fsamp(&self) -> FsampR {
        FsampR::new((self.bits >> 4) & 3)
    }
    ///Bit 6 - Voltage Monitor 1 Circuit Mode Select
    #[inline(always)]
    pub fn ri(&self) -> RiR {
        RiR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Voltage Monitor 1 Reset Negate Select
    #[inline(always)]
    pub fn rn(&self) -> RnR {
        RnR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LVD1CR0")
            .field("rie", &self.rie())
            .field("dfdis", &self.dfdis())
            .field("cmpe", &self.cmpe())
            .field("fsamp", &self.fsamp())
            .field("ri", &self.ri())
            .field("rn", &self.rn())
            .finish()
    }
}
impl W {
    ///Bit 0 - Voltage Monitor 1 Interrupt/Reset Enable
    #[inline(always)]
    pub fn rie(&mut self) -> RieW<Lvd1cr0Spec> {
        RieW::new(self, 0)
    }
    ///Bit 1 - Voltage monitor 1 Digital Filter Disabled Mode Select
    #[inline(always)]
    pub fn dfdis(&mut self) -> DfdisW<Lvd1cr0Spec> {
        DfdisW::new(self, 1)
    }
    ///Bit 2 - Voltage Monitor 1 Circuit Comparison Result Output Enable
    #[inline(always)]
    pub fn cmpe(&mut self) -> CmpeW<Lvd1cr0Spec> {
        CmpeW::new(self, 2)
    }
    ///Bits 4:5 - Sampling Clock Select
    #[inline(always)]
    pub fn fsamp(&mut self) -> FsampW<Lvd1cr0Spec> {
        FsampW::new(self, 4)
    }
    ///Bit 6 - Voltage Monitor 1 Circuit Mode Select
    #[inline(always)]
    pub fn ri(&mut self) -> RiW<Lvd1cr0Spec> {
        RiW::new(self, 6)
    }
    ///Bit 7 - Voltage Monitor 1 Reset Negate Select
    #[inline(always)]
    pub fn rn(&mut self) -> RnW<Lvd1cr0Spec> {
        RnW::new(self, 7)
    }
}
/**Voltage Monitor 1 Circuit Control Register 0

You can [`read`](crate::Reg::read) this register and get [`lvd1cr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvd1cr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Lvd1cr0Spec;
impl crate::RegisterSpec for Lvd1cr0Spec {
    type Ux = u8;
}
///`read()` method returns [`lvd1cr0::R`](R) reader structure
impl crate::Readable for Lvd1cr0Spec {}
///`write(|w| ..)` method takes [`lvd1cr0::W`](W) writer structure
impl crate::Writable for Lvd1cr0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LVD1CR0 to value 0x82
impl crate::Resettable for Lvd1cr0Spec {
    const RESET_VALUE: u8 = 0x82;
}
