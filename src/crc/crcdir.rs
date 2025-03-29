///Register `CRCDIR` reader
pub type R = crate::R<CrcdirSpec>;
///Register `CRCDIR` writer
pub type W = crate::W<CrcdirSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**CRC Data Input Register

You can [`read`](crate::Reg::read) this register and get [`crcdir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcdir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CrcdirSpec;
impl crate::RegisterSpec for CrcdirSpec {
    type Ux = u32;
}
///`read()` method returns [`crcdir::R`](R) reader structure
impl crate::Readable for CrcdirSpec {}
///`write(|w| ..)` method takes [`crcdir::W`](W) writer structure
impl crate::Writable for CrcdirSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CRCDIR to value 0
impl crate::Resettable for CrcdirSpec {}
