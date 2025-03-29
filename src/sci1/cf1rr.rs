///Register `CF1RR` reader
pub type R = crate::R<Cf1rrSpec>;
///Register `CF1RR` writer
pub type W = crate::W<Cf1rrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**Control Field 1 Receive Data Register

You can [`read`](crate::Reg::read) this register and get [`cf1rr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cf1rr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Cf1rrSpec;
impl crate::RegisterSpec for Cf1rrSpec {
    type Ux = u8;
}
///`read()` method returns [`cf1rr::R`](R) reader structure
impl crate::Readable for Cf1rrSpec {}
///`write(|w| ..)` method takes [`cf1rr::W`](W) writer structure
impl crate::Writable for Cf1rrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CF1RR to value 0
impl crate::Resettable for Cf1rrSpec {}
