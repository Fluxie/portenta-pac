///Register `DPSWCR` reader
pub type R = crate::R<DpswcrSpec>;
///Register `DPSWCR` writer
pub type W = crate::W<DpswcrSpec>;
/**Deep Software Wait Standby Time Setting Bit

Value on reset: 25*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wtsts {
    ///14: Wait cycle for fast recovery
    _0x0e = 14,
    ///25: Wait cycle for slow recovery
    _0x19 = 25,
    ///0: Setting prohibited
    Others = 0,
}
impl From<Wtsts> for u8 {
    #[inline(always)]
    fn from(variant: Wtsts) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wtsts {
    type Ux = u8;
}
impl crate::IsEnum for Wtsts {}
///Field `WTSTS` reader - Deep Software Wait Standby Time Setting Bit
pub type WtstsR = crate::FieldReader<Wtsts>;
impl WtstsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Wtsts {
        match self.bits {
            14 => Wtsts::_0x0e,
            25 => Wtsts::_0x19,
            _ => Wtsts::Others,
        }
    }
    ///Wait cycle for fast recovery
    #[inline(always)]
    pub fn is_0x0e(&self) -> bool {
        *self == Wtsts::_0x0e
    }
    ///Wait cycle for slow recovery
    #[inline(always)]
    pub fn is_0x19(&self) -> bool {
        *self == Wtsts::_0x19
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Wtsts::Others)
    }
}
///Field `WTSTS` writer - Deep Software Wait Standby Time Setting Bit
pub type WtstsW<'a, REG> = crate::FieldWriter<'a, REG, 6, Wtsts, crate::Safe>;
impl<'a, REG> WtstsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Wait cycle for fast recovery
    #[inline(always)]
    pub fn _0x0e(self) -> &'a mut crate::W<REG> {
        self.variant(Wtsts::_0x0e)
    }
    ///Wait cycle for slow recovery
    #[inline(always)]
    pub fn _0x19(self) -> &'a mut crate::W<REG> {
        self.variant(Wtsts::_0x19)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Wtsts::Others)
    }
}
impl R {
    ///Bits 0:5 - Deep Software Wait Standby Time Setting Bit
    #[inline(always)]
    pub fn wtsts(&self) -> WtstsR {
        WtstsR::new(self.bits & 0x3f)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DPSWCR").field("wtsts", &self.wtsts()).finish()
    }
}
impl W {
    ///Bits 0:5 - Deep Software Wait Standby Time Setting Bit
    #[inline(always)]
    pub fn wtsts(&mut self) -> WtstsW<DpswcrSpec> {
        WtstsW::new(self, 0)
    }
}
/**Deep Software Standby Wait Control Register

You can [`read`](crate::Reg::read) this register and get [`dpswcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpswcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DpswcrSpec;
impl crate::RegisterSpec for DpswcrSpec {
    type Ux = u8;
}
///`read()` method returns [`dpswcr::R`](R) reader structure
impl crate::Readable for DpswcrSpec {}
///`write(|w| ..)` method takes [`dpswcr::W`](W) writer structure
impl crate::Writable for DpswcrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DPSWCR to value 0x19
impl crate::Resettable for DpswcrSpec {
    const RESET_VALUE: u8 = 0x19;
}
