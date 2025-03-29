///Register `MALR` reader
pub type R = crate::R<MalrSpec>;
///Register `MALR` writer
pub type W = crate::W<MalrSpec>;
///Field `MALR` reader - MAC Address Lower Bit
pub type MalrR = crate::FieldReader<u16>;
///Field `MALR` writer - MAC Address Lower Bit
pub type MalrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - MAC Address Lower Bit
    #[inline(always)]
    pub fn malr(&self) -> MalrR {
        MalrR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MALR").field("malr", &self.malr()).finish()
    }
}
impl W {
    ///Bits 0:15 - MAC Address Lower Bit
    #[inline(always)]
    pub fn malr(&mut self) -> MalrW<MalrSpec> {
        MalrW::new(self, 0)
    }
}
/**MAC Address Lower Bit Register

You can [`read`](crate::Reg::read) this register and get [`malr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`malr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MalrSpec;
impl crate::RegisterSpec for MalrSpec {
    type Ux = u32;
}
///`read()` method returns [`malr::R`](R) reader structure
impl crate::Readable for MalrSpec {}
///`write(|w| ..)` method takes [`malr::W`](W) writer structure
impl crate::Writable for MalrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MALR to value 0
impl crate::Resettable for MalrSpec {}
