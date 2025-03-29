///Register `CRCDIR_BY` reader
pub type R = crate::R<CrcdirBySpec>;
///Register `CRCDIR_BY` writer
pub type W = crate::W<CrcdirBySpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**CRC Data Input Register

You can [`read`](crate::Reg::read) this register and get [`crcdir_by::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcdir_by::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CrcdirBySpec;
impl crate::RegisterSpec for CrcdirBySpec {
    type Ux = u8;
}
///`read()` method returns [`crcdir_by::R`](R) reader structure
impl crate::Readable for CrcdirBySpec {}
///`write(|w| ..)` method takes [`crcdir_by::W`](W) writer structure
impl crate::Writable for CrcdirBySpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CRCDIR_BY to value 0
impl crate::Resettable for CrcdirBySpec {}
