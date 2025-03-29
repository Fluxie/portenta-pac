///Register `CRCDOR_BY` reader
pub type R = crate::R<CrcdorBySpec>;
///Register `CRCDOR_BY` writer
pub type W = crate::W<CrcdorBySpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**CRC Data Output Register

You can [`read`](crate::Reg::read) this register and get [`crcdor_by::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcdor_by::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CrcdorBySpec;
impl crate::RegisterSpec for CrcdorBySpec {
    type Ux = u8;
}
///`read()` method returns [`crcdor_by::R`](R) reader structure
impl crate::Readable for CrcdorBySpec {}
///`write(|w| ..)` method takes [`crcdor_by::W`](W) writer structure
impl crate::Writable for CrcdorBySpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CRCDOR_BY to value 0
impl crate::Resettable for CrcdorBySpec {}
