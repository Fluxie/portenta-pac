///Register `DEVADD%s` reader
pub type R = crate::R<DevaddSpec>;
///Register `DEVADD%s` writer
pub type W = crate::W<DevaddSpec>;
/**Transfer Speed of Communication Target Device

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Usbspd {
    ///0: Do not use DEVADDm
    _00 = 0,
    ///1: Low speed
    _01 = 1,
    ///2: Full speed
    _10 = 2,
    ///3: High speed
    _11 = 3,
}
impl From<Usbspd> for u8 {
    #[inline(always)]
    fn from(variant: Usbspd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Usbspd {
    type Ux = u8;
}
impl crate::IsEnum for Usbspd {}
///Field `USBSPD` reader - Transfer Speed of Communication Target Device
pub type UsbspdR = crate::FieldReader<Usbspd>;
impl UsbspdR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Usbspd {
        match self.bits {
            0 => Usbspd::_00,
            1 => Usbspd::_01,
            2 => Usbspd::_10,
            3 => Usbspd::_11,
            _ => unreachable!(),
        }
    }
    ///Do not use DEVADDm
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Usbspd::_00
    }
    ///Low speed
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Usbspd::_01
    }
    ///Full speed
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Usbspd::_10
    }
    ///High speed
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Usbspd::_11
    }
}
///Field `USBSPD` writer - Transfer Speed of Communication Target Device
pub type UsbspdW<'a, REG> = crate::FieldWriter<'a, REG, 2, Usbspd, crate::Safe>;
impl<'a, REG> UsbspdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Do not use DEVADDm
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Usbspd::_00)
    }
    ///Low speed
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Usbspd::_01)
    }
    ///Full speed
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Usbspd::_10)
    }
    ///High speed
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Usbspd::_11)
    }
}
/**Communication Target Connecting Hub Port

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hubport {
    ///0: Connect directly to the USBHS port
    _000 = 0,
    ///1: Port number of the hub
    Others = 1,
}
impl From<Hubport> for u8 {
    #[inline(always)]
    fn from(variant: Hubport) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hubport {
    type Ux = u8;
}
impl crate::IsEnum for Hubport {}
///Field `HUBPORT` reader - Communication Target Connecting Hub Port
pub type HubportR = crate::FieldReader<Hubport>;
impl HubportR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Hubport {
        match self.bits {
            0 => Hubport::_000,
            _ => Hubport::Others,
        }
    }
    ///Connect directly to the USBHS port
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Hubport::_000
    }
    ///Port number of the hub
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Hubport::Others)
    }
}
///Field `HUBPORT` writer - Communication Target Connecting Hub Port
pub type HubportW<'a, REG> = crate::FieldWriter<'a, REG, 3, Hubport, crate::Safe>;
impl<'a, REG> HubportW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Connect directly to the USBHS port
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Hubport::_000)
    }
    ///Port number of the hub
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Hubport::Others)
    }
}
/**Communication Target Connecting Hub Register

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Upphub {
    ///0: Connect directly to the USBHS port
    _0x0 = 0,
    ///1: USB address of the hub. The value as 0xB or more is reserved.
    Others = 1,
}
impl From<Upphub> for u8 {
    #[inline(always)]
    fn from(variant: Upphub) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Upphub {
    type Ux = u8;
}
impl crate::IsEnum for Upphub {}
///Field `UPPHUB` reader - Communication Target Connecting Hub Register
pub type UpphubR = crate::FieldReader<Upphub>;
impl UpphubR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Upphub {
        match self.bits {
            0 => Upphub::_0x0,
            _ => Upphub::Others,
        }
    }
    ///Connect directly to the USBHS port
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == Upphub::_0x0
    }
    ///USB address of the hub. The value as 0xB or more is reserved.
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Upphub::Others)
    }
}
///Field `UPPHUB` writer - Communication Target Connecting Hub Register
pub type UpphubW<'a, REG> = crate::FieldWriter<'a, REG, 4, Upphub, crate::Safe>;
impl<'a, REG> UpphubW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Connect directly to the USBHS port
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Upphub::_0x0)
    }
    ///USB address of the hub. The value as 0xB or more is reserved.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Upphub::Others)
    }
}
impl R {
    ///Bits 6:7 - Transfer Speed of Communication Target Device
    #[inline(always)]
    pub fn usbspd(&self) -> UsbspdR {
        UsbspdR::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:10 - Communication Target Connecting Hub Port
    #[inline(always)]
    pub fn hubport(&self) -> HubportR {
        HubportR::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 11:14 - Communication Target Connecting Hub Register
    #[inline(always)]
    pub fn upphub(&self) -> UpphubR {
        UpphubR::new(((self.bits >> 11) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEVADD")
            .field("usbspd", &self.usbspd())
            .field("hubport", &self.hubport())
            .field("upphub", &self.upphub())
            .finish()
    }
}
impl W {
    ///Bits 6:7 - Transfer Speed of Communication Target Device
    #[inline(always)]
    pub fn usbspd(&mut self) -> UsbspdW<DevaddSpec> {
        UsbspdW::new(self, 6)
    }
    ///Bits 8:10 - Communication Target Connecting Hub Port
    #[inline(always)]
    pub fn hubport(&mut self) -> HubportW<DevaddSpec> {
        HubportW::new(self, 8)
    }
    ///Bits 11:14 - Communication Target Connecting Hub Register
    #[inline(always)]
    pub fn upphub(&mut self) -> UpphubW<DevaddSpec> {
        UpphubW::new(self, 11)
    }
}
/**Device Address %s Configuration Register

You can [`read`](crate::Reg::read) this register and get [`devadd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devadd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DevaddSpec;
impl crate::RegisterSpec for DevaddSpec {
    type Ux = u16;
}
///`read()` method returns [`devadd::R`](R) reader structure
impl crate::Readable for DevaddSpec {}
///`write(|w| ..)` method takes [`devadd::W`](W) writer structure
impl crate::Writable for DevaddSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DEVADD%s to value 0
impl crate::Resettable for DevaddSpec {}
