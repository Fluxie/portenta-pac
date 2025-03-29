///Register `SYSCFG` reader
pub type R = crate::R<SyscfgSpec>;
///Register `SYSCFG` writer
pub type W = crate::W<SyscfgSpec>;
/**USBFS Operation Enable

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
///Field `USBE` reader - USBFS Operation Enable
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
///Field `USBE` writer - USBFS Operation Enable
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
/**D+/D– Line Resistor Control

Value on reset: 0*/
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
///Field `DRPD` reader - D+/D– Line Resistor Control
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
///Field `DRPD` writer - D+/D– Line Resistor Control
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
/**Controller Function Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dcfm {
    ///0: Select device controller
    _0 = 0,
    ///1: Select host controller
    _1 = 1,
}
impl From<Dcfm> for bool {
    #[inline(always)]
    fn from(variant: Dcfm) -> Self {
        variant as u8 != 0
    }
}
///Field `DCFM` reader - Controller Function Select
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
    ///Select device controller
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dcfm::_0
    }
    ///Select host controller
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dcfm::_1
    }
}
///Field `DCFM` writer - Controller Function Select
pub type DcfmW<'a, REG> = crate::BitWriter<'a, REG, Dcfm>;
impl<'a, REG> DcfmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Select device controller
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dcfm::_0)
    }
    ///Select host controller
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dcfm::_1)
    }
}
/**USB Clock Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scke {
    ///0: Stop clock supply to the USBFS
    _0 = 0,
    ///1: Enable clock supply to the USBFS
    _1 = 1,
}
impl From<Scke> for bool {
    #[inline(always)]
    fn from(variant: Scke) -> Self {
        variant as u8 != 0
    }
}
///Field `SCKE` reader - USB Clock Enable
pub type SckeR = crate::BitReader<Scke>;
impl SckeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Scke {
        match self.bits {
            false => Scke::_0,
            true => Scke::_1,
        }
    }
    ///Stop clock supply to the USBFS
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Scke::_0
    }
    ///Enable clock supply to the USBFS
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Scke::_1
    }
}
///Field `SCKE` writer - USB Clock Enable
pub type SckeW<'a, REG> = crate::BitWriter<'a, REG, Scke>;
impl<'a, REG> SckeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Stop clock supply to the USBFS
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Scke::_0)
    }
    ///Enable clock supply to the USBFS
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Scke::_1)
    }
}
impl R {
    ///Bit 0 - USBFS Operation Enable
    #[inline(always)]
    pub fn usbe(&self) -> UsbeR {
        UsbeR::new((self.bits & 1) != 0)
    }
    ///Bit 4 - D+ Line Resistor Control
    #[inline(always)]
    pub fn dprpu(&self) -> DprpuR {
        DprpuR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - D+/D– Line Resistor Control
    #[inline(always)]
    pub fn drpd(&self) -> DrpdR {
        DrpdR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Controller Function Select
    #[inline(always)]
    pub fn dcfm(&self) -> DcfmR {
        DcfmR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 10 - USB Clock Enable
    #[inline(always)]
    pub fn scke(&self) -> SckeR {
        SckeR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSCFG")
            .field("usbe", &self.usbe())
            .field("dprpu", &self.dprpu())
            .field("drpd", &self.drpd())
            .field("dcfm", &self.dcfm())
            .field("scke", &self.scke())
            .finish()
    }
}
impl W {
    ///Bit 0 - USBFS Operation Enable
    #[inline(always)]
    pub fn usbe(&mut self) -> UsbeW<SyscfgSpec> {
        UsbeW::new(self, 0)
    }
    ///Bit 4 - D+ Line Resistor Control
    #[inline(always)]
    pub fn dprpu(&mut self) -> DprpuW<SyscfgSpec> {
        DprpuW::new(self, 4)
    }
    ///Bit 5 - D+/D– Line Resistor Control
    #[inline(always)]
    pub fn drpd(&mut self) -> DrpdW<SyscfgSpec> {
        DrpdW::new(self, 5)
    }
    ///Bit 6 - Controller Function Select
    #[inline(always)]
    pub fn dcfm(&mut self) -> DcfmW<SyscfgSpec> {
        DcfmW::new(self, 6)
    }
    ///Bit 10 - USB Clock Enable
    #[inline(always)]
    pub fn scke(&mut self) -> SckeW<SyscfgSpec> {
        SckeW::new(self, 10)
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
///`reset()` method sets SYSCFG to value 0
impl crate::Resettable for SyscfgSpec {}
