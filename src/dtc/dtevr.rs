///Register `DTEVR` reader
pub type R = crate::R<DtevrSpec>;
///Register `DTEVR` writer
pub type W = crate::W<DtevrSpec>;
///Field `DTEV` reader - DTC Error Vector Number
pub type DtevR = crate::FieldReader;
/**DTC Error Vector Number SA Monitor

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dtevsam {
    ///0: Secure vector number
    _0 = 0,
    ///1: Non-Secure vector number
    _1 = 1,
}
impl From<Dtevsam> for bool {
    #[inline(always)]
    fn from(variant: Dtevsam) -> Self {
        variant as u8 != 0
    }
}
///Field `DTEVSAM` reader - DTC Error Vector Number SA Monitor
pub type DtevsamR = crate::BitReader<Dtevsam>;
impl DtevsamR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dtevsam {
        match self.bits {
            false => Dtevsam::_0,
            true => Dtevsam::_1,
        }
    }
    ///Secure vector number
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dtevsam::_0
    }
    ///Non-Secure vector number
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dtevsam::_1
    }
}
/**DTC Error Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dtesta {
    ///0: No DTC transfer error occurred
    _0 = 0,
    ///1: DTC transfer error occurred
    _1 = 1,
}
impl From<Dtesta> for bool {
    #[inline(always)]
    fn from(variant: Dtesta) -> Self {
        variant as u8 != 0
    }
}
///Field `DTESTA` reader - DTC Error Status Flag
pub type DtestaR = crate::BitReader<Dtesta>;
impl DtestaR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dtesta {
        match self.bits {
            false => Dtesta::_0,
            true => Dtesta::_1,
        }
    }
    ///No DTC transfer error occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dtesta::_0
    }
    ///DTC transfer error occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dtesta::_1
    }
}
///Field `DTESTA` writer - DTC Error Status Flag
pub type DtestaW<'a, REG> = crate::BitWriter<'a, REG, Dtesta>;
impl<'a, REG> DtestaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No DTC transfer error occurred
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dtesta::_0)
    }
    ///DTC transfer error occurred
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dtesta::_1)
    }
}
impl R {
    ///Bits 0:7 - DTC Error Vector Number
    #[inline(always)]
    pub fn dtev(&self) -> DtevR {
        DtevR::new((self.bits & 0xff) as u8)
    }
    ///Bit 8 - DTC Error Vector Number SA Monitor
    #[inline(always)]
    pub fn dtevsam(&self) -> DtevsamR {
        DtevsamR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 16 - DTC Error Status Flag
    #[inline(always)]
    pub fn dtesta(&self) -> DtestaR {
        DtestaR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DTEVR")
            .field("dtev", &self.dtev())
            .field("dtevsam", &self.dtevsam())
            .field("dtesta", &self.dtesta())
            .finish()
    }
}
impl W {
    ///Bit 16 - DTC Error Status Flag
    #[inline(always)]
    pub fn dtesta(&mut self) -> DtestaW<DtevrSpec> {
        DtestaW::new(self, 16)
    }
}
/**DTC Error Vector Register

You can [`read`](crate::Reg::read) this register and get [`dtevr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtevr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DtevrSpec;
impl crate::RegisterSpec for DtevrSpec {
    type Ux = u32;
}
///`read()` method returns [`dtevr::R`](R) reader structure
impl crate::Readable for DtevrSpec {}
///`write(|w| ..)` method takes [`dtevr::W`](W) writer structure
impl crate::Writable for DtevrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DTEVR to value 0
impl crate::Resettable for DtevrSpec {}
