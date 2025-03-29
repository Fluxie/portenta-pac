///Register `CTSUSDPRS` reader
pub type R = crate::R<CtsusdprsSpec>;
///Register `CTSUSDPRS` writer
pub type W = crate::W<CtsusdprsSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**CTSU Synchronous Noise Reduction Setting Register

You can [`read`](crate::Reg::read) this register and get [`ctsusdprs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsusdprs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CtsusdprsSpec;
impl crate::RegisterSpec for CtsusdprsSpec {
    type Ux = u8;
}
///`read()` method returns [`ctsusdprs::R`](R) reader structure
impl crate::Readable for CtsusdprsSpec {}
///`write(|w| ..)` method takes [`ctsusdprs::W`](W) writer structure
impl crate::Writable for CtsusdprsSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CTSUSDPRS to value 0
impl crate::Resettable for CtsusdprsSpec {}
