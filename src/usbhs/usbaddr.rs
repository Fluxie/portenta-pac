///Register `USBADDR` reader
pub type R = crate::R<UsbaddrSpec>;
///Register `USBADDR` writer
pub type W = crate::W<UsbaddrSpec>;
///Field `USBADDR` reader - USB Address Flag
pub type UsbaddrR = crate::FieldReader;
/**Status Recovery

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Stsrecov0 {
    ///0: Reserved
    _000 = 0,
    ///1: \[D\] Return to the full-speed connection and Default state
    _001 = 1,
    ///2: \[D\] Return to the full-speed connection and Address state \[H\] Return to the low-speed state (bits DVSTCTR0.RHST\[2:0\] = 001b)
    _010 = 2,
    ///3: \[D\] Return to the full-speed connection and Configured state
    _011 = 3,
    ///4: \[D\] Return to the suspend connection and Suspend state \[H\] Return to the full-speed state (bits DVSTCTR0.RHST\[2:0\] = 010b)
    _100 = 4,
    ///5: \[D\] Return to the high-speed connection and Default state
    _101 = 5,
    ///6: \[D\] Return to the high-speed connection and Address state \[H\] Return to the high-speed state (bits DVSTCTR0.RHST\[2:0\] = 011b)
    _110 = 6,
    ///7: \[D\] Return to the high-speed connection and Configured state
    _111 = 7,
}
impl From<Stsrecov0> for u8 {
    #[inline(always)]
    fn from(variant: Stsrecov0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Stsrecov0 {
    type Ux = u8;
}
impl crate::IsEnum for Stsrecov0 {}
///Field `STSRECOV0` reader - Status Recovery
pub type Stsrecov0R = crate::FieldReader<Stsrecov0>;
impl Stsrecov0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Stsrecov0 {
        match self.bits {
            0 => Stsrecov0::_000,
            1 => Stsrecov0::_001,
            2 => Stsrecov0::_010,
            3 => Stsrecov0::_011,
            4 => Stsrecov0::_100,
            5 => Stsrecov0::_101,
            6 => Stsrecov0::_110,
            7 => Stsrecov0::_111,
            _ => unreachable!(),
        }
    }
    ///Reserved
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Stsrecov0::_000
    }
    ///D\] Return to the full-speed connection and Default state
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Stsrecov0::_001
    }
    ///D\] Return to the full-speed connection and Address state \[H\] Return to the low-speed state (bits DVSTCTR0.RHST\[2:0\] = 001b)
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Stsrecov0::_010
    }
    ///D\] Return to the full-speed connection and Configured state
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Stsrecov0::_011
    }
    ///D\] Return to the suspend connection and Suspend state \[H\] Return to the full-speed state (bits DVSTCTR0.RHST\[2:0\] = 010b)
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Stsrecov0::_100
    }
    ///D\] Return to the high-speed connection and Default state
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Stsrecov0::_101
    }
    ///D\] Return to the high-speed connection and Address state \[H\] Return to the high-speed state (bits DVSTCTR0.RHST\[2:0\] = 011b)
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Stsrecov0::_110
    }
    ///D\] Return to the high-speed connection and Configured state
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == Stsrecov0::_111
    }
}
///Field `STSRECOV0` writer - Status Recovery
pub type Stsrecov0W<'a, REG> = crate::FieldWriter<'a, REG, 3, Stsrecov0, crate::Safe>;
impl<'a, REG> Stsrecov0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Reserved
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Stsrecov0::_000)
    }
    ///D\] Return to the full-speed connection and Default state
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Stsrecov0::_001)
    }
    ///D\] Return to the full-speed connection and Address state \[H\] Return to the low-speed state (bits DVSTCTR0.RHST\[2:0\] = 001b)
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Stsrecov0::_010)
    }
    ///D\] Return to the full-speed connection and Configured state
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Stsrecov0::_011)
    }
    ///D\] Return to the suspend connection and Suspend state \[H\] Return to the full-speed state (bits DVSTCTR0.RHST\[2:0\] = 010b)
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Stsrecov0::_100)
    }
    ///D\] Return to the high-speed connection and Default state
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Stsrecov0::_101)
    }
    ///D\] Return to the high-speed connection and Address state \[H\] Return to the high-speed state (bits DVSTCTR0.RHST\[2:0\] = 011b)
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Stsrecov0::_110)
    }
    ///D\] Return to the high-speed connection and Configured state
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(Stsrecov0::_111)
    }
}
impl R {
    ///Bits 0:6 - USB Address Flag
    #[inline(always)]
    pub fn usbaddr(&self) -> UsbaddrR {
        UsbaddrR::new((self.bits & 0x7f) as u8)
    }
    ///Bits 8:10 - Status Recovery
    #[inline(always)]
    pub fn stsrecov0(&self) -> Stsrecov0R {
        Stsrecov0R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USBADDR")
            .field("usbaddr", &self.usbaddr())
            .field("stsrecov0", &self.stsrecov0())
            .finish()
    }
}
impl W {
    ///Bits 8:10 - Status Recovery
    #[inline(always)]
    pub fn stsrecov0(&mut self) -> Stsrecov0W<UsbaddrSpec> {
        Stsrecov0W::new(self, 8)
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
