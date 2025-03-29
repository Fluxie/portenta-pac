///Register `SYSCFG` reader
pub type R = crate::R<SyscfgSpec>;
///Register `SYSCFG` writer
pub type W = crate::W<SyscfgSpec>;
/**USBHS Operation Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbe {
    ///0: Disable
    _0 = 0,
    ///1: Enable
    _1 = 1,
}
impl From<Usbe> for bool {
    #[inline(always)]
    fn from(variant: Usbe) -> Self {
        variant as u8 != 0
    }
}
///Field `USBE` reader - USBHS Operation Enable
pub type UsbeR = crate::BitReader<Usbe>;
impl UsbeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Usbe {
        match self.bits {
            false => Usbe::_0,
            true => Usbe::_1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Usbe::_0
    }
    ///Enable
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Usbe::_1
    }
}
///Field `USBE` writer - USBHS Operation Enable
pub type UsbeW<'a, REG> = crate::BitWriter<'a, REG, Usbe>;
impl<'a, REG> UsbeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Usbe::_0)
    }
    ///Enable
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Usbe::_1)
    }
}
/**D+ Line Resistor Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dprpu {
    ///0: Disable line pull-up
    _0 = 0,
    ///1: Enable line pull-up
    _1 = 1,
}
impl From<Dprpu> for bool {
    #[inline(always)]
    fn from(variant: Dprpu) -> Self {
        variant as u8 != 0
    }
}
///Field `DPRPU` reader - D+ Line Resistor Control
pub type DprpuR = crate::BitReader<Dprpu>;
impl DprpuR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dprpu {
        match self.bits {
            false => Dprpu::_0,
            true => Dprpu::_1,
        }
    }
    ///Disable line pull-up
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dprpu::_0
    }
    ///Enable line pull-up
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dprpu::_1
    }
}
///Field `DPRPU` writer - D+ Line Resistor Control
pub type DprpuW<'a, REG> = crate::BitWriter<'a, REG, Dprpu>;
impl<'a, REG> DprpuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable line pull-up
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dprpu::_0)
    }
    ///Enable line pull-up
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dprpu::_1)
    }
}
/**D+/D- Line Resistor Control

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Drpd {
    ///0: Disable line pull-down
    _0 = 0,
    ///1: Enable line pull-down
    _1 = 1,
}
impl From<Drpd> for bool {
    #[inline(always)]
    fn from(variant: Drpd) -> Self {
        variant as u8 != 0
    }
}
///Field `DRPD` reader - D+/D- Line Resistor Control
pub type DrpdR = crate::BitReader<Drpd>;
impl DrpdR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Drpd {
        match self.bits {
            false => Drpd::_0,
            true => Drpd::_1,
        }
    }
    ///Disable line pull-down
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Drpd::_0
    }
    ///Enable line pull-down
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Drpd::_1
    }
}
///Field `DRPD` writer - D+/D- Line Resistor Control
pub type DrpdW<'a, REG> = crate::BitWriter<'a, REG, Drpd>;
impl<'a, REG> DrpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable line pull-down
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Drpd::_0)
    }
    ///Enable line pull-down
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Drpd::_1)
    }
}
/**Controller Operation Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dcfm {
    ///0: Select device controller mode
    _0 = 0,
    ///1: Select host controller mode
    _1 = 1,
}
impl From<Dcfm> for bool {
    #[inline(always)]
    fn from(variant: Dcfm) -> Self {
        variant as u8 != 0
    }
}
///Field `DCFM` reader - Controller Operation Select
pub type DcfmR = crate::BitReader<Dcfm>;
impl DcfmR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dcfm {
        match self.bits {
            false => Dcfm::_0,
            true => Dcfm::_1,
        }
    }
    ///Select device controller mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dcfm::_0
    }
    ///Select host controller mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dcfm::_1
    }
}
///Field `DCFM` writer - Controller Operation Select
pub type DcfmW<'a, REG> = crate::BitWriter<'a, REG, Dcfm>;
impl<'a, REG> DcfmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Select device controller mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dcfm::_0)
    }
    ///Select host controller mode
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dcfm::_1)
    }
}
/**High-Speed Operation Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hse {
    ///0: Disable Device controller mode: full-speed Host controller mode: full- or low-speed
    _0 = 0,
    ///1: Enable The controller detects the communication speed
    _1 = 1,
}
impl From<Hse> for bool {
    #[inline(always)]
    fn from(variant: Hse) -> Self {
        variant as u8 != 0
    }
}
///Field `HSE` reader - High-Speed Operation Enable
pub type HseR = crate::BitReader<Hse>;
impl HseR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Hse {
        match self.bits {
            false => Hse::_0,
            true => Hse::_1,
        }
    }
    ///Disable Device controller mode: full-speed Host controller mode: full- or low-speed
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Hse::_0
    }
    ///Enable The controller detects the communication speed
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Hse::_1
    }
}
///Field `HSE` writer - High-Speed Operation Enable
pub type HseW<'a, REG> = crate::BitWriter<'a, REG, Hse>;
impl<'a, REG> HseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable Device controller mode: full-speed Host controller mode: full- or low-speed
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Hse::_0)
    }
    ///Enable The controller detects the communication speed
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Hse::_1)
    }
}
/**Single-ended Receiver Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cnen {
    ///0: Disable
    _0 = 0,
    ///1: Enable
    _1 = 1,
}
impl From<Cnen> for bool {
    #[inline(always)]
    fn from(variant: Cnen) -> Self {
        variant as u8 != 0
    }
}
///Field `CNEN` reader - Single-ended Receiver Enable
pub type CnenR = crate::BitReader<Cnen>;
impl CnenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Cnen {
        match self.bits {
            false => Cnen::_0,
            true => Cnen::_1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cnen::_0
    }
    ///Enable
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cnen::_1
    }
}
///Field `CNEN` writer - Single-ended Receiver Enable
pub type CnenW<'a, REG> = crate::BitWriter<'a, REG, Cnen>;
impl<'a, REG> CnenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cnen::_0)
    }
    ///Enable
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cnen::_1)
    }
}
impl R {
    ///Bit 0 - USBHS Operation Enable
    #[inline(always)]
    pub fn usbe(&self) -> UsbeR {
        UsbeR::new((self.bits & 1) != 0)
    }
    ///Bit 4 - D+ Line Resistor Control
    #[inline(always)]
    pub fn dprpu(&self) -> DprpuR {
        DprpuR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - D+/D- Line Resistor Control
    #[inline(always)]
    pub fn drpd(&self) -> DrpdR {
        DrpdR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Controller Operation Select
    #[inline(always)]
    pub fn dcfm(&self) -> DcfmR {
        DcfmR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - High-Speed Operation Enable
    #[inline(always)]
    pub fn hse(&self) -> HseR {
        HseR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Single-ended Receiver Enable
    #[inline(always)]
    pub fn cnen(&self) -> CnenR {
        CnenR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSCFG")
            .field("usbe", &self.usbe())
            .field("dprpu", &self.dprpu())
            .field("drpd", &self.drpd())
            .field("dcfm", &self.dcfm())
            .field("hse", &self.hse())
            .field("cnen", &self.cnen())
            .finish()
    }
}
impl W {
    ///Bit 0 - USBHS Operation Enable
    #[inline(always)]
    pub fn usbe(&mut self) -> UsbeW<SyscfgSpec> {
        UsbeW::new(self, 0)
    }
    ///Bit 4 - D+ Line Resistor Control
    #[inline(always)]
    pub fn dprpu(&mut self) -> DprpuW<SyscfgSpec> {
        DprpuW::new(self, 4)
    }
    ///Bit 5 - D+/D- Line Resistor Control
    #[inline(always)]
    pub fn drpd(&mut self) -> DrpdW<SyscfgSpec> {
        DrpdW::new(self, 5)
    }
    ///Bit 6 - Controller Operation Select
    #[inline(always)]
    pub fn dcfm(&mut self) -> DcfmW<SyscfgSpec> {
        DcfmW::new(self, 6)
    }
    ///Bit 7 - High-Speed Operation Enable
    #[inline(always)]
    pub fn hse(&mut self) -> HseW<SyscfgSpec> {
        HseW::new(self, 7)
    }
    ///Bit 8 - Single-ended Receiver Enable
    #[inline(always)]
    pub fn cnen(&mut self) -> CnenW<SyscfgSpec> {
        CnenW::new(self, 8)
    }
}
/**System Configuration Control Register

You can [`read`](crate::Reg::read) this register and get [`syscfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SyscfgSpec;
impl crate::RegisterSpec for SyscfgSpec {
    type Ux = u16;
}
///`read()` method returns [`syscfg::R`](R) reader structure
impl crate::Readable for SyscfgSpec {}
///`write(|w| ..)` method takes [`syscfg::W`](W) writer structure
impl crate::Writable for SyscfgSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SYSCFG to value 0x20
impl crate::Resettable for SyscfgSpec {
    const RESET_VALUE: u16 = 0x20;
}
