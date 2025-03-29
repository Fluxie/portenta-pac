///Register `MDLR` reader
pub type R = crate::R<MdlrSpec>;
///Register `MDLR` writer
pub type W = crate::W<MdlrSpec>;
///Field `DV0RDL` reader - Device 0 Read dummy length setting
pub type Dv0rdlR = crate::FieldReader;
///Field `DV0RDL` writer - Device 0 Read dummy length setting
pub type Dv0rdlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DV0WDL` reader - Device 0 Write dummy length setting
pub type Dv0wdlR = crate::FieldReader;
///Field `DV0WDL` writer - Device 0 Write dummy length setting
pub type Dv0wdlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DV1RDL` reader - Device 1 Read dummy length setting
pub type Dv1rdlR = crate::FieldReader;
///Field `DV1RDL` writer - Device 1 Read dummy length setting
pub type Dv1rdlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DV1WDL` reader - Device 1 Write dummy length setting
pub type Dv1wdlR = crate::FieldReader;
///Field `DV1WDL` writer - Device 1 Write dummy length setting
pub type Dv1wdlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Device 0 Read dummy length setting
    #[inline(always)]
    pub fn dv0rdl(&self) -> Dv0rdlR {
        Dv0rdlR::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Device 0 Write dummy length setting
    #[inline(always)]
    pub fn dv0wdl(&self) -> Dv0wdlR {
        Dv0wdlR::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Device 1 Read dummy length setting
    #[inline(always)]
    pub fn dv1rdl(&self) -> Dv1rdlR {
        Dv1rdlR::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Device 1 Write dummy length setting
    #[inline(always)]
    pub fn dv1wdl(&self) -> Dv1wdlR {
        Dv1wdlR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MDLR")
            .field("dv0rdl", &self.dv0rdl())
            .field("dv0wdl", &self.dv0wdl())
            .field("dv1rdl", &self.dv1rdl())
            .field("dv1wdl", &self.dv1wdl())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Device 0 Read dummy length setting
    #[inline(always)]
    pub fn dv0rdl(&mut self) -> Dv0rdlW<MdlrSpec> {
        Dv0rdlW::new(self, 0)
    }
    ///Bits 8:15 - Device 0 Write dummy length setting
    #[inline(always)]
    pub fn dv0wdl(&mut self) -> Dv0wdlW<MdlrSpec> {
        Dv0wdlW::new(self, 8)
    }
    ///Bits 16:23 - Device 1 Read dummy length setting
    #[inline(always)]
    pub fn dv1rdl(&mut self) -> Dv1rdlW<MdlrSpec> {
        Dv1rdlW::new(self, 16)
    }
    ///Bits 24:31 - Device 1 Write dummy length setting
    #[inline(always)]
    pub fn dv1wdl(&mut self) -> Dv1wdlW<MdlrSpec> {
        Dv1wdlW::new(self, 24)
    }
}
/**Memory Map Dummy Length Register

You can [`read`](crate::Reg::read) this register and get [`mdlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MdlrSpec;
impl crate::RegisterSpec for MdlrSpec {
    type Ux = u32;
}
///`read()` method returns [`mdlr::R`](R) reader structure
impl crate::Readable for MdlrSpec {}
///`write(|w| ..)` method takes [`mdlr::W`](W) writer structure
impl crate::Writable for MdlrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MDLR to value 0
impl crate::Resettable for MdlrSpec {}
