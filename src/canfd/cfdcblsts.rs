///Register `CFDC%sBLSTS` reader
pub type R = crate::R<CfdcblstsSpec>;
///Register `CFDC%sBLSTS` writer
pub type W = crate::W<CfdcblstsSpec>;
///Field `BLC` reader - Bus Load Counter
pub type BlcR = crate::FieldReader<u32>;
impl R {
    ///Bits 3:31 - Bus Load Counter
    #[inline(always)]
    pub fn blc(&self) -> BlcR {
        BlcR::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFDCBLSTS").field("blc", &self.blc()).finish()
    }
}
impl W {}
/**Channel %s Bus Load Status Register

You can [`read`](crate::Reg::read) this register and get [`cfdcblsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdcblsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfdcblstsSpec;
impl crate::RegisterSpec for CfdcblstsSpec {
    type Ux = u32;
}
///`read()` method returns [`cfdcblsts::R`](R) reader structure
impl crate::Readable for CfdcblstsSpec {}
///`write(|w| ..)` method takes [`cfdcblsts::W`](W) writer structure
impl crate::Writable for CfdcblstsSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFDC%sBLSTS to value 0
impl crate::Resettable for CfdcblstsSpec {}
