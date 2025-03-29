///Register `USBINDX` reader
pub type R = crate::R<UsbindxSpec>;
///Register `USBINDX` writer
pub type W = crate::W<UsbindxSpec>;
///Field `WINDEX` reader - USB request wIndex value
pub type WindexR = crate::FieldReader<u16>;
///Field `WINDEX` writer - USB request wIndex value
pub type WindexW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - USB request wIndex value
    #[inline(always)]
    pub fn windex(&self) -> WindexR {
        WindexR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USBINDX").field("windex", &self.windex()).finish()
    }
}
impl W {
    ///Bits 0:15 - USB request wIndex value
    #[inline(always)]
    pub fn windex(&mut self) -> WindexW<UsbindxSpec> {
        WindexW::new(self, 0)
    }
}
/**USB Request Index Register

You can [`read`](crate::Reg::read) this register and get [`usbindx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbindx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct UsbindxSpec;
impl crate::RegisterSpec for UsbindxSpec {
    type Ux = u16;
}
///`read()` method returns [`usbindx::R`](R) reader structure
impl crate::Readable for UsbindxSpec {}
///`write(|w| ..)` method takes [`usbindx::W`](W) writer structure
impl crate::Writable for UsbindxSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets USBINDX to value 0
impl crate::Resettable for UsbindxSpec {}
