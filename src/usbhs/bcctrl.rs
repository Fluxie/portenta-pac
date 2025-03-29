///Register `BCCTRL` reader
pub type R = crate::R<BcctrlSpec>;
///Register `BCCTRL` writer
pub type W = crate::W<BcctrlSpec>;
/**IDPSRC Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Idpsrce {
    ///0: Disable IDP_SRC circuit
    _0 = 0,
    ///1: Enable IDP_SRC circuit
    _1 = 1,
}
impl From<Idpsrce> for bool {
    #[inline(always)]
    fn from(variant: Idpsrce) -> Self {
        variant as u8 != 0
    }
}
///Field `IDPSRCE` reader - IDPSRC Control
pub type IdpsrceR = crate::BitReader<Idpsrce>;
impl IdpsrceR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Idpsrce {
        match self.bits {
            false => Idpsrce::_0,
            true => Idpsrce::_1,
        }
    }
    ///Disable IDP_SRC circuit
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Idpsrce::_0
    }
    ///Enable IDP_SRC circuit
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Idpsrce::_1
    }
}
///Field `IDPSRCE` writer - IDPSRC Control
pub type IdpsrceW<'a, REG> = crate::BitWriter<'a, REG, Idpsrce>;
impl<'a, REG> IdpsrceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable IDP_SRC circuit
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Idpsrce::_0)
    }
    ///Enable IDP_SRC circuit
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Idpsrce::_1)
    }
}
/**IDMSINK Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Idmsinke {
    ///0: Disable IDM_SINK circuit
    _0 = 0,
    ///1: Enable IDM_SINK circuit
    _1 = 1,
}
impl From<Idmsinke> for bool {
    #[inline(always)]
    fn from(variant: Idmsinke) -> Self {
        variant as u8 != 0
    }
}
///Field `IDMSINKE` reader - IDMSINK Control
pub type IdmsinkeR = crate::BitReader<Idmsinke>;
impl IdmsinkeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Idmsinke {
        match self.bits {
            false => Idmsinke::_0,
            true => Idmsinke::_1,
        }
    }
    ///Disable IDM_SINK circuit
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Idmsinke::_0
    }
    ///Enable IDM_SINK circuit
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Idmsinke::_1
    }
}
///Field `IDMSINKE` writer - IDMSINK Control
pub type IdmsinkeW<'a, REG> = crate::BitWriter<'a, REG, Idmsinke>;
impl<'a, REG> IdmsinkeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable IDM_SINK circuit
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Idmsinke::_0)
    }
    ///Enable IDM_SINK circuit
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Idmsinke::_1)
    }
}
/**VDPSRC Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vdpsrce {
    ///0: Disable VDP_SRC circuit
    _0 = 0,
    ///1: Enable VDP_SRC circuit
    _1 = 1,
}
impl From<Vdpsrce> for bool {
    #[inline(always)]
    fn from(variant: Vdpsrce) -> Self {
        variant as u8 != 0
    }
}
///Field `VDPSRCE` reader - VDPSRC Control
pub type VdpsrceR = crate::BitReader<Vdpsrce>;
impl VdpsrceR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Vdpsrce {
        match self.bits {
            false => Vdpsrce::_0,
            true => Vdpsrce::_1,
        }
    }
    ///Disable VDP_SRC circuit
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Vdpsrce::_0
    }
    ///Enable VDP_SRC circuit
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Vdpsrce::_1
    }
}
///Field `VDPSRCE` writer - VDPSRC Control
pub type VdpsrceW<'a, REG> = crate::BitWriter<'a, REG, Vdpsrce>;
impl<'a, REG> VdpsrceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable VDP_SRC circuit
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Vdpsrce::_0)
    }
    ///Enable VDP_SRC circuit
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Vdpsrce::_1)
    }
}
/**IDPSINK Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Idpsinke {
    ///0: Disable IDP_SINK circuit
    _0 = 0,
    ///1: Enable IDP_SINK circuit
    _1 = 1,
}
impl From<Idpsinke> for bool {
    #[inline(always)]
    fn from(variant: Idpsinke) -> Self {
        variant as u8 != 0
    }
}
///Field `IDPSINKE` reader - IDPSINK Control
pub type IdpsinkeR = crate::BitReader<Idpsinke>;
impl IdpsinkeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Idpsinke {
        match self.bits {
            false => Idpsinke::_0,
            true => Idpsinke::_1,
        }
    }
    ///Disable IDP_SINK circuit
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Idpsinke::_0
    }
    ///Enable IDP_SINK circuit
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Idpsinke::_1
    }
}
///Field `IDPSINKE` writer - IDPSINK Control
pub type IdpsinkeW<'a, REG> = crate::BitWriter<'a, REG, Idpsinke>;
impl<'a, REG> IdpsinkeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable IDP_SINK circuit
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Idpsinke::_0)
    }
    ///Enable IDP_SINK circuit
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Idpsinke::_1)
    }
}
/**VDMSRC Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vdmsrce {
    ///0: Disable VDM_SRC circuit
    _0 = 0,
    ///1: Enable VDM_SRC circuit
    _1 = 1,
}
impl From<Vdmsrce> for bool {
    #[inline(always)]
    fn from(variant: Vdmsrce) -> Self {
        variant as u8 != 0
    }
}
///Field `VDMSRCE` reader - VDMSRC Control
pub type VdmsrceR = crate::BitReader<Vdmsrce>;
impl VdmsrceR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Vdmsrce {
        match self.bits {
            false => Vdmsrce::_0,
            true => Vdmsrce::_1,
        }
    }
    ///Disable VDM_SRC circuit
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Vdmsrce::_0
    }
    ///Enable VDM_SRC circuit
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Vdmsrce::_1
    }
}
///Field `VDMSRCE` writer - VDMSRC Control
pub type VdmsrceW<'a, REG> = crate::BitWriter<'a, REG, Vdmsrce>;
impl<'a, REG> VdmsrceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable VDM_SRC circuit
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Vdmsrce::_0)
    }
    ///Enable VDM_SRC circuit
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Vdmsrce::_1)
    }
}
/**DCP Mode Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dcpmode {
    ///0: Disable RDCP_DAT resistor
    _0 = 0,
    ///1: Enable RDCP_DAT resistor
    _1 = 1,
}
impl From<Dcpmode> for bool {
    #[inline(always)]
    fn from(variant: Dcpmode) -> Self {
        variant as u8 != 0
    }
}
///Field `DCPMODE` reader - DCP Mode Control
pub type DcpmodeR = crate::BitReader<Dcpmode>;
impl DcpmodeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dcpmode {
        match self.bits {
            false => Dcpmode::_0,
            true => Dcpmode::_1,
        }
    }
    ///Disable RDCP_DAT resistor
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dcpmode::_0
    }
    ///Enable RDCP_DAT resistor
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dcpmode::_1
    }
}
///Field `DCPMODE` writer - DCP Mode Control
pub type DcpmodeW<'a, REG> = crate::BitWriter<'a, REG, Dcpmode>;
impl<'a, REG> DcpmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable RDCP_DAT resistor
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dcpmode::_0)
    }
    ///Enable RDCP_DAT resistor
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dcpmode::_1)
    }
}
/**CHGDET Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chgdetsts {
    ///0: The CHGDET pin is at low level
    _0 = 0,
    ///1: The CHGDET pin is at high level
    _1 = 1,
}
impl From<Chgdetsts> for bool {
    #[inline(always)]
    fn from(variant: Chgdetsts) -> Self {
        variant as u8 != 0
    }
}
///Field `CHGDETSTS` reader - CHGDET Status Flag
pub type ChgdetstsR = crate::BitReader<Chgdetsts>;
impl ChgdetstsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Chgdetsts {
        match self.bits {
            false => Chgdetsts::_0,
            true => Chgdetsts::_1,
        }
    }
    ///The CHGDET pin is at low level
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Chgdetsts::_0
    }
    ///The CHGDET pin is at high level
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Chgdetsts::_1
    }
}
/**PDDET Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pddetsts {
    ///0: The PDDET pin is at low level
    _0 = 0,
    ///1: The PDDET pin is at high level
    _1 = 1,
}
impl From<Pddetsts> for bool {
    #[inline(always)]
    fn from(variant: Pddetsts) -> Self {
        variant as u8 != 0
    }
}
///Field `PDDETSTS` reader - PDDET Status Flag
pub type PddetstsR = crate::BitReader<Pddetsts>;
impl PddetstsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pddetsts {
        match self.bits {
            false => Pddetsts::_0,
            true => Pddetsts::_1,
        }
    }
    ///The PDDET pin is at low level
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pddetsts::_0
    }
    ///The PDDET pin is at high level
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pddetsts::_1
    }
}
impl R {
    ///Bit 0 - IDPSRC Control
    #[inline(always)]
    pub fn idpsrce(&self) -> IdpsrceR {
        IdpsrceR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - IDMSINK Control
    #[inline(always)]
    pub fn idmsinke(&self) -> IdmsinkeR {
        IdmsinkeR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - VDPSRC Control
    #[inline(always)]
    pub fn vdpsrce(&self) -> VdpsrceR {
        VdpsrceR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - IDPSINK Control
    #[inline(always)]
    pub fn idpsinke(&self) -> IdpsinkeR {
        IdpsinkeR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - VDMSRC Control
    #[inline(always)]
    pub fn vdmsrce(&self) -> VdmsrceR {
        VdmsrceR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - DCP Mode Control
    #[inline(always)]
    pub fn dcpmode(&self) -> DcpmodeR {
        DcpmodeR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - CHGDET Status Flag
    #[inline(always)]
    pub fn chgdetsts(&self) -> ChgdetstsR {
        ChgdetstsR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - PDDET Status Flag
    #[inline(always)]
    pub fn pddetsts(&self) -> PddetstsR {
        PddetstsR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BCCTRL")
            .field("idpsrce", &self.idpsrce())
            .field("idmsinke", &self.idmsinke())
            .field("vdpsrce", &self.vdpsrce())
            .field("idpsinke", &self.idpsinke())
            .field("vdmsrce", &self.vdmsrce())
            .field("dcpmode", &self.dcpmode())
            .field("chgdetsts", &self.chgdetsts())
            .field("pddetsts", &self.pddetsts())
            .finish()
    }
}
impl W {
    ///Bit 0 - IDPSRC Control
    #[inline(always)]
    pub fn idpsrce(&mut self) -> IdpsrceW<BcctrlSpec> {
        IdpsrceW::new(self, 0)
    }
    ///Bit 1 - IDMSINK Control
    #[inline(always)]
    pub fn idmsinke(&mut self) -> IdmsinkeW<BcctrlSpec> {
        IdmsinkeW::new(self, 1)
    }
    ///Bit 2 - VDPSRC Control
    #[inline(always)]
    pub fn vdpsrce(&mut self) -> VdpsrceW<BcctrlSpec> {
        VdpsrceW::new(self, 2)
    }
    ///Bit 3 - IDPSINK Control
    #[inline(always)]
    pub fn idpsinke(&mut self) -> IdpsinkeW<BcctrlSpec> {
        IdpsinkeW::new(self, 3)
    }
    ///Bit 4 - VDMSRC Control
    #[inline(always)]
    pub fn vdmsrce(&mut self) -> VdmsrceW<BcctrlSpec> {
        VdmsrceW::new(self, 4)
    }
    ///Bit 5 - DCP Mode Control
    #[inline(always)]
    pub fn dcpmode(&mut self) -> DcpmodeW<BcctrlSpec> {
        DcpmodeW::new(self, 5)
    }
}
/**Battery Charging Control Register

You can [`read`](crate::Reg::read) this register and get [`bcctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BcctrlSpec;
impl crate::RegisterSpec for BcctrlSpec {
    type Ux = u16;
}
///`read()` method returns [`bcctrl::R`](R) reader structure
impl crate::Readable for BcctrlSpec {}
///`write(|w| ..)` method takes [`bcctrl::W`](W) writer structure
impl crate::Writable for BcctrlSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BCCTRL to value 0
impl crate::Resettable for BcctrlSpec {}
