///Register `ELSR%s` reader
pub type R = crate::R<ElsrSpec>;
///Register `ELSR%s` writer
pub type W = crate::W<ElsrSpec>;
///Field `ELS` reader - Event Link Select
pub type ElsR = crate::FieldReader<u16>;
///Field `ELS` writer - Event Link Select
pub type ElsW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - Event Link Select
    #[inline(always)]
    pub fn els(&self) -> ElsR {
        ElsR::new(self.bits & 0x01ff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ELSR").field("els", &self.els()).finish()
    }
}
impl W {
    ///Bits 0:8 - Event Link Select
    #[inline(always)]
    pub fn els(&mut self) -> ElsW<ElsrSpec> {
        ElsW::new(self, 0)
    }
}
/**Event Link Setting Register %s

You can [`read`](crate::Reg::read) this register and get [`elsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`elsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ElsrSpec;
impl crate::RegisterSpec for ElsrSpec {
    type Ux = u16;
}
///`read()` method returns [`elsr::R`](R) reader structure
impl crate::Readable for ElsrSpec {}
///`write(|w| ..)` method takes [`elsr::W`](W) writer structure
impl crate::Writable for ElsrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ELSR%s to value 0
impl crate::Resettable for ElsrSpec {}
