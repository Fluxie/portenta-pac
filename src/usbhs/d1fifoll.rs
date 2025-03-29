///Register `D1FIFOLL` reader
pub type R = crate::R<D1fifollSpec>;
///Register `D1FIFOLL` writer
pub type W = crate::W<D1fifollSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**FIFO Port Register

You can [`read`](crate::Reg::read) this register and get [`d1fifoll::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d1fifoll::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct D1fifollSpec;
impl crate::RegisterSpec for D1fifollSpec {
    type Ux = u8;
}
///`read()` method returns [`d1fifoll::R`](R) reader structure
impl crate::Readable for D1fifollSpec {}
///`write(|w| ..)` method takes [`d1fifoll::W`](W) writer structure
impl crate::Writable for D1fifollSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets D1FIFOLL to value 0
impl crate::Resettable for D1fifollSpec {}
