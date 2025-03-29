///Register `BCFRR` reader
pub type R = crate::R<BcfrrSpec>;
///Register `BCFRR` writer
pub type W = crate::W<BcfrrSpec>;
///Field `BCF` reader -
pub type BcfR = crate::FieldReader<u16>;
///Field `BCF` writer -
pub type BcfW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15
    #[inline(always)]
    pub fn bcf(&self) -> BcfR {
        BcfR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BCFRR").field("bcf", &self.bcf()).finish()
    }
}
impl W {
    ///Bits 0:15
    #[inline(always)]
    pub fn bcf(&mut self) -> BcfW<BcfrrSpec> {
        BcfW::new(self, 0)
    }
}
/**Broadcast Frame Receive Count Setting Register

You can [`read`](crate::Reg::read) this register and get [`bcfrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcfrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BcfrrSpec;
impl crate::RegisterSpec for BcfrrSpec {
    type Ux = u32;
}
///`read()` method returns [`bcfrr::R`](R) reader structure
impl crate::Readable for BcfrrSpec {}
///`write(|w| ..)` method takes [`bcfrr::W`](W) writer structure
impl crate::Writable for BcfrrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BCFRR to value 0
impl crate::Resettable for BcfrrSpec {}
