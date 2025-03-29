///Register `BRR` reader
pub type R = crate::R<BrrSpec>;
///Register `BRR` writer
pub type W = crate::W<BrrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**Bit Rate Register

You can [`read`](crate::Reg::read) this register and get [`brr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BrrSpec;
impl crate::RegisterSpec for BrrSpec {
    type Ux = u8;
}
///`read()` method returns [`brr::R`](R) reader structure
impl crate::Readable for BrrSpec {}
///`write(|w| ..)` method takes [`brr::W`](W) writer structure
impl crate::Writable for BrrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BRR to value 0xff
impl crate::Resettable for BrrSpec {
    const RESET_VALUE: u8 = 0xff;
}
