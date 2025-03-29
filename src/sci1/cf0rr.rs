///Register `CF0RR` reader
pub type R = crate::R<Cf0rrSpec>;
///Register `CF0RR` writer
pub type W = crate::W<Cf0rrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**Control Field 0 Receive Data Register

You can [`read`](crate::Reg::read) this register and get [`cf0rr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cf0rr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Cf0rrSpec;
impl crate::RegisterSpec for Cf0rrSpec {
    type Ux = u8;
}
///`read()` method returns [`cf0rr::R`](R) reader structure
impl crate::Readable for Cf0rrSpec {}
///`write(|w| ..)` method takes [`cf0rr::W`](W) writer structure
impl crate::Writable for Cf0rrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CF0RR to value 0
impl crate::Resettable for Cf0rrSpec {}
