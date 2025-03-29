///Register `CR2` reader
pub type R = crate::R<Cr2Spec>;
///Register `CR2` writer
pub type W = crate::W<Cr2Spec>;
/**RXDXn Signal Digital Filter Clock Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dfcs {
    ///0: Filter is disabled.
    _000 = 0,
    ///1: Filter clock is SCI base clock
    _001 = 1,
    ///2: Filter clock is PCLK/8
    _010 = 2,
    ///3: Filter clock is PCLK/16
    _011 = 3,
    ///4: Filter clock is PCLK/32
    _100 = 4,
    ///5: Filter clock is PCLK/64
    _101 = 5,
    ///6: Filter clock is PCLK/128
    _110 = 6,
    ///7: Setting prohibited
    _111 = 7,
}
impl From<Dfcs> for u8 {
    #[inline(always)]
    fn from(variant: Dfcs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dfcs {
    type Ux = u8;
}
impl crate::IsEnum for Dfcs {}
///Field `DFCS` reader - RXDXn Signal Digital Filter Clock Select
pub type DfcsR = crate::FieldReader<Dfcs>;
impl DfcsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dfcs {
        match self.bits {
            0 => Dfcs::_000,
            1 => Dfcs::_001,
            2 => Dfcs::_010,
            3 => Dfcs::_011,
            4 => Dfcs::_100,
            5 => Dfcs::_101,
            6 => Dfcs::_110,
            7 => Dfcs::_111,
            _ => unreachable!(),
        }
    }
    ///Filter is disabled.
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Dfcs::_000
    }
    ///Filter clock is SCI base clock
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Dfcs::_001
    }
    ///Filter clock is PCLK/8
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Dfcs::_010
    }
    ///Filter clock is PCLK/16
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Dfcs::_011
    }
    ///Filter clock is PCLK/32
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Dfcs::_100
    }
    ///Filter clock is PCLK/64
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Dfcs::_101
    }
    ///Filter clock is PCLK/128
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Dfcs::_110
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == Dfcs::_111
    }
}
///Field `DFCS` writer - RXDXn Signal Digital Filter Clock Select
pub type DfcsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Dfcs, crate::Safe>;
impl<'a, REG> DfcsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Filter is disabled.
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Dfcs::_000)
    }
    ///Filter clock is SCI base clock
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Dfcs::_001)
    }
    ///Filter clock is PCLK/8
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Dfcs::_010)
    }
    ///Filter clock is PCLK/16
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Dfcs::_011)
    }
    ///Filter clock is PCLK/32
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Dfcs::_100)
    }
    ///Filter clock is PCLK/64
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Dfcs::_101)
    }
    ///Filter clock is PCLK/128
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Dfcs::_110)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(Dfcs::_111)
    }
}
/**Bus Collision Detection Clock Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bccs {
    ///0: SCI base clock
    _00 = 0,
    ///1: SCI base clock frequency divided by 2
    _01 = 1,
    ///2: SCI base clock frequency divided by 4
    _10 = 2,
    ///3: Setting prohibited
    _11 = 3,
}
impl From<Bccs> for u8 {
    #[inline(always)]
    fn from(variant: Bccs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bccs {
    type Ux = u8;
}
impl crate::IsEnum for Bccs {}
///Field `BCCS` reader - Bus Collision Detection Clock Select
pub type BccsR = crate::FieldReader<Bccs>;
impl BccsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Bccs {
        match self.bits {
            0 => Bccs::_00,
            1 => Bccs::_01,
            2 => Bccs::_10,
            3 => Bccs::_11,
            _ => unreachable!(),
        }
    }
    ///SCI base clock
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Bccs::_00
    }
    ///SCI base clock frequency divided by 2
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Bccs::_01
    }
    ///SCI base clock frequency divided by 4
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Bccs::_10
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Bccs::_11
    }
}
///Field `BCCS` writer - Bus Collision Detection Clock Select
pub type BccsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Bccs, crate::Safe>;
impl<'a, REG> BccsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///SCI base clock
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Bccs::_00)
    }
    ///SCI base clock frequency divided by 2
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Bccs::_01)
    }
    ///SCI base clock frequency divided by 4
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Bccs::_10)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Bccs::_11)
    }
}
/**RXDXn Reception Sampling Timing Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rts {
    ///0: Rising edge of the 8th cycle of SCI base clock
    _00 = 0,
    ///1: Rising edge of the 10th cycle of SCI base clock
    _01 = 1,
    ///2: Rising edge of the 12th cycle of SCI base clock
    _10 = 2,
    ///3: Rising edge of the 14th cycle of SCI base clock
    _11 = 3,
}
impl From<Rts> for u8 {
    #[inline(always)]
    fn from(variant: Rts) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rts {
    type Ux = u8;
}
impl crate::IsEnum for Rts {}
///Field `RTS` reader - RXDXn Reception Sampling Timing Select
pub type RtsR = crate::FieldReader<Rts>;
impl RtsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rts {
        match self.bits {
            0 => Rts::_00,
            1 => Rts::_01,
            2 => Rts::_10,
            3 => Rts::_11,
            _ => unreachable!(),
        }
    }
    ///Rising edge of the 8th cycle of SCI base clock
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Rts::_00
    }
    ///Rising edge of the 10th cycle of SCI base clock
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Rts::_01
    }
    ///Rising edge of the 12th cycle of SCI base clock
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Rts::_10
    }
    ///Rising edge of the 14th cycle of SCI base clock
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Rts::_11
    }
}
///Field `RTS` writer - RXDXn Reception Sampling Timing Select
pub type RtsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Rts, crate::Safe>;
impl<'a, REG> RtsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Rising edge of the 8th cycle of SCI base clock
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Rts::_00)
    }
    ///Rising edge of the 10th cycle of SCI base clock
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Rts::_01)
    }
    ///Rising edge of the 12th cycle of SCI base clock
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Rts::_10)
    }
    ///Rising edge of the 14th cycle of SCI base clock
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Rts::_11)
    }
}
impl R {
    ///Bits 0:2 - RXDXn Signal Digital Filter Clock Select
    #[inline(always)]
    pub fn dfcs(&self) -> DfcsR {
        DfcsR::new(self.bits & 7)
    }
    ///Bits 4:5 - Bus Collision Detection Clock Select
    #[inline(always)]
    pub fn bccs(&self) -> BccsR {
        BccsR::new((self.bits >> 4) & 3)
    }
    ///Bits 6:7 - RXDXn Reception Sampling Timing Select
    #[inline(always)]
    pub fn rts(&self) -> RtsR {
        RtsR::new((self.bits >> 6) & 3)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR2")
            .field("dfcs", &self.dfcs())
            .field("bccs", &self.bccs())
            .field("rts", &self.rts())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - RXDXn Signal Digital Filter Clock Select
    #[inline(always)]
    pub fn dfcs(&mut self) -> DfcsW<Cr2Spec> {
        DfcsW::new(self, 0)
    }
    ///Bits 4:5 - Bus Collision Detection Clock Select
    #[inline(always)]
    pub fn bccs(&mut self) -> BccsW<Cr2Spec> {
        BccsW::new(self, 4)
    }
    ///Bits 6:7 - RXDXn Reception Sampling Timing Select
    #[inline(always)]
    pub fn rts(&mut self) -> RtsW<Cr2Spec> {
        RtsW::new(self, 6)
    }
}
/**Control Register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Cr2Spec;
impl crate::RegisterSpec for Cr2Spec {
    type Ux = u8;
}
///`read()` method returns [`cr2::R`](R) reader structure
impl crate::Readable for Cr2Spec {}
///`write(|w| ..)` method takes [`cr2::W`](W) writer structure
impl crate::Writable for Cr2Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR2 to value 0
impl crate::Resettable for Cr2Spec {}
