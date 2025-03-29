///Register `RFLR` reader
pub type R = crate::R<RflrSpec>;
///Register `RFLR` writer
pub type W = crate::W<RflrSpec>;
///Field `RFL` reader - Receive Frame Maximum Length
pub type RflR = crate::FieldReader<u16>;
///Field `RFL` writer - Receive Frame Maximum Length
pub type RflW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 0:11 - Receive Frame Maximum Length
    #[inline(always)]
    pub fn rfl(&self) -> RflR {
        RflR::new((self.bits & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RFLR").field("rfl", &self.rfl()).finish()
    }
}
impl W {
    ///Bits 0:11 - Receive Frame Maximum Length
    #[inline(always)]
    pub fn rfl(&mut self) -> RflW<RflrSpec> {
        RflW::new(self, 0)
    }
}
/**Receive Frame Maximum Length Register

You can [`read`](crate::Reg::read) this register and get [`rflr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rflr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RflrSpec;
impl crate::RegisterSpec for RflrSpec {
    type Ux = u32;
}
///`read()` method returns [`rflr::R`](R) reader structure
impl crate::Readable for RflrSpec {}
///`write(|w| ..)` method takes [`rflr::W`](W) writer structure
impl crate::Writable for RflrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RFLR to value 0
impl crate::Resettable for RflrSpec {}
