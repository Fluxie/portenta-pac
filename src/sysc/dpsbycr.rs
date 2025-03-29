///Register `DPSBYCR` reader
pub type R = crate::R<DpsbycrSpec>;
///Register `DPSBYCR` writer
pub type W = crate::W<DpsbycrSpec>;
/**Power-Supply Control

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Deepcut {
    ///0: Power to the standby RAM, Low-speed on-chip oscillator, AGTn (n = 0 to 3), and USBFS/USBHS resume detecting unit is supplied in Deep Software Standby mode.
    _00 = 0,
    ///1: Power to the standby RAM, Low-speed on-chip oscillator, AGT, and USBFS/USBHS resume detecting unit is not supplied in Deep Software Standby mode.
    _01 = 1,
    ///2: Setting prohibited
    _10 = 2,
    ///3: Power to the standby RAM, Low-speed on-chip oscillator, AGT, and USBFS/USBHS resume detecting unit is not supplied in Deep Software Standby mode. In addition, LVD is disabled and the low power function in a power-on reset circuit is enabled.
    _11 = 3,
}
impl From<Deepcut> for u8 {
    #[inline(always)]
    fn from(variant: Deepcut) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Deepcut {
    type Ux = u8;
}
impl crate::IsEnum for Deepcut {}
///Field `DEEPCUT` reader - Power-Supply Control
pub type DeepcutR = crate::FieldReader<Deepcut>;
impl DeepcutR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Deepcut {
        match self.bits {
            0 => Deepcut::_00,
            1 => Deepcut::_01,
            2 => Deepcut::_10,
            3 => Deepcut::_11,
            _ => unreachable!(),
        }
    }
    ///Power to the standby RAM, Low-speed on-chip oscillator, AGTn (n = 0 to 3), and USBFS/USBHS resume detecting unit is supplied in Deep Software Standby mode.
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Deepcut::_00
    }
    ///Power to the standby RAM, Low-speed on-chip oscillator, AGT, and USBFS/USBHS resume detecting unit is not supplied in Deep Software Standby mode.
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Deepcut::_01
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Deepcut::_10
    }
    ///Power to the standby RAM, Low-speed on-chip oscillator, AGT, and USBFS/USBHS resume detecting unit is not supplied in Deep Software Standby mode. In addition, LVD is disabled and the low power function in a power-on reset circuit is enabled.
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Deepcut::_11
    }
}
///Field `DEEPCUT` writer - Power-Supply Control
pub type DeepcutW<'a, REG> = crate::FieldWriter<'a, REG, 2, Deepcut, crate::Safe>;
impl<'a, REG> DeepcutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Power to the standby RAM, Low-speed on-chip oscillator, AGTn (n = 0 to 3), and USBFS/USBHS resume detecting unit is supplied in Deep Software Standby mode.
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Deepcut::_00)
    }
    ///Power to the standby RAM, Low-speed on-chip oscillator, AGT, and USBFS/USBHS resume detecting unit is not supplied in Deep Software Standby mode.
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Deepcut::_01)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Deepcut::_10)
    }
    ///Power to the standby RAM, Low-speed on-chip oscillator, AGT, and USBFS/USBHS resume detecting unit is not supplied in Deep Software Standby mode. In addition, LVD is disabled and the low power function in a power-on reset circuit is enabled.
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Deepcut::_11)
    }
}
/**I/O Port Retention

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iokeep {
    ///0: When the Deep Software Standby mode is canceled, the I/O ports are in the reset state.
    _0 = 0,
    ///1: When the Deep Software Standby mode is canceled, the I/O ports are in the same state as in the Deep Software Standby mode.
    _1 = 1,
}
impl From<Iokeep> for bool {
    #[inline(always)]
    fn from(variant: Iokeep) -> Self {
        variant as u8 != 0
    }
}
///Field `IOKEEP` reader - I/O Port Retention
pub type IokeepR = crate::BitReader<Iokeep>;
impl IokeepR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Iokeep {
        match self.bits {
            false => Iokeep::_0,
            true => Iokeep::_1,
        }
    }
    ///When the Deep Software Standby mode is canceled, the I/O ports are in the reset state.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Iokeep::_0
    }
    ///When the Deep Software Standby mode is canceled, the I/O ports are in the same state as in the Deep Software Standby mode.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Iokeep::_1
    }
}
///Field `IOKEEP` writer - I/O Port Retention
pub type IokeepW<'a, REG> = crate::BitWriter<'a, REG, Iokeep>;
impl<'a, REG> IokeepW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///When the Deep Software Standby mode is canceled, the I/O ports are in the reset state.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Iokeep::_0)
    }
    ///When the Deep Software Standby mode is canceled, the I/O ports are in the same state as in the Deep Software Standby mode.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Iokeep::_1)
    }
}
/**Deep Software Standby

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dpsby {
    ///0: Sleep mode (SBYCR.SSBY=0) / Software Standby mode (SBYCR.SSBY=1)
    _0 = 0,
    ///1: Sleep mode (SBYCR.SSBY=0) / Deep Software Standby mode (SBYCR.SSBY=1)
    _1 = 1,
}
impl From<Dpsby> for bool {
    #[inline(always)]
    fn from(variant: Dpsby) -> Self {
        variant as u8 != 0
    }
}
///Field `DPSBY` reader - Deep Software Standby
pub type DpsbyR = crate::BitReader<Dpsby>;
impl DpsbyR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dpsby {
        match self.bits {
            false => Dpsby::_0,
            true => Dpsby::_1,
        }
    }
    ///Sleep mode (SBYCR.SSBY=0) / Software Standby mode (SBYCR.SSBY=1)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dpsby::_0
    }
    ///Sleep mode (SBYCR.SSBY=0) / Deep Software Standby mode (SBYCR.SSBY=1)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dpsby::_1
    }
}
///Field `DPSBY` writer - Deep Software Standby
pub type DpsbyW<'a, REG> = crate::BitWriter<'a, REG, Dpsby>;
impl<'a, REG> DpsbyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Sleep mode (SBYCR.SSBY=0) / Software Standby mode (SBYCR.SSBY=1)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dpsby::_0)
    }
    ///Sleep mode (SBYCR.SSBY=0) / Deep Software Standby mode (SBYCR.SSBY=1)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dpsby::_1)
    }
}
impl R {
    ///Bits 0:1 - Power-Supply Control
    #[inline(always)]
    pub fn deepcut(&self) -> DeepcutR {
        DeepcutR::new(self.bits & 3)
    }
    ///Bit 6 - I/O Port Retention
    #[inline(always)]
    pub fn iokeep(&self) -> IokeepR {
        IokeepR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Deep Software Standby
    #[inline(always)]
    pub fn dpsby(&self) -> DpsbyR {
        DpsbyR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DPSBYCR")
            .field("deepcut", &self.deepcut())
            .field("iokeep", &self.iokeep())
            .field("dpsby", &self.dpsby())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Power-Supply Control
    #[inline(always)]
    pub fn deepcut(&mut self) -> DeepcutW<DpsbycrSpec> {
        DeepcutW::new(self, 0)
    }
    ///Bit 6 - I/O Port Retention
    #[inline(always)]
    pub fn iokeep(&mut self) -> IokeepW<DpsbycrSpec> {
        IokeepW::new(self, 6)
    }
    ///Bit 7 - Deep Software Standby
    #[inline(always)]
    pub fn dpsby(&mut self) -> DpsbyW<DpsbycrSpec> {
        DpsbyW::new(self, 7)
    }
}
/**Deep Software Standby Control Register

You can [`read`](crate::Reg::read) this register and get [`dpsbycr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpsbycr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DpsbycrSpec;
impl crate::RegisterSpec for DpsbycrSpec {
    type Ux = u8;
}
///`read()` method returns [`dpsbycr::R`](R) reader structure
impl crate::Readable for DpsbycrSpec {}
///`write(|w| ..)` method takes [`dpsbycr::W`](W) writer structure
impl crate::Writable for DpsbycrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DPSBYCR to value 0x01
impl crate::Resettable for DpsbycrSpec {
    const RESET_VALUE: u8 = 0x01;
}
