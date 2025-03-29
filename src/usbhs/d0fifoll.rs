///Register `D0FIFOLL` reader
pub type R = crate::R<D0fifollSpec>;
///Register `D0FIFOLL` writer
pub type W = crate::W<D0fifollSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**FIFO Port Register

You can [`read`](crate::Reg::read) this register and get [`d0fifoll::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d0fifoll::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct D0fifollSpec;
impl crate::RegisterSpec for D0fifollSpec {
    type Ux = u8;
}
///`read()` method returns [`d0fifoll::R`](R) reader structure
impl crate::Readable for D0fifollSpec {}
///`write(|w| ..)` method takes [`d0fifoll::W`](W) writer structure
impl crate::Writable for D0fifollSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets D0FIFOLL to value 0
impl crate::Resettable for D0fifollSpec {}
