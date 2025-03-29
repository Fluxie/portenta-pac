///Register `CFIFOLL` reader
pub type R = crate::R<CfifollSpec>;
///Register `CFIFOLL` writer
pub type W = crate::W<CfifollSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**FIFO Port Register

You can [`read`](crate::Reg::read) this register and get [`cfifoll::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfifoll::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CfifollSpec;
impl crate::RegisterSpec for CfifollSpec {
    type Ux = u8;
}
///`read()` method returns [`cfifoll::R`](R) reader structure
impl crate::Readable for CfifollSpec {}
///`write(|w| ..)` method takes [`cfifoll::W`](W) writer structure
impl crate::Writable for CfifollSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFIFOLL to value 0
impl crate::Resettable for CfifollSpec {}
