///Register `PIPEMAXP` reader
pub type R = crate::R<PipemaxpSpec>;
///Register `PIPEMAXP` writer
pub type W = crate::W<PipemaxpSpec>;
///Field `MXPS` reader - Maximum Packet Size
pub type MxpsR = crate::FieldReader<u16>;
///Field `MXPS` writer - Maximum Packet Size
pub type MxpsW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `DEVSEL` reader - Device Select
pub type DevselR = crate::FieldReader;
///Field `DEVSEL` writer - Device Select
pub type DevselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:10 - Maximum Packet Size
    #[inline(always)]
    pub fn mxps(&self) -> MxpsR {
        MxpsR::new(self.bits & 0x07ff)
    }
    ///Bits 12:15 - Device Select
    #[inline(always)]
    pub fn devsel(&self) -> DevselR {
        DevselR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIPEMAXP")
            .field("mxps", &self.mxps())
            .field("devsel", &self.devsel())
            .finish()
    }
}
impl W {
    ///Bits 0:10 - Maximum Packet Size
    #[inline(always)]
    pub fn mxps(&mut self) -> MxpsW<PipemaxpSpec> {
        MxpsW::new(self, 0)
    }
    ///Bits 12:15 - Device Select
    #[inline(always)]
    pub fn devsel(&mut self) -> DevselW<PipemaxpSpec> {
        DevselW::new(self, 12)
    }
}
/**Pipe Maximum Packet Size Register

You can [`read`](crate::Reg::read) this register and get [`pipemaxp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pipemaxp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PipemaxpSpec;
impl crate::RegisterSpec for PipemaxpSpec {
    type Ux = u16;
}
///`read()` method returns [`pipemaxp::R`](R) reader structure
impl crate::Readable for PipemaxpSpec {}
///`write(|w| ..)` method takes [`pipemaxp::W`](W) writer structure
impl crate::Writable for PipemaxpSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PIPEMAXP to value 0
impl crate::Resettable for PipemaxpSpec {}
