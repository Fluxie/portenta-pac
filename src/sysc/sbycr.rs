///Register `SBYCR` reader
pub type R = crate::R<SbycrSpec>;
///Register `SBYCR` writer
pub type W = crate::W<SbycrSpec>;
/**Output Port Enable

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ope {
    ///0: In Software Standby mode or Deep Software Standby mode, set the address bus and other bus control signal to the high-impedance state. In snooze mode, the status of the address bus and bus control signals are same as before entering Software Standby mode.
    _0 = 0,
    ///1: In Software Standby mode or Deep Software Standby mode, address bus and other bus control signal retain the output state.
    _1 = 1,
}
impl From<Ope> for bool {
    #[inline(always)]
    fn from(variant: Ope) -> Self {
        variant as u8 != 0
    }
}
///Field `OPE` reader - Output Port Enable
pub type OpeR = crate::BitReader<Ope>;
impl OpeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ope {
        match self.bits {
            false => Ope::_0,
            true => Ope::_1,
        }
    }
    ///In Software Standby mode or Deep Software Standby mode, set the address bus and other bus control signal to the high-impedance state. In snooze mode, the status of the address bus and bus control signals are same as before entering Software Standby mode.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ope::_0
    }
    ///In Software Standby mode or Deep Software Standby mode, address bus and other bus control signal retain the output state.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ope::_1
    }
}
///Field `OPE` writer - Output Port Enable
pub type OpeW<'a, REG> = crate::BitWriter<'a, REG, Ope>;
impl<'a, REG> OpeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///In Software Standby mode or Deep Software Standby mode, set the address bus and other bus control signal to the high-impedance state. In snooze mode, the status of the address bus and bus control signals are same as before entering Software Standby mode.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ope::_0)
    }
    ///In Software Standby mode or Deep Software Standby mode, address bus and other bus control signal retain the output state.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ope::_1)
    }
}
/**Software Standby Mode Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ssby {
    ///0: Sleep mode
    _0 = 0,
    ///1: Software Standby mode.
    _1 = 1,
}
impl From<Ssby> for bool {
    #[inline(always)]
    fn from(variant: Ssby) -> Self {
        variant as u8 != 0
    }
}
///Field `SSBY` reader - Software Standby Mode Select
pub type SsbyR = crate::BitReader<Ssby>;
impl SsbyR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ssby {
        match self.bits {
            false => Ssby::_0,
            true => Ssby::_1,
        }
    }
    ///Sleep mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ssby::_0
    }
    ///Software Standby mode.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ssby::_1
    }
}
///Field `SSBY` writer - Software Standby Mode Select
pub type SsbyW<'a, REG> = crate::BitWriter<'a, REG, Ssby>;
impl<'a, REG> SsbyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Sleep mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ssby::_0)
    }
    ///Software Standby mode.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ssby::_1)
    }
}
impl R {
    ///Bit 14 - Output Port Enable
    #[inline(always)]
    pub fn ope(&self) -> OpeR {
        OpeR::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Software Standby Mode Select
    #[inline(always)]
    pub fn ssby(&self) -> SsbyR {
        SsbyR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SBYCR")
            .field("ope", &self.ope())
            .field("ssby", &self.ssby())
            .finish()
    }
}
impl W {
    ///Bit 14 - Output Port Enable
    #[inline(always)]
    pub fn ope(&mut self) -> OpeW<SbycrSpec> {
        OpeW::new(self, 14)
    }
    ///Bit 15 - Software Standby Mode Select
    #[inline(always)]
    pub fn ssby(&mut self) -> SsbyW<SbycrSpec> {
        SsbyW::new(self, 15)
    }
}
/**Standby Control Register

You can [`read`](crate::Reg::read) this register and get [`sbycr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbycr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SbycrSpec;
impl crate::RegisterSpec for SbycrSpec {
    type Ux = u16;
}
///`read()` method returns [`sbycr::R`](R) reader structure
impl crate::Readable for SbycrSpec {}
///`write(|w| ..)` method takes [`sbycr::W`](W) writer structure
impl crate::Writable for SbycrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SBYCR to value 0x4000
impl crate::Resettable for SbycrSpec {
    const RESET_VALUE: u16 = 0x4000;
}
