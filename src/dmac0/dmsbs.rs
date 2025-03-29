///Register `DMSBS` reader
pub type R = crate::R<DmsbsSpec>;
///Register `DMSBS` writer
pub type W = crate::W<DmsbsSpec>;
///Field `DMSBSL` reader - Functions as data transfer counter in repeat-block transfer mode
pub type DmsbslR = crate::FieldReader<u16>;
///Field `DMSBSL` writer - Functions as data transfer counter in repeat-block transfer mode
pub type DmsbslW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `DMSBSH` reader - Specifies the repeat-area size in repeat-block transfer mode
pub type DmsbshR = crate::FieldReader<u16>;
///Field `DMSBSH` writer - Specifies the repeat-area size in repeat-block transfer mode
pub type DmsbshW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Functions as data transfer counter in repeat-block transfer mode
    #[inline(always)]
    pub fn dmsbsl(&self) -> DmsbslR {
        DmsbslR::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Specifies the repeat-area size in repeat-block transfer mode
    #[inline(always)]
    pub fn dmsbsh(&self) -> DmsbshR {
        DmsbshR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMSBS")
            .field("dmsbsl", &self.dmsbsl())
            .field("dmsbsh", &self.dmsbsh())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Functions as data transfer counter in repeat-block transfer mode
    #[inline(always)]
    pub fn dmsbsl(&mut self) -> DmsbslW<DmsbsSpec> {
        DmsbslW::new(self, 0)
    }
    ///Bits 16:31 - Specifies the repeat-area size in repeat-block transfer mode
    #[inline(always)]
    pub fn dmsbsh(&mut self) -> DmsbshW<DmsbsSpec> {
        DmsbshW::new(self, 16)
    }
}
/**DMA Source Buffer Size Register

You can [`read`](crate::Reg::read) this register and get [`dmsbs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmsbs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DmsbsSpec;
impl crate::RegisterSpec for DmsbsSpec {
    type Ux = u32;
}
///`read()` method returns [`dmsbs::R`](R) reader structure
impl crate::Readable for DmsbsSpec {}
///`write(|w| ..)` method takes [`dmsbs::W`](W) writer structure
impl crate::Writable for DmsbsSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMSBS to value 0
impl crate::Resettable for DmsbsSpec {}
