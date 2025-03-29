///Register `MPR` reader
pub type R = crate::R<MprSpec>;
///Register `MPR` writer
pub type W = crate::W<MprSpec>;
///Field `MP` reader - Manual PAUSE Time Setting
pub type MpR = crate::FieldReader<u16>;
///Field `MP` writer - Manual PAUSE Time Setting
pub type MpW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Manual PAUSE Time Setting
    #[inline(always)]
    pub fn mp(&self) -> MpR {
        MpR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MPR").field("mp", &self.mp()).finish()
    }
}
impl W {
    ///Bits 0:15 - Manual PAUSE Time Setting
    #[inline(always)]
    pub fn mp(&mut self) -> MpW<MprSpec> {
        MpW::new(self, 0)
    }
}
/**Manual PAUSE Frame Register

You can [`read`](crate::Reg::read) this register and get [`mpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MprSpec;
impl crate::RegisterSpec for MprSpec {
    type Ux = u32;
}
///`read()` method returns [`mpr::R`](R) reader structure
impl crate::Readable for MprSpec {}
///`write(|w| ..)` method takes [`mpr::W`](W) writer structure
impl crate::Writable for MprSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MPR to value 0
impl crate::Resettable for MprSpec {}
