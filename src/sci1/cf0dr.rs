///Register `CF0DR` reader
pub type R = crate::R<Cf0drSpec>;
///Register `CF0DR` writer
pub type W = crate::W<Cf0drSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**Control Field 0 Data Register

You can [`read`](crate::Reg::read) this register and get [`cf0dr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cf0dr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Cf0drSpec;
impl crate::RegisterSpec for Cf0drSpec {
    type Ux = u8;
}
///`read()` method returns [`cf0dr::R`](R) reader structure
impl crate::Readable for Cf0drSpec {}
///`write(|w| ..)` method takes [`cf0dr::W`](W) writer structure
impl crate::Writable for Cf0drSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CF0DR to value 0
impl crate::Resettable for Cf0drSpec {}
