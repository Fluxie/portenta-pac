///Register `APR` reader
pub type R = crate::R<AprSpec>;
///Register `APR` writer
pub type W = crate::W<AprSpec>;
///Field `AP` reader - Automatic PAUSE Time Setting
pub type ApR = crate::FieldReader<u16>;
///Field `AP` writer - Automatic PAUSE Time Setting
pub type ApW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Automatic PAUSE Time Setting
    #[inline(always)]
    pub fn ap(&self) -> ApR {
        ApR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APR").field("ap", &self.ap()).finish()
    }
}
impl W {
    ///Bits 0:15 - Automatic PAUSE Time Setting
    #[inline(always)]
    pub fn ap(&mut self) -> ApW<AprSpec> {
        ApW::new(self, 0)
    }
}
/**Automatic PAUSE Frame Register

You can [`read`](crate::Reg::read) this register and get [`apr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AprSpec;
impl crate::RegisterSpec for AprSpec {
    type Ux = u32;
}
///`read()` method returns [`apr::R`](R) reader structure
impl crate::Readable for AprSpec {}
///`write(|w| ..)` method takes [`apr::W`](W) writer structure
impl crate::Writable for AprSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APR to value 0
impl crate::Resettable for AprSpec {}
