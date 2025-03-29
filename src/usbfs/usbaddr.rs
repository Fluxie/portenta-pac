///Register `USBADDR` reader
pub type R = crate::R<UsbaddrSpec>;
///Register `USBADDR` writer
pub type W = crate::W<UsbaddrSpec>;
///Field `USBADDR` reader - USB Address
pub type UsbaddrR = crate::FieldReader;
///Field `USBADDR` writer - USB Address
pub type UsbaddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
/**Status Recovery

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Stsrecov {
    ///4: Recovery in device controller mode: Setting prohibited Recovery in host controller mode: Return to the low-speed state (bits DVSTCTR0.RHST\[2:0\] = 001b)
    _0x4 = 4,
    ///8: Recovery in device controller mode: Setting prohibited Recovery in host controller mode: Return to the full-speed state (bits DVSTCTR0.RHST\[2:0\] = 010b)
    _0x8 = 8,
    ///9: Recovery in device controller mode: Return to the full-speed state (bits DVSTCTR0.RHST\[2:0\] = 010b), bits INTSTS0.DVSQ\[2:0\] = 001b (default state) Recovery in host controller mode: Setting prohibited
    _0x9 = 9,
    ///10: Recovery in device controller mode: Return to the full-speed state (bits DVSTCTR0.RHST\[2:0\] = 010b), bits INTSTS0.DVSQ\[2:0\] = 010b (address state) Recovery in host controller mode: Setting prohibited
    _0xA = 10,
    ///11: Recovery in device controller mode: Return to the full-speed state (bits DVSTCTR0.RHST\[2:0\] = 010b), bits INTSTS0.DVSQ\[2:0\] = 011b (configured state) Recovery in host controller mode: Setting prohibited
    _0xB = 11,
    ///0: Setting prohibited
    Others = 0,
}
impl From<Stsrecov> for u8 {
    #[inline(always)]
    fn from(variant: Stsrecov) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Stsrecov {
    type Ux = u8;
}
impl crate::IsEnum for Stsrecov {}
///Field `STSRECOV` reader - Status Recovery
pub type StsrecovR = crate::FieldReader<Stsrecov>;
impl StsrecovR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Stsrecov {
        match self.bits {
            4 => Stsrecov::_0x4,
            8 => Stsrecov::_0x8,
            9 => Stsrecov::_0x9,
            10 => Stsrecov::_0xA,
            11 => Stsrecov::_0xB,
            _ => Stsrecov::Others,
        }
    }
    ///Recovery in device controller mode: Setting prohibited Recovery in host controller mode: Return to the low-speed state (bits DVSTCTR0.RHST\[2:0\] = 001b)
    #[inline(always)]
    pub fn is_0x4(&self) -> bool {
        *self == Stsrecov::_0x4
    }
    ///Recovery in device controller mode: Setting prohibited Recovery in host controller mode: Return to the full-speed state (bits DVSTCTR0.RHST\[2:0\] = 010b)
    #[inline(always)]
    pub fn is_0x8(&self) -> bool {
        *self == Stsrecov::_0x8
    }
    ///Recovery in device controller mode: Return to the full-speed state (bits DVSTCTR0.RHST\[2:0\] = 010b), bits INTSTS0.DVSQ\[2:0\] = 001b (default state) Recovery in host controller mode: Setting prohibited
    #[inline(always)]
    pub fn is_0x9(&self) -> bool {
        *self == Stsrecov::_0x9
    }
    ///Recovery in device controller mode: Return to the full-speed state (bits DVSTCTR0.RHST\[2:0\] = 010b), bits INTSTS0.DVSQ\[2:0\] = 010b (address state) Recovery in host controller mode: Setting prohibited
    #[inline(always)]
    pub fn is_0x_a(&self) -> bool {
        *self == Stsrecov::_0xA
    }
    ///Recovery in device controller mode: Return to the full-speed state (bits DVSTCTR0.RHST\[2:0\] = 010b), bits INTSTS0.DVSQ\[2:0\] = 011b (configured state) Recovery in host controller mode: Setting prohibited
    #[inline(always)]
    pub fn is_0x_b(&self) -> bool {
        *self == Stsrecov::_0xB
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Stsrecov::Others)
    }
}
///Field `STSRECOV` writer - Status Recovery
pub type StsrecovW<'a, REG> = crate::FieldWriter<'a, REG, 4, Stsrecov, crate::Safe>;
impl<'a, REG> StsrecovW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Recovery in device controller mode: Setting prohibited Recovery in host controller mode: Return to the low-speed state (bits DVSTCTR0.RHST\[2:0\] = 001b)
    #[inline(always)]
    pub fn _0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Stsrecov::_0x4)
    }
    ///Recovery in device controller mode: Setting prohibited Recovery in host controller mode: Return to the full-speed state (bits DVSTCTR0.RHST\[2:0\] = 010b)
    #[inline(always)]
    pub fn _0x8(self) -> &'a mut crate::W<REG> {
        self.variant(Stsrecov::_0x8)
    }
    ///Recovery in device controller mode: Return to the full-speed state (bits DVSTCTR0.RHST\[2:0\] = 010b), bits INTSTS0.DVSQ\[2:0\] = 001b (default state) Recovery in host controller mode: Setting prohibited
    #[inline(always)]
    pub fn _0x9(self) -> &'a mut crate::W<REG> {
        self.variant(Stsrecov::_0x9)
    }
    ///Recovery in device controller mode: Return to the full-speed state (bits DVSTCTR0.RHST\[2:0\] = 010b), bits INTSTS0.DVSQ\[2:0\] = 010b (address state) Recovery in host controller mode: Setting prohibited
    #[inline(always)]
    pub fn _0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(Stsrecov::_0xA)
    }
    ///Recovery in device controller mode: Return to the full-speed state (bits DVSTCTR0.RHST\[2:0\] = 010b), bits INTSTS0.DVSQ\[2:0\] = 011b (configured state) Recovery in host controller mode: Setting prohibited
    #[inline(always)]
    pub fn _0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(Stsrecov::_0xB)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Stsrecov::Others)
    }
}
impl R {
    ///Bits 0:6 - USB Address
    #[inline(always)]
    pub fn usbaddr(&self) -> UsbaddrR {
        UsbaddrR::new((self.bits & 0x7f) as u8)
    }
    ///Bits 8:11 - Status Recovery
    #[inline(always)]
    pub fn stsrecov(&self) -> StsrecovR {
        StsrecovR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USBADDR")
            .field("usbaddr", &self.usbaddr())
            .field("stsrecov", &self.stsrecov())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - USB Address
    #[inline(always)]
    pub fn usbaddr(&mut self) -> UsbaddrW<UsbaddrSpec> {
        UsbaddrW::new(self, 0)
    }
    ///Bits 8:11 - Status Recovery
    #[inline(always)]
    pub fn stsrecov(&mut self) -> StsrecovW<UsbaddrSpec> {
        StsrecovW::new(self, 8)
    }
}
/**USB Address Register

You can [`read`](crate::Reg::read) this register and get [`usbaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct UsbaddrSpec;
impl crate::RegisterSpec for UsbaddrSpec {
    type Ux = u16;
}
///`read()` method returns [`usbaddr::R`](R) reader structure
impl crate::Readable for UsbaddrSpec {}
///`write(|w| ..)` method takes [`usbaddr::W`](W) writer structure
impl crate::Writable for UsbaddrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets USBADDR to value 0
impl crate::Resettable for UsbaddrSpec {}
