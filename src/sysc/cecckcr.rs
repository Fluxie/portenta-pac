///Register `CECCKCR` reader
pub type R = crate::R<CecckcrSpec>;
///Register `CECCKCR` writer
pub type W = crate::W<CecckcrSpec>;
/**CEC clock (CECCLK) Source Select

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ceccksel {
    ///3: Main clock oscillator
    _011 = 3,
    ///4: Sub-clock oscillator
    _100 = 4,
    ///0: Setting prohibited
    Others = 0,
}
impl From<Ceccksel> for u8 {
    #[inline(always)]
    fn from(variant: Ceccksel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ceccksel {
    type Ux = u8;
}
impl crate::IsEnum for Ceccksel {}
///Field `CECCKSEL` reader - CEC clock (CECCLK) Source Select
pub type CecckselR = crate::FieldReader<Ceccksel>;
impl CecckselR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ceccksel {
        match self.bits {
            3 => Ceccksel::_011,
            4 => Ceccksel::_100,
            _ => Ceccksel::Others,
        }
    }
    ///Main clock oscillator
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Ceccksel::_011
    }
    ///Sub-clock oscillator
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Ceccksel::_100
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Ceccksel::Others)
    }
}
///Field `CECCKSEL` writer - CEC clock (CECCLK) Source Select
pub type CecckselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ceccksel, crate::Safe>;
impl<'a, REG> CecckselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Main clock oscillator
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Ceccksel::_011)
    }
    ///Sub-clock oscillator
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Ceccksel::_100)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Ceccksel::Others)
    }
}
/**CEC clock (CECCLK) Switching Request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ceccksreq {
    ///0: No request
    _0 = 0,
    ///1: Request switching
    _1 = 1,
}
impl From<Ceccksreq> for bool {
    #[inline(always)]
    fn from(variant: Ceccksreq) -> Self {
        variant as u8 != 0
    }
}
///Field `CECCKSREQ` reader - CEC clock (CECCLK) Switching Request
pub type CeccksreqR = crate::BitReader<Ceccksreq>;
impl CeccksreqR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ceccksreq {
        match self.bits {
            false => Ceccksreq::_0,
            true => Ceccksreq::_1,
        }
    }
    ///No request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ceccksreq::_0
    }
    ///Request switching
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ceccksreq::_1
    }
}
///Field `CECCKSREQ` writer - CEC clock (CECCLK) Switching Request
pub type CeccksreqW<'a, REG> = crate::BitWriter<'a, REG, Ceccksreq>;
impl<'a, REG> CeccksreqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ceccksreq::_0)
    }
    ///Request switching
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ceccksreq::_1)
    }
}
/**CEC clock (CECCLK) Switching Ready state flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ceccksrdy {
    ///0: Impossible to Switch
    _0 = 0,
    ///1: Possible to Switch
    _1 = 1,
}
impl From<Ceccksrdy> for bool {
    #[inline(always)]
    fn from(variant: Ceccksrdy) -> Self {
        variant as u8 != 0
    }
}
///Field `CECCKSRDY` reader - CEC clock (CECCLK) Switching Ready state flag
pub type CeccksrdyR = crate::BitReader<Ceccksrdy>;
impl CeccksrdyR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ceccksrdy {
        match self.bits {
            false => Ceccksrdy::_0,
            true => Ceccksrdy::_1,
        }
    }
    ///Impossible to Switch
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ceccksrdy::_0
    }
    ///Possible to Switch
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ceccksrdy::_1
    }
}
impl R {
    ///Bits 0:2 - CEC clock (CECCLK) Source Select
    #[inline(always)]
    pub fn ceccksel(&self) -> CecckselR {
        CecckselR::new(self.bits & 7)
    }
    ///Bit 6 - CEC clock (CECCLK) Switching Request
    #[inline(always)]
    pub fn ceccksreq(&self) -> CeccksreqR {
        CeccksreqR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - CEC clock (CECCLK) Switching Ready state flag
    #[inline(always)]
    pub fn ceccksrdy(&self) -> CeccksrdyR {
        CeccksrdyR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CECCKCR")
            .field("ceccksel", &self.ceccksel())
            .field("ceccksreq", &self.ceccksreq())
            .field("ceccksrdy", &self.ceccksrdy())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - CEC clock (CECCLK) Source Select
    #[inline(always)]
    pub fn ceccksel(&mut self) -> CecckselW<CecckcrSpec> {
        CecckselW::new(self, 0)
    }
    ///Bit 6 - CEC clock (CECCLK) Switching Request
    #[inline(always)]
    pub fn ceccksreq(&mut self) -> CeccksreqW<CecckcrSpec> {
        CeccksreqW::new(self, 6)
    }
}
/**CEC Clock Control Register

You can [`read`](crate::Reg::read) this register and get [`cecckcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cecckcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CecckcrSpec;
impl crate::RegisterSpec for CecckcrSpec {
    type Ux = u8;
}
///`read()` method returns [`cecckcr::R`](R) reader structure
impl crate::Readable for CecckcrSpec {}
///`write(|w| ..)` method takes [`cecckcr::W`](W) writer structure
impl crate::Writable for CecckcrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CECCKCR to value 0x01
impl crate::Resettable for CecckcrSpec {
    const RESET_VALUE: u8 = 0x01;
}
