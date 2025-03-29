///Register `DVSTCTR0` reader
pub type R = crate::R<Dvstctr0Spec>;
///Register `DVSTCTR0` writer
pub type W = crate::W<Dvstctr0Spec>;
/**USB Bus Reset Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rhst {
    ///0: Communication speed indeterminate (powered state or no connection)
    _000 = 0,
    ///1: Host controller mode Low-speed connection Device controller mode USB bus reset in progress or low-speed connection
    _001 = 1,
    ///2: Host controller mode Full-speed connection Device controller mode USB bus reset in progress or full-speed connection
    _010 = 2,
    ///3: Host controller mode High-speed connection Device controller mode USB bus reset in progress or high-speed connection
    _011 = 3,
    ///4: Host controller mode USB bus reset in progress Device controller mode Setting prohibited
    Others = 4,
}
impl From<Rhst> for u8 {
    #[inline(always)]
    fn from(variant: Rhst) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rhst {
    type Ux = u8;
}
impl crate::IsEnum for Rhst {}
///Field `RHST` reader - USB Bus Reset Status Flag
pub type RhstR = crate::FieldReader<Rhst>;
impl RhstR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rhst {
        match self.bits {
            0 => Rhst::_000,
            1 => Rhst::_001,
            2 => Rhst::_010,
            3 => Rhst::_011,
            _ => Rhst::Others,
        }
    }
    ///Communication speed indeterminate (powered state or no connection)
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Rhst::_000
    }
    ///Host controller mode Low-speed connection Device controller mode USB bus reset in progress or low-speed connection
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Rhst::_001
    }
    ///Host controller mode Full-speed connection Device controller mode USB bus reset in progress or full-speed connection
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Rhst::_010
    }
    ///Host controller mode High-speed connection Device controller mode USB bus reset in progress or high-speed connection
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Rhst::_011
    }
    ///Host controller mode USB bus reset in progress Device controller mode Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Rhst::Others)
    }
}
/**USB Bus Operation Enable for the Host Controller Operation

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uact {
    ///0: Disable downstream port (disable SOF or micro-SOF transmission)
    _0 = 0,
    ///1: Enable downstream port (enable SOF or micro-SOF transmission)
    _1 = 1,
}
impl From<Uact> for bool {
    #[inline(always)]
    fn from(variant: Uact) -> Self {
        variant as u8 != 0
    }
}
///Field `UACT` reader - USB Bus Operation Enable for the Host Controller Operation
pub type UactR = crate::BitReader<Uact>;
impl UactR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Uact {
        match self.bits {
            false => Uact::_0,
            true => Uact::_1,
        }
    }
    ///Disable downstream port (disable SOF or micro-SOF transmission)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Uact::_0
    }
    ///Enable downstream port (enable SOF or micro-SOF transmission)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Uact::_1
    }
}
///Field `UACT` writer - USB Bus Operation Enable for the Host Controller Operation
pub type UactW<'a, REG> = crate::BitWriter<'a, REG, Uact>;
impl<'a, REG> UactW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable downstream port (disable SOF or micro-SOF transmission)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Uact::_0)
    }
    ///Enable downstream port (enable SOF or micro-SOF transmission)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Uact::_1)
    }
}
/**Resume Signal Output for the Host Controller Operation

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Resume {
    ///0: Do not output resume signal
    _0 = 0,
    ///1: Output resume signal
    _1 = 1,
}
impl From<Resume> for bool {
    #[inline(always)]
    fn from(variant: Resume) -> Self {
        variant as u8 != 0
    }
}
///Field `RESUME` reader - Resume Signal Output for the Host Controller Operation
pub type ResumeR = crate::BitReader<Resume>;
impl ResumeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Resume {
        match self.bits {
            false => Resume::_0,
            true => Resume::_1,
        }
    }
    ///Do not output resume signal
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Resume::_0
    }
    ///Output resume signal
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Resume::_1
    }
}
///Field `RESUME` writer - Resume Signal Output for the Host Controller Operation
pub type ResumeW<'a, REG> = crate::BitWriter<'a, REG, Resume>;
impl<'a, REG> ResumeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not output resume signal
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Resume::_0)
    }
    ///Output resume signal
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Resume::_1)
    }
}
/**USB Bus Reset Output for the Host Controller Operation

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbrst {
    ///0: Do not output USB bus reset signal
    _0 = 0,
    ///1: Output USB bus reset signal
    _1 = 1,
}
impl From<Usbrst> for bool {
    #[inline(always)]
    fn from(variant: Usbrst) -> Self {
        variant as u8 != 0
    }
}
///Field `USBRST` reader - USB Bus Reset Output for the Host Controller Operation
pub type UsbrstR = crate::BitReader<Usbrst>;
impl UsbrstR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Usbrst {
        match self.bits {
            false => Usbrst::_0,
            true => Usbrst::_1,
        }
    }
    ///Do not output USB bus reset signal
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Usbrst::_0
    }
    ///Output USB bus reset signal
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Usbrst::_1
    }
}
///Field `USBRST` writer - USB Bus Reset Output for the Host Controller Operation
pub type UsbrstW<'a, REG> = crate::BitWriter<'a, REG, Usbrst>;
impl<'a, REG> UsbrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not output USB bus reset signal
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Usbrst::_0)
    }
    ///Output USB bus reset signal
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Usbrst::_1)
    }
}
/**Remote Wakeup Detection Enable for the Host Controller Operation

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rwupe {
    ///0: Disable downstream port remote wakeup
    _0 = 0,
    ///1: Enable downstream port remote wakeup
    _1 = 1,
}
impl From<Rwupe> for bool {
    #[inline(always)]
    fn from(variant: Rwupe) -> Self {
        variant as u8 != 0
    }
}
///Field `RWUPE` reader - Remote Wakeup Detection Enable for the Host Controller Operation
pub type RwupeR = crate::BitReader<Rwupe>;
impl RwupeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rwupe {
        match self.bits {
            false => Rwupe::_0,
            true => Rwupe::_1,
        }
    }
    ///Disable downstream port remote wakeup
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rwupe::_0
    }
    ///Enable downstream port remote wakeup
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rwupe::_1
    }
}
///Field `RWUPE` writer - Remote Wakeup Detection Enable for the Host Controller Operation
pub type RwupeW<'a, REG> = crate::BitWriter<'a, REG, Rwupe>;
impl<'a, REG> RwupeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable downstream port remote wakeup
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rwupe::_0)
    }
    ///Enable downstream port remote wakeup
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rwupe::_1)
    }
}
/**Remote Wakeup Output for the Device Controller Operation

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkup {
    ///0: Do not output remote wakeup signal
    _0 = 0,
    ///1: Output remote wakeup signal
    _1 = 1,
}
impl From<Wkup> for bool {
    #[inline(always)]
    fn from(variant: Wkup) -> Self {
        variant as u8 != 0
    }
}
///Field `WKUP` reader - Remote Wakeup Output for the Device Controller Operation
pub type WkupR = crate::BitReader<Wkup>;
impl WkupR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Wkup {
        match self.bits {
            false => Wkup::_0,
            true => Wkup::_1,
        }
    }
    ///Do not output remote wakeup signal
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wkup::_0
    }
    ///Output remote wakeup signal
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wkup::_1
    }
}
///Field `WKUP` writer - Remote Wakeup Output for the Device Controller Operation
pub type WkupW<'a, REG> = crate::BitWriter<'a, REG, Wkup>;
impl<'a, REG> WkupW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not output remote wakeup signal
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wkup::_0)
    }
    ///Output remote wakeup signal
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wkup::_1)
    }
}
/**USBHS_VBUSEN Output Pin Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vbusen {
    ///0: Output low on external USBHS_VBUSEN pin
    _0 = 0,
    ///1: Output high on external USBHS_VBUSEN pin
    _1 = 1,
}
impl From<Vbusen> for bool {
    #[inline(always)]
    fn from(variant: Vbusen) -> Self {
        variant as u8 != 0
    }
}
///Field `VBUSEN` reader - USBHS_VBUSEN Output Pin Control
pub type VbusenR = crate::BitReader<Vbusen>;
impl VbusenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Vbusen {
        match self.bits {
            false => Vbusen::_0,
            true => Vbusen::_1,
        }
    }
    ///Output low on external USBHS_VBUSEN pin
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Vbusen::_0
    }
    ///Output high on external USBHS_VBUSEN pin
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Vbusen::_1
    }
}
///Field `VBUSEN` writer - USBHS_VBUSEN Output Pin Control
pub type VbusenW<'a, REG> = crate::BitWriter<'a, REG, Vbusen>;
impl<'a, REG> VbusenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Output low on external USBHS_VBUSEN pin
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Vbusen::_0)
    }
    ///Output high on external USBHS_VBUSEN pin
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Vbusen::_1)
    }
}
/**USBHS_EXICEN Output Pin Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Exicen {
    ///0: Output low on external USBHS_EXICEN pin
    _0 = 0,
    ///1: Output high on external USBHS_EXICEN pin
    _1 = 1,
}
impl From<Exicen> for bool {
    #[inline(always)]
    fn from(variant: Exicen) -> Self {
        variant as u8 != 0
    }
}
///Field `EXICEN` reader - USBHS_EXICEN Output Pin Control
pub type ExicenR = crate::BitReader<Exicen>;
impl ExicenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Exicen {
        match self.bits {
            false => Exicen::_0,
            true => Exicen::_1,
        }
    }
    ///Output low on external USBHS_EXICEN pin
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Exicen::_0
    }
    ///Output high on external USBHS_EXICEN pin
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Exicen::_1
    }
}
///Field `EXICEN` writer - USBHS_EXICEN Output Pin Control
pub type ExicenW<'a, REG> = crate::BitWriter<'a, REG, Exicen>;
impl<'a, REG> ExicenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Output low on external USBHS_EXICEN pin
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Exicen::_0)
    }
    ///Output high on external USBHS_EXICEN pin
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Exicen::_1)
    }
}
///Field `HNPBTOA` reader - Host Negotiation Protocol (HNP) Control
pub type HnpbtoaR = crate::BitReader;
///Field `HNPBTOA` writer - Host Negotiation Protocol (HNP) Control
pub type HnpbtoaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:2 - USB Bus Reset Status Flag
    #[inline(always)]
    pub fn rhst(&self) -> RhstR {
        RhstR::new((self.bits & 7) as u8)
    }
    ///Bit 4 - USB Bus Operation Enable for the Host Controller Operation
    #[inline(always)]
    pub fn uact(&self) -> UactR {
        UactR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Resume Signal Output for the Host Controller Operation
    #[inline(always)]
    pub fn resume(&self) -> ResumeR {
        ResumeR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - USB Bus Reset Output for the Host Controller Operation
    #[inline(always)]
    pub fn usbrst(&self) -> UsbrstR {
        UsbrstR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Remote Wakeup Detection Enable for the Host Controller Operation
    #[inline(always)]
    pub fn rwupe(&self) -> RwupeR {
        RwupeR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Remote Wakeup Output for the Device Controller Operation
    #[inline(always)]
    pub fn wkup(&self) -> WkupR {
        WkupR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - USBHS_VBUSEN Output Pin Control
    #[inline(always)]
    pub fn vbusen(&self) -> VbusenR {
        VbusenR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - USBHS_EXICEN Output Pin Control
    #[inline(always)]
    pub fn exicen(&self) -> ExicenR {
        ExicenR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Host Negotiation Protocol (HNP) Control
    #[inline(always)]
    pub fn hnpbtoa(&self) -> HnpbtoaR {
        HnpbtoaR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DVSTCTR0")
            .field("rhst", &self.rhst())
            .field("uact", &self.uact())
            .field("resume", &self.resume())
            .field("usbrst", &self.usbrst())
            .field("rwupe", &self.rwupe())
            .field("wkup", &self.wkup())
            .field("vbusen", &self.vbusen())
            .field("exicen", &self.exicen())
            .field("hnpbtoa", &self.hnpbtoa())
            .finish()
    }
}
impl W {
    ///Bit 4 - USB Bus Operation Enable for the Host Controller Operation
    #[inline(always)]
    pub fn uact(&mut self) -> UactW<Dvstctr0Spec> {
        UactW::new(self, 4)
    }
    ///Bit 5 - Resume Signal Output for the Host Controller Operation
    #[inline(always)]
    pub fn resume(&mut self) -> ResumeW<Dvstctr0Spec> {
        ResumeW::new(self, 5)
    }
    ///Bit 6 - USB Bus Reset Output for the Host Controller Operation
    #[inline(always)]
    pub fn usbrst(&mut self) -> UsbrstW<Dvstctr0Spec> {
        UsbrstW::new(self, 6)
    }
    ///Bit 7 - Remote Wakeup Detection Enable for the Host Controller Operation
    #[inline(always)]
    pub fn rwupe(&mut self) -> RwupeW<Dvstctr0Spec> {
        RwupeW::new(self, 7)
    }
    ///Bit 8 - Remote Wakeup Output for the Device Controller Operation
    #[inline(always)]
    pub fn wkup(&mut self) -> WkupW<Dvstctr0Spec> {
        WkupW::new(self, 8)
    }
    ///Bit 9 - USBHS_VBUSEN Output Pin Control
    #[inline(always)]
    pub fn vbusen(&mut self) -> VbusenW<Dvstctr0Spec> {
        VbusenW::new(self, 9)
    }
    ///Bit 10 - USBHS_EXICEN Output Pin Control
    #[inline(always)]
    pub fn exicen(&mut self) -> ExicenW<Dvstctr0Spec> {
        ExicenW::new(self, 10)
    }
    ///Bit 11 - Host Negotiation Protocol (HNP) Control
    #[inline(always)]
    pub fn hnpbtoa(&mut self) -> HnpbtoaW<Dvstctr0Spec> {
        HnpbtoaW::new(self, 11)
    }
}
/**Device State Control Register 0

You can [`read`](crate::Reg::read) this register and get [`dvstctr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvstctr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Dvstctr0Spec;
impl crate::RegisterSpec for Dvstctr0Spec {
    type Ux = u16;
}
///`read()` method returns [`dvstctr0::R`](R) reader structure
impl crate::Readable for Dvstctr0Spec {}
///`write(|w| ..)` method takes [`dvstctr0::W`](W) writer structure
impl crate::Writable for Dvstctr0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DVSTCTR0 to value 0
impl crate::Resettable for Dvstctr0Spec {}
