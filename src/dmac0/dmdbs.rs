///Register `DMDBS` reader
pub type R = crate::R<DmdbsSpec>;
///Register `DMDBS` writer
pub type W = crate::W<DmdbsSpec>;
///Field `DMDBSL` reader - Functions as data transfer counter in repeat-block transfer mode.
pub type DmdbslR = crate::FieldReader<u16>;
///Field `DMDBSL` writer - Functions as data transfer counter in repeat-block transfer mode.
pub type DmdbslW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `DMDBSH` reader - Specifies the repeat-area size in repeat-block transfer mode.
pub type DmdbshR = crate::FieldReader<u16>;
///Field `DMDBSH` writer - Specifies the repeat-area size in repeat-block transfer mode.
pub type DmdbshW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Functions as data transfer counter in repeat-block transfer mode.
    #[inline(always)]
    pub fn dmdbsl(&self) -> DmdbslR {
        DmdbslR::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Specifies the repeat-area size in repeat-block transfer mode.
    #[inline(always)]
    pub fn dmdbsh(&self) -> DmdbshR {
        DmdbshR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMDBS")
            .field("dmdbsl", &self.dmdbsl())
            .field("dmdbsh", &self.dmdbsh())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Functions as data transfer counter in repeat-block transfer mode.
    #[inline(always)]
    pub fn dmdbsl(&mut self) -> DmdbslW<DmdbsSpec> {
        DmdbslW::new(self, 0)
    }
    ///Bits 16:31 - Specifies the repeat-area size in repeat-block transfer mode.
    #[inline(always)]
    pub fn dmdbsh(&mut self) -> DmdbshW<DmdbsSpec> {
        DmdbshW::new(self, 16)
    }
}
/**DMA Destination Buffer Size Register

You can [`read`](crate::Reg::read) this register and get [`dmdbs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmdbs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DmdbsSpec;
impl crate::RegisterSpec for DmdbsSpec {
    type Ux = u32;
}
///`read()` method returns [`dmdbs::R`](R) reader structure
impl crate::Readable for DmdbsSpec {}
///`write(|w| ..)` method takes [`dmdbs::W`](W) writer structure
impl crate::Writable for DmdbsSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMDBS to value 0
impl crate::Resettable for DmdbsSpec {}
