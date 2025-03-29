///Register `INTSTS0` reader
pub type R = crate::R<Intsts0Spec>;
///Register `INTSTS0` writer
pub type W = crate::W<Intsts0Spec>;
/**Control Transfer Stage

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctsq {
    ///0: Idle or setup stage
    _000 = 0,
    ///1: Control read data stage
    _001 = 1,
    ///2: Control read status stage
    _010 = 2,
    ///3: Control write data stage
    _011 = 3,
    ///4: Control write status stage
    _100 = 4,
    ///5: Control write (no data) status stage
    _101 = 5,
    ///6: Control transfer sequence error
    _110 = 6,
}
impl From<Ctsq> for u8 {
    #[inline(always)]
    fn from(variant: Ctsq) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctsq {
    type Ux = u8;
}
impl crate::IsEnum for Ctsq {}
///Field `CTSQ` reader - Control Transfer Stage
pub type CtsqR = crate::FieldReader<Ctsq>;
impl CtsqR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ctsq> {
        match self.bits {
            0 => Some(Ctsq::_000),
            1 => Some(Ctsq::_001),
            2 => Some(Ctsq::_010),
            3 => Some(Ctsq::_011),
            4 => Some(Ctsq::_100),
            5 => Some(Ctsq::_101),
            6 => Some(Ctsq::_110),
            _ => None,
        }
    }
    ///Idle or setup stage
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Ctsq::_000
    }
    ///Control read data stage
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Ctsq::_001
    }
    ///Control read status stage
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Ctsq::_010
    }
    ///Control write data stage
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Ctsq::_011
    }
    ///Control write status stage
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Ctsq::_100
    }
    ///Control write (no data) status stage
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Ctsq::_101
    }
    ///Control transfer sequence error
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Ctsq::_110
    }
}
/**USB Request Reception

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Valid {
    ///0: Setup packet not received
    _0 = 0,
    ///1: Setup packet received
    _1 = 1,
}
impl From<Valid> for bool {
    #[inline(always)]
    fn from(variant: Valid) -> Self {
        variant as u8 != 0
    }
}
///Field `VALID` reader - USB Request Reception
pub type ValidR = crate::BitReader<Valid>;
impl ValidR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Valid {
        match self.bits {
            false => Valid::_0,
            true => Valid::_1,
        }
    }
    ///Setup packet not received
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Valid::_0
    }
    ///Setup packet received
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Valid::_1
    }
}
///Field `VALID` writer - USB Request Reception
pub type ValidW<'a, REG> = crate::BitWriter<'a, REG, Valid>;
impl<'a, REG> ValidW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Setup packet not received
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Valid::_0)
    }
    ///Setup packet received
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Valid::_1)
    }
}
/**Device State

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dvsq {
    ///0: Powered state
    _000 = 0,
    ///1: Default state
    _001 = 1,
    ///2: Address state
    _010 = 2,
    ///3: Configured state
    _011 = 3,
    ///4: Suspend state
    Others = 4,
}
impl From<Dvsq> for u8 {
    #[inline(always)]
    fn from(variant: Dvsq) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dvsq {
    type Ux = u8;
}
impl crate::IsEnum for Dvsq {}
///Field `DVSQ` reader - Device State
pub type DvsqR = crate::FieldReader<Dvsq>;
impl DvsqR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dvsq {
        match self.bits {
            0 => Dvsq::_000,
            1 => Dvsq::_001,
            2 => Dvsq::_010,
            3 => Dvsq::_011,
            _ => Dvsq::Others,
        }
    }
    ///Powered state
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Dvsq::_000
    }
    ///Default state
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Dvsq::_001
    }
    ///Address state
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Dvsq::_010
    }
    ///Configured state
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Dvsq::_011
    }
    ///Suspend state
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Dvsq::Others)
    }
}
/**VBUS Input Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vbsts {
    ///0: USB_VBUS pin is low
    _0 = 0,
    ///1: USB_VBUS pin is high
    _1 = 1,
}
impl From<Vbsts> for bool {
    #[inline(always)]
    fn from(variant: Vbsts) -> Self {
        variant as u8 != 0
    }
}
///Field `VBSTS` reader - VBUS Input Status
pub type VbstsR = crate::BitReader<Vbsts>;
impl VbstsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Vbsts {
        match self.bits {
            false => Vbsts::_0,
            true => Vbsts::_1,
        }
    }
    ///USB_VBUS pin is low
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Vbsts::_0
    }
    ///USB_VBUS pin is high
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Vbsts::_1
    }
}
/**Buffer Ready Interrupt Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Brdy {
    ///0: No BRDY interrupt occurred
    _0 = 0,
    ///1: BRDY interrupt occurred
    _1 = 1,
}
impl From<Brdy> for bool {
    #[inline(always)]
    fn from(variant: Brdy) -> Self {
        variant as u8 != 0
    }
}
///Field `BRDY` reader - Buffer Ready Interrupt Status
pub type BrdyR = crate::BitReader<Brdy>;
impl BrdyR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Brdy {
        match self.bits {
            false => Brdy::_0,
            true => Brdy::_1,
        }
    }
    ///No BRDY interrupt occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Brdy::_0
    }
    ///BRDY interrupt occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Brdy::_1
    }
}
/**Buffer Not Ready Interrupt Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nrdy {
    ///0: No NRDY interrupt occurred
    _0 = 0,
    ///1: NRDY interrupt occurred
    _1 = 1,
}
impl From<Nrdy> for bool {
    #[inline(always)]
    fn from(variant: Nrdy) -> Self {
        variant as u8 != 0
    }
}
///Field `NRDY` reader - Buffer Not Ready Interrupt Status
pub type NrdyR = crate::BitReader<Nrdy>;
impl NrdyR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Nrdy {
        match self.bits {
            false => Nrdy::_0,
            true => Nrdy::_1,
        }
    }
    ///No NRDY interrupt occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Nrdy::_0
    }
    ///NRDY interrupt occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Nrdy::_1
    }
}
/**Buffer Empty Interrupt Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bemp {
    ///0: No BEMP interrupt occurred
    _0 = 0,
    ///1: BEMP interrupt occurred
    _1 = 1,
}
impl From<Bemp> for bool {
    #[inline(always)]
    fn from(variant: Bemp) -> Self {
        variant as u8 != 0
    }
}
///Field `BEMP` reader - Buffer Empty Interrupt Status
pub type BempR = crate::BitReader<Bemp>;
impl BempR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Bemp {
        match self.bits {
            false => Bemp::_0,
            true => Bemp::_1,
        }
    }
    ///No BEMP interrupt occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bemp::_0
    }
    ///BEMP interrupt occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bemp::_1
    }
}
/**Control Transfer Stage Transition Interrupt Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctrt {
    ///0: No control transfer stage transition interrupt occurred
    _0 = 0,
    ///1: Control transfer stage transition interrupt occurred
    _1 = 1,
}
impl From<Ctrt> for bool {
    #[inline(always)]
    fn from(variant: Ctrt) -> Self {
        variant as u8 != 0
    }
}
///Field `CTRT` reader - Control Transfer Stage Transition Interrupt Status
pub type CtrtR = crate::BitReader<Ctrt>;
impl CtrtR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ctrt {
        match self.bits {
            false => Ctrt::_0,
            true => Ctrt::_1,
        }
    }
    ///No control transfer stage transition interrupt occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ctrt::_0
    }
    ///Control transfer stage transition interrupt occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ctrt::_1
    }
}
///Field `CTRT` writer - Control Transfer Stage Transition Interrupt Status
pub type CtrtW<'a, REG> = crate::BitWriter<'a, REG, Ctrt>;
impl<'a, REG> CtrtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No control transfer stage transition interrupt occurred
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ctrt::_0)
    }
    ///Control transfer stage transition interrupt occurred
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ctrt::_1)
    }
}
/**Device State Transition Interrupt Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dvst {
    ///0: No device state transition interrupt occurred
    _0 = 0,
    ///1: Device state transition interrupt occurred
    _1 = 1,
}
impl From<Dvst> for bool {
    #[inline(always)]
    fn from(variant: Dvst) -> Self {
        variant as u8 != 0
    }
}
///Field `DVST` reader - Device State Transition Interrupt Status
pub type DvstR = crate::BitReader<Dvst>;
impl DvstR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dvst {
        match self.bits {
            false => Dvst::_0,
            true => Dvst::_1,
        }
    }
    ///No device state transition interrupt occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dvst::_0
    }
    ///Device state transition interrupt occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dvst::_1
    }
}
///Field `DVST` writer - Device State Transition Interrupt Status
pub type DvstW<'a, REG> = crate::BitWriter<'a, REG, Dvst>;
impl<'a, REG> DvstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No device state transition interrupt occurred
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dvst::_0)
    }
    ///Device state transition interrupt occurred
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dvst::_1)
    }
}
/**Frame Number Refresh Interrupt Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sofr {
    ///0: No SOF interrupt occurred
    _0 = 0,
    ///1: SOF interrupt occurred
    _1 = 1,
}
impl From<Sofr> for bool {
    #[inline(always)]
    fn from(variant: Sofr) -> Self {
        variant as u8 != 0
    }
}
///Field `SOFR` reader - Frame Number Refresh Interrupt Status
pub type SofrR = crate::BitReader<Sofr>;
impl SofrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sofr {
        match self.bits {
            false => Sofr::_0,
            true => Sofr::_1,
        }
    }
    ///No SOF interrupt occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sofr::_0
    }
    ///SOF interrupt occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sofr::_1
    }
}
///Field `SOFR` writer - Frame Number Refresh Interrupt Status
pub type SofrW<'a, REG> = crate::BitWriter<'a, REG, Sofr>;
impl<'a, REG> SofrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No SOF interrupt occurred
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sofr::_0)
    }
    ///SOF interrupt occurred
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sofr::_1)
    }
}
/**Resume Interrupt Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Resm {
    ///0: No resume interrupt occurred
    _0 = 0,
    ///1: Resume interrupt occurred
    _1 = 1,
}
impl From<Resm> for bool {
    #[inline(always)]
    fn from(variant: Resm) -> Self {
        variant as u8 != 0
    }
}
///Field `RESM` reader - Resume Interrupt Status
pub type ResmR = crate::BitReader<Resm>;
impl ResmR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Resm {
        match self.bits {
            false => Resm::_0,
            true => Resm::_1,
        }
    }
    ///No resume interrupt occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Resm::_0
    }
    ///Resume interrupt occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Resm::_1
    }
}
///Field `RESM` writer - Resume Interrupt Status
pub type ResmW<'a, REG> = crate::BitWriter<'a, REG, Resm>;
impl<'a, REG> ResmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No resume interrupt occurred
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Resm::_0)
    }
    ///Resume interrupt occurred
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Resm::_1)
    }
}
/**VBUS Interrupt Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vbint {
    ///0: No VBUS interrupt occurred
    _0 = 0,
    ///1: VBUS interrupt occurred
    _1 = 1,
}
impl From<Vbint> for bool {
    #[inline(always)]
    fn from(variant: Vbint) -> Self {
        variant as u8 != 0
    }
}
///Field `VBINT` reader - VBUS Interrupt Status
pub type VbintR = crate::BitReader<Vbint>;
impl VbintR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Vbint {
        match self.bits {
            false => Vbint::_0,
            true => Vbint::_1,
        }
    }
    ///No VBUS interrupt occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Vbint::_0
    }
    ///VBUS interrupt occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Vbint::_1
    }
}
///Field `VBINT` writer - VBUS Interrupt Status
pub type VbintW<'a, REG> = crate::BitWriter<'a, REG, Vbint>;
impl<'a, REG> VbintW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No VBUS interrupt occurred
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Vbint::_0)
    }
    ///VBUS interrupt occurred
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Vbint::_1)
    }
}
impl R {
    ///Bits 0:2 - Control Transfer Stage
    #[inline(always)]
    pub fn ctsq(&self) -> CtsqR {
        CtsqR::new((self.bits & 7) as u8)
    }
    ///Bit 3 - USB Request Reception
    #[inline(always)]
    pub fn valid(&self) -> ValidR {
        ValidR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:6 - Device State
    #[inline(always)]
    pub fn dvsq(&self) -> DvsqR {
        DvsqR::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - VBUS Input Status
    #[inline(always)]
    pub fn vbsts(&self) -> VbstsR {
        VbstsR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Buffer Ready Interrupt Status
    #[inline(always)]
    pub fn brdy(&self) -> BrdyR {
        BrdyR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Buffer Not Ready Interrupt Status
    #[inline(always)]
    pub fn nrdy(&self) -> NrdyR {
        NrdyR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Buffer Empty Interrupt Status
    #[inline(always)]
    pub fn bemp(&self) -> BempR {
        BempR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Control Transfer Stage Transition Interrupt Status
    #[inline(always)]
    pub fn ctrt(&self) -> CtrtR {
        CtrtR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Device State Transition Interrupt Status
    #[inline(always)]
    pub fn dvst(&self) -> DvstR {
        DvstR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Frame Number Refresh Interrupt Status
    #[inline(always)]
    pub fn sofr(&self) -> SofrR {
        SofrR::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Resume Interrupt Status
    #[inline(always)]
    pub fn resm(&self) -> ResmR {
        ResmR::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - VBUS Interrupt Status
    #[inline(always)]
    pub fn vbint(&self) -> VbintR {
        VbintR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTSTS0")
            .field("ctsq", &self.ctsq())
            .field("valid", &self.valid())
            .field("dvsq", &self.dvsq())
            .field("vbsts", &self.vbsts())
            .field("brdy", &self.brdy())
            .field("nrdy", &self.nrdy())
            .field("bemp", &self.bemp())
            .field("ctrt", &self.ctrt())
            .field("dvst", &self.dvst())
            .field("sofr", &self.sofr())
            .field("resm", &self.resm())
            .field("vbint", &self.vbint())
            .finish()
    }
}
impl W {
    ///Bit 3 - USB Request Reception
    #[inline(always)]
    pub fn valid(&mut self) -> ValidW<Intsts0Spec> {
        ValidW::new(self, 3)
    }
    ///Bit 11 - Control Transfer Stage Transition Interrupt Status
    #[inline(always)]
    pub fn ctrt(&mut self) -> CtrtW<Intsts0Spec> {
        CtrtW::new(self, 11)
    }
    ///Bit 12 - Device State Transition Interrupt Status
    #[inline(always)]
    pub fn dvst(&mut self) -> DvstW<Intsts0Spec> {
        DvstW::new(self, 12)
    }
    ///Bit 13 - Frame Number Refresh Interrupt Status
    #[inline(always)]
    pub fn sofr(&mut self) -> SofrW<Intsts0Spec> {
        SofrW::new(self, 13)
    }
    ///Bit 14 - Resume Interrupt Status
    #[inline(always)]
    pub fn resm(&mut self) -> ResmW<Intsts0Spec> {
        ResmW::new(self, 14)
    }
    ///Bit 15 - VBUS Interrupt Status
    #[inline(always)]
    pub fn vbint(&mut self) -> VbintW<Intsts0Spec> {
        VbintW::new(self, 15)
    }
}
/**Interrupt Status Register 0

You can [`read`](crate::Reg::read) this register and get [`intsts0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intsts0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Intsts0Spec;
impl crate::RegisterSpec for Intsts0Spec {
    type Ux = u16;
}
///`read()` method returns [`intsts0::R`](R) reader structure
impl crate::Readable for Intsts0Spec {}
///`write(|w| ..)` method takes [`intsts0::W`](W) writer structure
impl crate::Writable for Intsts0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets INTSTS0 to value 0
impl crate::Resettable for Intsts0Spec {}
